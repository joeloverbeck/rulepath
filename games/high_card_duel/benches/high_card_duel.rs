use std::{
    env,
    hint::black_box,
    process::Command,
    time::{Duration, Instant},
};

use engine_core::{
    Actor, CommandEnvelope, EffectCursor, EffectLog, RulesVersion, Seed, StableSerialize, Viewer,
};
use high_card_duel::{
    active_commit_seat, apply_action, commit_segment, export_public_observer_replay,
    generate_internal_full_trace, legal_action_tree, project_view, replay_internal_full_trace,
    setup_match, validate_command, HighCardDuelRandomBot, HighCardDuelSeat, HighCardDuelState,
    Phase, SetupOptions, GAME_ID, RULES_VERSION_LABEL,
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
    threshold("standard_setup_shuffle", "setups_per_second"),
    threshold("legal_actions_lead", "trees_per_second"),
    threshold("legal_actions_reply", "trees_per_second"),
    threshold("validate_commit", "validations_per_second"),
    threshold("apply_commit", "actions_per_second"),
    threshold("apply_reveal_refill", "actions_per_second"),
    threshold("public_view_generation", "views_per_second"),
    threshold("seat_private_view_generation", "views_per_second"),
    threshold("effect_filtering", "filters_per_second"),
    threshold("public_replay_export", "exports_per_second"),
    threshold("internal_replay_reconstruction", "replays_per_second"),
    threshold("serialization", "serializations_per_second"),
    threshold("random_playout", "games_per_second"),
    threshold("level0_bot_decision", "decisions_per_second"),
];

const fn threshold(operation: &'static str, unit: &'static str) -> Threshold {
    Threshold {
        operation,
        unit,
        threshold: 1.0,
        rationale_class: "baseline_pending_non_blocking",
        caveat: "Smoke floor until high_card_duel has stable repeated CI measurements.",
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
    println!("BEGIN_HIGH_CARD_DUEL_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&metadata, &results));
    println!("END_HIGH_CARD_DUEL_BENCHMARK_JSON");
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
            "standard_setup_shuffle",
            "setups",
            20_000,
            bench_standard_setup_shuffle,
        ),
        (
            "legal_actions_lead",
            "trees",
            50_000,
            bench_legal_actions_lead,
        ),
        (
            "legal_actions_reply",
            "trees",
            50_000,
            bench_legal_actions_reply,
        ),
        (
            "validate_commit",
            "validations",
            50_000,
            bench_validate_commit,
        ),
        ("apply_commit", "actions", 50_000, bench_apply_commit),
        (
            "apply_reveal_refill",
            "actions",
            50_000,
            bench_apply_reveal_refill,
        ),
        (
            "public_view_generation",
            "views",
            50_000,
            bench_public_view_generation,
        ),
        (
            "seat_private_view_generation",
            "views",
            50_000,
            bench_seat_private_view_generation,
        ),
        (
            "effect_filtering",
            "filters",
            50_000,
            bench_effect_filtering,
        ),
        (
            "public_replay_export",
            "exports",
            5_000,
            bench_public_replay_export,
        ),
        (
            "internal_replay_reconstruction",
            "replays",
            5_000,
            bench_internal_replay_reconstruction,
        ),
        (
            "serialization",
            "serializations",
            25_000,
            bench_serialization,
        ),
        ("random_playout", "games", 5_000, bench_random_playout),
        (
            "level0_bot_decision",
            "decisions",
            50_000,
            bench_level0_bot_decision,
        ),
    ];

    benches
        .into_iter()
        .filter(|(name, _, _, _)| filter.is_none_or(|filter| name.contains(filter)))
        .map(|(name, unit, iterations, benchmark)| measure(name, unit, iterations, benchmark))
        .collect()
}

fn print_human_summary(results: &[BenchResult]) {
    println!("high_card_duel native benchmarks");
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

fn bench_standard_setup_shuffle(iterations: u64) {
    for index in 0..iterations {
        black_box(setup(Seed(index)));
    }
}

fn bench_legal_actions_lead(iterations: u64) {
    let state = setup(Seed(7));
    let actor = actor_for_state(&state);
    for _ in 0..iterations {
        black_box(legal_action_tree(&state, &actor));
    }
}

fn bench_legal_actions_reply(iterations: u64) {
    let state = reply_state();
    let actor = actor_for_state(&state);
    for _ in 0..iterations {
        black_box(legal_action_tree(&state, &actor));
    }
}

fn bench_validate_commit(iterations: u64) {
    let state = setup(Seed(7));
    let command = first_command(&state);
    for _ in 0..iterations {
        black_box(validate_command(&state, &command).expect("command validates"));
    }
}

fn bench_apply_commit(iterations: u64) {
    let state = setup(Seed(7));
    let action = validate_command(&state, &first_command(&state)).expect("command validates");
    for _ in 0..iterations {
        let mut state = state.clone();
        black_box(apply_action(&mut state, action));
    }
}

fn bench_apply_reveal_refill(iterations: u64) {
    let state = reply_state();
    let action = validate_command(&state, &first_command(&state)).expect("command validates");
    for _ in 0..iterations {
        let mut state = state.clone();
        black_box(apply_action(&mut state, action));
    }
}

fn bench_public_view_generation(iterations: u64) {
    let state = reply_state();
    for _ in 0..iterations {
        black_box(project_view(&state, &Viewer { seat_id: None }));
    }
}

fn bench_seat_private_view_generation(iterations: u64) {
    let state = reply_state();
    let viewer = Viewer {
        seat_id: Some(state.seats[HighCardDuelSeat::Seat0.index()].clone()),
    };
    for _ in 0..iterations {
        black_box(project_view(&state, &viewer));
    }
}

fn bench_effect_filtering(iterations: u64) {
    let mut state = setup(Seed(7));
    let action = validate_command(&state, &first_command(&state)).expect("command validates");
    let mut log = EffectLog::new();
    for effect in apply_action(&mut state, action) {
        log.push(effect);
    }
    let viewer = Viewer {
        seat_id: Some(state.seats[HighCardDuelSeat::Seat0.index()].clone()),
    };
    for _ in 0..iterations {
        black_box(log.since(EffectCursor(0), &viewer));
    }
}

fn bench_public_replay_export(iterations: u64) {
    let trace = generate_internal_full_trace(7);
    for _ in 0..iterations {
        black_box(export_public_observer_replay(&trace));
    }
}

fn bench_internal_replay_reconstruction(iterations: u64) {
    let trace = generate_internal_full_trace(7);
    for _ in 0..iterations {
        black_box(replay_internal_full_trace(&trace));
    }
}

fn bench_serialization(iterations: u64) {
    let trace = generate_internal_full_trace(7);
    let export = export_public_observer_replay(&trace);
    for _ in 0..iterations {
        black_box(export.to_json());
        black_box(export.stable_hash());
    }
}

fn bench_random_playout(iterations: u64) {
    for index in 0..iterations {
        black_box(playout(Seed(index)));
    }
}

fn bench_level0_bot_decision(iterations: u64) {
    let state = setup(Seed(7));
    let actor = active_commit_seat(&state).expect("initial state has active seat");
    for index in 0..iterations {
        let bot = HighCardDuelRandomBot::new(Seed(index));
        black_box(bot.select_action(&state, actor).expect("bot chooses"));
    }
}

fn setup(seed: Seed) -> HighCardDuelState {
    setup_match(
        seed,
        &high_card_duel::default_seats(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn reply_state() -> HighCardDuelState {
    let mut state = setup(Seed(7));
    let action = validate_command(&state, &first_command(&state)).expect("command validates");
    apply_action(&mut state, action);
    assert_eq!(state.phase, Phase::ReplyCommit);
    state
}

fn first_command(state: &HighCardDuelState) -> CommandEnvelope {
    let actor_seat = active_commit_seat(state).expect("state has active seat");
    let card = state.hand_for(actor_seat)[0];
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        },
        action_path: engine_core::ActionPath {
            segments: vec![commit_segment(card)],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn actor_for_state(state: &HighCardDuelState) -> Actor {
    let actor_seat = active_commit_seat(state).expect("state has active seat");
    Actor {
        seat_id: state.seats[actor_seat.index()].clone(),
    }
}

fn playout(seed: Seed) -> Option<HighCardDuelSeat> {
    let mut state = setup(seed);
    let mut action_index = 0_u64;
    while state.phase != Phase::Terminal {
        let actor_seat = active_commit_seat(&state).expect("non-terminal state has active seat");
        let bot = HighCardDuelRandomBot::new(Seed(seed.0.wrapping_add(action_index)));
        let action_path = bot
            .select_action(&state, actor_seat)
            .expect("bot chooses legal action");
        let command = CommandEnvelope {
            actor: Actor {
                seat_id: state.seats[actor_seat.index()].clone(),
            },
            action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        };
        let action = validate_command(&state, &command).expect("bot command validates");
        apply_action(&mut state, action);
        action_index += 1;
    }
    match state.terminal_outcome {
        Some(high_card_duel::TerminalOutcome::Win { seat }) => Some(seat),
        Some(high_card_duel::TerminalOutcome::Draw) | None => None,
    }
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}
