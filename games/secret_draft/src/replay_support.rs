use crate::state::SecretDraftState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayResult {
    pub command_count: usize,
}

pub fn state_hash(state: &SecretDraftState) -> String {
    format!(
        "secret_draft:round={}:pool={}:history={}",
        state.round_number,
        state.visible_pool.len(),
        state.revealed_history.len()
    )
}
