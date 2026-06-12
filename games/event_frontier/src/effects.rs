//! Semantic effects for Event Frontier.

use engine_core::{EffectEnvelope, VisibilityScope};

use crate::{cards::CardId, ids::FactionId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventFrontierEffect {
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
}

pub type EventFrontierEffectEnvelope = EffectEnvelope<EventFrontierEffect>;

pub fn public_effect(payload: EventFrontierEffect) -> EventFrontierEffectEnvelope {
    EffectEnvelope {
        visibility: VisibilityScope::Public,
        payload,
    }
}
