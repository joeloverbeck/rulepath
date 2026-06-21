use ai_core::RandomLegalBot;
use engine_core::{ActionPath, ActionTree, Actor, Diagnostic, Seed, Viewer};
use game_stdlib::trick_taking::winning_play_index;

use crate::{
    actions::{legal_action_tree, ACTION_PLAY},
    cards::{CardId, Rank, Suit},
    ids::{VowTideSeat, ACTION_BID},
    state::TrickPlay,
    visibility::{project_view, PrivateView, PublicView},
};

pub const L0_POLICY_ID: &str = "vow-tide-random-legal-v0";
pub const L1_POLICY_ID: &str = "vow-tide-level1-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VowTideBotInput {
    pub bot_seat: VowTideSeat,
    pub legal_action_tree: ActionTree,
    pub view: PublicView,
    pub own_hand: Vec<CardId>,
}

impl VowTideBotInput {
    pub fn stable_summary(&self) -> String {
        format!(
            "seat={};choices={};view={};own_hand={}",
            self.bot_seat.as_str(),
            legal_action_paths(&self.legal_action_tree).len(),
            self.view.stable_summary(),
            self.own_hand
                .iter()
                .map(|card| card.as_str())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub policy_id: String,
    pub policy_version: u32,
    pub level: u8,
    pub action_path: ActionPath,
    pub rationale: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VowTideL0Bot {
    pub seed: Seed,
}

impl VowTideL0Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_decision(
        &self,
        state: &crate::state::VowTideState,
        bot_seat: VowTideSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let input = bot_input_for(state, bot_seat);
        let action_path = RandomLegalBot::new(self.seed).select_action(&input.legal_action_tree)?;
        Ok(BotDecision {
            policy_id: L0_POLICY_ID.to_owned(),
            policy_version: 0,
            level: 0,
            action_path,
            rationale: "Selected a seeded random legal Vow Tide action.".to_owned(),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VowTideL1Bot {
    pub seed: Seed,
}

impl VowTideL1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_decision(
        &self,
        state: &crate::state::VowTideState,
        bot_seat: VowTideSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let input = bot_input_for(state, bot_seat);
        let legal = legal_action_paths(&input.legal_action_tree);
        if legal.is_empty() {
            return Err(Diagnostic {
                code: "VT_BOT_NO_LEGAL_ACTIONS".to_owned(),
                message: "no Vow Tide legal action is available to the bot".to_owned(),
            });
        }
        let action_path = choose_l1_action(&input, &legal);
        Ok(BotDecision {
            policy_id: L1_POLICY_ID.to_owned(),
            policy_version: 1,
            level: 1,
            rationale: l1_rationale(&input, &action_path),
            action_path,
        })
    }
}

pub fn bot_input_for(state: &crate::state::VowTideState, bot_seat: VowTideSeat) -> VowTideBotInput {
    let actor = Actor {
        seat_id: state.seats[bot_seat.index()].clone(),
    };
    let viewer = Viewer {
        seat_id: Some(actor.seat_id.clone()),
    };
    let view = project_view(state, &viewer);
    let own_hand = match &view.private_view {
        PrivateView::Seat(private) if private.seat == bot_seat => private
            .own_hand
            .iter()
            .filter_map(|card| CardId::parse(&card.card_id))
            .collect(),
        _ => Vec::new(),
    };
    VowTideBotInput {
        bot_seat,
        legal_action_tree: legal_action_tree(state, &actor),
        view,
        own_hand,
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

fn choose_l1_action(input: &VowTideBotInput, legal: &[ActionPath]) -> ActionPath {
    match input.view.phase.as_str() {
        "bidding" => choose_l1_bid(input, legal),
        "playing_trick" => choose_l1_play(input, legal),
        _ => legal[0].clone(),
    }
}

fn choose_l1_bid(input: &VowTideBotInput, legal: &[ActionPath]) -> ActionPath {
    let estimate = own_hand_control_estimate(input);
    legal
        .iter()
        .filter(|path| {
            path.segments
                .first()
                .is_some_and(|segment| segment == ACTION_BID)
        })
        .min_by_key(|path| {
            let bid = path.segments[1].parse::<u8>().unwrap_or_default();
            (estimate.abs_diff(bid), bid)
        })
        .cloned()
        .unwrap_or_else(|| legal[0].clone())
}

fn choose_l1_play(input: &VowTideBotInput, legal: &[ActionPath]) -> ActionPath {
    let needed = contract_needed(input);
    let mut cards = legal
        .iter()
        .filter(|path| {
            path.segments
                .first()
                .is_some_and(|segment| segment == ACTION_PLAY)
        })
        .filter_map(|path| CardId::parse(&path.segments[1]).map(|card| (path, card)))
        .collect::<Vec<_>>();
    if cards.is_empty() {
        return legal[0].clone();
    }
    let card_sort_key = |card: CardId| (card.card().rank.value(), card.index());
    if needed > 0 {
        let mut winning_cards = cards
            .iter()
            .filter(|(_, card)| card_is_currently_winning(input, *card))
            .collect::<Vec<_>>();
        winning_cards.sort_by_key(|(_, card)| card_sort_key(*card));
        winning_cards
            .first()
            .map(|(path, _)| (*path).clone())
            .unwrap_or_else(|| {
                cards.sort_by_key(|(_, card)| card_sort_key(*card));
                cards.last().expect("non-empty cards").0.clone()
            })
    } else {
        let mut losing_cards = cards
            .iter()
            .filter(|(_, card)| !card_is_currently_winning(input, *card))
            .collect::<Vec<_>>();
        losing_cards.sort_by_key(|(_, card)| card_sort_key(*card));
        losing_cards
            .first()
            .map(|(path, _)| (*path).clone())
            .unwrap_or_else(|| {
                cards.sort_by_key(|(_, card)| card_sort_key(*card));
                cards.first().expect("non-empty cards").0.clone()
            })
    }
}

fn card_is_currently_winning(input: &VowTideBotInput, candidate: CardId) -> bool {
    let mut plays = input
        .view
        .current_trick
        .iter()
        .filter_map(|played| {
            CardId::parse(&played.card.card_id).map(|card| TrickPlay {
                seat: played.seat,
                card,
            })
        })
        .collect::<Vec<_>>();
    plays.push(TrickPlay {
        seat: input.bot_seat,
        card: candidate,
    });
    let led_suit = match plays.first() {
        Some(play) => play.card.card().suit,
        None => return true,
    };
    let trump = suit_from_public_value(&input.view.trump_indicator.suit);
    winning_play_index(
        &plays,
        led_suit,
        trump,
        |play| play.card.card().suit,
        |play| play.card.card().rank,
    )
    .is_some_and(|index| plays[index].seat == input.bot_seat)
}

fn suit_from_public_value(value: &str) -> Option<Suit> {
    Suit::ALL.into_iter().find(|suit| suit.as_str() == value)
}

fn own_hand_control_estimate(input: &VowTideBotInput) -> u8 {
    let trump = input.view.trump_indicator.suit.as_str();
    let controls = input
        .own_hand
        .iter()
        .filter(|card| {
            let card = card.card();
            card.rank == Rank::Ace || card.suit.as_str() == trump && card.rank.value() >= 11
        })
        .count() as u8;
    controls.min(input.view.hand_size)
}

fn contract_needed(input: &VowTideBotInput) -> u8 {
    let bid = input
        .view
        .public_bids
        .iter()
        .find(|(seat, _)| *seat == input.bot_seat)
        .and_then(|(_, bid)| *bid)
        .unwrap_or_default();
    let taken = input
        .view
        .trick_counts
        .iter()
        .find(|(seat, _)| *seat == input.bot_seat)
        .map(|(_, count)| *count)
        .unwrap_or_default();
    bid.saturating_sub(taken)
}

fn l1_rationale(input: &VowTideBotInput, path: &ActionPath) -> String {
    match path.segments.as_slice() {
        [family, value] if family == ACTION_BID => {
            format!("Estimated contract from own hand controls; chose legal bid {value}.")
        }
        [family, value] if family == ACTION_PLAY => {
            let needed = contract_needed(input);
            format!("Contract needs {needed} more tricks; chose legal card {value}.")
        }
        _ => "Chose the first legal Vow Tide action.".to_owned(),
    }
}
