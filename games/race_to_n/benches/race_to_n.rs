use std::{
    env,
    hint::black_box,
    process::Command,
    time::{Duration, Instant},
};

use engine_core::{
    ActionPath, Actor, CommandEnvelope, EffectCursor, EffectLog, HashValue, RulesVersion, SeatId,
    Seed, StableSerialize, Viewer,
};
use race_to_n::{
    apply_action, legal_action_tree, project_view, setup_match, validate_command, RaceEffect,
    RaceRandomBot, RaceSnapshot, RaceState, SetupOptions,
};

const RULES_VERSION: u32 = 1;
const GAME_ID: &str = "race_to_n";
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
        threshold: 1_000_000.0,
        rationale_class: "conservative_ci_floor",
        caveat: "Initial CI floor below the first WSL2 baseline to avoid treating noisy local variance as doctrine.",
    },
    Threshold {
        operation: "apply_action",
        unit: "actions_per_second",
        threshold: 5_000_000.0,
        rationale_class: "conservative_ci_floor",
        caveat: "Initial CI floor below the first WSL2 baseline; validation is included in the measured loop.",
    },
    Threshold {
        operation: "public_view_generation",
        unit: "views_per_second",
        threshold: 10_000_000.0,
        rationale_class: "conservative_ci_floor",
        caveat: "Initial CI floor for the public perfect-information view path.",
    },
    Threshold {
        operation: "effect_filtering",
        unit: "filters_per_second",
        threshold: 10_000_000.0,
        rationale_class: "conservative_ci_floor",
        caveat: "Initial CI floor for public EffectLog filtering over the tiny Gate 1 effect set.",
    },
    Threshold {
        operation: "serialization_roundtrip",
        unit: "roundtrips_per_second",
        threshold: 180_000.0,
        rationale_class: "conservative_ci_floor",
        caveat: "CI-calibrated floor below observed ubuntu-latest values 192697 in run 27086098697, 194572.37 in run 27087214359, and 187667.53 in run 27087615808; native WSL2 target remains documented in BENCHMARKS.md.",
    },
    Threshold {
        operation: "replay_throughput",
        unit: "replays_per_second",
        threshold: 220_000.0,
        rationale_class: "conservative_ci_floor",
        caveat: "CI-calibrated floor below observed ubuntu-latest values 231094 in run 27086098697, 227909.73 in run 27087214359, and 226842.75 in run 27087615808; native WSL2 target remains documented in BENCHMARKS.md.",
    },
    Threshold {
        operation: "random_playout",
        unit: "games_per_second",
        threshold: 65_000.0,
        rationale_class: "accepted_adr",
        caveat: "ADR 0003 recalibrates the enforced CI floor below observed ubuntu-latest values 66050 in run 27086098697 and 67531.86 in run 27087214359; the ADR 0001 100000 native target remains documented in BENCHMARKS.md.",
    },
    Threshold {
        operation: "bot_decision",
        unit: "decisions_per_second",
        threshold: 950_000.0,
        rationale_class: "conservative_ci_floor",
        caveat: "CI-calibrated floor below observed ubuntu-latest values 985488 in run 27086098697, 978335.89 in run 27087214359, and 980644.97 in run 27087615808; native WSL2 target remains documented in BENCHMARKS.md.",
    },
];

fn main() {
    let filter = operation_filter();
    let results = run_benchmarks(filter.as_deref());
    let metadata = ReportMetadata::new();

    print_human_summary(&results);
    println!("BEGIN_RACE_TO_N_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&metadata, &results));
    println!("END_RACE_TO_N_BENCHMARK_JSON");
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
        ("legal_actions", "trees", 1_000_000, bench_legal_actions),
        ("apply_action", "actions", 1_000_000, bench_apply_action),
        (
            "public_view_generation",
            "views",
            1_000_000,
            bench_public_view,
        ),
        (
            "effect_filtering",
            "filters",
            1_000_000,
            bench_effect_filtering,
        ),
        (
            "serialization_roundtrip",
            "roundtrips",
            500_000,
            bench_serialization_roundtrip,
        ),
        ("replay_throughput", "replays", 100_000, bench_replay),
        ("random_playout", "games", 100_000, bench_random_playout),
        ("bot_decision", "decisions", 1_000_000, bench_bot_decision),
    ];

    benches
        .into_iter()
        .filter(|(name, _, _, _)| filter.is_none_or(|filter| name.contains(filter)))
        .map(|(name, unit, iterations, benchmark)| measure(name, unit, iterations, benchmark))
        .collect()
}

fn print_human_summary(results: &[BenchResult]) {
    println!("race_to_n native benchmarks");
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
            "  \"rules_version\": \"race_to_n-rules-v{}\",\n",
            "  \"data_version\": \"{}\",\n",
            "  \"engine_version\": \"{}\",\n",
            "  \"build_profile\": \"{}\",\n",
            "  \"command\": \"{}\",\n",
            "  \"os\": \"{}\",\n",
            "  \"rust_version\": \"{}\",\n",
            "  \"hardware_environment_notes\": \"{}\",\n",
            "  \"known_caveats\": [\"WSL2 and local workstation runs can be noisy; bench-report owns threshold gating.\"],\n",
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
    iterations: u64,
    mut benchmark: impl FnMut(u64),
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
        if state.winner.is_some() {
            state = initial_state(2);
        }
        let command = command_for_state(&state, "add-1");
        let action = validate_command(&state, &command).expect("benchmark command validates");
        black_box(apply_action(black_box(&mut state), black_box(action)));
    }
}

fn bench_public_view(iterations: u64) {
    let state = initial_state(3);
    for _ in 0..iterations {
        black_box(project_view(black_box(&state)));
    }
}

fn bench_effect_filtering(iterations: u64) {
    let mut log = EffectLog::new();
    let mut state = initial_state(4);
    let action = validate_command(&state, &command_for_state(&state, "add-1"))
        .expect("benchmark command validates");
    for effect in apply_action(&mut state, action) {
        log.push(effect);
    }
    let viewer = Viewer { seat_id: None };

    for _ in 0..iterations {
        black_box(log.since(black_box(EffectCursor(0)), black_box(&viewer)));
    }
}

fn bench_serialization_roundtrip(iterations: u64) {
    let snapshot = RaceSnapshot::from_state(&initial_state(5));
    let json = snapshot.to_json();
    for _ in 0..iterations {
        let parsed = RaceSnapshot::from_json(black_box(&json)).expect("snapshot parses");
        black_box(parsed.stable_hash());
    }
}

fn bench_replay(iterations: u64) {
    let commands = [
        "add-3", "add-3", "add-3", "add-3", "add-3", "add-3", "add-3",
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

fn bench_bot_decision(iterations: u64) {
    let state = initial_state(6);
    for seed in 0..iterations {
        let bot = RaceRandomBot::new(Seed(seed));
        black_box(
            bot.select_action(black_box(&state), black_box(state.active_seat))
                .expect("bot selects a legal action"),
        );
    }
}

fn run_random_playout(seed: u64) -> u64 {
    let mut state = initial_state(seed);
    let mut actions = 0;

    while state.winner.is_none() {
        let bot = RaceRandomBot::new(Seed(seed.wrapping_add(actions)));
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

fn replay_commands(seed: u64, commands: &[&str]) -> HashValue {
    let mut state = initial_state(seed);
    let mut effects = Vec::new();

    for segment in commands {
        let command = command_for_state(&state, segment);
        let action = validate_command(&state, &command).expect("replay command validates");
        effects.extend(apply_action(&mut state, action));
    }

    let bytes = effects
        .iter()
        .map(effect_json)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

fn initial_state(seed: u64) -> RaceState {
    setup_match(Seed(seed), &seats(), &SetupOptions::default()).expect("setup succeeds")
}

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor_for_state(state: &RaceState) -> Actor {
    Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

fn command_for_state(state: &RaceState, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_state(state),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(RULES_VERSION),
    }
}

fn effect_json(effect: &engine_core::EffectEnvelope<RaceEffect>) -> String {
    match &effect.payload {
        RaceEffect::ActionStarted { actor, amount } => {
            format!("ActionStarted:{}:{amount}", actor.as_str())
        }
        RaceEffect::CounterAdvanced {
            actor,
            from,
            to,
            amount,
        } => format!(
            "CounterAdvanced:{}:{}:{}:{amount}",
            actor.as_str(),
            from.0,
            to.0
        ),
        RaceEffect::TurnChanged { next_actor } => {
            format!("TurnChanged:{}", next_actor.as_str())
        }
        RaceEffect::GameEnded { winner } => format!("GameEnded:{}", winner.as_str()),
        RaceEffect::ActionCompleted { actor } => format!("ActionCompleted:{}", actor.as_str()),
    }
}
