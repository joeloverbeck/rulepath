use crate::ActionPath;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FreshnessToken(pub u64);

impl FreshnessToken {
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionTree {
    pub root: ActionNode,
    pub freshness_token: FreshnessToken,
}

impl ActionTree {
    pub fn flat(freshness_token: FreshnessToken, choices: Vec<ActionChoice>) -> Self {
        Self {
            root: ActionNode { choices },
            freshness_token,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionNode {
    pub choices: Vec<ActionChoice>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionChoice {
    pub segment: String,
    pub label: String,
    pub accessibility_label: String,
    pub metadata: Vec<ActionMetadata>,
    pub tags: Vec<String>,
    pub preview: ActionPreview,
    pub next: Option<Box<ActionNode>>,
}

impl ActionChoice {
    pub fn leaf(
        segment: impl Into<String>,
        label: impl Into<String>,
        accessibility_label: impl Into<String>,
    ) -> Self {
        Self {
            segment: segment.into(),
            label: label.into(),
            accessibility_label: accessibility_label.into(),
            metadata: Vec::new(),
            tags: Vec::new(),
            preview: ActionPreview::Unavailable,
            next: None,
        }
    }

    pub fn path(&self) -> ActionPath {
        ActionPath {
            segments: vec![self.segment.clone()],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionMetadata {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ActionPreview {
    Unavailable,
    Available,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_action_tree_keeps_stable_choice_segments() {
        let tree = ActionTree::flat(
            FreshnessToken(3),
            vec![
                ActionChoice::leaf("add-1", "Add 1", "Add one"),
                ActionChoice::leaf("add-2", "Add 2", "Add two"),
            ],
        );

        assert_eq!(tree.freshness_token, FreshnessToken(3));
        assert_eq!(tree.root.choices.len(), 2);
        assert_eq!(
            tree.root.choices[0].path(),
            ActionPath {
                segments: vec!["add-1".to_owned()]
            }
        );
        assert_eq!(tree.root.choices[1].segment, "add-2");
    }

    #[test]
    fn freshness_token_compares_by_version() {
        assert!(FreshnessToken(8) > FreshnessToken(7));
        assert_eq!(FreshnessToken(8).next(), FreshnessToken(9));
    }
}
