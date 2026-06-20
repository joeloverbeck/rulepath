use engine_core::Viewer;

use crate::{cards::CardId, ids::BriarCircuitSeat, state::BriarCircuitState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PassView {
    pub direction: String,
    pub committed_count: usize,
    pub pending_count: usize,
    pub own_selection: Vec<CardId>,
    pub own_committed: bool,
}

pub fn project_pass_view(state: &BriarCircuitState, viewer: &Viewer) -> Option<PassView> {
    let pass = state.pass_state()?;
    let viewer_seat = viewer
        .seat_id
        .as_ref()
        .and_then(|seat_id| BriarCircuitSeat::parse(&seat_id.0));
    let own_selection = viewer_seat
        .map(|seat| pass.selection_for(seat).to_vec())
        .unwrap_or_default();
    let own_committed = viewer_seat
        .map(|seat| pass.is_committed(seat))
        .unwrap_or(false);

    Some(PassView {
        direction: pass.direction.as_str().to_owned(),
        committed_count: pass.committed_count(),
        pending_count: pass.pending_count(),
        own_selection,
        own_committed,
    })
}
