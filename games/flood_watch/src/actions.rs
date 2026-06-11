//! Legal action tree and action-path parsing for Flood Watch.

use engine_core::{
    ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor, CommandEnvelope, Diagnostic,
};

use crate::{
    ids::{DistrictId, FloodWatchRole},
    state::{FloodWatchState, Phase},
};

pub const ACTION_BAIL: &str = "bail";
pub const ACTION_REINFORCE: &str = "reinforce";
pub const ACTION_FORECAST: &str = "forecast";
pub const ACTION_END_TURN: &str = "end_turn";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FloodWatchAction {
    Bail(DistrictId),
    Reinforce(DistrictId),
    Forecast,
    EndTurn,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub action: FloodWatchAction,
    pub actor_index: usize,
    pub budget_remaining: u8,
}

pub fn legal_action_tree(state: &FloodWatchState, actor: &Actor) -> ActionTree {
    if state.terminal_outcome.is_some() || state.phase == Phase::Terminal {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }
    let Some(actor_index) = state.seat_index(&actor.seat_id) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if state.seats[actor_index] != state.active_seat {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }
    let Phase::Action { budget_remaining } = state.phase else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if budget_remaining == 0 {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    ActionTree::flat(
        state.freshness_token,
        legal_action_choices(state, actor_index, budget_remaining),
    )
}

pub fn legal_action_metadata(state: &FloodWatchState, actor: &Actor) -> Vec<ActionMetadata> {
    let Some(actor_index) = state.seat_index(&actor.seat_id) else {
        return vec![metadata("action_status", "not_seated")];
    };
    if state.terminal_outcome.is_some() || state.phase == Phase::Terminal {
        return vec![metadata("action_status", "terminal")];
    }
    if state.seats[actor_index] == state.active_seat {
        return vec![
            metadata("action_status", "available"),
            metadata("actor_seat", actor.seat_id.0.clone()),
            metadata("phase", "action"),
        ];
    }
    vec![
        metadata("action_status", "waiting"),
        metadata("actor_seat", actor.seat_id.0.clone()),
        metadata("waiting_for", state.active_seat.0.clone()),
        metadata(
            "reason",
            "waiting for the active teammate to spend the action budget",
        ),
    ]
}

pub fn parse_action_path(segments: &[String]) -> Option<FloodWatchAction> {
    match segments {
        [single] => parse_single_segment(single),
        [family, district] if family == ACTION_BAIL => {
            DistrictId::parse(district).map(FloodWatchAction::Bail)
        }
        [family, district] if family == ACTION_REINFORCE => {
            DistrictId::parse(district).map(FloodWatchAction::Reinforce)
        }
        _ => None,
    }
}

pub fn validate_command(
    state: &FloodWatchState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if command.freshness_token != state.freshness_token {
        return Err(stale_action_diagnostic());
    }
    if state.terminal_outcome.is_some() || state.phase == Phase::Terminal {
        return Err(terminal_diagnostic());
    }
    let actor_index = state
        .seat_index(&command.actor.seat_id)
        .ok_or_else(wrong_actor_diagnostic)?;
    if state.seats[actor_index] != state.active_seat {
        return Err(wrong_seat_diagnostic());
    }
    let Phase::Action { budget_remaining } = state.phase else {
        return Err(wrong_phase_diagnostic());
    };
    if budget_remaining == 0 {
        return Err(out_of_budget_diagnostic());
    }
    let action =
        parse_action_path(&command.action_path.segments).ok_or_else(malformed_action_diagnostic)?;
    validate_action_available(state, &action)?;

    Ok(ValidatedAction {
        action,
        actor_index,
        budget_remaining,
    })
}

pub fn role_bail_amount(role: FloodWatchRole) -> u8 {
    match role {
        FloodWatchRole::Pumpwright => 2,
        FloodWatchRole::LeveeWarden => 1,
    }
}

pub fn role_reinforce_amount(role: FloodWatchRole) -> u8 {
    match role {
        FloodWatchRole::Pumpwright => 1,
        FloodWatchRole::LeveeWarden => 2,
    }
}

pub fn action_segment(action: FloodWatchAction) -> String {
    match action {
        FloodWatchAction::Bail(district) => format!("{ACTION_BAIL}/{}", district.as_str()),
        FloodWatchAction::Reinforce(district) => {
            format!("{ACTION_REINFORCE}/{}", district.as_str())
        }
        FloodWatchAction::Forecast => ACTION_FORECAST.to_owned(),
        FloodWatchAction::EndTurn => ACTION_END_TURN.to_owned(),
    }
}

fn legal_action_choices(
    state: &FloodWatchState,
    actor_index: usize,
    budget_remaining: u8,
) -> Vec<ActionChoice> {
    let role = state.roles[actor_index];
    let mut choices = Vec::new();

    for district in &state.districts {
        if district.flood_level > 0 {
            choices.push(district_choice(
                FloodWatchAction::Bail(district.district),
                role_bail_amount(role),
                budget_remaining,
                district.district.label(),
                "bail",
                district.flood_level,
                district.levees,
            ));
        }
    }
    for district in &state.districts {
        if district.levees < state.variant.levee_cap {
            choices.push(district_choice(
                FloodWatchAction::Reinforce(district.district),
                role_reinforce_amount(role),
                budget_remaining,
                district.district.label(),
                "reinforce",
                district.flood_level,
                district.levees,
            ));
        }
    }
    if state.forecast.is_none() && state.top_undrawn_card().is_some() {
        choices.push(simple_choice(
            FloodWatchAction::Forecast,
            "Forecast",
            "Reveal the next storm card to both teammates",
            budget_remaining,
            "forecast",
        ));
    }
    choices.push(simple_choice(
        FloodWatchAction::EndTurn,
        "End turn",
        "End the turn and let the storm resolve",
        budget_remaining,
        "end-turn",
    ));

    choices
}

fn validate_action_available(
    state: &FloodWatchState,
    action: &FloodWatchAction,
) -> Result<(), Diagnostic> {
    match action {
        FloodWatchAction::Bail(district) => {
            let district = state
                .district(*district)
                .ok_or_else(unavailable_action_diagnostic)?;
            if district.flood_level == 0 {
                return Err(dry_bail_diagnostic());
            }
        }
        FloodWatchAction::Reinforce(district) => {
            let district = state
                .district(*district)
                .ok_or_else(unavailable_action_diagnostic)?;
            if district.levees >= state.variant.levee_cap {
                return Err(levee_cap_diagnostic());
            }
        }
        FloodWatchAction::Forecast => {
            if state.forecast.is_some() {
                return Err(forecast_unavailable_diagnostic());
            }
            if state.top_undrawn_card().is_none() {
                return Err(forecast_unavailable_diagnostic());
            }
        }
        FloodWatchAction::EndTurn => {}
    }
    Ok(())
}

fn parse_single_segment(segment: &str) -> Option<FloodWatchAction> {
    if segment == ACTION_FORECAST {
        return Some(FloodWatchAction::Forecast);
    }
    if segment == ACTION_END_TURN {
        return Some(FloodWatchAction::EndTurn);
    }
    let (family, district) = segment.split_once('/')?;
    match family {
        ACTION_BAIL => DistrictId::parse(district).map(FloodWatchAction::Bail),
        ACTION_REINFORCE => DistrictId::parse(district).map(FloodWatchAction::Reinforce),
        _ => None,
    }
}

fn district_choice(
    action: FloodWatchAction,
    role_amount: u8,
    budget_remaining: u8,
    district_label: &str,
    verb: &str,
    flood_level: u8,
    levees: u8,
) -> ActionChoice {
    let label = match verb {
        "bail" => format!("Bail {district_label}"),
        "reinforce" => format!("Reinforce {district_label}"),
        _ => district_label.to_owned(),
    };
    let accessibility_label = format!("{label}; remaining budget {budget_remaining}");
    let mut choice = ActionChoice::leaf(
        action_segment(action),
        label.clone(),
        accessibility_label.clone(),
    );
    choice.metadata = vec![
        metadata("action_family", verb),
        metadata("remaining_budget", budget_remaining.to_string()),
        metadata("role_amount", role_amount.to_string()),
        metadata("district_label", district_label),
        metadata("flood_level", flood_level.to_string()),
        metadata("levees", levees.to_string()),
        metadata("accessibility_copy", accessibility_label),
    ];
    choice.tags = vec![verb.to_owned(), "budgeted".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn simple_choice(
    action: FloodWatchAction,
    label: &str,
    accessibility_label: &str,
    budget_remaining: u8,
    tag: &str,
) -> ActionChoice {
    let mut choice = ActionChoice::leaf(action_segment(action), label, accessibility_label);
    choice.metadata = vec![
        metadata("action_family", tag),
        metadata("remaining_budget", budget_remaining.to_string()),
    ];
    choice.tags = vec![tag.to_owned(), "budgeted".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

pub fn metadata(key: impl Into<String>, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.into(),
        value: value.into(),
    }
}

pub fn wrong_actor_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_actor".to_owned(),
        message: "only a seated Flood Watch actor may submit an action".to_owned(),
    }
}

pub fn wrong_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_seat".to_owned(),
        message: "only the active teammate may act right now".to_owned(),
    }
}

pub fn wrong_phase_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_phase".to_owned(),
        message: "that action is not available in the current phase".to_owned(),
    }
}

pub fn terminal_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "terminal_state".to_owned(),
        message: "actions cannot be submitted after the match is complete".to_owned(),
    }
}

pub fn stale_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "stale_action".to_owned(),
        message: "the action was submitted for an older decision point".to_owned(),
    }
}

pub fn malformed_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "malformed_action".to_owned(),
        message: "Flood Watch actions must be bail, reinforce, forecast, or end_turn".to_owned(),
    }
}

pub fn unavailable_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "action_unavailable".to_owned(),
        message: "that Flood Watch action is not available now".to_owned(),
    }
}

pub fn dry_bail_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "dry_bail".to_owned(),
        message: "that district has no flood water to bail".to_owned(),
    }
}

pub fn levee_cap_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "levee_cap".to_owned(),
        message: "that district's levees are already at the scenario cap".to_owned(),
    }
}

pub fn forecast_unavailable_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "forecast_unavailable".to_owned(),
        message: "the next event is already forecast or no event remains".to_owned(),
    }
}

pub fn out_of_budget_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "out_of_budget".to_owned(),
        message: "the active teammate has no action budget remaining".to_owned(),
    }
}
