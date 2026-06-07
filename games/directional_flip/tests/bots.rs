use directional_flip::{
    apply_action, legal_action_tree, setup_match, validate_command, DirectionalFlipLevel2Bot,
    DirectionalFlipRandomBot, DirectionalFlipSeat, SetupOptions,
};
use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn command(state: &directional_flip::DirectionalFlipState, segment: String) -> CommandEnvelope {
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
fn df_bot_001_002_level0_and_level2_choices_validate_across_seeds() {
    for seed in 0..32 {
        let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
        for segment in ["place/r3c4", "place/r3c5", "place/r3c6"] {
            let action = validate_command(&state, &command(&state, segment.to_owned())).unwrap();
            apply_action(&mut state, action);
        }
        let bot_seat = state.active_seat;
        for action_path in [
            DirectionalFlipRandomBot::new(Seed(seed))
                .select_action(&state, bot_seat)
                .unwrap(),
            DirectionalFlipLevel2Bot::new(Seed(seed))
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
fn df_bot_002_level2_deterministic_safe_explanation_and_effect() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let bot = DirectionalFlipLevel2Bot::new(Seed(7));

    let left = bot
        .select_decision(&state, DirectionalFlipSeat::Seat0)
        .unwrap();
    let right = bot
        .select_decision(&state, DirectionalFlipSeat::Seat0)
        .unwrap();

    assert_eq!(left, right);
    assert!(!left.rationale.is_empty());
    assert!(!left.rationale.contains("candidate"));
    assert!(!left.rationale.contains("score"));
    assert!(!left.rationale.contains("debug"));
    assert!(!left.rationale.contains("hash"));
    assert!(!left.rationale.contains('['));
    assert!(format!("{:?}", left.effects).contains("BotChoseAction"));
}
