use engine_core::{ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor};

use crate::{
    ids::{CardId, HighCardDuelSeat},
    state::{HighCardDuelState, Phase},
};

pub const COMMIT_SEGMENT_PREFIX: &str = "commit/";

pub fn legal_action_tree(state: &HighCardDuelState, actor: &Actor) -> ActionTree {
    let Some(actor_seat) = actor_seat(state, actor) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if active_commit_seat(state) != Some(actor_seat) || state.commitment_for(actor_seat).is_some() {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    ActionTree::flat(
        state.freshness_token,
        state
            .hand_for(actor_seat)
            .iter()
            .map(|card| commit_choice(actor_seat, *card, state.phase))
            .collect(),
    )
}

pub fn actor_seat(state: &HighCardDuelState, actor: &Actor) -> Option<HighCardDuelSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(HighCardDuelSeat::from_index)
}

pub fn active_commit_seat(state: &HighCardDuelState) -> Option<HighCardDuelSeat> {
    match state.phase {
        Phase::LeadCommit => Some(state.lead_seat),
        Phase::ReplyCommit => Some(state.lead_seat.other()),
        Phase::Revealed | Phase::Terminal => None,
    }
}

pub fn commit_segment(card: CardId) -> String {
    format!("{COMMIT_SEGMENT_PREFIX}{}", card.stable_id())
}

pub fn parse_commit_segment(segment: &str) -> Option<CardId> {
    CardId::parse(segment.strip_prefix(COMMIT_SEGMENT_PREFIX)?)
}

fn commit_choice(actor: HighCardDuelSeat, card: CardId, phase: Phase) -> ActionChoice {
    let stable_id = card.stable_id();
    let mut choice = ActionChoice::leaf(
        commit_segment(card),
        format!("Commit {stable_id}"),
        format!(
            "Commit your rank {} {} card face-down",
            card.rank(),
            card.sigil().as_str()
        ),
    );
    choice.metadata = vec![
        metadata("phase", phase.as_str()),
        metadata("actor_seat", actor.as_str()),
        metadata("card_id", stable_id),
        metadata("rank", card.rank().to_string()),
        metadata("sigil", card.sigil().as_str()),
    ];
    choice.tags = vec!["private-commit".to_owned(), "own-card".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn metadata(key: impl Into<String>, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.into(),
        value: value.into(),
    }
}
