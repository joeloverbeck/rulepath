use briar_circuit::{
    setup::{deal_order_after, next_dealer},
    setup_match, BriarCircuitSeat, PassDirection, Phase, SetupOptions,
};
use engine_core::{SeatId, Seed};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

#[test]
fn setup_accepts_exactly_four_seats() {
    let state = setup_match(Seed(16), &seats(4), &SetupOptions::default()).expect("setup succeeds");

    assert_eq!(state.seats[0], SeatId("seat_0".to_owned()));
    assert_eq!(state.dealer, BriarCircuitSeat::Seat0);
    assert_eq!(state.hand_index, 0);
    assert_eq!(state.cumulative_scores, [0, 0, 0, 0]);
    assert!(matches!(
        state.phase,
        Phase::Passing(ref pass) if pass.direction == PassDirection::Left
    ));
}

#[test]
fn deal_starts_left_of_dealer_and_dealer_rotates_clockwise() {
    assert_eq!(
        deal_order_after(BriarCircuitSeat::Seat0),
        [
            BriarCircuitSeat::Seat1,
            BriarCircuitSeat::Seat2,
            BriarCircuitSeat::Seat3,
            BriarCircuitSeat::Seat0,
        ]
    );
    assert_eq!(
        next_dealer(BriarCircuitSeat::Seat0),
        BriarCircuitSeat::Seat1
    );
    assert_eq!(
        next_dealer(BriarCircuitSeat::Seat3),
        BriarCircuitSeat::Seat0
    );
}

#[test]
fn setup_rejects_every_non_four_count_with_stable_diagnostic() {
    for count in [0, 1, 2, 3, 5, 6, 7] {
        let err = setup_match(Seed(16), &seats(count), &SetupOptions::default())
            .expect_err("unsupported seat count rejects");

        assert_eq!(err.code, "BC_UNSUPPORTED_SEAT_COUNT");
        assert!(
            err.message.contains("requires exactly four seats"),
            "unexpected diagnostic for count {count}: {}",
            err.message
        );
    }
}

#[test]
fn pass_direction_cycle_and_targets_are_stable() {
    assert_eq!(PassDirection::for_hand_index(0), PassDirection::Left);
    assert_eq!(PassDirection::for_hand_index(1), PassDirection::Right);
    assert_eq!(PassDirection::for_hand_index(2), PassDirection::Across);
    assert_eq!(PassDirection::for_hand_index(3), PassDirection::Hold);
    assert_eq!(PassDirection::for_hand_index(4), PassDirection::Left);
    assert_eq!(
        PassDirection::Left.target_for(BriarCircuitSeat::Seat3),
        BriarCircuitSeat::Seat0
    );
    assert_eq!(
        PassDirection::Right.target_for(BriarCircuitSeat::Seat0),
        BriarCircuitSeat::Seat3
    );
    assert_eq!(
        PassDirection::Across.target_for(BriarCircuitSeat::Seat1),
        BriarCircuitSeat::Seat3
    );
}
