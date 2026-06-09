use engine_core::{FreshnessToken, SeatId};

use crate::{
    effects::TerminalReason,
    ids::{CellId, ColumnId, DirectionalFlipSeat, RowId},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CellOccupancy {
    Empty,
    Occupied(DirectionalFlipSeat),
}

impl CellOccupancy {
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalOutcome {
    Win { seat: DirectionalFlipSeat },
    Draw,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectionalFlipState {
    pub variant: Variant,
    pub cells: [CellOccupancy; 64],
    pub active_seat: DirectionalFlipSeat,
    pub seats: [SeatId; 2],
    pub ply_count: u8,
    pub consecutive_forced_passes: u8,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub terminal_reason: Option<TerminalReason>,
    pub freshness_token: FreshnessToken,
}

impl DirectionalFlipState {
    pub fn empty_cells() -> [CellOccupancy; 64] {
        [CellOccupancy::Empty; 64]
    }

    pub fn occupancy(&self, cell: CellId) -> CellOccupancy {
        self.cells[cell.index()]
    }

    pub fn set_occupancy(&mut self, cell: CellId, occupancy: CellOccupancy) {
        self.cells[cell.index()] = occupancy;
    }

    pub fn cell(row_index: usize, column_index: usize) -> Option<CellId> {
        Some(CellId::new(
            RowId::from_index(row_index)?,
            ColumnId::from_index(column_index)?,
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectionalFlipSnapshot {
    pub schema_version: u32,
    pub rules_version: u32,
    pub rules_version_label: String,
    pub variant: Variant,
    pub cells: [CellOccupancy; 64],
    pub active_seat: DirectionalFlipSeat,
    pub seats: [SeatId; 2],
    pub ply_count: u8,
    pub consecutive_forced_passes: u8,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub terminal_reason: Option<TerminalReason>,
    pub freshness_token: FreshnessToken,
}

impl DirectionalFlipSnapshot {
    pub fn from_state(state: &DirectionalFlipState) -> Self {
        Self {
            schema_version: 1,
            rules_version: 1,
            rules_version_label: state.variant.rules_version_label.clone(),
            variant: state.variant.clone(),
            cells: state.cells,
            active_seat: state.active_seat,
            seats: state.seats.clone(),
            ply_count: state.ply_count,
            consecutive_forced_passes: state.consecutive_forced_passes,
            terminal_outcome: state.terminal_outcome,
            terminal_reason: state.terminal_reason,
            freshness_token: state.freshness_token,
        }
    }

    pub fn into_state(self) -> DirectionalFlipState {
        DirectionalFlipState {
            variant: self.variant,
            cells: self.cells,
            active_seat: self.active_seat,
            seats: self.seats,
            ply_count: self.ply_count,
            consecutive_forced_passes: self.consecutive_forced_passes,
            terminal_outcome: self.terminal_outcome,
            terminal_reason: self.terminal_reason,
            freshness_token: self.freshness_token,
        }
    }

    pub fn stable_summary(&self) -> String {
        let cells = CellId::ALL
            .iter()
            .map(|cell| {
                format!(
                    "{}:{}",
                    cell.as_string(),
                    occupancy_summary(self.cells[cell.index()])
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "schema={};rules={};rules_label={};variant={};seat_count={};first_seat={};columns={};rows={};cell_scheme={};opening={};ending={};cells={};active={};seat_0={};seat_1={};ply={};passes={};terminal={};terminal_reason={};freshness={}",
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
            cells,
            self.active_seat.as_str(),
            self.seats[0].0,
            self.seats[1].0,
            self.ply_count,
            self.consecutive_forced_passes,
            terminal_summary(self.terminal_outcome),
            self.terminal_reason.map_or("none", TerminalReason::as_str),
            self.freshness_token.0
        )
    }
}

impl engine_core::StableSerialize for DirectionalFlipSnapshot {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn occupancy_summary(occupancy: CellOccupancy) -> &'static str {
    match occupancy {
        CellOccupancy::Empty => "empty",
        CellOccupancy::Occupied(seat) => seat.as_str(),
    }
}

fn terminal_summary(outcome: Option<TerminalOutcome>) -> String {
    match outcome {
        None => "none".to_owned(),
        Some(TerminalOutcome::Draw) => "draw".to_owned(),
        Some(TerminalOutcome::Win { seat }) => format!("win:{}", seat.as_str()),
    }
}
