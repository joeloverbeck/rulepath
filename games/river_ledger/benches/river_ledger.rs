use std::{
    env,
    hint::black_box,
    time::{Duration, Instant},
};

use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};
use river_ledger::{
    apply_action, best_five_from_seven, legal_action_tree, replay_support, setup_match,
    validate_command, Card, Rank, RiverLedgerAction, RiverLedgerLevel2Bot, RiverLedgerSeat,
    RiverLedgerState, SetupOptions, Suit, GAME_ID, RULES_VERSION_LABEL,
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
        ("setup_deal_6p", "setups", 10_000, bench_setup_deal_6p),
        (
            "legal_actions_initial_6p",
            "trees",
            25_000,
            bench_legal_actions_initial_6p,
        ),
        ("apply_call_6p", "actions", 10_000, bench_apply_call_6p),
        (
            "project_all_viewers_6p",
            "viewer_sets",
            10_000,
            bench_project_all_viewers_6p,
        ),
        (
            "public_export_import_6p",
            "exports",
            5_000,
            bench_public_export_import_6p,
        ),
        (
            "evaluator_showdown_batch_6p",
            "batches",
            10_000,
            bench_evaluator_showdown_batch_6p,
        ),
        (
            "level2_full_playout_6p",
            "hands",
            1_000,
            bench_level2_full_playout_6p,
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

fn bench_legal_actions_initial_6p(iterations: u64) {
    let state = setup_with_seed(7, 6);
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

fn bench_public_export_import_6p(iterations: u64) {
    let trace = replay_support::trace_from_commands(7, 6, &[(3, "call"), (4, "call")]);
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        let export = replay_support::export_public_replay(black_box(&trace), black_box(&viewer));
        black_box(replay_support::import_public_export(black_box(&export)));
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

fn setup_with_seed(seed: u64, count: usize) -> RiverLedgerState {
    setup_match(Seed(seed), &default_seats(count), &SetupOptions::default()).expect("setup")
}

fn default_seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

fn full_playout_state(seed: Seed, count: usize) -> RiverLedgerState {
    let mut state =
        setup_match(seed, &default_seats(count), &SetupOptions::default()).expect("setup");
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
            "  \"operations\": [\n",
            "    {}\n",
            "  ]\n",
            "}}"
        ),
        GAME_ID, RULES_VERSION_LABEL, DATA_VERSION, ENGINE_VERSION, BUILD_PROFILE, operations
    )
}
