use std::collections::BTreeSet;

use engine_core::Seed;
use vow_tide::{
    cards::{canonical_deck, CardId},
    ids::{
        canonical_seat_ids, hand_schedule_for_seats, VowTideSeat, STANDARD_MAX_SEATS,
        STANDARD_MIN_SEATS,
    },
    setup::{deal_for_hand, seed_for_hand, setup_match, SetupOptions},
};

#[test]
fn same_seed_and_hand_index_reproduce_identical_deal() {
    for seat_count in STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize {
        let hand_size = hand_schedule_for_seats(seat_count).expect("schedule exists")[0];
        let first = deal_for_hand(Seed(42), VowTideSeat::Seat0, 0, seat_count, hand_size)
            .expect("first deal succeeds");
        let second = deal_for_hand(Seed(42), VowTideSeat::Seat0, 0, seat_count, hand_size)
            .expect("second deal succeeds");

        assert_eq!(first, second);
    }
}

#[test]
fn different_seed_changes_deal_for_supported_counts() {
    for seat_count in STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize {
        let hand_size = hand_schedule_for_seats(seat_count).expect("schedule exists")[0];
        let first = deal_for_hand(Seed(42), VowTideSeat::Seat0, 0, seat_count, hand_size)
            .expect("first deal succeeds");
        let second = deal_for_hand(Seed(43), VowTideSeat::Seat0, 0, seat_count, hand_size)
            .expect("second deal succeeds");

        assert_ne!(first.hands, second.hands);
        assert_ne!(first.trump_indicator, second.trump_indicator);
    }
}

#[test]
fn hand_seed_derivation_is_partitioned_by_hand_index() {
    assert_eq!(seed_for_hand(Seed(77), 0), Seed(77));
    assert_ne!(seed_for_hand(Seed(77), 1), Seed(77));
    assert_ne!(seed_for_hand(Seed(77), 1), seed_for_hand(Seed(77), 2));
}

#[test]
fn setup_deals_clockwise_from_left_of_dealer_and_conserves_cards() {
    let options = SetupOptions::default();
    let canonical = canonical_deck().into_iter().collect::<BTreeSet<_>>();

    for seat_count in STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize {
        let seats = canonical_seat_ids(seat_count);
        let state = setup_match(Seed(91), &seats, &options).expect("setup succeeds");
        let hand_size = state.current_hand_size().expect("current hand size");

        assert_eq!(state.deal_order[0], VowTideSeat::Seat1);
        assert_eq!(state.deal_order.last(), Some(&VowTideSeat::Seat0));
        for (_, hand) in &state.private_hands {
            assert_eq!(hand.len(), hand_size as usize);
        }

        let all_cards = all_internal_cards(
            &state.private_hands,
            state.trump_indicator,
            &state.hidden_stock,
        );
        assert_eq!(all_cards.len(), canonical.len());
        assert_eq!(all_cards, canonical);
        assert!(!state.hidden_stock.contains(&state.trump_indicator));
        assert!(state
            .private_hands
            .iter()
            .all(|(_, hand)| !hand.contains(&state.trump_indicator)));
        assert_eq!(state.trump_suit(), state.trump_indicator.card().suit);
    }
}

fn all_internal_cards(
    hands: &[(VowTideSeat, Vec<CardId>)],
    trump_indicator: CardId,
    hidden_stock: &[CardId],
) -> BTreeSet<CardId> {
    let mut cards = BTreeSet::new();
    for (_, hand) in hands {
        for card in hand {
            assert!(
                cards.insert(*card),
                "duplicate dealt card {}",
                card.as_str()
            );
        }
    }
    assert!(
        cards.insert(trump_indicator),
        "duplicate trump {}",
        trump_indicator.as_str()
    );
    for card in hidden_stock {
        assert!(
            cards.insert(*card),
            "duplicate stock card {}",
            card.as_str()
        );
    }
    cards
}
