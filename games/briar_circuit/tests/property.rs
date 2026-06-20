use std::collections::BTreeSet;

use briar_circuit::{
    canonical_deck, canonical_seat_ids, setup_match, Card, CardId, Rank, SetupOptions, Suit,
    STANDARD_CARD_COUNT, STANDARD_HAND_SIZE,
};
use engine_core::Seed;

#[test]
fn canonical_deck_contains_fifty_two_unique_cards_in_stable_order() {
    let deck = canonical_deck();

    assert_eq!(deck.len(), STANDARD_CARD_COUNT as usize);
    assert_eq!(deck.iter().copied().collect::<BTreeSet<_>>().len(), 52);
    assert_eq!(deck[0], Card::new(Rank::Two, Suit::Clubs).id());
    assert_eq!(deck[12], Card::new(Rank::Ace, Suit::Clubs).id());
    assert_eq!(deck[13], Card::new(Rank::Two, Suit::Diamonds).id());
    assert_eq!(deck[26], Card::new(Rank::Two, Suit::Hearts).id());
    assert_eq!(deck[39], Card::new(Rank::Two, Suit::Spades).id());
    assert_eq!(deck[51], Card::new(Rank::Ace, Suit::Spades).id());
}

#[test]
fn card_ids_round_trip_to_cards_and_strings() {
    for card_id in canonical_deck() {
        let card = card_id.card();

        assert_eq!(card.id(), card_id);
        assert_eq!(CardId::parse(&card.id_str()), Some(card_id));
    }
}

#[test]
fn point_cards_match_briar_circuit_penalty_values() {
    assert_eq!(Card::new(Rank::Two, Suit::Hearts).point_value(), 1);
    assert_eq!(Card::new(Rank::Ace, Suit::Hearts).point_value(), 1);
    assert_eq!(Card::new(Rank::Queen, Suit::Spades).point_value(), 13);
    assert_eq!(Card::new(Rank::Queen, Suit::Clubs).point_value(), 0);
}

#[test]
fn setup_deals_full_deck_into_four_private_hands() {
    let state = setup_match(Seed(1605), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    let mut all_dealt = Vec::new();

    for seat in briar_circuit::BriarCircuitSeat::ALL {
        let hand = state.hand_for_internal(seat);
        assert_eq!(hand.len(), STANDARD_HAND_SIZE as usize);
        all_dealt.extend_from_slice(hand);
    }

    all_dealt.sort();
    assert_eq!(all_dealt, canonical_deck());
    assert_eq!(all_dealt.len(), STANDARD_CARD_COUNT as usize);
}
