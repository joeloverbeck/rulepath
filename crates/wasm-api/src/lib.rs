//! Browser-facing Rulepath API surface.

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    slice, str,
};

use column_four::{
    apply_action as column_apply_action, legal_action_tree as column_legal_action_tree,
    project_view as column_project_view, replay_support::replay_commands as column_replay_commands,
    setup_match as column_setup_match, ColumnFourEffect, ColumnFourLevel2Bot, ColumnFourSeat,
    ColumnFourState,
};
use directional_flip::{
    apply_action as directional_apply_action, legal_action_tree as directional_legal_action_tree,
    project_view as directional_project_view,
    replay_support::replay_commands as directional_replay_commands,
    setup_match as directional_setup_match, DirectionalFlipEffect, DirectionalFlipLevel2Bot,
    DirectionalFlipSeat, DirectionalFlipState,
};
use engine_core::{
    ActionChoice, ActionPath, ActionTree, Actor, CommandEnvelope, EffectCursor, EffectEnvelope,
    EffectLog, RulesVersion, SeatId, Seed, Viewer, VisibilityScope,
};
use race_to_n::{
    apply_action as race_apply_action, legal_action_tree, project_view,
    replay_support::replay_commands as race_replay_commands, setup_match as race_setup_match,
    RaceEffect, RaceRandomBot, RaceSeat, RaceState, SetupOptions as RaceSetupOptions,
};
use three_marks::{
    apply_action as three_apply_action, legal_action_tree as three_legal_action_tree,
    project_view as three_project_view, replay_support::replay_commands as three_replay_commands,
    setup_match as three_setup_match, ThreeMarksEffect, ThreeMarksLevel1Bot, ThreeMarksSeat,
    ThreeMarksState,
};

const API_VERSION: &str = "rulepath-wasm-api/0.1.0";
const GAME_RACE_TO_N: &str = "race_to_n";
const GAME_RACE_TO_N_DISPLAY_NAME: &str = "Race to 21";
const GAME_THREE_MARKS: &str = "three_marks";
const GAME_THREE_MARKS_DISPLAY_NAME: &str = "Three Marks";
const GAME_COLUMN_FOUR: &str = "column_four";
const GAME_COLUMN_FOUR_DISPLAY_NAME: &str = "Column Four";
const GAME_DIRECTIONAL_FLIP: &str = "directional_flip";
const GAME_DIRECTIONAL_FLIP_DISPLAY_NAME: &str = "Directional Flip";
const RULES_VERSION: u32 = 1;
const SCHEMA_VERSION: u32 = 1;
const SUPPORTED_OPERATIONS: &[&str] = &[
    "feature_report",
    "list_games",
    "new_match",
    "get_view",
    "get_action_tree",
    "apply_action",
    "run_bot_turn",
    "get_effects",
    "export_replay",
    "import_replay",
    "replay_step",
    "replay_reset",
];
const FEATURE_FLAGS: &[&str] = &["catalog", "match_store", "legal_action_tree", "effects"];
const RACE_TRACE_RULES_VERSION: &str = "race_to_n-rules-v1";
const THREE_MARKS_TRACE_RULES_VERSION: &str = "three_marks-rules-v1";
const COLUMN_FOUR_TRACE_RULES_VERSION: &str = "column_four-rules-v1";
const DIRECTIONAL_FLIP_TRACE_RULES_VERSION: &str = "directional_flip-rules-v1";
const ENGINE_VERSION: &str = "engine-core-0.1.0";
const DATA_VERSION: &str = "1";
const VARIANT_RACE_TO_21: &str = "race_to_21";
const VARIANT_THREE_MARKS_STANDARD: &str = "three_marks_standard";
const VARIANT_COLUMN_FOUR_STANDARD: &str = "column_four_standard";
const VARIANT_DIRECTIONAL_FLIP_STANDARD: &str = "directional_flip_standard";
const MAX_REPLAY_IMPORT_BYTES: usize = 128 * 1024;

thread_local! {
    static MATCHES: RefCell<BTreeMap<String, MatchRecord>> = const { RefCell::new(BTreeMap::new()) };
    static REPLAYS: RefCell<BTreeMap<String, ReplayRecord>> = const { RefCell::new(BTreeMap::new()) };
    static NEXT_MATCH_ID: Cell<u64> = const { Cell::new(1) };
    static NEXT_REPLAY_ID: Cell<u64> = const { Cell::new(1) };
    static LAST_OUTPUT: RefCell<String> = const { RefCell::new(String::new()) };
}

#[derive(Clone, Debug)]
enum MatchRecord {
    RaceToN {
        game_id: String,
        seed: u64,
        state: RaceState,
        effects: EffectLog<RaceEffect>,
        commands: Vec<AppliedCommand>,
    },
    ThreeMarks {
        game_id: String,
        seed: u64,
        state: ThreeMarksState,
        effects: EffectLog<ThreeMarksEffect>,
        commands: Vec<AppliedCommand>,
    },
    ColumnFour {
        game_id: String,
        seed: u64,
        state: ColumnFourState,
        effects: EffectLog<ColumnFourEffect>,
        commands: Vec<AppliedCommand>,
    },
    DirectionalFlip {
        game_id: String,
        seed: u64,
        state: DirectionalFlipState,
        effects: EffectLog<DirectionalFlipEffect>,
        commands: Vec<AppliedCommand>,
    },
}

#[derive(Clone, Debug)]
struct ReplayRecord {
    game_id: String,
    seed: u64,
    commands: Vec<AppliedCommand>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AppliedCommand {
    actor_seat: String,
    action_path: Vec<String>,
    freshness_token: u64,
}

impl AppliedCommand {
    fn single_segment(&self) -> Result<String, String> {
        if self.action_path.len() != 1 {
            return Err(diagnostic_string(
                "unsupported_replay_action_path",
                "replay export supports one-segment action paths",
            ));
        }
        Ok(self.action_path[0].clone())
    }
}

#[derive(Clone, Debug)]
struct ParsedReplayDocument {
    schema_version: u64,
    game_id: String,
    rules_version: String,
    seed: u64,
    commands: Vec<AppliedCommand>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RegisteredGame {
    RaceToN,
    ThreeMarks,
    ColumnFour,
    DirectionalFlip,
}

pub fn placeholder_version() -> &'static str {
    API_VERSION
}

pub fn list_games() -> Result<String, String> {
    let games = [
        RegisteredGame::RaceToN,
        RegisteredGame::ThreeMarks,
        RegisteredGame::ColumnFour,
        RegisteredGame::DirectionalFlip,
    ]
        .iter()
        .map(|game| match game {
            RegisteredGame::RaceToN => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{}}}",
                escape_json(GAME_RACE_TO_N),
                escape_json(GAME_RACE_TO_N_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION
            ),
            RegisteredGame::ThreeMarks => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":[\"{}\"]}}",
                escape_json(GAME_THREE_MARKS),
                escape_json(GAME_THREE_MARKS_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                escape_json(VARIANT_THREE_MARKS_STANDARD)
            ),
            RegisteredGame::ColumnFour => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":[\"{}\"]}}",
                escape_json(GAME_COLUMN_FOUR),
                escape_json(GAME_COLUMN_FOUR_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                escape_json(VARIANT_COLUMN_FOUR_STANDARD)
            ),
            RegisteredGame::DirectionalFlip => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":[\"{}\"]}}",
                escape_json(GAME_DIRECTIONAL_FLIP),
                escape_json(GAME_DIRECTIONAL_FLIP_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                escape_json(VARIANT_DIRECTIONAL_FLIP_STANDARD)
            ),
        })
        .collect::<Vec<_>>()
        .join(",");
    Ok(format!("[{games}]"))
}

pub fn feature_report() -> Result<String, String> {
    Ok(format!(
        "{{\"api_version\":\"{}\",\"operations\":{},\"features\":{}}}",
        escape_json(API_VERSION),
        string_array_json(SUPPORTED_OPERATIONS),
        string_array_json(FEATURE_FLAGS)
    ))
}

pub fn new_match(game_id: &str, seed: u64) -> Result<String, String> {
    match resolve_game(game_id)? {
        RegisteredGame::RaceToN => {
            let seats = seats();
            let state = race_setup_match(Seed(seed), &seats, &RaceSetupOptions::default())
                .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::RaceToN {
                        game_id: GAME_RACE_TO_N.to_owned(),
                        seed,
                        state,
                        effects: EffectLog::new(),
                        commands: Vec::new(),
                    },
                );
            });
            Ok(format!(
                "{{\"match_id\":\"{}\",\"game_id\":\"{}\"}}",
                escape_json(&match_id),
                escape_json(game_id)
            ))
        }
        RegisteredGame::ThreeMarks => {
            let seats = seats();
            let state =
                three_setup_match(Seed(seed), &seats, &three_marks::SetupOptions::default())
                    .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::ThreeMarks {
                        game_id: GAME_THREE_MARKS.to_owned(),
                        seed,
                        state,
                        effects: EffectLog::new(),
                        commands: Vec::new(),
                    },
                );
            });
            Ok(format!(
                "{{\"match_id\":\"{}\",\"game_id\":\"{}\",\"variant_id\":\"{}\"}}",
                escape_json(&match_id),
                escape_json(game_id),
                escape_json(VARIANT_THREE_MARKS_STANDARD)
            ))
        }
        RegisteredGame::ColumnFour => {
            let seats = seats();
            let state =
                column_setup_match(Seed(seed), &seats, &column_four::SetupOptions::default())
                    .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::ColumnFour {
                        game_id: GAME_COLUMN_FOUR.to_owned(),
                        seed,
                        state,
                        effects: EffectLog::new(),
                        commands: Vec::new(),
                    },
                );
            });
            Ok(format!(
                "{{\"match_id\":\"{}\",\"game_id\":\"{}\",\"variant_id\":\"{}\"}}",
                escape_json(&match_id),
                escape_json(game_id),
                escape_json(VARIANT_COLUMN_FOUR_STANDARD)
            ))
        }
        RegisteredGame::DirectionalFlip => {
            let seats = seats();
            let state = directional_setup_match(
                Seed(seed),
                &seats,
                &directional_flip::SetupOptions::default(),
            )
            .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::DirectionalFlip {
                        game_id: GAME_DIRECTIONAL_FLIP.to_owned(),
                        seed,
                        state,
                        effects: EffectLog::new(),
                        commands: Vec::new(),
                    },
                );
            });
            Ok(format!(
                "{{\"match_id\":\"{}\",\"game_id\":\"{}\",\"variant_id\":\"{}\"}}",
                escape_json(&match_id),
                escape_json(game_id),
                escape_json(VARIANT_DIRECTIONAL_FLIP_STANDARD)
            ))
        }
    }
}

pub fn get_view(match_id: &str, _viewer_seat: Option<&str>) -> Result<String, String> {
    with_match(match_id, |record| match record {
        MatchRecord::RaceToN { game_id, state, .. } => {
            resolve_game(game_id)?;
            Ok(project_view(state).to_json())
        }
        MatchRecord::ThreeMarks { game_id, state, .. } => {
            resolve_game(game_id)?;
            Ok(three_project_view(state, &Viewer { seat_id: None }).to_json())
        }
        MatchRecord::ColumnFour { game_id, state, .. } => {
            resolve_game(game_id)?;
            Ok(column_view_json(&column_project_view(
                state,
                &Viewer { seat_id: None },
            )))
        }
        MatchRecord::DirectionalFlip { game_id, state, .. } => {
            resolve_game(game_id)?;
            Ok(directional_view_json(&directional_project_view(
                state,
                &Viewer { seat_id: None },
            )))
        }
    })
}

pub fn get_action_tree(match_id: &str, actor_seat: &str) -> Result<String, String> {
    with_match(match_id, |record| match record {
        MatchRecord::RaceToN { game_id, state, .. } => {
            resolve_game(game_id)?;
            let actor = race_actor_for_seat(state, parse_race_seat(actor_seat)?)?;
            Ok(action_tree_json(&legal_action_tree(state, &actor)))
        }
        MatchRecord::ThreeMarks { game_id, state, .. } => {
            resolve_game(game_id)?;
            let actor = three_actor_for_seat(state, parse_three_seat(actor_seat)?)?;
            Ok(action_tree_json(&three_legal_action_tree(state, &actor)))
        }
        MatchRecord::ColumnFour { game_id, state, .. } => {
            resolve_game(game_id)?;
            let actor = column_actor_for_seat(state, parse_column_seat(actor_seat)?)?;
            Ok(action_tree_json(&column_legal_action_tree(state, &actor)))
        }
        MatchRecord::DirectionalFlip { game_id, state, .. } => {
            resolve_game(game_id)?;
            let actor = directional_actor_for_seat(state, parse_directional_seat(actor_seat)?)?;
            Ok(action_tree_json(&directional_legal_action_tree(
                state, &actor,
            )))
        }
    })
}

pub fn apply_action(
    match_id: &str,
    actor_seat: &str,
    action_path: &str,
    freshness_token: u64,
) -> Result<String, String> {
    with_match_mut(match_id, |record| match record {
        MatchRecord::RaceToN {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_race_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: race_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = race_to_n::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = race_apply_action(state, action);
            let effect_json = race_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_race_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                project_view(state).to_json()
            ))
        }
        MatchRecord::ThreeMarks {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_three_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: three_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = three_marks::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = three_apply_action(state, action);
            let effect_json = three_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_three_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                three_project_view(state, &Viewer { seat_id: None }).to_json()
            ))
        }
        MatchRecord::ColumnFour {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_column_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: column_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = column_four::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = column_apply_action(state, action);
            let effect_json = column_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_column_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                column_view_json(&column_project_view(state, &Viewer { seat_id: None }))
            ))
        }
        MatchRecord::DirectionalFlip {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_directional_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: directional_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                directional_flip::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = directional_apply_action(state, action);
            let effect_json = directional_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_directional_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                directional_view_json(&directional_project_view(state, &Viewer { seat_id: None }))
            ))
        }
    })
}

pub fn run_bot_turn(match_id: &str, actor_seat: &str, bot_seed: u64) -> Result<String, String> {
    with_match_mut(match_id, |record| match record {
        MatchRecord::RaceToN {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_race_seat(actor_seat)?;
            let bot = RaceRandomBot::new(Seed(bot_seed));
            let action_path = bot.select_action(state, seat).map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: race_actor_for_seat(state, seat)?,
                action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = race_to_n::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = race_apply_action(state, action);
            let effect_json = race_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_race_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                project_view(state).to_json()
            ))
        }
        MatchRecord::ThreeMarks {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_three_seat(actor_seat)?;
            let decision = ThreeMarksLevel1Bot::new()
                .select_decision(state, seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: three_actor_for_seat(state, seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = three_marks::validate_command(state, &command).map_err(diagnostic_json)?;
            let mut effects = decision.effects;
            effects.extend(three_apply_action(state, action));
            let effect_json = three_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_three_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                three_project_view(state, &Viewer { seat_id: None }).to_json()
            ))
        }
        MatchRecord::ColumnFour {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_column_seat(actor_seat)?;
            let decision = ColumnFourLevel2Bot::new(Seed(bot_seed))
                .select_decision(state, seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: column_actor_for_seat(state, seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = column_four::validate_command(state, &command).map_err(diagnostic_json)?;
            let mut effects = decision.effects;
            effects.extend(column_apply_action(state, action));
            let effect_json = column_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_column_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                column_view_json(&column_project_view(state, &Viewer { seat_id: None }))
            ))
        }
        MatchRecord::DirectionalFlip {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_directional_seat(actor_seat)?;
            let decision = DirectionalFlipLevel2Bot::new(Seed(bot_seed))
                .select_decision(state, seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: directional_actor_for_seat(state, seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                directional_flip::validate_command(state, &command).map_err(diagnostic_json)?;
            let mut effects = decision.effects;
            effects.extend(directional_apply_action(state, action));
            let effect_json = directional_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_directional_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                directional_view_json(&directional_project_view(state, &Viewer { seat_id: None }))
            ))
        }
    })
}

pub fn get_effects(
    match_id: &str,
    since_cursor: u64,
    viewer_seat: Option<&str>,
) -> Result<String, String> {
    with_match(match_id, |record| match record {
        MatchRecord::RaceToN {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = race_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        race_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
        MatchRecord::ThreeMarks {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = three_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        three_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
        MatchRecord::ColumnFour {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = column_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        column_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
        MatchRecord::DirectionalFlip {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = directional_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        directional_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
    })
}

pub fn export_replay(match_id: &str) -> Result<String, String> {
    with_match(match_id, |record| match record {
        MatchRecord::RaceToN {
            game_id,
            seed,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            race_replay_document_json(&format!("export-{match_id}"), *seed, commands)
        }
        MatchRecord::ThreeMarks {
            game_id,
            seed,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            three_replay_document_json(&format!("export-{match_id}"), *seed, commands)
        }
        MatchRecord::ColumnFour {
            game_id,
            seed,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            column_replay_document_json(&format!("export-{match_id}"), *seed, commands)
        }
        MatchRecord::DirectionalFlip {
            game_id,
            seed,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            directional_replay_document_json(&format!("export-{match_id}"), *seed, commands)
        }
    })
}

pub fn import_replay(doc: &str) -> Result<String, String> {
    if doc.len() > MAX_REPLAY_IMPORT_BYTES {
        return Err(diagnostic_string(
            "replay_too_large",
            "replay document exceeds import size limit",
        ));
    }
    let parsed = parse_replay_document(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid replay document: {message}"),
        )
    })?;
    let game = resolve_game(&parsed.game_id).map_err(|_| {
        diagnostic_string(
            "unsupported_replay_game",
            &format!("unsupported replay game id: {}", parsed.game_id),
        )
    })?;
    if parsed.game_id != GAME_RACE_TO_N
        && parsed.game_id != GAME_THREE_MARKS
        && parsed.game_id != GAME_COLUMN_FOUR
        && parsed.game_id != GAME_DIRECTIONAL_FLIP
    {
        return Err(diagnostic_string(
            "unsupported_replay_game",
            &format!("unsupported replay game id: {}", parsed.game_id),
        ));
    }
    if parsed.schema_version != SCHEMA_VERSION as u64 {
        return Err(diagnostic_string(
            "unsupported_replay_schema",
            &format!(
                "unsupported replay schema version: {}",
                parsed.schema_version
            ),
        ));
    }
    let expected_rules = trace_rules_version(game);
    if parsed.rules_version != expected_rules {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {}", parsed.rules_version),
        ));
    }

    let command_count = parsed.commands.len();
    let (final_view, effect_count) = match game {
        RegisteredGame::RaceToN => {
            let (state, effects) =
                race_replay_to_cursor(parsed.seed, &parsed.commands, command_count)?;
            (project_view(&state).to_json(), effects.len())
        }
        RegisteredGame::ThreeMarks => {
            let (state, effects) =
                three_replay_to_cursor(parsed.seed, &parsed.commands, command_count)?;
            (
                three_project_view(&state, &Viewer { seat_id: None }).to_json(),
                effects.len(),
            )
        }
        RegisteredGame::ColumnFour => {
            let (state, effects) =
                column_replay_to_cursor(parsed.seed, &parsed.commands, command_count)?;
            (
                column_view_json(&column_project_view(&state, &Viewer { seat_id: None })),
                effects.len(),
            )
        }
        RegisteredGame::DirectionalFlip => {
            let (state, effects) =
                directional_replay_to_cursor(parsed.seed, &parsed.commands, command_count)?;
            (
                directional_view_json(&directional_project_view(&state, &Viewer { seat_id: None })),
                effects.len(),
            )
        }
    };
    let replay_id = next_replay_id(&parsed.game_id);
    let game_id = parsed.game_id.clone();
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: game_id.clone(),
                seed: parsed.seed,
                commands: parsed.commands,
            },
        );
    });

    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"command_count\":{},\"final_view\":{},\"effect_count\":{}}}",
        escape_json(&replay_id),
        escape_json(&game_id),
        command_count,
        final_view,
        effect_count
    ))
}

pub fn replay_step(replay_id: &str, cursor: usize) -> Result<String, String> {
    with_replay(replay_id, |record| match resolve_game(&record.game_id)? {
        RegisteredGame::RaceToN => {
            let bounded_cursor = cursor.min(record.commands.len());
            let (state, effects) =
                race_replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
            Ok(race_replay_step_json(
                replay_id,
                bounded_cursor,
                record.commands.len(),
                &state,
                &effects,
            ))
        }
        RegisteredGame::ThreeMarks => {
            let bounded_cursor = cursor.min(record.commands.len());
            let (state, effects) =
                three_replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
            Ok(three_replay_step_json(
                replay_id,
                bounded_cursor,
                record.commands.len(),
                &state,
                &effects,
            ))
        }
        RegisteredGame::ColumnFour => {
            let bounded_cursor = cursor.min(record.commands.len());
            let (state, effects) =
                column_replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
            Ok(column_replay_step_json(
                replay_id,
                bounded_cursor,
                record.commands.len(),
                &state,
                &effects,
            ))
        }
        RegisteredGame::DirectionalFlip => {
            let bounded_cursor = cursor.min(record.commands.len());
            let (state, effects) =
                directional_replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
            Ok(directional_replay_step_json(
                replay_id,
                bounded_cursor,
                record.commands.len(),
                &state,
                &effects,
            ))
        }
    })
}

pub fn replay_reset(replay_id: &str) -> Result<String, String> {
    replay_step(replay_id, 0)
}

fn resolve_game(game_id: &str) -> Result<RegisteredGame, String> {
    match game_id {
        GAME_RACE_TO_N => Ok(RegisteredGame::RaceToN),
        GAME_THREE_MARKS => Ok(RegisteredGame::ThreeMarks),
        GAME_COLUMN_FOUR => Ok(RegisteredGame::ColumnFour),
        GAME_DIRECTIONAL_FLIP => Ok(RegisteredGame::DirectionalFlip),
        _ => Err(format!(
            "{{\"code\":\"unknown_game\",\"message\":\"unsupported game id: {}\"}}",
            escape_json(game_id)
        )),
    }
}

fn next_replay_id(game_id: &str) -> String {
    NEXT_REPLAY_ID.with(|next| {
        let id = next.get();
        next.set(id.saturating_add(1));
        format!("{game_id}-replay-{id}")
    })
}

fn next_match_id(game_id: &str) -> String {
    NEXT_MATCH_ID.with(|next| {
        let id = next.get();
        next.set(id.saturating_add(1));
        format!("{game_id}-{id}")
    })
}

fn with_match<T>(
    match_id: &str,
    read: impl FnOnce(&MatchRecord) -> Result<T, String>,
) -> Result<T, String> {
    MATCHES.with(|matches| {
        let matches = matches.borrow();
        let record = matches
            .get(match_id)
            .ok_or_else(|| missing_match_json(match_id))?;
        read(record)
    })
}

fn with_match_mut<T>(
    match_id: &str,
    update: impl FnOnce(&mut MatchRecord) -> Result<T, String>,
) -> Result<T, String> {
    MATCHES.with(|matches| {
        let mut matches = matches.borrow_mut();
        let record = matches
            .get_mut(match_id)
            .ok_or_else(|| missing_match_json(match_id))?;
        update(record)
    })
}

fn with_replay<T>(
    replay_id: &str,
    read: impl FnOnce(&ReplayRecord) -> Result<T, String>,
) -> Result<T, String> {
    REPLAYS.with(|replays| {
        let replays = replays.borrow();
        let record = replays
            .get(replay_id)
            .ok_or_else(|| missing_replay_json(replay_id))?;
        read(record)
    })
}

fn missing_match_json(match_id: &str) -> String {
    format!(
        "{{\"code\":\"unknown_match\",\"message\":\"unknown match id: {}\"}}",
        escape_json(match_id)
    )
}

fn missing_replay_json(replay_id: &str) -> String {
    diagnostic_string("unknown_replay", &format!("unknown replay id: {replay_id}"))
}

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn trace_rules_version(game: RegisteredGame) -> &'static str {
    match game {
        RegisteredGame::RaceToN => RACE_TRACE_RULES_VERSION,
        RegisteredGame::ThreeMarks => THREE_MARKS_TRACE_RULES_VERSION,
        RegisteredGame::ColumnFour => COLUMN_FOUR_TRACE_RULES_VERSION,
        RegisteredGame::DirectionalFlip => DIRECTIONAL_FLIP_TRACE_RULES_VERSION,
    }
}

fn parse_race_seat(value: &str) -> Result<RaceSeat, String> {
    RaceSeat::parse(value).ok_or_else(|| {
        format!(
            "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
            escape_json(value)
        )
    })
}

fn parse_replay_race_seat(value: &str) -> Result<RaceSeat, String> {
    match value {
        "seat-0" => Ok(RaceSeat::Seat0),
        "seat-1" => Ok(RaceSeat::Seat1),
        _ => parse_race_seat(value),
    }
}

fn trace_race_seat(seat: RaceSeat) -> &'static str {
    match seat {
        RaceSeat::Seat0 => "seat-0",
        RaceSeat::Seat1 => "seat-1",
    }
}

fn parse_three_seat(value: &str) -> Result<ThreeMarksSeat, String> {
    match value {
        "seat-0" => Ok(ThreeMarksSeat::Seat0),
        "seat-1" => Ok(ThreeMarksSeat::Seat1),
        _ => ThreeMarksSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

fn trace_three_seat(seat: ThreeMarksSeat) -> &'static str {
    match seat {
        ThreeMarksSeat::Seat0 => "seat-0",
        ThreeMarksSeat::Seat1 => "seat-1",
    }
}

fn parse_column_seat(value: &str) -> Result<ColumnFourSeat, String> {
    match value {
        "seat-0" => Ok(ColumnFourSeat::Seat0),
        "seat-1" => Ok(ColumnFourSeat::Seat1),
        _ => ColumnFourSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

fn trace_column_seat(seat: ColumnFourSeat) -> &'static str {
    match seat {
        ColumnFourSeat::Seat0 => "seat-0",
        ColumnFourSeat::Seat1 => "seat-1",
    }
}

fn parse_directional_seat(value: &str) -> Result<DirectionalFlipSeat, String> {
    match value {
        "seat-0" => Ok(DirectionalFlipSeat::Seat0),
        "seat-1" => Ok(DirectionalFlipSeat::Seat1),
        _ => DirectionalFlipSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

fn trace_directional_seat(seat: DirectionalFlipSeat) -> &'static str {
    match seat {
        DirectionalFlipSeat::Seat0 => "seat-0",
        DirectionalFlipSeat::Seat1 => "seat-1",
    }
}

fn race_actor_for_seat(state: &RaceState, seat: RaceSeat) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

fn three_actor_for_seat(state: &ThreeMarksState, seat: ThreeMarksSeat) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

fn column_actor_for_seat(state: &ColumnFourState, seat: ColumnFourSeat) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

fn directional_actor_for_seat(
    state: &DirectionalFlipState,
    seat: DirectionalFlipSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

fn race_viewer_for_seat(state: &RaceState, seat: Option<&str>) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_race_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

fn three_viewer_for_seat(state: &ThreeMarksState, seat: Option<&str>) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_three_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

fn column_viewer_for_seat(state: &ColumnFourState, seat: Option<&str>) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_column_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

fn directional_viewer_for_seat(
    state: &DirectionalFlipState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_directional_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

fn parse_action_path(action_path: &str) -> ActionPath {
    ActionPath {
        segments: if action_path.is_empty() {
            Vec::new()
        } else {
            vec![action_path.to_owned()]
        },
    }
}

fn race_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(RaceState, Vec<EffectEnvelope<RaceEffect>>), String> {
    let seats = seats();
    let mut state = race_setup_match(Seed(seed), &seats, &RaceSetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_replay_race_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: race_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = race_to_n::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(race_apply_action(&mut state, action));
    }
    Ok((state, all_effects))
}

fn three_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(ThreeMarksState, Vec<EffectEnvelope<ThreeMarksEffect>>), String> {
    let seats = seats();
    let mut state = three_setup_match(Seed(seed), &seats, &three_marks::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_three_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: three_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = three_marks::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(three_apply_action(&mut state, action));
    }
    Ok((state, all_effects))
}

fn column_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(ColumnFourState, Vec<EffectEnvelope<ColumnFourEffect>>), String> {
    let seats = seats();
    let mut state = column_setup_match(Seed(seed), &seats, &column_four::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_column_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: column_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = column_four::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(column_apply_action(&mut state, action));
    }
    Ok((state, all_effects))
}

fn directional_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<
    (
        DirectionalFlipState,
        Vec<EffectEnvelope<DirectionalFlipEffect>>,
    ),
    String,
> {
    let seats = seats();
    let mut state = directional_setup_match(
        Seed(seed),
        &seats,
        &directional_flip::SetupOptions::default(),
    )
    .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_directional_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: directional_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action =
            directional_flip::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(directional_apply_action(&mut state, action));
    }
    Ok((state, all_effects))
}

fn action_tree_json(tree: &ActionTree) -> String {
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
    format!(
        "{{\"segment\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\",\"metadata\":[{}],\"tags\":[{}]}}",
        escape_json(&choice.segment),
        escape_json(&choice.label),
        escape_json(&choice.accessibility_label),
        metadata,
        tags
    )
}

fn race_effects_json(effects: &[EffectEnvelope<RaceEffect>]) -> String {
    let body = effects
        .iter()
        .map(race_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn three_effects_json(effects: &[EffectEnvelope<ThreeMarksEffect>]) -> String {
    let body = effects
        .iter()
        .map(three_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn column_effects_json(effects: &[EffectEnvelope<ColumnFourEffect>]) -> String {
    let body = effects
        .iter()
        .map(column_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn directional_effects_json(effects: &[EffectEnvelope<DirectionalFlipEffect>]) -> String {
    let body = effects
        .iter()
        .map(directional_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn race_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let command_segments = commands
        .iter()
        .map(|command| command.single_segment())
        .collect::<Result<Vec<_>, _>>()?;
    let hashes = race_replay_commands(seed, &command_segments);
    let commands_json = commands
        .iter()
        .enumerate()
        .map(|(index, command)| command_record_json(index, command))
        .collect::<Vec<_>>()
        .join(",");
    let checkpoints = if commands.is_empty() {
        "[{\"id\":\"final\",\"after_command_index\":0}]".to_owned()
    } else {
        format!(
            "[{{\"id\":\"final\",\"after_command_index\":{}}}]",
            commands.len().saturating_sub(1)
        )
    };
    let terminal = hashes.outcome.is_some();
    let winner = hashes.outcome.map_or_else(
        || "null".to_owned(),
        |winner| format!("\"{}\"", winner.as_str()),
    );

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported_replay\",\"note\":\"Replay exported by the Rulepath WASM API from the Rust command log.\",\"migration_update_note\":\"Generated by Gate 3 WASM replay export; expected hashes are computed by Rust replay support.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat-0\",\"player_id\":\"player-0\"}},{{\"seat_id\":\"seat-1\",\"player_id\":\"player-1\"}}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_private_view_hashes\":{{\"not_applicable\":\"race_to_n is perfect-information and has no private-view API.\"}},\"expected_outcome\":{{\"terminal\":{},\"winner\":{}}},\"expected_terminal_state\":{{\"terminal\":{},\"winner\":{}}},\"not_applicable\":{{\"hidden_information\":\"race_to_n is perfect-information and has no hidden state to redact.\",\"stochastic_game_events\":\"race_to_n game rules use no randomness; bot RNG is not replayed from exported documents because resolved commands are recorded.\",\"private_view_hashes\":\"race_to_n has no private-view API.\",\"preview_hashes\":\"race_to_n has no Rust preview surface in Gate 3.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_RACE_TO_N),
        escape_json(RACE_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_RACE_TO_21),
        commands_json,
        checkpoints,
        hashes.state_hash.0,
        hashes.effect_hash.0,
        hashes.action_tree_hash.0,
        hashes.view_hash.0,
        terminal,
        winner,
        terminal,
        winner
    ))
}

fn three_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let command_segments = commands
        .iter()
        .map(|command| command.single_segment())
        .collect::<Result<Vec<_>, _>>()?;
    let hashes = three_replay_commands(seed, &command_segments);
    let commands_json = commands
        .iter()
        .enumerate()
        .map(|(index, command)| command_record_json(index, command))
        .collect::<Vec<_>>()
        .join(",");
    let checkpoints = if commands.is_empty() {
        "[{\"id\":\"final\",\"after_command_index\":0}]".to_owned()
    } else {
        format!(
            "[{{\"id\":\"final\",\"after_command_index\":{}}}]",
            commands.len().saturating_sub(1)
        )
    };
    let outcome = hashes.outcome.map_or_else(
        || "{\"terminal\":false,\"winner\":null,\"kind\":\"none\"}".to_owned(),
        |outcome| match outcome {
            three_marks::TerminalOutcome::Draw => {
                "{\"terminal\":true,\"winner\":null,\"kind\":\"draw\"}".to_owned()
            }
            three_marks::TerminalOutcome::Win { seat, line } => format!(
                "{{\"terminal\":true,\"winner\":\"{}\",\"kind\":\"win\",\"line\":[\"{}\",\"{}\",\"{}\"]}}",
                trace_three_seat(seat),
                line.cells[0].as_str(),
                line.cells[1].as_str(),
                line.cells[2].as_str()
            ),
        },
    );

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported_replay\",\"note\":\"Replay exported by the Rulepath WASM API from the Rust command log.\",\"migration_update_note\":\"Generated by Gate 4 WASM replay export; expected hashes are computed by Three Marks Rust replay support.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat-0\",\"player_id\":\"player-0\"}},{{\"seat_id\":\"seat-1\",\"player_id\":\"player-1\"}}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_private_view_hashes\":{{\"not_applicable\":\"three_marks is perfect-information and has no private-view API.\"}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_outcome\":{},\"expected_terminal_state\":{},\"not_applicable\":{{\"hidden_information\":\"three_marks is perfect-information and has no hidden state to redact.\",\"stochastic_game_events\":\"three_marks game rules use no randomness; bot RNG is not replayed from exported documents because resolved commands are recorded.\",\"private_view_hashes\":\"three_marks has no private-view API.\",\"preview_hashes\":\"three_marks has no Rust preview surface in Gate 4.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_THREE_MARKS),
        escape_json(THREE_MARKS_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_THREE_MARKS_STANDARD),
        commands_json,
        checkpoints,
        hashes.state_hash.0,
        hashes.effect_hash.0,
        hashes.action_tree_hash.0,
        hashes.view_hash.0,
        hashes.replay_hash.0,
        outcome,
        outcome
    ))
}

fn column_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let command_segments = commands
        .iter()
        .map(|command| command.single_segment())
        .collect::<Result<Vec<_>, _>>()?;
    let hashes = column_replay_commands(seed, &command_segments);
    let commands_json = commands
        .iter()
        .enumerate()
        .map(|(index, command)| command_record_json(index, command))
        .collect::<Vec<_>>()
        .join(",");
    let checkpoints = if commands.is_empty() {
        "[{\"id\":\"final\",\"after_command_index\":0}]".to_owned()
    } else {
        format!(
            "[{{\"id\":\"final\",\"after_command_index\":{}}}]",
            commands.len().saturating_sub(1)
        )
    };
    let outcome = hashes.outcome.map_or_else(
        || "{\"terminal\":false,\"winner\":null,\"kind\":\"none\",\"draw\":false}".to_owned(),
        |outcome| match outcome {
            column_four::TerminalOutcome::Draw => {
                "{\"terminal\":true,\"winner\":null,\"kind\":\"draw\",\"draw\":true}".to_owned()
            }
            column_four::TerminalOutcome::Win { seat, line } => format!(
                "{{\"terminal\":true,\"winner\":\"{}\",\"kind\":\"win\",\"draw\":false,\"line\":[\"{}\",\"{}\",\"{}\",\"{}\"]}}",
                trace_column_seat(seat),
                line.cells[0].as_string(),
                line.cells[1].as_string(),
                line.cells[2].as_string(),
                line.cells[3].as_string()
            ),
        },
    );

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported_replay\",\"note\":\"Replay exported by the Rulepath WASM API from the Rust command log.\",\"migration_update_note\":\"Generated by Gate 5 WASM replay export; expected hashes are computed by Column Four Rust replay support.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat-0\",\"player_id\":\"player-0\"}},{{\"seat_id\":\"seat-1\",\"player_id\":\"player-1\"}}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_private_view_hashes\":{{\"not_applicable\":\"column_four is perfect-information and has no private-view API.\"}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_outcome\":{},\"expected_terminal_state\":{},\"not_applicable\":{{\"hidden_information\":\"column_four is perfect-information and has no hidden state to redact.\",\"stochastic_game_events\":\"column_four game rules use no randomness; bot RNG is not replayed from exported documents because resolved commands are recorded.\",\"private_view_hashes\":\"column_four has no private-view API.\",\"preview_hashes\":\"column_four has no Rust preview surface in Gate 5.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_COLUMN_FOUR),
        escape_json(COLUMN_FOUR_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_COLUMN_FOUR_STANDARD),
        commands_json,
        checkpoints,
        hashes.state_hash.0,
        hashes.effect_hash.0,
        hashes.action_tree_hash.0,
        hashes.view_hash.0,
        hashes.replay_hash.0,
        outcome,
        outcome
    ))
}

fn directional_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let command_segments = commands
        .iter()
        .map(|command| command.single_segment())
        .collect::<Result<Vec<_>, _>>()?;
    let hashes = directional_replay_commands(seed, &command_segments);
    let commands_json = commands
        .iter()
        .enumerate()
        .map(|(index, command)| command_record_json(index, command))
        .collect::<Vec<_>>()
        .join(",");
    let checkpoints = if commands.is_empty() {
        "[{\"id\":\"final\",\"after_command_index\":0}]".to_owned()
    } else {
        format!(
            "[{{\"id\":\"final\",\"after_command_index\":{}}}]",
            commands.len().saturating_sub(1)
        )
    };
    let outcome = hashes.outcome.map_or_else(
        || "{\"terminal\":false,\"winner\":null,\"kind\":\"none\",\"draw\":false}".to_owned(),
        |outcome| match outcome {
            directional_flip::TerminalOutcome::Draw => {
                "{\"terminal\":true,\"winner\":null,\"kind\":\"draw\",\"draw\":true}".to_owned()
            }
            directional_flip::TerminalOutcome::Win { seat } => format!(
                "{{\"terminal\":true,\"winner\":\"{}\",\"kind\":\"win\",\"draw\":false}}",
                trace_directional_seat(seat)
            ),
        },
    );

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported_replay\",\"note\":\"Replay exported by the Rulepath WASM API from the Rust command log.\",\"migration_update_note\":\"Generated by Gate 6 WASM replay export; expected hashes are computed by Directional Flip Rust replay support.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat-0\",\"player_id\":\"player-0\"}},{{\"seat_id\":\"seat-1\",\"player_id\":\"player-1\"}}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_private_view_hashes\":{{\"not_applicable\":\"directional_flip is perfect-information and has no private-view API.\"}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_outcome\":{},\"expected_terminal_state\":{},\"not_applicable\":{{\"hidden_information\":\"directional_flip is perfect-information and has no hidden state to redact.\",\"stochastic_game_events\":\"directional_flip game rules use no randomness; bot RNG is not replayed from exported documents because resolved commands are recorded.\",\"private_view_hashes\":\"directional_flip has no private-view API.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_DIRECTIONAL_FLIP),
        escape_json(DIRECTIONAL_FLIP_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_DIRECTIONAL_FLIP_STANDARD),
        commands_json,
        checkpoints,
        hashes.state_hash.0,
        hashes.effect_hash.0,
        hashes.action_tree_hash.0,
        hashes.view_hash.0,
        hashes.replay_hash.0,
        outcome,
        outcome
    ))
}

fn command_record_json(index: usize, command: &AppliedCommand) -> String {
    let action_path = command
        .action_path
        .iter()
        .map(|segment| format!("\"{}\"", escape_json(segment)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"index\":{},\"actor_seat\":\"{}\",\"action_path\":[{}],\"freshness_token\":\"{}\",\"expect\":\"applied\"}}",
        index,
        escape_json(&command.actor_seat),
        action_path,
        command.freshness_token
    )
}

fn race_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &RaceState,
    effects: &[EffectEnvelope<RaceEffect>],
) -> String {
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        project_view(state).to_json(),
        race_effects_json(effects)
    )
}

fn three_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &ThreeMarksState,
    effects: &[EffectEnvelope<ThreeMarksEffect>],
) -> String {
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        three_project_view(state, &Viewer { seat_id: None }).to_json(),
        three_effects_json(effects)
    )
}

fn column_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &ColumnFourState,
    effects: &[EffectEnvelope<ColumnFourEffect>],
) -> String {
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        column_view_json(&column_project_view(state, &Viewer { seat_id: None })),
        column_effects_json(effects)
    )
}

fn directional_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &DirectionalFlipState,
    effects: &[EffectEnvelope<DirectionalFlipEffect>],
) -> String {
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        directional_view_json(&directional_project_view(state, &Viewer { seat_id: None })),
        directional_effects_json(effects)
    )
}

fn visibility_json(visibility: &VisibilityScope) -> String {
    match visibility {
        VisibilityScope::Public => "\"public\"".to_owned(),
        VisibilityScope::PrivateToSeat(seat) => {
            format!("{{\"private_to_seat\":\"{}\"}}", escape_json(&seat.0))
        }
    }
}

fn race_effect_json(effect: &EffectEnvelope<RaceEffect>) -> String {
    let visibility = match &effect.visibility {
        VisibilityScope::Public => "\"public\"".to_owned(),
        VisibilityScope::PrivateToSeat(seat) => {
            format!("{{\"private_to_seat\":\"{}\"}}", escape_json(&seat.0))
        }
    };
    let payload = match &effect.payload {
        RaceEffect::ActionStarted { actor, amount } => format!(
            "{{\"type\":\"action_started\",\"actor\":\"{}\",\"amount\":{}}}",
            actor.as_str(),
            amount
        ),
        RaceEffect::CounterAdvanced {
            actor,
            from,
            to,
            amount,
        } => format!(
            "{{\"type\":\"counter_advanced\",\"actor\":\"{}\",\"from\":{},\"to\":{},\"amount\":{}}}",
            actor.as_str(),
            from.0,
            to.0,
            amount
        ),
        RaceEffect::TurnChanged { next_actor } => format!(
            "{{\"type\":\"turn_changed\",\"next_actor\":\"{}\"}}",
            next_actor.as_str()
        ),
        RaceEffect::GameEnded { winner } => {
            format!("{{\"type\":\"game_ended\",\"winner\":\"{}\"}}", winner.as_str())
        }
        RaceEffect::ActionCompleted { actor } => format!(
            "{{\"type\":\"action_completed\",\"actor\":\"{}\"}}",
            actor.as_str()
        ),
    };
    format!("{{\"visibility\":{},\"payload\":{}}}", visibility, payload)
}

fn three_effect_json(effect: &EffectEnvelope<ThreeMarksEffect>) -> String {
    let payload = match &effect.payload {
        ThreeMarksEffect::SetupComplete {
            game_id,
            variant_id,
            rules_version,
            seats,
        } => format!(
            "{{\"type\":\"setup_complete\",\"game_id\":\"{}\",\"variant_id\":\"{}\",\"rules_version\":\"{}\",\"seats\":[\"{}\",\"{}\"]}}",
            escape_json(game_id),
            escape_json(variant_id),
            escape_json(rules_version),
            escape_json(&seats[0]),
            escape_json(&seats[1])
        ),
        ThreeMarksEffect::MarkPlaced {
            seat,
            cell,
            ply,
            occupancy_summary,
        } => format!(
            "{{\"type\":\"mark_placed\",\"seat\":\"{}\",\"cell\":\"{}\",\"ply\":{},\"occupancy_summary\":\"{}\"}}",
            seat.as_str(),
            cell.as_str(),
            ply,
            escape_json(occupancy_summary)
        ),
        ThreeMarksEffect::ActivePlayerChanged {
            previous_seat,
            active_seat,
            ply,
        } => format!(
            "{{\"type\":\"active_player_changed\",\"previous_seat\":\"{}\",\"active_seat\":\"{}\",\"ply\":{}}}",
            previous_seat.as_str(),
            active_seat.as_str(),
            ply
        ),
        ThreeMarksEffect::PlacementRejected { reason, label } => format!(
            "{{\"type\":\"placement_rejected\",\"reason\":\"{}\",\"label\":\"{}\"}}",
            rejection_reason_json(*reason),
            escape_json(label)
        ),
        ThreeMarksEffect::LineCompleted { winning_seat, line } => format!(
            "{{\"type\":\"line_completed\",\"winning_seat\":\"{}\",\"line\":[\"{}\",\"{}\",\"{}\"]}}",
            winning_seat.as_str(),
            line.cells[0].as_str(),
            line.cells[1].as_str(),
            line.cells[2].as_str()
        ),
        ThreeMarksEffect::DrawReached {
            final_ply,
            full_board,
        } => format!(
            "{{\"type\":\"draw_reached\",\"final_ply\":{},\"full_board\":{}}}",
            final_ply, full_board
        ),
        ThreeMarksEffect::GameEnded {
            outcome,
            final_ply,
            terminal_hash_ref,
        } => format!(
            "{{\"type\":\"game_ended\",\"outcome\":{},\"final_ply\":{},\"terminal_hash_ref\":\"{}\"}}",
            terminal_outcome_json(*outcome),
            final_ply,
            escape_json(terminal_hash_ref)
        ),
        ThreeMarksEffect::BotChoseAction {
            level,
            policy_id,
            action_id,
            cell,
            explanation,
        } => format!(
            "{{\"type\":\"bot_chose_action\",\"level\":{},\"policy_id\":\"{}\",\"action_id\":\"{}\",\"cell\":\"{}\",\"explanation\":\"{}\"}}",
            level,
            escape_json(policy_id),
            escape_json(action_id),
            cell.as_str(),
            escape_json(explanation)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn column_view_json(view: &column_four::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"board_rows\":{},\"board_columns\":{},\"cells\":[{}],\"columns\":[{}],\"active_seat\":{},\"ply_count\":{},\"status_label\":\"{}\",\"freshness_token\":{},\"legal_targets\":[{}],\"terminal_kind\":\"{}\",\"winning_seat\":{},\"winning_line\":[{}],\"private_view_status\":\"{}\",\"hidden_fields\":[{}],\"replay_step_index\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.board_rows,
        view.board_columns,
        view.cells
            .iter()
            .map(column_cell_json)
            .collect::<Vec<_>>()
            .join(","),
        view.columns
            .iter()
            .map(column_summary_json)
            .collect::<Vec<_>>()
            .join(","),
        option_column_seat_json(view.active_seat),
        view.ply_count,
        escape_json(&view.status_label),
        view.freshness_token.0,
        view.legal_targets
            .iter()
            .map(column_legal_target_json)
            .collect::<Vec<_>>()
            .join(","),
        column_terminal_kind(&view.terminal),
        option_column_seat_json(column_terminal_winner(&view.terminal)),
        string_array(&column_terminal_line(&view.terminal)),
        escape_json(&view.private_view.status),
        string_array(&view.private_view.hidden_fields),
        view.replay_step_index
            .map_or_else(|| "null".to_owned(), |step| step.to_string())
    )
}

fn column_cell_json(cell: &column_four::CellView) -> String {
    format!(
        "{{\"cell\":\"{}\",\"row\":{},\"column\":{},\"occupancy\":\"{}\",\"owner\":{},\"piece_token_key\":{},\"piece_shape_label\":{}}}",
        cell.cell.as_string(),
        cell.row,
        cell.column,
        escape_json(&cell.occupancy),
        option_column_seat_json(cell.owner),
        option_string_json(cell.piece_token_key.as_deref()),
        option_string_json(cell.piece_shape_label.as_deref())
    )
}

fn column_summary_json(column: &column_four::ColumnSummaryView) -> String {
    format!(
        "{{\"column\":\"{}\",\"column_id\":\"{}\",\"label\":\"{}\",\"is_full\":{},\"legal_action_segment\":{},\"landing_preview\":{}}}",
        column.column.as_str(),
        escape_json(&column.column_id),
        escape_json(&column.label),
        column.is_full,
        option_string_json(column.legal_action_segment.as_deref()),
        option_cell_json(column.landing_preview)
    )
}

fn column_legal_target_json(target: &column_four::LegalColumnTargetView) -> String {
    format!(
        "{{\"column\":\"{}\",\"action_segment\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\",\"freshness_token\":{},\"landing_preview\":\"{}\"}}",
        target.column.as_str(),
        escape_json(&target.action_segment),
        escape_json(&target.label),
        escape_json(&target.accessibility_label),
        target.freshness_token.0,
        target.landing_preview.as_string()
    )
}

fn column_effect_json(effect: &EffectEnvelope<ColumnFourEffect>) -> String {
    let payload = match &effect.payload {
        ColumnFourEffect::DropAccepted { seat, column, ply } => format!(
            "{{\"type\":\"drop_accepted\",\"seat\":\"{}\",\"column\":\"{}\",\"ply\":{}}}",
            seat.as_str(),
            column.as_str(),
            ply
        ),
        ColumnFourEffect::PieceLanded {
            seat,
            column,
            row,
            cell,
            display_from_anchor,
            display_to_anchor,
        } => format!(
            "{{\"type\":\"piece_landed\",\"seat\":\"{}\",\"column\":\"{}\",\"row\":\"{}\",\"cell\":\"{}\",\"display_from_anchor\":\"{}\",\"display_to_anchor\":\"{}\"}}",
            seat.as_str(),
            column.as_str(),
            row.as_str(),
            cell.as_string(),
            escape_json(display_from_anchor),
            escape_json(display_to_anchor)
        ),
        ColumnFourEffect::ActivePlayerChanged {
            previous_seat,
            active_seat,
            ply,
        } => format!(
            "{{\"type\":\"active_player_changed\",\"previous_seat\":\"{}\",\"active_seat\":\"{}\",\"ply\":{}}}",
            previous_seat.as_str(),
            active_seat.as_str(),
            ply
        ),
        ColumnFourEffect::WinDetected { winning_seat, line } => format!(
            "{{\"type\":\"win_detected\",\"winning_seat\":\"{}\",\"line\":[\"{}\",\"{}\",\"{}\",\"{}\"]}}",
            winning_seat.as_str(),
            line.cells[0].as_string(),
            line.cells[1].as_string(),
            line.cells[2].as_string(),
            line.cells[3].as_string()
        ),
        ColumnFourEffect::DrawDetected {
            final_ply,
            full_board,
        } => format!(
            "{{\"type\":\"draw_detected\",\"final_ply\":{},\"full_board\":{}}}",
            final_ply, full_board
        ),
        ColumnFourEffect::GameEnded {
            outcome,
            final_ply,
            terminal_hash_ref,
        } => format!(
            "{{\"type\":\"game_ended\",\"outcome\":{},\"final_ply\":{},\"terminal_hash_ref\":\"{}\"}}",
            column_terminal_outcome_json(*outcome),
            final_ply,
            escape_json(terminal_hash_ref)
        ),
        ColumnFourEffect::BotChoseAction {
            level,
            policy_id,
            action_id,
            column,
            rationale,
        } => format!(
            "{{\"type\":\"bot_chose_action\",\"level\":{},\"policy_id\":\"{}\",\"action_id\":\"{}\",\"column\":\"{}\",\"rationale\":\"{}\"}}",
            level,
            escape_json(policy_id),
            escape_json(action_id),
            column.as_str(),
            escape_json(rationale)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn column_terminal_kind(terminal: &column_four::TerminalView) -> &'static str {
    match terminal {
        column_four::TerminalView::NonTerminal => "non_terminal",
        column_four::TerminalView::Win { .. } => "win",
        column_four::TerminalView::Draw => "draw",
    }
}

fn column_terminal_winner(terminal: &column_four::TerminalView) -> Option<ColumnFourSeat> {
    match terminal {
        column_four::TerminalView::Win { winning_seat, .. } => Some(*winning_seat),
        _ => None,
    }
}

fn column_terminal_line(terminal: &column_four::TerminalView) -> Vec<String> {
    match terminal {
        column_four::TerminalView::Win { line, .. } => {
            line.iter().map(|cell| cell.as_string()).collect()
        }
        _ => Vec::new(),
    }
}

fn column_terminal_outcome_json(outcome: column_four::TerminalOutcome) -> String {
    match outcome {
        column_four::TerminalOutcome::Draw => "{\"kind\":\"draw\"}".to_owned(),
        column_four::TerminalOutcome::Win { seat, line } => format!(
            "{{\"kind\":\"win\",\"seat\":\"{}\",\"line\":[\"{}\",\"{}\",\"{}\",\"{}\"]}}",
            seat.as_str(),
            line.cells[0].as_string(),
            line.cells[1].as_string(),
            line.cells[2].as_string(),
            line.cells[3].as_string()
        ),
    }
}

fn directional_view_json(view: &directional_flip::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"board_rows\":{},\"board_columns\":{},\"cells\":[{}],\"active_seat\":{},\"ply_count\":{},\"status_label\":\"{}\",\"freshness_token\":{},\"score\":{},\"legal_targets\":[{}],\"terminal_kind\":\"{}\",\"winning_seat\":{},\"final_score\":{},\"private_view_status\":\"{}\",\"hidden_fields\":[{}],\"ui\":{},\"last_action_summary\":{},\"bot_rationale\":{},\"replay_step_index\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.board_rows,
        view.board_columns,
        view.cells
            .iter()
            .map(directional_cell_json)
            .collect::<Vec<_>>()
            .join(","),
        option_directional_seat_json(view.active_seat),
        view.ply_count,
        escape_json(&view.status_label),
        view.freshness_token.0,
        directional_score_view_json(&view.score),
        view.legal_targets
            .iter()
            .map(directional_legal_target_json)
            .collect::<Vec<_>>()
            .join(","),
        directional_terminal_kind(&view.terminal),
        option_directional_seat_json(directional_terminal_winner(&view.terminal)),
        directional_terminal_score_json(&view.terminal),
        escape_json(&view.private_view.status),
        string_array(&view.private_view.hidden_fields),
        directional_ui_json(&view.ui),
        option_string_json(view.last_action_summary.as_deref()),
        option_string_json(view.bot_rationale.as_deref()),
        view.replay_step_index
            .map_or_else(|| "null".to_owned(), |step| step.to_string())
    )
}

fn directional_cell_json(cell: &directional_flip::CellView) -> String {
    format!(
        "{{\"cell\":\"{}\",\"cell_id\":\"{}\",\"row\":{},\"column\":{},\"occupancy\":\"{}\",\"owner\":{},\"disc_token_key\":{},\"disc_shape_label\":{},\"disc_pattern_label\":{}}}",
        cell.cell.as_string(),
        escape_json(&cell.cell_id),
        cell.row,
        cell.column,
        escape_json(&cell.occupancy),
        option_directional_seat_json(cell.owner),
        option_string_json(cell.disc_token_key.as_deref()),
        option_string_json(cell.disc_shape_label.as_deref()),
        option_string_json(cell.disc_pattern_label.as_deref())
    )
}

fn directional_legal_target_json(target: &directional_flip::LegalTargetView) -> String {
    format!(
        "{{\"action_kind\":\"{}\",\"action_segment\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\",\"freshness_token\":{},\"cell\":{},\"preview\":{},\"reason_code\":{},\"explanation\":\"{}\"}}",
        escape_json(&target.action_kind),
        escape_json(&target.action_segment),
        escape_json(&target.label),
        escape_json(&target.accessibility_label),
        target.freshness_token.0,
        option_directional_cell_json(target.cell),
        target
            .preview
            .as_ref()
            .map_or_else(|| "null".to_owned(), directional_preview_json),
        option_string_json(target.reason_code.as_deref()),
        escape_json(&target.explanation)
    )
}

fn directional_preview_json(preview: &directional_flip::PlacementPreviewView) -> String {
    format!(
        "{{\"preview_id\":\"{}\",\"target_cell\":\"{}\",\"target_cell_id\":\"{}\",\"row\":{},\"column\":{},\"ordered_flip_cells\":[{}],\"ordered_flip_cell_ids\":[{}],\"direction_groups\":[{}],\"explanation\":\"{}\"}}",
        escape_json(&preview.preview_id),
        preview.target_cell.as_string(),
        escape_json(&preview.target_cell_id),
        preview.row,
        preview.column,
        directional_cell_array_json(&preview.ordered_flip_cells),
        string_array(&preview.ordered_flip_cell_ids),
        preview
            .direction_groups
            .iter()
            .map(directional_group_json)
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&preview.explanation)
    )
}

fn directional_group_json(group: &directional_flip::visibility::DirectionGroupView) -> String {
    format!(
        "{{\"direction\":\"{}\",\"cells\":[{}],\"cell_ids\":[{}]}}",
        escape_json(&group.direction),
        directional_cell_array_json(&group.cells),
        string_array(&group.cell_ids)
    )
}

fn directional_ui_json(ui: &directional_flip::UiMetadata) -> String {
    format!(
        "{{\"board_label\":\"{}\",\"row_count\":{},\"column_count\":{},\"first_disc_token_key\":\"{}\",\"first_disc_shape_label\":\"{}\",\"first_disc_pattern_label\":\"{}\",\"second_disc_token_key\":\"{}\",\"second_disc_shape_label\":\"{}\",\"second_disc_pattern_label\":\"{}\",\"legal_target_shape_label\":\"{}\",\"forced_pass_label\":\"{}\"}}",
        escape_json(&ui.board_label),
        ui.row_count,
        ui.column_count,
        escape_json(&ui.first_disc_token_key),
        escape_json(&ui.first_disc_shape_label),
        escape_json(&ui.first_disc_pattern_label),
        escape_json(&ui.second_disc_token_key),
        escape_json(&ui.second_disc_shape_label),
        escape_json(&ui.second_disc_pattern_label),
        escape_json(&ui.legal_target_shape_label),
        escape_json(&ui.forced_pass_label)
    )
}

fn directional_effect_json(effect: &EffectEnvelope<DirectionalFlipEffect>) -> String {
    let payload = match &effect.payload {
        DirectionalFlipEffect::PlacementAccepted { seat, cell, ply } => format!(
            "{{\"type\":\"placement_accepted\",\"seat\":\"{}\",\"cell\":\"{}\",\"ply\":{}}}",
            seat.as_str(),
            cell.as_string(),
            ply
        ),
        DirectionalFlipEffect::DiscPlaced {
            seat,
            cell,
            display_to_anchor,
        } => format!(
            "{{\"type\":\"disc_placed\",\"seat\":\"{}\",\"cell\":\"{}\",\"display_to_anchor\":\"{}\"}}",
            seat.as_str(),
            cell.as_string(),
            escape_json(display_to_anchor)
        ),
        DirectionalFlipEffect::DiscsFlipped { seat, flips } => format!(
            "{{\"type\":\"discs_flipped\",\"seat\":\"{}\",\"flips\":[{}]}}",
            seat.as_str(),
            flips
                .iter()
                .map(directional_flip_entry_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
        DirectionalFlipEffect::PassTaken { seat, ply, reason } => format!(
            "{{\"type\":\"pass_taken\",\"seat\":\"{}\",\"ply\":{},\"reason\":\"{}\"}}",
            seat.as_str(),
            ply,
            escape_json(reason)
        ),
        DirectionalFlipEffect::ActivePlayerChanged {
            previous_seat,
            active_seat,
            ply,
        } => format!(
            "{{\"type\":\"active_player_changed\",\"previous_seat\":\"{}\",\"active_seat\":\"{}\",\"ply\":{}}}",
            previous_seat.as_str(),
            active_seat.as_str(),
            ply
        ),
        DirectionalFlipEffect::GameEnded {
            outcome,
            final_score,
            final_ply,
            reason,
            ..
        } => format!(
            "{{\"type\":\"game_ended\",\"outcome\":{},\"final_score\":{},\"final_ply\":{},\"reason\":\"{}\"}}",
            directional_terminal_outcome_json(*outcome),
            directional_score_json(*final_score),
            final_ply,
            directional_terminal_reason(*reason)
        ),
        DirectionalFlipEffect::BotChoseAction {
            level,
            policy_id,
            action_id,
            rationale,
        } => format!(
            "{{\"type\":\"bot_chose_action\",\"level\":{},\"policy_id\":\"{}\",\"action_id\":\"{}\",\"rationale\":\"{}\"}}",
            level,
            escape_json(policy_id),
            escape_json(action_id),
            escape_json(rationale)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn directional_flip_entry_json(flip: &directional_flip::FlipEntry) -> String {
    format!(
        "{{\"cell\":\"{}\",\"previous_owner\":\"{}\",\"new_owner\":\"{}\",\"direction\":\"{}\",\"distance\":{},\"order_index\":{},\"display_anchor\":\"{}\"}}",
        flip.cell.as_string(),
        flip.previous_owner.as_str(),
        flip.new_owner.as_str(),
        flip.direction.as_str(),
        flip.distance,
        flip.order_index,
        escape_json(&flip.display_anchor)
    )
}

fn directional_terminal_kind(terminal: &directional_flip::TerminalView) -> &'static str {
    match terminal {
        directional_flip::TerminalView::NonTerminal => "non_terminal",
        directional_flip::TerminalView::Win { .. } => "win",
        directional_flip::TerminalView::Draw { .. } => "draw",
    }
}

fn directional_terminal_winner(
    terminal: &directional_flip::TerminalView,
) -> Option<DirectionalFlipSeat> {
    match terminal {
        directional_flip::TerminalView::Win { winning_seat, .. } => Some(*winning_seat),
        _ => None,
    }
}

fn directional_terminal_score_json(terminal: &directional_flip::TerminalView) -> String {
    match terminal {
        directional_flip::TerminalView::NonTerminal => "null".to_owned(),
        directional_flip::TerminalView::Win { final_score, .. }
        | directional_flip::TerminalView::Draw { final_score } => {
            directional_score_view_json(final_score)
        }
    }
}

fn directional_terminal_outcome_json(outcome: directional_flip::TerminalOutcome) -> String {
    match outcome {
        directional_flip::TerminalOutcome::Draw => "{\"kind\":\"draw\"}".to_owned(),
        directional_flip::TerminalOutcome::Win { seat } => {
            format!("{{\"kind\":\"win\",\"seat\":\"{}\"}}", seat.as_str())
        }
    }
}

fn directional_terminal_reason(reason: directional_flip::TerminalReason) -> &'static str {
    match reason {
        directional_flip::TerminalReason::BoardFull => "board_full",
        directional_flip::TerminalReason::NoContinuation => "no_continuation",
        directional_flip::TerminalReason::DoubleForcedPass => "double_forced_pass",
    }
}

fn directional_score_json(score: directional_flip::Score) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{}}}",
        score.seat_0, score.seat_1
    )
}

fn directional_score_view_json(score: &directional_flip::ScoreView) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{}}}",
        score.seat_0, score.seat_1
    )
}

fn option_directional_seat_json(seat: Option<DirectionalFlipSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

fn option_directional_cell_json(cell: Option<directional_flip::CellId>) -> String {
    cell.map_or_else(
        || "null".to_owned(),
        |cell| format!("\"{}\"", cell.as_string()),
    )
}

fn directional_cell_array_json(values: &[directional_flip::CellId]) -> String {
    values
        .iter()
        .map(|cell| format!("\"{}\"", cell.as_string()))
        .collect::<Vec<_>>()
        .join(",")
}

fn option_column_seat_json(seat: Option<ColumnFourSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

fn option_cell_json(cell: Option<column_four::CellId>) -> String {
    cell.map_or_else(
        || "null".to_owned(),
        |cell| format!("\"{}\"", cell.as_string()),
    )
}

fn option_string_json(value: Option<&str>) -> String {
    value.map_or_else(
        || "null".to_owned(),
        |value| format!("\"{}\"", escape_json(value)),
    )
}

fn string_array(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",")
}

fn terminal_outcome_json(outcome: three_marks::TerminalOutcome) -> String {
    match outcome {
        three_marks::TerminalOutcome::Draw => "{\"kind\":\"draw\"}".to_owned(),
        three_marks::TerminalOutcome::Win { seat, line } => format!(
            "{{\"kind\":\"win\",\"seat\":\"{}\",\"line\":[\"{}\",\"{}\",\"{}\"]}}",
            seat.as_str(),
            line.cells[0].as_str(),
            line.cells[1].as_str(),
            line.cells[2].as_str()
        ),
    }
}

fn rejection_reason_json(reason: three_marks::RejectionReason) -> &'static str {
    match reason {
        three_marks::RejectionReason::Occupied => "occupied",
        three_marks::RejectionReason::Stale => "stale",
        three_marks::RejectionReason::InvalidCell => "invalid_cell",
        three_marks::RejectionReason::WrongActor => "wrong_actor",
        three_marks::RejectionReason::Terminal => "terminal",
        three_marks::RejectionReason::UnknownActor => "unknown_actor",
        three_marks::RejectionReason::InvalidPath => "invalid_path",
        three_marks::RejectionReason::InvalidAction => "invalid_action",
    }
}

fn diagnostic_json(diagnostic: engine_core::Diagnostic) -> String {
    format!(
        "{{\"code\":\"{}\",\"message\":\"{}\"}}",
        escape_json(&diagnostic.code),
        escape_json(&diagnostic.message)
    )
}

fn diagnostic_string(code: &str, message: &str) -> String {
    format!(
        "{{\"code\":\"{}\",\"message\":\"{}\"}}",
        escape_json(code),
        escape_json(message)
    )
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

fn string_array_json(values: &[&str]) -> String {
    let body = values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn parse_replay_document(input: &str) -> Result<ParsedReplayDocument, String> {
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

fn parse_replay_command(input: &str) -> Result<AppliedCommand, String> {
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

fn validate_json_object(input: &str) -> Result<(), String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err("malformed JSON object".to_owned());
    }
    let mut depth = 0_i32;
    let mut in_string = false;
    let mut escaped = false;
    for ch in trimmed.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' | '[' => depth += 1,
            '}' | ']' => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return Err("malformed JSON nesting".to_owned());
        }
    }
    if depth != 0 || in_string {
        return Err("malformed JSON nesting".to_owned());
    }
    Ok(())
}

fn reject_unknown_root_fields(input: &str, allowed: &[&str]) -> Result<(), String> {
    for key in top_level_keys(input)? {
        if !allowed.contains(&key.as_str()) {
            return Err(format!("unknown field `{key}`"));
        }
    }
    Ok(())
}

fn top_level_keys(input: &str) -> Result<Vec<String>, String> {
    let body = input
        .trim()
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .ok_or_else(|| "malformed JSON object".to_owned())?;
    let mut keys = Vec::new();
    let mut index = 0;
    while index < body.len() {
        let rest = body[index..].trim_start();
        if rest.is_empty() {
            break;
        }
        let skipped = body[index..].len() - rest.len();
        index += skipped;
        if body[index..].starts_with(',') {
            index += 1;
            continue;
        }
        let (key, next) = parse_json_string_at(body, index)?;
        index = next;
        let after_key = body[index..].trim_start();
        if !after_key.starts_with(':') {
            return Err("malformed JSON field".to_owned());
        }
        index += body[index..].len() - after_key.len() + 1;
        index = skip_json_value(body, index)?;
        keys.push(key);
    }
    Ok(keys)
}

fn skip_json_value(input: &str, mut index: usize) -> Result<usize, String> {
    while input[index..].starts_with(char::is_whitespace) {
        index += input[index..].chars().next().unwrap().len_utf8();
    }
    let mut in_string = false;
    let mut escaped = false;
    let mut depth = 0_i32;
    for (offset, ch) in input[index..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' | '[' => depth += 1,
            '}' | ']' => {
                if depth == 0 {
                    return Ok(index + offset);
                }
                depth -= 1;
            }
            ',' if depth == 0 => return Ok(index + offset + 1),
            _ => {}
        }
    }
    Ok(input.len())
}

fn string_field(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    parse_string_at(input, start).ok_or_else(|| format!("field `{key}` must be a string"))
}

fn number_field(input: &str, key: &str) -> Result<u64, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    parse_number_at(input, start).ok_or_else(|| format!("field `{key}` must be a number"))
}

fn string_array_field(input: &str, key: &str) -> Result<Vec<String>, String> {
    array_items(input, key)?
        .into_iter()
        .map(|item| parse_json_string(item.trim()))
        .collect()
}

fn array_items(input: &str, key: &str) -> Result<Vec<String>, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    let open = input[start..]
        .find('[')
        .ok_or_else(|| format!("field `{key}` must be an array"))?
        + start;
    let close = matching_bracket(input, open, '[', ']')?;
    let body = &input[open + 1..close];
    if body.trim().is_empty() {
        return Ok(Vec::new());
    }
    split_top_level(body, ',')
}

fn matching_bracket(
    input: &str,
    open: usize,
    open_ch: char,
    close_ch: char,
) -> Result<usize, String> {
    let mut depth = 0_u32;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, ch) in input[open..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        if ch == '"' {
            in_string = true;
        } else if ch == open_ch {
            depth += 1;
        } else if ch == close_ch {
            depth = depth
                .checked_sub(1)
                .ok_or_else(|| "unbalanced JSON".to_owned())?;
            if depth == 0 {
                return Ok(open + offset);
            }
        }
    }
    Err("unbalanced JSON".to_owned())
}

fn split_top_level(input: &str, delimiter: char) -> Result<Vec<String>, String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut nested = 0usize;
    for (index, ch) in input.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            '{' | '[' if !in_string => nested += 1,
            '}' | ']' if !in_string => {
                nested = nested
                    .checked_sub(1)
                    .ok_or_else(|| "unbalanced JSON".to_owned())?;
            }
            ch if ch == delimiter && !in_string && nested == 0 => {
                parts.push(input[start..index].to_owned());
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    if in_string || nested != 0 {
        return Err("unbalanced JSON".to_owned());
    }
    parts.push(input[start..].to_owned());
    Ok(parts)
}

fn parse_string_at(input: &str, start: usize) -> Option<String> {
    let tail = input[start..].trim_start();
    parse_json_string_prefix(tail).map(|(value, _)| value).ok()
}

fn parse_json_string_at(input: &str, start: usize) -> Result<(String, usize), String> {
    let (value, consumed) = parse_json_string_prefix(&input[start..])?;
    Ok((value, start + consumed))
}

fn parse_json_string(input: &str) -> Result<String, String> {
    let body = input
        .trim()
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| "expected JSON string".to_owned())?;
    let mut output = String::new();
    let mut escaped = false;
    for ch in body.chars() {
        if escaped {
            output.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else {
            output.push(ch);
        }
    }
    if escaped {
        return Err("unterminated escape".to_owned());
    }
    Ok(output)
}

fn parse_json_string_prefix(input: &str) -> Result<(String, usize), String> {
    let tail = input
        .strip_prefix('"')
        .ok_or_else(|| "expected JSON string".to_owned())?;
    let mut output = String::new();
    let mut escaped = false;
    for (index, ch) in tail.char_indices() {
        if escaped {
            output.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            return Ok((output, index + 2));
        } else {
            output.push(ch);
        }
    }
    Err("unterminated JSON string".to_owned())
}

fn parse_number_at(input: &str, start: usize) -> Option<u64> {
    let tail = input[start..].trim_start();
    let digits = tail
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

#[no_mangle]
pub extern "C" fn rulepath_placeholder_version_ptr() -> *const u8 {
    API_VERSION.as_ptr()
}

#[no_mangle]
pub extern "C" fn rulepath_placeholder_version_len() -> usize {
    API_VERSION.len()
}

#[no_mangle]
pub extern "C" fn rulepath_feature_report() -> i32 {
    write_result(feature_report())
}

#[no_mangle]
pub extern "C" fn rulepath_list_games() -> i32 {
    write_result(list_games())
}

#[no_mangle]
pub extern "C" fn rulepath_alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}

#[no_mangle]
/// # Safety
///
/// `ptr` must have been returned by `rulepath_alloc` with the same `len`, and it
/// must not be used after this call.
pub unsafe extern "C" fn rulepath_dealloc(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        drop(unsafe { Vec::from_raw_parts(ptr, 0, len) });
    }
}

#[no_mangle]
pub extern "C" fn rulepath_last_output_ptr() -> *const u8 {
    LAST_OUTPUT.with(|output| output.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn rulepath_last_output_len() -> usize {
    LAST_OUTPUT.with(|output| output.borrow().len())
}

#[no_mangle]
/// # Safety
///
/// `game_ptr..game_ptr + game_len` must be a valid UTF-8 buffer for the duration
/// of the call.
pub unsafe extern "C" fn rulepath_new_match(
    game_ptr: *const u8,
    game_len: usize,
    seed: u64,
) -> i32 {
    let game_id = unsafe { read_string(game_ptr, game_len) };
    write_result(game_id.and_then(|game_id| new_match(&game_id, seed)))
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` must be a valid UTF-8 buffer for the
/// duration of the call.
pub unsafe extern "C" fn rulepath_get_view(match_ptr: *const u8, match_len: usize) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    write_result(match_id.and_then(|match_id| get_view(&match_id, None)))
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` and `seat_ptr..seat_ptr + seat_len` must
/// be valid UTF-8 buffers for the duration of the call.
pub unsafe extern "C" fn rulepath_get_action_tree(
    match_ptr: *const u8,
    match_len: usize,
    seat_ptr: *const u8,
    seat_len: usize,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let seat = unsafe { read_string(seat_ptr, seat_len) };
    write_result(
        match_id.and_then(|match_id| seat.and_then(|seat| get_action_tree(&match_id, &seat))),
    )
}

#[no_mangle]
/// # Safety
///
/// `match_ptr`, `seat_ptr`, and `path_ptr` with their lengths must be valid
/// UTF-8 buffers for the duration of the call.
pub unsafe extern "C" fn rulepath_apply_action(
    match_ptr: *const u8,
    match_len: usize,
    seat_ptr: *const u8,
    seat_len: usize,
    path_ptr: *const u8,
    path_len: usize,
    freshness_token: u64,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let seat = unsafe { read_string(seat_ptr, seat_len) };
    let path = unsafe { read_string(path_ptr, path_len) };
    write_result(match_id.and_then(|match_id| {
        seat.and_then(|seat| {
            path.and_then(|path| apply_action(&match_id, &seat, &path, freshness_token))
        })
    }))
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` and `seat_ptr..seat_ptr + seat_len` must
/// be valid UTF-8 buffers for the duration of the call.
pub unsafe extern "C" fn rulepath_run_bot_turn(
    match_ptr: *const u8,
    match_len: usize,
    seat_ptr: *const u8,
    seat_len: usize,
    bot_seed: u64,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let seat = unsafe { read_string(seat_ptr, seat_len) };
    write_result(
        match_id
            .and_then(|match_id| seat.and_then(|seat| run_bot_turn(&match_id, &seat, bot_seed))),
    )
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` must be valid UTF-8. If `viewer_len` is
/// nonzero, `viewer_ptr..viewer_ptr + viewer_len` must also be valid UTF-8.
pub unsafe extern "C" fn rulepath_get_effects(
    match_ptr: *const u8,
    match_len: usize,
    since_cursor: u64,
    viewer_ptr: *const u8,
    viewer_len: usize,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let viewer = if viewer_len == 0 {
        Ok(None)
    } else {
        unsafe { read_string(viewer_ptr, viewer_len) }.map(Some)
    };
    write_result(match_id.and_then(|match_id| {
        viewer.and_then(|viewer| get_effects(&match_id, since_cursor, viewer.as_deref()))
    }))
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` must be a valid UTF-8 buffer for the
/// duration of the call.
pub unsafe extern "C" fn rulepath_export_replay(match_ptr: *const u8, match_len: usize) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    write_result(match_id.and_then(|match_id| export_replay(&match_id)))
}

#[no_mangle]
/// # Safety
///
/// `doc_ptr..doc_ptr + doc_len` must be a valid UTF-8 buffer for the duration of
/// the call.
pub unsafe extern "C" fn rulepath_import_replay(doc_ptr: *const u8, doc_len: usize) -> i32 {
    let doc = unsafe { read_string(doc_ptr, doc_len) };
    write_result(doc.and_then(|doc| import_replay(&doc)))
}

#[no_mangle]
/// # Safety
///
/// `replay_ptr..replay_ptr + replay_len` must be a valid UTF-8 buffer for the
/// duration of the call.
pub unsafe extern "C" fn rulepath_replay_step(
    replay_ptr: *const u8,
    replay_len: usize,
    cursor: usize,
) -> i32 {
    let replay_id = unsafe { read_string(replay_ptr, replay_len) };
    write_result(replay_id.and_then(|replay_id| replay_step(&replay_id, cursor)))
}

#[no_mangle]
/// # Safety
///
/// `replay_ptr..replay_ptr + replay_len` must be a valid UTF-8 buffer for the
/// duration of the call.
pub unsafe extern "C" fn rulepath_replay_reset(replay_ptr: *const u8, replay_len: usize) -> i32 {
    let replay_id = unsafe { read_string(replay_ptr, replay_len) };
    write_result(replay_id.and_then(|replay_id| replay_reset(&replay_id)))
}

unsafe fn read_string(ptr: *const u8, len: usize) -> Result<String, String> {
    if ptr.is_null() && len > 0 {
        return Err(
            "{\"code\":\"invalid_pointer\",\"message\":\"input pointer is null\"}".to_owned(),
        );
    }
    let bytes = unsafe { slice::from_raw_parts(ptr, len) };
    str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| "{\"code\":\"invalid_utf8\",\"message\":\"input is not utf-8\"}".to_owned())
}

fn write_result(result: Result<String, String>) -> i32 {
    match result {
        Ok(output) => {
            write_output(output);
            0
        }
        Err(error) => {
            write_output(error);
            1
        }
    }
}

fn write_output(output: String) {
    LAST_OUTPUT.with(|last| {
        *last.borrow_mut() = output;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_version_is_stable() {
        assert_eq!(placeholder_version(), "rulepath-wasm-api/0.1.0");
    }

    #[test]
    fn list_games_reports_registered_games() {
        let games = list_games().expect("games listed");
        assert!(games.contains("\"game_id\":\"race_to_n\""));
        assert!(games.contains("\"game_id\":\"three_marks\""));
        assert!(games.contains("\"game_id\":\"column_four\""));
        assert!(games.contains("\"game_id\":\"directional_flip\""));
        assert!(games.contains("\"variants\":[\"three_marks_standard\"]"));
        assert!(games.contains("\"variants\":[\"column_four_standard\"]"));
        assert!(games.contains("\"variants\":[\"directional_flip_standard\"]"));
    }

    #[test]
    fn feature_report_lists_ops() {
        let report = feature_report().expect("feature report returned");
        assert!(report.contains("\"api_version\":\"rulepath-wasm-api/0.1.0\""));
        for operation in SUPPORTED_OPERATIONS {
            assert!(
                report.contains(&format!("\"{operation}\"")),
                "missing operation {operation} in {report}"
            );
        }
        assert!(report.contains(
            "\"features\":[\"catalog\",\"match_store\",\"legal_action_tree\",\"effects\"]"
        ));
    }

    #[test]
    fn new_ops_use_status_output_convention() {
        assert_eq!(rulepath_feature_report(), 0);
        assert!(last_output_string().contains("\"api_version\":\"rulepath-wasm-api/0.1.0\""));

        assert_eq!(rulepath_list_games(), 0);
        assert!(last_output_string().contains("\"game_id\":\"race_to_n\""));
    }

    #[test]
    fn surface_drives_minimal_turn_loop() {
        let created = new_match("race_to_n", 11).expect("match created");
        let match_id = extract_match_id(&created);

        let view = get_view(&match_id, None).expect("view returned");
        assert!(view.contains("\"counter\":0"));
        assert!(!view.contains("seat-0"));

        let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
        assert!(tree.contains("\"segment\":\"add-1\""));
        assert!(tree.contains("\"freshness_token\":0"));

        let applied = apply_action(&match_id, "seat_0", "add-1", 0).expect("human action applies");
        assert!(applied.contains("\"counter\":1"));
        assert!(applied.contains("\"type\":\"counter_advanced\""));

        let effects = get_effects(&match_id, 0, None).expect("effects returned");
        assert!(effects.contains("\"cursor\":1"));
        assert!(effects.contains("\"visibility\":\"public\""));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        assert!(bot.contains("\"ok\":true"));
        assert!(bot.contains("\"active_seat\":\"seat_0\""));
    }

    #[test]
    fn three_marks_surface_drives_operation_group() {
        let created = new_match("three_marks", 31).expect("match created");
        let match_id = extract_match_id(&created);
        assert!(created.contains("\"variant_id\":\"three_marks_standard\""));

        let view = get_view(&match_id, None).expect("view returned");
        assert!(view.contains("\"game_id\":\"three_marks\""));
        assert!(view.contains("\"variant_id\":\"three_marks_standard\""));
        assert!(view.contains("\"freshness_token\":0"));

        let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
        assert!(tree.contains("\"segment\":\"place/r1c1\""));
        assert!(tree.contains("\"freshness_token\":0"));

        let applied =
            apply_action(&match_id, "seat_0", "place/r1c1", 0).expect("human action applies");
        assert!(applied.contains("\"type\":\"mark_placed\""));
        assert!(applied.contains("\"active_seat\":\"seat_1\""));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        assert!(bot.contains("\"type\":\"bot_chose_action\""));
        assert!(bot.contains("\"active_seat\":\"seat_0\""));

        let effects = get_effects(&match_id, 0, None).expect("effects returned");
        assert!(effects.contains("\"type\":\"mark_placed\""));
        assert!(effects.contains("\"type\":\"bot_chose_action\""));

        let exported = export_replay(&match_id).expect("replay exported");
        assert!(exported.contains("\"game_id\":\"three_marks\""));
        assert!(exported.contains("\"expected_replay_hashes\""));
        assert!(
            exported.contains("\"private_view_hashes\":\"three_marks has no private-view API.\"")
        );

        let imported = import_replay(&exported).expect("replay imported");
        let replay_id = extract_replay_id(&imported);
        assert!(imported.contains("\"game_id\":\"three_marks\""));

        let reset = replay_reset(&replay_id).expect("replay reset returned");
        assert!(reset.contains("\"cursor\":0"));
        assert!(reset.contains("\"ply_count\":0"));

        let step = replay_step(&replay_id, 1).expect("replay stepped");
        assert!(step.contains("\"cursor\":1"));
        assert!(step.contains("\"ply_count\":1"));
    }

    #[test]
    fn column_four_surface_drives_operation_group() {
        let created = new_match("column_four", 41).expect("match created");
        let match_id = extract_match_id(&created);
        assert!(created.contains("\"variant_id\":\"column_four_standard\""));

        let view = get_view(&match_id, None).expect("view returned");
        assert!(view.contains("\"game_id\":\"column_four\""));
        assert!(view.contains("\"variant_id\":\"column_four_standard\""));
        assert!(view.contains("\"board_rows\":6"));
        assert!(view.contains("\"board_columns\":7"));
        assert!(view.contains("\"freshness_token\":0"));
        assert!(view.contains("\"hidden_fields\":[]"));

        let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
        assert!(tree.contains("\"segment\":\"drop/c4\""));
        assert!(tree.contains("\"freshness_token\":0"));

        let applied =
            apply_action(&match_id, "seat_0", "drop/c4", 0).expect("human action applies");
        assert!(applied.contains("\"type\":\"piece_landed\""));
        assert!(applied.contains("\"active_seat\":\"seat_1\""));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        assert!(bot.contains("\"type\":\"bot_chose_action\""));
        assert!(bot.contains("\"ply_count\":2"));

        let effects = get_effects(&match_id, 0, None).expect("effects returned");
        assert!(effects.contains("\"type\":\"piece_landed\""));
        assert!(effects.contains("\"type\":\"bot_chose_action\""));

        let exported = export_replay(&match_id).expect("replay exported");
        assert!(exported.contains("\"game_id\":\"column_four\""));
        assert!(exported.contains("\"rules_version\":\"column_four-rules-v1\""));
        assert!(exported.contains("\"expected_replay_hashes\""));
        assert!(
            exported.contains("\"private_view_hashes\":\"column_four has no private-view API.\"")
        );

        let imported = import_replay(&exported).expect("replay imported");
        let replay_id = extract_replay_id(&imported);
        assert!(imported.contains("\"game_id\":\"column_four\""));

        let reset = replay_reset(&replay_id).expect("replay reset returned");
        assert!(reset.contains("\"cursor\":0"));
        assert!(reset.contains("\"ply_count\":0"));

        let step = replay_step(&replay_id, 1).expect("replay stepped");
        assert!(step.contains("\"cursor\":1"));
        assert!(step.contains("\"ply_count\":1"));
    }

    #[test]
    fn directional_flip_surface_drives_operation_group() {
        let created = new_match("directional_flip", 51).expect("match created");
        let match_id = extract_match_id(&created);
        assert!(created.contains("\"variant_id\":\"directional_flip_standard\""));

        let view = get_view(&match_id, None).expect("view returned");
        assert!(view.contains("\"game_id\":\"directional_flip\""));
        assert!(view.contains("\"variant_id\":\"directional_flip_standard\""));
        assert!(view.contains("\"board_rows\":8"));
        assert!(view.contains("\"board_columns\":8"));
        assert!(view.contains("\"freshness_token\":0"));
        assert!(view.contains("\"score\":{\"seat_0\":2,\"seat_1\":2}"));
        assert!(view.contains("\"hidden_fields\":[]"));
        assert!(view.contains("\"ordered_flip_cells\""));

        let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
        assert!(tree.contains("\"segment\":\"place/"));
        assert!(tree.contains("\"freshness_token\":0"));

        let action_segment = tree
            .split("\"segment\":\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .expect("placement segment present")
            .to_owned();
        let applied =
            apply_action(&match_id, "seat_0", &action_segment, 0).expect("human action applies");
        assert!(applied.contains("\"type\":\"disc_placed\""));
        assert!(applied.contains("\"type\":\"discs_flipped\""));
        assert!(applied.contains("\"active_seat\":\"seat_1\""));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        assert!(bot.contains("\"type\":\"bot_chose_action\""));
        assert!(bot.contains("\"ply_count\":2"));

        let effects = get_effects(&match_id, 0, None).expect("effects returned");
        assert!(effects.contains("\"type\":\"disc_placed\""));
        assert!(effects.contains("\"type\":\"bot_chose_action\""));

        let exported = export_replay(&match_id).expect("replay exported");
        assert!(exported.contains("\"game_id\":\"directional_flip\""));
        assert!(exported.contains("\"rules_version\":\"directional_flip-rules-v1\""));
        assert!(exported.contains("\"expected_replay_hashes\""));
        assert!(exported
            .contains("\"private_view_hashes\":\"directional_flip has no private-view API.\""));
        assert!(!exported.contains("initial_snapshot"));

        let imported = import_replay(&exported).expect("replay imported");
        let replay_id = extract_replay_id(&imported);
        assert!(imported.contains("\"game_id\":\"directional_flip\""));

        let reset = replay_reset(&replay_id).expect("replay reset returned");
        assert!(reset.contains("\"cursor\":0"));
        assert!(reset.contains("\"ply_count\":0"));

        let step = replay_step(&replay_id, 1).expect("replay stepped");
        assert!(step.contains("\"cursor\":1"));
        assert!(step.contains("\"ply_count\":1"));
    }

    #[test]
    fn stale_action_returns_diagnostic_without_mutation() {
        let created = new_match("race_to_n", 12).expect("match created");
        let match_id = extract_match_id(&created);

        apply_action(&match_id, "seat_0", "add-1", 0).expect("first action applies");
        let stale =
            apply_action(&match_id, "seat_1", "add-1", 0).expect_err("stale token rejected");
        assert!(stale.contains("\"code\":\"stale_action\""));

        let view = get_view(&match_id, None).expect("view returned");
        assert!(view.contains("\"counter\":1"));
        assert!(view.contains("\"freshness_token\":1"));
    }

    #[test]
    fn replay_round_trip_reproduces_hashes() {
        let created = new_match("race_to_n", 21).expect("match created");
        let match_id = extract_match_id(&created);
        apply_action(&match_id, "seat_0", "add-1", 0).expect("first action applies");
        apply_action(&match_id, "seat_1", "add-2", 1).expect("second action applies");

        let exported = export_replay(&match_id).expect("replay exported");
        let expected = race_replay_commands(21, &["add-1".to_owned(), "add-2".to_owned()]);
        assert!(exported.contains(&format!(
            "\"expected_state_hashes\":{{\"final\":{}}}",
            expected.state_hash.0
        )));
        assert!(exported.contains(&format!(
            "\"expected_effect_hashes\":{{\"final\":{}}}",
            expected.effect_hash.0
        )));

        let imported = import_replay(&exported).expect("replay imported");
        let replay_id = extract_replay_id(&imported);
        let stepped = replay_step(&replay_id, 2).expect("replay stepped");
        assert!(stepped.contains("\"cursor\":2"));
        assert!(stepped.contains("\"counter\":3"));
        assert!(stepped.contains("\"done\":true"));
    }

    #[test]
    fn import_rejects_wrong_game_version_malformed_and_oversized() {
        let created = new_match("race_to_n", 22).expect("match created");
        let match_id = extract_match_id(&created);
        apply_action(&match_id, "seat_0", "add-1", 0).expect("action applies");
        let exported = export_replay(&match_id).expect("replay exported");
        let matches_before = match_count();
        let replays_before = replay_count();

        let wrong_game = exported.replace("\"game_id\":\"race_to_n\"", "\"game_id\":\"wrong\"");
        assert!(import_replay(&wrong_game)
            .expect_err("wrong game rejected")
            .contains("\"code\":\"unsupported_replay_game\""));

        let wrong_rules = exported.replace(
            "\"rules_version\":\"race_to_n-rules-v1\"",
            "\"rules_version\":\"race_to_n-rules-v99\"",
        );
        assert!(import_replay(&wrong_rules)
            .expect_err("wrong rules rejected")
            .contains("\"code\":\"unsupported_replay_rules\""));

        assert!(import_replay("{ nope")
            .expect_err("malformed replay rejected")
            .contains("\"code\":\"invalid_replay\""));

        let oversized = "x".repeat(MAX_REPLAY_IMPORT_BYTES + 1);
        assert!(import_replay(&oversized)
            .expect_err("oversized replay rejected")
            .contains("\"code\":\"replay_too_large\""));

        assert_eq!(match_count(), matches_before);
        assert_eq!(replay_count(), replays_before);
    }

    #[test]
    fn replay_step_and_reset_match_rust_replay() {
        let created = new_match("race_to_n", 23).expect("match created");
        let match_id = extract_match_id(&created);
        apply_action(&match_id, "seat_0", "add-3", 0).expect("action applies");
        let live_view = get_view(&match_id, None).expect("live view returned");
        let exported = export_replay(&match_id).expect("replay exported");
        let imported = import_replay(&exported).expect("replay imported");
        let replay_id = extract_replay_id(&imported);

        let step = replay_step(&replay_id, 1).expect("replay step returned");
        assert!(step.contains("\"cursor\":1"));
        assert!(step.contains("\"counter\":3"));
        assert!(step.contains("\"type\":\"counter_advanced\""));
        assert!(step.contains(&format!("\"view\":{live_view}")));

        let reset = replay_reset(&replay_id).expect("replay reset returned");
        assert!(reset.contains("\"cursor\":0"));
        assert!(reset.contains("\"counter\":0"));
        assert!(reset.contains("\"effects\":[]"));
    }

    #[test]
    fn replay_export_omits_internal_state_surfaces() {
        let created = new_match("race_to_n", 24).expect("match created");
        let match_id = extract_match_id(&created);
        apply_action(&match_id, "seat_0", "add-1", 0).expect("action applies");
        let exported = export_replay(&match_id).expect("replay exported");

        assert!(exported.contains("\"commands\":["));
        assert!(!exported.contains("initial_snapshot"));
        assert!(!exported.contains("legal_additions"));
        assert!(!exported.contains("\"state\":"));
        assert!(!exported.contains("\"effects\":["));
    }

    #[test]
    fn three_marks_wasm_export_matches_golden_fixture() {
        let commands = vec![AppliedCommand {
            actor_seat: "seat-0".to_owned(),
            action_path: vec!["place/r1c1".to_owned()],
            freshness_token: 0,
        }];
        let exported =
            three_replay_document_json("wasm-exported", 1, &commands).expect("fixture exported");
        let fixture =
            include_str!("../../../games/three_marks/tests/golden_traces/wasm-exported.trace.json");

        assert_eq!(compact_json_layout(fixture), exported);
    }

    fn extract_match_id(created: &str) -> String {
        created
            .split("\"match_id\":\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .expect("match id is present")
            .to_owned()
    }

    fn extract_replay_id(created: &str) -> String {
        created
            .split("\"replay_id\":\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .expect("replay id is present")
            .to_owned()
    }

    fn match_count() -> usize {
        MATCHES.with(|matches| matches.borrow().len())
    }

    fn replay_count() -> usize {
        REPLAYS.with(|replays| replays.borrow().len())
    }

    fn last_output_string() -> String {
        LAST_OUTPUT.with(|last| last.borrow().clone())
    }

    fn compact_json_layout(input: &str) -> String {
        let mut output = String::new();
        let mut in_string = false;
        let mut escaped = false;
        for ch in input.chars() {
            if in_string {
                output.push(ch);
                if escaped {
                    escaped = false;
                } else if ch == '\\' {
                    escaped = true;
                } else if ch == '"' {
                    in_string = false;
                }
            } else if ch == '"' {
                in_string = true;
                output.push(ch);
            } else if !ch.is_whitespace() {
                output.push(ch);
            }
        }
        output
    }
}
