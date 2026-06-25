use engine_core::{ActionChoice, ActionNode, ActionPreview, ActionTree, Actor, Diagnostic};

use crate::{
    effects::BlackglassPactEffect,
    ids::BlackglassSeat,
    partnerships::team_for_seat,
    rules::{legal_play_cards, ACTION_PLAY},
    setup::{complete_blind_nil_and_deal, BLIND_NIL_DEFICIT_THRESHOLD},
    state::{Bid, BlackglassPactState, BlindNilChoice, Phase},
};

pub const ACTION_BLIND_NIL: &str = "blind_nil";
pub const ACTION_BLIND_DECLARE: &str = "declare";
pub const ACTION_BLIND_DECLINE: &str = "decline";
pub const ACTION_BID: &str = "bid";
pub const ACTION_BID_NIL: &str = "nil";
pub const MIN_NUMERIC_BID: u8 = 1;
pub const MAX_NUMERIC_BID: u8 = 13;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BlindNilAction {
    pub choice: BlindNilChoice,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BidAction {
    pub bid: Bid,
}

pub fn legal_action_tree(state: &BlackglassPactState, actor: &Actor) -> ActionTree {
    let Some(actor_seat) = actor_seat(state, actor) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if state.active_blind_nil_seat() == Some(actor_seat) {
        return blind_nil_action_tree(state);
    }

    if active_bid_seat(state) == Some(actor_seat) {
        return bid_action_tree(state);
    }
    if active_play_seat(state) == Some(actor_seat) {
        return play_action_tree(state, actor_seat);
    }

    ActionTree::flat(state.freshness_token, Vec::new())
}

fn blind_nil_action_tree(state: &BlackglassPactState) -> ActionTree {
    let mut blind_choice =
        ActionChoice::leaf(ACTION_BLIND_NIL, "Blind nil", "Choose blind nil commitment");
    blind_choice.tags = vec!["blind-nil".to_owned(), "pre-deal".to_owned()];
    blind_choice.preview = ActionPreview::Available;
    blind_choice.next = Some(Box::new(ActionNode {
        choices: vec![
            blind_choice_leaf(
                ACTION_BLIND_DECLARE,
                "Declare",
                "Declare blind nil before the deal",
            ),
            blind_choice_leaf(
                ACTION_BLIND_DECLINE,
                "Decline",
                "Decline blind nil before the deal",
            ),
        ],
    }));

    ActionTree {
        root: ActionNode {
            choices: vec![blind_choice],
        },
        freshness_token: state.freshness_token,
    }
}

fn bid_action_tree(state: &BlackglassPactState) -> ActionTree {
    let mut bid_choice = ActionChoice::leaf(ACTION_BID, "Bid", "Choose a Blackglass Pact bid");
    bid_choice.tags = vec!["bid".to_owned(), "contract".to_owned()];
    bid_choice.preview = ActionPreview::Available;
    bid_choice.next = Some(Box::new(ActionNode {
        choices: legal_bid_values()
            .into_iter()
            .map(|bid| bid_choice_leaf(bid_segment(bid), bid_label(bid), bid_accessibility(bid)))
            .collect(),
    }));

    ActionTree {
        root: ActionNode {
            choices: vec![bid_choice],
        },
        freshness_token: state.freshness_token,
    }
}

fn play_action_tree(state: &BlackglassPactState, actor: BlackglassSeat) -> ActionTree {
    let legal_cards = legal_play_cards(state, actor);
    if legal_cards.is_empty() {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let mut play_choice = ActionChoice::leaf(ACTION_PLAY, "Play", "Play a card");
    play_choice.tags = vec!["play".to_owned(), "card-choice".to_owned()];
    play_choice.preview = ActionPreview::Available;
    play_choice.next = Some(Box::new(ActionNode {
        choices: legal_cards
            .into_iter()
            .map(|card| {
                let label = card.card().public_label();
                let mut choice =
                    ActionChoice::leaf(card.as_str(), label.clone(), format!("Play {label}"));
                choice.tags = vec!["play".to_owned(), "card".to_owned()];
                choice.preview = ActionPreview::Available;
                choice
            })
            .collect(),
    }));

    ActionTree {
        root: ActionNode {
            choices: vec![play_choice],
        },
        freshness_token: state.freshness_token,
    }
}

pub fn parse_blind_nil_action_path(segments: &[String]) -> Result<BlindNilAction, Diagnostic> {
    match segments {
        [family, leaf] if family == ACTION_BLIND_NIL && leaf == ACTION_BLIND_DECLARE => {
            Ok(BlindNilAction {
                choice: BlindNilChoice::Declared,
            })
        }
        [family, leaf] if family == ACTION_BLIND_NIL && leaf == ACTION_BLIND_DECLINE => {
            Ok(BlindNilAction {
                choice: BlindNilChoice::Declined,
            })
        }
        _ => Err(invalid_blind_nil_action_diagnostic()),
    }
}

pub fn parse_bid_action_path(segments: &[String]) -> Result<BidAction, Diagnostic> {
    match segments {
        [family, leaf] if family == ACTION_BID && leaf == ACTION_BID_NIL => {
            Ok(BidAction { bid: Bid::Nil })
        }
        [family, leaf] if family == ACTION_BID => {
            let value = leaf
                .parse::<u8>()
                .map_err(|_| bid_out_of_range_diagnostic())?;
            if !(MIN_NUMERIC_BID..=MAX_NUMERIC_BID).contains(&value) {
                return Err(bid_out_of_range_diagnostic());
            }
            Ok(BidAction {
                bid: Bid::Tricks(value),
            })
        }
        _ => Err(bid_out_of_range_diagnostic()),
    }
}

pub fn apply_blind_nil_action(
    state: &mut BlackglassPactState,
    actor: &Actor,
    action: BlindNilAction,
) -> Result<Vec<BlackglassPactEffect>, Diagnostic> {
    let actor_seat = actor_seat(state, actor).ok_or_else(wrong_actor_diagnostic)?;
    apply_blind_nil_choice(state, actor_seat, action.choice)
}

pub fn apply_bid_action(
    state: &mut BlackglassPactState,
    actor: &Actor,
    action: BidAction,
) -> Result<Vec<BlackglassPactEffect>, Diagnostic> {
    let actor_seat = actor_seat(state, actor).ok_or_else(wrong_actor_diagnostic)?;
    apply_bid_choice(state, actor_seat, action.bid)
}

pub fn apply_bid_choice(
    state: &mut BlackglassPactState,
    seat: BlackglassSeat,
    bid: Bid,
) -> Result<Vec<BlackglassPactEffect>, Diagnostic> {
    validate_bid_value(bid)?;
    if state.bid_for(seat).is_some() {
        return Err(bid_locked_diagnostic());
    }
    if active_bid_seat(state) != Some(seat) {
        return Err(wrong_bid_seat_diagnostic());
    }

    state.bids[seat.index()] = Some(bid);
    advance_bid_cursor(state)?;
    state.advance_freshness();
    Ok(vec![BlackglassPactEffect::BidAccepted {
        seat,
        team: team_for_seat(seat),
        bid,
    }])
}

pub fn apply_blind_nil_choice(
    state: &mut BlackglassPactState,
    seat: BlackglassSeat,
    choice: BlindNilChoice,
) -> Result<Vec<BlackglassPactEffect>, Diagnostic> {
    if state.active_blind_nil_seat() != Some(seat) {
        return Err(wrong_blind_nil_seat_diagnostic());
    }
    if state.blind_nil_choice_for(seat).is_some() {
        return Err(blind_nil_already_resolved_diagnostic());
    }

    state.blind_nil_choices[seat.index()] = Some(choice);
    let mut effects = match choice {
        BlindNilChoice::Declared => {
            state.bids[seat.index()] = Some(Bid::BlindNil);
            vec![BlackglassPactEffect::BlindNilDeclared {
                seat,
                team: team_for_seat(seat),
            }]
        }
        BlindNilChoice::Declined => vec![BlackglassPactEffect::BlindNilDeclined { seat }],
    };

    advance_blind_nil_cursor(state)?;
    if state.active_blind_nil_seat().is_none() {
        complete_blind_nil_and_deal(state)?;
        effects.extend(deal_effects(state));
    } else {
        state.advance_freshness();
    }

    Ok(effects)
}

pub fn opening_blind_nil_effect(state: &BlackglassPactState) -> Option<BlackglassPactEffect> {
    match &state.phase {
        Phase::BlindNilCommitment { pending, .. } if !pending.is_empty() => {
            Some(BlackglassPactEffect::BlindNilWindowOpened {
                pending: pending.clone(),
                threshold: BLIND_NIL_DEFICIT_THRESHOLD,
            })
        }
        _ => None,
    }
}

pub fn actor_seat(state: &BlackglassPactState, actor: &Actor) -> Option<BlackglassSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(BlackglassSeat::from_index)
}

pub fn active_bid_seat(state: &BlackglassPactState) -> Option<BlackglassSeat> {
    match state.phase {
        Phase::Bidding { next, .. } if state.bid_for(next).is_none() => Some(next),
        _ => None,
    }
}

pub fn active_play_seat(state: &BlackglassPactState) -> Option<BlackglassSeat> {
    match state.phase {
        Phase::PlayingTrick { next, .. } => Some(next),
        _ => None,
    }
}

fn advance_blind_nil_cursor(state: &mut BlackglassPactState) -> Result<(), Diagnostic> {
    match &mut state.phase {
        Phase::BlindNilCommitment {
            pending,
            next_index,
        } => {
            if *next_index >= pending.len() {
                return Err(blind_nil_already_resolved_diagnostic());
            }
            *next_index += 1;
            Ok(())
        }
        _ => Err(wrong_phase_diagnostic()),
    }
}

fn deal_effects(state: &BlackglassPactState) -> Vec<BlackglassPactEffect> {
    let next_bidder = match state.phase {
        Phase::Bidding { next, .. } => next,
        _ => state.dealer.next_clockwise(),
    };
    let mut effects = vec![BlackglassPactEffect::DealCompleted {
        dealer: state.dealer,
        hand_index: state.hand_index,
        counts: state
            .private_hands
            .iter()
            .map(|(seat, hand)| (*seat, hand.len()))
            .collect(),
        next_bidder,
    }];
    effects.extend(state.private_hands.iter().map(|(seat, cards)| {
        BlackglassPactEffect::PrivateHandReceived {
            seat: *seat,
            cards: cards.clone(),
        }
    }));
    effects
}

fn advance_bid_cursor(state: &mut BlackglassPactState) -> Result<(), Diagnostic> {
    let Phase::Bidding { next, .. } = state.phase else {
        return Err(wrong_phase_diagnostic());
    };

    let Some(next_seat) = next_unbid_non_blind_after(state, next) else {
        state.phase = Phase::PlayingTrick {
            leader: state.dealer.next_clockwise(),
            next: state.dealer.next_clockwise(),
            plays: Vec::new(),
            trick_index: 0,
        };
        return Ok(());
    };

    state.phase = Phase::Bidding {
        next: next_seat,
        accepted: state.bids,
    };
    Ok(())
}

fn next_unbid_non_blind_after(
    state: &BlackglassPactState,
    after: BlackglassSeat,
) -> Option<BlackglassSeat> {
    let mut seat = after.next_clockwise();
    for _ in 0..crate::ids::STANDARD_SEAT_COUNT {
        if state.bids[seat.index()].is_none() {
            return Some(seat);
        }
        seat = seat.next_clockwise();
    }
    None
}

fn legal_bid_values() -> Vec<Bid> {
    let mut bids = Vec::with_capacity(14);
    bids.push(Bid::Nil);
    bids.extend((MIN_NUMERIC_BID..=MAX_NUMERIC_BID).map(Bid::Tricks));
    bids
}

fn validate_bid_value(bid: Bid) -> Result<(), Diagnostic> {
    match bid {
        Bid::Nil => Ok(()),
        Bid::Tricks(value) if (MIN_NUMERIC_BID..=MAX_NUMERIC_BID).contains(&value) => Ok(()),
        Bid::Tricks(_) | Bid::BlindNil => Err(bid_out_of_range_diagnostic()),
    }
}

fn bid_segment(bid: Bid) -> String {
    match bid {
        Bid::Nil => ACTION_BID_NIL.to_owned(),
        Bid::Tricks(value) => value.to_string(),
        Bid::BlindNil => "blind_nil".to_owned(),
    }
}

fn bid_label(bid: Bid) -> String {
    match bid {
        Bid::Nil => "Nil".to_owned(),
        Bid::Tricks(value) => value.to_string(),
        Bid::BlindNil => "Blind nil".to_owned(),
    }
}

fn bid_accessibility(bid: Bid) -> String {
    match bid {
        Bid::Nil => "Bid nil".to_owned(),
        Bid::Tricks(value) => format!("Bid {value} tricks"),
        Bid::BlindNil => "Blind nil is fixed before bidding".to_owned(),
    }
}

fn bid_choice_leaf(segment: String, label: String, accessibility_label: String) -> ActionChoice {
    let mut choice = ActionChoice::leaf(segment, label, accessibility_label);
    choice.tags = vec!["bid".to_owned(), "contract".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn blind_choice_leaf(segment: &str, label: &str, accessibility_label: &str) -> ActionChoice {
    let mut choice = ActionChoice::leaf(segment, label, accessibility_label);
    choice.tags = vec!["blind-nil".to_owned(), "pre-deal".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn invalid_blind_nil_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_INVALID_BLIND_NIL_ACTION".to_owned(),
        message: "blind nil action must be blind_nil/declare or blind_nil/decline".to_owned(),
    }
}

fn wrong_actor_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_UNKNOWN_ACTOR".to_owned(),
        message: "actor does not map to a blackglass_pact seat".to_owned(),
    }
}

fn wrong_blind_nil_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_WRONG_BLIND_NIL_SEAT".to_owned(),
        message: "only the active eligible seat may resolve blind nil".to_owned(),
    }
}

fn wrong_bid_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_WRONG_BID_SEAT".to_owned(),
        message: "only the active non-blind bidding seat may bid".to_owned(),
    }
}

fn bid_out_of_range_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_BID_OUT_OF_RANGE".to_owned(),
        message: "bid action must be bid/nil or bid/1 through bid/13".to_owned(),
    }
}

fn bid_locked_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_BID_LOCKED".to_owned(),
        message: "accepted bids are immutable".to_owned(),
    }
}

fn blind_nil_already_resolved_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_BLIND_NIL_ALREADY_RESOLVED".to_owned(),
        message: "blind nil commitment has already been resolved for this seat".to_owned(),
    }
}

fn wrong_phase_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_WRONG_PHASE".to_owned(),
        message: "blind nil action is only legal during blind nil commitment".to_owned(),
    }
}
