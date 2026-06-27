use std::{
    env,
    hint::black_box,
    time::{Duration, Instant},
};

use engine_core::{
    ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, StableSerialize, Viewer,
};
use starbridge_crossing::{
    apply_jump_command, apply_pass_blocked_command, apply_step_command, legal_action_paths,
    legal_action_tree, parse_action_path, project_view, replay_commands, setup_match,
    StarbridgeAction, StarbridgeCrossingL0Bot, StarbridgeState, DATA_VERSION_LABEL, GAME_ID,
    RULES_VERSION_LABEL,
};

const ENGINE_VERSION: &str = "engine-core-0.1.0";
const BUILD_PROFILE: &str = "bench";
const REPORT_SCHEMA_VERSION: u32 = 1;

struct BenchResult {
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    elapsed: Duration,
}

struct BenchSpec {
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    benchmark: fn(u64),
}

struct Threshold {
    operation: &'static str,
    unit: &'static str,
    threshold: f64,
    rationale_class: &'static str,
    caveat: &'static str,
}

impl BenchResult {
    fn current_value(&self) -> f64 {
        self.iterations as f64 / self.elapsed.as_secs_f64()
    }
}

fn main() {
    let filter = operation_filter();
    let results = run_benchmarks(filter.as_deref());

    print_human_summary(&results);
    println!("BEGIN_STARBRIDGE_CROSSING_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&results));
    println!("END_STARBRIDGE_CROSSING_BENCHMARK_JSON");
}

fn operation_filter() -> Option<String> {
    env::args().skip(1).find(|arg| {
        !arg.starts_with('-')
            && threshold_catalog()
                .iter()
                .any(|threshold| threshold.operation.contains(arg))
    })
}

fn run_benchmarks(filter: Option<&str>) -> Vec<BenchResult> {
    bench_specs()
        .into_iter()
        .filter(|spec| filter.is_none_or(|filter| spec.name.contains(filter)))
        .map(measure)
        .collect()
}

fn bench_specs() -> Vec<BenchSpec> {
    vec![
        spec("setup_121_spaces_2p", "setups", 2_000, bench_setup_2p),
        spec("setup_121_spaces_3p", "setups", 2_000, bench_setup_3p),
        spec("setup_121_spaces_4p", "setups", 2_000, bench_setup_4p),
        spec("setup_121_spaces_6p", "setups", 2_000, bench_setup_6p),
        spec(
            "legal_actions_start_6p",
            "trees",
            5_000,
            bench_legal_actions_start_6p,
        ),
        spec(
            "legal_actions_midgame_6p",
            "trees",
            2_000,
            bench_legal_actions_midgame_6p,
        ),
        spec(
            "jump_chain_enumeration_dense_6p",
            "trees",
            2_000,
            bench_jump_chain_enumeration_dense_6p,
        ),
        spec(
            "apply_single_step_6p",
            "actions",
            2_000,
            bench_apply_single_step_6p,
        ),
        spec(
            "apply_multi_hop_6p",
            "actions",
            1_000,
            bench_apply_multi_hop_6p,
        ),
        spec(
            "apply_blocked_pass_6p",
            "actions",
            2_000,
            bench_apply_blocked_pass_6p,
        ),
        spec(
            "simulate_l0_6p_64_actions",
            "playouts",
            100,
            bench_simulate_l0_6p_64_actions,
        ),
        spec(
            "serialize_public_view_6p",
            "views",
            5_000,
            bench_serialize_public_view_6p,
        ),
        spec(
            "replay_full_trace_6p",
            "replays",
            1_000,
            bench_replay_full_trace_6p,
        ),
        spec(
            "wasm_public_view_bridge_6p",
            "views",
            5_000,
            bench_wasm_public_view_bridge_6p,
        ),
    ]
}

fn spec(name: &'static str, unit: &'static str, iterations: u64, benchmark: fn(u64)) -> BenchSpec {
    BenchSpec {
        name,
        unit,
        iterations,
        benchmark,
    }
}

fn measure(spec: BenchSpec) -> BenchResult {
    let started = Instant::now();
    (spec.benchmark)(spec.iterations);
    BenchResult {
        name: spec.name,
        unit: spec.unit,
        iterations: spec.iterations,
        elapsed: started.elapsed(),
    }
}

fn bench_setup_2p(iterations: u64) {
    bench_setup(iterations, 2);
}

fn bench_setup_3p(iterations: u64) {
    bench_setup(iterations, 3);
}

fn bench_setup_4p(iterations: u64) {
    bench_setup(iterations, 4);
}

fn bench_setup_6p(iterations: u64) {
    bench_setup(iterations, 6);
}

fn bench_setup(iterations: u64, seat_count: usize) {
    for index in 0..iterations {
        black_box(setup_state(20_000 + index, seat_count));
    }
}

fn bench_legal_actions_start_6p(iterations: u64) {
    let state = setup_state(20_100, 6);
    for _ in 0..iterations {
        black_box(action_tree_for_active(&state));
    }
}

fn bench_legal_actions_midgame_6p(iterations: u64) {
    let state = playout_state(20_200, 6, 48);
    for _ in 0..iterations {
        black_box(action_tree_for_active(&state));
    }
}

fn bench_jump_chain_enumeration_dense_6p(iterations: u64) {
    let state = state_with_jump(20_300, 6);
    for _ in 0..iterations {
        let tree = action_tree_for_active(&state);
        black_box(legal_action_paths(&tree));
    }
}

fn bench_apply_single_step_6p(iterations: u64) {
    for index in 0..iterations {
        let mut state = setup_state(20_400 + index, 6);
        let path = first_path_with_segment(&action_tree_for_active(&state), "step")
            .expect("opening state has a step");
        black_box(apply_path(&mut state, path));
    }
}

fn bench_apply_multi_hop_6p(iterations: u64) {
    let base = state_with_jump(20_500, 6);
    let path = first_path_with_segment(&action_tree_for_active(&base), "jump")
        .expect("prepared state has a jump");
    for _ in 0..iterations {
        let mut state = base.clone();
        black_box(apply_path(&mut state, path.clone()));
    }
}

fn bench_apply_blocked_pass_6p(iterations: u64) {
    for _ in 0..iterations {
        let mut state = blocked_state(20_600, 6);
        black_box(apply_path(&mut state, vec!["pass_blocked".to_owned()]));
    }
}

fn bench_simulate_l0_6p_64_actions(iterations: u64) {
    for index in 0..iterations {
        black_box(playout_state(20_700 + index, 6, 64));
    }
}

fn bench_serialize_public_view_6p(iterations: u64) {
    let state = playout_state(20_800, 6, 32);
    let view = project_view(&state, &Viewer { seat_id: None });
    for _ in 0..iterations {
        black_box(view.stable_hash());
    }
}

fn bench_replay_full_trace_6p(iterations: u64) {
    let commands = trace_commands(20_900, 6, 24);
    for _ in 0..iterations {
        black_box(replay_commands(20_900, 6, &commands).expect("trace replays"));
    }
}

fn bench_wasm_public_view_bridge_6p(iterations: u64) {
    let state = playout_state(21_000, 6, 32);
    for _ in 0..iterations {
        let view = project_view(&state, &Viewer { seat_id: None });
        black_box(view.stable_hash());
    }
}

fn setup_state(seed: u64, seat_count: usize) -> StarbridgeState {
    setup_match(Seed(seed), &seat_ids(seat_count), &Default::default()).expect("starbridge setup")
}

fn seat_ids(seat_count: usize) -> Vec<SeatId> {
    (0..seat_count)
        .map(|index| SeatId::from_zero_based_index(index as u32))
        .collect()
}

fn action_tree_for_active(state: &StarbridgeState) -> engine_core::ActionTree {
    legal_action_tree(state, &actor_for_active(state))
}

fn actor_for_active(state: &StarbridgeState) -> Actor {
    let seat = state
        .seats
        .get(usize::from(state.active_seat_index))
        .expect("active seat exists");
    Actor {
        seat_id: seat.seat_id.clone(),
    }
}

fn command_for_active(state: &StarbridgeState, path: Vec<String>) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_active(state),
        action_path: ActionPath { segments: path },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn apply_path(
    state: &mut StarbridgeState,
    path: Vec<String>,
) -> Vec<starbridge_crossing::StarbridgeEffectEnvelope> {
    let command = command_for_active(state, path);
    match parse_action_path(&command.action_path.segments).expect("bench path parses") {
        StarbridgeAction::Step { .. } => apply_step_command(state, &command).expect("step applies"),
        StarbridgeAction::Jump { .. } => apply_jump_command(state, &command).expect("jump applies"),
        StarbridgeAction::PassBlocked => {
            apply_pass_blocked_command(state, &command).expect("blocked pass applies")
        }
    }
}

fn first_path_with_segment(tree: &engine_core::ActionTree, segment: &str) -> Option<Vec<String>> {
    legal_action_paths(tree)
        .into_iter()
        .map(|path| path.segments)
        .find(|path| path.iter().any(|candidate| candidate == segment))
}

fn playout_state(seed: u64, seat_count: usize, action_cap: u64) -> StarbridgeState {
    let mut state = setup_state(seed, seat_count);
    for index in 0..action_cap {
        if state.terminal_status.is_some() {
            break;
        }
        let path = StarbridgeCrossingL0Bot::new(Seed(seed + index))
            .select_action(&state)
            .expect("l0 action");
        apply_path(&mut state, path.segments);
    }
    state
}

fn state_with_jump(seed: u64, seat_count: usize) -> StarbridgeState {
    let mut state = setup_state(seed, seat_count);
    for index in 0..256 {
        if first_path_with_segment(&action_tree_for_active(&state), "jump").is_some() {
            return state;
        }
        let path = StarbridgeCrossingL0Bot::new(Seed(seed + index))
            .select_action(&state)
            .expect("l0 action while seeking jump");
        apply_path(&mut state, path.segments);
    }
    panic!("could not prepare Starbridge state with a jump path");
}

fn blocked_state(seed: u64, seat_count: usize) -> StarbridgeState {
    let mut state = setup_state(seed, seat_count);
    state.occupancy.fill(None);
    state
        .pegs
        .retain(|peg| peg.owner_seat_index != state.active_seat_index);
    state
}

fn trace_commands(seed: u64, seat_count: usize, action_cap: u64) -> Vec<Vec<String>> {
    let mut state = setup_state(seed, seat_count);
    let mut commands = Vec::new();
    for index in 0..action_cap {
        if state.terminal_status.is_some() {
            break;
        }
        let path = StarbridgeCrossingL0Bot::new(Seed(seed + index))
            .select_action(&state)
            .expect("l0 action for trace");
        let segments = path.segments;
        apply_path(&mut state, segments.clone());
        commands.push(segments);
    }
    commands
}

fn print_human_summary(results: &[BenchResult]) {
    println!("starbridge_crossing native benchmarks");
    println!("operation,unit,iterations,total_ms,current_per_second");
    for result in results {
        println!(
            "{},{},{},{:.3},{:.3}",
            result.name,
            result.unit,
            result.iterations,
            result.elapsed.as_secs_f64() * 1000.0,
            result.current_value()
        );
    }
}

fn benchmark_report_json(results: &[BenchResult]) -> String {
    let thresholds = threshold_catalog();
    let results_json = results
        .iter()
        .map(|result| {
            let threshold = thresholds
                .iter()
                .find(|threshold| threshold.operation == result.name)
                .expect("bench operation has threshold metadata");
            format!(
                "{{\"operation_name\":\"{}\",\"unit\":\"{}\",\"iterations\":{},\"elapsed_ms\":{:.6},\"current_value\":{:.6},\"threshold\":{},\"pass\":{},\"rationale_class\":\"{}\",\"caveat\":\"{}\"}}",
                result.name,
                threshold.unit,
                result.iterations,
                result.elapsed.as_secs_f64() * 1000.0,
                result.current_value(),
                threshold.threshold,
                result.current_value() >= threshold.threshold,
                threshold.rationale_class,
                threshold.caveat
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version\":\"{}\",\"data_version\":\"{}\",\"engine_version\":\"{}\",\"build_profile\":\"{}\",\"results\":[{}]}}",
        REPORT_SCHEMA_VERSION,
        GAME_ID,
        RULES_VERSION_LABEL,
        DATA_VERSION_LABEL,
        ENGINE_VERSION,
        BUILD_PROFILE,
        results_json
    )
}

fn threshold_catalog() -> Vec<Threshold> {
    vec![
        threshold(
            "setup_121_spaces_2p",
            "setups_per_second",
            "2-seat setup over the 121-space topology.",
        ),
        threshold(
            "setup_121_spaces_3p",
            "setups_per_second",
            "3-seat setup over the 121-space topology.",
        ),
        threshold(
            "setup_121_spaces_4p",
            "setups_per_second",
            "4-seat setup over the 121-space topology.",
        ),
        threshold(
            "setup_121_spaces_6p",
            "setups_per_second",
            "6-seat setup over the 121-space topology.",
        ),
        threshold(
            "legal_actions_start_6p",
            "trees_per_second",
            "Opening legal action tree at max seat count.",
        ),
        threshold(
            "legal_actions_midgame_6p",
            "trees_per_second",
            "Midgame legal action tree after deterministic L0 actions.",
        ),
        threshold(
            "jump_chain_enumeration_dense_6p",
            "trees_per_second",
            "Legal path collection from a prepared jump-bearing state.",
        ),
        threshold(
            "apply_single_step_6p",
            "actions_per_second",
            "Apply a Rust-validated single-step command.",
        ),
        threshold(
            "apply_multi_hop_6p",
            "actions_per_second",
            "Apply a Rust-validated jump command from a prepared state.",
        ),
        threshold(
            "apply_blocked_pass_6p",
            "actions_per_second",
            "Apply blocked-pass command on an active seat with no pegs.",
        ),
        threshold(
            "simulate_l0_6p_64_actions",
            "playouts_per_second",
            "Bounded 64-action max-seat L0 playout.",
        ),
        threshold(
            "serialize_public_view_6p",
            "views_per_second",
            "Stable serialization of the all-public max-seat view.",
        ),
        threshold(
            "replay_full_trace_6p",
            "replays_per_second",
            "Replay a deterministic 24-command max-seat trace.",
        ),
        threshold(
            "wasm_public_view_bridge_6p",
            "views_per_second",
            "Rust public-view projection and stable serialization consumed by the WASM bridge.",
        ),
    ]
}

fn threshold(operation: &'static str, unit: &'static str, caveat: &'static str) -> Threshold {
    Threshold {
        operation,
        unit,
        threshold: 1.0,
        rationale_class: "baseline_pending_non_blocking",
        caveat,
    }
}
