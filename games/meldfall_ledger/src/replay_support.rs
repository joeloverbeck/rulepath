//! Replay, fixture, export, and hash support for Meldfall Ledger.
//!
//! This module keeps replay/export surfaces deterministic and viewer-scoped.

use engine_core::{
    ActionTree, ActionTreeEncodingVersion, EffectEnvelope, HashValue, SchemaVersion,
    StableSerialize, Viewer,
};

use crate::{
    effects::{effect_stable_string, MeldfallEffect, MeldfallEffectEnvelope},
    ids::{DATA_VERSION_LABEL, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    state::MatchState,
    visibility::{project_action_tree_for_viewer, project_effects_for_viewer, project_view},
};

pub const TRACE_SCHEMA_VERSION: SchemaVersion = SchemaVersion(1);
pub const EXPORT_FORMAT_VERSION: u32 = 2;
pub const FIXTURE_COMPLETION_PROFILE: &str = "meldfall_ledger_fixture_completion_v1";
pub const VIEWER_EXPORT_CLASS: &str = "meldfall_ledger_viewer_scoped_observation_v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplaySkeletonRecord {
    pub schema_version: SchemaVersion,
    pub export_format_version: u32,
    pub game_id: &'static str,
    pub rules_version_label: &'static str,
    pub data_version_label: &'static str,
    pub action_tree_encoding: ActionTreeEncodingVersion,
    pub fixture_completion_profile: &'static str,
    pub state_summary: String,
    pub action_tree_hash: HashValue,
    pub effect_hash: HashValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewerReplayExport {
    pub schema_version: u32,
    pub export_format_version: u32,
    pub export_class: String,
    pub viewer: String,
    pub game_id: String,
    pub rules_version: String,
    pub data_version: String,
    pub variant: String,
    pub steps: Vec<ViewerReplayStep>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewerReplayStep {
    pub step_index: usize,
    pub view_summary: String,
    pub action_tree_hash: HashValue,
    pub effect_summaries: Vec<String>,
    pub terminal: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewerReplayTimeline {
    pub viewer: String,
    pub steps: Vec<ViewerReplayStep>,
}

impl ReplaySkeletonRecord {
    pub fn stable_string(&self) -> String {
        format!(
            "replay|schema={}|export={}|game={}|rules={}|data={}|action_encoding={}|fixture_profile={}|state_hash={}|action_tree_hash={:016x}|effect_hash={:016x}",
            self.schema_version.0,
            self.export_format_version,
            self.game_id,
            self.rules_version_label,
            self.data_version_label,
            action_tree_encoding_label(self.action_tree_encoding),
            self.fixture_completion_profile,
            HashValue::from_stable_bytes(self.state_summary.as_bytes()).0,
            self.action_tree_hash.0,
            self.effect_hash.0
        )
    }
}

pub fn replay_skeleton_record(
    state: &MatchState,
    action_tree: &ActionTree,
    effects: &[MeldfallEffectEnvelope],
) -> ReplaySkeletonRecord {
    ReplaySkeletonRecord {
        schema_version: TRACE_SCHEMA_VERSION,
        export_format_version: EXPORT_FORMAT_VERSION,
        game_id: GAME_ID,
        rules_version_label: RULES_VERSION_LABEL,
        data_version_label: DATA_VERSION_LABEL,
        action_tree_encoding: ActionTreeEncodingVersion::V1,
        fixture_completion_profile: FIXTURE_COMPLETION_PROFILE,
        state_summary: state.stable_internal_summary(),
        action_tree_hash: action_tree.stable_hash(ActionTreeEncodingVersion::V1),
        effect_hash: effects_hash(effects),
    }
}

pub fn export_viewer_snapshot(
    state: &MatchState,
    action_tree: &ActionTree,
    effects: &[EffectEnvelope<MeldfallEffect>],
    viewer: &Viewer,
) -> ViewerReplayExport {
    let projected_tree = project_action_tree_for_viewer(action_tree, state, viewer);
    let projected_effects = project_effects_for_viewer(effects, viewer);
    ViewerReplayExport {
        schema_version: TRACE_SCHEMA_VERSION.0,
        export_format_version: EXPORT_FORMAT_VERSION,
        export_class: VIEWER_EXPORT_CLASS.to_owned(),
        viewer: viewer_label(viewer),
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        data_version: DATA_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        steps: vec![ViewerReplayStep {
            step_index: 0,
            view_summary: project_view(state, viewer).stable_string(),
            action_tree_hash: projected_tree.stable_hash(ActionTreeEncodingVersion::V1),
            effect_summaries: projected_effects.iter().map(effect_stable_string).collect(),
            terminal: state.terminal.is_some(),
        }],
    }
}

pub fn import_viewer_export(
    export: &ViewerReplayExport,
    requested_viewer: &Viewer,
) -> Result<ViewerReplayTimeline, String> {
    validate_viewer_export(export)?;
    let requested_label = viewer_label(requested_viewer);
    if export.viewer != requested_label {
        return Err(format!(
            "viewer scope mismatch: export is {} but requested {}",
            export.viewer, requested_label
        ));
    }
    Ok(ViewerReplayTimeline {
        viewer: export.viewer.clone(),
        steps: export.steps.clone(),
    })
}

pub fn validate_viewer_export(export: &ViewerReplayExport) -> Result<(), String> {
    if export.schema_version != TRACE_SCHEMA_VERSION.0 {
        return Err(format!(
            "unsupported schema version {}",
            export.schema_version
        ));
    }
    if export.export_format_version != EXPORT_FORMAT_VERSION {
        return Err(format!(
            "unsupported export format {}",
            export.export_format_version
        ));
    }
    if export.export_class != VIEWER_EXPORT_CLASS {
        return Err(format!("unsupported export class {}", export.export_class));
    }
    if export.game_id != GAME_ID {
        return Err(format!("unsupported game {}", export.game_id));
    }
    if export.rules_version != RULES_VERSION_LABEL {
        return Err(format!("unsupported rules {}", export.rules_version));
    }
    if export.data_version != DATA_VERSION_LABEL {
        return Err(format!("unsupported data {}", export.data_version));
    }
    if export.variant != VARIANT_ID {
        return Err(format!("unsupported variant {}", export.variant));
    }
    if export.steps.is_empty() {
        return Err("viewer export must contain at least one step".to_owned());
    }
    Ok(())
}

pub fn effects_hash(effects: &[MeldfallEffectEnvelope]) -> HashValue {
    let bytes = effects
        .iter()
        .map(effect_stable_string)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

impl ViewerReplayExport {
    pub fn stable_string(&self) -> String {
        format!(
            "schema={};export={};class={};viewer={};game={};rules={};data={};variant={};steps=[{}]",
            self.schema_version,
            self.export_format_version,
            self.export_class,
            self.viewer,
            self.game_id,
            self.rules_version,
            self.data_version,
            self.variant,
            self.steps
                .iter()
                .map(ViewerReplayStep::stable_string)
                .collect::<Vec<_>>()
                .join(";")
        )
    }

    pub fn stable_hash(&self) -> HashValue {
        HashValue::from_stable_bytes(self.stable_string().as_bytes())
    }

    pub fn to_json(&self) -> String {
        let steps = self
            .steps
            .iter()
            .map(ViewerReplayStep::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema_version\":{},\"export_format_version\":{},\"export_class\":\"{}\",\"viewer\":\"{}\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"data_version\":\"{}\",\"variant\":\"{}\",\"steps\":[{}]}}",
            self.schema_version,
            self.export_format_version,
            escape_json(&self.export_class),
            escape_json(&self.viewer),
            escape_json(&self.game_id),
            escape_json(&self.rules_version),
            escape_json(&self.data_version),
            escape_json(&self.variant),
            steps
        )
    }
}

impl StableSerialize for ViewerReplayExport {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_string().into_bytes()
    }
}

impl ViewerReplayStep {
    pub fn stable_string(&self) -> String {
        format!(
            "step={}:view_hash={:016x}:action_hash={:016x}:effects=[{}]:terminal={}",
            self.step_index,
            HashValue::from_stable_bytes(self.view_summary.as_bytes()).0,
            self.action_tree_hash.0,
            self.effect_summaries.join("|"),
            self.terminal
        )
    }

    fn to_json(&self) -> String {
        let effects = self
            .effect_summaries
            .iter()
            .map(|effect| format!("\"{}\"", escape_json(effect)))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"step_index\":{},\"view_summary\":\"{}\",\"action_tree_hash\":\"{:016x}\",\"effect_summaries\":[{}],\"terminal\":{}}}",
            self.step_index,
            escape_json(&self.view_summary),
            self.action_tree_hash.0,
            effects,
            self.terminal
        )
    }
}

impl StableSerialize for ViewerReplayStep {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_string().into_bytes()
    }
}

pub const fn action_tree_encoding_label(version: ActionTreeEncodingVersion) -> &'static str {
    match version {
        ActionTreeEncodingVersion::V1 => "action_tree_v1",
    }
}

fn viewer_label(viewer: &Viewer) -> String {
    viewer
        .seat_id
        .as_ref()
        .map(|seat_id| seat_id.0.clone())
        .unwrap_or_else(|| "observer".to_owned())
}

fn escape_json(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| match character {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            other => vec![other],
        })
        .collect()
}
