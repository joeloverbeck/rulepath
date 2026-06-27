use ai_core::RandomLegalBot;
use engine_core::{ActionNode, ActionPath, ActionTree, Actor, Diagnostic, Seed};

use crate::{
    actions::{legal_action_tree, parse_action_path, StarbridgeAction},
    state::StarbridgeState,
};

pub const L0_POLICY_ID: &str = "starbridge-crossing-l0-random-legal-v1";
pub const L1_POLICY_STATUS: &str = "not_admitted_pending_strategy_evidence";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StarbridgeL0Decision {
    pub policy_id: &'static str,
    pub seat_index: u8,
    pub action_path: ActionPath,
    pub explanation: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StarbridgeCrossingL0Bot {
    seed: Seed,
}

impl StarbridgeCrossingL0Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(&self, state: &StarbridgeState) -> Result<ActionPath, Diagnostic> {
        RandomLegalBot::new(self.seed).select_action(&legal_tree_for_active_seat(state))
    }

    pub fn select_decision(
        &self,
        state: &StarbridgeState,
    ) -> Result<StarbridgeL0Decision, Diagnostic> {
        let tree = legal_tree_for_active_seat(state);
        let action_path = RandomLegalBot::new(self.seed).select_action(&tree)?;
        Ok(StarbridgeL0Decision {
            policy_id: L0_POLICY_ID,
            seat_index: state.active_seat_index,
            action_path,
            explanation: format!(
                "Selected one seeded legal Starbridge action from {} public choices.",
                legal_action_paths(&tree).len()
            ),
        })
    }
}

pub fn parse_bot_action(path: &ActionPath) -> Result<StarbridgeAction, Diagnostic> {
    parse_action_path(&path.segments)
}

pub fn legal_action_paths(tree: &ActionTree) -> Vec<ActionPath> {
    let mut paths = Vec::new();
    collect_paths(&tree.root, Vec::new(), &mut paths);
    paths
}

fn legal_tree_for_active_seat(state: &StarbridgeState) -> ActionTree {
    let Some(seat) = state.seats.get(usize::from(state.active_seat_index)) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    legal_action_tree(
        state,
        &Actor {
            seat_id: seat.seat_id.clone(),
        },
    )
}

fn collect_paths(node: &ActionNode, prefix: Vec<String>, paths: &mut Vec<ActionPath>) {
    for choice in &node.choices {
        let mut next_prefix = prefix.clone();
        next_prefix.push(choice.segment.clone());
        if let Some(next) = &choice.next {
            collect_paths(next, next_prefix, paths);
        } else {
            paths.push(ActionPath {
                segments: next_prefix,
            });
        }
    }
}
