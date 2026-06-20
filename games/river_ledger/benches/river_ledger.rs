use std::{
    env,
    hint::black_box,
    time::{Duration, Instant},
};

use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};
use river_ledger::{
    apply_action, best_five_from_seven, legal_action_tree, replay_support, setup_match,
    validate_command, Card, Rank, RiverLedgerAction, RiverLedgerLevel2Bot, RiverLedgerSeat,
    RiverLedgerState, SeatLedger, SeatStatus, SetupOptions, Suit, GAME_ID, RULES_VERSION_LABEL,
};

const DATA_VERSION: &str = "1";
const ENGINE_VERSION: &str = "engine-core-0.1.0";
const BUILD_PROFILE: &str = "bench";

struct BenchResult {
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    elapsed: Duration,
}

type BenchSpec = (&'static str, &'static str, u64, fn(u64));

fn main() {
    let benches: Vec<BenchSpec> = vec![
        (
            "setup_3p_equal_stacks",
            "setups",
            10_000,
            bench_setup_3p_equal_stacks,
        ),
        (
            "setup_6p_asymmetric_stacks",
            "setups",
            10_000,
            bench_setup_6p_asymmetric_stacks,
        ),
        ("setup_deal_6p", "setups", 10_000, bench_setup_deal_6p),
        (
            "legal_actions_initial_6p",
            "trees",
            25_000,
            bench_legal_actions_initial_6p,
        ),
        (
            "legal_actions_short_stack",
            "trees",
            25_000,
            bench_legal_actions_short_stack,
        ),
        ("apply_call_6p", "actions", 10_000, bench_apply_call_6p),
        (
            "apply_short_all_in_raise",
            "actions",
            10_000,
            bench_apply_short_all_in_raise,
        ),
        (
            "construct_side_pots_6p_max_layers",
            "layers",
            25_000,
            bench_construct_side_pots_6p_max_layers,
        ),
        (
            "allocate_side_pots_6p_split_winners",
            "allocations",
            25_000,
            bench_allocate_side_pots_6p_split_winners,
        ),
        (
            "resolve_all_in_showdown_6p",
            "showdowns",
            5_000,
            bench_resolve_all_in_showdown_6p,
        ),
        (
            "project_all_viewers_6p",
            "viewer_sets",
            10_000,
            bench_project_all_viewers_6p,
        ),
        (
            "project_view_6p_multi_pot",
            "viewer_sets",
            5_000,
            bench_project_view_6p_multi_pot,
        ),
        (
            "public_export_import_6p",
            "exports",
            5_000,
            bench_public_export_import_6p,
        ),
        (
            "serialize_replay_6p_multi_pot",
            "serializations",
            5_000,
            bench_serialize_replay_6p_multi_pot,
        ),
        (
            "evaluator_showdown_batch_6p",
            "batches",
            10_000,
            bench_evaluator_showdown_batch_6p,
        ),
        (
            "bot_policy_6p_short_stack",
            "decisions",
            10_000,
            bench_bot_policy_6p_short_stack,
        ),
        (
            "level2_full_playout_6p",
            "hands",
            1_000,
            bench_level2_full_playout_6p,
        ),
        (
            "full_game_6p_all_in_pressure",
            "hands",
            1_000,
            bench_full_game_6p_all_in_pressure,
        ),
    ];
    let filter = env::args().skip(1).find(|arg| !arg.starts_with('-'));
    let results = benches
        .into_iter()
        .filter(|(name, _, _, _)| filter.as_ref().is_none_or(|filter| name.contains(filter)))
        .map(|(name, unit, iterations, bench)| measure(name, unit, iterations, bench))
        .collect::<Vec<_>>();

    println!("river_ledger native benchmarks");
    println!("operation,iterations,unit,elapsed_ms,per_second");
    for result in &results {
        println!(
            "{},{},{},{:.3},{:.2}",
            result.name,
            result.iterations,
            result.unit,
            result.elapsed.as_secs_f64() * 1_000.0,
            result.iterations as f64 / result.elapsed.as_secs_f64()
        );
    }
    println!("BEGIN_RIVER_LEDGER_BENCHMARK_JSON");
    println!("{}", report_json(&results));
    println!("END_RIVER_LEDGER_BENCHMARK_JSON");
}

fn measure(name: &'static str, unit: &'static str, iterations: u64, bench: fn(u64)) -> BenchResult {
    let started = Instant::now();
    bench(iterations);
    BenchResult {
        name,
        unit,
        iterations,
        elapsed: started.elapsed(),
    }
}

fn bench_setup_deal_6p(iterations: u64) {
    for index in 0..iterations {
        black_box(setup_with_seed(index, 6));
    }
}

fn bench_setup_3p_equal_stacks(iterations: u64) {
    for index in 0..iterations {
        black_box(setup_with_options(index, 3, SetupOptions::default()));
    }
}

fn bench_setup_6p_asymmetric_stacks(iterations: u64) {
    for index in 0..iterations {
        black_box(setup_with_options(index, 6, asymmetric_setup_options(6)));
    }
}

fn bench_legal_actions_initial_6p(iterations: u64) {
    let state = setup_with_seed(7, 6);
    let actor = actor(&state, state.active_seat.unwrap());
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_legal_actions_short_stack(iterations: u64) {
    let state = setup_with_options(7, 3, short_raise_setup_options());
    let actor = actor(&state, state.active_seat.unwrap());
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_apply_call_6p(iterations: u64) {
    for _ in 0..iterations {
        let mut state = setup_with_seed(7, 6);
        let active_seat = state.active_seat.unwrap();
        apply_known_action(&mut state, active_seat, RiverLedgerAction::Call);
        black_box(state);
    }
}

fn bench_apply_short_all_in_raise(iterations: u64) {
    for _ in 0..iterations {
        let mut state = setup_with_options(7, 3, short_raise_setup_options());
        let active_seat = state.active_seat.unwrap();
        apply_known_action(&mut state, active_seat, RiverLedgerAction::Raise);
        black_box(state);
    }
}

fn bench_construct_side_pots_6p_max_layers(iterations: u64) {
    let ledgers = max_layer_ledgers();
    assert_max_layer_fixture(&ledgers);
    for _ in 0..iterations {
        black_box(river_ledger::pot::construct_contribution_layers(black_box(
            &ledgers,
        )));
    }
}

fn bench_allocate_side_pots_6p_split_winners(iterations: u64) {
    let ledgers = max_layer_ledgers();
    let layers = river_ledger::pot::construct_contribution_layers(&ledgers);
    assert_max_layer_fixture(&ledgers);
    let winners = max_layer_winners();
    for _ in 0..iterations {
        black_box(river_ledger::pot::allocate_layered_pots(
            black_box(layers.clone()),
            black_box(&winners),
            RiverLedgerSeat::from_index(0).unwrap(),
            6,
        ));
    }
}

fn bench_resolve_all_in_showdown_6p(iterations: u64) {
    let state = max_layer_showdown_state(19);
    for _ in 0..iterations {
        black_box(river_ledger::resolve_showdown(black_box(&state)));
    }
}

fn bench_project_all_viewers_6p(iterations: u64) {
    let state = setup_with_seed(7, 6);
    let viewers = std::iter::once(Viewer { seat_id: None })
        .chain(default_seats(6).into_iter().map(|seat_id| Viewer {
            seat_id: Some(seat_id),
        }))
        .collect::<Vec<_>>();
    for _ in 0..iterations {
        for viewer in &viewers {
            black_box(river_ledger::project_view(
                black_box(&state),
                black_box(viewer),
            ));
        }
    }
}

fn bench_project_view_6p_multi_pot(iterations: u64) {
    let state = terminal_max_layer_state(19);
    let viewers = std::iter::once(Viewer { seat_id: None })
        .chain(default_seats(6).into_iter().map(|seat_id| Viewer {
            seat_id: Some(seat_id),
        }))
        .collect::<Vec<_>>();
    for _ in 0..iterations {
        for viewer in &viewers {
            black_box(river_ledger::project_view(
                black_box(&state),
                black_box(viewer),
            ));
        }
    }
}

fn bench_public_export_import_6p(iterations: u64) {
    let trace = replay_support::trace_from_commands(7, 6, &[(3, "call"), (4, "call")]);
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        let export = replay_support::export_public_replay(black_box(&trace), black_box(&viewer));
        black_box(replay_support::import_public_export(black_box(&export)));
    }
}

fn bench_serialize_replay_6p_multi_pot(iterations: u64) {
    let trace = replay_support::trace_from_commands(
        31,
        6,
        &[(3, "raise"), (4, "call"), (5, "call"), (0, "call")],
    );
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        let export = replay_support::export_public_replay(black_box(&trace), black_box(&viewer));
        black_box(export.to_json());
    }
}

fn bench_evaluator_showdown_batch_6p(iterations: u64) {
    let cards = [
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::Ten, Suit::Clubs),
        Card::new(Rank::Nine, Suit::Spades),
    ];
    for _ in 0..iterations {
        for _ in 0..6 {
            black_box(best_five_from_seven(black_box(cards)));
        }
    }
}

fn bench_level2_full_playout_6p(iterations: u64) {
    for index in 0..iterations {
        black_box(full_playout_state(Seed(index), 6));
    }
}

fn bench_bot_policy_6p_short_stack(iterations: u64) {
    let state = setup_with_options(7, 6, pressure_setup_options(6));
    let active_seat = state.active_seat.unwrap();
    let bot = RiverLedgerLevel2Bot::new(Seed(21));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(active_seat))
                .expect("bot decision"),
        );
    }
}

fn bench_full_game_6p_all_in_pressure(iterations: u64) {
    for index in 0..iterations {
        black_box(full_playout_state_with_options(
            Seed(index),
            6,
            pressure_setup_options(6),
        ));
    }
}

fn setup_with_seed(seed: u64, count: usize) -> RiverLedgerState {
    setup_match(Seed(seed), &default_seats(count), &SetupOptions::default()).expect("setup")
}

fn setup_with_options(seed: u64, count: usize, options: SetupOptions) -> RiverLedgerState {
    setup_match(Seed(seed), &default_seats(count), &options).expect("setup")
}

fn default_seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

fn full_playout_state(seed: Seed, count: usize) -> RiverLedgerState {
    full_playout_state_with_options(seed, count, SetupOptions::default())
}

fn full_playout_state_with_options(
    seed: Seed,
    count: usize,
    options: SetupOptions,
) -> RiverLedgerState {
    let mut state = setup_match(seed, &default_seats(count), &options).expect("setup");
    while let Some(active_seat) = state.active_seat {
        let decision = RiverLedgerLevel2Bot::new(seed)
            .select_decision(&state, active_seat)
            .expect("bot decision");
        let [segment] = decision.action_path.segments.as_slice() else {
            panic!("malformed bot action path");
        };
        let action = river_ledger::parse_action_segment(segment).expect("bot action parses");
        apply_known_action(&mut state, active_seat, action);
    }
    state
}

fn asymmetric_setup_options(count: usize) -> SetupOptions {
    SetupOptions {
        starting_stacks: Some((0..count).map(|index| 4 + index as u16 * 4).collect()),
        ..SetupOptions::default()
    }
}

fn short_raise_setup_options() -> SetupOptions {
    SetupOptions {
        starting_stacks: Some(vec![3, 16, 24]),
        ..SetupOptions::default()
    }
}

fn pressure_setup_options(count: usize) -> SetupOptions {
    let base = [8, 3, 2, 5, 13, 21];
    SetupOptions {
        starting_stacks: Some(base[..count].to_vec()),
        ..SetupOptions::default()
    }
}

fn max_layer_ledgers() -> Vec<SeatLedger> {
    [
        (0, SeatStatus::ShowdownEligible, 1),
        (1, SeatStatus::ShowdownEligible, 2),
        (2, SeatStatus::Folded, 3),
        (3, SeatStatus::ShowdownEligible, 5),
        (4, SeatStatus::ShowdownEligible, 8),
        (5, SeatStatus::ShowdownEligible, 13),
    ]
    .into_iter()
    .map(|(index, status, total_contribution)| SeatLedger {
        seat: RiverLedgerSeat::from_index(index).unwrap(),
        status,
        starting_stack: 13,
        remaining_stack: 13 - total_contribution,
        street_contribution: total_contribution,
        total_contribution,
    })
    .collect()
}

fn assert_max_layer_fixture(ledgers: &[SeatLedger]) {
    let layers = river_ledger::pot::construct_contribution_layers(ledgers);
    let distinct_caps = ledgers
        .iter()
        .map(|ledger| ledger.total_contribution)
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(ledgers.len(), 6);
    assert_eq!(distinct_caps.len(), 6);
    assert!(ledgers
        .iter()
        .any(|ledger| ledger.status == SeatStatus::Folded));
    assert!(layers.pots.len() >= 3);
    assert!(layers.returns.iter().any(|returned| returned.amount > 0));
    assert!(layers.pots.iter().any(|pot| pot
        .contributors
        .contains(&RiverLedgerSeat::from_index(2).unwrap())
        && !pot
            .eligible
            .contains(&RiverLedgerSeat::from_index(2).unwrap())));
    assert!(max_layer_winners()
        .iter()
        .any(|(_, winners)| winners.len() > 1));
}

fn max_layer_winners() -> Vec<(String, Vec<RiverLedgerSeat>)> {
    vec![
        (
            "main_pot".to_owned(),
            vec![
                RiverLedgerSeat::from_index(0).unwrap(),
                RiverLedgerSeat::from_index(1).unwrap(),
            ],
        ),
        (
            "side_pot_1".to_owned(),
            vec![
                RiverLedgerSeat::from_index(1).unwrap(),
                RiverLedgerSeat::from_index(3).unwrap(),
            ],
        ),
        (
            "side_pot_2".to_owned(),
            vec![RiverLedgerSeat::from_index(3).unwrap()],
        ),
        (
            "side_pot_3".to_owned(),
            vec![
                RiverLedgerSeat::from_index(4).unwrap(),
                RiverLedgerSeat::from_index(5).unwrap(),
            ],
        ),
    ]
}

fn max_layer_showdown_state(seed: u64) -> RiverLedgerState {
    let mut state = setup_with_options(seed, 6, pressure_setup_options(6));
    state.ledger.seats = no_return_showdown_ledgers();
    state.ledger.pot_total = state
        .ledger
        .seats
        .iter()
        .map(|seat| seat.total_contribution)
        .sum();
    state.active_seat = None;
    state.betting.actors_to_respond.clear();
    state.board = state.community_deck_internal().to_vec();
    state
}

fn terminal_max_layer_state(seed: u64) -> RiverLedgerState {
    let mut state = max_layer_showdown_state(seed);
    state.terminal_outcome = Some(river_ledger::resolve_showdown(&state));
    state
}

fn no_return_showdown_ledgers() -> Vec<SeatLedger> {
    [
        (0, SeatStatus::ShowdownEligible, 1),
        (1, SeatStatus::ShowdownEligible, 2),
        (2, SeatStatus::Folded, 3),
        (3, SeatStatus::ShowdownEligible, 5),
        (4, SeatStatus::ShowdownEligible, 8),
        (5, SeatStatus::ShowdownEligible, 8),
    ]
    .into_iter()
    .map(|(index, status, total_contribution)| SeatLedger {
        seat: RiverLedgerSeat::from_index(index).unwrap(),
        status,
        starting_stack: 13,
        remaining_stack: 13 - total_contribution,
        street_contribution: total_contribution,
        total_contribution,
    })
    .collect()
}

fn apply_known_action(
    state: &mut RiverLedgerState,
    seat: RiverLedgerSeat,
    action: RiverLedgerAction,
) {
    let command = CommandEnvelope {
        actor: actor(state, seat),
        action_path: ActionPath {
            segments: vec![action.segment().to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    let action = validate_command(state, &command).expect("command validates");
    apply_action(state, action).expect("action applies");
}

fn actor(state: &RiverLedgerState, seat: RiverLedgerSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

fn report_json(results: &[BenchResult]) -> String {
    let operations = results
        .iter()
        .map(|result| {
            format!(
                "{{\"operation_name\":\"{}\",\"iterations\":{},\"unit\":\"{}\",\"current_value\":{:.2}}}",
                result.name,
                result.iterations,
                result.unit,
                result.iterations as f64 / result.elapsed.as_secs_f64()
            )
        })
        .collect::<Vec<_>>()
        .join(",\n    ");
    format!(
        concat!(
            "{{\n",
            "  \"schema_version\": 1,\n",
            "  \"game_id\": \"{}\",\n",
            "  \"rules_version\": \"{}\",\n",
            "  \"data_version\": \"{}\",\n",
            "  \"engine_version\": \"{}\",\n",
            "  \"build_profile\": \"{}\",\n",
            "  \"command\": \"cargo bench -p river_ledger\",\n",
            "  \"os\": \"{}\",\n",
            "  \"rust_version\": \"{}\",\n",
            "  \"hardware_environment_notes\": \"native runner; thresholds are smoke floors pending repeated CI calibration\",\n",
            "  \"operations\": [\n",
            "    {}\n",
            "  ]\n",
            "}}"
        ),
        GAME_ID,
        RULES_VERSION_LABEL,
        DATA_VERSION,
        ENGINE_VERSION,
        BUILD_PROFILE,
        env::consts::OS,
        option_env!("RUSTC_VERSION").unwrap_or("rustc version not captured by harness"),
        operations
    )
}
