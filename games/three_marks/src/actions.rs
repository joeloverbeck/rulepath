use engine_core::{ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor};

use crate::{
    ids::{CellId, ThreeMarksSeat},
    state::ThreeMarksState,
};

pub const ACTION_SEGMENT_PREFIX: &str = "place/";

pub fn legal_action_tree(state: &ThreeMarksState, actor: &Actor) -> ActionTree {
    if state.terminal_outcome.is_some() || actor_seat(state, actor) != Some(state.active_seat) {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    ActionTree::flat(
        state.freshness_token,
        legal_cells(state).into_iter().map(action_choice).collect(),
    )
}

pub fn legal_cells(state: &ThreeMarksState) -> Vec<CellId> {
    if state.terminal_outcome.is_some() {
        return Vec::new();
    }

    CellId::ALL
        .into_iter()
        .filter(|cell| state.occupancy(*cell).is_empty())
        .collect()
}

pub fn actor_seat(state: &ThreeMarksState, actor: &Actor) -> Option<ThreeMarksSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(ThreeMarksSeat::from_index)
}

pub fn parse_place_segment(segment: &str) -> Option<CellId> {
    CellId::parse(segment.strip_prefix(ACTION_SEGMENT_PREFIX)?)
}

fn action_choice(cell: CellId) -> ActionChoice {
    let cell_id = cell.as_str();
    let mut choice = ActionChoice::leaf(
        format!("{ACTION_SEGMENT_PREFIX}{cell_id}"),
        format!("Place at {cell_id}"),
        format!("Place mark at cell {cell_id}"),
    );
    choice.metadata = vec![ActionMetadata {
        key: "cell".to_owned(),
        value: cell_id.to_owned(),
    }];
    choice.tags = vec!["flat".to_owned(), "placement".to_owned(), "cell".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}
