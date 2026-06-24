use engine_core::{EffectEnvelope, SeatId};

use crate::{
    ids::{CrestCardId, PokerLiteSeat},
    state::{PokerLiteState, ShowdownReveal, TerminalOutcome},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PokerLiteEffect {
    PrivateCrestDealt {
        owner: PokerLiteSeat,
        card: CrestCardId,
    },
    CrestDealStarted {
        private_count_per_seat: u8,
        center_count: u8,
        deck_tail_count: u8,
    },
    OpeningPoolSet {
        contributions: [u8; 2],
        shared_pool: u8,
    },
    PledgeHeld {
        actor: PokerLiteSeat,
        round_index: u8,
    },
    PledgePressed {
        actor: PokerLiteSeat,
        round_index: u8,
        amount: u8,
        shared_pool_after: u8,
    },
    PledgeLifted {
        actor: PokerLiteSeat,
        round_index: u8,
        amount: u8,
        shared_pool_after: u8,
        lift_cap_consumed: bool,
    },
    PledgeMatched {
        actor: PokerLiteSeat,
        round_index: u8,
        amount: u8,
        shared_pool_after: u8,
    },
    SeatYielded {
        actor: PokerLiteSeat,
        winner: PokerLiteSeat,
        shared_pool: u8,
    },
    CenterRevealStarted {
        group_id: String,
    },
    CenterRevealed {
        group_id: String,
        center: CrestCardId,
    },
    ShowdownRevealStarted {
        group_id: String,
    },
    ShowdownRevealed {
        group_id: String,
        reveal: ShowdownReveal,
    },
    LedgerResolved {
        shared_pool: u8,
        contributions: [u8; 2],
        allocation: LedgerAllocation,
    },
    Terminal {
        outcome: TerminalOutcome,
    },
    BotChoseActionPublic {
        policy_id: String,
        action_family: String,
    },
    BotChoseActionPrivate {
        owner: PokerLiteSeat,
        policy_id: String,
        action_family: String,
        strength_bucket: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LedgerAllocation {
    Winner { seat: PokerLiteSeat, amount: u8 },
    Split { each: u8 },
}

pub fn public_effect(payload: PokerLiteEffect) -> EffectEnvelope<PokerLiteEffect> {
    EffectEnvelope::public(payload)
}

pub fn private_effect(
    owner_seat_id: SeatId,
    payload: PokerLiteEffect,
) -> EffectEnvelope<PokerLiteEffect> {
    EffectEnvelope::private_to(owner_seat_id, payload)
}

pub fn setup_effects(state: &PokerLiteState) -> Vec<EffectEnvelope<PokerLiteEffect>> {
    vec![
        deal_private_crest_effect(
            PokerLiteSeat::Seat0,
            state.seats[PokerLiteSeat::Seat0.index()].clone(),
            state.private_card_for_internal(PokerLiteSeat::Seat0),
        ),
        deal_private_crest_effect(
            PokerLiteSeat::Seat1,
            state.seats[PokerLiteSeat::Seat1.index()].clone(),
            state.private_card_for_internal(PokerLiteSeat::Seat1),
        ),
        public_effect(PokerLiteEffect::CrestDealStarted {
            private_count_per_seat: 1,
            center_count: 1,
            deck_tail_count: state.deck_tail_internal().len() as u8,
        }),
        opening_pool_set_effect(state.contributions, state.shared_pool),
    ]
}

pub fn deal_private_crest_effect(
    owner: PokerLiteSeat,
    owner_seat_id: SeatId,
    card: CrestCardId,
) -> EffectEnvelope<PokerLiteEffect> {
    private_effect(
        owner_seat_id,
        PokerLiteEffect::PrivateCrestDealt { owner, card },
    )
}

pub fn opening_pool_set_effect(
    contributions: [u8; 2],
    shared_pool: u8,
) -> EffectEnvelope<PokerLiteEffect> {
    public_effect(PokerLiteEffect::OpeningPoolSet {
        contributions,
        shared_pool,
    })
}

pub fn pledge_held_effect(
    actor: PokerLiteSeat,
    round_index: u8,
) -> EffectEnvelope<PokerLiteEffect> {
    public_effect(PokerLiteEffect::PledgeHeld { actor, round_index })
}

pub fn pledge_pressed_effect(
    actor: PokerLiteSeat,
    round_index: u8,
    amount: u8,
    shared_pool_after: u8,
) -> EffectEnvelope<PokerLiteEffect> {
    public_effect(PokerLiteEffect::PledgePressed {
        actor,
        round_index,
        amount,
        shared_pool_after,
    })
}

pub fn pledge_lifted_effect(
    actor: PokerLiteSeat,
    round_index: u8,
    amount: u8,
    shared_pool_after: u8,
) -> EffectEnvelope<PokerLiteEffect> {
    public_effect(PokerLiteEffect::PledgeLifted {
        actor,
        round_index,
        amount,
        shared_pool_after,
        lift_cap_consumed: true,
    })
}

pub fn pledge_matched_effect(
    actor: PokerLiteSeat,
    round_index: u8,
    amount: u8,
    shared_pool_after: u8,
) -> EffectEnvelope<PokerLiteEffect> {
    public_effect(PokerLiteEffect::PledgeMatched {
        actor,
        round_index,
        amount,
        shared_pool_after,
    })
}

pub fn seat_yielded_effect(
    actor: PokerLiteSeat,
    winner: PokerLiteSeat,
    shared_pool: u8,
) -> EffectEnvelope<PokerLiteEffect> {
    public_effect(PokerLiteEffect::SeatYielded {
        actor,
        winner,
        shared_pool,
    })
}

pub fn center_reveal_effects(center: CrestCardId) -> Vec<EffectEnvelope<PokerLiteEffect>> {
    let group_id = "poker_lite_center_reveal".to_owned();
    vec![
        public_effect(PokerLiteEffect::CenterRevealStarted {
            group_id: group_id.clone(),
        }),
        public_effect(PokerLiteEffect::CenterRevealed { group_id, center }),
    ]
}

pub fn showdown_reveal_effects(reveal: ShowdownReveal) -> Vec<EffectEnvelope<PokerLiteEffect>> {
    let group_id = "poker_lite_showdown_reveal".to_owned();
    vec![
        public_effect(PokerLiteEffect::ShowdownRevealStarted {
            group_id: group_id.clone(),
        }),
        public_effect(PokerLiteEffect::ShowdownRevealed { group_id, reveal }),
    ]
}

pub fn ledger_resolved_effect(
    shared_pool: u8,
    contributions: [u8; 2],
    allocation: LedgerAllocation,
) -> EffectEnvelope<PokerLiteEffect> {
    public_effect(PokerLiteEffect::LedgerResolved {
        shared_pool,
        contributions,
        allocation,
    })
}

pub fn terminal_effect(outcome: TerminalOutcome) -> EffectEnvelope<PokerLiteEffect> {
    public_effect(PokerLiteEffect::Terminal { outcome })
}

pub fn bot_chose_action_public_effect(
    policy_id: impl Into<String>,
    action_family: impl Into<String>,
) -> EffectEnvelope<PokerLiteEffect> {
    public_effect(PokerLiteEffect::BotChoseActionPublic {
        policy_id: policy_id.into(),
        action_family: action_family.into(),
    })
}

pub fn bot_chose_action_private_effect(
    owner: PokerLiteSeat,
    owner_seat_id: SeatId,
    policy_id: impl Into<String>,
    action_family: impl Into<String>,
    strength_bucket: impl Into<String>,
) -> EffectEnvelope<PokerLiteEffect> {
    private_effect(
        owner_seat_id,
        PokerLiteEffect::BotChoseActionPrivate {
            owner,
            policy_id: policy_id.into(),
            action_family: action_family.into(),
            strength_bucket: strength_bucket.into(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{setup_match, SetupOptions};
    use engine_core::{SeatId, Seed, VisibilityScope};

    #[test]
    fn private_deal_effects_are_scoped_to_owner() {
        let state = setup_match(
            Seed(5),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        let effects = setup_effects(&state);

        assert_eq!(
            effects[0].visibility,
            VisibilityScope::PrivateToSeat(SeatId("seat_0".to_owned()))
        );
        assert_eq!(
            effects[1].visibility,
            VisibilityScope::PrivateToSeat(SeatId("seat_1".to_owned()))
        );
        assert_eq!(effects[2].visibility, VisibilityScope::Public);
        assert_eq!(effects[3].visibility, VisibilityScope::Public);

        let public_text = format!("{:?}{:?}", effects[2], effects[3]);
        for card in CrestCardId::ALL {
            assert!(!public_text.contains(card.as_str()));
            assert!(!public_text.contains(card.rank().label()));
        }
    }

    #[test]
    fn private_effect_constructor_preserves_owner_scope_and_payload() {
        let owner = SeatId("seat_1".to_owned());
        let payload = PokerLiteEffect::PrivateCrestDealt {
            owner: PokerLiteSeat::Seat1,
            card: CrestCardId::HighDusk,
        };
        let effect = private_effect(owner.clone(), payload.clone());

        assert_eq!(effect.visibility, VisibilityScope::PrivateToSeat(owner));
        assert_eq!(effect.payload, payload);
    }

    #[test]
    fn showdown_reveal_is_single_grouped_payload() {
        let reveal = ShowdownReveal {
            seat_0_private: CrestCardId::LowDawn,
            seat_1_private: CrestCardId::HighDusk,
            center: CrestCardId::LowDusk,
        };
        let effects = showdown_reveal_effects(reveal);

        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0].visibility, VisibilityScope::Public);
        assert_eq!(effects[1].visibility, VisibilityScope::Public);
        assert!(matches!(
            effects[0].payload,
            PokerLiteEffect::ShowdownRevealStarted { .. }
        ));
        assert!(matches!(
            effects[1].payload,
            PokerLiteEffect::ShowdownRevealed {
                reveal: actual,
                ..
            } if actual == reveal
        ));
    }
}
