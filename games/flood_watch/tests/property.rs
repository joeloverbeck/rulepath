use engine_core::{Actor, SeatId, Seed};
use flood_watch::{legal_action_tree, setup_match, SetupOptions, ACTION_END_TURN};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

#[test]
fn active_action_phase_tree_always_contains_end_turn() {
    for seed in 0..25 {
        let state = setup_match(Seed(seed), &seats(), &SetupOptions::default()).unwrap();
        let tree = legal_action_tree(&state, &actor("seat_0"));

        assert!(tree
            .root
            .choices
            .iter()
            .any(|choice| choice.segment == ACTION_END_TURN));
    }
}
