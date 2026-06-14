use engine_core::{
    ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor, CommandEnvelope, Diagnostic,
};

use crate::{
    betting::{call_price, live_seats},
    ids::{RiverLedgerSeat, ACTION_BET, ACTION_CALL, ACTION_CHECK, ACTION_FOLD, ACTION_RAISE},
    state::{Phase, RiverLedgerState},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RiverLedgerAction {
    Fold,
    Check,
    Call,
    Bet,
    Raise,
}

impl RiverLedgerAction {
    pub const fn segment(self) -> &'static str {
        match self {
            Self::Fold => ACTION_FOLD,
            Self::Check => ACTION_CHECK,
            Self::Call => ACTION_CALL,
            Self::Bet => ACTION_BET,
            Self::Raise => ACTION_RAISE,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Fold => "Fold",
            Self::Check => "Check",
            Self::Call => "Call",
            Self::Bet => "Bet",
            Self::Raise => "Raise",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub actor: RiverLedgerSeat,
    pub action: RiverLedgerAction,
    pub required_to_call: u16,
    pub adds_to_pot: u16,
    pub street_unit: u8,
}

pub fn legal_action_tree(state: &RiverLedgerState, actor: &Actor) -> ActionTree {
    let Some(actor_seat) = actor_seat(state, actor) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    ActionTree::flat(
        state.freshness_token,
        legal_actions(state, actor_seat)
            .into_iter()
            .map(|action| action_choice(state, actor_seat, action))
            .collect(),
    )
}

pub fn legal_actions(state: &RiverLedgerState, actor: RiverLedgerSeat) -> Vec<RiverLedgerAction> {
    if !matches!(state.phase, Phase::Betting { .. })
        || state.terminal_outcome.is_some()
        || state.active_seat != Some(actor)
        || live_seats(state).len() < 2
    {
        return Vec::new();
    }

    let Some(price) = call_price(state, actor) else {
        return Vec::new();
    };

    if price > 0 {
        let mut actions = vec![RiverLedgerAction::Fold, RiverLedgerAction::Call];
        if !state.betting.raise_cap_reached() {
            actions.push(RiverLedgerAction::Raise);
        }
        return actions;
    }

    if state.betting.current_to_call == 0 {
        vec![RiverLedgerAction::Check, RiverLedgerAction::Bet]
    } else if state.betting.raise_cap_reached() {
        vec![RiverLedgerAction::Check]
    } else {
        vec![RiverLedgerAction::Check, RiverLedgerAction::Raise]
    }
}

pub fn actor_seat(state: &RiverLedgerState, actor: &Actor) -> Option<RiverLedgerSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(RiverLedgerSeat::from_index)
}

pub fn parse_action_segment(segment: &str) -> Option<RiverLedgerAction> {
    match segment {
        ACTION_FOLD => Some(RiverLedgerAction::Fold),
        ACTION_CHECK => Some(RiverLedgerAction::Check),
        ACTION_CALL => Some(RiverLedgerAction::Call),
        ACTION_BET => Some(RiverLedgerAction::Bet),
        ACTION_RAISE => Some(RiverLedgerAction::Raise),
        _ => None,
    }
}

pub fn validate_command(
    state: &RiverLedgerState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if command.freshness_token != state.freshness_token {
        return Err(stale_action_diagnostic());
    }

    let actor = actor_seat(state, &command.actor).ok_or_else(wrong_seat_diagnostic)?;
    if state.terminal_outcome.is_some() || !matches!(state.phase, Phase::Betting { .. }) {
        return Err(terminal_or_non_betting_diagnostic());
    }
    if state.active_seat != Some(actor) {
        return Err(wrong_seat_diagnostic());
    }

    let [segment] = command.action_path.segments.as_slice() else {
        return Err(malformed_action_diagnostic());
    };
    let action = parse_action_segment(segment).ok_or_else(malformed_action_diagnostic)?;

    if action == RiverLedgerAction::Raise && state.betting.raise_cap_reached() {
        return Err(raise_cap_diagnostic());
    }

    if !legal_actions(state, actor).contains(&action) {
        return Err(unavailable_action_diagnostic());
    }

    let required_to_call = call_price(state, actor).unwrap_or(0);
    let adds_to_pot = match action {
        RiverLedgerAction::Fold | RiverLedgerAction::Check => 0,
        RiverLedgerAction::Call => required_to_call,
        RiverLedgerAction::Bet => u16::from(state.betting.street.unit()),
        RiverLedgerAction::Raise => required_to_call + u16::from(state.betting.street.unit()),
    };

    Ok(ValidatedAction {
        actor,
        action,
        required_to_call,
        adds_to_pot,
        street_unit: state.betting.street.unit(),
    })
}

pub fn wrong_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_seat".to_owned(),
        message: "only the active River Ledger seat may choose a betting action".to_owned(),
    }
}

pub fn terminal_or_non_betting_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "action_not_available".to_owned(),
        message: "betting actions are not available in the current River Ledger phase".to_owned(),
    }
}

pub fn malformed_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "malformed_action".to_owned(),
        message: "River Ledger actions require exactly one recognized action segment".to_owned(),
    }
}

pub fn unavailable_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "action_unavailable".to_owned(),
        message: "the requested River Ledger action is not available at this decision point"
            .to_owned(),
    }
}

pub fn raise_cap_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "raise_cap_reached".to_owned(),
        message: "the street raise cap has been reached".to_owned(),
    }
}

pub fn stale_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "stale_action".to_owned(),
        message: "the action was submitted for an older River Ledger decision point".to_owned(),
    }
}

fn action_choice(
    state: &RiverLedgerState,
    actor: RiverLedgerSeat,
    action: RiverLedgerAction,
) -> ActionChoice {
    let required_to_call = call_price(state, actor).unwrap_or(0);
    let adds_to_pot = match action {
        RiverLedgerAction::Fold | RiverLedgerAction::Check => 0,
        RiverLedgerAction::Call => required_to_call,
        RiverLedgerAction::Bet => u16::from(state.betting.street.unit()),
        RiverLedgerAction::Raise => required_to_call + u16::from(state.betting.street.unit()),
    };
    let accessibility_copy = accessibility_copy(action, adds_to_pot, required_to_call);
    let mut choice = ActionChoice::leaf(action.segment(), action.label(), &accessibility_copy);
    choice.metadata = vec![
        metadata("action_family", action.segment()),
        metadata("street", state.betting.street.as_str()),
        metadata("street_unit", state.betting.street.unit().to_string()),
        metadata("actor_seat", actor.as_str()),
        metadata("required_to_call", required_to_call.to_string()),
        metadata("adds_to_pot", adds_to_pot.to_string()),
        metadata(
            "pot_after",
            state
                .ledger
                .pot_total
                .saturating_add(adds_to_pot)
                .to_string(),
        ),
        metadata(
            "raises_remaining",
            crate::ids::MAX_RAISES_PER_STREET
                .saturating_sub(state.betting.raises_this_street)
                .to_string(),
        ),
        metadata("accessibility_copy", accessibility_copy),
    ];
    choice.tags = vec!["betting".to_owned(), action.segment().to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn accessibility_copy(
    action: RiverLedgerAction,
    adds_to_pot: u16,
    required_to_call: u16,
) -> String {
    match action {
        RiverLedgerAction::Fold => "Fold this seat out of the hand".to_owned(),
        RiverLedgerAction::Check => "Check without adding contribution units".to_owned(),
        RiverLedgerAction::Call => format!("Call by adding {required_to_call} contribution units"),
        RiverLedgerAction::Bet => format!("Bet by adding {adds_to_pot} contribution units"),
        RiverLedgerAction::Raise => format!(
            "Raise by calling {required_to_call} and adding {} contribution units",
            adds_to_pot.saturating_sub(required_to_call)
        ),
    }
}

fn metadata(key: impl Into<String>, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.into(),
        value: value.into(),
    }
}
