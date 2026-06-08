use engine_core::{ActionTree, FreshnessToken};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecretDraftAction {
    pub path: Vec<String>,
}

pub fn legal_action_tree(freshness_token: FreshnessToken) -> ActionTree {
    ActionTree::flat(freshness_token, Vec::new())
}
