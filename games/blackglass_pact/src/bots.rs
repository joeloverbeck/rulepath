use engine_core::{ActionPath, ActionTree, DeterministicRng, Diagnostic, Seed, SeededRng};

use crate::{
    bidding::{
        ACTION_BID, ACTION_BID_NIL, ACTION_BLIND_DECLARE, ACTION_BLIND_DECLINE, ACTION_BLIND_NIL,
    },
    cards::{Rank, Suit},
    ids::BlackglassSeat,
    partnerships::{partner_for, team_for_seat},
    rules::ACTION_PLAY,
    state::BlackglassPactState,
    visibility::{seat_view, SeatView},
};

pub const L0_POLICY_ID: &str = "blackglass-pact-l0-random-legal-v1";
pub const L1_POLICY_ID: &str = "blackglass-pact-l1-bounded-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlackglassBotInput {
    pub seat: BlackglassSeat,
    pub view: SeatView,
    pub legal_action_tree: ActionTree,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub policy_id: &'static str,
    pub level: u8,
    pub action_path: ActionPath,
    pub explanation: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlackglassL0Bot {
    pub seed: Seed,
}

impl BlackglassL0Bot {
    pub const fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_decision(
        self,
        state: &BlackglassPactState,
        seat: BlackglassSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let input = bot_input_for(state, seat);
        let mut legal = legal_action_paths(&input.legal_action_tree);
        legal.sort_by(|left, right| left.segments.cmp(&right.segments));
        if legal.is_empty() {
            return Err(no_legal_action_diagnostic());
        }
        let mut rng = SeededRng::from_seed(self.seed);
        let index = rng
            .next_index(legal.len())
            .expect("non-empty legal action list");
        Ok(BotDecision {
            policy_id: L0_POLICY_ID,
            level: 0,
            action_path: legal[index].clone(),
            explanation: "Selected one seeded legal action from the authorized action tree."
                .to_owned(),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlackglassL1Bot;

impl BlackglassL1Bot {
    pub fn select_decision(
        self,
        state: &BlackglassPactState,
        seat: BlackglassSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let input = bot_input_for(state, seat);
        let mut legal = legal_action_paths(&input.legal_action_tree);
        legal.sort_by(|left, right| left.segments.cmp(&right.segments));
        if legal.is_empty() {
            return Err(no_legal_action_diagnostic());
        }
        let action_path = choose_l1_action(state, &input, &legal);
        Ok(BotDecision {
            policy_id: L1_POLICY_ID,
            level: 1,
            explanation: l1_explanation(&action_path),
            action_path,
        })
    }
}

pub fn bot_input_for(state: &BlackglassPactState, seat: BlackglassSeat) -> BlackglassBotInput {
    let actor = engine_core::Actor {
        seat_id: state.seats[seat.index()].clone(),
    };
    BlackglassBotInput {
        seat,
        view: seat_view(state, seat),
        legal_action_tree: crate::legal_action_tree(state, &actor),
    }
}

pub fn legal_action_paths(tree: &ActionTree) -> Vec<ActionPath> {
    tree.root
        .choices
        .iter()
        .filter_map(|choice| choice.next.as_ref().map(|next| (choice, next)))
        .flat_map(|(choice, next)| {
            next.choices.iter().map(|leaf| ActionPath {
                segments: vec![choice.segment.clone(), leaf.segment.clone()],
            })
        })
        .collect()
}

fn choose_l1_action(
    state: &BlackglassPactState,
    input: &BlackglassBotInput,
    legal: &[ActionPath],
) -> ActionPath {
    match legal
        .first()
        .and_then(|path| path.segments.first())
        .map(String::as_str)
    {
        Some(ACTION_BLIND_NIL) => choose_l1_blind(state, input, legal),
        Some(ACTION_BID) => choose_l1_bid(input, legal),
        Some(ACTION_PLAY) => choose_l1_play(input, legal),
        _ => legal[0].clone(),
    }
}

fn choose_l1_blind(
    state: &BlackglassPactState,
    input: &BlackglassBotInput,
    legal: &[ActionPath],
) -> ActionPath {
    let team = team_for_seat(input.seat);
    let deficit = state.team_scores[1 - team.index()] - state.team_scores[team.index()];
    let partner_declared = state
        .bid_for(partner_for(input.seat))
        .is_some_and(|bid| matches!(bid, crate::Bid::BlindNil));
    let wanted = if deficit >= 300
        || (deficit >= 200 && !partner_declared && state.team_scores[1 - team.index()] < 450)
    {
        ACTION_BLIND_DECLARE
    } else {
        ACTION_BLIND_DECLINE
    };
    legal
        .iter()
        .find(|path| path.segments.get(1).is_some_and(|leaf| leaf == wanted))
        .cloned()
        .unwrap_or_else(|| legal[0].clone())
}

fn choose_l1_bid(input: &BlackglassBotInput, legal: &[ActionPath]) -> ActionPath {
    if nil_risk_is_low(&input.view.own_hand) {
        if let Some(path) = legal.iter().find(|path| {
            path.segments
                .get(1)
                .is_some_and(|leaf| leaf == ACTION_BID_NIL)
        }) {
            return path.clone();
        }
    }
    let estimate = own_hand_trick_estimate(&input.view.own_hand).clamp(1, 13);
    legal
        .iter()
        .filter_map(|path| {
            path.segments
                .get(1)
                .and_then(|leaf| leaf.parse::<u8>().ok())
                .map(|bid| (bid.abs_diff(estimate), bid, path))
        })
        .min_by_key(|(distance, bid, _)| (*distance, *bid))
        .map(|(_, _, path)| path.clone())
        .unwrap_or_else(|| legal[0].clone())
}

fn choose_l1_play(_input: &BlackglassBotInput, legal: &[ActionPath]) -> ActionPath {
    legal
        .iter()
        .filter_map(|path| {
            path.segments
                .get(1)
                .and_then(|leaf| crate::CardId::parse(leaf))
                .map(|card| (card.card().rank.value(), card.index(), path))
        })
        .min_by_key(|(rank, index, _)| (*rank, *index))
        .map(|(_, _, path)| path.clone())
        .unwrap_or_else(|| legal[0].clone())
}

fn own_hand_trick_estimate(hand: &[crate::CardId]) -> u8 {
    let high_cards = hand
        .iter()
        .filter(|card| matches!(card.card().rank, Rank::Ace | Rank::King))
        .count() as u8;
    let spades = hand
        .iter()
        .filter(|card| card.card().suit == Suit::Spades)
        .count() as u8;
    high_cards.saturating_add(spades.saturating_sub(3))
}

fn nil_risk_is_low(hand: &[crate::CardId]) -> bool {
    !hand.iter().any(|card| {
        card.card().suit == Suit::Spades
            || matches!(card.card().rank, Rank::Ace | Rank::King | Rank::Queen)
    })
}

fn l1_explanation(action_path: &ActionPath) -> String {
    match action_path.segments.first().map(String::as_str) {
        Some(ACTION_BLIND_NIL) => {
            "Applied public-score blind-nil thresholds to a legal blind action.".to_owned()
        }
        Some(ACTION_BID) => {
            "Estimated an own-hand bounded bid and selected a legal bid leaf.".to_owned()
        }
        Some(ACTION_PLAY) => {
            "Selected a legal play using own authorized hand and public trick context.".to_owned()
        }
        _ => "Selected a legal action from the authorized action tree.".to_owned(),
    }
}

fn no_legal_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_BOT_NO_LEGAL_ACTIONS".to_owned(),
        message: "no legal Blackglass Pact action is available to the bot".to_owned(),
    }
}
