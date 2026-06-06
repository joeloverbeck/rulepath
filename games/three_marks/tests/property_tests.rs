use std::collections::BTreeSet;

use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use three_marks::{
    apply_action, legal_action_tree, setup_match, validate_command, CellOccupancy, SetupOptions,
    TerminalOutcome, ThreeMarksSeat,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor(index: usize) -> Actor {
    Actor {
        seat_id: seats()[index].clone(),
    }
}

fn command(index: usize, segment: &str, freshness_token: FreshnessToken) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(index),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn action_ids_are_stable_unique_and_never_target_occupied_cells() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let initial = legal_action_tree(&state, &actor(0));
    let repeated = legal_action_tree(&state, &actor(0));
    assert_eq!(initial, repeated);

    let mut seen = BTreeSet::new();
    for choice in &initial.root.choices {
        assert!(seen.insert(choice.segment.clone()));
    }

    let action = validate_command(&state, &command(0, "place/r1c1", state.freshness_token))
        .expect("placement validates");
    apply_action(&mut state, action);

    let segments: Vec<_> = legal_action_tree(&state, &actor(1))
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.clone())
        .collect();
    assert!(!segments.contains(&"place/r1c1".to_owned()));
}

#[test]
fn deterministic_legal_sequence_preserves_mark_counts_and_bounds() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let mut seat_index = 0;

    while state.terminal_outcome.is_none() {
        let tree = legal_action_tree(&state, &actor(seat_index));
        assert!(!tree.root.choices.is_empty());
        let segment = tree.root.choices[0].segment.clone();
        let action = validate_command(&state, &command(seat_index, &segment, tree.freshness_token))
            .expect("generated action validates");
        apply_action(&mut state, action);

        let seat_0_marks = state
            .cells
            .iter()
            .filter(|cell| **cell == CellOccupancy::Occupied(ThreeMarksSeat::Seat0))
            .count();
        let seat_1_marks = state
            .cells
            .iter()
            .filter(|cell| **cell == CellOccupancy::Occupied(ThreeMarksSeat::Seat1))
            .count();
        assert!(seat_0_marks == seat_1_marks || seat_0_marks == seat_1_marks + 1);
        assert!(state.ply_count <= 9);

        seat_index = 1 - seat_index;
    }

    assert!(matches!(
        state.terminal_outcome,
        Some(TerminalOutcome::Win { .. }) | Some(TerminalOutcome::Draw)
    ));
    assert!(legal_action_tree(&state, &actor(0)).root.choices.is_empty());
    assert!(legal_action_tree(&state, &actor(1)).root.choices.is_empty());
}
