use engine_core::{FreshnessToken, SeatId, StableSerialize};
use game_stdlib::board_space::{Coord, Dimensions};

use crate::{
    ids::{board_dimensions, DraughtsLiteSeat, PieceId, TOTAL_STANDARD_PIECES},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PieceKind {
    Man,
    Crown,
}

impl PieceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Man => "man",
            Self::Crown => "crown",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Piece {
    pub id: PieceId,
    pub owner: DraughtsLiteSeat,
    pub kind: PieceKind,
    pub cell: Coord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CellOccupancy {
    Empty,
    Occupied(PieceId),
}

impl CellOccupancy {
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalOutcome {
    Win { seat: DraughtsLiteSeat },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DraughtsLiteState {
    pub variant: Variant,
    pub board: Dimensions,
    pub cells: [CellOccupancy; 64],
    pub pieces: Vec<Piece>,
    pub active_seat: DraughtsLiteSeat,
    pub seats: [SeatId; 2],
    pub ply_count: u32,
    pub command_count: u32,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl DraughtsLiteState {
    pub fn empty_cells() -> [CellOccupancy; 64] {
        [CellOccupancy::Empty; 64]
    }

    pub fn occupancy(&self, cell: Coord) -> Option<CellOccupancy> {
        Some(self.cells[cell.row_col_index(self.board)?])
    }

    pub fn set_occupancy(&mut self, cell: Coord, occupancy: CellOccupancy) -> Option<()> {
        let index = cell.row_col_index(self.board)?;
        self.cells[index] = occupancy;
        Some(())
    }

    pub fn piece(&self, id: PieceId) -> Option<&Piece> {
        self.pieces.iter().find(|piece| piece.id == id)
    }

    pub fn pieces_for_seat(&self, seat: DraughtsLiteSeat) -> impl Iterator<Item = &Piece> {
        self.pieces.iter().filter(move |piece| piece.owner == seat)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DraughtsLiteSnapshot {
    pub schema_version: u32,
    pub rules_version: u32,
    pub rules_version_label: String,
    pub variant: Variant,
    pub board: Dimensions,
    pub cells: [CellOccupancy; 64],
    pub pieces: Vec<Piece>,
    pub active_seat: DraughtsLiteSeat,
    pub seats: [SeatId; 2],
    pub ply_count: u32,
    pub command_count: u32,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl DraughtsLiteSnapshot {
    pub fn from_state(state: &DraughtsLiteState) -> Self {
        Self {
            schema_version: 1,
            rules_version: 1,
            rules_version_label: state.variant.rules_version_label.clone(),
            variant: state.variant.clone(),
            board: state.board,
            cells: state.cells,
            pieces: state.pieces.clone(),
            active_seat: state.active_seat,
            seats: state.seats.clone(),
            ply_count: state.ply_count,
            command_count: state.command_count,
            terminal_outcome: state.terminal_outcome,
            freshness_token: state.freshness_token,
        }
    }

    pub fn into_state(self) -> DraughtsLiteState {
        DraughtsLiteState {
            variant: self.variant,
            board: self.board,
            cells: self.cells,
            pieces: self.pieces,
            active_seat: self.active_seat,
            seats: self.seats,
            ply_count: self.ply_count,
            command_count: self.command_count,
            terminal_outcome: self.terminal_outcome,
            freshness_token: self.freshness_token,
        }
    }

    pub fn stable_summary(&self) -> String {
        let cells = board_dimensions()
            .row_major()
            .map(|cell| {
                format!(
                    "{}:{}",
                    cell.id(),
                    occupancy_summary(self.cells[cell.row_col_index(self.board).unwrap()])
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let pieces = self
            .pieces
            .iter()
            .map(piece_summary)
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "schema={};rules={};rules_label={};variant={};seat_count={};first_seat={};columns={};rows={};cell_scheme={};opening={};ending={};board_rows={};board_cols={};cells={};pieces={};active={};seat_0={};seat_1={};ply={};commands={};terminal={};freshness={}",
            self.schema_version,
            self.rules_version,
            self.rules_version_label,
            self.variant.id,
            self.variant.seat_count,
            self.variant.first_seat,
            self.variant.board_columns,
            self.variant.board_rows,
            self.variant.cell_id_scheme,
            self.variant.opening,
            self.variant.ending,
            self.board.rows(),
            self.board.cols(),
            cells,
            pieces,
            self.active_seat.as_str(),
            self.seats[0].0,
            self.seats[1].0,
            self.ply_count,
            self.command_count,
            terminal_summary(self.terminal_outcome),
            self.freshness_token.0
        )
    }
}

impl StableSerialize for DraughtsLiteSnapshot {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn piece_summary(piece: &Piece) -> String {
    format!(
        "{}:{}:{}:{}",
        piece.id.stable_id(),
        piece.owner.as_str(),
        piece.kind.as_str(),
        piece.cell.id()
    )
}

fn occupancy_summary(occupancy: CellOccupancy) -> String {
    match occupancy {
        CellOccupancy::Empty => "empty".to_owned(),
        CellOccupancy::Occupied(id) => id.stable_id(),
    }
}

fn terminal_summary(outcome: Option<TerminalOutcome>) -> String {
    match outcome {
        None => "none".to_owned(),
        Some(TerminalOutcome::Win { seat }) => format!("win:{}", seat.as_str()),
    }
}

pub fn sorted_standard_pieces(mut pieces: Vec<Piece>) -> Vec<Piece> {
    pieces.sort_by_key(|piece| piece.id);
    debug_assert_eq!(pieces.len(), TOTAL_STANDARD_PIECES);
    pieces
}

#[cfg(test)]
mod tests {
    use engine_core::{SeatId, Seed};

    use crate::{setup::SetupOptions, setup_match};

    use super::*;

    #[test]
    fn snapshot_round_trip_preserves_state() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let state = setup_match(Seed(1), &seats, &SetupOptions::default()).unwrap();
        let snapshot = DraughtsLiteSnapshot::from_state(&state);

        assert_eq!(snapshot.clone().into_state(), state);
        assert_eq!(
            DraughtsLiteSnapshot::from_state(&snapshot.into_state()).stable_bytes(),
            DraughtsLiteSnapshot::from_state(&state).stable_bytes()
        );
    }

    #[test]
    fn stable_summary_uses_piece_ids_and_board_order() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let state = setup_match(Seed(1), &seats, &SetupOptions::default()).unwrap();
        let summary = DraughtsLiteSnapshot::from_state(&state).stable_summary();

        assert!(summary.contains("board_rows=8;board_cols=8"));
        assert!(summary.contains("r1c1:empty,r1c2:seat_0-p01"));
        assert!(summary.contains("seat_1-p12:seat_1:man:r8c7"));
        assert!(summary.contains("freshness=0"));
    }
}
