//! Action-tree encoding and parsing for Starbridge Crossing.

use std::collections::BTreeMap;

use engine_core::{
    ActionChoice, ActionMetadata, ActionNode, ActionPreview, ActionTree, Actor, Diagnostic,
};

use crate::{
    ids::StarSpaceId,
    rules::{is_active_seat_blocked, legal_jump_landings, legal_step_moves},
    state::{StarPegId, StarbridgeState},
};

pub const ACTION_MOVE: &str = "move";
pub const ACTION_STEP: &str = "step";
pub const ACTION_JUMP: &str = "jump";
pub const ACTION_CONTINUE: &str = "continue";
pub const ACTION_STOP: &str = "stop";
pub const ACTION_PASS_BLOCKED: &str = "pass_blocked";

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum StarbridgeAction {
    Step {
        peg: StarPegId,
        destination: StarSpaceId,
    },
    Jump {
        peg: StarPegId,
        landings: Vec<StarSpaceId>,
    },
    PassBlocked,
}

pub fn legal_action_tree(state: &StarbridgeState, actor: &Actor) -> ActionTree {
    let Some(active_seat) = state.seats.get(usize::from(state.active_seat_index)) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if active_seat.seat_id != actor.seat_id || state.terminal_status.is_some() {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let step_moves = legal_step_moves(state);
    let jump_pegs = state
        .pegs_for_seat(state.active_seat_index)
        .filter(|peg| !legal_jump_landings(state, peg.id, peg.space, &[]).is_empty())
        .map(|peg| peg.id)
        .collect::<Vec<_>>();
    if step_moves.is_empty() && jump_pegs.is_empty() {
        if is_active_seat_blocked(state) {
            return ActionTree::flat(
                state.freshness_token,
                vec![ActionChoice {
                    segment: ACTION_PASS_BLOCKED.to_owned(),
                    label: "Pass blocked".to_owned(),
                    accessibility_label: "Pass because no legal move is available".to_owned(),
                    metadata: Vec::new(),
                    tags: vec!["pass_blocked".to_owned()],
                    preview: ActionPreview::Available,
                    next: None,
                }],
            );
        }
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let mut by_peg: BTreeMap<StarPegId, Vec<StarSpaceId>> = BTreeMap::new();
    for step in step_moves {
        by_peg.entry(step.peg).or_default().push(step.to);
    }
    for peg in jump_pegs {
        by_peg.entry(peg).or_default();
    }

    let peg_choices = by_peg
        .into_iter()
        .map(|(peg, destinations)| peg_choice(state, peg, destinations))
        .collect::<Vec<_>>();
    let move_choice = ActionChoice {
        segment: ACTION_MOVE.to_owned(),
        label: "Move".to_owned(),
        accessibility_label: "Move a peg".to_owned(),
        metadata: Vec::new(),
        tags: vec!["move".to_owned()],
        preview: ActionPreview::Available,
        next: Some(Box::new(ActionNode {
            choices: peg_choices,
        })),
    };

    ActionTree {
        root: ActionNode {
            choices: vec![move_choice],
        },
        freshness_token: state.freshness_token,
    }
}

pub fn encode_step_path(peg: StarPegId, destination: StarSpaceId) -> Vec<String> {
    vec![
        ACTION_MOVE.to_owned(),
        peg.stable_id(),
        ACTION_STEP.to_owned(),
        destination.to_string(),
    ]
}

pub fn encode_jump_path(peg: StarPegId, landings: &[StarSpaceId]) -> Vec<String> {
    let mut path = vec![
        ACTION_MOVE.to_owned(),
        peg.stable_id(),
        ACTION_JUMP.to_owned(),
    ];
    for (index, landing) in landings.iter().enumerate() {
        if index > 0 {
            path.push(ACTION_CONTINUE.to_owned());
        }
        path.push(landing.to_string());
    }
    path.push(ACTION_STOP.to_owned());
    path
}

pub fn parse_action_path(segments: &[String]) -> Result<StarbridgeAction, Diagnostic> {
    let expanded;
    let segments = if segments.len() == 1 && segments[0].contains('/') {
        expanded = segments[0]
            .split('/')
            .map(str::to_owned)
            .collect::<Vec<_>>();
        expanded.as_slice()
    } else {
        segments
    };

    match segments {
        [family, peg, kind, destination] if family == ACTION_MOVE && kind == ACTION_STEP => {
            Ok(StarbridgeAction::Step {
                peg: parse_peg_id(peg)?,
                destination: parse_space_id(destination)?,
            })
        }
        [family, peg, kind, rest @ ..] if family == ACTION_MOVE && kind == ACTION_JUMP => {
            Ok(StarbridgeAction::Jump {
                peg: parse_peg_id(peg)?,
                landings: parse_jump_landings(rest)?,
            })
        }
        [family, _, kind, ..]
            if family == ACTION_MOVE && (kind == ACTION_STEP || kind == ACTION_JUMP) =>
        {
            Err(mixed_move_kind_diagnostic())
        }
        [action] if action == ACTION_PASS_BLOCKED => Ok(StarbridgeAction::PassBlocked),
        _ => Err(malformed_action_diagnostic()),
    }
}

fn parse_jump_landings(segments: &[String]) -> Result<Vec<StarSpaceId>, Diagnostic> {
    if segments.len() < 2 || segments.last().is_none_or(|segment| segment != ACTION_STOP) {
        return Err(malformed_action_diagnostic());
    }
    let body = &segments[..segments.len() - 1];
    let mut landings = Vec::new();
    let mut expect_landing = true;
    for segment in body {
        if expect_landing {
            if segment == ACTION_CONTINUE || segment == ACTION_STOP {
                return Err(malformed_action_diagnostic());
            }
            landings.push(parse_space_id(segment)?);
        } else if segment != ACTION_CONTINUE {
            return Err(malformed_action_diagnostic());
        }
        expect_landing = !expect_landing;
    }
    if expect_landing || landings.is_empty() {
        return Err(malformed_action_diagnostic());
    }
    Ok(landings)
}

pub fn parse_peg_id(value: &str) -> Result<StarPegId, Diagnostic> {
    let Some(raw) = value.strip_prefix('p') else {
        return Err(malformed_action_diagnostic());
    };
    let Some((seat, ordinal)) = raw.split_once('_') else {
        return Err(malformed_action_diagnostic());
    };
    Ok(StarPegId::new(
        seat.parse().map_err(|_| malformed_action_diagnostic())?,
        ordinal.parse().map_err(|_| malformed_action_diagnostic())?,
    ))
}

pub fn parse_space_id(value: &str) -> Result<StarSpaceId, Diagnostic> {
    let Some(raw) = value.strip_prefix('s') else {
        return Err(malformed_action_diagnostic());
    };
    let index = raw
        .parse::<u16>()
        .map_err(|_| malformed_action_diagnostic())?;
    StarSpaceId::new(index).map_err(|_| off_board_destination_diagnostic())
}

pub fn malformed_action_diagnostic() -> Diagnostic {
    diagnostic(
        "malformed_action",
        "action path must be a Starbridge step or jump path",
    )
}

pub fn off_board_destination_diagnostic() -> Diagnostic {
    diagnostic(
        "off_board_destination",
        "step destination is not on the board",
    )
}

pub fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

pub fn mixed_move_kind_diagnostic() -> Diagnostic {
    diagnostic(
        "mixed_move_kind",
        "a move cannot mix step and jump segments",
    )
}

fn peg_choice(
    state: &StarbridgeState,
    peg: StarPegId,
    destinations: Vec<StarSpaceId>,
) -> ActionChoice {
    let mut choices = Vec::new();
    if !destinations.is_empty() {
        choices.push(ActionChoice {
            segment: ACTION_STEP.to_owned(),
            label: "Step".to_owned(),
            accessibility_label: format!("Step peg {}", peg.stable_id()),
            metadata: Vec::new(),
            tags: vec!["step".to_owned()],
            preview: ActionPreview::Available,
            next: Some(Box::new(ActionNode {
                choices: destinations
                    .into_iter()
                    .map(|destination| destination_choice(peg, destination))
                    .collect(),
            })),
        });
    }
    let current = state
        .pegs
        .iter()
        .find(|candidate| candidate.id == peg)
        .expect("action tree peg exists")
        .space;
    let jump_choices = jump_landing_choices(state, peg, current, Vec::new());
    if !jump_choices.is_empty() {
        choices.push(ActionChoice {
            segment: ACTION_JUMP.to_owned(),
            label: "Jump".to_owned(),
            accessibility_label: format!("Jump peg {}", peg.stable_id()),
            metadata: Vec::new(),
            tags: vec!["jump".to_owned()],
            preview: ActionPreview::Available,
            next: Some(Box::new(ActionNode {
                choices: jump_choices,
            })),
        });
    }

    ActionChoice {
        segment: peg.stable_id(),
        label: peg.stable_id(),
        accessibility_label: format!("Peg {}", peg.stable_id()),
        metadata: vec![metadata("peg", peg.stable_id())],
        tags: vec!["peg".to_owned()],
        preview: ActionPreview::Available,
        next: Some(Box::new(ActionNode { choices })),
    }
}

fn jump_landing_choices(
    state: &StarbridgeState,
    peg: StarPegId,
    current: StarSpaceId,
    visited: Vec<StarSpaceId>,
) -> Vec<ActionChoice> {
    legal_jump_landings(state, peg, current, &visited)
        .into_iter()
        .map(|jump| {
            let mut next_visited = visited.clone();
            next_visited.push(jump.landing);
            let mut choices = vec![ActionChoice {
                segment: ACTION_STOP.to_owned(),
                label: "Stop".to_owned(),
                accessibility_label: format!("Stop after landing on {}", jump.landing),
                metadata: Vec::new(),
                tags: vec!["stop".to_owned()],
                preview: ActionPreview::Available,
                next: None,
            }];
            let continuation = jump_landing_choices(state, peg, jump.landing, next_visited);
            if !continuation.is_empty() {
                choices.push(ActionChoice {
                    segment: ACTION_CONTINUE.to_owned(),
                    label: "Continue".to_owned(),
                    accessibility_label: format!("Continue from {}", jump.landing),
                    metadata: Vec::new(),
                    tags: vec!["continue".to_owned()],
                    preview: ActionPreview::Available,
                    next: Some(Box::new(ActionNode {
                        choices: continuation,
                    })),
                });
            }
            ActionChoice {
                segment: jump.landing.to_string(),
                label: jump.landing.to_string(),
                accessibility_label: format!("Jump peg {} to {}", peg.stable_id(), jump.landing),
                metadata: vec![
                    metadata("over", jump.over.to_string()),
                    metadata("landing", jump.landing.to_string()),
                ],
                tags: vec!["jump_landing".to_owned()],
                preview: ActionPreview::Available,
                next: Some(Box::new(ActionNode { choices })),
            }
        })
        .collect()
}

fn destination_choice(peg: StarPegId, destination: StarSpaceId) -> ActionChoice {
    ActionChoice {
        segment: destination.to_string(),
        label: destination.to_string(),
        accessibility_label: format!("Step peg {} to {}", peg.stable_id(), destination),
        metadata: vec![metadata("destination", destination.to_string())],
        tags: vec!["step_destination".to_owned()],
        preview: ActionPreview::Available,
        next: None,
    }
}

fn metadata(key: &str, value: String) -> ActionMetadata {
    ActionMetadata {
        key: key.to_owned(),
        value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{setup_match, SetupOptions};
    use engine_core::{Actor, SeatId, Seed};

    #[test]
    fn step_path_round_trips_segmented_and_slash_forms() {
        let peg = StarPegId::new(0, 3);
        let destination = StarSpaceId::new(12).unwrap();
        let path = encode_step_path(peg, destination);

        assert_eq!(
            parse_action_path(&path),
            Ok(StarbridgeAction::Step { peg, destination })
        );
        assert_eq!(
            parse_action_path(&[path.join("/")]),
            Ok(StarbridgeAction::Step { peg, destination })
        );
    }

    #[test]
    fn jump_path_round_trips_segmented_and_slash_forms() {
        let peg = StarPegId::new(0, 3);
        let landings = vec![StarSpaceId::new(12).unwrap(), StarSpaceId::new(18).unwrap()];
        let path = encode_jump_path(peg, &landings);

        assert_eq!(
            parse_action_path(&path),
            Ok(StarbridgeAction::Jump {
                peg,
                landings: landings.clone(),
            })
        );
        assert_eq!(
            parse_action_path(&[path.join("/")]),
            Ok(StarbridgeAction::Jump { peg, landings })
        );
    }

    #[test]
    fn action_tree_has_no_dead_branches_for_opening_steps() {
        let seats = vec![
            SeatId::from_zero_based_index(0),
            SeatId::from_zero_based_index(1),
        ];
        let state = setup_match(Seed(7), &seats, &SetupOptions::default()).unwrap();
        let tree = legal_action_tree(
            &state,
            &Actor {
                seat_id: seats[0].clone(),
            },
        );

        assert!(!tree.has_dead_branches());
        assert_eq!(tree.freshness_token, state.freshness_token);
        assert_eq!(tree.root.choices[0].segment, ACTION_MOVE);
    }
}
