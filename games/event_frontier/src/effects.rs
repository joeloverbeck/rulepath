//! Semantic effects for Event Frontier.

use engine_core::{EffectEnvelope, VisibilityScope};

use crate::ids::SiteId;
use crate::{cards::CardId, ids::FactionId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SiteScoreBreakdown {
    pub site: SiteId,
    pub charter_presence: u8,
    pub freeholder_presence: u8,
    pub awarded_to: Option<FactionId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventFrontierEffect {
    EventResolved {
        card: CardId,
        summary: String,
    },
    EdictActivated {
        card: CardId,
        edict: String,
    },
    EdictExpired {
        edict: String,
    },
    CardRevealed {
        card: CardId,
        next_public: Option<CardId>,
    },
    ChoiceTaken {
        faction: FactionId,
        choice: String,
    },
    CardDiscarded {
        card: CardId,
        reason: String,
    },
    EligibilityChanged {
        faction: FactionId,
        eligible: bool,
        reason: String,
    },
    ResourcesChanged {
        faction: FactionId,
        previous: u8,
        new: u8,
        reason: String,
    },
    OpResolved {
        faction: FactionId,
        op: String,
        sites: Vec<SiteId>,
    },
    AgentPlaced {
        site: SiteId,
        new_count: u8,
    },
    AgentRemoved {
        site: SiteId,
        new_count: u8,
    },
    DepotBuilt {
        site: SiteId,
    },
    CacheRemoved {
        site: SiteId,
        new_count: u8,
    },
    SettlerMoved {
        from: SiteId,
        to: SiteId,
        from_count: u8,
        to_count: u8,
    },
    CacheLaid {
        site: SiteId,
        new_count: u8,
    },
    SettlerRallied {
        site: SiteId,
        new_count: u8,
    },
    ReckoningResolved {
        round: u8,
        victory_check: String,
        site_breakdown: Vec<SiteScoreBreakdown>,
        income: (u8, u8),
        expired_edicts: Vec<String>,
    },
    Terminal {
        winner: FactionId,
        victory_type: String,
        totals: (u16, u16),
        summary: String,
    },
}

pub type EventFrontierEffectEnvelope = EffectEnvelope<EventFrontierEffect>;

pub fn public_effect(payload: EventFrontierEffect) -> EventFrontierEffectEnvelope {
    EffectEnvelope {
        visibility: VisibilityScope::Public,
        payload,
    }
}
