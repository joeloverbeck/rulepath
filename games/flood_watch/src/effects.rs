//! Semantic effects for Flood Watch.

use engine_core::EffectEnvelope;

use crate::ids::{DistrictId, EventKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalSummary {
    pub rule_id: String,
    pub public_summary: String,
    pub drawn_card_count: u8,
    pub surviving_levels: Vec<(DistrictId, u8)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FloodWatchEffect {
    DistrictBailed {
        district: DistrictId,
        amount: u8,
    },
    LeveePlaced {
        district: DistrictId,
        amount: u8,
    },
    ForecastRevealed {
        card: EventKind,
    },
    EnvironmentPhaseBegan {
        turn: u16,
        draws: u8,
    },
    EventDrawn {
        index: u8,
        card: EventKind,
    },
    LeveeAbsorbed {
        district: DistrictId,
        amount: u8,
        remaining_levees: u8,
    },
    FloodLevelRose {
        district: DistrictId,
        amount: u8,
        new_level: u8,
    },
    DistrictInundated {
        district: DistrictId,
    },
    DeckExhausted,
    Terminal {
        outcome: String,
        summary: TerminalSummary,
    },
}

pub type FloodWatchEffectEnvelope = EffectEnvelope<FloodWatchEffect>;

pub fn public_effect(payload: FloodWatchEffect) -> FloodWatchEffectEnvelope {
    EffectEnvelope::public(payload)
}
