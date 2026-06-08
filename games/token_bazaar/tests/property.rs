use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use token_bazaar::{
    apply_action, legal_actions, setup_match, validate_command, ResourceCounts,
    TokenBazaarLevel1Bot, TokenBazaarSeat, TokenBazaarState,
};

fn setup() -> TokenBazaarState {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    setup_match(Seed(5), &seats, &Default::default()).expect("setup succeeds")
}

fn command_for(state: &TokenBazaarState, segment: String) -> CommandEnvelope {
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
fn deterministic_playout_conserves_resources_and_never_panics() {
    let mut state = setup();

    for _ in 0..24 {
        assert_conserved(&state);
        if state.terminal_outcome.is_some() {
            break;
        }
        let action = legal_actions(&state)
            .into_iter()
            .next()
            .expect("non-terminal state has legal action");
        let command = command_for(&state, action.segment());
        let validated = validate_command(&state, &command).expect("legal action validates");
        apply_action(&mut state, validated);
        assert_conserved(&state);
    }
}

#[test]
fn level1_bot_actions_validate_during_playout() {
    let mut state = setup();
    let bot = TokenBazaarLevel1Bot::new(Seed(9));

    for _ in 0..8 {
        if state.terminal_outcome.is_some() {
            break;
        }
        let decision = bot
            .select_decision(&state, state.active_seat)
            .expect("bot chooses");
        let command = CommandEnvelope {
            actor: Actor {
                seat_id: state.seats[state.active_seat.index()].clone(),
            },
            action_path: decision.action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        };
        let validated = validate_command(&state, &command).expect("bot action validates");
        apply_action(&mut state, validated);
        assert_conserved(&state);
    }
}

fn assert_conserved(state: &TokenBazaarState) {
    let total = state.supply.total() + state.inventories[0].total() + state.inventories[1].total();
    assert_eq!(total, 48);
    assert_non_negative(state.supply);
    assert_non_negative(state.inventories[TokenBazaarSeat::Seat0.index()]);
    assert_non_negative(state.inventories[TokenBazaarSeat::Seat1.index()]);
}

fn assert_non_negative(_counts: ResourceCounts) {
    // Resource counts are u8; this assertion documents the no-negative invariant.
}
