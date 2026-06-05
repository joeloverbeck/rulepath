use engine_core::{ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor};

use crate::{ids::RaceSeat, state::RaceState};

pub const ACTION_SEGMENT_PREFIX: &str = "add-";

pub fn legal_action_tree(state: &RaceState, actor: &Actor) -> ActionTree {
    if state.winner.is_some() || actor_seat(state, actor) != Some(state.active_seat) {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    ActionTree::flat(
        state.freshness_token,
        legal_additions(state)
            .into_iter()
            .map(action_choice)
            .collect(),
    )
}

pub fn legal_additions(state: &RaceState) -> Vec<u8> {
    if state.winner.is_some() || state.counter.0 >= state.variant.target {
        return Vec::new();
    }

    let remaining = state.variant.target - state.counter.0;
    let max = state.variant.max_add.min(remaining);
    (1..=max).collect()
}

pub fn actor_seat(state: &RaceState, actor: &Actor) -> Option<RaceSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(RaceSeat::from_index)
}

pub fn parse_add_segment(segment: &str) -> Option<u8> {
    segment
        .strip_prefix(ACTION_SEGMENT_PREFIX)?
        .parse::<u8>()
        .ok()
}

fn action_choice(amount: u8) -> ActionChoice {
    let mut choice = ActionChoice::leaf(
        format!("{ACTION_SEGMENT_PREFIX}{amount}"),
        format!("Add {amount}"),
        format!("Add {amount} to the counter"),
    );
    choice.metadata = vec![ActionMetadata {
        key: "amount".to_owned(),
        value: amount.to_string(),
    }];
    choice.tags = vec!["flat".to_owned(), "counter".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}
