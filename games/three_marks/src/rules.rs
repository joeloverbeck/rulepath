use engine_core::{CommandEnvelope, Diagnostic, EffectEnvelope};

use crate::{
    actions::{actor_seat, legal_cells, parse_place_segment},
    ids::{CellId, ThreeMarksSeat},
    state::{CellOccupancy, TerminalOutcome, ThreeMarksState, WinningLine},
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

pub fn validate_command(
    state: &ThreeMarksState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if state.terminal_outcome.is_some() {
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

    let cell = parse_place_segment(&command.action_path.segments[0])
        .ok_or_else(|| diagnostic("invalid_cell", "the requested cell does not exist"))?;

    if !state.occupancy(cell).is_empty() {
        return Err(diagnostic(
            "occupied_cell",
            "the requested cell is already occupied",
        ));
    }

    if !legal_cells(state).contains(&cell) {
        return Err(diagnostic(
            "invalid_action",
            "the requested placement is not legal",
        ));
    }

    Ok(ValidatedAction { actor, cell })
}

pub fn apply_action(
    state: &mut ThreeMarksState,
    action: ValidatedAction,
) -> Vec<EffectEnvelope<()>> {
    state.cells[action.cell.index()] = CellOccupancy::Occupied(action.actor);
    state.ply_count += 1;
    state.freshness_token = state.freshness_token.next();

    if let Some(line) = winning_line(state, action.actor) {
        state.terminal_outcome = Some(TerminalOutcome::Win {
            seat: action.actor,
            line,
        });
    } else if state.cells.iter().all(|cell| !cell.is_empty()) {
        state.terminal_outcome = Some(TerminalOutcome::Draw);
    } else {
        state.active_seat = action.actor.other();
    }

    Vec::new()
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

fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}
