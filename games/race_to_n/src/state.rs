use engine_core::{FreshnessToken, SeatId};

use crate::{ids::RaceSeat, variants::Variant};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CounterValue(pub u8);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaceState {
    pub variant: Variant,
    pub counter: CounterValue,
    pub active_seat: RaceSeat,
    pub seats: [SeatId; 2],
    pub winner: Option<RaceSeat>,
    pub freshness_token: FreshnessToken,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FoundationView {
    pub counter: CounterValue,
    pub active_seat: RaceSeat,
    pub winner: Option<RaceSeat>,
    pub freshness_token: FreshnessToken,
}
