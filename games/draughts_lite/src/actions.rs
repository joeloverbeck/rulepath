use engine_core::{ActionChoice, ActionMetadata, ActionNode, ActionPreview, ActionTree, Actor};
use game_stdlib::board_space::Coord;

use crate::{
    rules::{legal_moves, LegalMove, MoveKind, MoveStep},
    state::{DraughtsLiteState, PieceKind},
    DraughtsLiteSeat,
};

pub const FROM_SEGMENT_PREFIX: &str = "from/";
pub const TO_SEGMENT_PREFIX: &str = "to/";
pub const JUMP_SEGMENT_PREFIX: &str = "jump/";

pub fn legal_action_tree(state: &DraughtsLiteState, actor: &Actor) -> ActionTree {
    if state.terminal_outcome.is_some() || actor_seat(state, actor) != Some(state.active_seat) {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let moves = legal_moves(state);
    let capture_mandatory = moves
        .iter()
        .any(|legal_move| legal_move.kind == MoveKind::Capture);
    let mut root = ActionNode {
        choices: Vec::new(),
    };

    for legal_move in &moves {
        insert_legal_move(&mut root, legal_move, capture_mandatory);
    }

    ActionTree {
        root,
        freshness_token: state.freshness_token,
    }
}

pub fn actor_seat(state: &DraughtsLiteState, actor: &Actor) -> Option<DraughtsLiteSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(DraughtsLiteSeat::from_index)
}

fn insert_legal_move(root: &mut ActionNode, legal_move: &LegalMove, capture_mandatory: bool) {
    let origin_segment = from_segment(legal_move.origin);
    let origin_index = match root
        .choices
        .iter()
        .position(|choice| choice.segment == origin_segment)
    {
        Some(index) => index,
        None => {
            root.choices
                .push(origin_choice(legal_move, capture_mandatory));
            root.choices.len() - 1
        }
    };

    let next = root.choices[origin_index].next.get_or_insert_with(|| {
        Box::new(ActionNode {
            choices: Vec::new(),
        })
    });
    insert_step(next, legal_move, 0, capture_mandatory);
}

fn insert_step(
    node: &mut ActionNode,
    legal_move: &LegalMove,
    step_index: usize,
    capture_mandatory: bool,
) {
    let step = legal_move.steps[step_index];
    let segment = step_segment(step);
    let choice_index = match node
        .choices
        .iter()
        .position(|choice| choice.segment == segment)
    {
        Some(index) => index,
        None => {
            node.choices
                .push(step_choice(legal_move, step, step_index, capture_mandatory));
            node.choices.len() - 1
        }
    };

    if step_index + 1 < legal_move.steps.len() {
        let next = node.choices[choice_index].next.get_or_insert_with(|| {
            Box::new(ActionNode {
                choices: Vec::new(),
            })
        });
        insert_step(next, legal_move, step_index + 1, capture_mandatory);
    }
}

fn origin_choice(legal_move: &LegalMove, capture_mandatory: bool) -> ActionChoice {
    let origin = legal_move.origin.id();
    let mut choice = ActionChoice::leaf(
        from_segment(legal_move.origin),
        format!("Select {origin}"),
        format!(
            "Select {} at {origin}",
            piece_kind_label(legal_move.steps[0].piece_kind_before)
        ),
    );
    choice.metadata = vec![
        metadata("phase", "origin"),
        metadata("cell_id", origin.clone()),
        metadata("piece_id", legal_move.piece_id.stable_id()),
        metadata(
            "piece_kind",
            piece_kind_value(legal_move.steps[0].piece_kind_before),
        ),
        metadata("active_seat", legal_move.actor.as_str()),
        metadata("capture_mandatory", bool_value(capture_mandatory)),
        metadata(
            "is_capture",
            bool_value(legal_move.kind == MoveKind::Capture),
        ),
        metadata("forced_by_continuation", "false"),
        metadata("would_promote", bool_value(legal_move.promotes())),
        metadata("preview_origin", origin),
    ];
    choice.tags = vec![
        "legal-origin".to_owned(),
        "compound".to_owned(),
        piece_kind_value(legal_move.steps[0].piece_kind_before).to_owned(),
    ];
    if capture_mandatory {
        choice.tags.push("capture-mandatory".to_owned());
    }
    choice.preview = ActionPreview::Available;
    choice
}

fn step_choice(
    legal_move: &LegalMove,
    step: MoveStep,
    step_index: usize,
    capture_mandatory: bool,
) -> ActionChoice {
    let is_capture = step.capture.is_some();
    let forced_by_continuation = is_capture && step_index > 0;
    let landing = step.to.id();
    let phase = if is_capture {
        if forced_by_continuation {
            "forced_continuation_landing"
        } else {
            "jump_landing"
        }
    } else {
        "quiet_landing"
    };

    let mut choice = ActionChoice::leaf(
        step_segment(step),
        landing_label(step),
        accessibility_label(step),
    );
    choice.metadata = vec![
        metadata("phase", phase),
        metadata("cell_id", landing.clone()),
        metadata("piece_id", legal_move.piece_id.stable_id()),
        metadata(
            "piece_kind_before",
            piece_kind_value(step.piece_kind_before),
        ),
        metadata("piece_kind_after", piece_kind_value(step.piece_kind_after)),
        metadata("active_seat", legal_move.actor.as_str()),
        metadata("capture_mandatory", bool_value(capture_mandatory)),
        metadata("is_capture", bool_value(is_capture)),
        metadata("forced_by_continuation", bool_value(forced_by_continuation)),
        metadata("would_promote", bool_value(step.promotes)),
        metadata("preview_origin", step.from.id()),
        metadata("preview_landing", landing),
    ];
    if let Some(capture) = step.capture {
        choice
            .metadata
            .push(metadata("captured_cell", capture.cell.id()));
        choice
            .metadata
            .push(metadata("captured_piece_id", capture.piece_id.stable_id()));
        choice
            .metadata
            .push(metadata("captured_owner", capture.owner.as_str()));
        choice
            .metadata
            .push(metadata("preview_captured_cell", capture.cell.id()));
    }
    if step_index + 1 < legal_move.steps.len() {
        choice
            .metadata
            .push(metadata("forced_continuation_available", "true"));
    } else {
        choice
            .metadata
            .push(metadata("forced_continuation_available", "false"));
    }

    choice.tags = vec!["legal-destination".to_owned(), phase.to_owned()];
    if is_capture {
        choice.tags.push("capture".to_owned());
    }
    if forced_by_continuation {
        choice.tags.push("forced".to_owned());
    }
    if step.promotes {
        choice.tags.push("promotion".to_owned());
    }
    choice.preview = ActionPreview::Available;
    choice
}

fn step_segment(step: MoveStep) -> String {
    if step.capture.is_some() {
        jump_segment(step.to)
    } else {
        to_segment(step.to)
    }
}

fn from_segment(cell: Coord) -> String {
    format!("{FROM_SEGMENT_PREFIX}{}", cell.id())
}

fn to_segment(cell: Coord) -> String {
    format!("{TO_SEGMENT_PREFIX}{}", cell.id())
}

fn jump_segment(cell: Coord) -> String {
    format!("{JUMP_SEGMENT_PREFIX}{}", cell.id())
}

fn landing_label(step: MoveStep) -> String {
    if step.capture.is_some() {
        format!("Jump to {}", step.to.id())
    } else {
        format!("Move to {}", step.to.id())
    }
}

fn accessibility_label(step: MoveStep) -> String {
    match step.capture {
        Some(capture) => format!(
            "Jump from {} to {}, capturing {} at {}",
            step.from.id(),
            step.to.id(),
            capture.piece_id.stable_id(),
            capture.cell.id()
        ),
        None => format!("Move from {} to {}", step.from.id(), step.to.id()),
    }
}

fn piece_kind_value(kind: PieceKind) -> &'static str {
    match kind {
        PieceKind::Man => "man",
        PieceKind::Crown => "crown",
    }
}

fn piece_kind_label(kind: PieceKind) -> &'static str {
    match kind {
        PieceKind::Man => "man",
        PieceKind::Crown => "crowned piece",
    }
}

fn bool_value(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn metadata(key: impl Into<String>, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.into(),
        value: value.into(),
    }
}

#[cfg(test)]
mod tests {
    use engine_core::{Actor, FreshnessToken, SeatId, Seed};

    use crate::{
        ids::{board_dimensions, PieceId},
        rules::{Diagonal, MoveKind},
        setup::{setup_match, SetupOptions},
        state::{sorted_pieces, CellOccupancy, DraughtsLiteState, Piece, PieceKind},
        variants::Variant,
    };

    use super::*;

    fn coord(row: u8, col: u8) -> Coord {
        Coord::checked(row, col).unwrap()
    }

    fn piece_id(owner: DraughtsLiteSeat, ordinal: u8) -> PieceId {
        PieceId::new(owner, ordinal).unwrap()
    }

    fn man(owner: DraughtsLiteSeat, ordinal: u8, row: u8, col: u8) -> Piece {
        Piece {
            id: piece_id(owner, ordinal),
            owner,
            kind: PieceKind::Man,
            cell: coord(row, col),
        }
    }

    fn empty_state(active_seat: DraughtsLiteSeat, pieces: Vec<Piece>) -> DraughtsLiteState {
        let board = board_dimensions();
        let mut cells = DraughtsLiteState::empty_cells();
        for piece in &pieces {
            cells[piece.cell.row_col_index(board).unwrap()] = CellOccupancy::Occupied(piece.id);
        }

        DraughtsLiteState {
            variant: Variant::draughts_lite_standard(),
            board,
            cells,
            pieces: sorted_pieces(pieces),
            active_seat,
            seats: [SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())],
            ply_count: 0,
            command_count: 0,
            terminal_outcome: None,
            freshness_token: FreshnessToken(0),
        }
    }

    fn actor(state: &DraughtsLiteState, seat: DraughtsLiteSeat) -> Actor {
        Actor {
            seat_id: state.seats[seat.index()].clone(),
        }
    }

    fn metadata_value<'a>(choice: &'a ActionChoice, key: &str) -> Option<&'a str> {
        choice
            .metadata
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| entry.value.as_str())
    }

    fn child_segments(choice: &ActionChoice) -> Vec<&str> {
        choice
            .next
            .as_ref()
            .map(|node| {
                node.choices
                    .iter()
                    .map(|choice| choice.segment.as_str())
                    .collect()
            })
            .unwrap_or_default()
    }

    #[test]
    fn action_tree_lists_legal_origins_and_landings_in_order() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let state = setup_match(Seed(1), &seats, &SetupOptions::default()).unwrap();

        let tree = legal_action_tree(&state, &actor(&state, DraughtsLiteSeat::Seat0));
        let origins = tree
            .root
            .choices
            .iter()
            .map(|choice| choice.segment.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            origins,
            ["from/r3c2", "from/r3c4", "from/r3c6", "from/r3c8"]
        );
        assert_eq!(
            child_segments(&tree.root.choices[0]),
            ["to/r4c1", "to/r4c3"]
        );
        assert_eq!(
            metadata_value(&tree.root.choices[0], "phase"),
            Some("origin")
        );
        assert_eq!(
            metadata_value(&tree.root.choices[0], "capture_mandatory"),
            Some("false")
        );
    }

    #[test]
    fn action_tree_uses_jump_segments_and_suppresses_quiet_landings_when_capture_exists() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat0, 2, 3, 6),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
            ],
        );

        let tree = legal_action_tree(&state, &actor(&state, DraughtsLiteSeat::Seat0));

        assert_eq!(tree.root.choices.len(), 1);
        assert_eq!(tree.root.choices[0].segment, "from/r3c2");
        assert_eq!(child_segments(&tree.root.choices[0]), ["jump/r5c4"]);
        assert_eq!(
            metadata_value(&tree.root.choices[0], "capture_mandatory"),
            Some("true")
        );
    }

    #[test]
    fn jump_children_carry_forced_continuation_nodes_until_leaf() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 2, 6, 5),
            ],
        );

        let tree = legal_action_tree(&state, &actor(&state, DraughtsLiteSeat::Seat0));
        let first_jump = &tree.root.choices[0].next.as_ref().unwrap().choices[0];
        let continuation = &first_jump.next.as_ref().unwrap().choices[0];

        assert_eq!(first_jump.segment, "jump/r5c4");
        assert_eq!(
            metadata_value(first_jump, "forced_continuation_available"),
            Some("true")
        );
        assert_eq!(continuation.segment, "jump/r7c6");
        assert_eq!(
            metadata_value(continuation, "phase"),
            Some("forced_continuation_landing")
        );
        assert_eq!(
            metadata_value(continuation, "forced_by_continuation"),
            Some("true")
        );
        assert!(continuation.next.is_none());
    }

    #[test]
    fn jump_metadata_preview_matches_rules_step() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
            ],
        );

        let legal_move = legal_moves(&state)
            .into_iter()
            .find(|legal_move| legal_move.kind == MoveKind::Capture)
            .unwrap();
        assert_eq!(legal_move.steps[0].direction, Diagonal::Southeast);

        let tree = legal_action_tree(&state, &actor(&state, DraughtsLiteSeat::Seat0));
        let jump = &tree.root.choices[0].next.as_ref().unwrap().choices[0];

        assert_eq!(metadata_value(jump, "cell_id"), Some("r5c4"));
        assert_eq!(metadata_value(jump, "captured_cell"), Some("r4c3"));
        assert_eq!(metadata_value(jump, "preview_captured_cell"), Some("r4c3"));
        assert_eq!(
            metadata_value(jump, "captured_piece_id"),
            Some("seat_1-p01")
        );
        assert_eq!(metadata_value(jump, "would_promote"), Some("false"));
    }

    #[test]
    fn promotion_landing_is_leaf_with_promotion_metadata() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 6, 3),
                man(DraughtsLiteSeat::Seat1, 1, 7, 4),
                man(DraughtsLiteSeat::Seat1, 2, 7, 6),
            ],
        );

        let tree = legal_action_tree(&state, &actor(&state, DraughtsLiteSeat::Seat0));
        let jump = &tree.root.choices[0].next.as_ref().unwrap().choices[0];

        assert_eq!(jump.segment, "jump/r8c5");
        assert_eq!(metadata_value(jump, "would_promote"), Some("true"));
        assert_eq!(metadata_value(jump, "piece_kind_after"), Some("crown"));
        assert!(jump.next.is_none());
    }

    #[test]
    fn tree_is_empty_for_terminal_or_non_active_actor() {
        let mut state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![man(DraughtsLiteSeat::Seat0, 1, 3, 2)],
        );

        let inactive_tree = legal_action_tree(&state, &actor(&state, DraughtsLiteSeat::Seat1));
        assert!(inactive_tree.root.choices.is_empty());

        state.terminal_outcome = Some(crate::TerminalOutcome::Win {
            seat: DraughtsLiteSeat::Seat0,
        });
        let terminal_tree = legal_action_tree(&state, &actor(&state, DraughtsLiteSeat::Seat0));
        assert!(terminal_tree.root.choices.is_empty());
    }
}
