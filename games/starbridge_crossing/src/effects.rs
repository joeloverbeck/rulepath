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
    JumpChain {
        seat_index: u8,
        peg: StarPegId,
        from: StarSpaceId,
        hops: Vec<JumpSubstep>,
    },
    FinishAssigned {
        seat_index: u8,
        rank: u8,
    },
    PassBlocked {
        seat_index: u8,
    },
    Terminal {
        reason: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct JumpSubstep {
    pub over: StarSpaceId,
    pub to: StarSpaceId,
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

    #[test]
    fn jump_chain_effect_groups_ordered_substeps() {
        let effect = public_effect(StarbridgeEffect::JumpChain {
            seat_index: 0,
            peg: StarPegId::new(0, 3),
            from: StarSpaceId::new(12).unwrap(),
            hops: vec![
                JumpSubstep {
                    over: StarSpaceId::new(13).unwrap(),
                    to: StarSpaceId::new(14).unwrap(),
                },
                JumpSubstep {
                    over: StarSpaceId::new(15).unwrap(),
                    to: StarSpaceId::new(16).unwrap(),
                },
            ],
        });

        assert_eq!(effect.visibility, VisibilityScope::Public);
        assert_eq!(
            effect.payload,
            StarbridgeEffect::JumpChain {
                seat_index: 0,
                peg: StarPegId::new(0, 3),
                from: StarSpaceId::new(12).unwrap(),
                hops: vec![
                    JumpSubstep {
                        over: StarSpaceId::new(13).unwrap(),
                        to: StarSpaceId::new(14).unwrap(),
                    },
                    JumpSubstep {
                        over: StarSpaceId::new(15).unwrap(),
                        to: StarSpaceId::new(16).unwrap(),
                    },
                ],
            }
        );
    }

    #[test]
    fn finish_pass_and_terminal_effects_are_public() {
        for payload in [
            StarbridgeEffect::FinishAssigned {
                seat_index: 1,
                rank: 2,
            },
            StarbridgeEffect::PassBlocked { seat_index: 0 },
            StarbridgeEffect::Terminal {
                reason: "terminal-turn-limit".to_owned(),
            },
        ] {
            let effect = public_effect(payload);

            assert_eq!(effect.visibility, VisibilityScope::Public);
        }
    }
}
