use std::{
    env,
    hint::black_box,
    time::{Duration, Instant},
};

use engine_core::{FreshnessToken, Seed, Viewer};
use meldfall_ledger::{
    actions::draw_source_action_tree,
    bots::{parse_bot_action, MeldfallL0Bot, L1_POLICY_STATUS},
    cards::{Card, CardId, Rank, Suit},
    effects::{public_effect, DrawSource, MeldfallEffect},
    replay_support::{export_viewer_snapshot, import_viewer_export},
    rules::{discard_card, draw_from_stock, finish_turn_after_table_plays},
    setup::{default_seats, setup_match, SetupOptions},
    state::{MatchState, MeldGroup, MeldId, MeldKind, MeldTableau, TableCard, TurnOrdinal},
    visibility::project_view,
    DATA_VERSION_LABEL, GAME_ID, RULES_VERSION_LABEL,
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
    println!("BEGIN_MELDFALL_LEDGER_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&results));
    println!("END_MELDFALL_LEDGER_BENCHMARK_JSON");
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
        spec(
            "native_2p_short_round",
            "actions",
            200,
            bench_native_2p_short_round,
        ),
        spec("native_4p_default", "actions", 500, bench_native_4p_default),
        spec(
            "native_6p_large_surface",
            "actions",
            200,
            bench_native_6p_large_surface,
        ),
        spec(
            "large_discard_tail",
            "trees",
            2_000,
            bench_large_discard_tail,
        ),
        spec(
            "large_public_tableau",
            "viewer_sets",
            1_000,
            bench_large_public_tableau,
        ),
        spec(
            "replay_export_import",
            "exports",
            1_000,
            bench_replay_export_import,
        ),
        spec("l0_bot_decision", "decisions", 2_000, bench_l0_bot_decision),
        spec(
            "l1_bot_decision",
            "status_checks",
            2_000,
            bench_l1_bot_decision,
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

fn bench_native_2p_short_round(iterations: u64) {
    bench_random_legal_actions(iterations, 2, 19_200);
}

fn bench_native_4p_default(iterations: u64) {
    bench_random_legal_actions(iterations, 4, 19_400);
}

fn bench_native_6p_large_surface(iterations: u64) {
    let mut state = setup_state(19_600, 6);
    for index in 0..iterations {
        let tree = draw_source_action_tree(FreshnessToken(index), &state.round);
        for viewer in all_viewers(&state) {
            black_box(project_view(&state, &viewer));
        }
        black_box(tree);
        apply_l0_step(&mut state, index);
    }
}

fn bench_large_discard_tail(iterations: u64) {
    let mut state = setup_state(19_610, 6);
    state.round.discard = vec![
        card(Rank::Ace, Suit::Clubs),
        card(Rank::Two, Suit::Clubs),
        card(Rank::Three, Suit::Clubs),
        card(Rank::Four, Suit::Clubs),
        card(Rank::Five, Suit::Clubs),
        card(Rank::Six, Suit::Clubs),
        card(Rank::Seven, Suit::Clubs),
        card(Rank::Eight, Suit::Clubs),
        card(Rank::Nine, Suit::Clubs),
        card(Rank::Ten, Suit::Clubs),
        card(Rank::Jack, Suit::Clubs),
        card(Rank::Queen, Suit::Clubs),
    ];
    for index in 0..iterations {
        black_box(draw_source_action_tree(FreshnessToken(index), &state.round));
    }
}

fn bench_large_public_tableau(iterations: u64) {
    let state = large_tableau_state();
    for _ in 0..iterations {
        for viewer in all_viewers(&state) {
            black_box(project_view(&state, &viewer));
        }
    }
}

fn bench_replay_export_import(iterations: u64) {
    let state = large_tableau_state();
    let action_tree = draw_source_action_tree(FreshnessToken(77), &state.round);
    let effects = vec![public_effect(MeldfallEffect::Draw {
        seat: 0,
        source: DrawSource::Stock,
        cards_moved: 1,
        stock_count_after: state.round.stock.len(),
        discard_count_after: state.round.discard.len(),
    })];
    let viewers = all_viewers(&state);
    for _ in 0..iterations {
        for viewer in &viewers {
            let export = export_viewer_snapshot(&state, &action_tree, &effects, viewer);
            black_box(export.stable_string());
            black_box(import_viewer_export(&export, viewer).expect("viewer export imports"));
        }
    }
}

fn bench_l0_bot_decision(iterations: u64) {
    let state = setup_state(19_620, 4);
    for index in 0..iterations {
        black_box(
            MeldfallL0Bot::new(Seed(index))
                .select_decision(&state, state.round.active_seat_index)
                .expect("l0 decision"),
        );
    }
}

fn bench_l1_bot_decision(iterations: u64) {
    for _ in 0..iterations {
        black_box(L1_POLICY_STATUS);
    }
}

fn bench_random_legal_actions(iterations: u64, seat_count: usize, seed: u64) {
    let mut state = setup_state(seed, seat_count);
    for index in 0..iterations {
        apply_l0_step(&mut state, index);
    }
}

fn apply_l0_step(state: &mut MatchState, index: u64) {
    let active = state.round.active_seat_index;
    let bot = MeldfallL0Bot::new(Seed(0x51A7_0000 ^ index));
    let Ok(path) = bot.select_action(state, active) else {
        return;
    };
    match parse_bot_action(&path).expect("bot action parses") {
        meldfall_ledger::actions::MeldfallAction::DrawFromStock => {
            let _ = draw_from_stock(&mut state.round, active).expect("draw applies");
        }
        meldfall_ledger::actions::MeldfallAction::FinishTurn => {
            let _ =
                finish_turn_after_table_plays(&mut state.round, active).expect("finish applies");
        }
        meldfall_ledger::actions::MeldfallAction::Discard { card } => {
            let _ = discard_card(&mut state.round, active, card).expect("discard applies");
        }
        _ => {}
    }
}

fn setup_state(seed: u64, seat_count: usize) -> MatchState {
    let seats = default_seats(seat_count).expect("seat count supported");
    let setup = setup_match(Seed(seed), &seats, &SetupOptions::default()).expect("setup succeeds");
    MatchState::from_initial_setup(setup)
}

fn large_tableau_state() -> MatchState {
    let mut state = setup_state(19_621, 6);
    state.round.tableau = MeldTableau {
        groups: vec![
            meld_group(
                0,
                MeldKind::Run { suit: Suit::Clubs },
                0,
                &[
                    card(Rank::Ace, Suit::Clubs),
                    card(Rank::Two, Suit::Clubs),
                    card(Rank::Three, Suit::Clubs),
                    card(Rank::Four, Suit::Clubs),
                    card(Rank::Five, Suit::Clubs),
                ],
            ),
            meld_group(
                1,
                MeldKind::Run {
                    suit: Suit::Diamonds,
                },
                1,
                &[
                    card(Rank::Seven, Suit::Diamonds),
                    card(Rank::Eight, Suit::Diamonds),
                    card(Rank::Nine, Suit::Diamonds),
                    card(Rank::Ten, Suit::Diamonds),
                ],
            ),
            meld_group(
                2,
                MeldKind::Set { rank: Rank::King },
                2,
                &[
                    card(Rank::King, Suit::Clubs),
                    card(Rank::King, Suit::Diamonds),
                    card(Rank::King, Suit::Hearts),
                    card(Rank::King, Suit::Spades),
                ],
            ),
        ],
    };
    state.round.discard = vec![
        card(Rank::Jack, Suit::Hearts),
        card(Rank::Queen, Suit::Hearts),
        card(Rank::Ace, Suit::Hearts),
    ];
    state
}

fn meld_group(id: u32, kind: MeldKind, origin_seat: usize, cards: &[CardId]) -> MeldGroup {
    MeldGroup {
        id: MeldId(id),
        kind,
        origin_seat,
        cards: cards
            .iter()
            .enumerate()
            .map(|(offset, card)| TableCard {
                card: *card,
                played_by: origin_seat,
                score_credit_owner: origin_seat,
                play_turn: TurnOrdinal(offset as u32),
            })
            .collect(),
    }
}

fn all_viewers(state: &MatchState) -> Vec<Viewer> {
    let mut viewers = vec![Viewer { seat_id: None }];
    viewers.extend(state.seats.iter().cloned().map(|seat_id| Viewer {
        seat_id: Some(seat_id),
    }));
    viewers
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
}

fn threshold_catalog() -> Vec<Threshold> {
    [
        ("native_2p_short_round", "actions_per_second"),
        ("native_4p_default", "actions_per_second"),
        ("native_6p_large_surface", "actions_per_second"),
        ("large_discard_tail", "trees_per_second"),
        ("large_public_tableau", "viewer_sets_per_second"),
        ("replay_export_import", "exports_per_second"),
        ("l0_bot_decision", "decisions_per_second"),
        ("l1_bot_decision", "status_checks_per_second"),
    ]
    .into_iter()
    .map(|(operation, unit)| Threshold {
        operation,
        unit,
        threshold: 1.0,
        rationale_class: "baseline_pending_non_blocking",
        caveat: "Smoke floor until meldfall_ledger has repeated CI-runner measurements under ADR 0002/0003/0005 calibration policy.",
    })
    .collect()
}

fn threshold_for(operation: &str) -> Threshold {
    threshold_catalog()
        .into_iter()
        .find(|threshold| threshold.operation == operation)
        .unwrap_or(Threshold {
            operation: "unknown",
            unit: "operations_per_second",
            threshold: 1.0,
            rationale_class: "baseline_pending_non_blocking",
            caveat: "Fallback smoke floor for newly added operation.",
        })
}

fn print_human_summary(results: &[BenchResult]) {
    println!("meldfall_ledger native benchmarks");
    println!("operation,iterations,unit,elapsed_ms,per_second,threshold,pass,seat_order");
    for result in results {
        let threshold = threshold_for(result.name);
        let current = result.current_value();
        println!(
            "{},{},{},{:.3},{:.2},{:.2},{},seat_0|seat_1|seat_2|seat_3|seat_4|seat_5",
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
            let threshold = threshold_for(result.name);
            format!(
                concat!(
                    "{{\"operation_name\":\"{}\",\"unit\":\"{}\",",
                    "\"iterations\":{},\"elapsed_ms\":{:.3},\"current_value\":{:.3},",
                    "\"threshold\":{:.3},\"pass\":{},\"rationale_class\":\"{}\",",
                    "\"seat_order\":[\"seat_0\",\"seat_1\",\"seat_2\",\"seat_3\",\"seat_4\",\"seat_5\"],",
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
            "\"command\":\"cargo bench -p meldfall_ledger\",",
            "\"os\":\"unknown\",\"rust_version\":\"unknown\",",
            "\"hardware_environment_notes\":\"uncontrolled smoke runner\",",
            "\"operations\":[{}]}}"
        ),
        REPORT_SCHEMA_VERSION,
        GAME_ID,
        RULES_VERSION_LABEL,
        DATA_VERSION_LABEL,
        ENGINE_VERSION,
        BUILD_PROFILE,
        benches
    )
}
