use std::{
    env,
    hint::black_box,
    time::{Duration, Instant},
};

use engine_core::{
    ActionPath, Actor, CommandEnvelope, RulesVersion, Seed, StableSerialize, Viewer,
};
use vow_tide::{
    actions::{legal_action_tree, validate_bid_command, validate_play_command, ACTION_PLAY},
    bots::{VowTideL0Bot, VowTideL1Bot},
    ids::{canonical_seat_ids, VowTideSeat, ACTION_BID, GAME_ID, RULES_VERSION_LABEL},
    replay_support::{export_for_viewer, import_viewer_export, seat_viewer, snapshot},
    rules::{apply_bid, apply_play},
    scoring,
    setup::{setup_match, SetupOptions},
    state::{Phase, VowTideState},
    visibility::{filter_effects_for_viewer, project_view},
};

const DATA_VERSION: &str = "vow-tide-data-v1";
const ENGINE_VERSION: &str = "engine-core-0.1.0";
const BUILD_PROFILE: &str = "bench";
const REPORT_SCHEMA_VERSION: u32 = 1;
const SEAT_COUNTS: [usize; 5] = [3, 4, 5, 6, 7];

struct BenchResult {
    name: String,
    unit: &'static str,
    iterations: u64,
    elapsed: Duration,
}

struct BenchSpec {
    name: String,
    unit: &'static str,
    iterations: u64,
    seat_count: usize,
    benchmark: fn(u64, usize),
}

struct Threshold {
    operation: String,
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
    println!("BEGIN_VOW_TIDE_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&results));
    println!("END_VOW_TIDE_BENCHMARK_JSON");
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
    let mut specs = Vec::new();
    for seat_count in SEAT_COUNTS {
        let suffix = format!("{seat_count}p");
        specs.extend([
            spec(
                format!("setup_deal_{suffix}"),
                "setups",
                5_000,
                seat_count,
                bench_setup_deal,
            ),
            spec(
                format!("bid_legal_first_{suffix}"),
                "trees",
                10_000,
                seat_count,
                bench_bid_legal_first,
            ),
            spec(
                format!("bid_legal_dealer_hook_{suffix}"),
                "trees",
                10_000,
                seat_count,
                bench_bid_legal_dealer_hook,
            ),
            spec(
                format!("play_legal_lead_{suffix}"),
                "trees",
                10_000,
                seat_count,
                bench_play_legal_lead,
            ),
            spec(
                format!("play_legal_follow_{suffix}"),
                "trees",
                10_000,
                seat_count,
                bench_play_legal_follow,
            ),
            spec(
                format!("validate_apply_bid_{suffix}"),
                "actions",
                5_000,
                seat_count,
                bench_validate_apply_bid,
            ),
            spec(
                format!("validate_apply_play_{suffix}"),
                "actions",
                5_000,
                seat_count,
                bench_validate_apply_play,
            ),
            spec(
                format!("trick_resolution_{suffix}"),
                "tricks",
                2_000,
                seat_count,
                bench_trick_resolution,
            ),
            spec(
                format!("score_hand_{suffix}"),
                "scores",
                5_000,
                seat_count,
                bench_score_hand,
            ),
            spec(
                format!("project_observer_{suffix}"),
                "views",
                10_000,
                seat_count,
                bench_project_observer,
            ),
            spec(
                format!("project_all_seats_{suffix}"),
                "viewer_sets",
                5_000,
                seat_count,
                bench_project_all_seats,
            ),
            spec(
                format!("effect_filter_all_viewers_{suffix}"),
                "viewer_sets",
                5_000,
                seat_count,
                bench_effect_filter_all_viewers,
            ),
            spec(
                format!("replay_snapshot_export_import_{suffix}"),
                "exports",
                2_000,
                seat_count,
                bench_replay_snapshot_export_import,
            ),
            spec(
                format!("l0_decision_{suffix}"),
                "decisions",
                10_000,
                seat_count,
                bench_l0_decision,
            ),
            spec(
                format!("l1_decision_{suffix}"),
                "decisions",
                10_000,
                seat_count,
                bench_l1_decision,
            ),
            spec(
                format!("full_seeded_match_{suffix}"),
                "matches",
                50,
                seat_count,
                bench_full_seeded_match,
            ),
        ]);
    }
    specs
}

fn spec(
    name: String,
    unit: &'static str,
    iterations: u64,
    seat_count: usize,
    benchmark: fn(u64, usize),
) -> BenchSpec {
    BenchSpec {
        name,
        unit,
        iterations,
        seat_count,
        benchmark,
    }
}

fn measure(spec: BenchSpec) -> BenchResult {
    let started = Instant::now();
    (spec.benchmark)(spec.iterations, spec.seat_count);
    BenchResult {
        name: spec.name,
        unit: spec.unit,
        iterations: spec.iterations,
        elapsed: started.elapsed(),
    }
}

fn bench_setup_deal(iterations: u64, seat_count: usize) {
    for index in 0..iterations {
        black_box(setup_state(seat_count, 10_000 + index));
    }
}

fn bench_bid_legal_first(iterations: u64, seat_count: usize) {
    let state = setup_state(seat_count, 20_000);
    let actor = actor(state.active_seat().expect("bidding active"), &state);
    for _ in 0..iterations {
        black_box(legal_action_tree(&state, &actor));
    }
}

fn bench_bid_legal_dealer_hook(iterations: u64, seat_count: usize) {
    let state = dealer_hook_state(seat_count);
    let actor = actor(state.active_seat().expect("dealer active"), &state);
    for _ in 0..iterations {
        black_box(legal_action_tree(&state, &actor));
    }
}

fn bench_play_legal_lead(iterations: u64, seat_count: usize) {
    let state = playing_state(seat_count);
    let actor = actor(state.active_seat().expect("playing active"), &state);
    for _ in 0..iterations {
        black_box(legal_action_tree(&state, &actor));
    }
}

fn bench_play_legal_follow(iterations: u64, seat_count: usize) {
    let mut state = playing_state(seat_count);
    apply_first_legal(&mut state);
    let actor = actor(state.active_seat().expect("follow active"), &state);
    for _ in 0..iterations {
        black_box(legal_action_tree(&state, &actor));
    }
}

fn bench_validate_apply_bid(iterations: u64, seat_count: usize) {
    for index in 0..iterations {
        let mut state = setup_state(seat_count, 30_000 + index);
        let active = state.active_seat().expect("bidding active");
        let path = first_legal_path(&state, active);
        let bid =
            validate_bid_command(&state, &command(&state, active, path)).expect("bid validates");
        black_box(apply_bid(&mut state, bid).expect("bid applies"));
    }
}

fn bench_validate_apply_play(iterations: u64, seat_count: usize) {
    for index in 0..iterations {
        let mut state = playing_state_with_seed(seat_count, 40_000 + index);
        let active = state.active_seat().expect("playing active");
        let path = first_legal_path(&state, active);
        let play =
            validate_play_command(&state, &command(&state, active, path)).expect("play validates");
        black_box(apply_play(&mut state, play).expect("play applies"));
    }
}

fn bench_trick_resolution(iterations: u64, seat_count: usize) {
    let fixture = almost_complete_trick_state(seat_count);
    for _ in 0..iterations {
        let mut state = fixture.clone();
        apply_first_legal(&mut state);
        black_box(state.captured_tricks.len());
    }
}

fn bench_score_hand(iterations: u64, seat_count: usize) {
    let fixture = scoring_state(seat_count);
    for _ in 0..iterations {
        let mut state = fixture.clone();
        black_box(scoring::score_current_hand(&mut state).expect("score succeeds"));
    }
}

fn bench_project_observer(iterations: u64, seat_count: usize) {
    let state = playing_state(seat_count);
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(project_view(&state, &viewer).stable_bytes());
    }
}

fn bench_project_all_seats(iterations: u64, seat_count: usize) {
    let state = playing_state(seat_count);
    let viewers = seat_viewers(&state);
    for _ in 0..iterations {
        for viewer in &viewers {
            black_box(project_view(&state, viewer).stable_bytes());
        }
    }
}

fn bench_effect_filter_all_viewers(iterations: u64, seat_count: usize) {
    let (state, effects) = one_action_effects(seat_count);
    let mut viewers = vec![Viewer { seat_id: None }];
    viewers.extend(seat_viewers(&state));
    for _ in 0..iterations {
        for viewer in &viewers {
            black_box(filter_effects_for_viewer(&effects, viewer));
        }
    }
}

fn bench_replay_snapshot_export_import(iterations: u64, seat_count: usize) {
    let (state, effects) = one_action_effects(seat_count);
    let viewer = seat_viewer(state.seats[0].0.clone());
    for _ in 0..iterations {
        black_box(snapshot(&state, &effects));
        let export = export_for_viewer(&state, &effects, &viewer);
        black_box(import_viewer_export(&export).expect("export imports"));
    }
}

fn bench_l0_decision(iterations: u64, seat_count: usize) {
    let state = setup_state(seat_count, 50_000);
    let seat = state.active_seat().expect("active seat");
    for index in 0..iterations {
        black_box(
            VowTideL0Bot::new(Seed(index))
                .select_decision(&state, seat)
                .expect("l0 selects"),
        );
    }
}

fn bench_l1_decision(iterations: u64, seat_count: usize) {
    let state = playing_state(seat_count);
    let seat = state.active_seat().expect("active seat");
    for index in 0..iterations {
        black_box(
            VowTideL1Bot::new(Seed(index))
                .select_decision(&state, seat)
                .expect("l1 selects"),
        );
    }
}

fn bench_full_seeded_match(iterations: u64, seat_count: usize) {
    for index in 0..iterations {
        black_box(complete_match(setup_state(seat_count, 60_000 + index)));
    }
}

fn setup_state(seat_count: usize, seed: u64) -> VowTideState {
    setup_match(
        Seed(seed),
        &canonical_seat_ids(seat_count),
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn dealer_hook_state(seat_count: usize) -> VowTideState {
    let mut state = setup_state(seat_count, 21);
    while state.active_seat() != Some(state.dealer) {
        let active = state.active_seat().expect("bidding active");
        apply_path(
            &mut state,
            active,
            ActionPath {
                segments: vec![ACTION_BID.to_owned(), "0".to_owned()],
            },
        );
    }
    state
}

fn playing_state(seat_count: usize) -> VowTideState {
    playing_state_with_seed(seat_count, 31)
}

fn playing_state_with_seed(seat_count: usize, seed: u64) -> VowTideState {
    let mut state = setup_state(seat_count, seed);
    complete_bidding(&mut state);
    state
}

fn almost_complete_trick_state(seat_count: usize) -> VowTideState {
    let mut state = playing_state(seat_count);
    while state
        .playing_state()
        .expect("playing")
        .current_trick
        .plays
        .len()
        + 1
        < seat_count
    {
        apply_first_legal(&mut state);
    }
    state
}

fn scoring_state(seat_count: usize) -> VowTideState {
    let mut state = playing_state(seat_count);
    let hand_size = state.current_hand_size().unwrap_or_default();
    for seat in VowTideSeat::ALL.into_iter().take(seat_count) {
        if let Some((_, count)) = state
            .trick_counts
            .iter_mut()
            .find(|(candidate, _)| *candidate == seat)
        {
            *count = if seat == VowTideSeat::Seat0 {
                hand_size
            } else {
                0
            };
        }
    }
    state
}

fn one_action_effects(seat_count: usize) -> (VowTideState, Vec<vow_tide::effects::VowTideEffect>) {
    let mut state = setup_state(seat_count, 41);
    let active = state.active_seat().expect("active seat");
    let effects = apply_first_legal_for(&mut state, active);
    (state, effects)
}

fn complete_bidding(state: &mut VowTideState) {
    while matches!(state.phase, Phase::Bidding(_)) {
        apply_first_legal(state);
    }
}

fn complete_match(mut state: VowTideState) -> VowTideState {
    for _ in 0..20_000 {
        if matches!(state.phase, Phase::Terminal(_)) {
            return state;
        }
        apply_first_legal(&mut state);
    }
    panic!("vow_tide match did not terminate");
}

fn apply_first_legal(state: &mut VowTideState) -> Vec<vow_tide::effects::VowTideEffect> {
    let active = state.active_seat().expect("active seat");
    apply_first_legal_for(state, active)
}

fn apply_first_legal_for(
    state: &mut VowTideState,
    active: VowTideSeat,
) -> Vec<vow_tide::effects::VowTideEffect> {
    let path = first_legal_path(state, active);
    apply_path(state, active, path)
}

fn apply_path(
    state: &mut VowTideState,
    active: VowTideSeat,
    path: ActionPath,
) -> Vec<vow_tide::effects::VowTideEffect> {
    let command = command(state, active, path);
    match command.action_path.segments.first().map(String::as_str) {
        Some(ACTION_BID) => {
            let bid = validate_bid_command(state, &command).expect("bid validates");
            apply_bid(state, bid).expect("bid applies")
        }
        Some(ACTION_PLAY) => {
            let play = validate_play_command(state, &command).expect("play validates");
            apply_play(state, play).expect("play applies")
        }
        _ => panic!("unknown action path"),
    }
}

fn first_legal_path(state: &VowTideState, active: VowTideSeat) -> ActionPath {
    let tree = legal_action_tree(state, &actor(active, state));
    tree.root
        .choices
        .iter()
        .filter_map(|choice| choice.next.as_ref().map(|node| (choice, node)))
        .flat_map(|(choice, node)| {
            node.choices.iter().map(move |leaf| ActionPath {
                segments: vec![choice.segment.clone(), leaf.segment.clone()],
            })
        })
        .next()
        .expect("legal path exists")
}

fn actor(seat: VowTideSeat, state: &VowTideState) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

fn command(state: &VowTideState, seat: VowTideSeat, action_path: ActionPath) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat, state),
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn seat_viewers(state: &VowTideState) -> Vec<Viewer> {
    state
        .seats
        .iter()
        .map(|seat| Viewer {
            seat_id: Some(seat.clone()),
        })
        .collect()
}

fn threshold_catalog() -> Vec<Threshold> {
    let mut thresholds = Vec::new();
    for seat_count in SEAT_COUNTS {
        let suffix = format!("{seat_count}p");
        for (operation, unit) in [
            ("setup_deal", "setups_per_second"),
            ("bid_legal_first", "trees_per_second"),
            ("bid_legal_dealer_hook", "trees_per_second"),
            ("play_legal_lead", "trees_per_second"),
            ("play_legal_follow", "trees_per_second"),
            ("validate_apply_bid", "actions_per_second"),
            ("validate_apply_play", "actions_per_second"),
            ("trick_resolution", "tricks_per_second"),
            ("score_hand", "scores_per_second"),
            ("project_observer", "views_per_second"),
            ("project_all_seats", "viewer_sets_per_second"),
            ("effect_filter_all_viewers", "viewer_sets_per_second"),
            ("replay_snapshot_export_import", "exports_per_second"),
            ("l0_decision", "decisions_per_second"),
            ("l1_decision", "decisions_per_second"),
        ] {
            thresholds.push(Threshold {
                operation: format!("{operation}_{suffix}"),
                unit,
                threshold: 1.0,
                rationale_class: "baseline_pending_non_blocking",
                caveat: "Smoke floor until vow_tide has repeated CI-runner measurements under ADR 0002/0003 calibration policy.",
            });
        }
        thresholds.push(Threshold {
            operation: format!("full_seeded_match_{suffix}"),
            unit: "matches_per_second",
            threshold: 75.0,
            rationale_class: "provisional_native_gate_floor",
            caveat: "Gate 17 provisional native floor from the spec target; recalibrate only from repeated CI-runner evidence without removing visibility, bidding, trick, scoring, or bot work.",
        });
    }
    thresholds
}

fn threshold_for(operation: &str) -> Threshold {
    threshold_catalog()
        .into_iter()
        .find(|threshold| threshold.operation == operation)
        .unwrap_or_else(|| Threshold {
            operation: operation.to_owned(),
            unit: "operations_per_second",
            threshold: 1.0,
            rationale_class: "baseline_pending_non_blocking",
            caveat: "Fallback smoke floor for newly added operation.",
        })
}

fn print_human_summary(results: &[BenchResult]) {
    println!("vow_tide native benchmarks");
    println!("operation,iterations,unit,elapsed_ms,per_second,threshold,pass");
    for result in results {
        let threshold = threshold_for(&result.name);
        let current = result.current_value();
        println!(
            "{},{},{},{:.3},{:.2},{:.2},{}",
            result.name,
            result.iterations,
            result.unit,
            result.elapsed.as_secs_f64() * 1000.0,
            current,
            threshold.threshold,
            current >= threshold.threshold
        );
    }
}

fn benchmark_report_json(results: &[BenchResult]) -> String {
    let benches = results
        .iter()
        .map(|result| {
            let threshold = threshold_for(&result.name);
            format!(
                concat!(
                    "{{\"operation_name\":\"{}\",\"unit\":\"{}\",",
                    "\"iterations\":{},\"elapsed_ms\":{:.3},\"current_value\":{:.3},",
                    "\"threshold\":{:.3},\"pass\":{},\"rationale_class\":\"{}\",",
                    "\"caveat\":\"{}\"}}"
                ),
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
        concat!(
            "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version\":\"{}\",",
            "\"data_version\":\"{}\",\"engine_version\":\"{}\",\"build_profile\":\"{}\",",
            "\"benchmarks\":[{}]}}"
        ),
        REPORT_SCHEMA_VERSION,
        GAME_ID,
        RULES_VERSION_LABEL,
        DATA_VERSION,
        ENGINE_VERSION,
        BUILD_PROFILE,
        benches
    )
}
