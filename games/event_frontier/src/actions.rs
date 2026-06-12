//! Legal action tree and action-path parsing for Event Frontier.

use engine_core::{
    ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor, CommandEnvelope, Diagnostic,
};

use crate::{
    ids::FactionId,
    state::{CardPhase, EventFrontierState, FirstChoice},
};

pub const ACTION_EVENT: &str = "event";
pub const ACTION_OPERATION: &str = "operation";
pub const ACTION_LIMITED_OPERATION: &str = "limited_operation";
pub const ACTION_PASS: &str = "pass";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventFrontierAction {
    Event,
    OperationPlaceholder,
    LimitedOperationPlaceholder,
    Pass,
}

impl EventFrontierAction {
    pub const fn as_choice_label(&self) -> &'static str {
        match self {
            Self::Event => "event",
            Self::OperationPlaceholder => "operation",
            Self::LimitedOperationPlaceholder => "limited_operation",
            Self::Pass => "pass",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChoicePosition {
    First,
    Second { first_choice: FirstChoice },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub action: EventFrontierAction,
    pub actor_faction: FactionId,
    pub position: ChoicePosition,
}

pub fn legal_action_tree(state: &EventFrontierState, actor: &Actor) -> ActionTree {
    let Some(actor_faction) = state.faction_for_seat(&actor.seat_id) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    let Some((choosing_faction, choices)) = choosing_menu(state) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if actor_faction != choosing_faction {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    ActionTree::flat(
        state.freshness_token,
        choices
            .into_iter()
            .map(|action| action_choice(state, choosing_faction, action))
            .collect(),
    )
}

pub fn legal_action_metadata(state: &EventFrontierState, actor: &Actor) -> Vec<ActionMetadata> {
    let Some(actor_faction) = state.faction_for_seat(&actor.seat_id) else {
        return vec![metadata("action_status", "not_seated")];
    };
    if state.terminal_outcome.is_some() || state.card_phase == CardPhase::Terminal {
        return vec![metadata("action_status", "terminal")];
    }
    let Some((choosing_faction, _)) = choosing_menu(state) else {
        return vec![
            metadata("action_status", "waiting"),
            metadata("actor_seat", actor.seat_id.0.clone()),
            metadata("actor_faction", actor_faction.as_str()),
            metadata("phase", state.card_phase.stable_summary()),
            metadata("reason", "card flow is resolving automatically"),
        ];
    };
    if actor_faction == choosing_faction {
        return vec![
            metadata("action_status", "available"),
            metadata("actor_seat", actor.seat_id.0.clone()),
            metadata("actor_faction", actor_faction.as_str()),
            metadata("phase", state.card_phase.stable_summary()),
            metadata("current_card", current_card_metadata(state)),
            metadata("next_public_card", next_public_card_metadata(state)),
        ];
    }
    vec![
        metadata("action_status", "waiting"),
        metadata("actor_seat", actor.seat_id.0.clone()),
        metadata("actor_faction", actor_faction.as_str()),
        metadata("waiting_for", choosing_faction.as_str()),
        metadata("phase", state.card_phase.stable_summary()),
        metadata("reason", "waiting for the eligible faction to choose"),
    ]
}

pub fn parse_action_path(segments: &[String]) -> Option<EventFrontierAction> {
    match segments {
        [single] => parse_single_segment(single),
        _ => None,
    }
}

pub fn validate_command(
    state: &EventFrontierState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if command.freshness_token != state.freshness_token {
        return Err(stale_action_diagnostic());
    }
    if state.terminal_outcome.is_some() || state.card_phase == CardPhase::Terminal {
        return Err(terminal_diagnostic());
    }
    let actor_faction = state
        .faction_for_seat(&command.actor.seat_id)
        .ok_or_else(wrong_actor_diagnostic)?;
    let action =
        parse_action_path(&command.action_path.segments).ok_or_else(malformed_action_diagnostic)?;
    let (choosing_faction, choices) = choosing_menu(state).ok_or_else(wrong_phase_diagnostic)?;
    if actor_faction != choosing_faction {
        return Err(wrong_faction_diagnostic());
    }
    if !choices.contains(&action) {
        return Err(unavailable_action_diagnostic());
    }

    Ok(ValidatedAction {
        action,
        actor_faction,
        position: choice_position(&state.card_phase).ok_or_else(wrong_phase_diagnostic)?,
    })
}

pub fn choosing_menu(state: &EventFrontierState) -> Option<(FactionId, Vec<EventFrontierAction>)> {
    match &state.card_phase {
        CardPhase::AwaitingFirstChoice { faction } => Some((
            *faction,
            vec![
                EventFrontierAction::Event,
                EventFrontierAction::OperationPlaceholder,
                EventFrontierAction::Pass,
            ],
        )),
        CardPhase::AwaitingSecondChoice {
            second_faction,
            first_choice,
            ..
        } => Some((*second_faction, second_choice_menu(*first_choice))),
        CardPhase::Reckoning | CardPhase::Terminal => None,
    }
}

pub fn second_choice_menu(first_choice: FirstChoice) -> Vec<EventFrontierAction> {
    match first_choice {
        FirstChoice::Event => vec![
            EventFrontierAction::OperationPlaceholder,
            EventFrontierAction::Pass,
        ],
        FirstChoice::Operation => vec![
            EventFrontierAction::Event,
            EventFrontierAction::LimitedOperationPlaceholder,
            EventFrontierAction::Pass,
        ],
        FirstChoice::Pass => vec![
            EventFrontierAction::Event,
            EventFrontierAction::OperationPlaceholder,
            EventFrontierAction::Pass,
        ],
    }
}

fn choice_position(phase: &CardPhase) -> Option<ChoicePosition> {
    match phase {
        CardPhase::AwaitingFirstChoice { .. } => Some(ChoicePosition::First),
        CardPhase::AwaitingSecondChoice { first_choice, .. } => Some(ChoicePosition::Second {
            first_choice: *first_choice,
        }),
        CardPhase::Reckoning | CardPhase::Terminal => None,
    }
}

fn parse_single_segment(segment: &str) -> Option<EventFrontierAction> {
    match segment {
        ACTION_EVENT => Some(EventFrontierAction::Event),
        ACTION_OPERATION => Some(EventFrontierAction::OperationPlaceholder),
        ACTION_LIMITED_OPERATION => Some(EventFrontierAction::LimitedOperationPlaceholder),
        ACTION_PASS => Some(EventFrontierAction::Pass),
        _ => None,
    }
}

fn action_choice(
    state: &EventFrontierState,
    faction: FactionId,
    action: EventFrontierAction,
) -> ActionChoice {
    let (segment, label, accessibility, tag) = match action {
        EventFrontierAction::Event => (
            ACTION_EVENT,
            "Event",
            "Resolve the current event card",
            "event",
        ),
        EventFrontierAction::OperationPlaceholder => (
            ACTION_OPERATION,
            "Operation",
            "Choose an operation path",
            "operation",
        ),
        EventFrontierAction::LimitedOperationPlaceholder => (
            ACTION_LIMITED_OPERATION,
            "Limited operation",
            "Choose a one-site limited operation path",
            "limited-operation",
        ),
        EventFrontierAction::Pass => (ACTION_PASS, "Pass", "Pass and gain one resource", "pass"),
    };
    let mut choice = ActionChoice::leaf(segment, label, accessibility);
    choice.preview = ActionPreview::Available;
    choice.tags.push(tag.to_owned());
    choice.metadata = vec![
        metadata("faction", faction.as_str()),
        metadata("choice", action.as_choice_label()),
        metadata("phase", state.card_phase.stable_summary()),
        metadata("current_card", current_card_metadata(state)),
        metadata("next_public_card", next_public_card_metadata(state)),
    ];
    choice
}

fn current_card_metadata(state: &EventFrontierState) -> String {
    state
        .deck
        .current
        .map(|card| card.as_str().to_owned())
        .unwrap_or_else(|| "none".to_owned())
}

fn next_public_card_metadata(state: &EventFrontierState) -> String {
    state
        .deck
        .next_public
        .map(|card| card.as_str().to_owned())
        .unwrap_or_else(|| "none".to_owned())
}

fn metadata(key: &str, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.to_owned(),
        value: value.into(),
    }
}

fn stale_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "stale_action".to_owned(),
        message: "that Event Frontier action was built for an older state".to_owned(),
    }
}

fn terminal_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "terminal".to_owned(),
        message: "Event Frontier is already terminal".to_owned(),
    }
}

fn wrong_actor_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_actor".to_owned(),
        message: "that seat is not seated in this Event Frontier match".to_owned(),
    }
}

fn wrong_phase_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_phase".to_owned(),
        message: "that Event Frontier action is not available in the current phase".to_owned(),
    }
}

fn wrong_faction_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_faction".to_owned(),
        message: "that faction is waiting for the eligible faction to choose".to_owned(),
    }
}

fn malformed_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "malformed_action".to_owned(),
        message: "that Event Frontier action path is malformed".to_owned(),
    }
}

fn unavailable_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "action_unavailable".to_owned(),
        message: "that Event Frontier choice is not available now".to_owned(),
    }
}
