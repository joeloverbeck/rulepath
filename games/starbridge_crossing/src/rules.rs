use engine_core::{CommandEnvelope, Diagnostic};

use crate::{
    actions::{self, parse_action_path, StarbridgeAction},
    effects::{public_effect, StarbridgeEffect, StarbridgeEffectEnvelope},
    ids::StarSpaceId,
    state::{StarPegId, StarbridgeState},
    topology::{neighbor_in_direction, StarDirection},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct StepMove {
    pub peg: StarPegId,
    pub from: StarSpaceId,
    pub to: StarSpaceId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ValidatedStep {
    pub seat_index: u8,
    pub step: StepMove,
}

pub fn legal_step_moves(state: &StarbridgeState) -> Vec<StepMove> {
    if state.terminal_status.is_some()
        || state
            .finish_ranks
            .iter()
            .any(|rank| rank.seat_index == state.active_seat_index)
    {
        return Vec::new();
    }

    let mut moves = Vec::new();
    for peg in state.pegs_for_seat(state.active_seat_index) {
        for direction in StarDirection::ALL {
            if let Some(destination) = neighbor_in_direction(peg.space, direction) {
                if state.occupancy(destination).is_none() {
                    moves.push(StepMove {
                        peg: peg.id,
                        from: peg.space,
                        to: destination,
                    });
                }
            }
        }
    }
    moves
}

pub fn validate_step_command(
    state: &StarbridgeState,
    command: &CommandEnvelope,
) -> Result<ValidatedStep, Diagnostic> {
    if command.freshness_token != state.freshness_token {
        return Err(stale_action_diagnostic());
    }
    if state.terminal_status.is_some() {
        return Err(terminal_diagnostic());
    }
    let active_seat = state
        .seats
        .get(usize::from(state.active_seat_index))
        .ok_or_else(wrong_seat_diagnostic)?;
    if command.actor.seat_id != active_seat.seat_id {
        return Err(wrong_seat_diagnostic());
    }
    if state
        .finish_ranks
        .iter()
        .any(|rank| rank.seat_index == state.active_seat_index)
    {
        return Err(finished_seat_diagnostic());
    }

    let StarbridgeAction::Step { peg, destination } =
        parse_action_path(&command.action_path.segments)?;
    let current = state
        .pegs
        .iter()
        .find(|candidate| candidate.id == peg)
        .ok_or_else(unknown_peg_diagnostic)?;
    if current.owner_seat_index != state.active_seat_index {
        return Err(wrong_peg_seat_diagnostic());
    }
    if state.occupancy(destination).is_some() {
        return Err(occupied_destination_diagnostic());
    }
    if !StarDirection::ALL
        .into_iter()
        .any(|direction| neighbor_in_direction(current.space, direction) == Some(destination))
    {
        return Err(non_adjacent_destination_diagnostic());
    }

    Ok(ValidatedStep {
        seat_index: state.active_seat_index,
        step: StepMove {
            peg,
            from: current.space,
            to: destination,
        },
    })
}

pub fn apply_step_command(
    state: &mut StarbridgeState,
    command: &CommandEnvelope,
) -> Result<Vec<StarbridgeEffectEnvelope>, Diagnostic> {
    let validated = validate_step_command(state, command)?;
    let peg = state
        .pegs
        .iter_mut()
        .find(|candidate| candidate.id == validated.step.peg)
        .expect("validated step peg exists");

    state.occupancy[usize::from(validated.step.from.index())] = None;
    state.occupancy[usize::from(validated.step.to.index())] = Some(validated.step.peg);
    peg.space = validated.step.to;
    state.active_seat_index = next_active_seat_index(state);
    state.ply_count = state.ply_count.saturating_add(1);
    state.command_count = state.command_count.saturating_add(1);
    state.freshness_token = state.freshness_token.next();

    Ok(vec![public_effect(StarbridgeEffect::Step {
        seat_index: validated.seat_index,
        peg: validated.step.peg,
        from: validated.step.from,
        to: validated.step.to,
    })])
}

fn next_active_seat_index(state: &StarbridgeState) -> u8 {
    if state.seats.is_empty() {
        return 0;
    }
    (usize::from(state.active_seat_index) + 1)
        .rem_euclid(state.seats.len())
        .try_into()
        .expect("supported seat count fits u8")
}

pub fn stale_action_diagnostic() -> Diagnostic {
    actions::diagnostic("stale_action", "action was built for an older state")
}

pub fn terminal_diagnostic() -> Diagnostic {
    actions::diagnostic("terminal", "match is already terminal")
}

pub fn wrong_seat_diagnostic() -> Diagnostic {
    actions::diagnostic("wrong_seat", "only the active seat may move")
}

pub fn finished_seat_diagnostic() -> Diagnostic {
    actions::diagnostic("finished_seat", "finished seats cannot move")
}

pub fn unknown_peg_diagnostic() -> Diagnostic {
    actions::diagnostic("unknown_peg", "step references an unknown peg")
}

pub fn wrong_peg_seat_diagnostic() -> Diagnostic {
    actions::diagnostic("wrong_peg_seat", "step peg is not owned by the active seat")
}

pub fn occupied_destination_diagnostic() -> Diagnostic {
    actions::diagnostic("occupied_destination", "step destination is occupied")
}

pub fn non_adjacent_destination_diagnostic() -> Diagnostic {
    actions::diagnostic(
        "non_adjacent_destination",
        "step destination is not adjacent",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{actions::encode_step_path, setup_match, SetupOptions};
    use engine_core::{ActionPath, Actor, FreshnessToken, RulesVersion, SeatId, Seed};

    fn command(
        actor: &Actor,
        path: Vec<String>,
        freshness_token: FreshnessToken,
    ) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor.clone(),
            action_path: ActionPath { segments: path },
            freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    #[test]
    fn accepted_step_mutates_state_and_emits_public_effect() {
        let seats = vec![
            SeatId::from_zero_based_index(0),
            SeatId::from_zero_based_index(1),
        ];
        let actor = Actor {
            seat_id: seats[0].clone(),
        };
        let mut state = setup_match(Seed(7), &seats, &SetupOptions::default()).unwrap();
        let step = legal_step_moves(&state)[0];

        let effects = apply_step_command(
            &mut state,
            &command(
                &actor,
                encode_step_path(step.peg, step.to),
                FreshnessToken(0),
            ),
        )
        .unwrap();

        assert_eq!(state.occupancy(step.from), None);
        assert_eq!(state.occupancy(step.to), Some(step.peg));
        assert_eq!(state.active_seat_index, 1);
        assert_eq!(state.ply_count, 1);
        assert_eq!(state.command_count, 1);
        assert_eq!(state.freshness_token, FreshnessToken(1));
        assert_eq!(
            effects[0].payload,
            StarbridgeEffect::Step {
                seat_index: 0,
                peg: step.peg,
                from: step.from,
                to: step.to,
            }
        );
    }
}
