use std::{
    env,
    hint::black_box,
    process::Command,
    time::{Duration, Instant},
};

use column_four::{
    apply_action, legal_action_tree, project_view,
    replay_support::{project_step, replay_commands, ColumnFourReplayJson},
    setup_match, validate_command, ColumnFourLevel2Bot, ColumnFourRandomBot, ColumnFourState,
    SetupOptions,
};
use engine_core::{
    ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, StableSerialize, Viewer,
};

const RULES_VERSION: u32 = 1;
const GAME_ID: &str = "column_four";
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

impl BenchResult {
    fn current_value(&self) -> f64 {
        self.iterations as f64 / self.elapsed.as_secs_f64()
    }
}

#[derive(Clone, Copy)]
struct Threshold {
    operation: &'static str,
    unit: &'static str,
    threshold: f64,
    rationale_class: &'static str,
    caveat: &'static str,
}

const THRESHOLDS: &[Threshold] = &[
    Threshold {
        operation: "legal_actions",
        unit: "trees_per_second",
        threshold: 200_000.0,
        rationale_class: "measured_baseline",
        caveat: "Initial floor below the first local native smoke measurement for the seven-column legal action tree.",
    },
    Threshold {
        operation: "apply_action",
        unit: "actions_per_second",
        threshold: 200_000.0,
        rationale_class: "measured_baseline",
        caveat: "Initial floor below the first local native smoke measurement for validation plus gravity placement.",
    },
    Threshold {
        operation: "public_view_generation",
        unit: "views_per_second",
        threshold: 100_000.0,
        rationale_class: "measured_baseline",
        caveat: "Initial floor below the first local native smoke measurement for Rust-projected public board views.",
    },
    Threshold {
        operation: "replay_step_projection",
        unit: "projections_per_second",
        threshold: 33_000.0,
        rationale_class: "conservative_ci_floor",
        caveat: "CI-calibrated floor below observed ubuntu-latest value 34175.00 in BENCICAL-001 run 27087214359; native baseline remains documented in BENCHMARKS.md per ADR 0003.",
    },
    Threshold {
        operation: "serialization_roundtrip",
        unit: "roundtrips_per_second",
        threshold: 50_000.0,
        rationale_class: "measured_baseline",
        caveat: "Initial floor below the first local native smoke measurement for replay JSON serialization round trips.",
    },
    Threshold {
        operation: "replay_throughput",
        unit: "replays_per_second",
        threshold: 3_200.0,
        rationale_class: "conservative_ci_floor",
        caveat: "CI-calibrated floor below observed ubuntu-latest value 3334.35 in BENCICAL-001 run 27087214359; native baseline remains documented in BENCHMARKS.md per ADR 0003.",
    },
    Threshold {
        operation: "random_playout",
        unit: "games_per_second",
        threshold: 9_000.0,
        rationale_class: "conservative_ci_floor",
        caveat: "CI-calibrated floor below observed ubuntu-latest value 9700.09 in BENCICAL-001 run 27087214359; native target miss remains documented in BENCHMARKS.md per ADR 0003.",
    },
    Threshold {
        operation: "level0_bot_decision",
        unit: "decisions_per_second",
        threshold: 100_000.0,
        rationale_class: "measured_baseline",
        caveat: "Initial floor below the first local native smoke measurement for seeded random legal selection.",
    },
    Threshold {
        operation: "level2_bot_decision",
        unit: "decisions_per_second",
        threshold: 3_000.0,
        rationale_class: "conservative_ci_floor",
        caveat: "CI-calibrated floor below observed ubuntu-latest value 3063.45 in BENCICAL-001 run 27087214359; native baseline remains documented in BENCHMARKS.md per ADR 0003.",
    },
];

fn main() {
    let filter = operation_filter();
    let results = run_benchmarks(filter.as_deref());
    let metadata = ReportMetadata::new();

    print_human_summary(&results);
    println!("BEGIN_COLUMN_FOUR_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&metadata, &results));
    println!("END_COLUMN_FOUR_BENCHMARK_JSON");
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
        ("legal_actions", "trees", 200_000, bench_legal_actions),
        ("apply_action", "actions", 200_000, bench_apply_action),
        (
            "public_view_generation",
            "views",
            100_000,
            bench_public_view,
        ),
        (
            "replay_step_projection",
            "projections",
            100_000,
            bench_replay_step_projection,
        ),
        (
            "serialization_roundtrip",
            "roundtrips",
            50_000,
            bench_serialization_roundtrip,
        ),
        ("replay_throughput", "replays", 25_000, bench_replay),
        ("random_playout", "games", 10_000, bench_random_playout),
        (
            "level0_bot_decision",
            "decisions",
            100_000,
            bench_level0_bot_decision,
        ),
        (
            "level2_bot_decision",
            "decisions",
            50_000,
            bench_level2_bot_decision,
        ),
    ];

    benches
        .into_iter()
        .filter(|(name, _, _, _)| filter.is_none_or(|filter| name.contains(filter)))
        .map(|(name, unit, iterations, benchmark)| measure(name, unit, iterations, benchmark))
        .collect()
}

fn print_human_summary(results: &[BenchResult]) {
    println!("column_four native benchmarks");
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
            "  \"rules_version\": \"column_four-rules-v{}\",\n",
            "  \"data_version\": \"{}\",\n",
            "  \"engine_version\": \"{}\",\n",
            "  \"build_profile\": \"{}\",\n",
            "  \"command\": \"{}\",\n",
            "  \"os\": \"{}\",\n",
            "  \"rust_version\": \"{}\",\n",
            "  \"hardware_environment_notes\": \"{}\",\n",
            "  \"known_caveats\": [\"Local workstation runs can be noisy; bench-report owns threshold gating.\"],\n",
            "  \"operations\": [\n",
            "    {}\n",
            "  ]\n",
            "}}"
        ),
        REPORT_SCHEMA_VERSION,
        GAME_ID,
        RULES_VERSION,
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
        .unwrap_or_else(|| panic!("missing threshold for operation `{operation}`"))
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

fn measure(
    name: &'static str,
    unit: &'static str,
    mut iterations: u64,
    mut benchmark: impl FnMut(u64),
) -> BenchResult {
    if cfg!(debug_assertions) {
        iterations = iterations.min(10);
    }
    let started = Instant::now();
    benchmark(iterations);
    BenchResult {
        name,
        unit,
        iterations,
        elapsed: started.elapsed(),
    }
}

fn bench_legal_actions(iterations: u64) {
    let state = initial_state(1);
    let actor = actor_for_state(&state);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_apply_action(iterations: u64) {
    let mut state = initial_state(2);
    for _ in 0..iterations {
        if state.terminal_outcome.is_some() {
            state = initial_state(2);
        }
        let segment = first_legal_segment(&state);
        let command = command_for_state(&state, &segment);
        let action = validate_command(&state, &command).expect("benchmark command validates");
        black_box(apply_action(black_box(&mut state), black_box(action)));
    }
}

fn bench_public_view(iterations: u64) {
    let state = initial_state(3);
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(project_view(black_box(&state), black_box(&viewer)));
    }
}

fn bench_replay_step_projection(iterations: u64) {
    let mut state = initial_state(4);
    let command = command_for_state(&state, "drop/c4");
    let action = validate_command(&state, &command).expect("benchmark command validates");
    let effects = apply_action(&mut state, action);

    for index in 0..iterations {
        black_box(project_step(
            black_box(index as usize),
            black_box(&state),
            black_box(&effects),
        ));
    }
}

fn bench_serialization_roundtrip(iterations: u64) {
    let replay = ColumnFourReplayJson {
        schema_version: 1,
        game_id: "column_four".to_owned(),
        rules_version: "column_four-rules-v1".to_owned(),
        variant: "column_four_standard".to_owned(),
        seed: 5,
        initial_snapshot: "snapshot".to_owned(),
        command_segments: vec!["drop/c4".to_owned(), "drop/c3".to_owned()],
    };
    let json = replay.to_json();
    for _ in 0..iterations {
        let parsed = ColumnFourReplayJson::from_json(black_box(&json)).expect("replay parses");
        black_box(parsed.stable_hash());
    }
}

fn bench_replay(iterations: u64) {
    let commands = [
        "drop/c1".to_owned(),
        "drop/c2".to_owned(),
        "drop/c1".to_owned(),
        "drop/c2".to_owned(),
        "drop/c1".to_owned(),
        "drop/c2".to_owned(),
        "drop/c1".to_owned(),
    ];
    for index in 0..iterations {
        black_box(replay_commands(black_box(index), black_box(&commands)));
    }
}

fn bench_random_playout(iterations: u64) {
    for seed in 0..iterations {
        black_box(run_random_playout(black_box(seed)));
    }
}

fn bench_level0_bot_decision(iterations: u64) {
    let state = initial_state(6);
    for seed in 0..iterations {
        let bot = ColumnFourRandomBot::new(Seed(seed));
        black_box(
            bot.select_action(black_box(&state), black_box(state.active_seat))
                .expect("bot selects a legal action"),
        );
    }
}

fn bench_level2_bot_decision(iterations: u64) {
    let state = initial_state(7);
    let bot = ColumnFourLevel2Bot::new(Seed(7));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(state.active_seat))
                .expect("bot selects a legal action"),
        );
    }
}

fn run_random_playout(seed: u64) -> u64 {
    let mut state = initial_state(seed);
    let mut actions = 0;

    while state.terminal_outcome.is_none() {
        let bot = ColumnFourRandomBot::new(Seed(seed.wrapping_add(actions)));
        let action_path = bot
            .select_action(&state, state.active_seat)
            .expect("bot selects a legal action");
        let command = CommandEnvelope {
            actor: actor_for_state(&state),
            action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = validate_command(&state, &command).expect("bot action validates");
        apply_action(&mut state, action);
        actions += 1;
    }

    actions
}

fn initial_state(seed: u64) -> ColumnFourState {
    setup_match(Seed(seed), &seats(), &SetupOptions::default()).expect("setup succeeds")
}

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor_for_state(state: &ColumnFourState) -> Actor {
    Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

fn command_for_state(state: &ColumnFourState, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_state(state),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(RULES_VERSION),
    }
}

fn first_legal_segment(state: &ColumnFourState) -> String {
    let tree = legal_action_tree(state, &actor_for_state(state));
    tree.root.choices[0].segment.clone()
}
