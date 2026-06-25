//! Bot policies for Meldfall Ledger.
//!
//! L0 is a deterministic random-legal selector over viewer-authorized legal
//! action surfaces. L1 is intentionally not admitted until strategy evidence
//! lands.

use ai_core::RandomLegalBot;
use engine_core::{
    ActionChoice, ActionNode, ActionPath, ActionTree, Diagnostic, FreshnessToken, Seed, Viewer,
};

use crate::{
    actions::{
        action_choice, draw_action_tree, MeldfallAction, DISCARD_SEGMENT_PREFIX,
        FINISH_TURN_SEGMENT,
    },
    state::{MatchState, SeatIndex, TurnPhase},
    visibility::{project_action_tree_for_viewer, project_view, MeldfallView},
};

pub const L0_POLICY_ID: &str = "meldfall-ledger-l0-random-legal-v1";
pub const L1_POLICY_STATUS: &str = "not_admitted_pending_strategy_evidence";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldfallBotInput {
    pub bot_seat: SeatIndex,
    pub view: MeldfallView,
    pub legal_action_tree: ActionTree,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldfallBotDecision {
    pub policy_id: &'static str,
    pub bot_seat: SeatIndex,
    pub action_path: ActionPath,
    pub explanation: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MeldfallL0Bot {
    seed: Seed,
}

impl MeldfallL0Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &MatchState, bot_seat: SeatIndex) -> MeldfallBotInput {
        bot_input_for(state, bot_seat)
    }

    pub fn select_action(
        &self,
        state: &MatchState,
        bot_seat: SeatIndex,
    ) -> Result<ActionPath, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        RandomLegalBot::new(self.seed).select_action(&input.legal_action_tree)
    }

    pub fn select_decision(
        &self,
        state: &MatchState,
        bot_seat: SeatIndex,
    ) -> Result<MeldfallBotDecision, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        let action_path = RandomLegalBot::new(self.seed).select_action(&input.legal_action_tree)?;
        Ok(MeldfallBotDecision {
            policy_id: L0_POLICY_ID,
            bot_seat,
            action_path,
            explanation: format!(
                "Selected one seeded legal action from {} viewer-authorized choices.",
                legal_action_paths(&input.legal_action_tree).len()
            ),
        })
    }
}

pub fn legal_action_paths(tree: &ActionTree) -> Vec<ActionPath> {
    let mut paths = Vec::new();
    collect_paths(&tree.root, Vec::new(), &mut paths);
    paths
}

pub fn bot_input_for(state: &MatchState, bot_seat: SeatIndex) -> MeldfallBotInput {
    let viewer = Viewer {
        seat_id: Some(state.seats[bot_seat].clone()),
    };
    let tree = legal_action_tree_for_seat(state, bot_seat, FreshnessToken(0));
    MeldfallBotInput {
        bot_seat,
        view: project_view(state, &viewer),
        legal_action_tree: project_action_tree_for_viewer(&tree, state, &viewer),
    }
}

pub fn legal_action_tree_for_seat(
    state: &MatchState,
    bot_seat: SeatIndex,
    freshness_token: FreshnessToken,
) -> ActionTree {
    if bot_seat != state.round.active_seat_index {
        return ActionTree::flat(freshness_token, Vec::new());
    }
    match state.round.phase {
        TurnPhase::Draw => draw_action_tree(freshness_token, !state.round.stock.is_empty(), &[]),
        TurnPhase::Table => ActionTree::flat(
            freshness_token,
            vec![action_choice(MeldfallAction::FinishTurn)],
        ),
        TurnPhase::Discard => discard_action_tree(state, bot_seat, freshness_token),
        TurnPhase::RoundSettled | TurnPhase::MatchComplete => {
            ActionTree::flat(freshness_token, Vec::new())
        }
    }
}

pub fn parse_bot_action(path: &ActionPath) -> Result<MeldfallAction, Diagnostic> {
    let Some(segment) = path.segments.first() else {
        return Err(no_legal_actions());
    };
    if segment == crate::actions::DRAW_STOCK_SEGMENT {
        return Ok(MeldfallAction::DrawFromStock);
    }
    if segment == FINISH_TURN_SEGMENT {
        return Ok(MeldfallAction::FinishTurn);
    }
    if let Some(card_id) = segment.strip_prefix(&format!("{DISCARD_SEGMENT_PREFIX}-")) {
        let card = crate::cards::CardId::parse(card_id).ok_or_else(|| Diagnostic {
            code: "ML_BOT_UNKNOWN_CARD".to_owned(),
            message: "meldfall_ledger bot selected an unknown discard card".to_owned(),
        })?;
        return Ok(MeldfallAction::Discard { card });
    }
    Err(Diagnostic {
        code: "ML_BOT_UNKNOWN_ACTION".to_owned(),
        message: "meldfall_ledger bot selected an unknown action".to_owned(),
    })
}

fn discard_action_tree(
    state: &MatchState,
    bot_seat: SeatIndex,
    freshness_token: FreshnessToken,
) -> ActionTree {
    let choices = state.round.seats[bot_seat]
        .hand
        .iter()
        .copied()
        .map(|card| action_choice(MeldfallAction::Discard { card }))
        .collect::<Vec<ActionChoice>>();
    ActionTree::flat(freshness_token, choices)
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

fn no_legal_actions() -> Diagnostic {
    Diagnostic {
        code: "no_legal_actions".to_owned(),
        message: "no Meldfall Ledger legal action is available to the bot".to_owned(),
    }
}
