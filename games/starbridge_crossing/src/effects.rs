//! Semantic effects for Starbridge Crossing.

use engine_core::EffectEnvelope;

use crate::ids::StarSpaceId;
use crate::state::StarPegId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StarbridgeEffect {
    Step {
        seat_index: u8,
        peg: StarPegId,
        from: StarSpaceId,
        to: StarSpaceId,
    },
}

pub type StarbridgeEffectEnvelope = EffectEnvelope<StarbridgeEffect>;

pub fn public_effect(payload: StarbridgeEffect) -> StarbridgeEffectEnvelope {
    EffectEnvelope::public(payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::VisibilityScope;

    #[test]
    fn step_effect_is_public_and_names_peg_and_spaces() {
        let effect = public_effect(StarbridgeEffect::Step {
            seat_index: 0,
            peg: StarPegId::new(0, 3),
            from: StarSpaceId::new(12).unwrap(),
            to: StarSpaceId::new(13).unwrap(),
        });

        assert_eq!(effect.visibility, VisibilityScope::Public);
        assert_eq!(
            effect.payload,
            StarbridgeEffect::Step {
                seat_index: 0,
                peg: StarPegId::new(0, 3),
                from: StarSpaceId::new(12).unwrap(),
                to: StarSpaceId::new(13).unwrap(),
            }
        );
    }
}
