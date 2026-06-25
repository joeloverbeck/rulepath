use std::collections::BTreeSet;

use engine_core::Seed;
use meldfall_ledger::{
    cards::{canonical_deck, ranks_are_consecutive_low_or_high, Card, CardId, Rank, Suit},
    rules::{take_new_meld_from_hand, validate_new_meld, validate_owned_meld},
    setup::{deal_for_round, default_seats, setup_match, validate_seat_count, SetupOptions},
    state::{MeldId, MeldKind, TurnOrdinal},
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
