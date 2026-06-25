use std::collections::BTreeSet;

use engine_core::{SeatId, Seed, Viewer};
use meldfall_ledger::{
    actions::LayoffPosition,
    cards::{canonical_deck, ranks_are_consecutive_low_or_high, Card, CardId, Rank, Suit},
    effects::{effect_stable_string, MeldfallEffect},
    rules::{
        lay_off_card, table_new_meld, take_new_meld_from_hand, validate_new_meld,
        validate_owned_meld,
    },
    setup::{deal_for_round, default_seats, setup_match, validate_seat_count, SetupOptions},
    state::{MatchState, MeldId, MeldKind, TurnOrdinal},
    visibility::{project_public_tableau, project_tableau_for_viewer},
    STANDARD_CARD_COUNT,
};

#[test]
fn canonical_deck_has_52_unique_local_card_ids() {
    let deck = canonical_deck();
    let unique = deck.iter().copied().collect::<BTreeSet<_>>();

    assert_eq!(deck.len(), STANDARD_CARD_COUNT as usize);
    assert_eq!(unique.len(), STANDARD_CARD_COUNT as usize);
    assert_eq!(deck[0], Card::new(Rank::Ace, Suit::Clubs).id());
    assert_eq!(deck[51], Card::new(Rank::King, Suit::Spades).id());
    assert_eq!(CardId::parse("ace_clubs"), Some(deck[0]));
    assert_eq!(deck[51].as_str(), "king_spades");
    assert_eq!(deck[51].card().public_label(), "KS");
}

#[test]
fn rummy_card_values_and_run_helpers_are_pinned() {
    assert_eq!(Rank::Ace.score_value(), 15);
    assert_eq!(Rank::King.score_value(), 10);
    assert_eq!(Rank::Ten.score_value(), 10);
    assert_eq!(Rank::Nine.score_value(), 9);

    assert!(ranks_are_consecutive_low_or_high(&[
        Rank::Ace,
        Rank::Two,
        Rank::Three,
    ]));
    assert!(ranks_are_consecutive_low_or_high(&[
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ]));
    assert!(!ranks_are_consecutive_low_or_high(&[
        Rank::King,
        Rank::Ace,
        Rank::Two,
    ]));
}

#[test]
fn setup_deals_correct_counts_for_supported_seat_counts() {
    for (seat_count, hand_size, expected_stock) in
        [(2, 13, 25), (3, 7, 30), (4, 7, 23), (5, 7, 16), (6, 7, 9)]
    {
        let seats = default_seats(seat_count).expect("seat count supported");
        let setup = setup_match(Seed(1900), &seats, &SetupOptions::default()).expect("setup ok");
        let public = setup.public_view();

        assert_eq!(setup.private_hands.len(), seat_count);
        assert!(setup
            .private_hands
            .iter()
            .all(|hand| hand.len() == hand_size));
        assert_eq!(public.hand_counts.len(), seat_count);
        assert!(public
            .hand_counts
            .iter()
            .all(|(_, count)| *count == hand_size));
        assert_eq!(public.discard_count, 1);
        assert_eq!(public.stock_count, expected_stock);
        assert_eq!(setup.stock.len(), expected_stock);
    }
}

#[test]
fn setup_rejects_unsupported_seat_counts_with_diagnostics() {
    for rejected in [1, 7] {
        let diagnostic = validate_seat_count(rejected).expect_err("seat count rejected");
        assert_eq!(diagnostic.code, "ML_INVALID_SEAT_COUNT");
        assert!(diagnostic.message.contains(&rejected.to_string()));
    }
}

#[test]
fn setup_is_deterministic_for_seed_and_seat_count() {
    let seats = default_seats(4).expect("seat count supported");
    let first = setup_match(Seed(1901), &seats, &SetupOptions::default()).expect("setup ok");
    let second = setup_match(Seed(1901), &seats, &SetupOptions::default()).expect("setup ok");
    let different = setup_match(Seed(1902), &seats, &SetupOptions::default()).expect("setup ok");

    assert_eq!(first, second);
    assert_ne!(first.private_hands, different.private_hands);
    assert_ne!(first.stock, different.stock);
}

#[test]
fn deal_order_starts_left_of_dealer_clockwise() {
    let deal = deal_for_round(Seed(1903), 2, 6, 7).expect("deal ok");

    assert_eq!(deal.dealer_index, 2);
    assert_eq!(deal.deal_order, vec![3, 4, 5, 0, 1, 2]);
}

#[test]
fn public_setup_view_is_count_only() {
    let seats = default_seats(6).expect("seat count supported");
    let setup = setup_match(Seed(1904), &seats, &SetupOptions::default()).expect("setup ok");
    let public = setup.public_view();

    assert_eq!(public.hand_counts.len(), 6);
    assert_eq!(public.stock_count, 9);
    assert_eq!(public.discard_count, 1);
    assert!(!format!("{public:?}").contains("ace_"));
    assert!(!format!("{public:?}").contains("king_"));
}

#[test]
fn meld_sets_accept_three_or_four_same_rank_distinct_suits() {
    let three = vec![
        Card::new(Rank::Seven, Suit::Clubs).id(),
        Card::new(Rank::Seven, Suit::Diamonds).id(),
        Card::new(Rank::Seven, Suit::Hearts).id(),
    ];
    let four = vec![
        Card::new(Rank::Queen, Suit::Clubs).id(),
        Card::new(Rank::Queen, Suit::Diamonds).id(),
        Card::new(Rank::Queen, Suit::Hearts).id(),
        Card::new(Rank::Queen, Suit::Spades).id(),
    ];

    assert_eq!(
        validate_new_meld(&three).expect("three-card set accepted"),
        MeldKind::Set { rank: Rank::Seven }
    );
    assert_eq!(
        validate_new_meld(&four).expect("four-card set accepted"),
        MeldKind::Set { rank: Rank::Queen }
    );
}

#[test]
fn meld_sets_reject_too_small_duplicate_mixed_rank_and_oversized_shapes() {
    let too_small = vec![
        Card::new(Rank::Seven, Suit::Clubs).id(),
        Card::new(Rank::Seven, Suit::Diamonds).id(),
    ];
    let duplicate = vec![
        Card::new(Rank::Seven, Suit::Clubs).id(),
        Card::new(Rank::Seven, Suit::Clubs).id(),
        Card::new(Rank::Seven, Suit::Diamonds).id(),
    ];
    let mixed_rank = vec![
        Card::new(Rank::Seven, Suit::Clubs).id(),
        Card::new(Rank::Eight, Suit::Diamonds).id(),
        Card::new(Rank::Seven, Suit::Hearts).id(),
    ];
    let oversized_same_rank = vec![
        Card::new(Rank::Seven, Suit::Clubs).id(),
        Card::new(Rank::Seven, Suit::Diamonds).id(),
        Card::new(Rank::Seven, Suit::Hearts).id(),
        Card::new(Rank::Seven, Suit::Spades).id(),
        Card::new(Rank::Eight, Suit::Spades).id(),
    ];

    assert_eq!(
        validate_new_meld(&too_small)
            .expect_err("too-small meld rejected")
            .code,
        "ML_MELD_TOO_SMALL"
    );
    assert_eq!(
        validate_new_meld(&duplicate)
            .expect_err("duplicate card rejected")
            .code,
        "ML_MELD_DUPLICATE_CARD"
    );
    assert_eq!(
        validate_new_meld(&mixed_rank)
            .expect_err("mixed-rank set rejected")
            .code,
        "ML_INVALID_MELD_SHAPE"
    );
    assert_eq!(
        validate_new_meld(&oversized_same_rank)
            .expect_err("oversized set rejected")
            .code,
        "ML_INVALID_MELD_SHAPE"
    );
}

#[test]
fn meld_runs_accept_same_suit_consecutive_with_ace_low_or_high() {
    let ace_low = vec![
        Card::new(Rank::Ace, Suit::Clubs).id(),
        Card::new(Rank::Two, Suit::Clubs).id(),
        Card::new(Rank::Three, Suit::Clubs).id(),
    ];
    let middle = vec![
        Card::new(Rank::Eight, Suit::Hearts).id(),
        Card::new(Rank::Nine, Suit::Hearts).id(),
        Card::new(Rank::Ten, Suit::Hearts).id(),
        Card::new(Rank::Jack, Suit::Hearts).id(),
    ];
    let ace_high = vec![
        Card::new(Rank::Queen, Suit::Spades).id(),
        Card::new(Rank::King, Suit::Spades).id(),
        Card::new(Rank::Ace, Suit::Spades).id(),
    ];

    assert_eq!(
        validate_new_meld(&ace_low).expect("ace-low run accepted"),
        MeldKind::Run { suit: Suit::Clubs }
    );
    assert_eq!(
        validate_new_meld(&middle).expect("middle run accepted"),
        MeldKind::Run { suit: Suit::Hearts }
    );
    assert_eq!(
        validate_new_meld(&ace_high).expect("ace-high run accepted"),
        MeldKind::Run { suit: Suit::Spades }
    );
}

#[test]
fn meld_runs_reject_mixed_suit_gap_and_around_the_corner() {
    let mixed_suit = vec![
        Card::new(Rank::Four, Suit::Clubs).id(),
        Card::new(Rank::Five, Suit::Clubs).id(),
        Card::new(Rank::Six, Suit::Diamonds).id(),
    ];
    let gapped = vec![
        Card::new(Rank::Four, Suit::Clubs).id(),
        Card::new(Rank::Five, Suit::Clubs).id(),
        Card::new(Rank::Seven, Suit::Clubs).id(),
    ];
    let king_ace_two = vec![
        Card::new(Rank::King, Suit::Clubs).id(),
        Card::new(Rank::Ace, Suit::Clubs).id(),
        Card::new(Rank::Two, Suit::Clubs).id(),
    ];
    let queen_king_ace_two = vec![
        Card::new(Rank::Queen, Suit::Clubs).id(),
        Card::new(Rank::King, Suit::Clubs).id(),
        Card::new(Rank::Ace, Suit::Clubs).id(),
        Card::new(Rank::Two, Suit::Clubs).id(),
    ];

    for rejected in [mixed_suit, gapped, king_ace_two, queen_king_ace_two] {
        assert_eq!(
            validate_new_meld(&rejected)
                .expect_err("invalid run shape rejected")
                .code,
            "ML_INVALID_MELD_SHAPE"
        );
    }
}

#[test]
fn owned_meld_validation_and_take_are_atomic() {
    let meld_cards = vec![
        Card::new(Rank::Four, Suit::Clubs).id(),
        Card::new(Rank::Five, Suit::Clubs).id(),
        Card::new(Rank::Six, Suit::Clubs).id(),
    ];
    let unplayed = Card::new(Rank::King, Suit::Hearts).id();
    let mut hand = vec![meld_cards[0], unplayed, meld_cards[1], meld_cards[2]];

    let missing = vec![
        Card::new(Rank::Ace, Suit::Spades).id(),
        Card::new(Rank::Two, Suit::Spades).id(),
        Card::new(Rank::Three, Suit::Spades).id(),
    ];
    assert_eq!(
        validate_owned_meld(&hand, &missing)
            .expect_err("missing card rejected")
            .code,
        "ML_MELD_CARD_NOT_OWNED"
    );
    assert_eq!(hand.len(), 4);

    let group = take_new_meld_from_hand(&mut hand, &meld_cards, MeldId(4), 2, TurnOrdinal(9))
        .expect("owned run accepted");

    assert_eq!(hand, vec![unplayed]);
    assert_eq!(group.id, MeldId(4));
    assert_eq!(group.kind, MeldKind::Run { suit: Suit::Clubs });
    assert_eq!(group.origin_seat, 2);
    assert_eq!(group.cards.len(), 3);
    assert!(group
        .cards
        .iter()
        .all(|card| card.played_by == 2 && card.score_credit_owner == 2));
}

#[test]
fn table_new_meld_moves_cards_to_public_tableau_with_stable_credit() {
    let mut state = sample_state();
    let meld_cards = vec![
        Card::new(Rank::Nine, Suit::Clubs).id(),
        Card::new(Rank::Nine, Suit::Diamonds).id(),
        Card::new(Rank::Nine, Suit::Hearts).id(),
    ];
    let unplayed = Card::new(Rank::Ace, Suit::Spades).id();
    state.round.seats[1].hand = vec![unplayed, meld_cards[0], meld_cards[1], meld_cards[2]];

    let group =
        table_new_meld(&mut state.round, 1, &meld_cards, TurnOrdinal(3)).expect("meld tables");

    assert_eq!(group.id, MeldId(0));
    assert_eq!(group.origin_seat, 1);
    assert_eq!(group.kind, MeldKind::Set { rank: Rank::Nine });
    assert_eq!(state.round.seats[1].hand, vec![unplayed]);
    assert_eq!(state.round.tableau.groups, vec![group.clone()]);
    assert_eq!(state.round.round_played_scores[1], 27);
    assert!(group
        .cards
        .iter()
        .all(|card| card.played_by == 1 && card.score_credit_owner == 1));

    let next_cards = vec![
        Card::new(Rank::Four, Suit::Spades).id(),
        Card::new(Rank::Five, Suit::Spades).id(),
        Card::new(Rank::Six, Suit::Spades).id(),
    ];
    state.round.seats[2].hand = next_cards.clone();
    let next =
        table_new_meld(&mut state.round, 2, &next_cards, TurnOrdinal(4)).expect("second meld");
    assert_eq!(next.id, MeldId(1));
    assert_eq!(state.round.round_played_scores[2], 15);
}

#[test]
fn public_tableau_projection_is_identical_for_every_viewer_and_hand_safe() {
    let mut state = sample_state();
    let meld_cards = vec![
        Card::new(Rank::Queen, Suit::Spades).id(),
        Card::new(Rank::King, Suit::Spades).id(),
        Card::new(Rank::Ace, Suit::Spades).id(),
    ];
    let hidden_remainder = Card::new(Rank::Two, Suit::Diamonds).id();
    state.round.seats[3].hand = vec![
        meld_cards[0],
        hidden_remainder,
        meld_cards[1],
        meld_cards[2],
    ];
    table_new_meld(&mut state.round, 3, &meld_cards, TurnOrdinal(8)).expect("meld tables");

    let public = project_public_tableau(&state.round.tableau);
    let observer = project_tableau_for_viewer(&state.round.tableau, &Viewer { seat_id: None });
    assert_eq!(observer, public);

    for viewer in state.seats.iter().cloned() {
        let seat_view = project_tableau_for_viewer(
            &state.round.tableau,
            &Viewer {
                seat_id: Some(viewer),
            },
        );
        assert_eq!(seat_view, public);
    }

    assert_eq!(
        public.stable_string(),
        "meld_0:run:spades:origin=3:cards=[queen_spades:played_by=3:credit=3:turn=8,king_spades:played_by=3:credit=3:turn=8,ace_spades:played_by=3:credit=3:turn=8]"
    );
    assert!(!public.stable_string().contains("two_diamonds"));
}

#[test]
fn lay_off_onto_own_meld_extends_run_and_credits_player() {
    let mut state = sample_state();
    let base = vec![
        Card::new(Rank::Four, Suit::Clubs).id(),
        Card::new(Rank::Five, Suit::Clubs).id(),
        Card::new(Rank::Six, Suit::Clubs).id(),
    ];
    let layoff = Card::new(Rank::Seven, Suit::Clubs).id();
    state.round.seats[0].hand = vec![base[0], base[1], base[2], layoff];
    table_new_meld(&mut state.round, 0, &base, TurnOrdinal(1)).expect("base meld tables");

    let effect = lay_off_card(
        &mut state.round,
        0,
        layoff,
        MeldId(0),
        LayoffPosition::Append,
        TurnOrdinal(2),
    )
    .expect("own lay-off accepted");

    assert_eq!(state.round.seats[0].hand, Vec::<CardId>::new());
    assert_eq!(state.round.round_played_scores[0], 22);
    assert_eq!(state.round.tableau.groups[0].origin_seat, 0);
    assert_eq!(state.round.tableau.groups[0].cards.len(), 4);
    assert_eq!(
        state.round.tableau.groups[0].cards[3].stable_string(),
        "seven_clubs:played_by=0:credit=0:turn=2"
    );
    assert_eq!(
        effect_stable_string(&effect),
        "LayOff:seat=0:meld=meld_0:card=seven_clubs:played_by=0:credit=0:turn=2:position=append"
    );
}

#[test]
fn lay_off_onto_opponent_meld_preserves_origin_and_credits_laying_off_seat() {
    let mut state = sample_state();
    let base = vec![
        Card::new(Rank::Nine, Suit::Clubs).id(),
        Card::new(Rank::Nine, Suit::Diamonds).id(),
        Card::new(Rank::Nine, Suit::Hearts).id(),
    ];
    let layoff = Card::new(Rank::Nine, Suit::Spades).id();
    state.round.seats[0].hand = base.clone();
    state.round.seats[2].hand = vec![layoff];
    table_new_meld(&mut state.round, 0, &base, TurnOrdinal(1)).expect("base set tables");

    let effect = lay_off_card(
        &mut state.round,
        2,
        layoff,
        MeldId(0),
        LayoffPosition::Append,
        TurnOrdinal(3),
    )
    .expect("opponent lay-off accepted");

    let group = &state.round.tableau.groups[0];
    assert_eq!(group.origin_seat, 0);
    assert_eq!(group.cards[3].played_by, 2);
    assert_eq!(group.cards[3].score_credit_owner, 2);
    assert_eq!(state.round.round_played_scores[0], 27);
    assert_eq!(state.round.round_played_scores[2], 9);
    assert!(matches!(
        effect.payload,
        MeldfallEffect::LayOff { seat: 2, .. }
    ));
}

#[test]
fn invalid_lay_off_gap_wrong_rank_and_missing_target_reject_without_mutation() {
    let mut state = sample_state();
    let run = vec![
        Card::new(Rank::Four, Suit::Hearts).id(),
        Card::new(Rank::Five, Suit::Hearts).id(),
        Card::new(Rank::Six, Suit::Hearts).id(),
    ];
    let gap = Card::new(Rank::Eight, Suit::Hearts).id();
    state.round.seats[0].hand = run.clone();
    state.round.seats[1].hand = vec![gap];
    table_new_meld(&mut state.round, 0, &run, TurnOrdinal(1)).expect("base run tables");
    let before = state.round.clone();

    let diagnostic = lay_off_card(
        &mut state.round,
        1,
        gap,
        MeldId(0),
        LayoffPosition::Append,
        TurnOrdinal(2),
    )
    .expect_err("gap lay-off rejected");
    assert_eq!(diagnostic.code, "ML_INVALID_LAYOFF");
    assert!(!diagnostic.message.contains("eight_hearts"));
    assert_eq!(state.round, before);

    let wrong_rank = Card::new(Rank::Ten, Suit::Clubs).id();
    state.round.seats[1].hand = vec![wrong_rank];
    let diagnostic = lay_off_card(
        &mut state.round,
        1,
        wrong_rank,
        MeldId(99),
        LayoffPosition::Append,
        TurnOrdinal(3),
    )
    .expect_err("missing target rejected");
    assert_eq!(diagnostic.code, "ML_UNKNOWN_MELD");

    let set = vec![
        Card::new(Rank::Queen, Suit::Clubs).id(),
        Card::new(Rank::Queen, Suit::Diamonds).id(),
        Card::new(Rank::Queen, Suit::Hearts).id(),
    ];
    state.round.seats[2].hand = set.clone();
    table_new_meld(&mut state.round, 2, &set, TurnOrdinal(4)).expect("base set tables");
    state.round.seats[1].hand = vec![wrong_rank];
    let before_wrong_rank = state.round.clone();
    let diagnostic = lay_off_card(
        &mut state.round,
        1,
        wrong_rank,
        MeldId(1),
        LayoffPosition::Append,
        TurnOrdinal(5),
    )
    .expect_err("wrong-rank lay-off rejected");
    assert_eq!(diagnostic.code, "ML_INVALID_LAYOFF");
    assert!(!diagnostic.message.contains("ten_clubs"));
    assert_eq!(state.round, before_wrong_rank);
}

fn sample_state() -> MatchState {
    let seats = vec![
        SeatId("seat_0".to_owned()),
        SeatId("seat_1".to_owned()),
        SeatId("seat_2".to_owned()),
        SeatId("seat_3".to_owned()),
    ];
    let setup = setup_match(Seed(1907), &seats, &SetupOptions::default()).expect("setup ok");
    MatchState::from_initial_setup(setup)
}
