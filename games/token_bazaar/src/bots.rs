use ai_core::RandomLegalBot;
use engine_core::{ActionPath, Actor, Diagnostic, Seed};

use crate::{
    actions::{legal_action_tree, parse_action_segment, TokenBazaarAction},
    ids::{ContractId, ResourceId, TokenBazaarSeat, TokenBazaarSlot},
    rules::legal_actions,
    state::{contract_spec, ResourceCounts, TokenBazaarState},
};

pub const RANDOM_POLICY_ID: &str = "token_bazaar-random-legal-v1";
pub const LEVEL1_POLICY_ID: &str = "token_bazaar_level1_v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub policy_id: String,
    pub policy_version: u32,
    pub level: u8,
    pub action_path: ActionPath,
    pub rationale: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenBazaarRandomBot {
    pub seed: Seed,
}

impl TokenBazaarRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &TokenBazaarState,
        bot_seat: TokenBazaarSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let tree = legal_action_tree(state, &actor_for_seat(state, bot_seat));
        RandomLegalBot::new(self.seed).select_action(&tree)
    }

    pub fn select_decision(
        &self,
        state: &TokenBazaarState,
        bot_seat: TokenBazaarSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        Ok(BotDecision {
            policy_id: RANDOM_POLICY_ID.to_owned(),
            policy_version: 1,
            level: 0,
            action_path,
            rationale: "Selected a seeded random legal Token Bazaar action.".to_owned(),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenBazaarLevel1Bot {
    pub seed: Seed,
}

impl TokenBazaarLevel1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &TokenBazaarState,
        bot_seat: TokenBazaarSeat,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }

    pub fn select_decision(
        &self,
        state: &TokenBazaarState,
        bot_seat: TokenBazaarSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let legal = legal_actions_for_seat(state, bot_seat);
        if legal.is_empty() {
            return Err(Diagnostic {
                code: "no_legal_actions".to_owned(),
                message: "no legal action is available".to_owned(),
            });
        }

        let (action, rationale) = choose_level1_action(state, &legal);
        Ok(BotDecision {
            policy_id: LEVEL1_POLICY_ID.to_owned(),
            policy_version: 1,
            level: 1,
            action_path: ActionPath {
                segments: vec![action.segment()],
            },
            rationale,
        })
    }
}

pub fn actor_for_seat(state: &TokenBazaarState, seat: TokenBazaarSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

fn legal_actions_for_seat(
    state: &TokenBazaarState,
    bot_seat: TokenBazaarSeat,
) -> Vec<TokenBazaarAction> {
    if state.active_seat != bot_seat {
        return Vec::new();
    }
    legal_actions(state)
}

fn choose_level1_action(
    state: &TokenBazaarState,
    legal: &[TokenBazaarAction],
) -> (TokenBazaarAction, String) {
    if let Some(action) = best_fulfill(state, legal) {
        return (
            action,
            "Fulfilled the highest-value affordable visible contract.".to_owned(),
        );
    }

    if let Some(target) = best_visible_target(state) {
        if let Some(action) = best_collect_toward(state, legal, target) {
            return (
                action,
                format!(
                    "Collected resources toward the visible {} contract.",
                    contract_spec(target).label
                ),
            );
        }
        if let Some(action) = best_exchange_toward(state, legal, target) {
            return (
                action,
                format!(
                    "Exchanged resources to reduce the public cost gap for {}.",
                    contract_spec(target).label
                ),
            );
        }
    }

    let action = legal
        .iter()
        .min_by_key(|action| action.segment())
        .copied()
        .expect("legal action list is non-empty");
    (
        action,
        "Chose the first legal action after equivalent public options were resolved deterministically."
            .to_owned(),
    )
}

fn best_fulfill(
    state: &TokenBazaarState,
    legal: &[TokenBazaarAction],
) -> Option<TokenBazaarAction> {
    legal
        .iter()
        .filter_map(|action| match action {
            TokenBazaarAction::Fulfill { slot } => {
                let contract = state.slot_contract(*slot)?;
                Some((
                    *action,
                    contract_spec(contract).points,
                    contract.as_str(),
                    slot.as_str(),
                ))
            }
            _ => None,
        })
        .max_by_key(|(_, points, contract_id, slot_id)| {
            (
                *points,
                std::cmp::Reverse(*contract_id),
                std::cmp::Reverse(*slot_id),
            )
        })
        .map(|(action, _, _, _)| action)
}

fn best_visible_target(state: &TokenBazaarState) -> Option<ContractId> {
    TokenBazaarSlot::ALL
        .iter()
        .filter_map(|slot| state.slot_contract(*slot))
        .max_by_key(|contract| {
            let spec = contract_spec(*contract);
            (
                spec.points,
                std::cmp::Reverse(deficit_total(
                    state.inventory_for(state.active_seat),
                    spec.cost,
                )),
                std::cmp::Reverse(contract.as_str()),
            )
        })
}

fn best_collect_toward(
    state: &TokenBazaarState,
    legal: &[TokenBazaarAction],
    target: ContractId,
) -> Option<TokenBazaarAction> {
    let target_cost = contract_spec(target).cost;
    legal
        .iter()
        .filter_map(|action| match action {
            TokenBazaarAction::Collect { bundle } => {
                let gain = crate::actions::collect_gain(*bundle);
                let improvement =
                    deficit_reduction(state.inventory_for(state.active_seat), target_cost, gain);
                (improvement > 0).then_some((*action, improvement, action.segment()))
            }
            _ => None,
        })
        .max_by_key(|(_, improvement, segment)| (*improvement, std::cmp::Reverse(segment.clone())))
        .map(|(action, _, _)| action)
}

fn best_exchange_toward(
    state: &TokenBazaarState,
    legal: &[TokenBazaarAction],
    target: ContractId,
) -> Option<TokenBazaarAction> {
    let target_cost = contract_spec(target).cost;
    legal
        .iter()
        .filter_map(|action| match action {
            TokenBazaarAction::Exchange { pay: _, take } => {
                let mut gain = ResourceCounts::default();
                gain.set(*take, 1);
                let improvement =
                    deficit_reduction(state.inventory_for(state.active_seat), target_cost, gain);
                (improvement > 0).then_some((*action, improvement, action.segment()))
            }
            _ => None,
        })
        .max_by_key(|(_, improvement, segment)| (*improvement, std::cmp::Reverse(segment.clone())))
        .map(|(action, _, _)| action)
}

fn deficit_total(inventory: ResourceCounts, cost: ResourceCounts) -> u8 {
    ResourceId::ALL
        .iter()
        .map(|resource| cost.get(*resource).saturating_sub(inventory.get(*resource)))
        .sum()
}

fn deficit_reduction(inventory: ResourceCounts, cost: ResourceCounts, gain: ResourceCounts) -> u8 {
    let before = deficit_total(inventory, cost);
    let after_inventory = ResourceCounts::new(
        inventory.amber.saturating_add(gain.amber),
        inventory.jade.saturating_add(gain.jade),
        inventory.iron.saturating_add(gain.iron),
    );
    before.saturating_sub(deficit_total(after_inventory, cost))
}

pub fn action_from_decision(decision: &BotDecision) -> Option<TokenBazaarAction> {
    let [segment] = decision.action_path.segments.as_slice() else {
        return None;
    };
    parse_action_segment(segment)
}
