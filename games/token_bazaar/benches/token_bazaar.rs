use std::{
    env,
    hint::black_box,
    process::Command,
    time::{Duration, Instant},
};

use engine_core::{
    ActionPath, Actor, CommandEnvelope, EffectCursor, EffectLog, RulesVersion, Seed,
    StableSerialize, Viewer,
};
use token_bazaar::{
    apply_action, command_for_state, default_seats, export_public_replay, legal_action_tree,
    project_view, setup_match, validate_command, TokenBazaarLevel1Bot, TokenBazaarRandomBot,
    TokenBazaarSeat, TokenBazaarState, GAME_ID, RULES_VERSION_LABEL,
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
    threshold("standard_setup", "setups_per_second"),
    threshold("legal_actions_initial", "trees_per_second"),
    threshold("validate_apply_collect", "actions_per_second"),
    threshold("validate_apply_exchange", "actions_per_second"),
    threshold("validate_apply_fulfill_refill", "actions_per_second"),
    threshold("public_view_generation", "views_per_second"),
    threshold("effect_serialization_filtering", "filters_per_second"),
    threshold("replay_command_stream", "replays_per_second"),
    threshold("random_legal_playout", "games_per_second"),
    threshold("level1_bot_decision", "decisions_per_second"),
    threshold("wasm_operation_smoke", "operations_per_second"),
];

const fn threshold(operation: &'static str, unit: &'static str) -> Threshold {
    Threshold {
        operation,
        unit,
        threshold: 1.0,
        rationale_class: "baseline_pending_non_blocking",
        caveat: "Smoke floor only until token_bazaar has stable CI measurements.",
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
    println!("BEGIN_TOKEN_BAZAAR_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&metadata, &results));
    println!("END_TOKEN_BAZAAR_BENCHMARK_JSON");
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
        ("standard_setup", "setups", 20_000, bench_standard_setup),
        (
            "legal_actions_initial",
            "trees",
            50_000,
            bench_legal_actions_initial,
        ),
        (
            "validate_apply_collect",
            "actions",
            50_000,
            bench_validate_apply_collect,
        ),
        (
            "validate_apply_exchange",
            "actions",
            50_000,
            bench_validate_apply_exchange,
        ),
        (
            "validate_apply_fulfill_refill",
            "actions",
            50_000,
            bench_validate_apply_fulfill_refill,
        ),
        ("public_view_generation", "views", 50_000, bench_public_view),
        (
            "effect_serialization_filtering",
            "filters",
            50_000,
            bench_effect_serialization_filtering,
        ),
        (
            "replay_command_stream",
            "replays",
            5_000,
            bench_replay_command_stream,
        ),
        (
            "random_legal_playout",
            "games",
            5_000,
            bench_random_legal_playout,
        ),
        (
            "level1_bot_decision",
            "decisions",
            50_000,
            bench_level1_bot_decision,
        ),
        (
            "wasm_operation_smoke",
            "operations",
            25_000,
            bench_wasm_operation_smoke,
        ),
    ];

    benches
        .into_iter()
        .filter(|(name, _, _, _)| filter.is_none_or(|filter| name.contains(filter)))
        .map(|(name, unit, iterations, benchmark)| measure(name, unit, iterations, benchmark))
        .collect()
}

fn print_human_summary(results: &[BenchResult]) {
    println!("token_bazaar native benchmarks");
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

fn threshold_for(name: &str) -> Threshold {
    THRESHOLDS
        .iter()
        .copied()
        .find(|threshold| threshold.operation == name)
        .expect("benchmark has threshold")
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

fn bench_standard_setup(iterations: u64) {
    for index in 0..iterations {
        black_box(setup(Seed(index)));
    }
}

fn bench_legal_actions_initial(iterations: u64) {
    let state = setup(Seed(7));
    let actor = actor_for_state(&state);
    for _ in 0..iterations {
        black_box(legal_action_tree(&state, &actor));
    }
}

fn bench_validate_apply_collect(iterations: u64) {
    let state = setup(Seed(7));
    let command = command_for_state(&state, vec!["collect/amber".to_owned()]);
    for _ in 0..iterations {
        let mut state = state.clone();
        let action = validate_command(&state, &command).expect("collect validates");
        black_box(apply_action(&mut state, action));
    }
}

fn bench_validate_apply_exchange(iterations: u64) {
    let state = exchange_ready_state();
    let command = command_for_state(&state, vec!["exchange/amber/iron".to_owned()]);
    for _ in 0..iterations {
        let mut state = state.clone();
        let action = validate_command(&state, &command).expect("exchange validates");
        black_box(apply_action(&mut state, action));
    }
}

fn bench_validate_apply_fulfill_refill(iterations: u64) {
    let state = setup(Seed(7));
    let command = command_for_state(&state, vec!["fulfill/slot_0".to_owned()]);
    for _ in 0..iterations {
        let mut state = state.clone();
        let action = validate_command(&state, &command).expect("fulfill validates");
        black_box(apply_action(&mut state, action));
    }
}

fn bench_public_view(iterations: u64) {
    let state = exchange_ready_state();
    for _ in 0..iterations {
        black_box(project_view(&state, &Viewer { seat_id: None }));
    }
}

fn bench_effect_serialization_filtering(iterations: u64) {
    let mut state = setup(Seed(7));
    let command = command_for_state(&state, vec!["fulfill/slot_0".to_owned()]);
    let action = validate_command(&state, &command).expect("fulfill validates");
    let effects = apply_action(&mut state, action);
    let mut log = EffectLog::new();
    for effect in effects {
        black_box(effect.payload.stable_hash());
        log.push(effect);
    }

    for _ in 0..iterations {
        black_box(log.since(EffectCursor(0), &Viewer { seat_id: None }));
    }
}

fn bench_replay_command_stream(iterations: u64) {
    let commands = vec![
        vec!["collect/amber".to_owned()],
        vec!["collect/jade".to_owned()],
        vec!["exchange/amber/iron".to_owned()],
    ];
    for _ in 0..iterations {
        black_box(token_bazaar::replay_commands(7, &commands));
    }
}

fn bench_random_legal_playout(iterations: u64) {
    for index in 0..iterations {
        black_box(playout(Seed(index)));
    }
}

fn bench_level1_bot_decision(iterations: u64) {
    let state = setup(Seed(7));
    for index in 0..iterations {
        let bot = TokenBazaarLevel1Bot::new(Seed(index));
        black_box(
            bot.select_decision(&state, TokenBazaarSeat::Seat0)
                .expect("bot chooses"),
        );
    }
}

fn bench_wasm_operation_smoke(iterations: u64) {
    let state = exchange_ready_state();
    let commands = vec![
        vec!["collect/amber".to_owned()],
        vec!["collect/jade".to_owned()],
        vec!["exchange/amber/iron".to_owned()],
    ];
    for _ in 0..iterations {
        black_box(project_view(&state, &Viewer { seat_id: None }).stable_hash());
        black_box(export_public_replay(7, &commands).to_json());
    }
}

fn setup(seed: Seed) -> TokenBazaarState {
    setup_match(seed, &default_seats(), &Default::default()).expect("setup succeeds")
}

fn exchange_ready_state() -> TokenBazaarState {
    let mut state = setup(Seed(7));
    apply_path(&mut state, "collect/amber");
    apply_path(&mut state, "collect/jade");
    state
}

fn apply_path(state: &mut TokenBazaarState, segment: &str) {
    let command = command_for_state(state, vec![segment.to_owned()]);
    let action = validate_command(state, &command).expect("path validates");
    apply_action(state, action);
}

fn actor_for_state(state: &TokenBazaarState) -> Actor {
    Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

fn playout(seed: Seed) -> Option<TokenBazaarSeat> {
    let mut state = setup(seed);
    let mut action_index = 0_u64;
    while state.terminal_outcome.is_none() {
        let bot = TokenBazaarRandomBot::new(Seed(seed.0.wrapping_add(action_index)));
        let action_path = bot
            .select_action(&state, state.active_seat)
            .expect("bot chooses legal action");
        let command = command_for_action_path(&state, action_path);
        let action = validate_command(&state, &command).expect("bot command validates");
        apply_action(&mut state, action);
        action_index += 1;
    }
    match state.terminal_outcome {
        Some(token_bazaar::TerminalOutcome::Win { seat }) => Some(seat),
        Some(token_bazaar::TerminalOutcome::Draw) | None => None,
    }
}

fn command_for_action_path(state: &TokenBazaarState, action_path: ActionPath) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}
