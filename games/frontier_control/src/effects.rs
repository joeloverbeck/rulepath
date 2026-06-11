//! Semantic effects for Frontier Control.

use engine_core::{EffectEnvelope, VisibilityScope};

use crate::ids::{FactionId, SiteId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FrontierControlEffect {
    CrewMarched {
        from: SiteId,
        to: SiteId,
    },
    GuardPatrolled {
        from: SiteId,
        to: SiteId,
    },
    ClashResolved {
        site: SiteId,
        guard_removed: bool,
        crew_removed: bool,
        entering_faction: FactionId,
    },
    StakePlaced {
        site: SiteId,
    },
    StakeDismantled {
        site: SiteId,
    },
    CrewMustered {
        site: SiteId,
        crews: u8,
    },
    GuardReinforced {
        site: SiteId,
        guards: u8,
    },
    TurnEnded {
        faction: FactionId,
        round: u8,
    },
}

pub type FrontierControlEffectEnvelope = EffectEnvelope<FrontierControlEffect>;

pub fn public_effect(payload: FrontierControlEffect) -> FrontierControlEffectEnvelope {
    EffectEnvelope {
        visibility: VisibilityScope::Public,
        payload,
    }
}
