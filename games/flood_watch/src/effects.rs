//! Semantic-effect skeleton for Flood Watch.

use crate::ids::{DistrictId, EventKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FloodWatchEffect {
    DistrictBailed { district: DistrictId, amount: u8 },
    LeveePlaced { district: DistrictId, amount: u8 },
    ForecastRevealed { card: EventKind },
    EnvironmentPhaseBegan { draw_count: u8 },
    EventDrawn { card: EventKind },
    LeveeAbsorbed { district: DistrictId, amount: u8 },
    FloodLevelRose { district: DistrictId, amount: u8 },
    DistrictInundated { district: DistrictId },
    DeckExhausted,
    Terminal { shared_outcome: String },
}
