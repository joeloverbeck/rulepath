use engine_core::{Actor, CommandEnvelope, Diagnostic, EffectEnvelope};
use game_stdlib::board_space::{Coord, CoordIdError};

use crate::{
    actions::{FROM_SEGMENT_PREFIX, JUMP_SEGMENT_PREFIX, TO_SEGMENT_PREFIX},
    effects::{
        forced_capture_available_effect, move_effects, terminal_win_effect, DraughtsLiteEffect,
        TerminalWinReason,
    },
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub actor: DraughtsLiteSeat,
    pub legal_move: LegalMove,
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

pub fn validate_command(
    state: &DraughtsLiteState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if state.terminal_outcome.is_some() {
        return Err(diagnostic(
            "terminal_match",
            "the match is already finished",
        ));
    }

    if command.freshness_token != state.freshness_token {
        return Err(diagnostic(
            "stale_action",
            "the action was submitted for an older decision point",
        ));
    }

    let Some(actor) = actor_seat(state, &command.actor) else {
        return Err(diagnostic("unknown_actor", "the actor is not seated"));
    };

    if actor != state.active_seat {
        return Err(diagnostic(
            "not_active_seat",
            "only the active seat may act now",
        ));
    }

    if command.action_path.segments.is_empty() {
        return Err(diagnostic("empty_action_path", "the action path is empty"));
    }

    let path = parse_path(state, &command.action_path.segments)?;
    let legal_moves = legal_moves_for(state, actor);
    let legal_paths = legal_moves
        .iter()
        .map(segments_for_move)
        .collect::<Vec<_>>();

    if let Some((index, _)) = legal_paths
        .iter()
        .enumerate()
        .find(|(_, segments)| *segments == &command.action_path.segments)
    {
        return Ok(ValidatedAction {
            actor,
            legal_move: legal_moves[index].clone(),
        });
    }

    if legal_paths
        .iter()
        .any(|segments| segments.starts_with(&command.action_path.segments))
    {
        return Err(diagnostic(
            "mandatory_continuation_incomplete",
            "the capture path stops before a mandatory continuation",
        ));
    }

    if legal_paths.iter().any(|segments| {
        command.action_path.segments.starts_with(segments)
            && command.action_path.segments.len() > segments.len()
            && legal_moves
                .iter()
                .find(|legal_move| segments_for_move(legal_move) == *segments)
                .is_some_and(LegalMove::promotes)
    }) {
        return Err(diagnostic(
            "continues_after_promotion_stop",
            "the path continues after promotion ended the capture",
        ));
    }

    if path.segments.len() >= 2
        && path.segments[1].kind == ParsedSegmentKind::Quiet
        && legal_moves
            .iter()
            .any(|legal_move| legal_move.kind == MoveKind::Capture)
    {
        return Err(diagnostic(
            "quiet_move_while_capture_available",
            "a capture is available, so quiet moves are not legal",
        ));
    }

    Err(diagnostic(
        "action_path_not_available",
        "the action path is not available",
    ))
}

pub fn apply_action(
    state: &mut DraughtsLiteState,
    action: ValidatedAction,
) -> Vec<EffectEnvelope<DraughtsLiteEffect>> {
    let piece_id = action.legal_move.piece_id;
    let mut effects = move_effects(&action.legal_move);

    for step in &action.legal_move.steps {
        state.set_occupancy(step.from, CellOccupancy::Empty);
        if let Some(capture) = step.capture {
            state.set_occupancy(capture.cell, CellOccupancy::Empty);
            state.pieces.retain(|piece| piece.id != capture.piece_id);
        }
        state.set_occupancy(step.to, CellOccupancy::Occupied(piece_id));

        let piece = state
            .pieces
            .iter_mut()
            .find(|piece| piece.id == piece_id)
            .expect("validated action piece must exist");
        piece.cell = step.to;
        piece.kind = step.piece_kind_after;
    }

    state.ply_count = state.ply_count.saturating_add(1);
    state.command_count = state.command_count.saturating_add(1);
    state.freshness_token = state.freshness_token.next();

    let next_seat = action.actor.other();
    if let Some((outcome, reason)) = terminal_outcome_and_reason_for_seat_to_act(state, next_seat) {
        state.terminal_outcome = Some(outcome);
        state.terminal_reason = Some(reason);
        let TerminalOutcome::Win { seat: winner } = outcome;
        effects.push(terminal_win_effect(winner, next_seat, reason));
    } else {
        state.active_seat = next_seat;
        let captures = capture_moves_for(state, next_seat);
        if !captures.is_empty() {
            let mut origins = captures
                .iter()
                .map(|legal_move| legal_move.piece_id)
                .collect::<Vec<_>>();
            origins.sort();
            origins.dedup();
            effects.push(forced_capture_available_effect(
                next_seat,
                origins.len() as u8,
            ));
        }
    }

    effects
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

fn terminal_outcome_and_reason_for_seat_to_act(
    state: &DraughtsLiteState,
    seat_to_act: DraughtsLiteSeat,
) -> Option<(TerminalOutcome, TerminalWinReason)> {
    if state.pieces_for_seat(seat_to_act).next().is_none() {
        Some((
            TerminalOutcome::Win {
                seat: seat_to_act.other(),
            },
            TerminalWinReason::OpponentNoPieces,
        ))
    } else if !has_legal_move(state, seat_to_act) {
        Some((
            TerminalOutcome::Win {
                seat: seat_to_act.other(),
            },
            TerminalWinReason::OpponentNoLegalMove,
        ))
    } else {
        None
    }
}

pub fn segments_for_move(legal_move: &LegalMove) -> Vec<String> {
    let mut segments = Vec::with_capacity(1 + legal_move.steps.len());
    segments.push(format!("{FROM_SEGMENT_PREFIX}{}", legal_move.origin.id()));
    segments.extend(legal_move.steps.iter().map(|step| {
        if step.capture.is_some() {
            format!("{JUMP_SEGMENT_PREFIX}{}", step.to.id())
        } else {
            format!("{TO_SEGMENT_PREFIX}{}", step.to.id())
        }
    }));
    segments
}

fn actor_seat(state: &DraughtsLiteState, actor: &Actor) -> Option<DraughtsLiteSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(DraughtsLiteSeat::from_index)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParsedPath {
    segments: Vec<ParsedSegment>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParsedSegment {
    kind: ParsedSegmentKind,
    cell: Coord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParsedSegmentKind {
    Origin,
    Quiet,
    Jump,
}

fn parse_path(state: &DraughtsLiteState, segments: &[String]) -> Result<ParsedPath, Diagnostic> {
    let mut parsed = Vec::with_capacity(segments.len());
    for (index, segment) in segments.iter().enumerate() {
        let (kind, cell) = parse_segment(state, segment, index)?;
        parsed.push(ParsedSegment { kind, cell });
    }

    if parsed[0].kind != ParsedSegmentKind::Origin {
        return Err(diagnostic(
            "malformed_segment",
            "the first action segment must select an origin",
        ));
    }

    Ok(ParsedPath { segments: parsed })
}

fn parse_segment(
    state: &DraughtsLiteState,
    segment: &str,
    index: usize,
) -> Result<(ParsedSegmentKind, Coord), Diagnostic> {
    let (kind, raw_cell) = if let Some(raw_cell) = segment.strip_prefix(FROM_SEGMENT_PREFIX) {
        (ParsedSegmentKind::Origin, raw_cell)
    } else if let Some(raw_cell) = segment.strip_prefix(TO_SEGMENT_PREFIX) {
        (ParsedSegmentKind::Quiet, raw_cell)
    } else if let Some(raw_cell) = segment.strip_prefix(JUMP_SEGMENT_PREFIX) {
        (ParsedSegmentKind::Jump, raw_cell)
    } else {
        return Err(diagnostic(
            "malformed_segment",
            "the action segment has an unknown prefix",
        ));
    };

    let cell = state
        .board
        .parse_coord_id(raw_cell)
        .map_err(|error| coord_diagnostic(error, index == 0))?;
    if !is_playable_cell(cell) {
        return Err(diagnostic(
            if index == 0 {
                "origin_not_playable"
            } else {
                "destination_not_playable"
            },
            "the action segment refers to a non-playable cell",
        ));
    }
    Ok((kind, cell))
}

fn coord_diagnostic(error: CoordIdError, origin: bool) -> Diagnostic {
    match error {
        CoordIdError::Malformed | CoordIdError::Zero => diagnostic(
            "malformed_segment",
            "the action segment does not contain a valid cell id",
        ),
        CoordIdError::OutOfBounds => diagnostic(
            if origin {
                "origin_outside_board"
            } else {
                "destination_outside_board"
            },
            "the action segment is outside the board",
        ),
    }
}

fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use engine_core::{
        ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed,
        StableSerialize,
    };

    use crate::{
        effects::{DraughtsLiteEffect, TerminalWinReason},
        ids::board_dimensions,
        setup::{setup_match, SetupOptions},
        state::{sorted_pieces, DraughtsLiteSnapshot},
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
            terminal_reason: None,
            freshness_token: FreshnessToken(0),
        }
    }

    fn actor_for(state: &DraughtsLiteState, seat: DraughtsLiteSeat) -> Actor {
        Actor {
            seat_id: state.seats[seat.index()].clone(),
        }
    }

    fn command_for(
        state: &DraughtsLiteState,
        seat: DraughtsLiteSeat,
        segments: Vec<&str>,
    ) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor_for(state, seat),
            action_path: ActionPath {
                segments: segments.into_iter().map(str::to_owned).collect(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
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
                man(DraughtsLiteSeat::Seat1, 3, 8, 7),
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

    #[test]
    fn validate_accepts_current_leaf_path_and_apply_is_atomic_for_multi_jump() {
        let mut state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 2, 6, 5),
                man(DraughtsLiteSeat::Seat1, 3, 8, 7),
            ],
        );
        let command = command_for(
            &state,
            DraughtsLiteSeat::Seat0,
            vec!["from/r3c2", "jump/r5c4", "jump/r7c6"],
        );

        let action = validate_command(&state, &command).expect("path validates");
        let effects = apply_action(&mut state, action);

        let moved_id = piece_id(DraughtsLiteSeat::Seat0, 1);
        assert_eq!(state.piece(moved_id).unwrap().cell, coord(7, 6));
        assert_eq!(state.occupancy(coord(3, 2)), Some(CellOccupancy::Empty));
        assert_eq!(state.occupancy(coord(4, 3)), Some(CellOccupancy::Empty));
        assert_eq!(state.occupancy(coord(6, 5)), Some(CellOccupancy::Empty));
        assert_eq!(
            state.occupancy(coord(7, 6)),
            Some(CellOccupancy::Occupied(moved_id))
        );
        assert!(state.piece(piece_id(DraughtsLiteSeat::Seat1, 1)).is_none());
        assert!(state.piece(piece_id(DraughtsLiteSeat::Seat1, 2)).is_none());
        assert!(state.piece(piece_id(DraughtsLiteSeat::Seat1, 3)).is_some());
        assert_eq!(state.active_seat, DraughtsLiteSeat::Seat1);
        assert_eq!(state.ply_count, 1);
        assert_eq!(state.command_count, 1);
        assert_eq!(state.freshness_token, FreshnessToken(1));
        assert!(matches!(
            effects[0].payload.clone(),
            DraughtsLiteEffect::MoveCommitted {
                move_kind: MoveKind::Capture,
                path_length: 2,
                ..
            }
        ));
        assert!(matches!(
            effects[1].payload.clone(),
            DraughtsLiteEffect::CaptureStep {
                captured_cell,
                captured_piece_id,
                ..
            } if captured_cell == coord(4, 3)
                && captured_piece_id == piece_id(DraughtsLiteSeat::Seat1, 1)
        ));
        assert!(matches!(
            effects[2].payload.clone(),
            DraughtsLiteEffect::ForcedContinuationRequired {
                current_landing,
                continuation_destination_count: 1,
                ..
            } if current_landing == coord(5, 4)
        ));
        assert!(matches!(
            effects[3].payload.clone(),
            DraughtsLiteEffect::CaptureStep {
                captured_cell,
                captured_piece_id,
                ..
            } if captured_cell == coord(6, 5)
                && captured_piece_id == piece_id(DraughtsLiteSeat::Seat1, 2)
        ));
    }

    #[test]
    fn invalid_command_returns_stable_diagnostic_and_does_not_mutate() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
            ],
        );
        let mutated = state.clone();
        let before = DraughtsLiteSnapshot::from_state(&mutated).stable_bytes();
        let command = command_for(
            &mutated,
            DraughtsLiteSeat::Seat0,
            vec!["from/r3c2", "to/r4c1"],
        );

        let diagnostic = validate_command(&mutated, &command).expect_err("quiet rejected");

        assert_eq!(diagnostic.code, "quiet_move_while_capture_available");
        assert_eq!(
            DraughtsLiteSnapshot::from_state(&mutated).stable_bytes(),
            before
        );
    }

    #[test]
    fn validation_rejects_stale_wrong_actor_terminal_empty_and_malformed_paths() {
        let mut state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![man(DraughtsLiteSeat::Seat0, 1, 3, 2)],
        );

        let mut stale = command_for(
            &state,
            DraughtsLiteSeat::Seat0,
            vec!["from/r3c2", "to/r4c1"],
        );
        stale.freshness_token = FreshnessToken(99);
        assert_eq!(
            validate_command(&state, &stale).unwrap_err().code,
            "stale_action"
        );

        let wrong_actor = command_for(
            &state,
            DraughtsLiteSeat::Seat1,
            vec!["from/r3c2", "to/r4c1"],
        );
        assert_eq!(
            validate_command(&state, &wrong_actor).unwrap_err().code,
            "not_active_seat"
        );

        let empty = command_for(&state, DraughtsLiteSeat::Seat0, Vec::new());
        assert_eq!(
            validate_command(&state, &empty).unwrap_err().code,
            "empty_action_path"
        );

        let malformed = command_for(&state, DraughtsLiteSeat::Seat0, vec!["bad/r3c2"]);
        assert_eq!(
            validate_command(&state, &malformed).unwrap_err().code,
            "malformed_segment"
        );

        state.terminal_outcome = Some(TerminalOutcome::Win {
            seat: DraughtsLiteSeat::Seat0,
        });
        let terminal = command_for(
            &state,
            DraughtsLiteSeat::Seat0,
            vec!["from/r3c2", "to/r4c1"],
        );
        assert_eq!(
            validate_command(&state, &terminal).unwrap_err().code,
            "terminal_match"
        );
    }

    #[test]
    fn validation_rejects_partial_continuation_and_promotion_stop_overrun() {
        let continuation_state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 2, 6, 5),
            ],
        );
        let partial = command_for(
            &continuation_state,
            DraughtsLiteSeat::Seat0,
            vec!["from/r3c2", "jump/r5c4"],
        );
        assert_eq!(
            validate_command(&continuation_state, &partial)
                .unwrap_err()
                .code,
            "mandatory_continuation_incomplete"
        );

        let promotion_state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 6, 3),
                man(DraughtsLiteSeat::Seat1, 1, 7, 4),
                man(DraughtsLiteSeat::Seat1, 2, 7, 6),
            ],
        );
        let overrun = command_for(
            &promotion_state,
            DraughtsLiteSeat::Seat0,
            vec!["from/r6c3", "jump/r8c5", "jump/r6c7"],
        );
        assert_eq!(
            validate_command(&promotion_state, &overrun)
                .unwrap_err()
                .code,
            "continues_after_promotion_stop"
        );
    }

    #[test]
    fn apply_emits_quiet_promotion_forced_capture_and_terminal_effects() {
        let mut quiet_state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 5, 4),
            ],
        );
        let quiet = validate_command(
            &quiet_state,
            &command_for(
                &quiet_state,
                DraughtsLiteSeat::Seat0,
                vec!["from/r3c2", "to/r4c3"],
            ),
        )
        .unwrap();
        let quiet_effects = apply_action(&mut quiet_state, quiet);
        assert!(matches!(
            quiet_effects[0].payload.clone(),
            DraughtsLiteEffect::MoveCommitted {
                move_kind: MoveKind::Quiet,
                start_cell,
                final_cell,
                ..
            } if start_cell == coord(3, 2) && final_cell == coord(4, 3)
        ));
        assert!(matches!(
            quiet_effects[1].payload.clone(),
            DraughtsLiteEffect::QuietStep {
                origin,
                landing,
                ..
            } if origin == coord(3, 2) && landing == coord(4, 3)
        ));
        assert!(matches!(
            quiet_effects[2].payload.clone(),
            DraughtsLiteEffect::ForcedCaptureAvailable {
                active_seat: DraughtsLiteSeat::Seat1,
                capture_origin_count: 1,
                ..
            }
        ));

        let mut promotion_state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![man(DraughtsLiteSeat::Seat0, 1, 7, 2)],
        );
        let promotion = validate_command(
            &promotion_state,
            &command_for(
                &promotion_state,
                DraughtsLiteSeat::Seat0,
                vec!["from/r7c2", "to/r8c1"],
            ),
        )
        .unwrap();
        let promotion_effects = apply_action(&mut promotion_state, promotion);
        assert!(matches!(
            promotion_effects[2].payload.clone(),
            DraughtsLiteEffect::Promotion {
                piece_id: promoted_piece_id,
                cell,
                during_capture: false,
                ..
            } if promoted_piece_id == piece_id(DraughtsLiteSeat::Seat0, 1)
                && cell == coord(8, 1)
        ));

        let mut terminal_state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
            ],
        );
        let terminal = validate_command(
            &terminal_state,
            &command_for(
                &terminal_state,
                DraughtsLiteSeat::Seat0,
                vec!["from/r3c2", "jump/r5c4"],
            ),
        )
        .unwrap();
        let terminal_effects = apply_action(&mut terminal_state, terminal);
        assert!(matches!(
            terminal_effects.last().unwrap().payload.clone(),
            DraughtsLiteEffect::TerminalWin {
                winner: DraughtsLiteSeat::Seat0,
                loser: DraughtsLiteSeat::Seat1,
                reason: TerminalWinReason::OpponentNoPieces,
            }
        ));
    }
}
