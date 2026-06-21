use engine_core::{DeterministicRng, Diagnostic, Seed, SeededRng};

use crate::{
    actions::{PassAction, PlayAction},
    cards::CardId,
    ids::BriarCircuitSeat,
    rules::legal_play_cards,
    state::BriarCircuitState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BriarCircuitBotAction {
    Pass(PassAction),
    Play(PlayAction),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BriarCircuitBotDecision {
    pub seat: BriarCircuitSeat,
    pub action: BriarCircuitBotAction,
    pub action_path: Vec<String>,
    pub explanation: String,
}

#[derive(Clone, Debug)]
pub struct BriarCircuitL0Bot {
    rng: SeededRng,
}

impl BriarCircuitL0Bot {
    pub fn new(seed: Seed) -> Self {
        Self {
            rng: SeededRng::from_seed(seed),
        }
    }

    pub fn select_decision(
        &mut self,
        state: &BriarCircuitState,
        seat: BriarCircuitSeat,
    ) -> Result<BriarCircuitBotDecision, Diagnostic> {
        let legal = legal_bot_actions(state, seat)?;
        let index = self
            .rng
            .next_index(legal.len())
            .ok_or_else(no_legal_action)?;
        Ok(decision(
            seat,
            legal[index],
            format!("Random legal choice from {} actions.", legal.len()),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct BriarCircuitL1Bot {
    _seed: Seed,
}

impl BriarCircuitL1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { _seed: seed }
    }

    pub fn select_decision(
        &self,
        state: &BriarCircuitState,
        seat: BriarCircuitSeat,
    ) -> Result<BriarCircuitBotDecision, Diagnostic> {
        let legal = legal_bot_actions(state, seat)?;
        let action = legal
            .into_iter()
            .min_by_key(|action| l1_priority(state, seat, *action))
            .ok_or_else(no_legal_action)?;
        Ok(decision(
            seat,
            action,
            "Selected a bounded legal action from public state and own hand.".to_owned(),
        ))
    }
}

pub fn legal_bot_actions(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
) -> Result<Vec<BriarCircuitBotAction>, Diagnostic> {
    if let Some(pass) = state.pass_state() {
        if pass.is_committed(seat) {
            return Ok(Vec::new());
        }
        let selected = pass.selection_for(seat);
        if selected.len() == crate::STANDARD_PASS_SIZE as usize {
            return Ok(vec![BriarCircuitBotAction::Pass(PassAction::Confirm)]);
        }
        return Ok(state
            .hand_for_internal(seat)
            .iter()
            .copied()
            .filter(|card| !selected.contains(card))
            .map(|card| BriarCircuitBotAction::Pass(PassAction::Select(card)))
            .collect());
    }

    legal_play_cards(state, seat).map(|cards| {
        cards
            .into_iter()
            .map(|card| BriarCircuitBotAction::Play(PlayAction::Play(card)))
            .collect()
    })
}

fn l1_priority(
    state: &BriarCircuitState,
    _seat: BriarCircuitSeat,
    action: BriarCircuitBotAction,
) -> (u8, u8, u8) {
    match action {
        BriarCircuitBotAction::Pass(PassAction::Select(card)) => {
            let card = card.card();
            let point_pressure = card.point_value();
            (0, u8::MAX - point_pressure, u8::MAX - card.rank.value())
        }
        BriarCircuitBotAction::Pass(PassAction::Confirm) => (1, 0, 0),
        BriarCircuitBotAction::Pass(PassAction::Unselect(_)) => (2, 0, 0),
        BriarCircuitBotAction::Play(PlayAction::Play(card)) => {
            let detail = card.card();
            let point_value = detail.point_value();
            let rank = detail.rank.value();
            let led_suit = state
                .playing_state()
                .and_then(|play| play.current_trick.plays.first())
                .map(|first| first.card.card().suit);
            match led_suit {
                // Leading a trick, or following the led suit: keep control low by
                // playing the lowest-penalty, lowest-ranked legal card (duck).
                None => (0, point_value, rank),
                Some(suit) if detail.suit == suit => (0, point_value, rank),
                // Void in the led suit: an off-suit card can never win this trick, so
                // shed the most dangerous holding first (queen of spades, then high
                // hearts, then high cards) rather than hoarding penalties for later.
                // This is own-hand danger management only; it reads no opponent state.
                Some(_) => (0, u8::MAX - point_value, u8::MAX - rank),
            }
        }
    }
}

fn decision(
    seat: BriarCircuitSeat,
    action: BriarCircuitBotAction,
    explanation: String,
) -> BriarCircuitBotDecision {
    BriarCircuitBotDecision {
        seat,
        action,
        action_path: action_path(action),
        explanation,
    }
}

fn action_path(action: BriarCircuitBotAction) -> Vec<String> {
    match action {
        BriarCircuitBotAction::Pass(PassAction::Select(card)) => {
            vec!["pass".to_owned(), "select".to_owned(), card.as_str()]
        }
        BriarCircuitBotAction::Pass(PassAction::Unselect(card)) => {
            vec!["pass".to_owned(), "unselect".to_owned(), card.as_str()]
        }
        BriarCircuitBotAction::Pass(PassAction::Confirm) => {
            vec!["pass".to_owned(), "confirm".to_owned()]
        }
        BriarCircuitBotAction::Play(PlayAction::Play(card)) => {
            vec!["play".to_owned(), card.as_str()]
        }
    }
}

fn no_legal_action() -> Diagnostic {
    Diagnostic {
        code: "BC_NO_LEGAL_ACTION".to_owned(),
        message: "bot had no legal Briar Circuit action".to_owned(),
    }
}

#[allow(dead_code)]
fn _card_id_type_is_game_local(_: CardId) {}
