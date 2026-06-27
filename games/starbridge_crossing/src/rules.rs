use engine_core::{CommandEnvelope, Diagnostic};

use crate::{
    actions::{self, parse_action_path, StarbridgeAction},
    effects::{public_effect, JumpSubstep, StarbridgeEffect, StarbridgeEffectEnvelope},
    ids::StarSpaceId,
    state::{StarPegId, StarbridgeState},
    topology::{home_spaces, neighbor_in_direction, StarDirection},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct StepMove {
    pub peg: StarPegId,
    pub from: StarSpaceId,
    pub to: StarSpaceId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct JumpLanding {
    pub over: StarSpaceId,
    pub landing: StarSpaceId,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct JumpChain {
    pub peg: StarPegId,
    pub from: StarSpaceId,
    pub hops: Vec<JumpLanding>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ValidatedStep {
    pub seat_index: u8,
    pub step: StepMove,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ValidatedJump {
    pub seat_index: u8,
    pub chain: JumpChain,
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
        if state.occupancy(peg.space) != Some(peg.id) {
            continue;
        }
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

pub fn is_active_seat_blocked(state: &StarbridgeState) -> bool {
    state.terminal_status.is_none()
        && legal_step_moves(state).is_empty()
        && state
            .pegs_for_seat(state.active_seat_index)
            .all(|peg| legal_jump_landings(state, peg.id, peg.space, &[]).is_empty())
}

pub fn legal_jump_landings(
    state: &StarbridgeState,
    peg: StarPegId,
    current: StarSpaceId,
    visited_landings: &[StarSpaceId],
) -> Vec<JumpLanding> {
    let Some(origin) = state
        .pegs
        .iter()
        .find(|candidate| candidate.id == peg)
        .map(|candidate| candidate.space)
    else {
        return Vec::new();
    };
    if state.occupancy(origin) != Some(peg) {
        return Vec::new();
    }

    let mut jumps = Vec::new();
    for direction in StarDirection::ALL {
        let Some(over) = neighbor_in_direction(current, direction) else {
            continue;
        };
        let Some(landing) = neighbor_in_direction(over, direction) else {
            continue;
        };
        if visited_landings.contains(&landing) {
            continue;
        }
        if occupancy_during_chain(state, peg, origin, current, over).is_none() {
            continue;
        }
        if occupancy_during_chain(state, peg, origin, current, landing).is_some() {
            continue;
        }
        jumps.push(JumpLanding { over, landing });
    }
    jumps
}

pub fn validate_jump_command(
    state: &StarbridgeState,
    command: &CommandEnvelope,
) -> Result<ValidatedJump, Diagnostic> {
    validate_common_command_state(state, command)?;
    let StarbridgeAction::Jump { peg, landings } =
        parse_action_path(&command.action_path.segments)?
    else {
        return Err(actions::mixed_move_kind_diagnostic());
    };
    let current = state
        .pegs
        .iter()
        .find(|candidate| candidate.id == peg)
        .ok_or_else(unknown_peg_diagnostic)?;
    if current.owner_seat_index != state.active_seat_index {
        return Err(wrong_peg_seat_diagnostic());
    }

    let origin = current.space;
    let mut current_space = origin;
    let mut visited = Vec::new();
    let mut hops = Vec::new();
    for landing in landings {
        if visited.contains(&landing) {
            return Err(repeated_landing_diagnostic());
        }
        let jump = legal_jump_landings(state, peg, current_space, &visited)
            .into_iter()
            .find(|candidate| candidate.landing == landing)
            .ok_or_else(invalid_jump_diagnostic)?;
        visited.push(landing);
        current_space = landing;
        hops.push(jump);
    }

    if hops.is_empty() {
        return Err(invalid_jump_diagnostic());
    }
    Ok(ValidatedJump {
        seat_index: state.active_seat_index,
        chain: JumpChain {
            peg,
            from: origin,
            hops,
        },
    })
}

pub fn validate_pass_blocked_command(
    state: &StarbridgeState,
    command: &CommandEnvelope,
) -> Result<u8, Diagnostic> {
    validate_common_command_state(state, command)?;
    match parse_action_path(&command.action_path.segments)? {
        StarbridgeAction::PassBlocked => {}
        _ => return Err(actions::mixed_move_kind_diagnostic()),
    }
    if !is_active_seat_blocked(state) {
        return Err(pass_not_blocked_diagnostic());
    }
    Ok(state.active_seat_index)
}

pub fn validate_step_command(
    state: &StarbridgeState,
    command: &CommandEnvelope,
) -> Result<ValidatedStep, Diagnostic> {
    validate_common_command_state(state, command)?;

    let StarbridgeAction::Step { peg, destination } =
        parse_action_path(&command.action_path.segments)?
    else {
        return Err(actions::mixed_move_kind_diagnostic());
    };
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
    state.ply_count = state.ply_count.saturating_add(1);
    state.command_count = state.command_count.saturating_add(1);
    state.freshness_token = state.freshness_token.next();
    let mut effects = vec![public_effect(StarbridgeEffect::Step {
        seat_index: validated.seat_index,
        peg: validated.step.peg,
        from: validated.step.from,
        to: validated.step.to,
    })];
    resolve_end_of_turn(state, validated.seat_index, &mut effects);

    Ok(effects)
}

pub fn apply_jump_command(
    state: &mut StarbridgeState,
    command: &CommandEnvelope,
) -> Result<Vec<StarbridgeEffectEnvelope>, Diagnostic> {
    let validated = validate_jump_command(state, command)?;
    let final_landing = validated
        .chain
        .hops
        .last()
        .expect("validated jump has at least one hop")
        .landing;
    let peg = state
        .pegs
        .iter_mut()
        .find(|candidate| candidate.id == validated.chain.peg)
        .expect("validated jump peg exists");

    state.occupancy[usize::from(validated.chain.from.index())] = None;
    state.occupancy[usize::from(final_landing.index())] = Some(validated.chain.peg);
    peg.space = final_landing;
    state.ply_count = state.ply_count.saturating_add(1);
    state.command_count = state.command_count.saturating_add(1);
    state.freshness_token = state.freshness_token.next();
    let mut effects = vec![public_effect(StarbridgeEffect::JumpChain {
        seat_index: validated.seat_index,
        peg: validated.chain.peg,
        from: validated.chain.from,
        hops: validated
            .chain
            .hops
            .iter()
            .map(|hop| JumpSubstep {
                over: hop.over,
                to: hop.landing,
            })
            .collect(),
    })];
    resolve_end_of_turn(state, validated.seat_index, &mut effects);

    Ok(effects)
}

pub fn apply_pass_blocked_command(
    state: &mut StarbridgeState,
    command: &CommandEnvelope,
) -> Result<Vec<StarbridgeEffectEnvelope>, Diagnostic> {
    let seat_index = validate_pass_blocked_command(state, command)?;
    state.ply_count = state.ply_count.saturating_add(1);
    state.command_count = state.command_count.saturating_add(1);
    state.freshness_token = state.freshness_token.next();
    let mut effects = vec![public_effect(StarbridgeEffect::PassBlocked { seat_index })];
    resolve_end_of_turn(state, seat_index, &mut effects);
    Ok(effects)
}

fn validate_common_command_state(
    state: &StarbridgeState,
    command: &CommandEnvelope,
) -> Result<(), Diagnostic> {
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
    Ok(())
}

fn occupancy_during_chain(
    state: &StarbridgeState,
    peg: StarPegId,
    origin: StarSpaceId,
    current: StarSpaceId,
    space: StarSpaceId,
) -> Option<StarPegId> {
    if space == current {
        Some(peg)
    } else if space == origin {
        None
    } else {
        state.occupancy(space)
    }
}

fn resolve_end_of_turn(
    state: &mut StarbridgeState,
    acted_seat: u8,
    effects: &mut Vec<StarbridgeEffectEnvelope>,
) {
    if seat_has_finished(state, acted_seat)
        && !state
            .finish_ranks
            .iter()
            .any(|rank| rank.seat_index == acted_seat)
    {
        let rank = next_finish_rank(state);
        state.finish_ranks.push(crate::state::FinishRank {
            seat_index: acted_seat,
            rank,
        });
        effects.push(public_effect(StarbridgeEffect::FinishAssigned {
            seat_index: acted_seat,
            rank,
        }));
    }

    if state.finish_ranks.len() + 1 >= state.seats.len() {
        assign_remaining_ranks(state);
        state.terminal_status = Some(crate::state::TerminalStatus::Complete);
        effects.push(public_effect(StarbridgeEffect::Terminal {
            reason: "terminal-all-but-one-finished".to_owned(),
        }));
        return;
    }

    if state.ply_count >= state.variant.max_plies {
        assign_turn_limit_ranks(state);
        state.terminal_status = Some(crate::state::TerminalStatus::TurnLimit {
            max_plies: state.variant.max_plies,
        });
        effects.push(public_effect(StarbridgeEffect::Terminal {
            reason: "terminal-turn-limit".to_owned(),
        }));
        return;
    }

    state.active_seat_index = next_active_seat_index(state);
}

fn seat_has_finished(state: &StarbridgeState, seat_index: u8) -> bool {
    let Some(assignment) = state.seats.get(usize::from(seat_index)) else {
        return false;
    };
    let target_spaces = home_spaces(assignment.target)
        .map(|space| space.id)
        .collect::<Vec<_>>();
    let seat_pegs = state.pegs_for_seat(seat_index).collect::<Vec<_>>();
    seat_pegs.len() == usize::from(state.variant.pegs_per_seat)
        && seat_pegs
            .iter()
            .all(|peg| target_spaces.contains(&peg.space))
}

fn next_finish_rank(state: &StarbridgeState) -> u8 {
    u8::try_from(state.finish_ranks.len() + 1).expect("supported seat count fits u8")
}

fn assign_remaining_ranks(state: &mut StarbridgeState) {
    for seat_index in 0..state.seats.len() {
        let seat_index = u8::try_from(seat_index).expect("supported seat count fits u8");
        if !state
            .finish_ranks
            .iter()
            .any(|rank| rank.seat_index == seat_index)
        {
            let rank = next_finish_rank(state);
            state
                .finish_ranks
                .push(crate::state::FinishRank { seat_index, rank });
        }
    }
}

fn assign_turn_limit_ranks(state: &mut StarbridgeState) {
    let mut unfinished = (0..state.seats.len())
        .map(|seat_index| u8::try_from(seat_index).expect("supported seat count fits u8"))
        .filter(|seat_index| {
            !state
                .finish_ranks
                .iter()
                .any(|rank| rank.seat_index == *seat_index)
        })
        .map(|seat_index| (seat_index, progress_score(state, seat_index)))
        .collect::<Vec<_>>();
    unfinished.sort_by_key(|(seat_index, score)| (std::cmp::Reverse(*score), *seat_index));
    for (seat_index, _) in unfinished {
        let rank = next_finish_rank(state);
        state
            .finish_ranks
            .push(crate::state::FinishRank { seat_index, rank });
    }
}

fn progress_score(state: &StarbridgeState, seat_index: u8) -> u8 {
    let Some(assignment) = state.seats.get(usize::from(seat_index)) else {
        return 0;
    };
    let target_spaces = home_spaces(assignment.target)
        .map(|space| space.id)
        .collect::<Vec<_>>();
    u8::try_from(
        state
            .pegs_for_seat(seat_index)
            .filter(|peg| target_spaces.contains(&peg.space))
            .count(),
    )
    .expect("standard peg count fits u8")
}

fn next_active_seat_index(state: &StarbridgeState) -> u8 {
    if state.seats.is_empty() {
        return 0;
    }
    for offset in 1..=state.seats.len() {
        let candidate =
            (usize::from(state.active_seat_index) + offset).rem_euclid(state.seats.len());
        let candidate = u8::try_from(candidate).expect("supported seat count fits u8");
        if !state
            .finish_ranks
            .iter()
            .any(|rank| rank.seat_index == candidate)
        {
            return candidate;
        }
    }
    state.active_seat_index
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

pub fn invalid_jump_diagnostic() -> Diagnostic {
    actions::diagnostic(
        "invalid_jump",
        "jump path is not legal from the current board",
    )
}

pub fn repeated_landing_diagnostic() -> Diagnostic {
    actions::diagnostic(
        "repeated_landing",
        "jump chain cannot revisit a landing in one turn",
    )
}

pub fn pass_not_blocked_diagnostic() -> Diagnostic {
    actions::diagnostic(
        "pass_not_blocked",
        "blocked pass is legal only with no moves",
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
