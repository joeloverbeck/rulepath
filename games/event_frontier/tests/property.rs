use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use event_frontier::{
    actions::choosing_menu, apply_command, legal_action_tree, setup_match, CardPhase, FactionId,
    SetupOptions, ACTION_PASS,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn actor_for(faction: FactionId) -> Actor {
    let seat = match faction {
        FactionId::Charter => "seat_0",
        FactionId::Freeholders => "seat_1",
    };
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn command_for(faction: FactionId, state: &event_frontier::EventFrontierState) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for(faction),
        action_path: ActionPath {
            segments: vec![ACTION_PASS.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn flow_never_stalls_before_automated_reckoning_or_terminal() {
    let seats = seats();

    for seed in 0..100 {
        let mut state =
            setup_match(Seed(seed), &seats, &SetupOptions::default()).expect("setup succeeds");

        for _ in 0..10 {
            match state.card_phase {
                CardPhase::Reckoning | CardPhase::Terminal => {
                    assert!(choosing_menu(&state).is_none());
                    assert!(legal_action_tree(&state, &actor_for(FactionId::Charter))
                        .root
                        .choices
                        .is_empty());
                    assert!(
                        legal_action_tree(&state, &actor_for(FactionId::Freeholders))
                            .root
                            .choices
                            .is_empty()
                    );
                    break;
                }
                _ => {
                    let (faction, _) = choosing_menu(&state).expect("non-automated choice");
                    let active_tree = legal_action_tree(&state, &actor_for(faction));
                    assert!(
                        !active_tree.root.choices.is_empty(),
                        "seed {seed} stalled for {:?}",
                        state.card_phase
                    );
                    let waiting_tree =
                        legal_action_tree(&state, &actor_for(other_faction(faction)));
                    assert!(waiting_tree.root.choices.is_empty());
                    let command = command_for(faction, &state);
                    apply_command(&mut state, &command).expect("pass command advances flow");
                }
            }
        }
    }
}

fn other_faction(faction: FactionId) -> FactionId {
    match faction {
        FactionId::Charter => FactionId::Freeholders,
        FactionId::Freeholders => FactionId::Charter,
    }
}
