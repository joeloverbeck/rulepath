use std::collections::BTreeSet;

use engine_core::{SeatId, Seed};
use high_card_duel::{
    canonical_deck, next_bounded_index_unbiased, setup_match, HighCardDuelSeat, SetupOptions,
    STANDARD_DECK_CARD_COUNT, STANDARD_HAND_SIZE,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

#[test]
fn setup_same_seed_same_initial_deal_internal() {
    let options = SetupOptions::default();
    let left = setup_match(Seed(42), &seats(), &options).expect("setup succeeds");
    let right = setup_match(Seed(42), &seats(), &options).expect("setup succeeds");

    assert_eq!(left, right);
    assert_eq!(left.internal_card_order(), right.internal_card_order());
}

#[test]
fn setup_different_seeds_can_change_initial_deal() {
    let options = SetupOptions::default();
    let left = setup_match(Seed(42), &seats(), &options).expect("setup succeeds");
    let right = setup_match(Seed(43), &seats(), &options).expect("setup succeeds");

    assert_ne!(left.internal_card_order(), right.internal_card_order());
}

#[test]
fn setup_deals_private_hands_and_hides_deck() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");

    assert_eq!(
        state.hand_for(HighCardDuelSeat::Seat0).len(),
        STANDARD_HAND_SIZE as usize
    );
    assert_eq!(
        state.hand_for(HighCardDuelSeat::Seat1).len(),
        STANDARD_HAND_SIZE as usize
    );
    assert_eq!(
        state.deck.len(),
        STANDARD_DECK_CARD_COUNT as usize - (STANDARD_HAND_SIZE as usize * 2)
    );
    assert!(state.commitment_for(HighCardDuelSeat::Seat0).is_none());
    assert!(state.commitment_for(HighCardDuelSeat::Seat1).is_none());

    let all_cards = state.internal_card_order();
    let unique_cards = all_cards.iter().collect::<BTreeSet<_>>();
    assert_eq!(all_cards.len(), STANDARD_DECK_CARD_COUNT as usize);
    assert_eq!(unique_cards.len(), STANDARD_DECK_CARD_COUNT as usize);

    let canonical = canonical_deck()
        .into_iter()
        .map(|card| card.stable_id())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        unique_cards.into_iter().cloned().collect::<BTreeSet<_>>(),
        canonical
    );
}

#[test]
fn setup_shuffle_uses_unbiased_bounded_index_or_documented_helper() {
    struct FixedRng {
        values: Vec<u64>,
    }

    impl engine_core::DeterministicRng for FixedRng {
        fn next_u64(&mut self) -> u64 {
            self.values.remove(0)
        }
    }

    let range = u128::from(u64::MAX) + 1;
    let accepted_zone_for_three = range - (range % 3);
    let rejected = accepted_zone_for_three as u64;
    let mut rng = FixedRng {
        values: vec![rejected, 4],
    };

    assert_eq!(next_bounded_index_unbiased(&mut rng, 3), Some(1));
}
