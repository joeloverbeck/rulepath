use engine_core::{HashValue, SeatId, Viewer};

use crate::{
    effects::BriarCircuitEffect,
    ids::{BriarCircuitSeat, GAME_ID, RULES_VERSION_LABEL},
    state::BriarCircuitState,
    visibility::{filter_effects_for_viewer, project_action_previews, project_view},
};

pub const VIEWER_EXPORT_VERSION: u32 = 1;
pub const TRACE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayHashSnapshot {
    pub state_hash: HashValue,
    pub public_view_hash: HashValue,
    pub private_view_hashes: Vec<(BriarCircuitSeat, HashValue)>,
    pub public_action_hash: HashValue,
    pub private_action_hashes: Vec<(BriarCircuitSeat, HashValue)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewerExportClass {
    Public,
    SeatPrivate(BriarCircuitSeat),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewerReplayExport {
    pub game_id: String,
    pub rules_version: String,
    pub export_version: u32,
    pub class: ViewerExportClass,
    pub viewer_label: String,
    pub observation_timeline: Vec<String>,
    pub migration_notes: Vec<String>,
}

pub fn replay_hash_snapshot(state: &BriarCircuitState) -> ReplayHashSnapshot {
    ReplayHashSnapshot {
        state_hash: hash_debug(&state.stable_internal_summary()),
        public_view_hash: view_hash(state, &Viewer { seat_id: None }),
        private_view_hashes: BriarCircuitSeat::ALL
            .into_iter()
            .map(|seat| {
                (
                    seat,
                    view_hash(
                        state,
                        &Viewer {
                            seat_id: Some(SeatId(seat.as_str().to_owned())),
                        },
                    ),
                )
            })
            .collect(),
        public_action_hash: action_hash(state, &Viewer { seat_id: None }),
        private_action_hashes: BriarCircuitSeat::ALL
            .into_iter()
            .map(|seat| {
                (
                    seat,
                    action_hash(
                        state,
                        &Viewer {
                            seat_id: Some(SeatId(seat.as_str().to_owned())),
                        },
                    ),
                )
            })
            .collect(),
    }
}

pub fn view_hash(state: &BriarCircuitState, viewer: &Viewer) -> HashValue {
    hash_debug(&project_view(state, viewer))
}

pub fn action_hash(state: &BriarCircuitState, viewer: &Viewer) -> HashValue {
    hash_debug(&project_action_previews(state, viewer))
}

pub fn effect_hash(effects: &[BriarCircuitEffect], viewer: &Viewer) -> HashValue {
    let envelopes: Vec<_> = effects
        .iter()
        .cloned()
        .flat_map(crate::visibility::effect_envelopes)
        .collect();
    hash_debug(&filter_effects_for_viewer(&envelopes, viewer))
}

pub fn export_viewer_timeline(
    state: &BriarCircuitState,
    class: ViewerExportClass,
) -> ViewerReplayExport {
    let viewer = viewer_for_class(&class);
    ViewerReplayExport {
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        export_version: VIEWER_EXPORT_VERSION,
        viewer_label: viewer_label(&class),
        class,
        observation_timeline: vec![format!("{:?}", project_view(state, &viewer))],
        migration_notes: vec!["trace-schema-v1 additive briar_circuit export".to_owned()],
    }
}

pub fn import_viewer_timeline(export: &ViewerReplayExport) -> Result<ViewerReplayExport, String> {
    if export.game_id != GAME_ID {
        return Err("viewer export game_id mismatch".to_owned());
    }
    if export.rules_version != RULES_VERSION_LABEL {
        return Err("viewer export rules_version mismatch".to_owned());
    }
    if export.export_version != VIEWER_EXPORT_VERSION {
        return Err("viewer export version mismatch".to_owned());
    }
    if export.migration_notes.is_empty() {
        return Err("viewer export requires migration notes".to_owned());
    }
    Ok(export.clone())
}

pub fn parse_export_header(input: &str) -> Result<ViewerReplayExport, String> {
    let mut game_id = None;
    let mut rules_version = None;
    let mut viewer = None;
    let mut class = None;

    for raw_line in input.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("malformed export header line: {line}"))?;
        let value = value.trim();
        match key.trim() {
            "game_id" => game_id = Some(value.to_owned()),
            "rules_version" => rules_version = Some(value.to_owned()),
            "viewer" => viewer = Some(value.to_owned()),
            "class" => {
                class = Some(match value {
                    "public" => ViewerExportClass::Public,
                    "seat_0" => ViewerExportClass::SeatPrivate(BriarCircuitSeat::Seat0),
                    "seat_1" => ViewerExportClass::SeatPrivate(BriarCircuitSeat::Seat1),
                    "seat_2" => ViewerExportClass::SeatPrivate(BriarCircuitSeat::Seat2),
                    "seat_3" => ViewerExportClass::SeatPrivate(BriarCircuitSeat::Seat3),
                    _ => return Err(format!("unknown viewer export class: {value}")),
                })
            }
            "export_version" => {
                if value != VIEWER_EXPORT_VERSION.to_string() {
                    return Err("viewer export version mismatch".to_owned());
                }
            }
            other => return Err(format!("unknown viewer export field: {other}")),
        }
    }

    let class = class.ok_or_else(|| "viewer export missing class".to_owned())?;
    Ok(ViewerReplayExport {
        game_id: game_id.ok_or_else(|| "viewer export missing game_id".to_owned())?,
        rules_version: rules_version
            .ok_or_else(|| "viewer export missing rules_version".to_owned())?,
        export_version: VIEWER_EXPORT_VERSION,
        viewer_label: viewer.ok_or_else(|| "viewer export missing viewer".to_owned())?,
        class,
        observation_timeline: Vec::new(),
        migration_notes: vec!["trace-schema-v1 additive briar_circuit export".to_owned()],
    })
}

fn viewer_for_class(class: &ViewerExportClass) -> Viewer {
    match class {
        ViewerExportClass::Public => Viewer { seat_id: None },
        ViewerExportClass::SeatPrivate(seat) => Viewer {
            seat_id: Some(SeatId(seat.as_str().to_owned())),
        },
    }
}

fn viewer_label(class: &ViewerExportClass) -> String {
    match class {
        ViewerExportClass::Public => "public".to_owned(),
        ViewerExportClass::SeatPrivate(seat) => seat.as_str().to_owned(),
    }
}

fn hash_debug<T: core::fmt::Debug>(value: &T) -> HashValue {
    HashValue::from_stable_bytes(format!("{value:?}").as_bytes())
}
