use engine_core::{Actor, SeatId, StableSerialize, Viewer};

use crate::{
    actions::legal_action_tree,
    effects::VowTideEffect,
    ids::{GAME_ID, RULES_VERSION_LABEL},
    state::VowTideState,
    visibility::{filter_effects_for_viewer, project_view},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplaySnapshot {
    pub schema_version: u32,
    pub game_id: String,
    pub rules_version_label: String,
    pub state_hash: u64,
    pub observer_view_hash: u64,
    pub seat_view_hashes: Vec<(String, u64)>,
    pub action_tree_hashes: Vec<(String, u64)>,
    pub effect_hash: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewerExport {
    pub schema_version: u32,
    pub game_id: String,
    pub rules_version_label: String,
    pub viewer: String,
    pub observations: Vec<String>,
}

impl StableSerialize for ViewerExport {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

impl ViewerExport {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};game={};rules={};viewer={};observations={}",
            self.schema_version,
            self.game_id,
            self.rules_version_label,
            self.viewer,
            self.observations.join("|")
        )
    }
}

pub fn snapshot(state: &VowTideState, effects: &[VowTideEffect]) -> ReplaySnapshot {
    let observer = Viewer { seat_id: None };
    ReplaySnapshot {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        state_hash: stable_hash(state.stable_internal_summary().as_bytes()),
        observer_view_hash: stable_hash(&project_view(state, &observer).stable_bytes()),
        seat_view_hashes: state
            .seats
            .iter()
            .map(|seat_id| {
                let viewer = Viewer {
                    seat_id: Some(seat_id.clone()),
                };
                (
                    seat_id.0.clone(),
                    stable_hash(&project_view(state, &viewer).stable_bytes()),
                )
            })
            .collect(),
        action_tree_hashes: state
            .seats
            .iter()
            .map(|seat_id| {
                let actor = Actor {
                    seat_id: seat_id.clone(),
                };
                (
                    seat_id.0.clone(),
                    stable_hash(format!("{:?}", legal_action_tree(state, &actor)).as_bytes()),
                )
            })
            .collect(),
        effect_hash: stable_hash(format!("{effects:?}").as_bytes()),
    }
}

pub fn export_for_viewer(
    state: &VowTideState,
    effects: &[VowTideEffect],
    viewer: &Viewer,
) -> ViewerExport {
    let view = project_view(state, viewer);
    let visible_effects = filter_effects_for_viewer(effects, viewer);
    ViewerExport {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        viewer: viewer
            .seat_id
            .as_ref()
            .map(|seat| seat.0.clone())
            .unwrap_or_else(|| "observer".to_owned()),
        observations: vec![
            format!("view:{}", view.stable_summary()),
            format!("effects:{visible_effects:?}"),
        ],
    }
}

pub fn import_viewer_export(export: &ViewerExport) -> Result<ViewerExport, String> {
    if export.schema_version != 1 {
        return Err(format!(
            "unsupported vow_tide viewer export schema {}",
            export.schema_version
        ));
    }
    if export.game_id != GAME_ID {
        return Err(format!(
            "unexpected viewer export game `{}`",
            export.game_id
        ));
    }
    Ok(export.clone())
}

pub fn observer() -> Viewer {
    Viewer { seat_id: None }
}

pub fn seat_viewer(seat_id: impl Into<String>) -> Viewer {
    Viewer {
        seat_id: Some(SeatId(seat_id.into())),
    }
}

pub fn stable_hash(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
