use std::collections::BTreeSet;

use draughts_lite::{
    apply_action, is_playable_cell, legal_action_tree, setup_match, validate_command,
    CellOccupancy, DraughtsLiteSeat, DraughtsLiteState, PieceKind, SetupOptions,
};
use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn command(state: &DraughtsLiteState, path: Vec<String>) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
        action_path: ActionPath { segments: path },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn legal_paths(state: &DraughtsLiteState) -> Vec<Vec<String>> {
    let tree = legal_action_tree(
        state,
        &Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
    );
    let mut paths = Vec::new();
    collect_paths(&tree.root, Vec::new(), &mut paths);
    paths
}

fn collect_paths(
    node: &engine_core::ActionNode,
    prefix: Vec<String>,
    paths: &mut Vec<Vec<String>>,
) {
    for choice in &node.choices {
        let mut next = prefix.clone();
        next.push(choice.segment.clone());
        if let Some(child) = &choice.next {
            collect_paths(child, next, paths);
        } else {
            paths.push(next);
        }
    }
}

fn assert_board_invariants(state: &DraughtsLiteState) {
    assert_eq!(state.board.rows(), 8);
    assert_eq!(state.board.cols(), 8);
    assert_eq!(state.cells.len(), 64);

    let mut seen_piece_ids = BTreeSet::new();
    let mut occupied_from_pieces = BTreeSet::new();
    for piece in &state.pieces {
        assert!(seen_piece_ids.insert(piece.id), "duplicate piece id");
        assert!(is_playable_cell(piece.cell), "piece on non-playable cell");
        assert!(
            state.board.contains(piece.cell),
            "piece is outside board bounds"
        );
        assert_eq!(
            state.occupancy(piece.cell),
            Some(CellOccupancy::Occupied(piece.id)),
            "piece/cell occupancy mismatch"
        );
        occupied_from_pieces.insert(piece.cell);
        if piece.kind == PieceKind::Man {
            match piece.owner {
                DraughtsLiteSeat::Seat0 => assert!(piece.cell.row() < 8),
                DraughtsLiteSeat::Seat1 => assert!(piece.cell.row() > 1),
            }
        }
    }
    assert_eq!(seen_piece_ids.len(), state.pieces.len());
    assert_eq!(occupied_from_pieces.len(), state.pieces.len());

    for cell in state.board.row_major() {
        if !is_playable_cell(cell) {
            assert_eq!(state.occupancy(cell), Some(CellOccupancy::Empty));
        }
        if let Some(CellOccupancy::Occupied(piece_id)) = state.occupancy(cell) {
            assert!(
                state.piece(piece_id).is_some(),
                "occupied cell references missing piece"
            );
        }
    }
}

#[test]
fn every_legal_tree_leaf_validates_in_opening_and_capture_positions() {
    let mut states = vec![setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap()];

    let mut capture_state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let first = validate_command(
        &capture_state,
        &command(
            &capture_state,
            vec!["from/r3c2".to_owned(), "to/r4c1".to_owned()],
        ),
    )
    .unwrap();
    apply_action(&mut capture_state, first);
    states.push(capture_state);

    for state in states {
        for path in legal_paths(&state) {
            validate_command(&state, &command(&state, path)).expect("legal leaf validates");
        }
    }
}

#[test]
fn bounded_random_legal_play_preserves_board_invariants_without_termination_assumption() {
    for seed in 0..16 {
        let mut state = setup_match(Seed(seed), &seats(), &SetupOptions::default()).unwrap();
        assert_board_invariants(&state);

        for step in 0..96 {
            if state.terminal_outcome.is_some() {
                assert!(legal_paths(&state).is_empty());
                break;
            }

            let paths = legal_paths(&state);
            assert!(!paths.is_empty(), "nonterminal state has a legal path");
            let path = paths[(seed as usize + step) % paths.len()].clone();
            let jump_count = path
                .iter()
                .filter(|segment| segment.starts_with("jump/"))
                .count();
            let pieces_before = state.pieces.len();
            let action = validate_command(&state, &command(&state, path.clone())).unwrap();
            apply_action(&mut state, action);

            assert_board_invariants(&state);
            assert_eq!(
                pieces_before - state.pieces.len(),
                jump_count,
                "piece count changes only by captures"
            );
        }
    }
}

#[test]
fn validated_actions_apply_without_panic_and_advance_command_state() {
    let mut state = setup_match(Seed(7), &seats(), &SetupOptions::default()).unwrap();

    for step in 0..12 {
        if state.terminal_outcome.is_some() {
            break;
        }
        let paths = legal_paths(&state);
        let path = paths[step % paths.len()].clone();
        let prior_ply = state.ply_count;
        let prior_commands = state.command_count;
        let prior_freshness = state.freshness_token;
        let action = validate_command(&state, &command(&state, path)).unwrap();
        apply_action(&mut state, action);

        assert_eq!(state.ply_count, prior_ply + 1);
        assert_eq!(state.command_count, prior_commands + 1);
        assert_eq!(state.freshness_token, prior_freshness.next());
        assert_board_invariants(&state);
    }
}
