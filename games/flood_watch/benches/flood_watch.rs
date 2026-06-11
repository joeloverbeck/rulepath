use std::{
    env,
    hint::black_box,
    time::{Duration, Instant},
};

use engine_core::{
    ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, Seed, Viewer,
};
use flood_watch::{
    apply_command, export_public_replay, legal_action_tree, project_view, public_replay_step,
    setup_match, validate_command, FloodWatchLevel1Bot, SetupOptions, ACTION_BAIL, ACTION_END_TURN,
    ACTION_REINFORCE, GAME_ID, RULES_VERSION_LABEL,
};

const DATA_VERSION: &str = "1";
const ENGINE_VERSION: &str = "engine-core-0.1.0";
const REPORT_SCHEMA_VERSION: u32 = 1;

struct BenchResult {
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    elapsed: Duration,
}

type BenchSpec = (&'static str, &'static str, u64, fn(u64));

const OPERATIONS: &[(&str, &str)] = &[
    ("legal_actions_action_phase", "trees_per_second"),
    ("validate_action", "validations_per_second"),
    ("apply_bail", "actions_per_second"),
    ("apply_reinforce", "actions_per_second"),
    ("apply_end_turn_environment_phase", "actions_per_second"),
    ("project_public_view_midgame", "views_per_second"),
    ("state_hash_terminal", "hashes_per_second"),
    ("public_export_timeline", "exports_per_second"),
    ("level1_bot_decision", "decisions_per_second"),
    ("random_playout", "playouts_per_second"),
];

impl BenchResult {
    fn current_value(&self) -> f64 {
        self.iterations as f64 / self.elapsed.as_secs_f64()
    }
}

fn main() {
    let filter = operation_filter();
    let results = run_benchmarks(filter.as_deref());

    println!("flood_watch native benchmarks");
    println!("operation,iterations,unit,elapsed_ms,per_second,threshold,pass");
    for result in &results {
        let current = result.current_value();
        println!(
            "{},{},{},{:.3},{:.2},1.00,{}",
            result.name,
            result.iterations,
            result.unit,
            result.elapsed.as_secs_f64() * 1_000.0,
            current,
            current >= 1.0
        );
    }
    println!("BEGIN_FLOOD_WATCH_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&results));
    println!("END_FLOOD_WATCH_BENCHMARK_JSON");
}

fn operation_filter() -> Option<String> {
    env::args().skip(1).find(|arg| {
        !arg.starts_with('-')
            && !arg.chars().all(|ch| ch.is_ascii_digit())
            && OPERATIONS
                .iter()
                .any(|(operation, _)| operation.contains(arg))
    })
}

fn run_benchmarks(filter: Option<&str>) -> Vec<BenchResult> {
    let benches: Vec<BenchSpec> = vec![
        (
            "legal_actions_action_phase",
            "trees",
            20_000,
            bench_legal_actions_action_phase,
        ),
        (
            "validate_action",
            "validations",
            20_000,
            bench_validate_action,
        ),
        ("apply_bail", "actions", 20_000, bench_apply_bail),
        ("apply_reinforce", "actions", 20_000, bench_apply_reinforce),
        (
            "apply_end_turn_environment_phase",
            "actions",
            10_000,
            bench_apply_end_turn_environment_phase,
        ),
        (
            "project_public_view_midgame",
            "views",
            20_000,
            bench_project_public_view_midgame,
        ),
        (
            "state_hash_terminal",
            "hashes",
            20_000,
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
            20_000,
            bench_level1_bot_decision,
        ),
        ("random_playout", "playouts", 100, bench_random_playout),
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

fn bench_legal_actions_action_phase(iterations: u64) {
    let state = setup();
    let actor = actor("seat_0");
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_validate_action(iterations: u64) {
    let state = setup();
    let command = command(
        &state,
        "seat_0",
        vec![ACTION_REINFORCE, "district_riverside"],
    );
    for _ in 0..iterations {
        black_box(validate_command(black_box(&state), black_box(&command)).unwrap());
    }
}

fn bench_apply_bail(iterations: u64) {
    let base = setup();
    for _ in 0..iterations {
        let mut state = base.clone();
        let command = command(&state, "seat_0", vec![ACTION_BAIL, "district_old_docks"]);
        black_box(apply_command(black_box(&mut state), black_box(&command)).unwrap());
    }
}

fn bench_apply_reinforce(iterations: u64) {
    let base = setup();
    for _ in 0..iterations {
        let mut state = base.clone();
        let command = command(
            &state,
            "seat_0",
            vec![ACTION_REINFORCE, "district_riverside"],
        );
        black_box(apply_command(black_box(&mut state), black_box(&command)).unwrap());
    }
}

fn bench_apply_end_turn_environment_phase(iterations: u64) {
    let base = setup();
    for _ in 0..iterations {
        let mut state = base.clone();
        let command = command(&state, "seat_0", vec![ACTION_END_TURN]);
        black_box(apply_command(black_box(&mut state), black_box(&command)).unwrap());
    }
}

fn bench_project_public_view_midgame(iterations: u64) {
    let state = state_after_reinforce();
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(project_view(black_box(&state), black_box(&viewer)));
    }
}

fn bench_state_hash_terminal(iterations: u64) {
    let state = terminal_state();
    for _ in 0..iterations {
        black_box(HashValue::from_stable_bytes(
            black_box(&state.stable_summary()).as_bytes(),
        ));
    }
}

fn bench_public_export_timeline(iterations: u64) {
    let mut state = setup();
    let command = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &command).expect("environment applies");
    let viewer = Viewer { seat_id: None };
    let step = public_replay_step(0, &state, &command, &applied.effects, &viewer);
    for _ in 0..iterations {
        black_box(export_public_replay(
            black_box(state.variant.id.clone()),
            black_box(&viewer),
            black_box(vec![step.clone()]),
        ));
    }
}

fn bench_level1_bot_decision(iterations: u64) {
    let state = setup();
    let bot = FloodWatchLevel1Bot::new(Seed(7));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(&state.seats[0]))
                .unwrap(),
        );
    }
}

fn bench_random_playout(iterations: u64) {
    for seed in 0..iterations {
        let mut state =
            setup_match(Seed(seed), &seats(), &SetupOptions::default()).expect("setup succeeds");
        let mut turns = 0;
        while state.terminal_outcome.is_none() && turns < 80 {
            let active = state.active_seat.clone();
            let decision = FloodWatchLevel1Bot::new(Seed(seed + turns))
                .select_decision(&state, &active)
                .expect("bot decision");
            let command = CommandEnvelope {
                actor: actor(&active.0),
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(1),
            };
            black_box(apply_command(&mut state, &command).expect("bot action applies"));
            turns += 1;
        }
        black_box(state.terminal_outcome);
    }
}

fn setup() -> flood_watch::FloodWatchState {
    setup_match(Seed(12), &seats(), &SetupOptions::default()).expect("setup succeeds")
}

fn state_after_reinforce() -> flood_watch::FloodWatchState {
    let mut state = setup();
    let command = command(
        &state,
        "seat_0",
        vec![ACTION_REINFORCE, "district_riverside"],
    );
    apply_command(&mut state, &command).expect("reinforce applies");
    state
}

fn terminal_state() -> flood_watch::FloodWatchState {
    let mut state = setup();
    let command = command(&state, "seat_0", vec![ACTION_END_TURN]);
    apply_command(&mut state, &command).expect("environment applies");
    state
}

fn command(
    state: &flood_watch::FloodWatchState,
    seat: &str,
    segments: Vec<&str>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: segments.into_iter().map(str::to_owned).collect(),
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn benchmark_report_json(results: &[BenchResult]) -> String {
    let operations = results
        .iter()
        .map(|result| {
            format!(
                "{{\"operation_name\":\"{}\",\"unit\":\"{}\",\"current_value\":{:.6}}}",
                result.name,
                unit_for(result.name),
                result.current_value()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version\":\"{}\",\"data_version\":\"{}\",\"engine_version\":\"{}\",\"build_profile\":\"bench\",\"command\":\"cargo bench -p flood_watch\",\"os\":\"unknown\",\"rust_version\":\"unknown\",\"hardware_environment_notes\":\"uncontrolled smoke runner\",\"operations\":[{}]}}",
        REPORT_SCHEMA_VERSION, GAME_ID, RULES_VERSION_LABEL, DATA_VERSION, ENGINE_VERSION, operations
    )
}

fn unit_for(operation: &str) -> &'static str {
    OPERATIONS
        .iter()
        .find(|(candidate, _)| *candidate == operation)
        .map(|(_, unit)| *unit)
        .expect("bench operation has threshold metadata")
}
