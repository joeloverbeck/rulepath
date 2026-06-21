use engine_core::{
    ActionChoice, ActionMetadata, ActionNode, ActionPreview, ActionTree, Actor, CommandEnvelope,
    Diagnostic,
};
use game_stdlib::trick_taking::follow_suit_indices;

use crate::{
    cards::CardId,
    ids::{VowTideSeat, ACTION_BID},
    rules,
    state::{BiddingState, Phase, VowTideState},
};

pub const ACTION_PLAY: &str = "play";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BidAction {
    pub value: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayAction {
    pub card: CardId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedBid {
    pub actor: VowTideSeat,
    pub value: u8,
    pub hand_index: u32,
    pub hand_size: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedPlay {
    pub actor: VowTideSeat,
    pub card: CardId,
    pub hand_index: u32,
    pub trick_index: u8,
}

pub fn legal_action_tree(state: &VowTideState, actor: &Actor) -> ActionTree {
    let Some(actor_seat) = actor_seat(state, actor) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if let Some(bidding) = state.bidding_state() {
        if bidding.active_seat != actor_seat {
            return ActionTree::flat(state.freshness_token, Vec::new());
        }
        return bidding_action_tree(state, bidding, actor_seat);
    }
    if state
        .playing_state()
        .is_some_and(|playing| playing.active_seat == actor_seat)
    {
        return play_action_tree(state, actor_seat);
    }

    ActionTree::flat(state.freshness_token, Vec::new())
}

fn bidding_action_tree(
    state: &VowTideState,
    bidding: &BiddingState,
    actor_seat: VowTideSeat,
) -> ActionTree {
    let legal_bids = legal_bids(state, actor_seat);
    if legal_bids.is_empty() {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let mut bid_choice = ActionChoice::leaf(ACTION_BID, "Bid", "Choose a Vow Tide bid");
    bid_choice.metadata = bid_metadata(state, bidding, actor_seat);
    bid_choice.tags = vec!["bid".to_owned(), "contract".to_owned()];
    bid_choice.preview = ActionPreview::Available;
    bid_choice.next = Some(Box::new(ActionNode {
        choices: legal_bids
            .into_iter()
            .map(|value| bid_value_choice(state, bidding, actor_seat, value))
            .collect(),
    }));

    ActionTree {
        root: ActionNode {
            choices: vec![bid_choice],
        },
        freshness_token: state.freshness_token,
    }
}

fn play_action_tree(state: &VowTideState, actor: VowTideSeat) -> ActionTree {
    let legal_cards = legal_cards(state, actor);
    if legal_cards.is_empty() {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let mut play_choice = ActionChoice::leaf(ACTION_PLAY, "Play", "Play a card");
    play_choice.metadata = play_metadata(state, actor);
    play_choice.tags = vec!["play".to_owned(), "card-choice".to_owned()];
    play_choice.preview = ActionPreview::Available;
    play_choice.next = Some(Box::new(ActionNode {
        choices: legal_cards
            .into_iter()
            .map(|card| card_choice(state, actor, card))
            .collect(),
    }));

    ActionTree {
        root: ActionNode {
            choices: vec![play_choice],
        },
        freshness_token: state.freshness_token,
    }
}

pub fn legal_bids(state: &VowTideState, actor: VowTideSeat) -> Vec<u8> {
    let Some(bidding) = state.bidding_state() else {
        return Vec::new();
    };
    if bidding.active_seat != actor || bidding.bid_for(actor).is_some() {
        return Vec::new();
    }
    let Some(hand_size) = state.current_hand_size() else {
        return Vec::new();
    };
    let hook_forbidden = hook_forbidden_bid(state, bidding, actor);

    (0..=hand_size)
        .filter(|bid| Some(*bid) != hook_forbidden)
        .collect()
}

pub fn hook_forbidden_bid(
    state: &VowTideState,
    bidding: &BiddingState,
    actor: VowTideSeat,
) -> Option<u8> {
    if actor != state.dealer {
        return None;
    }
    let hand_size = state.current_hand_size()?;
    let total = bidding.accepted_bid_total();
    if total <= hand_size {
        Some(hand_size - total)
    } else {
        None
    }
}

pub fn actor_seat(state: &VowTideState, actor: &Actor) -> Option<VowTideSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(VowTideSeat::from_index)
}

pub fn legal_cards(state: &VowTideState, actor: VowTideSeat) -> Vec<CardId> {
    let Some(playing) = state.playing_state() else {
        return Vec::new();
    };
    if playing.active_seat != actor {
        return Vec::new();
    }
    let hand = state.hand_for_internal(actor);
    let Some(led_suit) = playing
        .current_trick
        .plays
        .first()
        .map(|play| play.card.card().suit)
    else {
        return hand.to_vec();
    };
    follow_suit_indices(hand, led_suit, |card| card.card().suit)
        .into_iter()
        .map(|index| hand[index])
        .collect()
}

pub fn parse_bid_action_path(segments: &[String]) -> Result<BidAction, Diagnostic> {
    match segments {
        [family, value] if family == ACTION_BID => {
            let value = value
                .parse::<u8>()
                .map_err(|_| rules::bid_out_of_range_diagnostic())?;
            Ok(BidAction { value })
        }
        _ => Err(rules::bid_out_of_range_diagnostic()),
    }
}

pub fn parse_play_action_path(segments: &[String]) -> Result<PlayAction, Diagnostic> {
    match segments {
        [family, value] if family == ACTION_PLAY => {
            let card = CardId::parse(value).ok_or_else(rules::unknown_card_diagnostic)?;
            Ok(PlayAction { card })
        }
        _ => Err(rules::wrong_phase_diagnostic()),
    }
}

pub fn validate_bid_command(
    state: &VowTideState,
    envelope: &CommandEnvelope,
) -> Result<ValidatedBid, Diagnostic> {
    rules::validate_bid_command(state, envelope)
}

pub fn validate_play_command(
    state: &VowTideState,
    envelope: &CommandEnvelope,
) -> Result<ValidatedPlay, Diagnostic> {
    rules::validate_play_command(state, envelope)
}

fn bid_value_choice(
    state: &VowTideState,
    bidding: &BiddingState,
    actor: VowTideSeat,
    value: u8,
) -> ActionChoice {
    let mut choice =
        ActionChoice::leaf(value.to_string(), value.to_string(), format!("Bid {value}"));
    choice.metadata = bid_metadata(state, bidding, actor);
    choice
        .metadata
        .push(metadata("bid_value", value.to_string()));
    choice.tags = vec!["bid".to_owned(), "bid-value".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn bid_metadata(
    state: &VowTideState,
    bidding: &BiddingState,
    actor: VowTideSeat,
) -> Vec<ActionMetadata> {
    let mut values = vec![
        metadata("action_family", ACTION_BID),
        metadata(
            "hand_size",
            state.current_hand_size().unwrap_or_default().to_string(),
        ),
        metadata(
            "current_bid_total",
            bidding.accepted_bid_total().to_string(),
        ),
        metadata("actor_seat", actor.as_str()),
        metadata("is_dealer", (actor == state.dealer).to_string()),
    ];
    if let Some(forbidden) = hook_forbidden_bid(state, bidding, actor) {
        values.push(metadata("hook_forbidden_bid", forbidden.to_string()));
    }
    values
}

fn card_choice(state: &VowTideState, actor: VowTideSeat, card: CardId) -> ActionChoice {
    let label = card.card().public_label();
    let mut choice = ActionChoice::leaf(card.as_str(), label.clone(), format!("Play {label}"));
    choice.metadata = play_metadata(state, actor);
    choice.metadata.push(metadata("card_id", card.as_str()));
    choice
        .metadata
        .push(metadata("card_suit", card.card().suit.as_str()));
    choice
        .metadata
        .push(metadata("card_rank", card.card().rank.as_str()));
    choice.tags = vec![
        "play".to_owned(),
        "card".to_owned(),
        card.card().suit.as_str().to_owned(),
    ];
    choice.preview = ActionPreview::Available;
    choice
}

fn play_metadata(state: &VowTideState, actor: VowTideSeat) -> Vec<ActionMetadata> {
    let mut values = vec![
        metadata("action_family", ACTION_PLAY),
        metadata("actor_seat", actor.as_str()),
        metadata("trump_suit", state.trump_suit().as_str()),
    ];
    if let Some(playing) = state.playing_state() {
        values.push(metadata("trick_index", playing.trick_index.to_string()));
        values.push(metadata("leader", playing.leader.as_str()));
        if let Some(lead) = playing.current_trick.plays.first() {
            values.push(metadata("led_suit", lead.card.card().suit.as_str()));
        }
    }
    values
}

fn metadata(key: impl Into<String>, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.into(),
        value: value.into(),
    }
}

pub fn phase_is_bidding(state: &VowTideState) -> bool {
    matches!(state.phase, Phase::Bidding(_))
}
