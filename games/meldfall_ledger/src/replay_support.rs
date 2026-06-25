//! Replay, fixture, export, and hash support for Meldfall Ledger.
//!
//! This ticket establishes Trace Schema v1 labels and deterministic skeleton
//! records. Later tickets add complete viewer-scoped export/import.

use engine_core::{ActionTree, ActionTreeEncodingVersion, HashValue, SchemaVersion};

use crate::{
    effects::{effect_stable_string, MeldfallEffectEnvelope},
    ids::{DATA_VERSION_LABEL, GAME_ID, RULES_VERSION_LABEL},
    state::MatchState,
};

pub const TRACE_SCHEMA_VERSION: SchemaVersion = SchemaVersion(1);
pub const EXPORT_FORMAT_VERSION: u32 = 2;
pub const FIXTURE_COMPLETION_PROFILE: &str = "meldfall_ledger_fixture_completion_v1";

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

pub fn effects_hash(effects: &[MeldfallEffectEnvelope]) -> HashValue {
    let bytes = effects
        .iter()
        .map(effect_stable_string)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub const fn action_tree_encoding_label(version: ActionTreeEncodingVersion) -> &'static str {
    match version {
        ActionTreeEncodingVersion::V1 => "action_tree_v1",
    }
}
