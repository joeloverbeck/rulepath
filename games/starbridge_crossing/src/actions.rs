//! Action-tree encoding and parsing for Starbridge Crossing.

use std::collections::BTreeMap;

use engine_core::{
    ActionChoice, ActionMetadata, ActionNode, ActionPreview, ActionTree, Actor, Diagnostic,
};

use crate::{
    ids::StarSpaceId,
    rules::legal_step_moves,
    state::{StarPegId, StarbridgeState},
};

pub const ACTION_MOVE: &str = "move";
pub const ACTION_STEP: &str = "step";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StarbridgeAction {
    Step {
        peg: StarPegId,
        destination: StarSpaceId,
    },
}

pub fn legal_action_tree(state: &StarbridgeState, actor: &Actor) -> ActionTree {
    let Some(active_seat) = state.seats.get(usize::from(state.active_seat_index)) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if active_seat.seat_id != actor.seat_id || state.terminal_status.is_some() {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let moves = legal_step_moves(state);
    if moves.is_empty() {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let mut by_peg: BTreeMap<StarPegId, Vec<StarSpaceId>> = BTreeMap::new();
    for step in moves {
        by_peg.entry(step.peg).or_default().push(step.to);
    }

    let peg_choices = by_peg
        .into_iter()
        .map(|(peg, destinations)| peg_choice(peg, destinations))
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
        _ => Err(malformed_action_diagnostic()),
    }
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
        "action path must be move/<peg-id>/step/<dest-space>",
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

fn peg_choice(peg: StarPegId, destinations: Vec<StarSpaceId>) -> ActionChoice {
    ActionChoice {
        segment: peg.stable_id(),
        label: peg.stable_id(),
        accessibility_label: format!("Peg {}", peg.stable_id()),
        metadata: vec![metadata("peg", peg.stable_id())],
        tags: vec!["peg".to_owned()],
        preview: ActionPreview::Available,
        next: Some(Box::new(ActionNode {
            choices: vec![ActionChoice {
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
            }],
        })),
    }
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
