//! JSON serialization for engine-core legal-action trees.
//!
//! This is generic over every game: it renders `ActionTree` / `ActionChoice`
//! (including nested choices, tags, and the optional presentation payload that
//! games attach via action metadata) into the browser-facing JSON shape. The
//! per-game bridges call [`action_tree_json`] / [`empty_action_tree_json`] and
//! read presentation metadata via [`action_metadata_value`].

use engine_core::{ActionChoice, ActionTree, FreshnessToken};

use crate::json::escape_json;

pub(crate) fn action_tree_json(tree: &ActionTree) -> String {
    let choices = tree
        .root
        .choices
        .iter()
        .map(action_choice_json)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"freshness_token\":{},\"choices\":[{}]}}",
        tree.freshness_token.0, choices
    )
}

pub(crate) fn empty_action_tree_json(freshness_token: FreshnessToken) -> String {
    action_tree_json(&ActionTree::flat(freshness_token, Vec::new()))
}

fn action_choice_json(choice: &ActionChoice) -> String {
    let metadata = choice
        .metadata
        .iter()
        .map(|entry| {
            format!(
                "{{\"key\":\"{}\",\"value\":\"{}\"}}",
                escape_json(&entry.key),
                escape_json(&entry.value)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let tags = choice
        .tags
        .iter()
        .map(|tag| format!("\"{}\"", escape_json(tag)))
        .collect::<Vec<_>>()
        .join(",");
    let next = choice.next.as_ref().map_or_else(
        || "null".to_owned(),
        |node| {
            let choices = node
                .choices
                .iter()
                .map(action_choice_json)
                .collect::<Vec<_>>()
                .join(",");
            format!("{{\"choices\":[{choices}]}}")
        },
    );
    let presentation = action_choice_presentation_json(choice).unwrap_or_else(|| "null".to_owned());
    format!(
        "{{\"segment\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\",\"metadata\":[{}],\"presentation\":{},\"tags\":[{}],\"next\":{}}}",
        escape_json(&choice.segment),
        escape_json(&choice.label),
        escape_json(&choice.accessibility_label),
        metadata,
        presentation,
        tags,
        next
    )
}

fn action_choice_presentation_json(choice: &ActionChoice) -> Option<String> {
    let segment = action_metadata_value(choice, "presentation_segment")?;
    let label = action_metadata_value(choice, "presentation_label")?;
    let helper_text = action_metadata_value(choice, "presentation_helper_text")?;
    let accessibility_label = action_metadata_value(choice, "presentation_accessibility_label")?;
    let rows = action_choice_presentation_rows_json(choice);
    Some(format!(
        "{{\"segment\":\"{}\",\"label\":\"{}\",\"helper_text\":\"{}\",\"accessibility_label\":\"{}\",\"display_rows\":[{}]}}",
        escape_json(segment),
        escape_json(label),
        escape_json(helper_text),
        escape_json(accessibility_label),
        rows
    ))
}

fn action_choice_presentation_rows_json(choice: &ActionChoice) -> String {
    (0..)
        .map_while(|index| {
            let label = action_metadata_value(choice, &format!("presentation_row_{index}_label"))?;
            let value = action_metadata_value(choice, &format!("presentation_row_{index}_value"))?;
            let tone = action_metadata_value(choice, &format!("presentation_row_{index}_tone"))?;
            Some(format!(
                "{{\"label\":\"{}\",\"value\":\"{}\",\"tone\":\"{}\"}}",
                escape_json(label),
                escape_json(value),
                escape_json(tone)
            ))
        })
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn action_metadata_value<'a>(choice: &'a ActionChoice, key: &str) -> Option<&'a str> {
    choice
        .metadata
        .iter()
        .find(|entry| entry.key == key)
        .map(|entry| entry.value.as_str())
}
