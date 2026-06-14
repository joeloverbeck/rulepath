use std::collections::BTreeSet;

use engine_core::{SeatId, Seed};
use river_ledger::{
    canonical_deck, setup_match, RiverLedgerSeat, SetupOptions, Street, STANDARD_BIG_BLIND,
    STANDARD_CARD_COUNT, STANDARD_SMALL_BLIND,
};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

#[test]
fn setup_accepts_three_to_six_seats_and_rejects_other_counts() {
    let options = SetupOptions::default();

    for count in 3..=6 {
        setup_match(Seed(100 + count as u64), &seats(count), &options)
            .unwrap_or_else(|err| panic!("{count} seats should be accepted: {err:?}"));
    }

    for count in [0, 1, 2, 7] {
        let err = setup_match(Seed(200 + count as u64), &seats(count), &options)
            .expect_err("seat count should reject");
        assert_eq!(err.code, "invalid_seat_count");
        assert_eq!(err.message, "river_ledger requires between 3 and 6 seats");
    }
}

#[test]
fn setup_is_deterministic_for_same_seed_and_options() {
    let options = SetupOptions::default();
    let seats = seats(6);

    let first = setup_match(Seed(42), &seats, &options).expect("first setup");
    let second = setup_match(Seed(42), &seats, &options).expect("second setup");

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
}

#[test]
fn setup_varies_shuffle_for_different_seeds() {
    let options = SetupOptions::default();
    let seats = seats(6);

    let first = setup_match(Seed(42), &seats, &options).expect("first setup");
    let second = setup_match(Seed(43), &seats, &options).expect("second setup");

    assert_ne!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
}

#[test]
fn setup_assigns_button_blinds_active_seat_and_initial_ledger() {
    let options = SetupOptions::default();
    let state = setup_match(Seed(7), &seats(4), &options).expect("setup");

    assert_eq!(state.button, RiverLedgerSeat::from_index(0).unwrap());
    assert_eq!(state.small_blind, RiverLedgerSeat::from_index(1).unwrap());
    assert_eq!(state.big_blind, RiverLedgerSeat::from_index(2).unwrap());
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(3));
    assert_eq!(state.betting.street, Street::Preflop);
    assert_eq!(state.betting.current_to_call, u16::from(STANDARD_BIG_BLIND));
    assert_eq!(
        state.ledger.seats[1].total_contribution,
        u16::from(STANDARD_SMALL_BLIND)
    );
    assert_eq!(
        state.ledger.seats[2].total_contribution,
        u16::from(STANDARD_BIG_BLIND)
    );
    assert_eq!(
        state.ledger.pot_total,
        u16::from(STANDARD_SMALL_BLIND + STANDARD_BIG_BLIND)
    );
}

#[test]
fn setup_deals_unique_hole_cards_reserved_board_and_tail() {
    let options = SetupOptions::default();
    let state = setup_match(Seed(9), &seats(6), &options).expect("setup");

    assert_eq!(state.private_hands_internal().len(), 6);
    assert_eq!(state.community_deck_internal().len(), 5);
    assert_eq!(
        state.deck_tail_internal().len(),
        STANDARD_CARD_COUNT as usize - (6 * 2) - 5
    );

    let mut dealt = BTreeSet::new();
    for hand in state.private_hands_internal() {
        dealt.insert(hand[0]);
        dealt.insert(hand[1]);
    }
    for card in state.community_deck_internal() {
        dealt.insert(*card);
    }
    for card in state.deck_tail_internal() {
        dealt.insert(*card);
    }

    assert_eq!(dealt.len(), canonical_deck().len());
}

#[test]
fn setup_public_summary_exposes_no_hidden_card_identities() {
    let options = SetupOptions::default();
    let state = setup_match(Seed(11), &seats(3), &options).expect("setup");
    let public_summary = state.setup_public_summary();

    assert!(public_summary.contains("hole_counts=seat_0:2 hidden,seat_1:2 hidden,seat_2:2 hidden"));
    assert!(public_summary.contains("reserved_community_count=5"));
    assert!(public_summary.contains("deck_tail_count=41"));

    for card in canonical_deck() {
        assert!(
            !public_summary.contains(&card.id()),
            "public summary leaked {}",
            card.id()
        );
    }
}
