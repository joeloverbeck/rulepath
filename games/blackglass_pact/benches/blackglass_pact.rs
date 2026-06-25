use std::{
    env,
    hint::black_box,
    time::{Duration, Instant},
};

use blackglass_pact::{
    apply_bid_choice, apply_blind_nil_choice, apply_play_choice, canonical_seat_ids,
    export_for_viewer, export_stable_bytes, import_for_viewer, legal_action_tree,
    score_completed_hand, score_hand, setup_match, setup_match_with_scores, BlackglassL0Bot,
    BlackglassL1Bot, BlackglassPactState, BlackglassSeat, BlackglassViewer, BlindNilChoice, Card,
    Phase, PlayedCard, Rank, SetupOptions, Suit, TeamId, DATA_VERSION_LABEL, GAME_ID,
    RULES_VERSION_LABEL,
};
use engine_core::{Actor, SeatId, Seed};

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
    println!("BEGIN_BLACKGLASS_PACT_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&results));
    println!("END_BLACKGLASS_PACT_BENCHMARK_JSON");
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
            "setup_blind_deal_4p",
            "setups",
            2_000,
            bench_setup_blind_deal,
        ),
        spec(
            "legal_tree_blind_4p",
            "trees",
            5_000,
            bench_legal_tree_blind,
        ),
        spec("legal_tree_bid_4p", "trees", 5_000, bench_legal_tree_bid),
        spec("legal_tree_play_4p", "trees", 5_000, bench_legal_tree_play),
        spec(
            "validate_apply_bid_4p",
            "actions",
            2_000,
            bench_validate_apply_bid,
        ),
        spec(
            "validate_apply_play_4p",
            "actions",
            2_000,
            bench_validate_apply_play,
        ),
        spec(
            "promoted_helper_trick_resolution_4p",
            "tricks",
            5_000,
            bench_promoted_helper_trick_resolution,
        ),
        spec("score_hand_4p", "scores", 5_000, bench_score_hand),
        spec(
            "project_observer_4p",
            "views",
            5_000,
            bench_project_observer,
        ),
        spec(
            "project_all_seats_4p",
            "viewer_sets",
            2_000,
            bench_project_all_seats,
        ),
        spec(
            "replay_export_import_4p",
            "exports",
            2_000,
            bench_replay_export_import,
        ),
        spec("l0_decision_4p", "decisions", 5_000, bench_l0_decision),
        spec("l1_decision_4p", "decisions", 5_000, bench_l1_decision),
        spec(
            "full_seeded_match_smoke_4p",
            "matches",
            200,
            bench_full_seeded_match_smoke,
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

fn bench_setup_blind_deal(iterations: u64) {
    for index in 0..iterations {
        let mut state = setup_match_with_scores(
            Seed(180_400 + index),
            &canonical_seat_ids(),
            &SetupOptions::default(),
            [0, 100],
        )
        .expect("blind setup");
        apply_blind_nil_choice(&mut state, BlackglassSeat::South, BlindNilChoice::Declined)
            .expect("south blind choice");
        apply_blind_nil_choice(&mut state, BlackglassSeat::North, BlindNilChoice::Declined)
            .expect("north blind choice");
        black_box(state);
    }
}

fn bench_legal_tree_blind(iterations: u64) {
    let state = blind_state();
    let actor = actor_for(BlackglassSeat::South);
    for _ in 0..iterations {
        black_box(legal_action_tree(&state, &actor));
    }
}

fn bench_legal_tree_bid(iterations: u64) {
    let state = bid_state();
    let actor = actor_for(BlackglassSeat::East);
    for _ in 0..iterations {
        black_box(legal_action_tree(&state, &actor));
    }
}

fn bench_legal_tree_play(iterations: u64) {
    let state = play_state();
    let actor = actor_for(BlackglassSeat::East);
    for _ in 0..iterations {
        black_box(legal_action_tree(&state, &actor));
    }
}

fn bench_validate_apply_bid(iterations: u64) {
    for index in 0..iterations {
        let mut state = setup_match(
            Seed(181_000 + index),
            &canonical_seat_ids(),
            &SetupOptions::default(),
        )
        .expect("setup");
        black_box(
            apply_bid_choice(
                &mut state,
                BlackglassSeat::East,
                blackglass_pact::Bid::Tricks(3),
            )
            .expect("bid applies"),
        );
    }
}

fn bench_validate_apply_play(iterations: u64) {
    for _ in 0..iterations {
        let mut state = play_state();
        black_box(
            apply_play_choice(
                &mut state,
                BlackglassSeat::East,
                card(Rank::Two, Suit::Clubs),
            )
            .expect("play applies"),
        );
    }
}

fn bench_promoted_helper_trick_resolution(iterations: u64) {
    let plays = vec![
        PlayedCard {
            seat: BlackglassSeat::East,
            card: card(Rank::Two, Suit::Clubs),
        },
        PlayedCard {
            seat: BlackglassSeat::South,
            card: card(Rank::Ace, Suit::Clubs),
        },
        PlayedCard {
            seat: BlackglassSeat::West,
            card: card(Rank::Three, Suit::Spades),
        },
        PlayedCard {
            seat: BlackglassSeat::North,
            card: card(Rank::King, Suit::Clubs),
        },
    ];
    for _ in 0..iterations {
        black_box(blackglass_pact::trick_winner(&plays).expect("trick winner"));
    }
}

fn bench_score_hand(iterations: u64) {
    let state = scoring_state();
    for _ in 0..iterations {
        black_box(score_hand(&state).expect("score hand"));
    }
}

fn bench_project_observer(iterations: u64) {
    let state = bid_state();
    for _ in 0..iterations {
        black_box(blackglass_pact::observer_view(&state));
    }
}

fn bench_project_all_seats(iterations: u64) {
    let state = bid_state();
    for _ in 0..iterations {
        for seat in BlackglassSeat::ALL {
            black_box(blackglass_pact::seat_view(&state, seat));
        }
    }
}

fn bench_replay_export_import(iterations: u64) {
    let state = bid_state();
    for _ in 0..iterations {
        let public = export_for_viewer(&state, BlackglassViewer::Observer);
        black_box(export_stable_bytes(&public));
        black_box(import_for_viewer(&public, BlackglassViewer::Observer).expect("public import"));
        for seat in BlackglassSeat::ALL {
            let viewer = BlackglassViewer::Seat(seat);
            let export = export_for_viewer(&state, viewer);
            black_box(export_stable_bytes(&export));
            black_box(import_for_viewer(&export, viewer).expect("seat import"));
        }
    }
}

fn bench_l0_decision(iterations: u64) {
    let state = bid_state();
    for index in 0..iterations {
        black_box(
            BlackglassL0Bot::new(Seed(index))
                .select_decision(&state, BlackglassSeat::East)
                .expect("l0 decision"),
        );
    }
}

fn bench_l1_decision(iterations: u64) {
    let state = bid_state();
    for _ in 0..iterations {
        black_box(
            BlackglassL1Bot
                .select_decision(&state, BlackglassSeat::East)
                .expect("l1 decision"),
        );
    }
}

fn bench_full_seeded_match_smoke(iterations: u64) {
    for index in 0..iterations {
        let mut state = setup_match(
            Seed(180_400 + index),
            &canonical_seat_ids(),
            &SetupOptions::default(),
        )
        .expect("setup");
        black_box(
            BlackglassL1Bot
                .select_decision(&state, BlackglassSeat::East)
                .expect("l1 decision"),
        );
        black_box(
            BlackglassL0Bot::new(Seed(index))
                .select_decision(&state, BlackglassSeat::East)
                .expect("l0 decision"),
        );
        black_box(export_for_viewer(&state, BlackglassViewer::Observer));
        state.team_scores = if index % 2 == 0 {
            [500, 480]
        } else {
            [480, 500]
        };
        black_box((TeamId::NorthSouth.as_str(), TeamId::EastWest.as_str()));
    }
}

fn blind_state() -> BlackglassPactState {
    setup_match_with_scores(
        Seed(180_401),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("blind setup")
}

fn bid_state() -> BlackglassPactState {
    setup_match(
        Seed(180_402),
        &canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup")
}

fn play_state() -> BlackglassPactState {
    let mut state = setup_match(
        Seed(180_403),
        &canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup");
    state.private_hands = vec![
        (
            BlackglassSeat::East,
            vec![card(Rank::Two, Suit::Clubs), card(Rank::Ace, Suit::Clubs)],
        ),
        (BlackglassSeat::South, vec![card(Rank::Three, Suit::Clubs)]),
        (BlackglassSeat::West, vec![card(Rank::Four, Suit::Clubs)]),
        (BlackglassSeat::North, vec![card(Rank::Five, Suit::Clubs)]),
    ];
    state.phase = Phase::PlayingTrick {
        leader: BlackglassSeat::East,
        next: BlackglassSeat::East,
        plays: Vec::new(),
        trick_index: 0,
    };
    state
}

fn scoring_state() -> BlackglassPactState {
    let mut state = setup_match(
        Seed(180_404),
        &canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup");
    state.bids = [
        Some(blackglass_pact::Bid::Tricks(4)),
        Some(blackglass_pact::Bid::Tricks(3)),
        Some(blackglass_pact::Bid::Nil),
        Some(blackglass_pact::Bid::Tricks(4)),
    ];
    state.tricks_won = [5, 3, 0, 5];
    state.team_bags = [8, 4];
    state.phase = Phase::HandScoring {
        completed_tricks: blackglass_pact::STANDARD_HAND_SIZE,
    };
    let mut scored = state.clone();
    black_box(score_completed_hand(&mut scored).expect("completed hand scores"));
    state
}

fn actor_for(seat: BlackglassSeat) -> Actor {
    Actor {
        seat_id: SeatId::from_zero_based_index(seat.index() as u32),
    }
}

fn card(rank: Rank, suit: Suit) -> blackglass_pact::CardId {
    Card::new(rank, suit).id()
}

fn threshold_catalog() -> Vec<Threshold> {
    [
        ("setup_blind_deal_4p", "setups_per_second"),
        ("legal_tree_blind_4p", "trees_per_second"),
        ("legal_tree_bid_4p", "trees_per_second"),
        ("legal_tree_play_4p", "trees_per_second"),
        ("validate_apply_bid_4p", "actions_per_second"),
        ("validate_apply_play_4p", "actions_per_second"),
        ("promoted_helper_trick_resolution_4p", "tricks_per_second"),
        ("score_hand_4p", "scores_per_second"),
        ("project_observer_4p", "views_per_second"),
        ("project_all_seats_4p", "viewer_sets_per_second"),
        ("replay_export_import_4p", "exports_per_second"),
        ("l0_decision_4p", "decisions_per_second"),
        ("l1_decision_4p", "decisions_per_second"),
    ]
    .into_iter()
    .map(|(operation, unit)| Threshold {
        operation,
        unit,
        threshold: 1.0,
        rationale_class: "baseline_pending_non_blocking",
        caveat: "Smoke floor until blackglass_pact has repeated CI-runner measurements under ADR 0002/0003/0005 calibration policy.",
    })
    .chain(std::iter::once(Threshold {
        operation: "full_seeded_match_smoke_4p",
        unit: "matches_per_second",
        threshold: 75.0,
        rationale_class: "provisional_native_gate_floor",
        caveat: "Gate 18 provisional native floor for the fixed-four bot-smoke match lane; recalibrate only from repeated CI-runner evidence without removing setup, visibility, bot, or team-summary work.",
    }))
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
    println!("blackglass_pact native benchmarks");
    println!(
        "operation,iterations,unit,elapsed_ms,per_second,threshold,pass,seat_order,team_order"
    );
    for result in results {
        let threshold = threshold_for(result.name);
        let current = result.current_value();
        println!(
            "{},{},{},{:.3},{:.2},{:.2},{},seat_0|seat_1|seat_2|seat_3,team_0|team_1",
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
                    "\"seat_order\":[\"seat_0\",\"seat_1\",\"seat_2\",\"seat_3\"],",
                    "\"team_order\":[\"team_0\",\"team_1\"],\"caveat\":\"{}\"}}"
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
        DATA_VERSION_LABEL,
        ENGINE_VERSION,
        BUILD_PROFILE,
        benches
    )
}
