use blackglass_pact::{
    canonical_seat_ids, legal_action_paths, legal_action_tree, setup_match,
    setup_match_with_scores, BlackglassL0Bot, BlackglassL1Bot, BlackglassSeat, Card, CardId, Phase,
    Rank, SetupOptions, Suit,
};
use engine_core::{Actor, SeatId, Seed};

const BOT_TRACE_FIXTURES: &[(&str, &str)] = &[
    (
        "l0-blind-bid-and-play.trace.json",
        include_str!("golden_traces/l0-blind-bid-and-play.trace.json"),
    ),
    (
        "l1-partnership-bid-nil-and-play.trace.json",
        include_str!("golden_traces/l1-partnership-bid-nil-and-play.trace.json"),
    ),
    (
        "mixed-l0-l1-full-match.trace.json",
        include_str!("golden_traces/mixed-l0-l1-full-match.trace.json"),
    ),
];

#[test]
fn l0_selects_seeded_legal_action_deterministically() {
    let state = setup_match(Seed(1823), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    let bot = BlackglassL0Bot::new(Seed(99));

    let left = bot
        .select_decision(&state, BlackglassSeat::East)
        .expect("l0 decision");
    let right = bot
        .select_decision(&state, BlackglassSeat::East)
        .expect("l0 decision repeats");
    let legal = legal_action_paths(&legal_action_tree(&state, &actor_for(BlackglassSeat::East)));

    assert_eq!(left, right);
    assert!(legal.contains(&left.action_path));
    assert!(!left.explanation.contains("two_clubs"));
}

#[test]
fn l1_blind_nil_uses_public_score_thresholds_only() {
    let state = setup_match_with_scores(
        Seed(1824),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 250],
    )
    .expect("blind setup succeeds");

    let decision = BlackglassL1Bot
        .select_decision(&state, BlackglassSeat::South)
        .expect("l1 blind decision");

    assert_eq!(decision.action_path.segments, ["blind_nil", "declare"]);
    assert_no_card_identity(&decision.explanation);
}

#[test]
fn l1_bids_nil_for_low_safe_own_hand_and_numeric_for_controls() {
    let mut nil_state = setup_match(Seed(1825), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    set_hand(
        &mut nil_state,
        BlackglassSeat::East,
        vec![
            card(Rank::Two, Suit::Clubs),
            card(Rank::Three, Suit::Diamonds),
            card(Rank::Four, Suit::Hearts),
        ],
    );
    let nil_decision = BlackglassL1Bot
        .select_decision(&nil_state, BlackglassSeat::East)
        .expect("l1 nil bid");
    assert_eq!(nil_decision.action_path.segments, ["bid", "nil"]);

    let mut bid_state = nil_state.clone();
    set_hand(
        &mut bid_state,
        BlackglassSeat::East,
        vec![
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Ace, Suit::Clubs),
        ],
    );
    let bid_decision = BlackglassL1Bot
        .select_decision(&bid_state, BlackglassSeat::East)
        .expect("l1 numeric bid");
    assert!(matches!(
        bid_decision.action_path.segments.as_slice(),
        [family, value] if family == "bid" && value != "nil"
    ));
}

#[test]
fn l1_play_selects_lowest_legal_card_without_leaking_other_hands() {
    let mut state = setup_match(Seed(1826), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    state.phase = Phase::PlayingTrick {
        leader: BlackglassSeat::East,
        next: BlackglassSeat::East,
        plays: Vec::new(),
        trick_index: 0,
    };
    set_hand(
        &mut state,
        BlackglassSeat::East,
        vec![card(Rank::Ace, Suit::Clubs), card(Rank::Two, Suit::Clubs)],
    );
    set_hand(
        &mut state,
        BlackglassSeat::South,
        vec![card(Rank::King, Suit::Spades)],
    );

    let decision = BlackglassL1Bot
        .select_decision(&state, BlackglassSeat::East)
        .expect("l1 play");

    assert_eq!(decision.action_path.segments, ["play", "two_clubs"]);
    assert!(!decision.explanation.contains("king_spades"));
}

#[test]
fn bot_trace_inventory_exists_and_stays_viewer_safe() {
    for (name, fixture) in BOT_TRACE_FIXTURES {
        assert!(
            fixture.contains("\"schema_version\":1"),
            "{name} records trace schema"
        );
        assert!(
            fixture.contains("\"game_id\":\"blackglass_pact\""),
            "{name} records game id"
        );
        assert!(
            fixture.contains("\"fixture_kind\":\"bot-smoke\""),
            "{name} records bot-smoke fixture kind"
        );
        assert!(
            fixture.contains("\"hidden_information_policy\":\"viewer_authorized_only\""),
            "{name} records viewer boundary"
        );
        assert_no_card_identity(fixture);
    }
}

fn actor_for(seat: BlackglassSeat) -> Actor {
    Actor {
        seat_id: SeatId::from_zero_based_index(seat.index() as u32),
    }
}

fn set_hand(
    state: &mut blackglass_pact::BlackglassPactState,
    seat: BlackglassSeat,
    hand: Vec<CardId>,
) {
    if let Some((_, current)) = state
        .private_hands
        .iter_mut()
        .find(|(candidate, _)| *candidate == seat)
    {
        *current = hand;
    } else {
        state.private_hands.push((seat, hand));
    }
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
}

fn assert_no_card_identity(surface: &str) {
    for card_id in blackglass_pact::canonical_deck() {
        assert!(!surface.contains(&card_id.as_str()));
    }
}
