//! State skeleton for Flood Watch.

use crate::{ids::DistrictId, variants::ScenarioVariant};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistrictState {
    pub district: DistrictId,
    pub flood_level: u8,
    pub levees: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FloodWatchState {
    pub variant: ScenarioVariant,
    pub districts: Vec<DistrictState>,
}
