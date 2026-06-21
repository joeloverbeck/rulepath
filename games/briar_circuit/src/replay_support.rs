use engine_core::{Diagnostic, HashValue, SeatId, Seed, Viewer};

use crate::{
    actions::{apply_pass_action, apply_play_action},
    bots::{BriarCircuitBotAction, BriarCircuitL1Bot},
    effects::BriarCircuitEffect,
    ids::{canonical_seat_ids, BriarCircuitSeat, GAME_ID, RULES_VERSION_LABEL},
    setup::{setup_match, SetupOptions},
    state::{BriarCircuitState, Phase, TerminalOutcome},
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

/// Deterministic replay of a complete Level 1 bot-driven match. Because the L1
/// policy, deal, scoring, and rules are all deterministic, a fixed seed fully
/// determines the outcome, so this is the canonical regression surface for
/// multi-hand golden traces: any change to deal, rules, scoring, or the L1 policy
/// changes one of the returned hashes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotMatchReplay {
    pub snapshot: ReplayHashSnapshot,
    pub effect_hash: HashValue,
    pub terminal: bool,
    pub winner: Option<BriarCircuitSeat>,
    pub hands_played: u32,
    pub cumulative_scores: [u16; 4],
    pub action_count: u32,
}

pub fn replay_bot_match(seed: u64, action_cap: usize) -> Result<BotMatchReplay, Diagnostic> {
    let mut state = setup_match(Seed(seed), &canonical_seat_ids(), &SetupOptions::default())?;
    let bot = BriarCircuitL1Bot::new(Seed(seed ^ 0xB16A_C1CC));
    let mut effects: Vec<BriarCircuitEffect> = Vec::new();
    let mut action_count = 0_u32;

    while !matches!(state.phase, Phase::Terminal(_)) {
        let Some(seat) = bot_match_active_seat(&state) else {
            break;
        };
        let decision = bot.select_decision(&state, seat)?;
        match decision.action {
            BriarCircuitBotAction::Pass(action) => {
                effects.extend(apply_pass_action(&mut state, seat, action)?.effects);
            }
            BriarCircuitBotAction::Play(action) => {
                effects.extend(apply_play_action(&mut state, seat, action)?.effects);
            }
        }
        action_count += 1;
        if action_count as usize >= action_cap {
            break;
        }
    }

    let viewer = Viewer { seat_id: None };
    let (terminal, winner) = match &state.phase {
        Phase::Terminal(TerminalOutcome::UniqueLowScoreWin { winner, .. }) => (true, Some(*winner)),
        _ => (false, None),
    };
    // At terminal the final hand index is not advanced, so it counts as one more hand.
    let hands_played = if terminal {
        state.hand_index + 1
    } else {
        state.hand_index
    };

    Ok(BotMatchReplay {
        snapshot: replay_hash_snapshot(&state),
        effect_hash: effect_hash(&effects, &viewer),
        terminal,
        winner,
        hands_played,
        cumulative_scores: state.cumulative_scores,
        action_count,
    })
}

fn bot_match_active_seat(state: &BriarCircuitState) -> Option<BriarCircuitSeat> {
    match &state.phase {
        Phase::Passing(pass) => BriarCircuitSeat::ALL
            .into_iter()
            .find(|seat| !pass.is_committed(*seat)),
        Phase::PlayingTrick(play) => Some(play.active_seat),
        Phase::ScoringHand(_) | Phase::Terminal(_) => None,
    }
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
