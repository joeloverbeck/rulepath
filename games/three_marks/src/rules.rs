use engine_core::{CommandEnvelope, Diagnostic, EffectEnvelope};

use crate::{
    actions::{actor_seat, legal_cells, parse_place_segment},
    effects::{public_effect, RejectionReason, ThreeMarksEffect},
    ids::{CellId, ThreeMarksSeat},
    state::{CellOccupancy, TerminalOutcome, ThreeMarksSnapshot, ThreeMarksState, WinningLine},
};

const WINNING_LINES: [[CellId; 3]; 8] = [
    [CellId::R1C1, CellId::R1C2, CellId::R1C3],
    [CellId::R2C1, CellId::R2C2, CellId::R2C3],
    [CellId::R3C1, CellId::R3C2, CellId::R3C3],
    [CellId::R1C1, CellId::R2C1, CellId::R3C1],
    [CellId::R1C2, CellId::R2C2, CellId::R3C2],
    [CellId::R1C3, CellId::R2C3, CellId::R3C3],
    [CellId::R1C1, CellId::R2C2, CellId::R3C3],
    [CellId::R1C3, CellId::R2C2, CellId::R3C1],
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub actor: ThreeMarksSeat,
    pub cell: CellId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectedAction {
    pub diagnostic: Diagnostic,
    pub effects: Vec<EffectEnvelope<ThreeMarksEffect>>,
}

pub fn validate_command(
    state: &ThreeMarksState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    validate_command_with_effects(state, command).map_err(|rejected| rejected.diagnostic)
}

pub fn validate_command_with_effects(
    state: &ThreeMarksState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, RejectedAction> {
    if state.terminal_outcome.is_some() {
        return Err(rejected(
            RejectionReason::Terminal,
            "match_finished",
            "the match is already finished",
        ));
    }

    if command.freshness_token != state.freshness_token {
        return Err(rejected(
            RejectionReason::Stale,
            "stale_action",
            "the action was submitted for an older decision point",
        ));
    }

    let Some(actor) = actor_seat(state, &command.actor) else {
        return Err(rejected(
            RejectionReason::UnknownActor,
            "unknown_actor",
            "the actor is not seated",
        ));
    };

    if actor != state.active_seat {
        return Err(rejected(
            RejectionReason::WrongActor,
            "wrong_actor",
            "only the active seat may act now",
        ));
    }

    if command.action_path.segments.len() != 1 {
        return Err(rejected(
            RejectionReason::InvalidPath,
            "invalid_action_path",
            "the action path is not available",
        ));
    }

    let cell = parse_place_segment(&command.action_path.segments[0]).ok_or_else(|| {
        rejected(
            RejectionReason::InvalidCell,
            "invalid_cell",
            "the requested cell does not exist",
        )
    })?;

    if !state.occupancy(cell).is_empty() {
        return Err(rejected(
            RejectionReason::Occupied,
            "occupied_cell",
            "the requested cell is already occupied",
        ));
    }

    if !legal_cells(state).contains(&cell) {
        return Err(rejected(
            RejectionReason::InvalidAction,
            "invalid_action",
            "the requested placement is not legal",
        ));
    }

    Ok(ValidatedAction { actor, cell })
}

pub fn apply_action(
    state: &mut ThreeMarksState,
    action: ValidatedAction,
) -> Vec<EffectEnvelope<ThreeMarksEffect>> {
    state.cells[action.cell.index()] = CellOccupancy::Occupied(action.actor);
    state.ply_count += 1;
    state.freshness_token = state.freshness_token.next();
    let mut effects = vec![public_effect(ThreeMarksEffect::MarkPlaced {
        seat: action.actor,
        cell: action.cell,
        ply: state.ply_count,
        occupancy_summary: occupancy_summary(state),
    })];

    if let Some(line) = winning_line(state, action.actor) {
        let outcome = TerminalOutcome::Win {
            seat: action.actor,
            line,
        };
        state.terminal_outcome = Some(outcome);
        effects.push(public_effect(ThreeMarksEffect::LineCompleted {
            winning_seat: action.actor,
            line,
        }));
        effects.push(public_effect(ThreeMarksEffect::GameEnded {
            outcome,
            final_ply: state.ply_count,
            terminal_hash_ref: ThreeMarksSnapshot::from_state(state).stable_summary(),
        }));
    } else if state.cells.iter().all(|cell| !cell.is_empty()) {
        state.terminal_outcome = Some(TerminalOutcome::Draw);
        effects.push(public_effect(ThreeMarksEffect::DrawReached {
            final_ply: state.ply_count,
            full_board: true,
        }));
        effects.push(public_effect(ThreeMarksEffect::GameEnded {
            outcome: TerminalOutcome::Draw,
            final_ply: state.ply_count,
            terminal_hash_ref: ThreeMarksSnapshot::from_state(state).stable_summary(),
        }));
    } else {
        let previous_seat = action.actor;
        state.active_seat = action.actor.other();
        effects.push(public_effect(ThreeMarksEffect::ActivePlayerChanged {
            previous_seat,
            active_seat: state.active_seat,
            ply: state.ply_count,
        }));
    }

    effects
}

pub fn winning_line(state: &ThreeMarksState, seat: ThreeMarksSeat) -> Option<WinningLine> {
    WINNING_LINES
        .into_iter()
        .find(|line| {
            line.iter()
                .all(|cell| state.occupancy(*cell) == CellOccupancy::Occupied(seat))
        })
        .map(|cells| WinningLine { cells })
}

fn rejected(reason: RejectionReason, code: &str, message: &str) -> RejectedAction {
    let diagnostic = Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    };
    RejectedAction {
        diagnostic,
        effects: vec![public_effect(ThreeMarksEffect::PlacementRejected {
            reason,
            label: message.to_owned(),
        })],
    }
}

fn occupancy_summary(state: &ThreeMarksState) -> String {
    CellId::ALL
        .iter()
        .map(|cell| {
            let value = match state.occupancy(*cell) {
                CellOccupancy::Empty => "empty",
                CellOccupancy::Occupied(seat) => seat.as_str(),
            };
            format!("{}:{value}", cell.as_str())
        })
        .collect::<Vec<_>>()
        .join(",")
}
