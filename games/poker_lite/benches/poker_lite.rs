use std::{
    env,
    hint::black_box,
    process::Command,
    time::{Duration, Instant},
};

use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};
use poker_lite::{
    actor_for_seat, apply_action, legal_action_tree, replay_support, setup_match, validate_command,
    PokerLiteAction, PokerLiteLevel2Bot, PokerLiteSeat, PokerLiteState, SetupOptions, GAME_ID,
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
    threshold("setup_shuffle_deal", "setups_per_second"),
    threshold("legal_actions_initial_pledge", "trees_per_second"),
    threshold("validate_press", "validations_per_second"),
    threshold("apply_press", "actions_per_second"),
    threshold("project_observer_view", "views_per_second"),
    threshold("public_export_import", "exports_per_second"),
    threshold("state_hash_terminal", "hashes_per_second"),
    threshold("level2_bot_decision", "decisions_per_second"),
    Threshold {
        operation: "level2_full_playout",
        unit: "hands_per_second",
        threshold: 2_000.0,
        rationale_class: "provisional_native_gate_floor",
        caveat: "Provisional floor for completed hands; final calibration remains pending.",
    },
];

const fn threshold(operation: &'static str, unit: &'static str) -> Threshold {
    Threshold {
        operation,
        unit,
        threshold: 1.0,
        rationale_class: "baseline_pending_non_blocking",
        caveat: "Smoke floor only until poker_lite has stable CI measurements.",
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
    println!("BEGIN_POKER_LITE_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&metadata, &results));
    println!("END_POKER_LITE_BENCHMARK_JSON");
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
            "legal_actions_initial_pledge",
            "trees",
            50_000,
            bench_legal_actions_initial_pledge,
        ),
        (
            "validate_press",
            "validations",
            50_000,
            bench_validate_press,
        ),
        ("apply_press", "actions", 50_000, bench_apply_press),
        (
            "project_observer_view",
            "views",
            50_000,
            bench_project_observer_view,
        ),
        (
            "public_export_import",
            "exports",
            10_000,
            bench_public_export_import,
        ),
        (
            "state_hash_terminal",
            "hashes",
            50_000,
            bench_state_hash_terminal,
        ),
        (
            "level2_bot_decision",
            "decisions",
            50_000,
            bench_level2_bot_decision,
        ),
        (
            "level2_full_playout",
            "hands",
            10_000,
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
    println!("poker_lite native benchmarks");
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

fn bench_legal_actions_initial_pledge(iterations: u64) {
    let state = setup();
    let actor = actor_for_seat(&state, PokerLiteSeat::Seat0);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_validate_press(iterations: u64) {
    let state = setup();
    let command = command(&state, PokerLiteSeat::Seat0, PokerLiteAction::Press);
    for _ in 0..iterations {
        black_box(validate_command(black_box(&state), black_box(&command)).unwrap());
    }
}

fn bench_apply_press(iterations: u64) {
    for _ in 0..iterations {
        let mut state = setup();
        apply_known_action(&mut state, PokerLiteSeat::Seat0, PokerLiteAction::Press);
        black_box(state);
    }
}

fn bench_project_observer_view(iterations: u64) {
    let mut state = setup();
    apply_known_action(&mut state, PokerLiteSeat::Seat0, PokerLiteAction::Press);
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(poker_lite::project_view(
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

fn bench_state_hash_terminal(iterations: u64) {
    let state = full_playout_state(Seed(7));
    for _ in 0..iterations {
        black_box(replay_support::state_hash(black_box(&state)));
    }
}

fn bench_level2_bot_decision(iterations: u64) {
    let state = setup();
    let bot = PokerLiteLevel2Bot::new(Seed(7));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(PokerLiteSeat::Seat0))
                .unwrap(),
        );
    }
}

fn bench_level2_full_playout(iterations: u64) {
    for index in 0..iterations {
        black_box(full_playout_state(Seed(index)));
    }
}

fn setup() -> PokerLiteState {
    setup_with_seed(7)
}

fn setup_with_seed(seed: u64) -> PokerLiteState {
    setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).expect("setup succeeds")
}

fn default_seats() -> Vec<SeatId> {
    vec![SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn full_playout_state(seed: Seed) -> PokerLiteState {
    let mut state =
        setup_match(seed, &default_seats(), &SetupOptions::default()).expect("setup succeeds");
    let bot = PokerLiteLevel2Bot::new(seed);
    while let Some(active_seat) = state.active_seat {
        let action = bot
            .select_decision(&state, active_seat)
            .expect("bot decision succeeds");
        let [segment] = action.action_path.segments.as_slice() else {
            panic!("bot selected malformed action path");
        };
        let parsed = poker_lite::parse_action_segment(segment).expect("bot action parses");
        apply_known_action(&mut state, active_seat, parsed);
    }
    state
}

fn apply_known_action(state: &mut PokerLiteState, seat: PokerLiteSeat, action: PokerLiteAction) {
    let envelope = command(state, seat, action);
    let action = validate_command(state, &envelope).expect("command validates");
    apply_action(state, action).expect("action applies");
}

fn command(
    state: &PokerLiteState,
    seat: PokerLiteSeat,
    action: PokerLiteAction,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: vec![action.segment().to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
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
