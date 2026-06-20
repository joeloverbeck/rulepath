use briar_circuit::{
    canonical_seat_ids,
    setup::{deal_hand, next_dealer},
    setup_match, BriarCircuitSeat, PassDirection, SetupOptions,
};
use engine_core::{Seed, SeededRng};

#[test]
fn identical_seed_reproduces_identical_initial_deal() {
    let seats = canonical_seat_ids();
    let first = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("first setup");
    let second = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("second setup");

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
    for seat in BriarCircuitSeat::ALL {
        assert_eq!(
            first.hand_for_internal(seat),
            second.hand_for_internal(seat)
        );
    }
}

#[test]
fn different_seed_changes_the_deal_but_not_public_rotation_facts() {
    let seats = canonical_seat_ids();
    let first = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("first setup");
    let second = setup_match(Seed(1601), &seats, &SetupOptions::default()).expect("second setup");

    assert_ne!(
        first.hand_for_internal(BriarCircuitSeat::Seat0),
        second.hand_for_internal(BriarCircuitSeat::Seat0)
    );
    assert_eq!(first.dealer, second.dealer);
    assert_eq!(first.pass_direction(), second.pass_direction());
}

#[test]
fn sequential_hand_deals_are_replayable_from_seed_and_hand_index() {
    let mut first_rng = SeededRng::from_seed(Seed(1600));
    let first_hand = deal_hand(&mut first_rng, BriarCircuitSeat::Seat0, 0).expect("hand 0");
    let second_hand =
        deal_hand(&mut first_rng, next_dealer(BriarCircuitSeat::Seat0), 1).expect("hand 1");
    let hold_hand =
        deal_hand(&mut first_rng, BriarCircuitSeat::Seat3, 3).expect("hold hand fixture");

    let mut replay_rng = SeededRng::from_seed(Seed(1600));
    let replay_first = deal_hand(&mut replay_rng, BriarCircuitSeat::Seat0, 0).expect("hand 0");
    let replay_second =
        deal_hand(&mut replay_rng, next_dealer(BriarCircuitSeat::Seat0), 1).expect("hand 1");
    let replay_hold = deal_hand(&mut replay_rng, BriarCircuitSeat::Seat3, 3).expect("hand 3");

    assert_eq!(first_hand, replay_first);
    assert_eq!(second_hand, replay_second);
    assert_eq!(hold_hand, replay_hold);
    assert_eq!(first_hand.pass_direction, PassDirection::Left);
    assert_eq!(second_hand.pass_direction, PassDirection::Right);
    assert_eq!(hold_hand.pass_direction, PassDirection::Hold);
    assert_ne!(first_hand.hands, second_hand.hands);
}
