use engine_core::Diagnostic;

use crate::{actions::SecretDraftAction, state::SecretDraftState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub action: SecretDraftAction,
}

pub fn legal_actions(_state: &SecretDraftState) -> Vec<SecretDraftAction> {
    Vec::new()
}

pub fn validate_action(
    _state: &SecretDraftState,
    action: SecretDraftAction,
) -> Result<ValidatedAction, Diagnostic> {
    Ok(ValidatedAction { action })
}

pub fn apply_action(
    _state: &mut SecretDraftState,
    _action: ValidatedAction,
) -> Result<Vec<crate::effects::SecretDraftEffect>, Diagnostic> {
    Ok(Vec::new())
}
