use engine_core::{CommandEnvelope, Diagnostic};

use crate::{
    actions::{active_commit_seat, actor_seat, parse_commit_segment},
    ids::{CardId, HighCardDuelSeat},
    state::{HighCardDuelState, Phase},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub actor: HighCardDuelSeat,
    pub card: CardId,
}

pub fn validate_command(
    state: &HighCardDuelState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if command.freshness_token != state.freshness_token {
        return Err(stale_action_diagnostic());
    }

    let actor = actor_seat(state, &command.actor).ok_or_else(wrong_seat_diagnostic)?;
    let Some(active_seat) = active_commit_seat(state) else {
        return Err(wrong_phase_diagnostic(state.phase));
    };
    if actor != active_seat {
        return Err(wrong_seat_diagnostic());
    }
    if state.commitment_for(actor).is_some() {
        return Err(commitment_conflict_diagnostic());
    }

    let [segment] = command.action_path.segments.as_slice() else {
        return Err(wrong_phase_diagnostic(state.phase));
    };
    let card = parse_commit_segment(segment).ok_or_else(invalid_private_card_diagnostic)?;
    if !state.hand_for(actor).contains(&card) {
        return Err(invalid_private_card_diagnostic());
    }

    Ok(ValidatedAction { actor, card })
}

pub fn wrong_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_seat".to_owned(),
        message: "this seat is not allowed to act at the current decision point".to_owned(),
    }
}

pub fn wrong_phase_diagnostic(phase: Phase) -> Diagnostic {
    Diagnostic {
        code: "wrong_phase".to_owned(),
        message: format!("commit actions are not available during {}", phase.as_str()),
    }
}

pub fn invalid_private_card_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "invalid_private_card".to_owned(),
        message: "the requested private card is redacted or unavailable".to_owned(),
    }
}

pub fn stale_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "stale_action".to_owned(),
        message: "the action was submitted for an older decision point".to_owned(),
    }
}

pub fn commitment_conflict_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "commitment_unavailable".to_owned(),
        message: "a face-down commitment is already present for this round".to_owned(),
    }
}
