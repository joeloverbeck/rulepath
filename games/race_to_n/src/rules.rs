use engine_core::{CommandEnvelope, Diagnostic, EffectEnvelope};

use crate::{
    actions::{actor_seat, legal_additions, parse_add_segment},
    effects::{public_effect, RaceEffect},
    ids::RaceSeat,
    state::{CounterValue, RaceState},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub actor: RaceSeat,
    pub amount: u8,
}

pub fn validate_command(
    state: &RaceState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if state.winner.is_some() {
        return Err(diagnostic(
            "match_finished",
            "the match is already finished",
        ));
    }

    if command.freshness_token != state.freshness_token {
        return Err(diagnostic(
            "stale_action",
            "the action was submitted for an older decision point",
        ));
    }

    let Some(actor) = actor_seat(state, &command.actor) else {
        return Err(diagnostic("unknown_actor", "the actor is not seated"));
    };

    if actor != state.active_seat {
        return Err(diagnostic(
            "wrong_actor",
            "only the active seat may act now",
        ));
    }

    if command.action_path.segments.len() != 1 {
        return Err(diagnostic(
            "invalid_action_path",
            "the action path is not available",
        ));
    }

    let amount = parse_add_segment(&command.action_path.segments[0])
        .ok_or_else(|| diagnostic("invalid_action_path", "the action path is not available"))?;

    if !legal_additions(state).contains(&amount) {
        return Err(diagnostic(
            "invalid_action",
            "the requested counter advance is not legal",
        ));
    }

    Ok(ValidatedAction { actor, amount })
}

pub fn apply_action(
    state: &mut RaceState,
    action: ValidatedAction,
) -> Vec<EffectEnvelope<RaceEffect>> {
    let from = state.counter;
    let to = CounterValue(from.0 + action.amount);
    state.counter = to;
    state.freshness_token = state.freshness_token.next();

    let mut effects = vec![
        public_effect(RaceEffect::ActionStarted {
            actor: action.actor,
            amount: action.amount,
        }),
        public_effect(RaceEffect::CounterAdvanced {
            actor: action.actor,
            from,
            to,
            amount: action.amount,
        }),
    ];

    if to.0 == state.variant.target {
        state.winner = Some(action.actor);
        effects.push(public_effect(RaceEffect::GameEnded {
            winner: action.actor,
        }));
    } else {
        state.active_seat = action.actor.other();
        effects.push(public_effect(RaceEffect::TurnChanged {
            next_actor: state.active_seat,
        }));
    }

    effects.push(public_effect(RaceEffect::ActionCompleted {
        actor: action.actor,
    }));
    effects
}

fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}
