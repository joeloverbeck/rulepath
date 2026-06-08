use ai_core::RandomLegalBot;
use engine_core::{ActionPath, ActionTree, Actor, Diagnostic, Seed};

use crate::{legal_action_tree, state::HighCardDuelState, CardId, HighCardDuelSeat};

pub const RANDOM_POLICY_ID: &str = "high_card_duel-random-legal-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HighCardDuelBotInput {
    pub bot_seat: HighCardDuelSeat,
    pub legal_action_tree: ActionTree,
    pub own_hand: Vec<CardId>,
    pub own_commitment: Option<CardId>,
}

impl HighCardDuelBotInput {
    pub fn stable_summary(&self) -> String {
        format!(
            "seat={};tree_choices={};own_hand={};own_commitment={}",
            self.bot_seat.as_str(),
            self.legal_action_tree.root.choices.len(),
            self.own_hand
                .iter()
                .map(|card| card.stable_id())
                .collect::<Vec<_>>()
                .join(","),
            self.own_commitment
                .map_or_else(|| "none".to_owned(), |card| card.stable_id())
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub policy_id: String,
    pub policy_version: u32,
    pub level: u8,
    pub action_path: ActionPath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HighCardDuelRandomBot {
    pub seed: Seed,
}

impl HighCardDuelRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(
        state: &HighCardDuelState,
        bot_seat: HighCardDuelSeat,
    ) -> HighCardDuelBotInput {
        HighCardDuelBotInput {
            bot_seat,
            legal_action_tree: legal_action_tree(state, &actor_for_seat(state, bot_seat)),
            own_hand: state.hand_for(bot_seat).to_vec(),
            own_commitment: state.commitment_for(bot_seat),
        }
    }

    pub fn select_action(
        &self,
        state: &HighCardDuelState,
        bot_seat: HighCardDuelSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        RandomLegalBot::new(self.seed).select_action(&input.legal_action_tree)
    }

    pub fn select_decision(
        &self,
        state: &HighCardDuelState,
        bot_seat: HighCardDuelSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        Ok(BotDecision {
            policy_id: RANDOM_POLICY_ID.to_owned(),
            policy_version: 1,
            level: 0,
            action_path,
        })
    }
}

pub fn actor_for_seat(state: &HighCardDuelState, seat: HighCardDuelSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}
