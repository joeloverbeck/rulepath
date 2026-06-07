use game_stdlib::board_space::Coord;

use crate::{
    ids::{is_playable_cell, DraughtsLiteSeat, PieceId},
    state::{CellOccupancy, DraughtsLiteState, Piece, PieceKind, TerminalOutcome},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Diagonal {
    Northwest,
    Northeast,
    Southeast,
    Southwest,
}

impl Diagonal {
    pub const KING_ORDER: [Self; 4] = [
        Self::Northwest,
        Self::Northeast,
        Self::Southeast,
        Self::Southwest,
    ];

    pub fn man_order(seat: DraughtsLiteSeat) -> [Self; 2] {
        match seat {
            DraughtsLiteSeat::Seat0 => [Self::Southwest, Self::Southeast],
            DraughtsLiteSeat::Seat1 => [Self::Northwest, Self::Northeast],
        }
    }

    fn for_piece(piece: Piece) -> Vec<Self> {
        match piece.kind {
            PieceKind::Man => Self::man_order(piece.owner).to_vec(),
            PieceKind::Crown => Self::KING_ORDER.to_vec(),
        }
    }

    fn delta(self) -> (i16, i16) {
        match self {
            Self::Northwest => (-1, -1),
            Self::Northeast => (-1, 1),
            Self::Southeast => (1, 1),
            Self::Southwest => (1, -1),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MoveKind {
    Quiet,
    Capture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CaptureDetail {
    pub cell: Coord,
    pub piece_id: PieceId,
    pub owner: DraughtsLiteSeat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MoveStep {
    pub from: Coord,
    pub to: Coord,
    pub direction: Diagonal,
    pub capture: Option<CaptureDetail>,
    pub piece_kind_before: PieceKind,
    pub piece_kind_after: PieceKind,
    pub promotes: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegalMove {
    pub actor: DraughtsLiteSeat,
    pub piece_id: PieceId,
    pub origin: Coord,
    pub steps: Vec<MoveStep>,
    pub kind: MoveKind,
}

impl LegalMove {
    pub fn final_cell(&self) -> Coord {
        self.steps
            .last()
            .map(|step| step.to)
            .expect("legal moves always contain at least one step")
    }

    pub fn captured_piece_ids(&self) -> Vec<PieceId> {
        self.steps
            .iter()
            .filter_map(|step| step.capture.map(|capture| capture.piece_id))
            .collect()
    }

    pub fn promotes(&self) -> bool {
        self.steps.iter().any(|step| step.promotes)
    }
}

pub fn legal_moves(state: &DraughtsLiteState) -> Vec<LegalMove> {
    legal_moves_for(state, state.active_seat)
}

pub fn legal_moves_for(state: &DraughtsLiteState, seat: DraughtsLiteSeat) -> Vec<LegalMove> {
    if state.terminal_outcome.is_some() {
        return Vec::new();
    }

    let captures = capture_moves_for(state, seat);
    if captures.is_empty() {
        quiet_moves_for(state, seat)
    } else {
        captures
    }
}

pub fn has_legal_move(state: &DraughtsLiteState, seat: DraughtsLiteSeat) -> bool {
    !legal_moves_for(state, seat).is_empty()
}

pub fn terminal_outcome_for_active_player(state: &DraughtsLiteState) -> Option<TerminalOutcome> {
    terminal_outcome_for_seat_to_act(state, state.active_seat)
}

pub fn terminal_outcome_for_seat_to_act(
    state: &DraughtsLiteState,
    seat_to_act: DraughtsLiteSeat,
) -> Option<TerminalOutcome> {
    if state.terminal_outcome.is_some() {
        return state.terminal_outcome;
    }

    if state.pieces_for_seat(seat_to_act).next().is_none() || !has_legal_move(state, seat_to_act) {
        Some(TerminalOutcome::Win {
            seat: seat_to_act.other(),
        })
    } else {
        None
    }
}

fn quiet_moves_for(state: &DraughtsLiteState, seat: DraughtsLiteSeat) -> Vec<LegalMove> {
    let mut moves = Vec::new();

    for piece in pieces_by_origin(state, seat) {
        for direction in Diagonal::for_piece(piece) {
            let Some(to) = step(state, piece.cell, direction, 1) else {
                continue;
            };
            if !is_empty_virtual(state, to, piece.id, piece.cell, &[]) {
                continue;
            }

            let promotes = promotes_on(piece, to);
            moves.push(LegalMove {
                actor: seat,
                piece_id: piece.id,
                origin: piece.cell,
                steps: vec![MoveStep {
                    from: piece.cell,
                    to,
                    direction,
                    capture: None,
                    piece_kind_before: piece.kind,
                    piece_kind_after: if promotes {
                        PieceKind::Crown
                    } else {
                        piece.kind
                    },
                    promotes,
                }],
                kind: MoveKind::Quiet,
            });
        }
    }

    moves
}

fn capture_moves_for(state: &DraughtsLiteState, seat: DraughtsLiteSeat) -> Vec<LegalMove> {
    let mut moves = Vec::new();

    for piece in pieces_by_origin(state, seat) {
        let sequences = capture_sequences_from(state, piece, piece.cell, piece.kind, Vec::new());
        for steps in sequences {
            moves.push(LegalMove {
                actor: seat,
                piece_id: piece.id,
                origin: piece.cell,
                steps,
                kind: MoveKind::Capture,
            });
        }
    }

    moves
}

fn capture_sequences_from(
    state: &DraughtsLiteState,
    piece: Piece,
    from: Coord,
    kind: PieceKind,
    captured_so_far: Vec<PieceId>,
) -> Vec<Vec<MoveStep>> {
    let mut sequences = Vec::new();

    for direction in Diagonal::for_piece(Piece {
        kind,
        cell: from,
        ..piece
    }) {
        let Some(over) = step(state, from, direction, 1) else {
            continue;
        };
        let Some(to) = step(state, from, direction, 2) else {
            continue;
        };
        if !is_empty_virtual(state, to, piece.id, from, &captured_so_far) {
            continue;
        }

        let Some(captured) = piece_at_virtual(state, over, piece.id, from, &captured_so_far) else {
            continue;
        };
        if captured.owner == piece.owner {
            continue;
        }

        let promotes = kind == PieceKind::Man && reaches_crown_row(piece.owner, to);
        let after_kind = if promotes { PieceKind::Crown } else { kind };
        let step = MoveStep {
            from,
            to,
            direction,
            capture: Some(CaptureDetail {
                cell: over,
                piece_id: captured.id,
                owner: captured.owner,
            }),
            piece_kind_before: kind,
            piece_kind_after: after_kind,
            promotes,
        };

        if promotes {
            sequences.push(vec![step]);
            continue;
        }

        let mut next_captured = captured_so_far.clone();
        next_captured.push(captured.id);
        let continuations = capture_sequences_from(state, piece, to, after_kind, next_captured);
        if continuations.is_empty() {
            sequences.push(vec![step]);
        } else {
            for continuation in continuations {
                let mut full = Vec::with_capacity(1 + continuation.len());
                full.push(step);
                full.extend(continuation);
                sequences.push(full);
            }
        }
    }

    sequences
}

fn pieces_by_origin(state: &DraughtsLiteState, seat: DraughtsLiteSeat) -> Vec<Piece> {
    state
        .board
        .row_major()
        .filter_map(|cell| {
            let CellOccupancy::Occupied(id) = state.occupancy(cell)? else {
                return None;
            };
            let piece = state.piece(id).copied()?;
            (piece.owner == seat).then_some(piece)
        })
        .collect()
}

fn step(
    state: &DraughtsLiteState,
    from: Coord,
    direction: Diagonal,
    distance: i16,
) -> Option<Coord> {
    let (d_row, d_col) = direction.delta();
    let to = state
        .board
        .offset(from, d_row * distance, d_col * distance)?;
    is_playable_cell(to).then_some(to)
}

fn is_empty_virtual(
    state: &DraughtsLiteState,
    cell: Coord,
    moving_piece: PieceId,
    moving_cell: Coord,
    captured: &[PieceId],
) -> bool {
    piece_at_virtual(state, cell, moving_piece, moving_cell, captured).is_none()
}

fn piece_at_virtual(
    state: &DraughtsLiteState,
    cell: Coord,
    moving_piece: PieceId,
    moving_cell: Coord,
    captured: &[PieceId],
) -> Option<Piece> {
    if cell == moving_cell {
        return state.piece(moving_piece).copied();
    }

    let CellOccupancy::Occupied(id) = state.occupancy(cell)? else {
        return None;
    };
    if id == moving_piece || captured.contains(&id) {
        return None;
    }
    state.piece(id).copied()
}

fn promotes_on(piece: Piece, to: Coord) -> bool {
    piece.kind == PieceKind::Man && reaches_crown_row(piece.owner, to)
}

fn reaches_crown_row(seat: DraughtsLiteSeat, cell: Coord) -> bool {
    match seat {
        DraughtsLiteSeat::Seat0 => cell.row() == 8,
        DraughtsLiteSeat::Seat1 => cell.row() == 1,
    }
}

#[cfg(test)]
mod tests {
    use engine_core::{FreshnessToken, SeatId, Seed};

    use crate::{
        ids::board_dimensions,
        setup::{setup_match, SetupOptions},
        state::sorted_pieces,
        variants::Variant,
    };

    use super::*;

    fn coord(row: u8, col: u8) -> Coord {
        Coord::checked(row, col).unwrap()
    }

    fn piece_id(owner: DraughtsLiteSeat, ordinal: u8) -> PieceId {
        PieceId::new(owner, ordinal).unwrap()
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

    fn man(owner: DraughtsLiteSeat, ordinal: u8, row: u8, col: u8) -> Piece {
        Piece {
            id: piece_id(owner, ordinal),
            owner,
            kind: PieceKind::Man,
            cell: coord(row, col),
        }
    }

    fn crown(owner: DraughtsLiteSeat, ordinal: u8, row: u8, col: u8) -> Piece {
        Piece {
            id: piece_id(owner, ordinal),
            owner,
            kind: PieceKind::Crown,
            cell: coord(row, col),
        }
    }

    #[test]
    fn standard_setup_is_non_terminal_and_has_quiet_moves() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let state = setup_match(Seed(1), &seats, &SetupOptions::default()).unwrap();
        let moves = legal_moves(&state);

        assert_eq!(terminal_outcome_for_active_player(&state), None);
        assert!(moves
            .iter()
            .all(|legal_move| legal_move.kind == MoveKind::Quiet));
        assert_eq!(
            moves
                .iter()
                .map(|legal_move| (legal_move.origin.id(), legal_move.final_cell().id()))
                .collect::<Vec<_>>(),
            [
                ("r3c2".to_owned(), "r4c1".to_owned()),
                ("r3c2".to_owned(), "r4c3".to_owned()),
                ("r3c4".to_owned(), "r4c3".to_owned()),
                ("r3c4".to_owned(), "r4c5".to_owned()),
                ("r3c6".to_owned(), "r4c5".to_owned()),
                ("r3c6".to_owned(), "r4c7".to_owned()),
                ("r3c8".to_owned(), "r4c7".to_owned()),
            ]
        );
    }

    #[test]
    fn men_move_and_capture_forward_only() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 1, 5, 4),
                man(DraughtsLiteSeat::Seat1, 2, 3, 4),
            ],
        );

        let moves = legal_moves(&state);

        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].kind, MoveKind::Capture);
        assert_eq!(moves[0].steps[0].to, coord(6, 5));
        assert_eq!(moves[0].steps[0].capture.unwrap().cell, coord(5, 4));
    }

    #[test]
    fn kings_move_and_capture_in_all_diagonals() {
        let quiet_state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![crown(DraughtsLiteSeat::Seat0, 1, 4, 3)],
        );
        let quiet = legal_moves(&quiet_state);
        assert_eq!(
            quiet
                .iter()
                .map(|legal_move| legal_move.final_cell().id())
                .collect::<Vec<_>>(),
            ["r3c2", "r3c4", "r5c4", "r5c2"]
        );

        let capture_state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                crown(DraughtsLiteSeat::Seat0, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 2, 3, 4),
                man(DraughtsLiteSeat::Seat1, 3, 5, 4),
                man(DraughtsLiteSeat::Seat1, 4, 5, 2),
            ],
        );
        let captures = legal_moves(&capture_state);

        assert_eq!(
            captures
                .iter()
                .map(|legal_move| legal_move.final_cell().id())
                .collect::<Vec<_>>(),
            ["r2c1", "r2c5", "r6c5", "r6c1"]
        );
    }

    #[test]
    fn mandatory_capture_suppresses_quiet_moves_globally() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat0, 2, 3, 6),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
            ],
        );

        let moves = legal_moves(&state);

        assert!(moves
            .iter()
            .all(|legal_move| legal_move.kind == MoveKind::Capture));
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].piece_id, piece_id(DraughtsLiteSeat::Seat0, 1));
    }

    #[test]
    fn mandatory_continuation_builds_complete_capture_sequences() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 2, 6, 5),
            ],
        );

        let moves = legal_moves(&state);

        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].steps.len(), 2);
        assert_eq!(
            moves[0]
                .steps
                .iter()
                .map(|step| step.to.id())
                .collect::<Vec<_>>(),
            ["r5c4", "r7c6"]
        );
        assert_eq!(
            moves[0].captured_piece_ids(),
            [
                piece_id(DraughtsLiteSeat::Seat1, 1),
                piece_id(DraughtsLiteSeat::Seat1, 2)
            ]
        );
    }

    #[test]
    fn already_captured_piece_is_unavailable_for_continuation() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                crown(DraughtsLiteSeat::Seat0, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 1, 3, 4),
            ],
        );

        let moves = legal_moves(&state);

        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].steps.len(), 1);
        assert_eq!(moves[0].final_cell(), coord(2, 5));
    }

    #[test]
    fn promotion_during_capture_stops_sequence() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 6, 3),
                man(DraughtsLiteSeat::Seat1, 1, 7, 4),
                man(DraughtsLiteSeat::Seat1, 2, 7, 6),
            ],
        );

        let moves = legal_moves(&state);

        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].steps.len(), 1);
        assert_eq!(moves[0].final_cell(), coord(8, 5));
        assert!(moves[0].promotes());
        assert_eq!(moves[0].steps[0].piece_kind_after, PieceKind::Crown);
    }

    #[test]
    fn quiet_promotion_is_reported() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![man(DraughtsLiteSeat::Seat0, 1, 7, 2)],
        );

        let moves = legal_moves(&state);

        assert_eq!(moves.len(), 2);
        assert!(moves.iter().all(LegalMove::promotes));
        assert_eq!(
            moves
                .iter()
                .map(|legal_move| legal_move.final_cell().id())
                .collect::<Vec<_>>(),
            ["r8c1", "r8c3"]
        );
    }

    #[test]
    fn terminal_detection_covers_no_pieces_and_no_legal_move() {
        let no_pieces = empty_state(
            DraughtsLiteSeat::Seat1,
            vec![man(DraughtsLiteSeat::Seat0, 1, 3, 2)],
        );
        assert_eq!(
            terminal_outcome_for_active_player(&no_pieces),
            Some(TerminalOutcome::Win {
                seat: DraughtsLiteSeat::Seat0
            })
        );

        let blocked = empty_state(
            DraughtsLiteSeat::Seat1,
            vec![
                man(DraughtsLiteSeat::Seat1, 1, 1, 2),
                man(DraughtsLiteSeat::Seat0, 1, 2, 1),
                man(DraughtsLiteSeat::Seat0, 2, 2, 3),
            ],
        );
        assert_eq!(
            terminal_outcome_for_active_player(&blocked),
            Some(TerminalOutcome::Win {
                seat: DraughtsLiteSeat::Seat0
            })
        );
    }
}
