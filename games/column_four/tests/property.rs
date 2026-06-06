use column_four::{
    apply_action, setup_match, validate_command, ColumnFourRandomBot, SetupOptions, TerminalOutcome,
};
use engine_core::{Actor, CommandEnvelope, RulesVersion, SeatId, Seed};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

#[test]
fn random_legal_play_always_terminates_with_bounded_ply_count() {
    for seed in 0..64 {
        let mut state = setup_match(Seed(seed), &seats(), &SetupOptions::default()).unwrap();
        while state.terminal_outcome.is_none() {
            assert!(state.ply_count < 42);
            let bot_seat = state.active_seat;
            let action_path = ColumnFourRandomBot::new(Seed(seed + u64::from(state.ply_count)))
                .select_action(&state, bot_seat)
                .expect("random legal action exists before terminal");
            let command = CommandEnvelope {
                actor: Actor {
                    seat_id: state.seats[bot_seat.index()].clone(),
                },
                action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(1),
            };
            let action = validate_command(&state, &command).expect("bot action validates");
            apply_action(&mut state, action);
        }

        assert!(state.ply_count <= 42);
        assert!(matches!(
            state.terminal_outcome,
            Some(TerminalOutcome::Win { .. }) | Some(TerminalOutcome::Draw)
        ));
    }
}
