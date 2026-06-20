//! Replay-document data types and their JSON parsers.
//!
//! Models a parsed replay document (`ParsedReplayDocument`) and the
//! viewer-scoped public timeline (`PublicTimelineReplay` / `PublicTimelineStep`)
//! that hidden-information games export, plus the parsers that read them back
//! from JSON (built on the primitives in [`crate::json_parse`]). The per-game
//! replay exporters/importers in the crate root construct and consume these
//! types. Glob-imported at the crate root.

use crate::json_parse::{
    array_items, bool_field, number_field, reject_unknown_root_fields, string_array_field,
    string_field, validate_json_object,
};
use crate::AppliedCommand;

#[derive(Clone, Debug)]
pub(crate) struct PublicTimelineReplay {
    pub(crate) viewer: String,
    pub(crate) steps: Vec<PublicTimelineStep>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PublicTimelineStep {
    pub(crate) step_index: usize,
    pub(crate) public_view_summary: String,
    pub(crate) public_effects: Vec<String>,
    pub(crate) redacted_command_summary: String,
    pub(crate) terminal: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct ParsedReplayDocument {
    pub(crate) schema_version: u64,
    pub(crate) game_id: String,
    pub(crate) rules_version: String,
    pub(crate) seed: u64,
    pub(crate) commands: Vec<AppliedCommand>,
}

pub(crate) fn parse_replay_document(input: &str) -> Result<ParsedReplayDocument, String> {
    validate_json_object(input)?;
    reject_unknown_root_fields(
        input,
        &[
            "schema_version",
            "trace_id",
            "fixture_kind",
            "purpose",
            "note",
            "migration_update_note",
            "game_id",
            "rules_version",
            "engine_version",
            "data_version",
            "seed",
            "variant",
            "options",
            "seats",
            "commands",
            "checkpoints",
            "expected_state_hashes",
            "expected_effect_hashes",
            "expected_action_tree_hashes",
            "expected_public_view_hashes",
            "expected_private_view_hashes",
            "expected_replay_hashes",
            "expected_diagnostic_hashes",
            "expected_public_export_hashes",
            "expected_diagnostics",
            "expected_outcome",
            "expected_terminal_state",
            "not_applicable",
        ],
    )?;

    let commands = array_items(input, "commands")?
        .into_iter()
        .map(|command| parse_replay_command(&command))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ParsedReplayDocument {
        schema_version: number_field(input, "schema_version")?,
        game_id: string_field(input, "game_id")?,
        rules_version: string_field(input, "rules_version")?,
        seed: number_field(input, "seed")?,
        commands,
    })
}

pub(crate) fn parse_public_replay_steps(input: &str) -> Result<Vec<PublicTimelineStep>, String> {
    array_items(input, "steps")?
        .into_iter()
        .map(|item| parse_public_replay_step(&item))
        .collect()
}

pub(crate) fn parse_frontier_public_replay_steps(
    input: &str,
) -> Result<Vec<PublicTimelineStep>, String> {
    array_items(input, "steps")?
        .into_iter()
        .map(|item| parse_frontier_public_replay_step(&item))
        .collect()
}

pub(crate) fn parse_public_replay_step(input: &str) -> Result<PublicTimelineStep, String> {
    validate_json_object(input)?;
    reject_unknown_root_fields(
        input,
        &[
            "step_index",
            "public_view_summary",
            "public_effects",
            "redacted_command_summary",
            "terminal",
        ],
    )?;
    let step_index = number_field(input, "step_index")?
        .try_into()
        .map_err(|_| "step_index does not fit usize".to_owned())?;
    Ok(PublicTimelineStep {
        step_index,
        public_view_summary: string_field(input, "public_view_summary")?,
        public_effects: string_array_field(input, "public_effects")?,
        redacted_command_summary: string_field(input, "redacted_command_summary")?,
        terminal: bool_field(input, "terminal")?,
    })
}

pub(crate) fn parse_frontier_public_replay_step(input: &str) -> Result<PublicTimelineStep, String> {
    validate_json_object(input)?;
    reject_unknown_root_fields(
        input,
        &[
            "step_index",
            "public_view_summary",
            "public_effects",
            "command_summary",
            "terminal",
        ],
    )?;
    let step_index = number_field(input, "step_index")?
        .try_into()
        .map_err(|_| "step_index does not fit usize".to_owned())?;
    Ok(PublicTimelineStep {
        step_index,
        public_view_summary: string_field(input, "public_view_summary")?,
        public_effects: string_array_field(input, "public_effects")?,
        redacted_command_summary: string_field(input, "command_summary")?,
        terminal: bool_field(input, "terminal")?,
    })
}

pub(crate) fn parse_replay_command(input: &str) -> Result<AppliedCommand, String> {
    validate_json_object(input)?;
    reject_unknown_root_fields(
        input,
        &[
            "index",
            "actor_seat",
            "action_path",
            "freshness_token",
            "expect",
            "expected_diagnostic_code",
            "producer",
        ],
    )?;
    let expect = string_field(input, "expect")?;
    if expect != "applied" {
        return Err(format!("unsupported command expectation `{expect}`"));
    }
    let actor_seat = string_field(input, "actor_seat")?;
    let freshness_token = string_field(input, "freshness_token")?
        .parse::<u64>()
        .map_err(|_| "freshness_token must be a u64 string".to_owned())?;
    let action_path = string_array_field(input, "action_path")?;
    if action_path.is_empty() {
        return Err("replay commands must have an action path".to_owned());
    }
    Ok(AppliedCommand {
        actor_seat,
        action_path,
        freshness_token,
    })
}
