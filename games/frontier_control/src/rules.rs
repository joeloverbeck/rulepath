//! Rule application for Frontier Control.

use engine_core::Diagnostic;

use crate::{
    actions::{validate_command, FrontierControlAction, ValidatedAction},
    effects::{public_effect, FrontierControlEffect, FrontierControlEffectEnvelope},
    ids::{FactionId, SiteId},
    state::{FrontierControlState, Phase},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppliedAction {
    pub turn_ended: bool,
    pub effects: Vec<FrontierControlEffectEnvelope>,
}

pub fn apply_command(
    state: &mut FrontierControlState,
    command: &engine_core::CommandEnvelope,
) -> Result<AppliedAction, Diagnostic> {
    let validated = validate_command(state, command)?;
    apply_validated_action(state, validated)
}

pub fn apply_validated_action(
    state: &mut FrontierControlState,
    validated: ValidatedAction,
) -> Result<AppliedAction, Diagnostic> {
    match validated.action {
        FrontierControlAction::March { from, to } => {
            let mut effects = Vec::new();
            apply_march(state, from, to, &mut effects)?;
            finish_spend(state, validated.actor_faction, &mut effects)
        }
        FrontierControlAction::Stake { site } => {
            state.site_mut(site).ok_or_else(action_unavailable)?.stake = true;
            let mut effects = vec![public_effect(FrontierControlEffect::StakePlaced { site })];
            finish_spend(state, validated.actor_faction, &mut effects)
        }
        FrontierControlAction::Muster => {
            let camp = state.variant.base_camp;
            let crews = {
                let site = state.site_mut(camp).ok_or_else(action_unavailable)?;
                site.crews = site.crews.saturating_add(1);
                site.crews
            };
            let mut effects = vec![public_effect(FrontierControlEffect::CrewMustered {
                site: camp,
                crews,
            })];
            finish_spend(state, validated.actor_faction, &mut effects)
        }
        FrontierControlAction::Patrol { from, to } => {
            let mut effects = Vec::new();
            apply_patrol(state, from, to, &mut effects)?;
            finish_spend(state, validated.actor_faction, &mut effects)
        }
        FrontierControlAction::Reinforce { site } => {
            let guards = {
                let site_state = state.site_mut(site).ok_or_else(action_unavailable)?;
                site_state.guards = site_state.guards.saturating_add(1);
                site_state.guards
            };
            let mut effects = vec![public_effect(FrontierControlEffect::GuardReinforced {
                site,
                guards,
            })];
            finish_spend(state, validated.actor_faction, &mut effects)
        }
        FrontierControlAction::Dismantle { site } => {
            state.site_mut(site).ok_or_else(action_unavailable)?.stake = false;
            let mut effects = vec![public_effect(FrontierControlEffect::StakeDismantled {
                site,
            })];
            finish_spend(state, validated.actor_faction, &mut effects)
        }
        FrontierControlAction::EndTurn => {
            state.phase = Phase::Action {
                budget_remaining: 0,
            };
            state.freshness_token = state.freshness_token.next();
            let mut effects = Vec::new();
            end_turn(state, validated.actor_faction, &mut effects);
            Ok(AppliedAction {
                turn_ended: true,
                effects,
            })
        }
    }
}

fn apply_march(
    state: &mut FrontierControlState,
    from: SiteId,
    to: SiteId,
    effects: &mut Vec<FrontierControlEffectEnvelope>,
) -> Result<(), Diagnostic> {
    state.site_mut(from).ok_or_else(action_unavailable)?.crews = state
        .site(from)
        .ok_or_else(action_unavailable)?
        .crews
        .saturating_sub(1);
    effects.push(public_effect(FrontierControlEffect::CrewMarched {
        from,
        to,
    }));

    let target_guards = state.site(to).ok_or_else(action_unavailable)?.guards;
    if target_guards > 0 {
        state.site_mut(to).ok_or_else(action_unavailable)?.guards = target_guards - 1;
        effects.push(public_effect(FrontierControlEffect::ClashResolved {
            site: to,
            guard_removed: true,
            crew_removed: true,
            entering_faction: FactionId::Prospectors,
        }));
    } else {
        let site = state.site_mut(to).ok_or_else(action_unavailable)?;
        site.crews = site.crews.saturating_add(1);
    }

    Ok(())
}

fn apply_patrol(
    state: &mut FrontierControlState,
    from: SiteId,
    to: SiteId,
    effects: &mut Vec<FrontierControlEffectEnvelope>,
) -> Result<(), Diagnostic> {
    state.site_mut(from).ok_or_else(action_unavailable)?.guards = state
        .site(from)
        .ok_or_else(action_unavailable)?
        .guards
        .saturating_sub(1);
    effects.push(public_effect(FrontierControlEffect::GuardPatrolled {
        from,
        to,
    }));

    let target_crews = state.site(to).ok_or_else(action_unavailable)?.crews;
    if target_crews > 0 {
        let site = state.site_mut(to).ok_or_else(action_unavailable)?;
        site.crews = target_crews - 1;
        site.guards = site.guards.saturating_add(1);
        effects.push(public_effect(FrontierControlEffect::ClashResolved {
            site: to,
            guard_removed: false,
            crew_removed: true,
            entering_faction: FactionId::Garrison,
        }));
    } else {
        let site = state.site_mut(to).ok_or_else(action_unavailable)?;
        site.guards = site.guards.saturating_add(1);
    }

    Ok(())
}

fn finish_spend(
    state: &mut FrontierControlState,
    actor_faction: FactionId,
    effects: &mut Vec<FrontierControlEffectEnvelope>,
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

    let turn_ended = next_budget == 0;
    if turn_ended {
        end_turn(state, actor_faction, effects);
    }

    Ok(AppliedAction {
        turn_ended,
        effects: effects.clone(),
    })
}

fn end_turn(
    state: &mut FrontierControlState,
    actor_faction: FactionId,
    effects: &mut Vec<FrontierControlEffectEnvelope>,
) {
    effects.push(public_effect(FrontierControlEffect::TurnEnded {
        faction: actor_faction,
        round: state.round_number,
    }));

    match actor_faction {
        FactionId::Prospectors => {
            state.active_faction = FactionId::Garrison;
        }
        FactionId::Garrison => {
            state.active_faction = FactionId::Prospectors;
            state.round_number = state.round_number.saturating_add(1);
        }
    }
    state.phase = Phase::Action {
        budget_remaining: state.variant.action_budget,
    };
    state.freshness_token = state.freshness_token.next();
}

fn action_unavailable() -> Diagnostic {
    Diagnostic {
        code: "action_unavailable".to_owned(),
        message: "that Frontier Control action is not available now".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId};

    use crate::{
        actions::{
            ACTION_DISMANTLE, ACTION_END_TURN, ACTION_MARCH, ACTION_MUSTER, ACTION_PATROL,
            ACTION_REINFORCE, ACTION_STAKE,
        },
        setup::{setup_match, SetupOptions},
    };

    use super::*;

    fn state() -> FrontierControlState {
        setup_match(
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    fn actor_for(state: &FrontierControlState, faction: FactionId) -> Actor {
        Actor {
            seat_id: state
                .seats
                .iter()
                .find(|seat| state.faction_for_seat(seat) == Some(faction))
                .expect("seat exists")
                .clone(),
        }
    }

    fn command(
        state: &FrontierControlState,
        faction: FactionId,
        segments: Vec<&str>,
    ) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor_for(state, faction),
            action_path: ActionPath {
                segments: segments.into_iter().map(str::to_owned).collect(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    #[test]
    fn crew_entering_guards_trades_crew_for_guard() {
        let mut state = state();
        let first_march = command(
            &state,
            FactionId::Prospectors,
            vec![ACTION_MARCH, "site_base_camp", "site_ford"],
        );
        let applied = apply_command(&mut state, &first_march).expect("march applies");
        let clash_march = command(
            &state,
            FactionId::Prospectors,
            vec![ACTION_MARCH, "site_ford", "site_gatehouse"],
        );
        apply_command(&mut state, &clash_march).expect("clash applies");

        assert!(!applied.turn_ended);
        assert_eq!(state.site(SiteId::BaseCamp).expect("site").crews, 2);
        assert_eq!(state.site(SiteId::Ford).expect("site").crews, 0);
        assert_eq!(state.site(SiteId::Gatehouse).expect("site").guards, 1);
    }

    #[test]
    fn guard_entering_crews_removes_crew_and_survives() {
        let mut state = state();
        let end_turn = command(&state, FactionId::Prospectors, vec![ACTION_END_TURN]);
        apply_command(&mut state, &end_turn).expect("prospector turn ends");

        let patrol = command(
            &state,
            FactionId::Garrison,
            vec![ACTION_PATROL, "site_gatehouse", "site_ford"],
        );
        let applied = apply_command(&mut state, &patrol).expect("patrol applies");

        assert!(!applied.turn_ended);
        assert_eq!(state.site(SiteId::Gatehouse).expect("site").guards, 1);
        assert_eq!(state.site(SiteId::Ford).expect("site").guards, 1);
        assert_eq!(state.site(SiteId::Ford).expect("site").crews, 0);
    }

    #[test]
    fn budget_exhaustion_advances_turn() {
        let mut state = state();
        let march = command(
            &state,
            FactionId::Prospectors,
            vec![ACTION_MARCH, "site_base_camp", "site_ford"],
        );
        apply_command(&mut state, &march).expect("first action applies");
        let muster = command(&state, FactionId::Prospectors, vec![ACTION_MUSTER]);
        let applied = apply_command(&mut state, &muster).expect("second action applies");

        assert!(applied.turn_ended);
        assert_eq!(state.active_faction, FactionId::Garrison);
        assert_eq!(
            state.phase,
            Phase::Action {
                budget_remaining: state.variant.action_budget
            }
        );
    }

    #[test]
    fn stake_reinforce_and_dismantle_apply_through_validation() {
        let mut state = state();
        let march = command(
            &state,
            FactionId::Prospectors,
            vec![ACTION_MARCH, "site_base_camp", "site_ford"],
        );
        apply_command(&mut state, &march).expect("march applies");
        let stake = command(
            &state,
            FactionId::Prospectors,
            vec![ACTION_STAKE, "site_ford"],
        );
        apply_command(&mut state, &stake).expect("stake applies");
        assert!(state.site(SiteId::Ford).expect("site").stake);

        let patrol = command(
            &state,
            FactionId::Garrison,
            vec![ACTION_PATROL, "site_gatehouse", "site_ford"],
        );
        apply_command(&mut state, &patrol).expect("patrol applies");
        let dismantle = command(
            &state,
            FactionId::Garrison,
            vec![ACTION_DISMANTLE, "site_ford"],
        );
        apply_command(&mut state, &dismantle).expect("dismantle applies");
        assert!(!state.site(SiteId::Ford).expect("site").stake);

        state.active_faction = FactionId::Garrison;
        state.phase = Phase::Action {
            budget_remaining: state.variant.action_budget,
        };
        let before = state.site(SiteId::SignalHill).expect("site").guards;
        let reinforce = command(
            &state,
            FactionId::Garrison,
            vec![ACTION_REINFORCE, "site_signal_hill"],
        );
        apply_command(&mut state, &reinforce).expect("reinforce applies");
        assert_eq!(
            state.site(SiteId::SignalHill).expect("site").guards,
            before + 1
        );
    }
}
