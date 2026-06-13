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

    pub fn dead_branch_paths(&self) -> Vec<Vec<String>> {
        self.root.dead_branch_paths()
    }

    pub fn has_dead_branches(&self) -> bool {
        !self.dead_branch_paths().is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionNode {
    pub choices: Vec<ActionChoice>,
}

impl ActionNode {
    pub fn dead_branch_paths(&self) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        self.collect_dead_branch_paths(&mut Vec::new(), &mut paths);
        paths
    }

    fn collect_dead_branch_paths(&self, prefix: &mut Vec<String>, paths: &mut Vec<Vec<String>>) {
        for choice in &self.choices {
            if let Some(next) = &choice.next {
                prefix.push(choice.segment.clone());
                if !next.has_reachable_leaf() {
                    paths.push(prefix.clone());
                } else {
                    next.collect_dead_branch_paths(prefix, paths);
                }
                prefix.pop();
            }
        }
    }

    fn has_reachable_leaf(&self) -> bool {
        self.choices.iter().any(|choice| {
            choice
                .next
                .as_ref()
                .is_none_or(|next| next.has_reachable_leaf())
        })
    }
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

    #[test]
    fn leaf_choices_are_not_dead_branches() {
        let tree = ActionTree::flat(
            FreshnessToken(1),
            vec![
                ActionChoice::leaf("first", "First", "First"),
                ActionChoice::leaf("second", "Second", "Second"),
            ],
        );

        assert!(tree.dead_branch_paths().is_empty());
        assert!(!tree.has_dead_branches());
    }

    #[test]
    fn nested_choice_with_reachable_leaf_is_not_dead_branch() {
        let mut parent = ActionChoice::leaf("parent", "Parent", "Parent");
        parent.next = Some(Box::new(ActionNode {
            choices: vec![ActionChoice::leaf("leaf", "Leaf", "Leaf")],
        }));
        let tree = ActionTree::flat(FreshnessToken(1), vec![parent]);

        assert!(tree.dead_branch_paths().is_empty());
    }

    #[test]
    fn empty_next_node_reports_parent_path() {
        let mut parent = ActionChoice::leaf("parent", "Parent", "Parent");
        parent.next = Some(Box::new(ActionNode {
            choices: Vec::new(),
        }));
        let tree = ActionTree::flat(FreshnessToken(1), vec![parent]);

        assert_eq!(tree.dead_branch_paths(), vec![vec!["parent".to_owned()]]);
        assert!(tree.has_dead_branches());
    }

    #[test]
    fn recursively_dead_subtree_reports_top_dead_branch() {
        let mut child = ActionChoice::leaf("child", "Child", "Child");
        child.next = Some(Box::new(ActionNode {
            choices: Vec::new(),
        }));
        let mut parent = ActionChoice::leaf("parent", "Parent", "Parent");
        parent.next = Some(Box::new(ActionNode {
            choices: vec![child],
        }));
        let tree = ActionTree::flat(FreshnessToken(1), vec![parent]);

        assert_eq!(tree.dead_branch_paths(), vec![vec!["parent".to_owned()]]);
    }

    #[test]
    fn dead_branch_path_order_is_deterministic() {
        let mut first = ActionChoice::leaf("first", "First", "First");
        first.next = Some(Box::new(ActionNode {
            choices: Vec::new(),
        }));
        let mut second = ActionChoice::leaf("second", "Second", "Second");
        second.next = Some(Box::new(ActionNode {
            choices: Vec::new(),
        }));
        let tree = ActionTree::flat(FreshnessToken(1), vec![first, second]);

        let expected = vec![vec!["first".to_owned()], vec!["second".to_owned()]];
        assert_eq!(tree.dead_branch_paths(), expected);
        assert_eq!(tree.dead_branch_paths(), expected);
    }
}
