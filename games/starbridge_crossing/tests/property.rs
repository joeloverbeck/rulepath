use engine_core::{ActionNode, ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use starbridge_crossing::{
    apply_jump_command, apply_step_command, legal_action_tree, parse_action_path, setup_match,
    spaces_by_stable_order, SetupOptions, StarbridgeAction, StarbridgeState, SPACE_COUNT,
};

#[test]
fn topology_order_is_deterministic() {
    let first: Vec<_> = spaces_by_stable_order()
        .map(|space| {
            (
                space.id.index(),
                space.coord.q,
                space.coord.r,
                space.coord.s,
            )
        })
        .collect();
    let second: Vec<_> = spaces_by_stable_order()
        .map(|space| {
            (
                space.id.index(),
                space.coord.q,
                space.coord.r,
                space.coord.s,
            )
        })
        .collect();

    assert_eq!(first.len(), usize::from(SPACE_COUNT));
    assert_eq!(first, second);
}

#[test]
fn setup_has_at_most_one_occupant_per_space_for_supported_counts() {
    for count in [2, 3, 4, 6] {
        let seats = (0..count)
            .map(|index| engine_core::SeatId::from_zero_based_index(index as u32))
            .collect::<Vec<_>>();
        let state = starbridge_crossing::setup_match(
            engine_core::Seed(31),
            &seats,
            &starbridge_crossing::SetupOptions::default(),
        )
        .unwrap();
        let occupied = state
            .occupancy
            .iter()
            .filter(|occupant| occupant.is_some())
            .count();

        assert_eq!(occupied, state.pegs.len());
    }
}

#[test]
fn legal_step_destinations_are_empty_in_setup_position() {
    let seats = vec![
        engine_core::SeatId::from_zero_based_index(0),
        engine_core::SeatId::from_zero_based_index(1),
    ];
    let state = starbridge_crossing::setup_match(
        engine_core::Seed(31),
        &seats,
        &starbridge_crossing::SetupOptions::default(),
    )
    .unwrap();

    for step in starbridge_crossing::legal_step_moves(&state) {
        assert_eq!(state.occupancy(step.to), None);
    }
}

#[test]
fn committed_non_pass_turns_change_board_occupancy() {
    for count in [2, 3, 4, 6] {
        let seats = (0..count)
            .map(|index| SeatId::from_zero_based_index(index as u32))
            .collect::<Vec<_>>();
        let state = setup_match(Seed(31), &seats, &SetupOptions::default()).unwrap();
        assert_non_pass_actions_change_occupancy(&seats, &state);
    }
}

fn assert_non_pass_actions_change_occupancy(seats: &[SeatId], state: &StarbridgeState) {
    let actor = Actor {
        seat_id: seats[usize::from(state.active_seat_index)].clone(),
    };
    let tree = legal_action_tree(state, &actor);
    for path in action_paths(&tree.root) {
        let command = CommandEnvelope {
            actor: actor.clone(),
            action_path: ActionPath {
                segments: path.clone(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        };
        let before = state.occupancy;
        let mut candidate = state.clone();
        match parse_action_path(&path).expect("generated path parses") {
            StarbridgeAction::Step { .. } => {
                apply_step_command(&mut candidate, &command).expect("generated step applies");
                assert_ne!(candidate.occupancy, before, "step path {path:?} is a no-op");
            }
            StarbridgeAction::Jump { .. } => {
                apply_jump_command(&mut candidate, &command).expect("generated jump applies");
                assert_ne!(candidate.occupancy, before, "jump path {path:?} is a no-op");
            }
            StarbridgeAction::PassBlocked => {}
        }
    }
}

fn action_paths(root: &ActionNode) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    collect_action_paths(root, &mut Vec::new(), &mut paths);
    paths
}

fn collect_action_paths(node: &ActionNode, prefix: &mut Vec<String>, paths: &mut Vec<Vec<String>>) {
    for choice in &node.choices {
        prefix.push(choice.segment.clone());
        if let Some(next) = &choice.next {
            collect_action_paths(next, prefix, paths);
        } else {
            paths.push(prefix.clone());
        }
        prefix.pop();
    }
}
