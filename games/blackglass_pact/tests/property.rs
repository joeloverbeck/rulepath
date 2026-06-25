use std::collections::BTreeSet;

use blackglass_pact::{
    apply_blind_nil_choice, canonical_deck, canonical_seat_ids, setup_match_with_scores,
    BlackglassSeat, BlindNilChoice, Card, CardId, Rank, SetupOptions, Suit, STANDARD_CARD_COUNT,
    STANDARD_HAND_SIZE,
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
    let state = setup_match_with_scores(
        Seed(1803),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("setup succeeds");

    for seat in BlackglassSeat::ALL {
        assert!(state.hand_for_internal(seat).is_empty());
    }
    assert_eq!(state.stable_setup_summary(), "game=blackglass_pact;variant=blackglass_pact_standard;seats=seat_0,seat_1,seat_2,seat_3;dealer=seat_0;hand_index=0;teams=team_0:seat_0+seat_2|team_1:seat_1+seat_3");
}

#[test]
fn deterministic_deal_conserves_all_cards_as_four_thirteen_card_hands() {
    let mut state = setup_match_with_scores(
        Seed(1805),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("setup succeeds");

    apply_blind_nil_choice(&mut state, BlackglassSeat::South, BlindNilChoice::Declined)
        .expect("first blind decision succeeds");
    apply_blind_nil_choice(&mut state, BlackglassSeat::North, BlindNilChoice::Declared)
        .expect("second blind decision succeeds");

    let mut dealt = BTreeSet::new();
    for seat in BlackglassSeat::ALL {
        let hand = state.hand_for_internal(seat);
        assert_eq!(hand.len(), STANDARD_HAND_SIZE as usize);
        dealt.extend(hand.iter().copied());
    }

    assert_eq!(dealt.len(), STANDARD_CARD_COUNT as usize);
    assert_eq!(dealt, canonical_deck().into_iter().collect::<BTreeSet<_>>());
}

#[test]
fn blind_nil_choices_do_not_perturb_deal_bytes_for_same_seed() {
    let mut declared = setup_match_with_scores(
        Seed(1806),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("declared setup succeeds");
    let mut declined = setup_match_with_scores(
        Seed(1806),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("declined setup succeeds");

    apply_blind_nil_choice(
        &mut declared,
        BlackglassSeat::South,
        BlindNilChoice::Declared,
    )
    .expect("south declare succeeds");
    apply_blind_nil_choice(
        &mut declared,
        BlackglassSeat::North,
        BlindNilChoice::Declared,
    )
    .expect("north declare succeeds");
    apply_blind_nil_choice(
        &mut declined,
        BlackglassSeat::South,
        BlindNilChoice::Declined,
    )
    .expect("south decline succeeds");
    apply_blind_nil_choice(
        &mut declined,
        BlackglassSeat::North,
        BlindNilChoice::Declined,
    )
    .expect("north decline succeeds");

    let declared_hands = BlackglassSeat::ALL
        .into_iter()
        .map(|seat| state_hand_bytes(&declared, seat))
        .collect::<Vec<_>>();
    let declined_hands = BlackglassSeat::ALL
        .into_iter()
        .map(|seat| state_hand_bytes(&declined, seat))
        .collect::<Vec<_>>();

    assert_eq!(declared_hands, declined_hands);
}

fn state_hand_bytes(state: &blackglass_pact::BlackglassPactState, seat: BlackglassSeat) -> Vec<u8> {
    state
        .hand_for_internal(seat)
        .iter()
        .map(|card| card.index())
        .collect()
}
