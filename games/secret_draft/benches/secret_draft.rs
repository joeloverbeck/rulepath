use std::{
    env,
    hint::black_box,
    process::Command,
    time::{Duration, Instant},
};

use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Viewer};
use secret_draft::{
    actions::{commit_segment, legal_action_tree, validate_command},
    apply_action,
    bots::SecretDraftLevel1Bot,
    replay_support::{export_public_replay, generate_internal_full_trace, state_hash},
    setup_match, DraftItemId, SecretDraftSeat, SecretDraftState, SetupOptions, GAME_ID,
    RULES_VERSION_LABEL,
};

const DATA_VERSION: &str = "1";
const ENGINE_VERSION: &str = "engine-core-0.1.0";
const BUILD_PROFILE: &str = "bench";
const REPORT_SCHEMA_VERSION: u32 = 1;

struct BenchResult {
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    elapsed: Duration,
}

type BenchSpec = (&'static str, &'static str, u64, fn(u64));

#[derive(Clone, Copy)]
struct Threshold {
    operation: &'static str,
    unit: &'static str,
    threshold: f64,
    rationale_class: &'static str,
    caveat: &'static str,
}

const THRESHOLDS: &[Threshold] = &[
    threshold("legal_actions_initial_pool", "trees_per_second"),
    threshold("legal_actions_after_one_commit", "trees_per_second"),
    threshold("validate_commit", "validations_per_second"),
    threshold("apply_first_commit", "actions_per_second"),
    threshold("apply_second_commit_resolve_reveal", "actions_per_second"),
    threshold("project_public_view_pending", "views_per_second"),
    threshold("project_public_view_after_reveal", "views_per_second"),
    threshold("state_hash_terminal", "hashes_per_second"),
    threshold("public_export_timeline", "exports_per_second"),
    threshold("level1_bot_decision", "decisions_per_second"),
];

const fn threshold(operation: &'static str, unit: &'static str) -> Threshold {
    Threshold {
        operation,
        unit,
        threshold: 1.0,
        rationale_class: "baseline_pending_non_blocking",
        caveat: "Smoke floor only until secret_draft has stable CI measurements.",
    }
}

impl BenchResult {
    fn current_value(&self) -> f64 {
        self.iterations as f64 / self.elapsed.as_secs_f64()
    }
}

fn main() {
    let filter = operation_filter();
    let results = run_benchmarks(filter.as_deref());
    let metadata = ReportMetadata::new();

    print_human_summary(&results);
    println!("BEGIN_SECRET_DRAFT_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&metadata, &results));
    println!("END_SECRET_DRAFT_BENCHMARK_JSON");
}

fn operation_filter() -> Option<String> {
    env::args().skip(1).find(|arg| {
        !arg.starts_with('-')
            && THRESHOLDS
                .iter()
                .any(|threshold| threshold.operation.contains(arg))
    })
}

fn run_benchmarks(filter: Option<&str>) -> Vec<BenchResult> {
    let benches: Vec<BenchSpec> = vec![
        (
            "legal_actions_initial_pool",
            "trees",
            50_000,
            bench_legal_actions_initial_pool,
        ),
        (
            "legal_actions_after_one_commit",
            "trees",
            50_000,
            bench_legal_actions_after_one_commit,
        ),
        (
            "validate_commit",
            "validations",
            50_000,
            bench_validate_commit,
        ),
        (
            "apply_first_commit",
            "actions",
            50_000,
            bench_apply_first_commit,
        ),
        (
            "apply_second_commit_resolve_reveal",
            "actions",
            50_000,
            bench_apply_second_commit_resolve_reveal,
        ),
        (
            "project_public_view_pending",
            "views",
            50_000,
            bench_project_public_view_pending,
        ),
        (
            "project_public_view_after_reveal",
            "views",
            50_000,
            bench_project_public_view_after_reveal,
        ),
        (
            "state_hash_terminal",
            "hashes",
            50_000,
            bench_state_hash_terminal,
        ),
        (
            "public_export_timeline",
            "exports",
            10_000,
            bench_public_export_timeline,
        ),
        (
            "level1_bot_decision",
            "decisions",
            50_000,
            bench_level1_bot_decision,
        ),
    ];

    benches
        .into_iter()
        .filter(|(name, _, _, _)| filter.is_none_or(|filter| name.contains(filter)))
        .map(|(name, unit, iterations, benchmark)| measure(name, unit, iterations, benchmark))
        .collect()
}

fn measure(
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    benchmark: fn(u64),
) -> BenchResult {
    let started = Instant::now();
    benchmark(iterations);
    BenchResult {
        name,
        unit,
        iterations,
        elapsed: started.elapsed(),
    }
}

fn print_human_summary(results: &[BenchResult]) {
    println!("secret_draft native benchmarks");
    println!("operation,iterations,unit,elapsed_ms,per_second,threshold,pass");
    for result in results {
        let threshold = threshold_for(result.name);
        let current = result.current_value();
        println!(
            "{},{},{},{:.3},{:.2},{:.2},{}",
            result.name,
            result.iterations,
            result.unit,
            result.elapsed.as_secs_f64() * 1_000.0,
            current,
            threshold.threshold,
            current >= threshold.threshold
        );
    }
}

fn bench_legal_actions_initial_pool(iterations: u64) {
    let state = setup();
    let actor = actor(&state, SecretDraftSeat::Seat0);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_legal_actions_after_one_commit(iterations: u64) {
    let mut state = setup();
    apply_item(&mut state, SecretDraftSeat::Seat0, DraftItemId::Ember4);
    let actor = actor(&state, SecretDraftSeat::Seat1);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_validate_commit(iterations: u64) {
    let state = setup();
    let command = command(&state, SecretDraftSeat::Seat0, DraftItemId::Ember4);
    for _ in 0..iterations {
        black_box(validate_command(black_box(&state), black_box(&command)).unwrap());
    }
}

fn bench_apply_first_commit(iterations: u64) {
    for _ in 0..iterations {
        let mut state = setup();
        let action = validate_command(
            &state,
            &command(&state, SecretDraftSeat::Seat0, DraftItemId::Ember4),
        )
        .unwrap();
        black_box(apply_action(black_box(&mut state), black_box(action)).unwrap());
    }
}

fn bench_apply_second_commit_resolve_reveal(iterations: u64) {
    for _ in 0..iterations {
        let mut state = setup();
        apply_item(&mut state, SecretDraftSeat::Seat0, DraftItemId::Ember4);
        let action = validate_command(
            &state,
            &command(&state, SecretDraftSeat::Seat1, DraftItemId::Tide4),
        )
        .unwrap();
        black_box(apply_action(black_box(&mut state), black_box(action)).unwrap());
    }
}

fn bench_project_public_view_pending(iterations: u64) {
    let mut state = setup();
    apply_item(&mut state, SecretDraftSeat::Seat0, DraftItemId::Ember4);
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(secret_draft::project_view(
            black_box(&state),
            black_box(&viewer),
        ));
    }
}

fn bench_project_public_view_after_reveal(iterations: u64) {
    let mut state = setup();
    apply_item(&mut state, SecretDraftSeat::Seat0, DraftItemId::Ember4);
    apply_item(&mut state, SecretDraftSeat::Seat1, DraftItemId::Tide4);
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(secret_draft::project_view(
            black_box(&state),
            black_box(&viewer),
        ));
    }
}

fn bench_state_hash_terminal(iterations: u64) {
    let state = terminal_state();
    for _ in 0..iterations {
        black_box(state_hash(black_box(&state)));
    }
}

fn bench_public_export_timeline(iterations: u64) {
    let trace = generate_internal_full_trace();
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(export_public_replay(black_box(&trace), black_box(&viewer)));
    }
}

fn bench_level1_bot_decision(iterations: u64) {
    let state = setup();
    let bot = SecretDraftLevel1Bot::new(engine_core::Seed(7));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(SecretDraftSeat::Seat0))
                .unwrap(),
        );
    }
}

fn setup() -> SecretDraftState {
    setup_match(
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn terminal_state() -> SecretDraftState {
    let mut state = setup();
    for chunk in DraftItemId::ALL.chunks(2) {
        apply_item(&mut state, SecretDraftSeat::Seat0, chunk[0]);
        apply_item(&mut state, SecretDraftSeat::Seat1, chunk[1]);
    }
    state
}

fn apply_item(state: &mut SecretDraftState, seat: SecretDraftSeat, item: DraftItemId) {
    let envelope = command(state, seat, item);
    let action = validate_command(state, &envelope).expect("command validates");
    apply_action(state, action).expect("action applies");
}

fn command(state: &SecretDraftState, seat: SecretDraftSeat, item: DraftItemId) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(state, seat),
        action_path: ActionPath {
            segments: vec![commit_segment(item)],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn actor(state: &SecretDraftState, seat: SecretDraftSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

struct ReportMetadata {
    command: String,
    os: String,
    rust_version: String,
    hardware_environment_notes: String,
}

impl ReportMetadata {
    fn new() -> Self {
        Self {
            command: env::args().collect::<Vec<_>>().join(" "),
            os: format!("{} {}", env::consts::OS, env::consts::ARCH),
            rust_version: rust_version(),
            hardware_environment_notes:
                "Local native benchmark run; no CPU pinning, thermal isolation, or hardware probe."
                    .to_owned(),
        }
    }
}

fn rust_version() -> String {
    Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn benchmark_report_json(metadata: &ReportMetadata, results: &[BenchResult]) -> String {
    let operations = results
        .iter()
        .map(operation_json)
        .collect::<Vec<_>>()
        .join(",\n    ");
    format!(
        concat!(
            "{{\n",
            "  \"schema_version\": {},\n",
            "  \"game_id\": \"{}\",\n",
            "  \"rules_version\": \"{}\",\n",
            "  \"data_version\": \"{}\",\n",
            "  \"engine_version\": \"{}\",\n",
            "  \"build_profile\": \"{}\",\n",
            "  \"command\": \"{}\",\n",
            "  \"os\": \"{}\",\n",
            "  \"rust_version\": \"{}\",\n",
            "  \"hardware_environment_notes\": \"{}\",\n",
            "  \"known_caveats\": [\"Local workstation runs can be noisy; thresholds are smoke floors until stable CI baselines exist.\"],\n",
            "  \"operations\": [\n",
            "    {}\n",
            "  ]\n",
            "}}"
        ),
        REPORT_SCHEMA_VERSION,
        GAME_ID,
        RULES_VERSION_LABEL,
        DATA_VERSION,
        ENGINE_VERSION,
        BUILD_PROFILE,
        escape_json(&metadata.command),
        escape_json(&metadata.os),
        escape_json(&metadata.rust_version),
        escape_json(&metadata.hardware_environment_notes),
        operations
    )
}

fn operation_json(result: &BenchResult) -> String {
    let threshold = threshold_for(result.name);
    let current = result.current_value();
    format!(
        concat!(
            "{{\"operation_name\":\"{}\",",
            "\"iterations\":{},",
            "\"unit\":\"{}\",",
            "\"current_value\":{:.2},",
            "\"threshold\":{:.2},",
            "\"pass\":{},",
            "\"rationale_class\":\"{}\",",
            "\"known_caveats\":\"{}\"}}"
        ),
        result.name,
        result.iterations,
        threshold.unit,
        current,
        threshold.threshold,
        current >= threshold.threshold,
        threshold.rationale_class,
        escape_json(threshold.caveat)
    )
}

fn threshold_for(operation: &str) -> Threshold {
    THRESHOLDS
        .iter()
        .copied()
        .find(|threshold| threshold.operation == operation)
        .expect("threshold exists for operation")
}

fn escape_json(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
