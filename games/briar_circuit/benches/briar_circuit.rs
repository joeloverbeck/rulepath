use std::{
    env,
    hint::black_box,
    process::Command,
    time::{Duration, Instant},
};

use briar_circuit::{
    apply_pass_action, apply_play_action, canonical_deck, canonical_seat_ids, effect_envelopes,
    export_viewer_timeline, filter_effects_for_viewer, import_viewer_timeline, legal_bot_actions,
    replay_hash_snapshot, score_completed_hand, setup_match, validate_pass_command,
    validate_play_command, BriarCircuitBotAction, BriarCircuitL0Bot, BriarCircuitL1Bot,
    BriarCircuitSeat, BriarCircuitState, CapturedTrick, Card, CardId, Phase, PlayAction, Rank,
    SetupOptions, Suit, TrickPlay, ViewerExportClass, GAME_ID, RULES_VERSION_LABEL,
};
use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, Seed, Viewer};

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
    smoke("setup_shuffle_deal_serialize", "setups_per_second"),
    smoke("pass_legal_actions", "trees_per_second"),
    smoke("pass_select_apply", "actions_per_second"),
    smoke("pass_commit_exchange", "exchanges_per_second"),
    smoke("play_legal_actions_max_hand", "trees_per_second"),
    smoke("play_legal_actions_follow", "trees_per_second"),
    smoke("validate_apply_play", "actions_per_second"),
    smoke("trick_resolution", "tricks_per_second"),
    smoke("normal_hand_scoring", "scores_per_second"),
    smoke("moon_hand_scoring", "scores_per_second"),
    smoke("threshold_outcome", "outcomes_per_second"),
    smoke("project_observer_view", "views_per_second"),
    smoke("project_four_seat_views", "viewer_sets_per_second"),
    smoke("effect_filter_all_viewers", "viewer_sets_per_second"),
    smoke("full_internal_trace_replay", "hands_per_second"),
    smoke("viewer_scoped_export_import_public", "exports_per_second"),
    smoke("viewer_scoped_export_import_seat", "exports_per_second"),
    smoke("l0_action_selection", "decisions_per_second"),
    smoke("l1_action_selection", "decisions_per_second"),
    smoke("full_seeded_hand", "hands_per_second"),
    Threshold {
        operation: "full_seeded_match_terminal",
        unit: "matches_per_second",
        threshold: 100.0,
        rationale_class: "provisional_native_gate_floor",
        caveat: "Provisional Gate 16 native floor from the spec target; recalibrate only from repeated CI-runner evidence without removing visibility or explanation work.",
    },
];

const fn smoke(operation: &'static str, unit: &'static str) -> Threshold {
    Threshold {
        operation,
        unit,
        threshold: 1.0,
        rationale_class: "baseline_pending_non_blocking",
        caveat: "Smoke floor until briar_circuit has repeated CI-runner measurements under ADR 0002/0003 calibration policy.",
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
    println!("BEGIN_BRIAR_CIRCUIT_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&metadata, &results));
    println!("END_BRIAR_CIRCUIT_BENCHMARK_JSON");
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
            "setup_shuffle_deal_serialize",
            "setups",
            10_000,
            bench_setup_shuffle_deal_serialize,
        ),
        (
            "pass_legal_actions",
            "trees",
            25_000,
            bench_pass_legal_actions,
        ),
        (
            "pass_select_apply",
            "actions",
            10_000,
            bench_pass_select_apply,
        ),
        (
            "pass_commit_exchange",
            "exchanges",
            5_000,
            bench_pass_commit_exchange,
        ),
        (
            "play_legal_actions_max_hand",
            "trees",
            25_000,
            bench_play_legal_actions_max_hand,
        ),
        (
            "play_legal_actions_follow",
            "trees",
            25_000,
            bench_play_legal_actions_follow,
        ),
        (
            "validate_apply_play",
            "actions",
            10_000,
            bench_validate_apply_play,
        ),
        ("trick_resolution", "tricks", 10_000, bench_trick_resolution),
        (
            "normal_hand_scoring",
            "scores",
            25_000,
            bench_normal_hand_scoring,
        ),
        (
            "moon_hand_scoring",
            "scores",
            25_000,
            bench_moon_hand_scoring,
        ),
        (
            "threshold_outcome",
            "outcomes",
            25_000,
            bench_threshold_outcome,
        ),
        (
            "project_observer_view",
            "views",
            25_000,
            bench_project_observer_view,
        ),
        (
            "project_four_seat_views",
            "viewer_sets",
            10_000,
            bench_project_four_seat_views,
        ),
        (
            "effect_filter_all_viewers",
            "viewer_sets",
            10_000,
            bench_effect_filter_all_viewers,
        ),
        (
            "full_internal_trace_replay",
            "hands",
            500,
            bench_full_internal_trace_replay,
        ),
        (
            "viewer_scoped_export_import_public",
            "exports",
            10_000,
            bench_viewer_scoped_export_import_public,
        ),
        (
            "viewer_scoped_export_import_seat",
            "exports",
            10_000,
            bench_viewer_scoped_export_import_seat,
        ),
        (
            "l0_action_selection",
            "decisions",
            25_000,
            bench_l0_action_selection,
        ),
        (
            "l1_action_selection",
            "decisions",
            25_000,
            bench_l1_action_selection,
        ),
        ("full_seeded_hand", "hands", 500, bench_full_seeded_hand),
        (
            "full_seeded_match_terminal",
            "matches",
            500,
            bench_full_seeded_match_terminal,
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
    println!("briar_circuit native benchmarks");
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

fn bench_setup_shuffle_deal_serialize(iterations: u64) {
    for index in 0..iterations {
        let state = setup_with_seed(index);
        black_box(state.stable_internal_summary());
        black_box(replay_hash_snapshot(&state));
    }
}

fn bench_pass_legal_actions(iterations: u64) {
    let state = setup();
    for _ in 0..iterations {
        black_box(
            legal_bot_actions(black_box(&state), black_box(BriarCircuitSeat::Seat0)).unwrap(),
        );
    }
}

fn bench_pass_select_apply(iterations: u64) {
    let base = setup();
    let seat = BriarCircuitSeat::Seat0;
    let card = base.hand_for_internal(seat)[0];
    let command = pass_command(
        &base,
        seat,
        vec!["pass".to_owned(), "select".to_owned(), card.as_str()],
    );
    for _ in 0..iterations {
        let mut state = base.clone();
        let (seat, action) =
            validate_pass_command(black_box(&state), black_box(&command)).expect("pass validates");
        black_box(apply_pass_action(black_box(&mut state), seat, action).unwrap());
    }
}

fn bench_pass_commit_exchange(iterations: u64) {
    let base = state_before_fourth_pass_confirm();
    let seat = BriarCircuitSeat::Seat3;
    let command = pass_command(&base, seat, vec!["pass".to_owned(), "confirm".to_owned()]);
    for _ in 0..iterations {
        let mut state = base.clone();
        let (seat, action) = validate_pass_command(black_box(&state), black_box(&command))
            .expect("fourth confirm validates");
        black_box(apply_pass_action(black_box(&mut state), seat, action).unwrap());
    }
}

fn bench_play_legal_actions_max_hand(iterations: u64) {
    let state = state_after_pass_exchange();
    let active = active_seat(&state);
    for _ in 0..iterations {
        black_box(legal_bot_actions(black_box(&state), black_box(active)).unwrap());
    }
}

fn bench_play_legal_actions_follow(iterations: u64) {
    let state = state_after_n_plays(1);
    let active = active_seat(&state);
    for _ in 0..iterations {
        black_box(legal_bot_actions(black_box(&state), black_box(active)).unwrap());
    }
}

fn bench_validate_apply_play(iterations: u64) {
    let base = state_after_pass_exchange();
    let seat = active_seat(&base);
    let command = first_play_command(&base, seat);
    for _ in 0..iterations {
        let mut state = base.clone();
        let (seat, action) =
            validate_play_command(black_box(&state), black_box(&command)).expect("play validates");
        black_box(apply_play_action(black_box(&mut state), seat, action).unwrap());
    }
}

fn bench_trick_resolution(iterations: u64) {
    let base = state_after_n_plays(3);
    let seat = active_seat(&base);
    let command = first_play_command(&base, seat);
    for _ in 0..iterations {
        let mut state = base.clone();
        let (seat, action) = validate_play_command(black_box(&state), black_box(&command))
            .expect("trick play validates");
        let result = apply_play_action(black_box(&mut state), seat, action).unwrap();
        assert!(result.trick_completed);
        black_box(result);
    }
}

fn bench_normal_hand_scoring(iterations: u64) {
    let captured = normal_scoring_fixture();
    for _ in 0..iterations {
        black_box(score_completed_hand(
            black_box(&captured),
            black_box([0, 0, 0, 0]),
        ));
    }
}

fn bench_moon_hand_scoring(iterations: u64) {
    let captured = moon_scoring_fixture();
    for _ in 0..iterations {
        black_box(score_completed_hand(
            black_box(&captured),
            black_box([0, 0, 0, 0]),
        ));
    }
}

fn bench_threshold_outcome(iterations: u64) {
    let captured = normal_scoring_fixture();
    for _ in 0..iterations {
        black_box(score_completed_hand(
            black_box(&captured),
            black_box([99, 98, 97, 50]),
        ));
    }
}

fn bench_project_observer_view(iterations: u64) {
    let state = state_after_n_plays(5);
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(briar_circuit::project_view(
            black_box(&state),
            black_box(&viewer),
        ));
    }
}

fn bench_project_four_seat_views(iterations: u64) {
    let state = state_after_n_plays(5);
    let viewers = seat_viewers();
    for _ in 0..iterations {
        for viewer in &viewers {
            black_box(briar_circuit::project_view(
                black_box(&state),
                black_box(viewer),
            ));
        }
    }
}

fn bench_effect_filter_all_viewers(iterations: u64) {
    let mut state = state_before_fourth_pass_confirm();
    let result = apply_pass_action(
        &mut state,
        BriarCircuitSeat::Seat3,
        briar_circuit::PassAction::Confirm,
    )
    .expect("exchange effects");
    let envelopes: Vec<_> = result
        .effects
        .into_iter()
        .flat_map(effect_envelopes)
        .collect();
    let mut viewers = vec![Viewer { seat_id: None }];
    viewers.extend(seat_viewers());

    for _ in 0..iterations {
        for viewer in &viewers {
            black_box(filter_effects_for_viewer(
                black_box(&envelopes),
                black_box(viewer),
            ));
        }
    }
}

fn bench_full_internal_trace_replay(iterations: u64) {
    for index in 0..iterations {
        black_box(full_hand_hash_timeline(Seed(index)));
    }
}

fn bench_viewer_scoped_export_import_public(iterations: u64) {
    let state = full_hand_state(Seed(16));
    for _ in 0..iterations {
        let export = export_viewer_timeline(black_box(&state), ViewerExportClass::Public);
        black_box(import_viewer_timeline(black_box(&export)).unwrap());
    }
}

fn bench_viewer_scoped_export_import_seat(iterations: u64) {
    let state = full_hand_state(Seed(16));
    for _ in 0..iterations {
        let export = export_viewer_timeline(
            black_box(&state),
            ViewerExportClass::SeatPrivate(BriarCircuitSeat::Seat0),
        );
        black_box(import_viewer_timeline(black_box(&export)).unwrap());
    }
}

fn bench_l0_action_selection(iterations: u64) {
    let state = setup();
    for index in 0..iterations {
        let mut bot = BriarCircuitL0Bot::new(Seed(index));
        black_box(
            bot.select_decision(black_box(&state), black_box(BriarCircuitSeat::Seat0))
                .unwrap(),
        );
    }
}

fn bench_l1_action_selection(iterations: u64) {
    let state = setup();
    let bot = BriarCircuitL1Bot::new(Seed(16));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(BriarCircuitSeat::Seat0))
                .unwrap(),
        );
    }
}

fn bench_full_seeded_hand(iterations: u64) {
    for index in 0..iterations {
        black_box(full_hand_state(Seed(index)));
    }
}

fn bench_full_seeded_match_terminal(iterations: u64) {
    for index in 0..iterations {
        let state = full_terminal_match_state(Seed(index));
        assert!(matches!(state.phase, Phase::Terminal(_)));
        black_box(state);
    }
}

fn setup() -> BriarCircuitState {
    setup_with_seed(1606)
}

fn setup_with_seed(seed: u64) -> BriarCircuitState {
    setup_match(Seed(seed), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds")
}

fn state_before_fourth_pass_confirm() -> BriarCircuitState {
    let mut state = setup();
    for seat in [
        BriarCircuitSeat::Seat0,
        BriarCircuitSeat::Seat1,
        BriarCircuitSeat::Seat2,
    ] {
        select_three_and_confirm(&mut state, seat);
    }
    for card in state.hand_for_internal(BriarCircuitSeat::Seat3)[..3].to_vec() {
        apply_pass_action(
            &mut state,
            BriarCircuitSeat::Seat3,
            briar_circuit::PassAction::Select(card),
        )
        .expect("seat 3 select");
    }
    state
}

fn state_after_pass_exchange() -> BriarCircuitState {
    let mut state = state_before_fourth_pass_confirm();
    apply_pass_action(
        &mut state,
        BriarCircuitSeat::Seat3,
        briar_circuit::PassAction::Confirm,
    )
    .expect("seat 3 confirm");
    state
}

fn state_after_n_plays(count: usize) -> BriarCircuitState {
    let mut state = state_after_pass_exchange();
    for _ in 0..count {
        apply_first_legal_play(&mut state);
    }
    state
}

fn full_hand_state(seed: Seed) -> BriarCircuitState {
    let mut state = setup_with_seed(seed.0);
    drive_one_hand(&mut state);
    assert!(matches!(
        state.phase,
        Phase::ScoringHand(_) | Phase::Terminal(_)
    ));
    state
}

fn full_terminal_match_state(seed: Seed) -> BriarCircuitState {
    let mut state = setup_with_seed(seed.0);
    state.cumulative_scores = [100, 100, 100, 0];
    drive_one_hand(&mut state);
    state
}

fn full_hand_hash_timeline(seed: Seed) -> Vec<briar_circuit::ReplayHashSnapshot> {
    let mut state = setup_with_seed(seed.0);
    let mut hashes = vec![replay_hash_snapshot(&state)];
    while !matches!(state.phase, Phase::ScoringHand(_) | Phase::Terminal(_)) {
        match state.phase {
            Phase::Passing(_) => apply_next_pass_action(&mut state),
            Phase::PlayingTrick(_) => apply_first_legal_play(&mut state),
            Phase::ScoringHand(_) | Phase::Terminal(_) => unreachable!("loop condition"),
        }
        hashes.push(replay_hash_snapshot(&state));
    }
    hashes
}

fn drive_one_hand(state: &mut BriarCircuitState) {
    while !matches!(state.phase, Phase::ScoringHand(_) | Phase::Terminal(_)) {
        match state.phase {
            Phase::Passing(_) => apply_next_pass_action(state),
            Phase::PlayingTrick(_) => apply_first_legal_play(state),
            Phase::ScoringHand(_) | Phase::Terminal(_) => unreachable!("loop condition"),
        }
    }
}

fn apply_next_pass_action(state: &mut BriarCircuitState) {
    let seat = BriarCircuitSeat::ALL
        .into_iter()
        .find(|seat| {
            state
                .pass_state()
                .is_some_and(|pass| !pass.is_committed(*seat))
        })
        .expect("pending pass seat exists");
    let bot = BriarCircuitL1Bot::new(Seed(u64::from(state.freshness_token.0)));
    let decision = bot.select_decision(state, seat).expect("pass bot decision");
    apply_bot_decision(state, decision.action);
}

fn apply_first_legal_play(state: &mut BriarCircuitState) {
    let seat = active_seat(state);
    let command = first_play_command(state, seat);
    let (seat, action) = validate_play_command(state, &command).expect("play validates");
    apply_play_action(state, seat, action).expect("play applies");
}

fn apply_bot_decision(state: &mut BriarCircuitState, action: BriarCircuitBotAction) {
    match action {
        BriarCircuitBotAction::Pass(action) => {
            let seat = BriarCircuitSeat::ALL
                .into_iter()
                .find(|seat| {
                    state
                        .pass_state()
                        .is_some_and(|pass| !pass.is_committed(*seat))
                })
                .expect("pending pass actor exists");
            apply_pass_action(state, seat, action).expect("pass applies");
        }
        BriarCircuitBotAction::Play(PlayAction::Play(card)) => {
            let seat = active_seat(state);
            apply_play_action(state, seat, PlayAction::Play(card)).expect("play applies");
        }
    }
}

fn select_three_and_confirm(state: &mut BriarCircuitState, seat: BriarCircuitSeat) {
    for card in state.hand_for_internal(seat)[..3].to_vec() {
        apply_pass_action(state, seat, briar_circuit::PassAction::Select(card))
            .expect("select pass card");
    }
    apply_pass_action(state, seat, briar_circuit::PassAction::Confirm).expect("confirm pass");
}

fn first_play_command(state: &BriarCircuitState, seat: BriarCircuitSeat) -> CommandEnvelope {
    let action = legal_bot_actions(state, seat)
        .expect("legal play actions")
        .into_iter()
        .find_map(|action| match action {
            BriarCircuitBotAction::Play(PlayAction::Play(card)) => Some(card),
            BriarCircuitBotAction::Pass(_) => None,
        })
        .expect("legal play card exists");
    play_command(state, seat, action)
}

fn pass_command(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
    segments: Vec<String>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(state, seat),
        action_path: ActionPath { segments },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn play_command(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
    card: CardId,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(state, seat),
        action_path: ActionPath {
            segments: vec!["play".to_owned(), card.as_str()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn actor(state: &BriarCircuitState, seat: BriarCircuitSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

fn active_seat(state: &BriarCircuitState) -> BriarCircuitSeat {
    state.playing_state().expect("playing phase").active_seat
}

fn seat_viewers() -> Vec<Viewer> {
    BriarCircuitSeat::ALL
        .into_iter()
        .map(|seat| Viewer {
            seat_id: Some(engine_core::SeatId(seat.as_str().to_owned())),
        })
        .collect()
}

fn normal_scoring_fixture() -> Vec<CapturedTrick> {
    scoring_fixture(BriarCircuitSeat::Seat1)
}

fn moon_scoring_fixture() -> Vec<CapturedTrick> {
    scoring_fixture(BriarCircuitSeat::Seat0)
}

fn scoring_fixture(point_winner: BriarCircuitSeat) -> Vec<CapturedTrick> {
    canonical_deck()
        .chunks(4)
        .enumerate()
        .map(|(index, chunk)| {
            let queen_spades = card(Rank::Queen, Suit::Spades);
            let winner = if chunk
                .iter()
                .any(|card_id| card_id.card().is_heart() || *card_id == queen_spades)
            {
                point_winner
            } else {
                BriarCircuitSeat::from_index(index % 4).expect("seat index")
            };
            CapturedTrick {
                hand_index: 0,
                trick_index: index as u8,
                winner,
                plays: chunk
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(seat_index, card)| TrickPlay {
                        seat: BriarCircuitSeat::from_index(seat_index).expect("seat index"),
                        card,
                    })
                    .collect(),
            }
        })
        .collect()
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
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
