use briar_circuit::{
    canonical_seat_ids, export_viewer_timeline, import_viewer_timeline, replay_bot_match,
    replay_hash_snapshot,
    setup::{deal_hand, next_dealer},
    setup_match, BriarCircuitSeat, PassDirection, SetupOptions, ViewerExportClass,
};
use engine_core::{HashValue, Seed, SeededRng};

const REQUIRED_TRACES: &[&str] = &[
    "setup-four-seat-deterministic-deal.trace.json",
    "invalid-seat-count-below.trace.json",
    "invalid-seat-count-above.trace.json",
    "deal-private-no-leak.trace.json",
    "pass-left-atomic-exchange.trace.json",
    "pass-right-atomic-exchange.trace.json",
    "pass-across-atomic-exchange.trace.json",
    "hold-hand-no-pass.trace.json",
    "pass-choice-in-flight-no-leak.trace.json",
    "invalid-pass-not-three.trace.json",
    "invalid-pass-unowned-or-duplicate.trace.json",
    "two-clubs-forced-opening.trace.json",
    "follow-suit-forced.trace.json",
    "void-free-discard.trace.json",
    "first-trick-points-suppressed.trace.json",
    "first-trick-all-points-exception.trace.json",
    "hearts-not-broken-lead-diagnostic.trace.json",
    "only-hearts-lead-exception.trace.json",
    "heart-discard-breaks-hearts.trace.json",
    "queen-spades-does-not-break-hearts.trace.json",
    "off-suit-never-wins.trace.json",
    "trick-winner-leads-next.trace.json",
    "normal-hand-scoring.trace.json",
    "shoot-the-moon-fixed-addition.trace.json",
    "dealer-and-pass-cycle-rotation.trace.json",
    "threshold-unique-low-winner.trace.json",
    "threshold-low-tie-continues.trace.json",
    "invalid-wrong-seat-diagnostic.trace.json",
    "invalid-stale-diagnostic.trace.json",
    "l0-bot-action.trace.json",
    "l1-bot-pass-and-play.trace.json",
    "public-observer-no-leak.trace.json",
    "seat-private-pairwise-no-leak.trace.json",
    "public-replay-export-import.trace.json",
    "seat-private-replay-export-import.trace.json",
    "bot-vs-bot-full-match.trace.json",
    "wasm-exported-moon-terminal.trace.json",
];

#[test]
fn identical_seed_reproduces_identical_initial_deal() {
    let seats = canonical_seat_ids();
    let first = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("first setup");
    let second = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("second setup");

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
    for seat in BriarCircuitSeat::ALL {
        assert_eq!(
            first.hand_for_internal(seat),
            second.hand_for_internal(seat)
        );
    }
}

#[test]
fn different_seed_changes_the_deal_but_not_public_rotation_facts() {
    let seats = canonical_seat_ids();
    let first = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("first setup");
    let second = setup_match(Seed(1601), &seats, &SetupOptions::default()).expect("second setup");

    assert_ne!(
        first.hand_for_internal(BriarCircuitSeat::Seat0),
        second.hand_for_internal(BriarCircuitSeat::Seat0)
    );
    assert_eq!(first.dealer, second.dealer);
    assert_eq!(first.pass_direction(), second.pass_direction());
}

#[test]
fn sequential_hand_deals_are_replayable_from_seed_and_hand_index() {
    let mut first_rng = SeededRng::from_seed(Seed(1600));
    let first_hand = deal_hand(&mut first_rng, BriarCircuitSeat::Seat0, 0).expect("hand 0");
    let second_hand =
        deal_hand(&mut first_rng, next_dealer(BriarCircuitSeat::Seat0), 1).expect("hand 1");
    let hold_hand =
        deal_hand(&mut first_rng, BriarCircuitSeat::Seat3, 3).expect("hold hand fixture");

    let mut replay_rng = SeededRng::from_seed(Seed(1600));
    let replay_first = deal_hand(&mut replay_rng, BriarCircuitSeat::Seat0, 0).expect("hand 0");
    let replay_second =
        deal_hand(&mut replay_rng, next_dealer(BriarCircuitSeat::Seat0), 1).expect("hand 1");
    let replay_hold = deal_hand(&mut replay_rng, BriarCircuitSeat::Seat3, 3).expect("hand 3");

    assert_eq!(first_hand, replay_first);
    assert_eq!(second_hand, replay_second);
    assert_eq!(hold_hand, replay_hold);
    assert_eq!(first_hand.pass_direction, PassDirection::Left);
    assert_eq!(second_hand.pass_direction, PassDirection::Right);
    assert_eq!(hold_hand.pass_direction, PassDirection::Hold);
    assert_ne!(first_hand.hands, second_hand.hands);
}

#[test]
fn replay_hash_snapshot_reproduces_for_identical_seed() {
    let seats = canonical_seat_ids();
    let first = setup_match(Seed(1614), &seats, &SetupOptions::default()).expect("first setup");
    let second = setup_match(Seed(1614), &seats, &SetupOptions::default()).expect("second setup");

    assert_eq!(replay_hash_snapshot(&first), replay_hash_snapshot(&second));
}

#[test]
fn viewer_exports_round_trip_without_seed_or_deck_order() {
    let seats = canonical_seat_ids();
    let state = setup_match(Seed(1615), &seats, &SetupOptions::default()).expect("setup");
    let public_export = export_viewer_timeline(&state, ViewerExportClass::Public);
    let seat_export = export_viewer_timeline(
        &state,
        ViewerExportClass::SeatPrivate(BriarCircuitSeat::Seat0),
    );

    assert_eq!(
        import_viewer_timeline(&public_export),
        Ok(public_export.clone())
    );
    assert_eq!(
        import_viewer_timeline(&seat_export),
        Ok(seat_export.clone())
    );

    let public_payload = format!("{public_export:?}");
    assert!(!public_payload.contains("Seed("));
    assert!(!public_payload.contains("deck"));
    assert!(!public_payload.contains("private_hands"));

    let private_payload = format!("{seat_export:?}");
    assert!(private_payload.contains("seat_0"));
    assert!(!private_payload.contains("Seed("));
    assert!(!private_payload.contains("deck"));
}

#[test]
fn golden_trace_minimum_inventory_exists() {
    let trace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden_traces");

    for trace in REQUIRED_TRACES {
        let path = trace_dir.join(trace);
        assert!(path.exists(), "missing golden trace {}", path.display());
        let payload = std::fs::read_to_string(&path).expect("trace readable");
        assert!(
            payload.contains("\"schema_version\":1") || payload.contains("\"schema_version\": 1")
        );
        assert!(
            payload.contains("\"game_id\":\"briar_circuit\"")
                || payload.contains("\"game\": \"briar_circuit\"")
        );
        assert!(
            payload.contains("\"rules_version\":\"briar-circuit-rules-v1\"")
                || payload.contains("\"rules_version\": \"briar-circuit-rules-v1\"")
        );
        assert!(payload.contains("migration_notes"));
    }
}

#[test]
fn characterization_domain_fixture_artifacts_are_pinned() {
    let moon_fixture = include_str!("../data/fixtures/briar_circuit_moon.fixture.json");
    let first_trick_exception_fixture =
        include_str!("../data/fixtures/briar_circuit_first_trick_exception.fixture.json");

    assert_eq!(
        HashValue::from_stable_bytes(moon_fixture.as_bytes()),
        HashValue(12129920730792203110)
    );
    assert_eq!(
        HashValue::from_stable_bytes(first_trick_exception_fixture.as_bytes()),
        HashValue(16932830783837267987)
    );
    assert!(moon_fixture.contains("\"trace_id\": \"briar_circuit_moon\""));
    assert!(first_trick_exception_fixture
        .contains("\"trace_id\": \"briar_circuit_first_trick_exception\""));
    assert!(!moon_fixture.contains("selector"));
    assert!(!first_trick_exception_fixture.contains("selector"));
}

#[test]
fn bot_match_replay_is_deterministic_and_reaches_a_multi_hand_terminal() {
    // A fixed seed fully determines the L1 bot match, so two replays must be identical
    // and must reach a unique-low-score terminal across several hands.
    for seed in [1600_u64, 1601, 1700, 9001] {
        let first = replay_bot_match(seed, 8192).expect("bot match replays");
        let second = replay_bot_match(seed, 8192).expect("bot match replays");
        assert_eq!(first, second, "seed {seed} replay is deterministic");
        assert!(first.terminal, "seed {seed} reaches a terminal outcome");
        assert!(
            first.winner.is_some(),
            "seed {seed} has a unique-low winner"
        );
        assert!(
            first.hands_played >= 2,
            "seed {seed} plays multiple hands (got {})",
            first.hands_played
        );
        // The winner holds the unique lowest cumulative score.
        let winner = first.winner.expect("winner");
        let low = *first.cumulative_scores.iter().min().expect("scores");
        assert_eq!(first.cumulative_scores[winner.index()], low);
        assert_eq!(
            first
                .cumulative_scores
                .iter()
                .filter(|score| **score == low)
                .count(),
            1,
            "seed {seed} winner low score is unique"
        );
    }
}

#[test]
fn bot_match_replay_diverges_across_seeds() {
    // Different seeds produce different deals, so their replay hashes should differ.
    let a = replay_bot_match(1600, 8192).expect("replay");
    let b = replay_bot_match(2600, 8192).expect("replay");
    assert_ne!(
        a.snapshot.state_hash, b.snapshot.state_hash,
        "distinct seeds yield distinct terminal state hashes"
    );
}

#[test]
fn bot_match_replay_records_pass_cycle_dealer_rotation_and_tie_continuation() {
    // Pass direction and dealer rotation are deterministic from the hand index, so the
    // first four hands of any multi-hand match follow the published cycle.
    let canonical = replay_bot_match(1600, 8192).expect("replay");
    assert_eq!(
        canonical.pass_directions[..4],
        ["left", "right", "across", "hold"]
    );
    assert_eq!(
        canonical.dealers[..4],
        ["seat_0", "seat_1", "seat_2", "seat_3"]
    );
    assert_eq!(canonical.tie_continuation_hands, 0);

    // Seed 1625 crosses the threshold with a tied low score, so the match continues.
    let tie = replay_bot_match(1625, 8192).expect("replay");
    assert!(
        tie.tie_continuation_hands >= 1,
        "seed 1625 continues past a tied-low threshold hand"
    );
    assert!(tie.terminal && tie.winner.is_some());
}
