use std::collections::BTreeSet;

use blackglass_pact::{
    canonical_deck, canonical_seat_ids, setup_match, BlackglassSeat, Card, CardId, Rank,
    SetupOptions, Suit, STANDARD_CARD_COUNT,
};
use engine_core::Seed;

#[test]
fn canonical_deck_contains_fifty_two_unique_cards_in_stable_order() {
    let deck = canonical_deck();

    assert_eq!(deck.len(), STANDARD_CARD_COUNT as usize);
    assert_eq!(
        deck.iter().copied().collect::<BTreeSet<_>>().len(),
        STANDARD_CARD_COUNT as usize
    );
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
fn admitted_setup_has_no_private_deal_before_blind_commitment() {
    let state = setup_match(Seed(1803), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");

    for seat in BlackglassSeat::ALL {
        assert!(state.hand_for_internal(seat).is_empty());
    }
    assert_eq!(state.stable_setup_summary(), "game=blackglass_pact;variant=blackglass_pact_standard;seats=seat_0,seat_1,seat_2,seat_3;dealer=seat_0;hand_index=0;teams=team_0:seat_0+seat_2|team_1:seat_1+seat_3");
}
