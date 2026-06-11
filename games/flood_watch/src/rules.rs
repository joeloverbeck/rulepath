//! Rule application for Flood Watch's budgeted action phase.

use engine_core::Diagnostic;

use crate::{
    actions::{
        role_bail_amount, role_reinforce_amount, validate_command, FloodWatchAction,
        ValidatedAction,
    },
    state::{FloodWatchState, Phase},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppliedAction {
    pub environment_pending: bool,
}

pub fn apply_validated_action(
    state: &mut FloodWatchState,
    validated: ValidatedAction,
) -> Result<AppliedAction, Diagnostic> {
    match validated.action {
        FloodWatchAction::Bail(district) => {
            let role = state.roles[validated.actor_index];
            let amount = role_bail_amount(role);
            let district_state = state.district_mut(district).ok_or_else(|| Diagnostic {
                code: "action_unavailable".to_owned(),
                message: "that Flood Watch action is not available now".to_owned(),
            })?;
            district_state.flood_level = district_state.flood_level.saturating_sub(amount);
            spend_budget(state)
        }
        FloodWatchAction::Reinforce(district) => {
            let role = state.roles[validated.actor_index];
            let amount = role_reinforce_amount(role);
            let cap = state.variant.levee_cap;
            let district_state = state.district_mut(district).ok_or_else(|| Diagnostic {
                code: "action_unavailable".to_owned(),
                message: "that Flood Watch action is not available now".to_owned(),
            })?;
            district_state.levees = district_state.levees.saturating_add(amount).min(cap);
            spend_budget(state)
        }
        FloodWatchAction::Forecast => {
            state.forecast = state.top_undrawn_card().cloned();
            spend_budget(state)
        }
        FloodWatchAction::EndTurn => {
            state.phase = Phase::Action {
                budget_remaining: 0,
            };
            state.freshness_token = state.freshness_token.next();
            Ok(AppliedAction {
                environment_pending: true,
            })
        }
    }
}

pub fn apply_command(
    state: &mut FloodWatchState,
    command: &engine_core::CommandEnvelope,
) -> Result<AppliedAction, Diagnostic> {
    let validated = validate_command(state, command)?;
    apply_validated_action(state, validated)
}

fn spend_budget(state: &mut FloodWatchState) -> Result<AppliedAction, Diagnostic> {
    let Phase::Action { budget_remaining } = state.phase else {
        return Err(Diagnostic {
            code: "wrong_phase".to_owned(),
            message: "that action is not available in the current phase".to_owned(),
        });
    };
    let next_budget = budget_remaining.saturating_sub(1);
    state.phase = Phase::Action {
        budget_remaining: next_budget,
    };
    state.freshness_token = state.freshness_token.next();

    Ok(AppliedAction {
        environment_pending: next_budget == 0,
    })
}
