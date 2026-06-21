use engine_core::{
    ActionChoice, ActionMetadata, ActionNode, ActionPreview, ActionTree, Actor, CommandEnvelope,
    Diagnostic,
};

use crate::{
    ids::{VowTideSeat, ACTION_BID},
    rules,
    state::{BiddingState, Phase, VowTideState},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BidAction {
    pub value: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedBid {
    pub actor: VowTideSeat,
    pub value: u8,
    pub hand_index: u32,
    pub hand_size: u8,
}

pub fn legal_action_tree(state: &VowTideState, actor: &Actor) -> ActionTree {
    let Some(actor_seat) = actor_seat(state, actor) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    let Some(bidding) = state.bidding_state() else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if bidding.active_seat != actor_seat {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

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

pub fn validate_bid_command(
    state: &VowTideState,
    envelope: &CommandEnvelope,
) -> Result<ValidatedBid, Diagnostic> {
    rules::validate_bid_command(state, envelope)
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

fn metadata(key: impl Into<String>, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.into(),
        value: value.into(),
    }
}

pub fn phase_is_bidding(state: &VowTideState) -> bool {
    matches!(state.phase, Phase::Bidding(_))
}
