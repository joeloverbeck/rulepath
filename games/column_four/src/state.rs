use engine_core::{FreshnessToken, SeatId};

use crate::{
    ids::{CellId, ColumnFourSeat},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CellOccupancy {
    Empty,
    Occupied(ColumnFourSeat),
}

impl CellOccupancy {
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnFourState {
    pub variant: Variant,
    pub cells: [CellOccupancy; 42],
    pub active_seat: ColumnFourSeat,
    pub seats: [SeatId; 2],
    pub ply_count: u8,
    pub freshness_token: FreshnessToken,
}

impl ColumnFourState {
    pub fn empty_cells() -> [CellOccupancy; 42] {
        [CellOccupancy::Empty; 42]
    }

    pub fn occupancy(&self, cell: CellId) -> CellOccupancy {
        self.cells[cell.index()]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnFourSnapshot {
    pub schema_version: u32,
    pub rules_version: u32,
    pub rules_version_label: String,
    pub variant: Variant,
    pub cells: [CellOccupancy; 42],
    pub active_seat: ColumnFourSeat,
    pub seats: [SeatId; 2],
    pub ply_count: u8,
    pub freshness_token: FreshnessToken,
}

impl ColumnFourSnapshot {
    pub fn from_state(state: &ColumnFourState) -> Self {
        Self {
            schema_version: 1,
            rules_version: 1,
            rules_version_label: state.variant.rules_version_label.clone(),
            variant: state.variant.clone(),
            cells: state.cells,
            active_seat: state.active_seat,
            seats: state.seats.clone(),
            ply_count: state.ply_count,
            freshness_token: state.freshness_token,
        }
    }

    pub fn into_state(self) -> ColumnFourState {
        ColumnFourState {
            variant: self.variant,
            cells: self.cells,
            active_seat: self.active_seat,
            seats: self.seats,
            ply_count: self.ply_count,
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
            "schema={};rules={};rules_label={};variant={};seat_count={};first_seat={};columns={};rows={};cell_scheme={};ending={};cells={};active={};seat_0={};seat_1={};ply={};freshness={}",
            self.schema_version,
            self.rules_version,
            self.rules_version_label,
            self.variant.id,
            self.variant.seat_count,
            self.variant.first_seat,
            self.variant.board_columns,
            self.variant.board_rows,
            self.variant.cell_id_scheme,
            self.variant.ending,
            cells,
            self.active_seat.as_str(),
            self.seats[0].0,
            self.seats[1].0,
            self.ply_count,
            self.freshness_token.0
        )
    }
}

impl engine_core::StableSerialize for ColumnFourSnapshot {
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
