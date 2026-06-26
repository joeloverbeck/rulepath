use std::collections::BTreeSet;

use engine_core::{SeatId, Seed, Viewer};
use meldfall_ledger::{
    actions::draw_source_action_tree,
    actions::LayoffPosition,
    cards::{canonical_deck, ranks_are_consecutive_low_or_high, Card, CardId, Rank, Suit},
    effects::{effect_stable_string, MeldfallEffect},
    rules::{
        advance_to_next_round, discard_card, draw_from_discard, draw_from_stock,
        finish_turn_after_table_plays, lay_off_card, table_new_meld, take_new_meld_from_hand,
        validate_new_meld, validate_owned_meld,
    },
    scoring::{
        apply_round_deltas, card_score, settle_round, terminal_outcome_for_scores_after_deltas,
    },
    setup::{
        deal_for_round, deal_seed_for_round, default_seats, setup_match, validate_seat_count,
        SetupOptions,
    },
    state::{
        DiscardPickupCommitment, MatchState, MeldId, MeldKind, RoundEndReason, RoundEndSummary,
        TurnOrdinal, TurnPhase,
    },
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
fn per_round_deal_seed_keeps_round_zero_and_derives_later_rounds() {
    let base = Seed(1902);

    assert_eq!(deal_seed_for_round(base, 0), base);
    assert_eq!(deal_seed_for_round(base, 1), deal_seed_for_round(base, 1));
    assert_ne!(deal_seed_for_round(base, 1), base);
    assert_ne!(deal_seed_for_round(base, 1), deal_seed_for_round(base, 2));
    assert_ne!(
        deal_seed_for_round(base, 1),
        deal_seed_for_round(Seed(1903), 1)
    );
}

#[test]
fn deal_order_starts_left_of_dealer_clockwise() {
    let deal = deal_for_round(Seed(1903), 2, 6, 7).expect("deal ok");

    assert_eq!(deal.dealer_index, 2);
    assert_eq!(deal.deal_order, vec![3, 4, 5, 0, 1, 2]);
}

#[test]
fn advance_to_next_round_resets_round_only_state_and_preserves_match_scores() {
    let mut state = sample_state();
    let old_summary = state.round.stable_internal_summary();
    let meld_cards = vec![
        Card::new(Rank::Nine, Suit::Clubs).id(),
        Card::new(Rank::Nine, Suit::Diamonds).id(),
        Card::new(Rank::Nine, Suit::Hearts).id(),
    ];
    state.round.seats[0].hand = meld_cards.clone();
    table_new_meld(&mut state.round, 0, &meld_cards, TurnOrdinal(7)).expect("meld tables");
    state.round.pending_pickup = Some(DiscardPickupCommitment {
        selected_card: Card::new(Rank::Ace, Suit::Spades).id(),
        source_discard_index: 0,
        required_by_seat: 0,
    });
    state.round.round_end = Some(RoundEndSummary {
        reason: RoundEndReason::GoOutByFinalDiscard,
        seat_index: 0,
    });
    state.round.phase = TurnPhase::RoundSettled;
    state.cumulative_scores = vec![120, -15, 45, 0];

    advance_to_next_round(&mut state).expect("non-terminal round advances");

    assert_eq!(state.rounds_settled, 1);
    assert_eq!(state.dealer_index, 1);
    assert_eq!(state.round.active_seat_index, 2);
    assert_eq!(state.round.phase, TurnPhase::Draw);
    assert_eq!(state.cumulative_scores, vec![120, -15, 45, 0]);
    assert!(state.round.pending_pickup.is_none());
    assert!(state.round.round_end.is_none());
    assert!(state.round.tableau.groups.is_empty());
    assert_eq!(state.round.tableau.next_meld_id(), MeldId(0));
    assert_eq!(state.round.round_played_scores, vec![0, 0, 0, 0]);
    assert_eq!(state.round.discard.len(), 1);
    assert_eq!(state.round.stock.len(), 23);
    assert!(state.round.seats.iter().all(|seat| seat.hand.len() == 7));
    assert_ne!(state.round.stable_internal_summary(), old_summary);
    assert_round_conserves_deck(&state);
}

#[test]
fn advance_to_next_round_rotates_dealer_and_lead_for_supported_seat_counts() {
    for seat_count in 2..=6 {
        let seats = default_seats(seat_count).expect("supported seats");
        let setup = setup_match(
            Seed(1905),
            &seats,
            &SetupOptions {
                dealer_index: seat_count - 1,
                ..SetupOptions::default()
            },
        )
        .expect("setup ok");
        let mut state = MatchState::from_initial_setup(setup);
        state.round.phase = TurnPhase::RoundSettled;
        state.round.round_end = Some(RoundEndSummary {
            reason: RoundEndReason::StockExhausted,
            seat_index: state.round.active_seat_index,
        });

        advance_to_next_round(&mut state).expect("advance ok");

        assert_eq!(state.dealer_index, 0);
        assert_eq!(state.round.active_seat_index, 1);
        assert_eq!(state.rounds_settled, 1);
        assert_round_conserves_deck(&state);
    }
}

#[test]
fn advance_to_next_round_is_deterministic_for_same_seed_and_round_index() {
    fn settled_state(seed: u64) -> MatchState {
        let seats = default_seats(4).expect("supported seats");
        let setup = setup_match(Seed(seed), &seats, &SetupOptions::default()).expect("setup ok");
        let mut state = MatchState::from_initial_setup(setup);
        state.round.phase = TurnPhase::RoundSettled;
        state.round.round_end = Some(RoundEndSummary {
            reason: RoundEndReason::GoOutWithoutDiscard,
            seat_index: state.round.active_seat_index,
        });
        state
    }

    let mut first = settled_state(1910);
    let mut second = settled_state(1910);
    let mut different = settled_state(1911);

    advance_to_next_round(&mut first).expect("advance first");
    advance_to_next_round(&mut second).expect("advance second");
    advance_to_next_round(&mut different).expect("advance different");

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
    assert_ne!(
        first.stable_internal_summary(),
        different.stable_internal_summary()
    );
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

#[test]
fn draw_source_action_tree_exposes_stock_and_public_discard_indices_only() {
    let mut state = sample_state();
    state.round.stock = vec![Card::new(Rank::Ace, Suit::Spades).id()];
    state.round.discard = vec![
        Card::new(Rank::Four, Suit::Clubs).id(),
        Card::new(Rank::Five, Suit::Clubs).id(),
        Card::new(Rank::Six, Suit::Clubs).id(),
    ];

    let tree = draw_source_action_tree(engine_core::FreshnessToken(9), &state.round);

    assert_eq!(
        tree.root
            .choices
            .iter()
            .map(|choice| choice.segment.as_str())
            .collect::<Vec<_>>(),
        vec![
            "draw-stock",
            "draw-discard-0",
            "draw-discard-1",
            "draw-discard-2"
        ]
    );
    assert!(!format!("{tree:?}").contains("ace_spades"));
}

#[test]
fn stock_draw_moves_hidden_top_card_and_public_effect_hides_identity() {
    let mut state = sample_state();
    let hidden_bottom = Card::new(Rank::Ace, Suit::Clubs).id();
    let hidden_top = Card::new(Rank::King, Suit::Spades).id();
    state.round.active_seat_index = 0;
    state.round.stock = vec![hidden_bottom, hidden_top];
    state.round.discard = vec![Card::new(Rank::Four, Suit::Clubs).id()];
    state.round.seats[0].hand.clear();

    let effect = draw_from_stock(&mut state.round, 0).expect("stock draw accepted");

    assert_eq!(state.round.stock, vec![hidden_bottom]);
    assert_eq!(state.round.seats[0].hand, vec![hidden_top]);
    assert_eq!(state.round.phase.as_str(), "table");
    assert_eq!(
        effect_stable_string(&effect),
        "Draw:seat=0:source=stock:cards=1:stock_after=1:discard_after=1"
    );
    assert!(!effect_stable_string(&effect).contains("king_spades"));
}

#[test]
fn discard_pickup_takes_selected_card_plus_newer_and_meld_clears_commitment() {
    let mut state = sample_state();
    let four = Card::new(Rank::Four, Suit::Hearts).id();
    let five = Card::new(Rank::Five, Suit::Hearts).id();
    let six = Card::new(Rank::Six, Suit::Hearts).id();
    let seven = Card::new(Rank::Seven, Suit::Hearts).id();
    state.round.discard = vec![four, five, six, seven];
    state.round.seats[1].hand.clear();

    let effect = draw_from_discard(&mut state.round, 1, 1).expect("discard pickup accepted");

    assert_eq!(state.round.discard, vec![four]);
    assert_eq!(state.round.seats[1].hand, vec![five, six, seven]);
    assert_eq!(
        state.round.pending_pickup.as_ref().map(|pending| (
            pending.selected_card,
            pending.source_discard_index,
            pending.required_by_seat
        )),
        Some((five, 1, 1))
    );
    assert_eq!(
        effect_stable_string(&effect),
        "Draw:seat=1:source=discard:1:cards=3:stock_after=23:discard_after=1"
    );

    table_new_meld(&mut state.round, 1, &[five, six, seven], TurnOrdinal(2))
        .expect("using selected discard in meld clears commitment");
    assert_eq!(state.round.pending_pickup, None);
}

#[test]
fn discard_pickup_commitment_blocks_finish_and_discard_until_used() {
    let mut state = sample_state();
    let seven_clubs = Card::new(Rank::Seven, Suit::Clubs).id();
    let seven_diamonds = Card::new(Rank::Seven, Suit::Diamonds).id();
    let seven_hearts = Card::new(Rank::Seven, Suit::Hearts).id();
    state.round.discard = vec![seven_clubs];
    state.round.active_seat_index = 0;
    state.round.seats[0].hand = vec![seven_diamonds, seven_hearts];

    draw_from_discard(&mut state.round, 0, 0).expect("top discard pickup accepted");

    let finish_diagnostic = finish_turn_after_table_plays(&mut state.round, 0)
        .expect_err("top pickup must be used before finishing");
    assert_eq!(finish_diagnostic.code, "ML_PICKUP_COMMITMENT_UNSATISFIED");

    let discard_diagnostic = discard_card(&mut state.round, 0, seven_clubs)
        .expect_err("committed pickup cannot be discarded");
    assert_eq!(discard_diagnostic.code, "ML_PICKUP_COMMITMENT_UNSATISFIED");
    assert_eq!(state.round.discard, Vec::<CardId>::new());

    table_new_meld(
        &mut state.round,
        0,
        &[seven_clubs, seven_diamonds, seven_hearts],
        TurnOrdinal(3),
    )
    .expect("using top pickup in a meld clears commitment");
    finish_turn_after_table_plays(&mut state.round, 0)
        .expect("finish accepted after commitment use");
}

#[test]
fn normal_draw_meld_discard_turn_advances_to_next_draw() {
    let mut state = sample_state();
    let draw = Card::new(Rank::Ace, Suit::Clubs).id();
    let discard = Card::new(Rank::King, Suit::Spades).id();
    state.round.active_seat_index = 0;
    state.round.stock = vec![draw];
    state.round.seats[0].hand = vec![discard];

    draw_from_stock(&mut state.round, 0).expect("draw accepted");
    let phase = finish_turn_after_table_plays(&mut state.round, 0).expect("discard phase entered");
    assert_eq!(phase.as_str(), "discard");

    let effect = discard_card(&mut state.round, 0, discard).expect("discard accepted");

    assert_eq!(
        effect_stable_string(&effect),
        "Discard:seat=0:card=king_spades:discard_after=2"
    );
    assert_eq!(state.round.active_seat_index, 1);
    assert_eq!(state.round.phase.as_str(), "draw");
    assert_eq!(state.round.round_end, None);
}

#[test]
fn go_out_without_final_discard_settles_round_after_emptying_hand() {
    let mut state = sample_state();
    let meld = vec![
        Card::new(Rank::Three, Suit::Clubs).id(),
        Card::new(Rank::Three, Suit::Diamonds).id(),
        Card::new(Rank::Three, Suit::Hearts).id(),
    ];
    state.round.active_seat_index = 0;
    state.round.phase = meldfall_ledger::state::TurnPhase::Table;
    state.round.seats[0].hand = meld.clone();

    table_new_meld(&mut state.round, 0, &meld, TurnOrdinal(1)).expect("meld tables");
    let phase = finish_turn_after_table_plays(&mut state.round, 0).expect("round settles");

    assert_eq!(phase.as_str(), "round_settled");
    assert_eq!(
        state
            .round
            .round_end
            .expect("round end set")
            .stable_string(),
        "go_out_without_discard:seat=0"
    );
}

#[test]
fn go_out_by_final_discard_settles_round_after_discarding_last_card() {
    let mut state = sample_state();
    let final_card = Card::new(Rank::Two, Suit::Spades).id();
    state.round.active_seat_index = 2;
    state.round.phase = meldfall_ledger::state::TurnPhase::Discard;
    state.round.seats[2].hand = vec![final_card];

    discard_card(&mut state.round, 2, final_card).expect("final discard accepted");

    assert_eq!(state.round.phase.as_str(), "round_settled");
    assert_eq!(
        state
            .round
            .round_end
            .expect("round end set")
            .stable_string(),
        "go_out_by_final_discard:seat=2"
    );
    assert_eq!(state.round.discard.last().copied(), Some(final_card));
}

#[test]
fn stock_exhaustion_with_no_discard_draw_settles_round_and_wrong_phase_is_diagnostic() {
    let mut state = sample_state();
    state.round.active_seat_index = 1;
    state.round.stock.clear();
    state.round.discard.clear();

    assert!(
        meldfall_ledger::rules::settle_stock_exhaustion_if_no_discard_draw(&mut state.round, 1)
            .expect("stock exhaustion evaluated")
    );
    assert_eq!(state.round.phase.as_str(), "round_settled");
    assert_eq!(
        state
            .round
            .round_end
            .expect("round end set")
            .stable_string(),
        "stock_exhausted:seat=1"
    );

    let diagnostic = draw_from_stock(&mut state.round, 1).expect_err("settled round rejects draw");
    assert_eq!(diagnostic.code, "ML_WRONG_PHASE");
}

#[test]
fn wrong_seat_draw_is_diagnostic() {
    let mut state = sample_state();
    state.round.active_seat_index = 1;
    state.round.stock = vec![Card::new(Rank::Ace, Suit::Spades).id()];

    let diagnostic = draw_from_stock(&mut state.round, 0).expect_err("wrong seat rejected");
    assert_eq!(diagnostic.code, "ML_WRONG_SEAT");
}

#[test]
fn round_scoring_uses_tabled_credit_minus_hand_penalties_without_card_leak() {
    let mut state = sample_state();
    let base = vec![
        Card::new(Rank::Ace, Suit::Clubs).id(),
        Card::new(Rank::Ace, Suit::Diamonds).id(),
        Card::new(Rank::Ace, Suit::Hearts).id(),
    ];
    let layoff = Card::new(Rank::Ace, Suit::Spades).id();
    state.round.active_seat_index = 0;
    state.round.seats[0].hand = vec![
        base[0],
        base[1],
        base[2],
        Card::new(Rank::King, Suit::Clubs).id(),
    ];
    state.round.seats[1].hand = vec![layoff, Card::new(Rank::Two, Suit::Hearts).id()];
    table_new_meld(&mut state.round, 0, &base, TurnOrdinal(1)).expect("base set tables");
    state.round.active_seat_index = 1;
    lay_off_card(
        &mut state.round,
        1,
        layoff,
        MeldId(0),
        LayoffPosition::Append,
        TurnOrdinal(2),
    )
    .expect("layoff accepted");

    let settlement = settle_round(&mut state);

    assert_eq!(card_score(base[0]), 15);
    assert_eq!(settlement.seats[0].tabled_positive, 45);
    assert_eq!(settlement.seats[0].in_hand_penalty, 10);
    assert_eq!(settlement.seats[0].round_delta, 35);
    assert_eq!(settlement.seats[1].tabled_positive, 15);
    assert_eq!(settlement.seats[1].in_hand_penalty, 2);
    assert_eq!(settlement.seats[1].round_delta, 13);
    assert!(!settlement.stable_public_string().contains("king_clubs"));
    assert!(!settlement.stable_public_string().contains("two_hearts"));
}

#[test]
fn scores_can_go_negative_and_cumulative_scores_update() {
    let mut state = sample_state();
    state.round.seats[0].hand = vec![
        Card::new(Rank::Ace, Suit::Spades).id(),
        Card::new(Rank::King, Suit::Spades).id(),
    ];
    let zeroes = vec![0, 0, 0, 0];
    let penalties = vec![25, 0, 0, 0];
    let deltas = vec![-25, 0, 0, 0];

    let settlement = apply_round_deltas(&mut state, &deltas, &zeroes, &penalties);

    assert_eq!(settlement.seats[0].round_delta, -25);
    assert_eq!(settlement.seats[0].cumulative_score, -25);
    assert_eq!(state.cumulative_scores[0], -25);
    assert_eq!(state.terminal, None);
}

#[test]
fn unique_highest_at_or_above_500_wins_with_stable_standings() {
    let mut state = sample_state();
    state.cumulative_scores = vec![490, 480, 100, -20];
    let deltas = vec![20, 15, 0, 0];
    let zeroes = vec![0, 0, 0, 0];

    let settlement = apply_round_deltas(&mut state, &deltas, &zeroes, &zeroes);

    let terminal = settlement.terminal.expect("unique eligible winner");
    assert_eq!(terminal.winner, Some(0));
    assert_eq!(state.cumulative_scores, vec![510, 495, 100, -20]);
    assert_eq!(terminal.standings[0].rank, 1);
    assert!(terminal.standings[0].winner);
}

#[test]
fn tied_highest_at_or_above_500_continues_until_unique() {
    let current = vec![490, 495, 100, 0];
    let tied = vec![20, 15, 0, 0];
    assert_eq!(
        terminal_outcome_for_scores_after_deltas(&current, &tied),
        None
    );

    let unique = vec![25, 15, 0, 0];
    let outcome = terminal_outcome_for_scores_after_deltas(&current, &unique)
        .expect("unique winner after later round");
    assert_eq!(outcome.winner, Some(0));
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

fn assert_round_conserves_deck(state: &MatchState) {
    let mut cards = Vec::new();
    cards.extend(state.round.stock.iter().copied());
    cards.extend(state.round.discard.iter().copied());
    for seat in &state.round.seats {
        cards.extend(seat.hand.iter().copied());
    }
    for group in &state.round.tableau.groups {
        cards.extend(group.cards.iter().map(|card| card.card));
    }

    let unique = cards.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(cards.len(), STANDARD_CARD_COUNT as usize);
    assert_eq!(unique.len(), STANDARD_CARD_COUNT as usize);
}
