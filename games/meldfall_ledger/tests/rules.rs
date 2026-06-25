use std::collections::BTreeSet;

use engine_core::Seed;
use meldfall_ledger::{
    cards::{canonical_deck, ranks_are_consecutive_low_or_high, Card, CardId, Rank, Suit},
    setup::{deal_for_round, default_seats, setup_match, validate_seat_count, SetupOptions},
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
