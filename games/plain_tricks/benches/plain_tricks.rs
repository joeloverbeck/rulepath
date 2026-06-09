use std::{
    env,
    hint::black_box,
    process::Command,
    time::{Duration, Instant},
};

use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, Seed, Viewer};
use plain_tricks::{
    apply_action, legal_action_tree, replay_support, setup_match, validate_command, Phase,
    PlainTricksLevel2Bot, PlainTricksRandomBot, PlainTricksSeat, PlainTricksState, SetupOptions,
    TrickCardId, ACTION_PLAY, GAME_ID, RULES_VERSION_LABEL,
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
    threshold("setup_shuffle_deal", "setups_per_second"),
    threshold("legal_actions_lead", "trees_per_second"),
    threshold("legal_actions_follow", "trees_per_second"),
    threshold("validate_play", "validations_per_second"),
    threshold("apply_play", "actions_per_second"),
    threshold("trick_resolution", "tricks_per_second"),
    threshold("project_observer_view", "views_per_second"),
    threshold("project_seat_view", "views_per_second"),
    threshold("public_export_import", "exports_per_second"),
    Threshold {
        operation: "random_legal_full_playout",
        unit: "matches_per_second",
        threshold: 2_000.0,
        rationale_class: "provisional_native_gate_floor",
        caveat: "Provisional floor for completed random-legal matches; final calibration remains pending.",
    },
    threshold("level2_full_playout", "matches_per_second"),
];

const fn threshold(operation: &'static str, unit: &'static str) -> Threshold {
    Threshold {
        operation,
        unit,
        threshold: 1.0,
        rationale_class: "baseline_pending_non_blocking",
        caveat: "Smoke floor only until plain_tricks has stable repeated CI measurements.",
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
    println!("BEGIN_PLAIN_TRICKS_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&metadata, &results));
    println!("END_PLAIN_TRICKS_BENCHMARK_JSON");
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
            "setup_shuffle_deal",
            "setups",
            50_000,
            bench_setup_shuffle_deal,
        ),
        (
            "legal_actions_lead",
            "trees",
            50_000,
            bench_legal_actions_lead,
        ),
        (
            "legal_actions_follow",
            "trees",
            50_000,
            bench_legal_actions_follow,
        ),
        ("validate_play", "validations", 50_000, bench_validate_play),
        ("apply_play", "actions", 50_000, bench_apply_play),
        ("trick_resolution", "tricks", 50_000, bench_trick_resolution),
        (
            "project_observer_view",
            "views",
            50_000,
            bench_project_observer_view,
        ),
        (
            "project_seat_view",
            "views",
            50_000,
            bench_project_seat_view,
        ),
        (
            "public_export_import",
            "exports",
            10_000,
            bench_public_export_import,
        ),
        (
            "random_legal_full_playout",
            "matches",
            5_000,
            bench_random_legal_full_playout,
        ),
        (
            "level2_full_playout",
            "matches",
            5_000,
            bench_level2_full_playout,
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
    println!("plain_tricks native benchmarks");
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

fn bench_setup_shuffle_deal(iterations: u64) {
    for index in 0..iterations {
        black_box(setup_with_seed(index));
    }
}

fn bench_legal_actions_lead(iterations: u64) {
    let state = setup();
    let actor = actor_for_seat(&state, PlainTricksSeat::Seat0);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_legal_actions_follow(iterations: u64) {
    let state = state_after_first_play();
    let actor = actor_for_seat(&state, state.active_seat.expect("follow seat is active"));
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_validate_play(iterations: u64) {
    let state = setup();
    let command = first_legal_command(&state, PlainTricksSeat::Seat0);
    for _ in 0..iterations {
        black_box(validate_command(black_box(&state), black_box(&command)).unwrap());
    }
}

fn bench_apply_play(iterations: u64) {
    let base = setup();
    let command = first_legal_command(&base, PlainTricksSeat::Seat0);
    let action = validate_command(&base, &command).expect("command validates");
    for _ in 0..iterations {
        let mut state = base.clone();
        black_box(apply_action(black_box(&mut state), black_box(action)).unwrap());
        black_box(state);
    }
}

fn bench_trick_resolution(iterations: u64) {
    let base = state_after_first_play();
    let actor = base.active_seat.expect("follow seat is active");
    let command = first_legal_command(&base, actor);
    let action = validate_command(&base, &command).expect("command validates");
    for _ in 0..iterations {
        let mut state = base.clone();
        black_box(apply_action(black_box(&mut state), black_box(action)).unwrap());
        black_box(state);
    }
}

fn bench_project_observer_view(iterations: u64) {
    let state = state_after_first_play();
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(plain_tricks::project_view(
            black_box(&state),
            black_box(&viewer),
        ));
    }
}

fn bench_project_seat_view(iterations: u64) {
    let state = state_after_first_play();
    let viewer = Viewer {
        seat_id: Some(state.seats[PlainTricksSeat::Seat0.index()].clone()),
    };
    for _ in 0..iterations {
        black_box(plain_tricks::project_view(
            black_box(&state),
            black_box(&viewer),
        ));
    }
}

fn bench_public_export_import(iterations: u64) {
    let trace = replay_support::generate_internal_full_trace();
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        let export = replay_support::export_public_replay(black_box(&trace), black_box(&viewer));
        black_box(replay_support::import_public_export(black_box(&export)));
    }
}

fn bench_random_legal_full_playout(iterations: u64) {
    for index in 0..iterations {
        black_box(full_random_legal_playout(Seed(index)));
    }
}

fn bench_level2_full_playout(iterations: u64) {
    for index in 0..iterations {
        black_box(full_level2_playout(Seed(index)));
    }
}

fn setup() -> PlainTricksState {
    setup_with_seed(7)
}

fn setup_with_seed(seed: u64) -> PlainTricksState {
    setup_match(
        Seed(seed),
        &replay_support::default_seats(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn state_after_first_play() -> PlainTricksState {
    let mut state = setup();
    let command = first_legal_command(&state, PlainTricksSeat::Seat0);
    let action = validate_command(&state, &command).expect("lead validates");
    apply_action(&mut state, action).expect("lead applies");
    state
}

fn full_random_legal_playout(seed: Seed) -> PlainTricksState {
    let mut state = setup_with_seed(seed.0);
    while state.phase != Phase::Terminal {
        let actor = state.active_seat.expect("non-terminal state has actor");
        let bot = PlainTricksRandomBot::new(Seed(seed.0.wrapping_add(state.freshness_token.0)));
        let action_path = bot
            .select_action(&state, actor)
            .expect("random bot selects legal action");
        apply_path(&mut state, actor, action_path);
    }
    state
}

fn full_level2_playout(seed: Seed) -> PlainTricksState {
    let mut state = setup_with_seed(seed.0);
    while state.phase != Phase::Terminal {
        let actor = state.active_seat.expect("non-terminal state has actor");
        let bot = PlainTricksLevel2Bot::new(Seed(seed.0.wrapping_add(state.freshness_token.0)));
        let action_path = bot
            .select_action(&state, actor)
            .expect("level 2 bot selects legal action");
        apply_path(&mut state, actor, action_path);
    }
    state
}

fn first_legal_command(state: &PlainTricksState, seat: PlainTricksSeat) -> CommandEnvelope {
    let card = plain_tricks::legal_cards(state, seat)
        .into_iter()
        .next()
        .expect("legal card exists");
    command(state, seat, card)
}

fn apply_path(state: &mut PlainTricksState, seat: PlainTricksSeat, action_path: ActionPath) {
    let envelope = CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[seat.index()].clone(),
        },
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    let action = validate_command(state, &envelope).expect("command validates");
    apply_action(state, action).expect("action applies");
}

fn command(state: &PlainTricksState, seat: PlainTricksSeat, card: TrickCardId) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_seat(state, seat),
        action_path: ActionPath {
            segments: vec![ACTION_PLAY.to_owned(), card.as_str().to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn actor_for_seat(state: &PlainTricksState, seat: PlainTricksSeat) -> Actor {
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
