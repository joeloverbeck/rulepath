use engine_core::{FreshnessToken, SeatId};

use crate::{
    ids::{CellId, ThreeMarksSeat},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CellOccupancy {
    Empty,
    Occupied(ThreeMarksSeat),
}

impl CellOccupancy {
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct WinningLine {
    pub cells: [CellId; 3],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalOutcome {
    Win {
        seat: ThreeMarksSeat,
        line: WinningLine,
    },
    Draw,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThreeMarksState {
    pub variant: Variant,
    pub cells: [CellOccupancy; 9],
    pub active_seat: ThreeMarksSeat,
    pub seats: [SeatId; 2],
    pub ply_count: u8,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl ThreeMarksState {
    pub fn empty_cells() -> [CellOccupancy; 9] {
        [CellOccupancy::Empty; 9]
    }

    pub fn occupancy(&self, cell: CellId) -> CellOccupancy {
        self.cells[cell.index()]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThreeMarksSnapshot {
    pub schema_version: u32,
    pub rules_version: u32,
    pub rules_version_label: String,
    pub variant: Variant,
    pub cells: [CellOccupancy; 9],
    pub active_seat: ThreeMarksSeat,
    pub seats: [SeatId; 2],
    pub ply_count: u8,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl ThreeMarksSnapshot {
    pub fn from_state(state: &ThreeMarksState) -> Self {
        Self {
            schema_version: 1,
            rules_version: 1,
            rules_version_label: state.variant.rules_version_label.clone(),
            variant: state.variant.clone(),
            cells: state.cells,
            active_seat: state.active_seat,
            seats: state.seats.clone(),
            ply_count: state.ply_count,
            terminal_outcome: state.terminal_outcome,
            freshness_token: state.freshness_token,
        }
    }

    pub fn into_state(self) -> ThreeMarksState {
        ThreeMarksState {
            variant: self.variant,
            cells: self.cells,
            active_seat: self.active_seat,
            seats: self.seats,
            ply_count: self.ply_count,
            terminal_outcome: self.terminal_outcome,
            freshness_token: self.freshness_token,
        }
    }

    pub fn stable_summary(&self) -> String {
        let cells = CellId::ALL
            .iter()
            .map(|cell| {
                format!(
                    "{}:{}",
                    cell.as_str(),
                    occupancy_summary(self.cells[cell.index()])
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "schema={};rules={};rules_label={};variant={};seat_count={};first_seat={};cell_scheme={};ending={};cells={};active={};seat_0={};seat_1={};ply={};terminal={};freshness={}",
            self.schema_version,
            self.rules_version,
            self.rules_version_label,
            self.variant.id,
            self.variant.seat_count,
            self.variant.first_seat,
            self.variant.cell_id_scheme,
            self.variant.ending,
            cells,
            self.active_seat.as_str(),
            self.seats[0].0,
            self.seats[1].0,
            self.ply_count,
            terminal_summary(self.terminal_outcome),
            self.freshness_token.0
        )
    }
}

impl engine_core::StableSerialize for ThreeMarksSnapshot {
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
        Some(TerminalOutcome::Win { seat, line }) => format!(
            "win:{}:{}-{}-{}",
            seat.as_str(),
            line.cells[0].as_str(),
            line.cells[1].as_str(),
            line.cells[2].as_str()
        ),
    }
}
