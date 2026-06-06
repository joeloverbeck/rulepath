use column_four::{
    apply_action, legal_action_tree, setup_match, validate_command, ColumnFourLevel2Bot,
    ColumnFourRandomBot, ColumnFourSeat, SetupOptions, TerminalOutcome,
};
use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn command(state: &column_four::ColumnFourState, segment: String) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: vec![segment],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn level0_and_level2_choices_are_legal_and_validate_across_seeds() {
    for seed in 0..32 {
        let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
        for segment in ["drop/c4", "drop/c3", "drop/c4"] {
            let action = validate_command(&state, &command(&state, segment.to_owned())).unwrap();
            apply_action(&mut state, action);
        }
        let bot_seat = state.active_seat;
        for action_path in [
            ColumnFourRandomBot::new(Seed(seed))
                .select_action(&state, bot_seat)
                .unwrap(),
            ColumnFourLevel2Bot::new(Seed(seed))
                .select_action(&state, bot_seat)
                .unwrap(),
        ] {
            let legal_paths = legal_action_tree(
                &state,
                &Actor {
                    seat_id: state.seats[bot_seat.index()].clone(),
                },
            )
            .root
            .choices
            .iter()
            .map(|choice| choice.path())
            .collect::<Vec<_>>();
            assert!(legal_paths.contains(&action_path));
            validate_command(&state, &command(&state, action_path.segments[0].clone()))
                .expect("bot action validates");
        }
    }
}

#[test]
fn level2_determinism_and_public_explanation_hold() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let bot = ColumnFourLevel2Bot::new(Seed(7));

    let left = bot.select_decision(&state, ColumnFourSeat::Seat0).unwrap();
    let right = bot.select_decision(&state, ColumnFourSeat::Seat0).unwrap();

    assert_eq!(left, right);
    assert!(left.rationale.contains("central pressure"));
    assert!(!left.rationale.contains("score"));
    assert!(!left.rationale.contains("candidate"));
    assert!(!left.rationale.contains("debug"));
}

#[test]
fn terminal_state_yields_no_bot_action() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.terminal_outcome = Some(TerminalOutcome::Draw);

    assert_eq!(
        ColumnFourRandomBot::new(Seed(1))
            .select_action(&state, ColumnFourSeat::Seat0)
            .expect_err("terminal random bot has no action")
            .code,
        "no_legal_actions"
    );
    assert_eq!(
        ColumnFourLevel2Bot::new(Seed(1))
            .select_action(&state, ColumnFourSeat::Seat0)
            .expect_err("terminal level2 bot has no action")
            .code,
        "no_legal_actions"
    );
}
