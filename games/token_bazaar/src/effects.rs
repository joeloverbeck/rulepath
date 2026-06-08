use engine_core::{EffectEnvelope, StableSerialize, VisibilityScope};

use crate::{
    ids::{CollectBundleId, ContractId, ResourceId, TokenBazaarSeat, TokenBazaarSlot},
    state::{ResourceCounts, TerminalOutcome},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenBazaarEffect {
    ResourceCollected {
        seat: TokenBazaarSeat,
        bundle: CollectBundleId,
        gain: ResourceCounts,
        inventory_after: ResourceCounts,
        supply_after: ResourceCounts,
    },
    ResourceExchanged {
        seat: TokenBazaarSeat,
        paid_resource: ResourceId,
        taken_resource: ResourceId,
        cost: ResourceCounts,
        gain: ResourceCounts,
        inventory_after: ResourceCounts,
        supply_after: ResourceCounts,
    },
    ContractFulfilled {
        seat: TokenBazaarSeat,
        slot: TokenBazaarSlot,
        contract: ContractId,
        cost: ResourceCounts,
        points: u8,
        score_after: u32,
        fulfilled_count_after: u8,
    },
    SlotRefilled {
        slot: TokenBazaarSlot,
        contract: ContractId,
        remaining_queue_len: u8,
    },
    SlotEmptied {
        slot: TokenBazaarSlot,
        remaining_queue_len: u8,
    },
    PassAccepted {
        seat: TokenBazaarSeat,
    },
    TurnAdvanced {
        previous_seat: TokenBazaarSeat,
        active_seat: TokenBazaarSeat,
        turns_taken: [u8; 2],
    },
    Terminal {
        outcome: TerminalOutcome,
        scores: [u32; 2],
        fulfilled_counts: [u8; 2],
        inventory_totals: [u16; 2],
    },
}

impl TokenBazaarEffect {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::ResourceCollected { .. } => "tb_resource_collected",
            Self::ResourceExchanged { .. } => "tb_resource_exchanged",
            Self::ContractFulfilled { .. } => "tb_contract_fulfilled",
            Self::SlotRefilled { .. } => "tb_slot_refilled",
            Self::SlotEmptied { .. } => "tb_slot_emptied",
            Self::PassAccepted { .. } => "tb_pass_accepted",
            Self::TurnAdvanced { .. } => "tb_turn_advanced",
            Self::Terminal { .. } => "tb_terminal",
        }
    }

    pub fn stable_summary(&self) -> String {
        match self {
            Self::ResourceCollected {
                seat,
                bundle,
                gain,
                inventory_after,
                supply_after,
            } => format!(
                "{}:seat={};bundle={};gain={};inventory_after={};supply_after={}",
                self.kind(),
                seat.as_str(),
                bundle.as_str(),
                counts_summary(*gain),
                counts_summary(*inventory_after),
                counts_summary(*supply_after)
            ),
            Self::ResourceExchanged {
                seat,
                paid_resource,
                taken_resource,
                cost,
                gain,
                inventory_after,
                supply_after,
            } => format!(
                "{}:seat={};paid={};taken={};cost={};gain={};inventory_after={};supply_after={}",
                self.kind(),
                seat.as_str(),
                paid_resource.as_str(),
                taken_resource.as_str(),
                counts_summary(*cost),
                counts_summary(*gain),
                counts_summary(*inventory_after),
                counts_summary(*supply_after)
            ),
            Self::ContractFulfilled {
                seat,
                slot,
                contract,
                cost,
                points,
                score_after,
                fulfilled_count_after,
            } => format!(
                "{}:seat={};slot={};contract={};cost={};points={};score_after={};fulfilled_count_after={}",
                self.kind(),
                seat.as_str(),
                slot.as_str(),
                contract.as_str(),
                counts_summary(*cost),
                points,
                score_after,
                fulfilled_count_after
            ),
            Self::SlotRefilled {
                slot,
                contract,
                remaining_queue_len,
            } => format!(
                "{}:slot={};contract={};remaining_queue_len={}",
                self.kind(),
                slot.as_str(),
                contract.as_str(),
                remaining_queue_len
            ),
            Self::SlotEmptied {
                slot,
                remaining_queue_len,
            } => format!(
                "{}:slot={};remaining_queue_len={}",
                self.kind(),
                slot.as_str(),
                remaining_queue_len
            ),
            Self::PassAccepted { seat } => {
                format!("{}:seat={}", self.kind(), seat.as_str())
            }
            Self::TurnAdvanced {
                previous_seat,
                active_seat,
                turns_taken,
            } => format!(
                "{}:previous={};active={};turns_0={};turns_1={}",
                self.kind(),
                previous_seat.as_str(),
                active_seat.as_str(),
                turns_taken[0],
                turns_taken[1]
            ),
            Self::Terminal {
                outcome,
                scores,
                fulfilled_counts,
                inventory_totals,
            } => format!(
                "{}:outcome={};score_0={};score_1={};fulfilled_0={};fulfilled_1={};inventory_total_0={};inventory_total_1={}",
                self.kind(),
                outcome_summary(*outcome),
                scores[0],
                scores[1],
                fulfilled_counts[0],
                fulfilled_counts[1],
                inventory_totals[0],
                inventory_totals[1]
            ),
        }
    }
}

impl StableSerialize for TokenBazaarEffect {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

pub fn public_effect(payload: TokenBazaarEffect) -> EffectEnvelope<TokenBazaarEffect> {
    EffectEnvelope {
        visibility: VisibilityScope::Public,
        payload,
    }
}

fn counts_summary(counts: ResourceCounts) -> String {
    format!(
        "amber={},jade={},iron={}",
        counts.amber, counts.jade, counts.iron
    )
}

fn outcome_summary(outcome: TerminalOutcome) -> String {
    match outcome {
        TerminalOutcome::Draw => "draw".to_owned(),
        TerminalOutcome::Win { seat } => format!("win:{}", seat.as_str()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::{StableSerialize, VisibilityScope};

    #[test]
    fn effects_are_public_and_stably_serialized() {
        let effect = public_effect(TokenBazaarEffect::ResourceCollected {
            seat: TokenBazaarSeat::Seat0,
            bundle: CollectBundleId::AmberJade,
            gain: ResourceCounts::new(1, 1, 0),
            inventory_after: ResourceCounts::new(2, 2, 1),
            supply_after: ResourceCounts::new(13, 13, 14),
        });

        assert_eq!(effect.visibility, VisibilityScope::Public);
        assert_eq!(effect.payload.kind(), "tb_resource_collected");
        assert_eq!(
            effect.payload.stable_summary(),
            "tb_resource_collected:seat=seat_0;bundle=amber-jade;gain=amber=1,jade=1,iron=0;inventory_after=amber=2,jade=2,iron=1;supply_after=amber=13,jade=13,iron=14"
        );
        assert_eq!(
            effect.payload.stable_bytes(),
            effect.payload.stable_summary().into_bytes()
        );
    }
}
