use engine_core::{FreshnessToken, SeatId, StableSerialize};

use crate::{
    ids::{StarPoint, StarSpaceId, SPACE_COUNT},
    topology::spaces_by_stable_order,
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StarPegId {
    pub seat_index: u8,
    pub ordinal: u8,
}

impl StarPegId {
    pub const fn new(seat_index: u8, ordinal: u8) -> Self {
        Self {
            seat_index,
            ordinal,
        }
    }

    pub fn stable_id(self) -> String {
        format!("p{}_{}", self.seat_index, self.ordinal)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct StarPeg {
    pub id: StarPegId,
    pub owner_seat_index: u8,
    pub space: StarSpaceId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatAssignment {
    pub seat_id: SeatId,
    pub seat_index: u8,
    pub home: StarPoint,
    pub target: StarPoint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FinishRank {
    pub seat_index: u8,
    pub rank: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalStatus {
    Complete,
    TurnLimit { max_plies: u32 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StarbridgeState {
    pub variant: Variant,
    pub seats: Vec<SeatAssignment>,
    pub occupancy: [Option<StarPegId>; SPACE_COUNT as usize],
    pub pegs: Vec<StarPeg>,
    pub active_seat_index: u8,
    pub finish_ranks: Vec<FinishRank>,
    pub terminal_status: Option<TerminalStatus>,
    pub ply_count: u32,
    pub command_count: u32,
    pub freshness_token: FreshnessToken,
}

impl StarbridgeState {
    pub fn empty_occupancy() -> [Option<StarPegId>; SPACE_COUNT as usize] {
        [None; SPACE_COUNT as usize]
    }

    pub fn occupancy(&self, space: StarSpaceId) -> Option<StarPegId> {
        self.occupancy[usize::from(space.index())]
    }

    pub fn pegs_for_seat(&self, seat_index: u8) -> impl Iterator<Item = &StarPeg> {
        self.pegs
            .iter()
            .filter(move |peg| peg.owner_seat_index == seat_index)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StarbridgeSnapshot {
    pub schema_version: u32,
    pub rules_version: u32,
    pub rules_version_label: String,
    pub variant: Variant,
    pub seats: Vec<SeatAssignment>,
    pub occupancy: [Option<StarPegId>; SPACE_COUNT as usize],
    pub pegs: Vec<StarPeg>,
    pub active_seat_index: u8,
    pub finish_ranks: Vec<FinishRank>,
    pub terminal_status: Option<TerminalStatus>,
    pub ply_count: u32,
    pub command_count: u32,
    pub freshness_token: FreshnessToken,
}

impl StarbridgeSnapshot {
    pub fn from_state(state: &StarbridgeState) -> Self {
        Self {
            schema_version: 1,
            rules_version: 1,
            rules_version_label: state.variant.rules_version_label.clone(),
            variant: state.variant.clone(),
            seats: state.seats.clone(),
            occupancy: state.occupancy,
            pegs: state.pegs.clone(),
            active_seat_index: state.active_seat_index,
            finish_ranks: state.finish_ranks.clone(),
            terminal_status: state.terminal_status,
            ply_count: state.ply_count,
            command_count: state.command_count,
            freshness_token: state.freshness_token,
        }
    }

    pub fn into_state(self) -> StarbridgeState {
        StarbridgeState {
            variant: self.variant,
            seats: self.seats,
            occupancy: self.occupancy,
            pegs: self.pegs,
            active_seat_index: self.active_seat_index,
            finish_ranks: self.finish_ranks,
            terminal_status: self.terminal_status,
            ply_count: self.ply_count,
            command_count: self.command_count,
            freshness_token: self.freshness_token,
        }
    }

    pub fn stable_summary(&self) -> String {
        let seats = self
            .seats
            .iter()
            .map(|seat| {
                format!(
                    "{}:{}:{}:{}",
                    seat.seat_index,
                    seat.seat_id.0,
                    seat.home.label(),
                    seat.target.label()
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let occupancy = spaces_by_stable_order()
            .map(|space| {
                format!(
                    "{}:{}",
                    space.id,
                    self.occupancy[usize::from(space.id.index())]
                        .map_or_else(|| "empty".to_owned(), StarPegId::stable_id)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let pegs = self
            .pegs
            .iter()
            .map(|peg| {
                format!(
                    "{}:{}:{}",
                    peg.id.stable_id(),
                    peg.owner_seat_index,
                    peg.space
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let finish = self
            .finish_ranks
            .iter()
            .map(|rank| format!("{}:{}", rank.seat_index, rank.rank))
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "schema={};rules={};rules_label={};variant={};seat_count={};seats={};occupancy={};pegs={};active={};finish={};terminal={};ply={};commands={};freshness={}",
            self.schema_version,
            self.rules_version,
            self.rules_version_label,
            self.variant.id,
            self.seats.len(),
            seats,
            occupancy,
            pegs,
            self.active_seat_index,
            finish,
            terminal_summary(self.terminal_status),
            self.ply_count,
            self.command_count,
            self.freshness_token.0
        )
    }
}

impl StableSerialize for StarbridgeSnapshot {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn terminal_summary(status: Option<TerminalStatus>) -> String {
    match status {
        None => "none".to_owned(),
        Some(TerminalStatus::Complete) => "complete".to_owned(),
        Some(TerminalStatus::TurnLimit { max_plies }) => format!("turn_limit:{max_plies}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::{setup_match, SetupOptions};
    use engine_core::{SeatId, Seed};

    #[test]
    fn setup_snapshot_round_trip_is_deterministic() {
        let seats = vec![
            SeatId::from_zero_based_index(0),
            SeatId::from_zero_based_index(1),
        ];
        let left = setup_match(Seed(1), &seats, &SetupOptions::default()).unwrap();
        let right = setup_match(Seed(99), &seats, &SetupOptions::default()).unwrap();

        let left_snapshot = StarbridgeSnapshot::from_state(&left);
        let right_snapshot = StarbridgeSnapshot::from_state(&right);

        assert_eq!(left_snapshot, right_snapshot);
        assert_eq!(left_snapshot.stable_bytes(), right_snapshot.stable_bytes());
        assert_eq!(left_snapshot.clone().into_state(), left);
    }
}
