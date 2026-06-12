use engine_core::{Actor, SeatId};
use frontier_control::{
    command_for_decision, legal_action_tree, setup_match, validate_bot_decision, FactionId,
    FrontierGarrisonLevel1Bot, FrontierProspectorLevel1Bot, Phase, SetupOptions, ACTION_END_TURN,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

#[test]
fn every_legal_move_follows_an_edge_and_tree_never_stalls() {
    let state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let tree = legal_action_tree(
        &state,
        &Actor {
            seat_id: SeatId("seat_1".to_owned()),
        },
    );
    assert!(tree
        .root
        .choices
        .iter()
        .any(|choice| choice.segment == ACTION_END_TURN));
    for choice in tree.root.choices {
        let parts = choice.segment.split('/').collect::<Vec<_>>();
        if matches!(parts.first(), Some(&"march") | Some(&"patrol")) {
            let from = frontier_control::SiteId::parse(parts[1]).unwrap();
            let to = frontier_control::SiteId::parse(parts[2]).unwrap();
            assert!(state.sites_are_adjacent(from, to));
        }
    }
}

#[test]
fn level1_bot_sequence_reaches_terminal_without_illegal_actions() {
    let mut state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    for _ in 0..100 {
        if state.terminal_outcome.is_some() {
            assert_eq!(state.phase, Phase::Terminal);
            return;
        }
        let active = state.active_faction;
        let seat = state.active_seat().unwrap().clone();
        let decision = match active {
            FactionId::Garrison => FrontierGarrisonLevel1Bot::new(engine_core::Seed(5))
                .select_decision(&state, &seat)
                .unwrap(),
            FactionId::Prospectors => FrontierProspectorLevel1Bot::new(engine_core::Seed(5))
                .select_decision(&state, &seat)
                .unwrap(),
        };
        validate_bot_decision(&state, &seat, &decision).unwrap();
        let command = command_for_decision(&state, &seat, &decision);
        frontier_control::apply_command(&mut state, &command).unwrap();
    }
    panic!("bot property smoke did not reach terminal");
}
