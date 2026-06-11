//! Replay/export support for Flood Watch.
//!
//! Internal traces are native-test authority and may carry the full event-deck
//! order. Public replay exports are viewer-scoped observations and never carry
//! undrawn deck order.

use engine_core::{CommandEnvelope, StableSerialize, Viewer};

use crate::{
    effects::FloodWatchEffectEnvelope,
    ids::{GAME_ID, RULES_VERSION_LABEL},
    state::FloodWatchState,
    visibility::{filter_effects_for_viewer, project_view, public_effect_text},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FloodWatchInternalTrace {
    pub schema_version: u32,
    pub game_id: String,
    pub rules_version_label: String,
    pub seed: u64,
    pub variant: String,
    pub full_deck_order: Vec<String>,
    pub initial_state_summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicReplayExport {
    pub schema_version: u32,
    pub game_id: String,
    pub rules_version_label: String,
    pub variant: String,
    pub viewer: String,
    pub steps: Vec<PublicReplayStep>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicReplayStep {
    pub step_index: usize,
    pub public_view_summary: String,
    pub public_effects: Vec<String>,
    pub redacted_command_summary: String,
    pub terminal: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportedPublicReplay {
    pub raw_json: String,
}

pub fn generate_internal_full_trace(seed: u64, state: &FloodWatchState) -> FloodWatchInternalTrace {
    FloodWatchInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        seed,
        variant: state.variant.id.clone(),
        full_deck_order: state
            .event_deck_internal()
            .iter()
            .map(|card| card.stable_id())
            .collect(),
        initial_state_summary: state.stable_summary(),
    }
}

pub fn public_replay_step(
    step_index: usize,
    state_after_step: &FloodWatchState,
    command: &CommandEnvelope,
    effects: &[FloodWatchEffectEnvelope],
    viewer: &Viewer,
) -> PublicReplayStep {
    PublicReplayStep {
        step_index,
        public_view_summary: project_view(state_after_step, viewer).stable_summary(),
        public_effects: filter_effects_for_viewer(effects, viewer)
            .iter()
            .map(|effect| public_effect_text(&effect.payload))
            .collect(),
        redacted_command_summary: redacted_command_summary(command),
        terminal: state_after_step.terminal_outcome.is_some(),
    }
}

pub fn export_public_replay(
    variant: impl Into<String>,
    viewer: &Viewer,
    steps: Vec<PublicReplayStep>,
) -> PublicReplayExport {
    PublicReplayExport {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        variant: variant.into(),
        viewer: viewer
            .seat_id
            .as_ref()
            .map(|seat| seat.0.clone())
            .unwrap_or_else(|| "observer".to_owned()),
        steps,
    }
}

pub fn import_public_export(export: &PublicReplayExport) -> ImportedPublicReplay {
    ImportedPublicReplay {
        raw_json: export.to_json(),
    }
}

pub fn import_public_export_json(input: &str) -> Result<ImportedPublicReplay, String> {
    reject_unknown_json_keys(
        input,
        &[
            "schema_version",
            "game_id",
            "rules_version_label",
            "variant",
            "viewer",
            "steps",
            "step_index",
            "public_view_summary",
            "public_effects",
            "redacted_command_summary",
            "terminal",
        ],
    )?;
    Ok(ImportedPublicReplay {
        raw_json: input.to_owned(),
    })
}

impl FloodWatchInternalTrace {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};game={};rules={};seed={};variant={};deck={};initial={}",
            self.schema_version,
            self.game_id,
            self.rules_version_label,
            self.seed,
            self.variant,
            self.full_deck_order.join(","),
            self.initial_state_summary
        )
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version_label\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"full_deck_order\":[{}],\"initial_state_summary\":\"{}\"}}",
            self.schema_version,
            escape_json(&self.game_id),
            escape_json(&self.rules_version_label),
            self.seed,
            escape_json(&self.variant),
            encode_string_array(&self.full_deck_order),
            escape_json(&self.initial_state_summary),
        )
    }
}

impl StableSerialize for FloodWatchInternalTrace {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

impl PublicReplayExport {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};game={};rules={};variant={};viewer={};steps={}",
            self.schema_version,
            self.game_id,
            self.rules_version_label,
            self.variant,
            self.viewer,
            self.steps
                .iter()
                .map(PublicReplayStep::stable_summary)
                .collect::<Vec<_>>()
                .join("|")
        )
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version_label\":\"{}\",\"variant\":\"{}\",\"viewer\":\"{}\",\"steps\":[{}]}}",
            self.schema_version,
            escape_json(&self.game_id),
            escape_json(&self.rules_version_label),
            escape_json(&self.variant),
            escape_json(&self.viewer),
            self.steps
                .iter()
                .map(PublicReplayStep::to_json)
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl StableSerialize for PublicReplayExport {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

impl PublicReplayStep {
    pub fn stable_summary(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.step_index,
            self.public_view_summary,
            self.public_effects.join(","),
            self.redacted_command_summary,
            self.terminal
        )
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"step_index\":{},\"public_view_summary\":\"{}\",\"public_effects\":[{}],\"redacted_command_summary\":\"{}\",\"terminal\":{}}}",
            self.step_index,
            escape_json(&self.public_view_summary),
            encode_string_array(&self.public_effects),
            escape_json(&self.redacted_command_summary),
            self.terminal
        )
    }
}

impl ImportedPublicReplay {
    pub fn stable_summary(&self) -> String {
        self.raw_json.clone()
    }
}

impl StableSerialize for ImportedPublicReplay {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn redacted_command_summary(command: &CommandEnvelope) -> String {
    format!("{}:action_redacted", command.actor.seat_id.0)
}

fn reject_unknown_json_keys(input: &str, allowed: &[&str]) -> Result<(), String> {
    let bytes = input.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'"' {
            index += 1;
            continue;
        }
        let key_start = index + 1;
        let Some(key_end) = input[key_start..]
            .find('"')
            .map(|offset| key_start + offset)
        else {
            return Err("unterminated JSON string".to_owned());
        };
        let mut cursor = key_end + 1;
        while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor < bytes.len() && bytes[cursor] == b':' {
            let key = &input[key_start..key_end];
            if !allowed.contains(&key) {
                return Err(format!("unknown public replay field `{key}`"));
            }
        }
        index = key_end + 1;
    }
    Ok(())
}

fn encode_string_array(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",")
}

fn escape_json(value: &str) -> String {
    value
        .chars()
        .flat_map(|ch| match ch {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            _ => vec![ch],
        })
        .collect()
}
