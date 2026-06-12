use std::{
    env,
    hint::black_box,
    time::{Duration, Instant},
};

use engine_core::{
    ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, Seed, StableSerialize,
    Viewer,
};
use frontier_control::{
    apply_command, command_for_decision, legal_action_tree, project_view, setup_match,
    validate_command, FactionId, FrontierGarrisonLevel1Bot, FrontierProspectorLevel1Bot,
    SetupOptions, ACTION_END_TURN, ACTION_MARCH, GAME_ID, RULES_VERSION_LABEL,
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
    ("legal_actions_garrison_midgame", "trees_per_second"),
    ("legal_actions_prospectors_midgame", "trees_per_second"),
    ("validate_action", "validations_per_second"),
    ("apply_march_with_clash", "actions_per_second"),
    ("apply_end_turn_round_scoring", "actions_per_second"),
    ("supply_connectivity_traversal", "scores_per_second"),
    ("project_public_view_midgame", "views_per_second"),
    ("state_hash_terminal", "hashes_per_second"),
    ("garrison_level1_bot_decision", "decisions_per_second"),
    ("prospector_level1_bot_decision", "decisions_per_second"),
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

    println!("frontier_control native benchmarks");
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
    println!("BEGIN_FRONTIER_CONTROL_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&results));
    println!("END_FRONTIER_CONTROL_BENCHMARK_JSON");
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
            "legal_actions_garrison_midgame",
            "trees",
            20_000,
            bench_legal_actions_garrison_midgame,
        ),
        (
            "legal_actions_prospectors_midgame",
            "trees",
            20_000,
            bench_legal_actions_prospectors_midgame,
        ),
        (
            "validate_action",
            "validations",
            20_000,
            bench_validate_action,
        ),
        (
            "apply_march_with_clash",
            "actions",
            20_000,
            bench_apply_march_with_clash,
        ),
        (
            "apply_end_turn_round_scoring",
            "actions",
            20_000,
            bench_apply_end_turn_round_scoring,
        ),
        (
            "supply_connectivity_traversal",
            "scores",
            20_000,
            bench_supply_connectivity_traversal,
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
            "garrison_level1_bot_decision",
            "decisions",
            20_000,
            bench_garrison_level1_bot_decision,
        ),
        (
            "prospector_level1_bot_decision",
            "decisions",
            20_000,
            bench_prospector_level1_bot_decision,
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

fn bench_legal_actions_garrison_midgame(iterations: u64) {
    let state = garrison_state();
    let actor = actor("seat_0");
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_legal_actions_prospectors_midgame(iterations: u64) {
    let state = setup();
    let actor = actor("seat_1");
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_validate_action(iterations: u64) {
    let state = setup();
    let command = command(
        &state,
        "seat_1",
        vec![ACTION_MARCH, "site_base_camp", "site_ford"],
    );
    for _ in 0..iterations {
        black_box(validate_command(black_box(&state), black_box(&command)).unwrap());
    }
}

fn bench_apply_march_with_clash(iterations: u64) {
    let base = crew_at_ford_state();
    for _ in 0..iterations {
        let mut state = base.clone();
        let command = command(
            &state,
            "seat_1",
            vec![ACTION_MARCH, "site_ford", "site_gatehouse"],
        );
        black_box(apply_command(black_box(&mut state), black_box(&command)).unwrap());
    }
}

fn bench_apply_end_turn_round_scoring(iterations: u64) {
    let base = garrison_state();
    for _ in 0..iterations {
        let mut state = base.clone();
        let command = command(&state, "seat_0", vec![ACTION_END_TURN]);
        black_box(apply_command(black_box(&mut state), black_box(&command)).unwrap());
    }
}

fn bench_supply_connectivity_traversal(iterations: u64) {
    bench_apply_end_turn_round_scoring(iterations);
}

fn bench_project_public_view_midgame(iterations: u64) {
    let state = crew_at_ford_state();
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(project_view(black_box(&state), black_box(&viewer)));
    }
}

fn bench_state_hash_terminal(iterations: u64) {
    let state = terminal_state();
    for _ in 0..iterations {
        black_box(HashValue::from_stable_bytes(black_box(
            &state.stable_bytes(),
        )));
    }
}

fn bench_garrison_level1_bot_decision(iterations: u64) {
    let state = garrison_state();
    let bot = FrontierGarrisonLevel1Bot::new(Seed(7));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(&state.seats[0]))
                .unwrap(),
        );
    }
}

fn bench_prospector_level1_bot_decision(iterations: u64) {
    let state = setup();
    let bot = FrontierProspectorLevel1Bot::new(Seed(7));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(&state.seats[1]))
                .unwrap(),
        );
    }
}

fn bench_random_playout(iterations: u64) {
    for seed in 0..iterations {
        let mut state = setup();
        let mut turns = 0;
        while state.terminal_outcome.is_none() && turns < 100 {
            let active = state.active_faction;
            let seat = state.active_seat().expect("active seat").clone();
            let decision = match active {
                FactionId::Garrison => FrontierGarrisonLevel1Bot::new(Seed(seed + turns))
                    .select_decision(&state, &seat)
                    .expect("garrison decision"),
                FactionId::Prospectors => FrontierProspectorLevel1Bot::new(Seed(seed + turns))
                    .select_decision(&state, &seat)
                    .expect("prospector decision"),
            };
            let command = command_for_decision(&state, &seat, &decision);
            black_box(apply_command(&mut state, &command).expect("bot action applies"));
            turns += 1;
        }
        black_box(state.terminal_outcome.expect("playout reaches terminal"));
    }
}

fn setup() -> frontier_control::FrontierControlState {
    setup_match(&seats(), &SetupOptions::default()).expect("setup succeeds")
}

fn garrison_state() -> frontier_control::FrontierControlState {
    let mut state = setup();
    let command = command(&state, "seat_1", vec![ACTION_END_TURN]);
    apply_command(&mut state, &command).expect("advance to garrison");
    state
}

fn crew_at_ford_state() -> frontier_control::FrontierControlState {
    let mut state = setup();
    let command = command(
        &state,
        "seat_1",
        vec![ACTION_MARCH, "site_base_camp", "site_ford"],
    );
    apply_command(&mut state, &command).expect("march applies");
    state
}

fn terminal_state() -> frontier_control::FrontierControlState {
    let mut state = garrison_state();
    state.round_number = state.variant.round_count;
    let command = command(&state, "seat_0", vec![ACTION_END_TURN]);
    apply_command(&mut state, &command).expect("terminal applies");
    state
}

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn command(
    state: &frontier_control::FrontierControlState,
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

fn benchmark_report_json(results: &[BenchResult]) -> String {
    format!(
        "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version\":\"{}\",\"data_version\":\"{}\",\"engine_version\":\"{}\",\"operations\":[{}]}}",
        REPORT_SCHEMA_VERSION,
        GAME_ID,
        RULES_VERSION_LABEL,
        DATA_VERSION,
        ENGINE_VERSION,
        results
            .iter()
            .map(bench_result_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn bench_result_json(result: &BenchResult) -> String {
    format!(
        "{{\"operation_name\":\"{}\",\"unit\":\"{}_per_second\",\"iterations\":{},\"elapsed_ms\":{:.3},\"current_value\":{:.6}}}",
        result.name,
        result.unit,
        result.iterations,
        result.elapsed.as_secs_f64() * 1_000.0,
        result.current_value()
    )
}
