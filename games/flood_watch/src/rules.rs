//! Rule application for Flood Watch.

use engine_core::Diagnostic;

use crate::{
    actions::{
        role_bail_amount, role_reinforce_amount, validate_command, FloodWatchAction,
        ValidatedAction,
    },
    effects::{public_effect, FloodWatchEffect, FloodWatchEffectEnvelope},
    ids::{DistrictId, EventKind},
    state::{FloodWatchState, Phase, SharedOutcome},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppliedAction {
    pub environment_pending: bool,
    pub effects: Vec<FloodWatchEffectEnvelope>,
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
            let mut effects = vec![public_effect(FloodWatchEffect::DistrictBailed {
                district,
                amount,
            })];
            finish_spend(state, &mut effects)
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
            let mut effects = vec![public_effect(FloodWatchEffect::LeveePlaced {
                district,
                amount,
            })];
            finish_spend(state, &mut effects)
        }
        FloodWatchAction::Forecast => {
            state.forecast = state.top_undrawn_card().cloned();
            let mut effects = state
                .forecast
                .as_ref()
                .map(|card| {
                    vec![public_effect(FloodWatchEffect::ForecastRevealed {
                        card: card.kind,
                    })]
                })
                .unwrap_or_default();
            finish_spend(state, &mut effects)
        }
        FloodWatchAction::EndTurn => {
            state.phase = Phase::Action {
                budget_remaining: 0,
            };
            state.freshness_token = state.freshness_token.next();
            let mut effects = Vec::new();
            resolve_environment_phase(state, &mut effects);
            Ok(AppliedAction {
                environment_pending: false,
                effects,
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

fn finish_spend(
    state: &mut FloodWatchState,
    effects: &mut Vec<FloodWatchEffectEnvelope>,
) -> Result<AppliedAction, Diagnostic> {
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

    if next_budget == 0 {
        resolve_environment_phase(state, effects);
    }

    Ok(AppliedAction {
        environment_pending: false,
        effects: effects.clone(),
    })
}

fn resolve_environment_phase(
    state: &mut FloodWatchState,
    effects: &mut Vec<FloodWatchEffectEnvelope>,
) {
    let draw_limit = state.variant.draws_per_phase;
    effects.push(public_effect(FloodWatchEffect::EnvironmentPhaseBegan {
        turn: state.turn_number,
        draws: draw_limit,
    }));

    for draw_index in 1..=draw_limit {
        let Some(card) = state.draw_next_event() else {
            effects.push(public_effect(FloodWatchEffect::DeckExhausted));
            finalize_terminal(state, SharedOutcome::Won, effects);
            return;
        };
        effects.push(public_effect(FloodWatchEffect::EventDrawn {
            index: draw_index,
            card: card.kind,
        }));

        let Some(district) = event_district(card.kind) else {
            continue;
        };
        let rise = event_rise(card.kind);
        if rise == 0 {
            continue;
        }
        if let Some(inundated) = resolve_rise(state, district, rise, effects) {
            finalize_terminal(
                state,
                SharedOutcome::Lost {
                    district: inundated,
                },
                effects,
            );
            return;
        }
    }

    if state.undrawn_deck_len() == 0 {
        effects.push(public_effect(FloodWatchEffect::DeckExhausted));
        finalize_terminal(state, SharedOutcome::Won, effects);
        return;
    }

    state.advance_to_next_turn();
    state.freshness_token = state.freshness_token.next();
}

fn resolve_rise(
    state: &mut FloodWatchState,
    district: DistrictId,
    rise: u8,
    effects: &mut Vec<FloodWatchEffectEnvelope>,
) -> Option<DistrictId> {
    let max_flood_level = state.variant.max_flood_level;
    let district_state = state.district_mut(district)?;

    let absorbed = district_state.levees.min(rise);
    if absorbed > 0 {
        district_state.levees -= absorbed;
        effects.push(public_effect(FloodWatchEffect::LeveeAbsorbed {
            district,
            amount: absorbed,
            remaining_levees: district_state.levees,
        }));
    }

    let remainder = rise - absorbed;
    if remainder > 0 {
        let previous_level = district_state.flood_level;
        district_state.flood_level = district_state
            .flood_level
            .saturating_add(remainder)
            .min(max_flood_level);
        let actual_rise = district_state.flood_level.saturating_sub(previous_level);
        if actual_rise > 0 {
            effects.push(public_effect(FloodWatchEffect::FloodLevelRose {
                district,
                amount: actual_rise,
                new_level: district_state.flood_level,
            }));
        }
    }

    if district_state.flood_level >= max_flood_level {
        effects.push(public_effect(FloodWatchEffect::DistrictInundated {
            district,
        }));
        return Some(district);
    }

    None
}

fn event_district(kind: EventKind) -> Option<DistrictId> {
    match kind {
        EventKind::Downpour { district } | EventKind::StormSurge { district } => Some(district),
        EventKind::Reprieve => None,
    }
}

fn event_rise(kind: EventKind) -> u8 {
    match kind {
        EventKind::Downpour { .. } => 1,
        EventKind::StormSurge { .. } => 2,
        EventKind::Reprieve => 0,
    }
}

fn finalize_terminal(
    state: &mut FloodWatchState,
    outcome: SharedOutcome,
    effects: &mut Vec<FloodWatchEffectEnvelope>,
) {
    if state.terminal_outcome.is_some() {
        return;
    }

    state.terminal_outcome = Some(outcome.clone());
    state.phase = Phase::Terminal;
    effects.push(public_effect(FloodWatchEffect::Terminal {
        outcome: outcome.stable_summary(),
        summary: terminal_summary(state, &outcome),
    }));
}

fn terminal_summary(
    state: &FloodWatchState,
    outcome: &SharedOutcome,
) -> crate::effects::TerminalSummary {
    let surviving_levels = state
        .districts
        .iter()
        .map(|district| (district.district, district.flood_level))
        .collect::<Vec<_>>();
    let drawn_card_count = state.drawn.len() as u8;

    match outcome {
        SharedOutcome::Won => crate::effects::TerminalSummary {
            rule_id: "FW-END-002".to_owned(),
            public_summary: format!(
                "Shared win: the final event resolved with all districts below inundation after {drawn_card_count} drawn cards."
            ),
            drawn_card_count,
            surviving_levels,
        },
        SharedOutcome::Lost { district } => crate::effects::TerminalSummary {
            rule_id: "FW-END-001".to_owned(),
            public_summary: format!(
                "Shared loss: {} reached inundation on turn {} after {drawn_card_count} drawn cards.",
                district.label(),
                state.turn_number
            ),
            drawn_card_count,
            surviving_levels,
        },
    }
}
