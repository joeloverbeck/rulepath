use directional_flip::{
    apply_action, legal_action_tree, setup_match, validate_command, DirectionalFlipSeat,
    SetupOptions,
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

fn legal_segments(state: &directional_flip::DirectionalFlipState) -> Vec<String> {
    legal_action_tree(
        state,
        &Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
    )
    .root
    .choices
    .iter()
    .map(|choice| choice.segment.clone())
    .collect()
}

#[test]
fn df_preview_001_action_tree_preview_flip_set_equals_apply_for_all_opening_targets() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let actor = Actor {
        seat_id: state.seats[DirectionalFlipSeat::Seat0.index()].clone(),
    };
    let tree = legal_action_tree(&state, &actor);

    for choice in tree.root.choices {
        let preview_flips = choice
            .metadata
            .iter()
            .find(|entry| entry.key == "ordered_flip_cells")
            .map(|entry| entry.value.clone())
            .unwrap_or_default();
        let mut candidate = state.clone();
        let action = validate_command(&candidate, &command(&candidate, choice.segment)).unwrap();
        let effects = apply_action(&mut candidate, action);
        let effect_text = format!("{effects:?}");

        for cell_id in preview_flips.split(',').filter(|value| !value.is_empty()) {
            assert!(effect_text.contains(cell_id), "missing flip {cell_id}");
        }
    }
}

#[test]
fn df_replay_property_random_legal_play_terminates_with_bounded_ply_count() {
    for seed in 0..16 {
        let mut state = setup_match(Seed(seed), &seats(), &SetupOptions::default()).unwrap();
        let mut steps = 0_u16;
        while state.terminal_outcome.is_none() {
            let segments = legal_segments(&state);
            assert!(!segments.is_empty(), "nonterminal state has legal action");
            let segment = segments[(seed as usize + steps as usize) % segments.len()].clone();
            let action = validate_command(&state, &command(&state, segment)).unwrap();
            apply_action(&mut state, action);
            steps += 1;
            assert!(steps <= 128, "playout exceeded bounded ply count");
        }
    }
}
