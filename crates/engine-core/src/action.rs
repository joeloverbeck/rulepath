use crate::{
    ActionPath, HashValue, StableBytesRecordWriter, StableBytesTypeTag, StableBytesWriter,
    StableBytesWriterError,
};

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

    pub fn stable_bytes(&self, version: ActionTreeEncodingVersion) -> Vec<u8> {
        match version {
            ActionTreeEncodingVersion::V1 => self
                .stable_bytes_v1()
                .expect("action tree v1 encoding uses ordered fields and u32 lengths"),
        }
    }

    pub fn stable_hash(&self, version: ActionTreeEncodingVersion) -> HashValue {
        HashValue::from_stable_bytes(&self.stable_bytes(version))
    }

    fn stable_bytes_v1(&self) -> Result<Vec<u8>, StableBytesWriterError> {
        let mut writer = StableBytesWriter::new(
            ActionTreeEncodingVersion::V1.domain(),
            ActionTreeEncodingVersion::V1.surface_version(),
        )?;
        writer.write_u64_field(1, self.freshness_token.0)?;
        writer.write_record_field(2, |record| encode_action_node_v1(record, &self.root))?;
        Ok(writer.into_bytes())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ActionTreeEncodingVersion {
    V1,
}

impl ActionTreeEncodingVersion {
    pub const fn domain(self) -> &'static [u8] {
        match self {
            Self::V1 => b"action_tree",
        }
    }

    pub const fn surface_version(self) -> u32 {
        match self {
            Self::V1 => 1,
        }
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

fn encode_action_node_v1(
    record: &mut StableBytesRecordWriter,
    node: &ActionNode,
) -> Result<(), StableBytesWriterError> {
    let choices = node
        .choices
        .iter()
        .map(action_choice_v1_record)
        .collect::<Result<Vec<_>, _>>()?;
    record.write_sequence_field(1, choices)
}

fn action_choice_v1_record(choice: &ActionChoice) -> Result<Vec<u8>, StableBytesWriterError> {
    let mut record = StableBytesRecordWriter::new();
    record.write_string_field(1, &choice.segment)?;
    record.write_string_field(2, &choice.label)?;
    record.write_string_field(3, &choice.accessibility_label)?;
    record.write_sequence_field(4, action_metadata_v1_records(&choice.metadata)?)?;
    record.write_sequence_field(5, choice.tags.iter().map(String::as_bytes))?;
    record.write_enum_field(6, action_preview_v1_discriminant(choice.preview))?;
    if let Some(next) = &choice.next {
        let mut child = StableBytesRecordWriter::new();
        encode_action_node_v1(&mut child, next)?;
        record.write_some_field(7, StableBytesTypeTag::Record, &child.into_bytes())?;
    } else {
        record.write_none_field(7)?;
    }
    Ok(record.into_bytes())
}

fn action_metadata_v1_records(
    metadata: &[ActionMetadata],
) -> Result<Vec<Vec<u8>>, StableBytesWriterError> {
    metadata
        .iter()
        .map(|entry| {
            let mut record = StableBytesRecordWriter::new();
            record.write_string_field(1, &entry.key)?;
            record.write_string_field(2, &entry.value)?;
            Ok(record.into_bytes())
        })
        .collect()
}

const fn action_preview_v1_discriminant(preview: ActionPreview) -> u32 {
    match preview {
        ActionPreview::Unavailable => 0,
        ActionPreview::Available => 1,
    }
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
    fn action_tree_v1_empty_tree_matches_golden_bytes() {
        let tree = ActionTree::flat(FreshnessToken(7), Vec::new());

        assert_eq!(
            tree.stable_bytes(ActionTreeEncodingVersion::V1),
            vec![
                b'R', b'P', b'S', b'B', 1, 0, 11, 0, 0, 0, b'a', b'c', b't', b'i', b'o', b'n',
                b'_', b't', b'r', b'e', b'e', 1, 0, 0, 0, 1, 0, 0, 0, 5, 8, 0, 0, 0, 7, 0, 0, 0, 0,
                0, 0, 0, 2, 0, 0, 0, 9, 13, 0, 0, 0, 1, 0, 0, 0, 10, 4, 0, 0, 0, 0, 0, 0, 0,
            ]
        );
        assert_eq!(
            tree.stable_hash(ActionTreeEncodingVersion::V1),
            HashValue::from_stable_bytes(&tree.stable_bytes(ActionTreeEncodingVersion::V1))
        );
        assert_eq!(
            tree.stable_hash(ActionTreeEncodingVersion::V1),
            HashValue(3_765_532_138_415_974_872)
        );
    }

    #[test]
    fn action_tree_v1_flat_tree_preserves_choice_vector_order() {
        let tree = ActionTree::flat(
            FreshnessToken(3),
            vec![
                ActionChoice::leaf("add-1", "Add 1", "Add one"),
                ActionChoice::leaf("add-2", "Add 2", "Add two"),
            ],
        );
        let reversed = ActionTree::flat(
            FreshnessToken(3),
            vec![
                ActionChoice::leaf("add-2", "Add 2", "Add two"),
                ActionChoice::leaf("add-1", "Add 1", "Add one"),
            ],
        );

        assert_eq!(
            tree.stable_bytes(ActionTreeEncodingVersion::V1),
            expected_action_tree_v1_bytes(
                3,
                vec![
                    expected_choice_v1_record(
                        "add-1",
                        "Add 1",
                        "Add one",
                        Vec::new(),
                        Vec::new(),
                        ActionPreview::Unavailable,
                        None,
                    ),
                    expected_choice_v1_record(
                        "add-2",
                        "Add 2",
                        "Add two",
                        Vec::new(),
                        Vec::new(),
                        ActionPreview::Unavailable,
                        None,
                    ),
                ],
            )
        );
        assert_ne!(
            tree.stable_bytes(ActionTreeEncodingVersion::V1),
            reversed.stable_bytes(ActionTreeEncodingVersion::V1)
        );
    }

    #[test]
    fn action_tree_v1_metadata_tags_and_preview_are_framed_in_order() {
        let mut choice = ActionChoice::leaf("choose", "Choose", "Choose option");
        choice.metadata = vec![
            ActionMetadata {
                key: "cost".to_owned(),
                value: "1".to_owned(),
            },
            ActionMetadata {
                key: "phase".to_owned(),
                value: "main".to_owned(),
            },
        ];
        choice.tags = vec!["primary".to_owned(), "fast".to_owned()];
        choice.preview = ActionPreview::Available;
        let tree = ActionTree::flat(FreshnessToken(8), vec![choice]);

        assert_eq!(
            tree.stable_bytes(ActionTreeEncodingVersion::V1),
            expected_action_tree_v1_bytes(
                8,
                vec![expected_choice_v1_record(
                    "choose",
                    "Choose",
                    "Choose option",
                    vec![("cost", "1"), ("phase", "main")],
                    vec!["primary", "fast"],
                    ActionPreview::Available,
                    None,
                )],
            )
        );
    }

    #[test]
    fn action_tree_v1_recursive_tree_frames_child_and_hashes_deterministically() {
        let mut parent = ActionChoice::leaf("parent", "Parent", "Parent");
        parent.next = Some(Box::new(ActionNode {
            choices: vec![ActionChoice::leaf("child", "Child", "Child")],
        }));
        let tree = ActionTree::flat(FreshnessToken(9), vec![parent]);
        let child = expected_node_v1_record(vec![expected_choice_v1_record(
            "child",
            "Child",
            "Child",
            Vec::new(),
            Vec::new(),
            ActionPreview::Unavailable,
            None,
        )]);
        let expected = expected_action_tree_v1_bytes(
            9,
            vec![expected_choice_v1_record(
                "parent",
                "Parent",
                "Parent",
                Vec::new(),
                Vec::new(),
                ActionPreview::Unavailable,
                Some(child),
            )],
        );

        assert_eq!(tree.stable_bytes(ActionTreeEncodingVersion::V1), expected);
        assert_eq!(
            tree.stable_hash(ActionTreeEncodingVersion::V1),
            tree.stable_hash(ActionTreeEncodingVersion::V1)
        );
        assert_eq!(
            tree.stable_hash(ActionTreeEncodingVersion::V1),
            HashValue::from_stable_bytes(&tree.stable_bytes(ActionTreeEncodingVersion::V1))
        );
        assert_eq!(
            tree.stable_hash(ActionTreeEncodingVersion::V1),
            HashValue(3_065_934_150_664_393_470)
        );
    }

    #[test]
    fn action_tree_v1_omits_non_contract_disabled_fields() {
        let tree = ActionTree::flat(
            FreshnessToken(1),
            vec![ActionChoice::leaf("segment", "Label", "Accessible")],
        );
        let bytes = tree.stable_bytes(ActionTreeEncodingVersion::V1);

        assert!(!bytes
            .windows(b"disabled".len())
            .any(|window| window == b"disabled"));
        assert!(!bytes
            .windows(b"reason".len())
            .any(|window| window == b"reason"));
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

    fn expected_action_tree_v1_bytes(freshness_token: u64, choices: Vec<Vec<u8>>) -> Vec<u8> {
        let mut writer = StableBytesWriter::new(
            ActionTreeEncodingVersion::V1.domain(),
            ActionTreeEncodingVersion::V1.surface_version(),
        )
        .expect("writer");
        writer
            .write_u64_field(1, freshness_token)
            .expect("freshness");
        writer
            .write_record_field(2, |record| {
                record.write_sequence_field(1, choices)?;
                Ok(())
            })
            .expect("root");
        writer.into_bytes()
    }

    fn expected_node_v1_record(choices: Vec<Vec<u8>>) -> Vec<u8> {
        let mut record = StableBytesRecordWriter::new();
        record.write_sequence_field(1, choices).expect("choices");
        record.into_bytes()
    }

    fn expected_choice_v1_record(
        segment: &str,
        label: &str,
        accessibility_label: &str,
        metadata: Vec<(&str, &str)>,
        tags: Vec<&str>,
        preview: ActionPreview,
        next: Option<Vec<u8>>,
    ) -> Vec<u8> {
        let mut record = StableBytesRecordWriter::new();
        record.write_string_field(1, segment).expect("segment");
        record.write_string_field(2, label).expect("label");
        record
            .write_string_field(3, accessibility_label)
            .expect("accessibility label");
        record
            .write_sequence_field(4, expected_metadata_v1_records(metadata))
            .expect("metadata");
        record
            .write_sequence_field(5, tags.into_iter().map(str::as_bytes))
            .expect("tags");
        record
            .write_enum_field(6, action_preview_v1_discriminant(preview))
            .expect("preview");
        if let Some(next) = next {
            record
                .write_some_field(7, StableBytesTypeTag::Record, &next)
                .expect("next");
        } else {
            record.write_none_field(7).expect("next");
        }
        record.into_bytes()
    }

    fn expected_metadata_v1_records(metadata: Vec<(&str, &str)>) -> Vec<Vec<u8>> {
        metadata
            .into_iter()
            .map(|(key, value)| {
                let mut record = StableBytesRecordWriter::new();
                record.write_string_field(1, key).expect("key");
                record.write_string_field(2, value).expect("value");
                record.into_bytes()
            })
            .collect()
    }
}
