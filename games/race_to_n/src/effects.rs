use engine_core::{EffectEnvelope, VisibilityScope};

use crate::{ids::RaceSeat, state::CounterValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RaceEffect {
    ActionStarted {
        actor: RaceSeat,
        amount: u8,
    },
    CounterAdvanced {
        actor: RaceSeat,
        from: CounterValue,
        to: CounterValue,
        amount: u8,
    },
    TurnChanged {
        next_actor: RaceSeat,
    },
    GameEnded {
        winner: RaceSeat,
    },
    ActionCompleted {
        actor: RaceSeat,
    },
}

pub fn public_effect(payload: RaceEffect) -> EffectEnvelope<RaceEffect> {
    EffectEnvelope {
        visibility: VisibilityScope::Public,
        payload,
    }
}
