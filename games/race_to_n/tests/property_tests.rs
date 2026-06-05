use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use race_to_n::{
    apply_action, legal_action_tree, setup_match, validate_command, CounterValue, RaceSeat,
    SetupOptions,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor_for(state_seat: RaceSeat) -> Actor {
    Actor {
        seat_id: seats()[state_seat.index()].clone(),
    }
}

fn command_for(state_seat: RaceSeat, segment: String, token: FreshnessToken) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for(state_seat),
        action_path: ActionPath {
            segments: vec![segment],
        },
        freshness_token: token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn legal_action_generation_never_panics_across_reachable_counter_values() {
    for counter in 0..=21 {
        for active_seat in [RaceSeat::Seat0, RaceSeat::Seat1] {
            let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
            state.counter = CounterValue(counter);
            state.active_seat = active_seat;
            if counter == 21 {
                state.winner = Some(active_seat);
            }

            let tree = legal_action_tree(&state, &actor_for(active_seat));
            for choice in tree.root.choices {
                assert!(choice.segment.starts_with("add-"));
            }
        }
    }
}

#[test]
fn applying_each_generated_legal_action_preserves_invariants() {
    for counter in 0..21 {
        for active_seat in [RaceSeat::Seat0, RaceSeat::Seat1] {
            let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
            state.counter = CounterValue(counter);
            state.active_seat = active_seat;

            let tree = legal_action_tree(&state, &actor_for(active_seat));
            assert!(!tree.root.choices.is_empty());

            for choice in tree.root.choices {
                let mut next_state = state.clone();
                let command =
                    command_for(active_seat, choice.segment.clone(), state.freshness_token);
                let action =
                    validate_command(&next_state, &command).expect("generated action validates");
                let previous_token = next_state.freshness_token;
                let effects = apply_action(&mut next_state, action);

                assert!(next_state.counter.0 <= next_state.variant.target);
                assert_eq!(next_state.freshness_token, previous_token.next());
                assert!(!effects.is_empty());
                if next_state.counter.0 == next_state.variant.target {
                    assert_eq!(next_state.winner, Some(active_seat));
                } else {
                    assert_eq!(next_state.active_seat, active_seat.other());
                    assert_eq!(next_state.winner, None);
                }
            }
        }
    }
}
