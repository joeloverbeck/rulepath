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
use draughts_lite::{
    apply_action as draughts_apply_action, legal_action_tree as draughts_legal_action_tree,
    project_view as draughts_project_view,
    replay_support::replay_commands as draughts_replay_commands,
    setup_match as draughts_setup_match, DraughtsLiteEffect, DraughtsLiteLevel1Bot,
    DraughtsLiteSeat, DraughtsLiteState,
};
use engine_core::{
    ActionChoice, ActionPath, ActionTree, Actor, CommandEnvelope, EffectCursor, EffectEnvelope,
    EffectLog, RulesVersion, SeatId, Seed, Viewer, VisibilityScope,
};
use high_card_duel::{
    apply_action as high_card_apply_action,
    export_public_observer_replay as high_card_export_public_observer_replay,
    import_public_export as high_card_import_public_export,
    legal_action_tree as high_card_legal_action_tree, project_view as high_card_project_view,
    setup_match as high_card_setup_match, validate_command as high_card_validate_command,
    HighCardDuelEffect, HighCardDuelInternalTrace, HighCardDuelRandomBot, HighCardDuelSeat,
    HighCardDuelState, PublicReplayExport, PublicReplayStep,
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
use token_bazaar::{
    apply_action as token_apply_action, legal_action_tree as token_legal_action_tree,
    project_view as token_project_view, replay_support::replay_commands as token_replay_commands,
    setup_match as token_setup_match, TokenBazaarEffect, TokenBazaarLevel1Bot, TokenBazaarSeat,
    TokenBazaarState,
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
const GAME_DRAUGHTS_LITE: &str = "draughts_lite";
const GAME_DRAUGHTS_LITE_DISPLAY_NAME: &str = "Draughts Lite";
const GAME_HIGH_CARD_DUEL: &str = "high_card_duel";
const GAME_HIGH_CARD_DUEL_DISPLAY_NAME: &str = "High Card Duel";
const GAME_TOKEN_BAZAAR: &str = "token_bazaar";
const GAME_TOKEN_BAZAAR_DISPLAY_NAME: &str = "Token Bazaar";
const RULES_VERSION: u32 = 1;
const SCHEMA_VERSION: u32 = 1;
const SUPPORTED_OPERATIONS: &[&str] = &[
    "feature_report",
    "list_games",
    "new_match",
    "get_view",
    "get_view_for_viewer",
    "get_action_tree",
    "get_action_tree_for_viewer",
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
const DRAUGHTS_LITE_TRACE_RULES_VERSION: &str = "draughts_lite-rules-v1";
const HIGH_CARD_DUEL_TRACE_RULES_VERSION: &str = "high-card-duel-rules-v1";
const TOKEN_BAZAAR_TRACE_RULES_VERSION: &str = "token-bazaar-rules-v1";
const ENGINE_VERSION: &str = "engine-core-0.1.0";
const DATA_VERSION: &str = "1";
const VARIANT_RACE_TO_21: &str = "race_to_21";
const VARIANT_THREE_MARKS_STANDARD: &str = "three_marks_standard";
const VARIANT_COLUMN_FOUR_STANDARD: &str = "column_four_standard";
const VARIANT_DIRECTIONAL_FLIP_STANDARD: &str = "directional_flip_standard";
const VARIANT_DRAUGHTS_LITE_STANDARD: &str = "draughts_lite_standard";
const VARIANT_HIGH_CARD_DUEL_STANDARD: &str = "high_card_duel_standard";
const VARIANT_TOKEN_BAZAAR_STANDARD: &str = "token_bazaar_standard";
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
    DraughtsLite {
        game_id: String,
        seed: u64,
        state: DraughtsLiteState,
        effects: EffectLog<DraughtsLiteEffect>,
        commands: Vec<AppliedCommand>,
    },
    HighCardDuel {
        game_id: String,
        seed: u64,
        state: HighCardDuelState,
        effects: EffectLog<HighCardDuelEffect>,
        commands: Vec<AppliedCommand>,
    },
    TokenBazaar {
        game_id: String,
        seed: u64,
        state: TokenBazaarState,
        effects: EffectLog<TokenBazaarEffect>,
        commands: Vec<AppliedCommand>,
    },
}

#[derive(Clone, Debug)]
struct ReplayRecord {
    game_id: String,
    seed: u64,
    commands: Vec<AppliedCommand>,
    public_timeline: Option<PublicTimelineReplay>,
}

#[derive(Clone, Debug)]
struct PublicTimelineReplay {
    viewer: String,
    steps: Vec<PublicReplayStep>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AppliedCommand {
    actor_seat: String,
    action_path: Vec<String>,
    freshness_token: u64,
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
    DraughtsLite,
    HighCardDuel,
    TokenBazaar,
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
        RegisteredGame::DraughtsLite,
        RegisteredGame::HighCardDuel,
        RegisteredGame::TokenBazaar,
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
            RegisteredGame::DraughtsLite => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":[\"{}\"]}}",
                escape_json(GAME_DRAUGHTS_LITE),
                escape_json(GAME_DRAUGHTS_LITE_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                escape_json(VARIANT_DRAUGHTS_LITE_STANDARD)
            ),
            RegisteredGame::HighCardDuel => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":[\"{}\"],\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"],\"hidden_information\":true,\"tags\":[\"hidden_info\",\"viewer_filtered\",\"public_replay_export\"]}}",
                escape_json(GAME_HIGH_CARD_DUEL),
                escape_json(GAME_HIGH_CARD_DUEL_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                escape_json(VARIANT_HIGH_CARD_DUEL_STANDARD)
            ),
            RegisteredGame::TokenBazaar => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":[\"{}\"],\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"],\"hidden_information\":false,\"tags\":[\"public_accounting\",\"economy\",\"public_replay_export\"]}}",
                escape_json(GAME_TOKEN_BAZAAR),
                escape_json(GAME_TOKEN_BAZAAR_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                escape_json(VARIANT_TOKEN_BAZAAR_STANDARD)
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
        RegisteredGame::DraughtsLite => {
            let seats = seats();
            let state =
                draughts_setup_match(Seed(seed), &seats, &draughts_lite::SetupOptions::default())
                    .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::DraughtsLite {
                        game_id: GAME_DRAUGHTS_LITE.to_owned(),
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
                escape_json(VARIANT_DRAUGHTS_LITE_STANDARD)
            ))
        }
        RegisteredGame::HighCardDuel => {
            let seats = seats();
            let state =
                high_card_setup_match(Seed(seed), &seats, &high_card_duel::SetupOptions::default())
                    .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::HighCardDuel {
                        game_id: GAME_HIGH_CARD_DUEL.to_owned(),
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
                escape_json(VARIANT_HIGH_CARD_DUEL_STANDARD)
            ))
        }
        RegisteredGame::TokenBazaar => {
            let seats = seats();
            let state =
                token_setup_match(Seed(seed), &seats, &token_bazaar::SetupOptions::default())
                    .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::TokenBazaar {
                        game_id: GAME_TOKEN_BAZAAR.to_owned(),
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
                escape_json(VARIANT_TOKEN_BAZAAR_STANDARD)
            ))
        }
    }
}

pub fn get_view(match_id: &str, viewer_seat: Option<&str>) -> Result<String, String> {
    with_match(match_id, |record| match record {
        MatchRecord::RaceToN { game_id, state, .. } => {
            resolve_game(game_id)?;
            let _viewer = race_viewer_for_seat(state, viewer_seat)?;
            Ok(project_view(state).to_json())
        }
        MatchRecord::ThreeMarks { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = three_viewer_for_seat(state, viewer_seat)?;
            Ok(three_project_view(state, &viewer).to_json())
        }
        MatchRecord::ColumnFour { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = column_viewer_for_seat(state, viewer_seat)?;
            Ok(column_view_json(&column_project_view(state, &viewer)))
        }
        MatchRecord::DirectionalFlip { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = directional_viewer_for_seat(state, viewer_seat)?;
            Ok(directional_view_json(&directional_project_view(
                state, &viewer,
            )))
        }
        MatchRecord::DraughtsLite { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = draughts_viewer_for_seat(state, viewer_seat)?;
            Ok(draughts_view_json(&draughts_project_view(state, &viewer)))
        }
        MatchRecord::HighCardDuel { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = high_card_viewer_for_seat(state, viewer_seat)?;
            Ok(high_card_view_json(&high_card_project_view(state, &viewer)))
        }
        MatchRecord::TokenBazaar { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = token_viewer_for_seat(state, viewer_seat)?;
            Ok(token_view_json(&token_project_view(state, &viewer)))
        }
    })
}

pub fn get_action_tree(match_id: &str, actor_seat: &str) -> Result<String, String> {
    get_action_tree_for_viewer(match_id, actor_seat, Some(actor_seat))
}

pub fn get_action_tree_for_viewer(
    match_id: &str,
    actor_seat: &str,
    viewer_seat: Option<&str>,
) -> Result<String, String> {
    with_match(match_id, |record| match record {
        MatchRecord::RaceToN { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_race_seat(actor_seat)?;
            if !race_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = race_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&legal_action_tree(state, &actor)))
        }
        MatchRecord::ThreeMarks { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_three_seat(actor_seat)?;
            if !three_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = three_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&three_legal_action_tree(state, &actor)))
        }
        MatchRecord::ColumnFour { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_column_seat(actor_seat)?;
            if !column_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = column_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&column_legal_action_tree(state, &actor)))
        }
        MatchRecord::DirectionalFlip { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_directional_seat(actor_seat)?;
            if !directional_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = directional_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&directional_legal_action_tree(
                state, &actor,
            )))
        }
        MatchRecord::DraughtsLite { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_draughts_seat(actor_seat)?;
            if !draughts_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = draughts_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&draughts_legal_action_tree(state, &actor)))
        }
        MatchRecord::HighCardDuel { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_high_card_seat(actor_seat)?;
            if !high_card_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = high_card_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&high_card_legal_action_tree(
                state, &actor,
            )))
        }
        MatchRecord::TokenBazaar { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_token_seat(actor_seat)?;
            if !token_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = token_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&token_legal_action_tree(state, &actor)))
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
        MatchRecord::DraughtsLite {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_draughts_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: draughts_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                draughts_lite::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = draughts_apply_action(state, action);
            let effect_json = draughts_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_draughts_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                draughts_view_json(&draughts_project_view(state, &Viewer { seat_id: None }))
            ))
        }
        MatchRecord::HighCardDuel {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_high_card_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: high_card_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = high_card_validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = high_card_apply_action(state, action);
            let viewer = high_card_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = high_card_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_high_card_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                high_card_view_json(&high_card_project_view(state, &viewer))
            ))
        }
        MatchRecord::TokenBazaar {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_token_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: token_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                token_bazaar::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = token_apply_action(state, action);
            let effect_json = token_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_token_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                token_view_json(&token_project_view(state, &Viewer { seat_id: None }))
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
        MatchRecord::DraughtsLite {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_draughts_seat(actor_seat)?;
            let decision = DraughtsLiteLevel1Bot::new(Seed(bot_seed))
                .select_decision(state, seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: draughts_actor_for_seat(state, seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                draughts_lite::validate_command(state, &command).map_err(diagnostic_json)?;
            let mut effects = decision.effects;
            effects.extend(draughts_apply_action(state, action));
            let effect_json = draughts_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_draughts_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                draughts_view_json(&draughts_project_view(state, &Viewer { seat_id: None }))
            ))
        }
        MatchRecord::HighCardDuel {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_high_card_seat(actor_seat)?;
            let decision = HighCardDuelRandomBot::new(Seed(bot_seed))
                .select_decision(state, seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: high_card_actor_for_seat(state, seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = high_card_validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = high_card_apply_action(state, action);
            let viewer = high_card_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = high_card_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_high_card_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                high_card_view_json(&high_card_project_view(state, &viewer))
            ))
        }
        MatchRecord::TokenBazaar {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_token_seat(actor_seat)?;
            let decision = TokenBazaarLevel1Bot::new(Seed(bot_seed))
                .select_decision(state, seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: token_actor_for_seat(state, seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                token_bazaar::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = token_apply_action(state, action);
            let effect_json = token_effects_json(&effects);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_token_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                token_view_json(&token_project_view(state, &Viewer { seat_id: None }))
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
        MatchRecord::DraughtsLite {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = draughts_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        draughts_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
        MatchRecord::HighCardDuel {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = high_card_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        high_card_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
        MatchRecord::TokenBazaar {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = token_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        token_effect_json(&logged.envelope)
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
        MatchRecord::DraughtsLite {
            game_id,
            seed,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            draughts_replay_document_json(&format!("export-{match_id}"), *seed, commands)
        }
        MatchRecord::HighCardDuel {
            game_id,
            seed,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let trace = HighCardDuelInternalTrace {
                schema_version: SCHEMA_VERSION,
                game_id: high_card_duel::GAME_ID.to_owned(),
                rules_version: high_card_duel::RULES_VERSION_LABEL.to_owned(),
                variant: high_card_duel::VARIANT_ID.to_owned(),
                seed: *seed,
                command_paths: commands
                    .iter()
                    .map(|command| command.action_path.clone())
                    .collect(),
            };
            Ok(high_card_export_public_observer_replay(&trace).to_json())
        }
        MatchRecord::TokenBazaar {
            game_id,
            seed,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            token_replay_document_json(&format!("export-{match_id}"), *seed, commands)
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
    if is_high_card_public_export(doc) {
        return import_high_card_public_replay(doc);
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
        && parsed.game_id != GAME_DRAUGHTS_LITE
        && parsed.game_id != GAME_HIGH_CARD_DUEL
        && parsed.game_id != GAME_TOKEN_BAZAAR
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
        RegisteredGame::DraughtsLite => {
            let (state, effects) =
                draughts_replay_to_cursor(parsed.seed, &parsed.commands, command_count)?;
            (
                draughts_view_json(&draughts_project_view(&state, &Viewer { seat_id: None })),
                effects.len(),
            )
        }
        RegisteredGame::HighCardDuel => {
            let (state, effects) =
                high_card_replay_to_cursor(parsed.seed, &parsed.commands, command_count)?;
            (
                high_card_view_json(&high_card_project_view(&state, &Viewer { seat_id: None })),
                effects.len(),
            )
        }
        RegisteredGame::TokenBazaar => {
            let (state, effects) =
                token_replay_to_cursor(parsed.seed, &parsed.commands, command_count)?;
            (
                token_view_json(&token_project_view(&state, &Viewer { seat_id: None })),
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
                public_timeline: None,
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
    with_replay(replay_id, |record| {
        if let Some(timeline) = &record.public_timeline {
            return Ok(public_replay_step_json(replay_id, cursor, timeline));
        }
        match resolve_game(&record.game_id)? {
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
            RegisteredGame::DraughtsLite => {
                let bounded_cursor = cursor.min(record.commands.len());
                let (state, effects) =
                    draughts_replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
                Ok(draughts_replay_step_json(
                    replay_id,
                    bounded_cursor,
                    record.commands.len(),
                    &state,
                    &effects,
                ))
            }
            RegisteredGame::HighCardDuel => {
                let bounded_cursor = cursor.min(record.commands.len());
                let (state, effects) =
                    high_card_replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
                Ok(high_card_replay_step_json(
                    replay_id,
                    bounded_cursor,
                    record.commands.len(),
                    &state,
                    &effects,
                ))
            }
            RegisteredGame::TokenBazaar => {
                let bounded_cursor = cursor.min(record.commands.len());
                let (state, effects) =
                    token_replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
                Ok(token_replay_step_json(
                    replay_id,
                    bounded_cursor,
                    record.commands.len(),
                    &state,
                    &effects,
                ))
            }
        }
    })
}

pub fn replay_reset(replay_id: &str) -> Result<String, String> {
    replay_step(replay_id, 0)
}

fn is_high_card_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("public_observer_projection_v1")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(high_card_duel::GAME_ID)
    )
}

fn import_high_card_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let rules_version = string_field(doc, "rules_version")?;
    if rules_version != high_card_duel::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {rules_version}"),
        ));
    }
    let variant = string_field(doc, "variant")?;
    if variant != high_card_duel::VARIANT_ID {
        return Err(diagnostic_string(
            "unsupported_replay_variant",
            &format!("unsupported replay variant: {variant}"),
        ));
    }
    let viewer = string_field(doc, "viewer")?;
    if viewer != "observer" {
        return Err(diagnostic_string(
            "unsupported_replay_viewer",
            &format!("unsupported replay viewer: {viewer}"),
        ));
    }
    let steps = parse_public_replay_steps(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export = PublicReplayExport {
        schema_version: SCHEMA_VERSION,
        export_class: "public_observer_projection_v1".to_owned(),
        viewer: viewer.clone(),
        game_id: high_card_duel::GAME_ID.to_owned(),
        rules_version,
        variant,
        steps,
    };
    let timeline = high_card_import_public_export(&export);
    let replay_id = next_replay_id(GAME_HIGH_CARD_DUEL);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_HIGH_CARD_DUEL.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: timeline.viewer.clone(),
                    steps: timeline.steps.clone(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_HIGH_CARD_DUEL),
        escape_json(&timeline.viewer),
        timeline.steps.len()
    ))
}

fn public_replay_step_json(
    replay_id: &str,
    cursor: usize,
    timeline: &PublicTimelineReplay,
) -> String {
    let total_steps = timeline.steps.len().saturating_sub(1);
    let bounded_cursor = cursor.min(total_steps);
    let step = timeline.steps.get(bounded_cursor);
    let public_effects = step.map_or_else(String::new, |step| {
        step.public_effects
            .iter()
            .map(|effect| format!("\"{}\"", escape_json(effect)))
            .collect::<Vec<_>>()
            .join(",")
    });
    let redacted_command_summary = step.map_or("", |step| step.redacted_command_summary.as_str());
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_steps\":{},\"public_export\":true,\"viewer\":\"{}\",\"view\":null,\"effects\":[],\"public_effects\":[{}],\"redacted_command_summary\":\"{}\"}}",
        escape_json(replay_id),
        bounded_cursor,
        total_steps,
        escape_json(&timeline.viewer),
        public_effects,
        escape_json(redacted_command_summary)
    )
}

fn resolve_game(game_id: &str) -> Result<RegisteredGame, String> {
    match game_id {
        GAME_RACE_TO_N => Ok(RegisteredGame::RaceToN),
        GAME_THREE_MARKS => Ok(RegisteredGame::ThreeMarks),
        GAME_COLUMN_FOUR => Ok(RegisteredGame::ColumnFour),
        GAME_DIRECTIONAL_FLIP => Ok(RegisteredGame::DirectionalFlip),
        GAME_DRAUGHTS_LITE => Ok(RegisteredGame::DraughtsLite),
        GAME_HIGH_CARD_DUEL => Ok(RegisteredGame::HighCardDuel),
        GAME_TOKEN_BAZAAR => Ok(RegisteredGame::TokenBazaar),
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
        RegisteredGame::DraughtsLite => DRAUGHTS_LITE_TRACE_RULES_VERSION,
        RegisteredGame::HighCardDuel => HIGH_CARD_DUEL_TRACE_RULES_VERSION,
        RegisteredGame::TokenBazaar => TOKEN_BAZAAR_TRACE_RULES_VERSION,
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

fn parse_draughts_seat(value: &str) -> Result<DraughtsLiteSeat, String> {
    match value {
        "seat-0" => Ok(DraughtsLiteSeat::Seat0),
        "seat-1" => Ok(DraughtsLiteSeat::Seat1),
        _ => DraughtsLiteSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

fn trace_draughts_seat(seat: DraughtsLiteSeat) -> &'static str {
    match seat {
        DraughtsLiteSeat::Seat0 => "seat-0",
        DraughtsLiteSeat::Seat1 => "seat-1",
    }
}

fn parse_high_card_seat(value: &str) -> Result<HighCardDuelSeat, String> {
    match value {
        "seat-0" => Ok(HighCardDuelSeat::Seat0),
        "seat-1" => Ok(HighCardDuelSeat::Seat1),
        _ => HighCardDuelSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

fn trace_high_card_seat(seat: HighCardDuelSeat) -> &'static str {
    match seat {
        HighCardDuelSeat::Seat0 => "seat-0",
        HighCardDuelSeat::Seat1 => "seat-1",
    }
}

fn parse_token_seat(value: &str) -> Result<TokenBazaarSeat, String> {
    match value {
        "seat-0" => Ok(TokenBazaarSeat::Seat0),
        "seat-1" => Ok(TokenBazaarSeat::Seat1),
        _ => TokenBazaarSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

fn trace_token_seat(seat: TokenBazaarSeat) -> &'static str {
    match seat {
        TokenBazaarSeat::Seat0 => "seat_0",
        TokenBazaarSeat::Seat1 => "seat_1",
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

fn draughts_actor_for_seat(
    state: &DraughtsLiteState,
    seat: DraughtsLiteSeat,
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

fn high_card_actor_for_seat(
    state: &HighCardDuelState,
    seat: HighCardDuelSeat,
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

fn token_actor_for_seat(state: &TokenBazaarState, seat: TokenBazaarSeat) -> Result<Actor, String> {
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

fn draughts_viewer_for_seat(
    state: &DraughtsLiteState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_draughts_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

fn high_card_viewer_for_seat(
    state: &HighCardDuelState,
    seat: Option<&str>,
) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_high_card_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

fn token_viewer_for_seat(state: &TokenBazaarState, seat: Option<&str>) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_token_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

fn race_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: RaceSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_race_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

fn three_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: ThreeMarksSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_three_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

fn column_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: ColumnFourSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_column_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

fn directional_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: DirectionalFlipSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_directional_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

fn draughts_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: DraughtsLiteSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_draughts_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

fn high_card_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: HighCardDuelSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_high_card_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

fn token_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    actor: TokenBazaarSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_token_seat)
        .transpose()
        .map(|viewer| viewer == Some(actor))
}

fn parse_action_path(action_path: &str) -> ActionPath {
    ActionPath {
        segments: if action_path.is_empty() {
            Vec::new()
        } else {
            action_path.split('>').map(str::to_owned).collect()
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

fn draughts_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(DraughtsLiteState, Vec<EffectEnvelope<DraughtsLiteEffect>>), String> {
    let seats = seats();
    let mut state =
        draughts_setup_match(Seed(seed), &seats, &draughts_lite::SetupOptions::default())
            .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_draughts_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: draughts_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = draughts_lite::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(draughts_apply_action(&mut state, action));
    }
    Ok((state, all_effects))
}

fn high_card_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(HighCardDuelState, Vec<EffectEnvelope<HighCardDuelEffect>>), String> {
    let seats = seats();
    let mut state =
        high_card_setup_match(Seed(seed), &seats, &high_card_duel::SetupOptions::default())
            .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_high_card_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: high_card_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = high_card_validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(high_card_apply_action(&mut state, action));
    }
    Ok((state, all_effects))
}

fn token_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(TokenBazaarState, Vec<EffectEnvelope<TokenBazaarEffect>>), String> {
    let seats = seats();
    let mut state = token_setup_match(Seed(seed), &seats, &token_bazaar::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_token_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: token_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = token_bazaar::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(token_apply_action(&mut state, action));
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

fn empty_action_tree_json(freshness_token: engine_core::FreshnessToken) -> String {
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
    format!(
        "{{\"segment\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\",\"metadata\":[{}],\"tags\":[{}],\"next\":{}}}",
        escape_json(&choice.segment),
        escape_json(&choice.label),
        escape_json(&choice.accessibility_label),
        metadata,
        tags,
        next
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

fn draughts_effects_json(effects: &[EffectEnvelope<DraughtsLiteEffect>]) -> String {
    let body = effects
        .iter()
        .map(draughts_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn token_effects_json(effects: &[EffectEnvelope<TokenBazaarEffect>]) -> String {
    let body = effects
        .iter()
        .map(token_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn high_card_effects_json(
    effects: &[EffectEnvelope<HighCardDuelEffect>],
    viewer: &Viewer,
) -> String {
    let body = effects
        .iter()
        .filter(|effect| effect_visible_to_viewer(&effect.visibility, viewer))
        .map(high_card_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn effect_visible_to_viewer(visibility: &VisibilityScope, viewer: &Viewer) -> bool {
    match visibility {
        VisibilityScope::Public => true,
        VisibilityScope::PrivateToSeat(seat) => viewer.seat_id.as_ref() == Some(seat),
    }
}

fn race_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let command_segments = single_segment_commands(commands)?;
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
    let command_segments = single_segment_commands(commands)?;
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
    let command_segments = single_segment_commands(commands)?;
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
    let command_segments = single_segment_commands(commands)?;
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

fn draughts_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let command_paths = commands
        .iter()
        .map(|command| command.action_path.clone())
        .collect::<Vec<_>>();
    let hashes = draughts_replay_commands(seed, &command_paths);
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
            draughts_lite::TerminalOutcome::Win { seat } => format!(
                "{{\"terminal\":true,\"winner\":\"{}\",\"kind\":\"win\"}}",
                trace_draughts_seat(seat)
            ),
        },
    );

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported_replay\",\"note\":\"Replay exported by the Rulepath WASM API from the Rust command log with ordered multi-segment action paths preserved.\",\"migration_update_note\":\"Generated by Gate 7 WASM replay export; expected hashes are computed by Draughts Lite Rust replay support.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat-0\",\"player_id\":\"player-0\"}},{{\"seat_id\":\"seat-1\",\"player_id\":\"player-1\"}}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_private_view_hashes\":{{\"not_applicable\":\"draughts_lite is perfect-information and has no private-view API.\"}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_outcome\":{},\"expected_terminal_state\":{},\"not_applicable\":{{\"hidden_information\":\"draughts_lite is perfect-information and has no hidden state to redact.\",\"stochastic_game_events\":\"draughts_lite game rules use no randomness; bot RNG is not replayed from exported documents because resolved commands are recorded.\",\"private_view_hashes\":\"draughts_lite has no private-view API.\",\"preview_hashes\":\"draughts_lite uses action-tree metadata and semantic effects rather than a separate preview hash surface in Gate 7.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_DRAUGHTS_LITE),
        escape_json(DRAUGHTS_LITE_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_DRAUGHTS_LITE_STANDARD),
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

fn token_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let command_paths = commands
        .iter()
        .map(|command| command.action_path.clone())
        .collect::<Vec<_>>();
    let hashes = token_replay_commands(seed, &command_paths);
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
    let (terminal, winner, draw) = match hashes.terminal_outcome {
        Some(token_bazaar::TerminalOutcome::Win { seat }) => {
            (true, format!("\"{}\"", seat.as_str()), false)
        }
        Some(token_bazaar::TerminalOutcome::Draw) => (true, "null".to_owned(), true),
        None => (false, "null".to_owned(), false),
    };

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"token-bazaar-{}\",\"fixture_kind\":\"wasm\",\"purpose\":\"public_export_round_trip\",\"note\":\"Token Bazaar public_export_round_trip replay fixture.\",\"migration_update_note\":\"Initial Token Bazaar Trace Schema v1 fixture established by GAT9TOKBAZBRO-010.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat-0\",\"player_id\":\"player-0\"}},{{\"seat_id\":\"seat-1\",\"player_id\":\"player-1\"}}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_diagnostic_hashes\":null,\"expected_public_export_hashes\":{{\"final\":{}}},\"expected_outcome\":{{\"terminal\":{},\"winner\":{},\"draw\":{}}},\"expected_terminal_state\":{{\"terminal\":{},\"winner\":{},\"draw\":{}}},\"not_applicable\":{{\"hidden_information\":\"token_bazaar is fully public.\",\"stochastic_game_events\":\"token_bazaar game rules use no randomness.\",\"private_view_hashes\":\"token_bazaar observer and seat views are identical.\",\"preview_hashes\":\"token_bazaar uses legal action metadata rather than a separate preview hash.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_TOKEN_BAZAAR),
        escape_json(TOKEN_BAZAAR_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_TOKEN_BAZAAR_STANDARD),
        commands_json,
        checkpoints,
        hashes.final_state_hash.0,
        hashes.effect_hash.0,
        hashes.action_tree_hash.0,
        hashes.public_view_hash.0,
        hashes.replay_hash.0,
        hashes.replay_hash.0,
        terminal,
        winner,
        draw,
        terminal,
        winner,
        draw
    ))
}

fn single_segment_commands(commands: &[AppliedCommand]) -> Result<Vec<String>, String> {
    commands
        .iter()
        .map(|command| {
            if command.action_path.len() != 1 {
                return Err(diagnostic_string(
                    "unsupported_replay_action_path",
                    "this game exports one-segment action paths only",
                ));
            }
            Ok(command.action_path[0].clone())
        })
        .collect()
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

fn draughts_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &DraughtsLiteState,
    effects: &[EffectEnvelope<DraughtsLiteEffect>],
) -> String {
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        draughts_view_json(&draughts_project_view(state, &Viewer { seat_id: None })),
        draughts_effects_json(effects)
    )
}

fn token_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &TokenBazaarState,
    effects: &[EffectEnvelope<TokenBazaarEffect>],
) -> String {
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        token_view_json(&token_project_view(state, &Viewer { seat_id: None })),
        token_effects_json(effects)
    )
}

fn high_card_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &HighCardDuelState,
    effects: &[EffectEnvelope<HighCardDuelEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        high_card_view_json(&high_card_project_view(state, &viewer)),
        high_card_effects_json(effects, &viewer)
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

fn draughts_view_json(view: &draughts_lite::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"board_rows\":{},\"board_columns\":{},\"cells\":[{}],\"active_seat\":{},\"ply_count\":{},\"command_count\":{},\"status_label\":\"{}\",\"freshness_token\":{},\"terminal_kind\":\"{}\",\"winning_seat\":{},\"private_view_status\":\"{}\",\"hidden_fields\":[{}],\"ui\":{},\"replay_step_index\":{}}}",
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
            .map(draughts_cell_json)
            .collect::<Vec<_>>()
            .join(","),
        option_draughts_seat_json(view.active_seat),
        view.ply_count,
        view.command_count,
        escape_json(&view.status_label),
        view.freshness_token.0,
        draughts_terminal_kind(&view.terminal),
        option_draughts_seat_json(draughts_terminal_winner(&view.terminal)),
        escape_json(&view.private_view.status),
        string_array(&view.private_view.hidden_fields),
        draughts_ui_json(&view.ui),
        view.replay_step_index
            .map_or_else(|| "null".to_owned(), |step| step.to_string())
    )
}

fn draughts_cell_json(cell: &draughts_lite::CellView) -> String {
    format!(
        "{{\"cell\":\"{}\",\"cell_id\":\"{}\",\"row\":{},\"column\":{},\"playable\":{},\"presentation_token\":\"{}\",\"accessibility_label\":\"{}\",\"occupancy\":\"{}\",\"owner\":{},\"piece_id\":{},\"piece_kind\":{},\"piece_token_key\":{},\"piece_shape_label\":{},\"piece_pattern_label\":{},\"piece_label\":{},\"piece_accessibility_label\":{}}}",
        cell.cell.id(),
        escape_json(&cell.cell_id),
        cell.row,
        cell.column,
        cell.playable,
        escape_json(&cell.presentation_token),
        escape_json(&cell.accessibility_label),
        escape_json(&cell.occupancy),
        option_draughts_seat_json(cell.owner),
        option_string_json(cell.piece_id.as_deref()),
        cell.piece_kind
            .map_or_else(|| "null".to_owned(), |kind| format!("\"{}\"", kind.as_str())),
        option_string_json(cell.piece_token_key.as_deref()),
        option_string_json(cell.piece_shape_label.as_deref()),
        option_string_json(cell.piece_pattern_label.as_deref()),
        option_string_json(cell.piece_label.as_deref()),
        option_string_json(cell.piece_accessibility_label.as_deref())
    )
}

fn draughts_ui_json(ui: &draughts_lite::UiMetadata) -> String {
    format!(
        "{{\"board_label\":\"{}\",\"row_count\":{},\"column_count\":{},\"playable_cell_token\":\"{}\",\"non_playable_cell_token\":\"{}\",\"first_man_token_key\":\"{}\",\"first_man_shape_label\":\"{}\",\"first_crown_token_key\":\"{}\",\"first_crown_shape_label\":\"{}\",\"second_man_token_key\":\"{}\",\"second_man_shape_label\":\"{}\",\"second_crown_token_key\":\"{}\",\"second_crown_shape_label\":\"{}\"}}",
        escape_json(&ui.board_label),
        ui.row_count,
        ui.column_count,
        escape_json(&ui.playable_cell_token),
        escape_json(&ui.non_playable_cell_token),
        escape_json(&ui.first_man_token_key),
        escape_json(&ui.first_man_shape_label),
        escape_json(&ui.first_crown_token_key),
        escape_json(&ui.first_crown_shape_label),
        escape_json(&ui.second_man_token_key),
        escape_json(&ui.second_man_shape_label),
        escape_json(&ui.second_crown_token_key),
        escape_json(&ui.second_crown_shape_label)
    )
}

fn draughts_effect_json(effect: &EffectEnvelope<DraughtsLiteEffect>) -> String {
    let payload = match &effect.payload {
        DraughtsLiteEffect::MoveCommitted {
            action_path,
            seat,
            piece_id,
            start_cell,
            final_cell,
            move_kind,
            path_length,
        } => format!(
            "{{\"type\":\"move_committed\",\"action_path\":[{}],\"seat\":\"{}\",\"piece_id\":\"{}\",\"start_cell\":\"{}\",\"final_cell\":\"{}\",\"move_kind\":\"{}\",\"path_length\":{}}}",
            string_array(action_path),
            seat.as_str(),
            piece_id.stable_id(),
            start_cell.id(),
            final_cell.id(),
            draughts_move_kind(*move_kind),
            path_length
        ),
        DraughtsLiteEffect::QuietStep {
            piece_id,
            origin,
            landing,
            piece_kind_before,
            piece_kind_after,
        } => format!(
            "{{\"type\":\"quiet_step\",\"piece_id\":\"{}\",\"origin\":\"{}\",\"landing\":\"{}\",\"piece_kind_before\":\"{}\",\"piece_kind_after\":\"{}\"}}",
            piece_id.stable_id(),
            origin.id(),
            landing.id(),
            piece_kind_before.as_str(),
            piece_kind_after.as_str()
        ),
        DraughtsLiteEffect::CaptureStep {
            piece_id,
            origin,
            landing,
            captured_cell,
            captured_piece_id,
            captured_owner,
        } => format!(
            "{{\"type\":\"capture_step\",\"piece_id\":\"{}\",\"origin\":\"{}\",\"landing\":\"{}\",\"captured_cell\":\"{}\",\"captured_piece_id\":\"{}\",\"captured_owner\":\"{}\"}}",
            piece_id.stable_id(),
            origin.id(),
            landing.id(),
            captured_cell.id(),
            captured_piece_id.stable_id(),
            captured_owner.as_str()
        ),
        DraughtsLiteEffect::Promotion {
            piece_id,
            seat,
            cell,
            from,
            to,
            during_capture,
        } => format!(
            "{{\"type\":\"promotion\",\"piece_id\":\"{}\",\"seat\":\"{}\",\"cell\":\"{}\",\"from\":\"{}\",\"to\":\"{}\",\"during_capture\":{}}}",
            piece_id.stable_id(),
            seat.as_str(),
            cell.id(),
            from.as_str(),
            to.as_str(),
            during_capture
        ),
        DraughtsLiteEffect::ForcedCaptureAvailable {
            active_seat,
            capture_origin_count,
            explanation,
        } => format!(
            "{{\"type\":\"forced_capture_available\",\"active_seat\":\"{}\",\"capture_origin_count\":{},\"explanation\":\"{}\"}}",
            active_seat.as_str(),
            capture_origin_count,
            escape_json(explanation)
        ),
        DraughtsLiteEffect::ForcedContinuationRequired {
            piece_id,
            current_landing,
            continuation_destination_count,
            explanation,
        } => format!(
            "{{\"type\":\"forced_continuation_required\",\"piece_id\":\"{}\",\"current_landing\":\"{}\",\"continuation_destination_count\":{},\"explanation\":\"{}\"}}",
            piece_id.stable_id(),
            current_landing.id(),
            continuation_destination_count,
            escape_json(explanation)
        ),
        DraughtsLiteEffect::InvalidCommand {
            code,
            public_message,
            rejected_action_path,
        } => format!(
            "{{\"type\":\"invalid_command\",\"code\":\"{}\",\"public_message\":\"{}\",\"rejected_action_path\":[{}]}}",
            escape_json(code),
            escape_json(public_message),
            string_array(rejected_action_path)
        ),
        DraughtsLiteEffect::TerminalWin {
            winner,
            loser,
            reason,
        } => format!(
            "{{\"type\":\"terminal_win\",\"winner\":\"{}\",\"loser\":\"{}\",\"reason\":\"{}\"}}",
            winner.as_str(),
            loser.as_str(),
            draughts_terminal_reason(*reason)
        ),
        DraughtsLiteEffect::BotChoseAction {
            level,
            policy_id,
            action_path,
            rationale,
        } => format!(
            "{{\"type\":\"bot_chose_action\",\"level\":{},\"policy_id\":\"{}\",\"action_path\":[{}],\"rationale\":\"{}\"}}",
            level,
            escape_json(policy_id),
            string_array(action_path),
            escape_json(rationale)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn high_card_view_json(view: &high_card_duel::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"round_number\":{},\"round_limit\":{},\"phase\":\"{}\",\"active_seat\":{},\"lead_seat\":{},\"reply_seat\":{},\"score\":{},\"hand_counts\":{},\"deck_count\":{},\"commitments\":{},\"revealed_cards\":[{}],\"terminal_kind\":\"{}\",\"winning_seat\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.round_number,
        view.round_limit,
        view.phase.as_str(),
        option_high_card_seat_json(view.active_seat),
        option_high_card_seat_json(view.lead_seat),
        option_high_card_seat_json(view.reply_seat),
        high_card_score_json(view.score),
        high_card_hand_counts_json(&view.hand_counts),
        view.deck_count,
        high_card_commitments_json(&view.commitments),
        view.revealed_cards
            .iter()
            .map(high_card_revealed_round_json)
            .collect::<Vec<_>>()
            .join(","),
        high_card_terminal_kind(&view.terminal),
        option_high_card_seat_json(high_card_terminal_winner(&view.terminal)),
        view.freshness_token.0,
        high_card_private_view_json(&view.private_view),
        high_card_ui_json(&view.ui)
    )
}

fn high_card_card_json(card: &high_card_duel::CardView) -> String {
    format!(
        "{{\"card_id\":\"{}\",\"rank\":{},\"sigil\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.card_id),
        card.rank,
        escape_json(&card.sigil),
        escape_json(&card.accessibility_label)
    )
}

fn high_card_score_json(score: high_card_duel::Score) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{}}}",
        score.seat_0, score.seat_1
    )
}

fn high_card_hand_counts_json(counts: &high_card_duel::HandCountsView) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{}}}",
        counts.seat_0, counts.seat_1
    )
}

fn high_card_commitments_json(commitments: &high_card_duel::CommitmentViews) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{}}}",
        high_card_commitment_json(&commitments.seat_0),
        high_card_commitment_json(&commitments.seat_1)
    )
}

fn high_card_commitment_json(commitment: &high_card_duel::CommitmentView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"status\":\"{}\",\"card\":{},\"accessibility_label\":\"{}\"}}",
        commitment.seat.as_str(),
        escape_json(&commitment.status),
        commitment
            .card
            .as_ref()
            .map_or_else(|| "null".to_owned(), high_card_card_json),
        escape_json(&commitment.accessibility_label)
    )
}

fn high_card_revealed_round_json(round: &high_card_duel::RevealedRoundView) -> String {
    format!(
        "{{\"round_number\":{},\"seat_0_card\":{},\"seat_1_card\":{},\"winner\":{}}}",
        round.round_number,
        high_card_card_json(&round.seat_0_card),
        high_card_card_json(&round.seat_1_card),
        option_high_card_seat_json(round.winner)
    )
}

fn high_card_private_view_json(private_view: &high_card_duel::PrivateView) -> String {
    match private_view {
        high_card_duel::PrivateView::Observer => {
            "{\"status\":\"observer\",\"hand\":[],\"own_commitment\":null}".to_owned()
        }
        high_card_duel::PrivateView::Seat {
            seat,
            hand,
            own_commitment,
        } => {
            let hand = hand
                .iter()
                .map(high_card_card_json)
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{{\"status\":\"seat\",\"seat\":\"{}\",\"hand\":[{}],\"own_commitment\":{}}}",
                seat.as_str(),
                hand,
                own_commitment
                    .as_ref()
                    .map_or_else(|| "null".to_owned(), high_card_card_json)
            )
        }
    }
}

fn high_card_ui_json(ui: &high_card_duel::UiMetadata) -> String {
    format!(
        "{{\"table_label\":\"{}\",\"card_back_token\":\"{}\",\"own_card_token\":\"{}\",\"revealed_card_token\":\"{}\",\"empty_commitment_token\":\"{}\",\"face_down_commitment_token\":\"{}\",\"commit_action_label\":\"{}\",\"observer_disabled_reason\":\"{}\"}}",
        escape_json(&ui.table_label),
        escape_json(&ui.card_back_token),
        escape_json(&ui.own_card_token),
        escape_json(&ui.revealed_card_token),
        escape_json(&ui.empty_commitment_token),
        escape_json(&ui.face_down_commitment_token),
        escape_json(&ui.commit_action_label),
        escape_json(&ui.observer_disabled_reason)
    )
}

fn token_view_json(view: &token_bazaar::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"supply\":{},\"inventories\":[{},{}],\"scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"turns_taken\":{{\"seat_0\":{},\"seat_1\":{},\"turns_per_seat\":{}}},\"active_seat\":{},\"market_slots\":[{}],\"queue_remaining\":{},\"fulfilled\":{{\"seat_0\":[{}],\"seat_1\":[{}]}},\"legal_actions\":[{}],\"terminal\":{},\"freshness_token\":{},\"recent_effects\":[{}],\"private_view_status\":\"{}\",\"hidden_fields\":[{}],\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        token_supply_json(view.supply),
        token_inventory_json(&view.inventories[0]),
        token_inventory_json(&view.inventories[1]),
        view.scores[0],
        view.scores[1],
        view.turns_taken[0],
        view.turns_taken[1],
        view.turns_per_seat,
        option_token_seat_json(view.active_seat),
        view.market_slots
            .iter()
            .map(token_market_slot_json)
            .collect::<Vec<_>>()
            .join(","),
        view.queue_remaining,
        string_array(&view.fulfilled[0]),
        string_array(&view.fulfilled[1]),
        view.legal_actions
            .iter()
            .map(token_legal_action_json)
            .collect::<Vec<_>>()
            .join(","),
        token_terminal_json(&view.terminal),
        view.freshness_token.0,
        view.recent_effects
            .iter()
            .map(token_effect_view_json)
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&view.private_view_status),
        string_array(&view.hidden_fields),
        token_ui_json(&view.ui)
    )
}

fn token_supply_json(supply: token_bazaar::ResourceSupplyView) -> String {
    format!(
        "{{\"amber\":{},\"jade\":{},\"iron\":{}}}",
        supply.amber, supply.jade, supply.iron
    )
}

fn token_inventory_json(inventory: &token_bazaar::InventoryView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"resources\":{}}}",
        inventory.seat.as_str(),
        token_supply_json(inventory.resources)
    )
}

fn token_market_slot_json(slot: &token_bazaar::MarketSlotView) -> String {
    format!(
        "{{\"slot\":\"{}\",\"slot_id\":\"{}\",\"contract\":{},\"is_empty\":{},\"accessibility_label\":\"{}\"}}",
        slot.slot.as_str(),
        escape_json(&slot.slot_id),
        slot.contract
            .as_ref()
            .map_or_else(|| "null".to_owned(), token_contract_json),
        slot.is_empty,
        escape_json(&slot.accessibility_label)
    )
}

fn token_contract_json(contract: &token_bazaar::ContractView) -> String {
    format!(
        "{{\"contract_id\":\"{}\",\"label\":\"{}\",\"cost\":{},\"points\":{},\"accessibility_label\":\"{}\"}}",
        escape_json(&contract.contract_id),
        escape_json(&contract.label),
        token_supply_json(contract.cost),
        contract.points,
        escape_json(&contract.accessibility_label)
    )
}

fn token_legal_action_json(action: &token_bazaar::LegalActionView) -> String {
    let metadata = action
        .metadata
        .iter()
        .map(|(key, value)| {
            format!(
                "{{\"key\":\"{}\",\"value\":\"{}\"}}",
                escape_json(key),
                escape_json(value)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"action_segment\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\",\"metadata\":[{}],\"freshness_token\":{}}}",
        escape_json(&action.action_segment),
        escape_json(&action.label),
        escape_json(&action.accessibility_label),
        metadata,
        action.freshness_token.0
    )
}

fn token_terminal_json(terminal: &token_bazaar::TerminalView) -> String {
    match terminal {
        token_bazaar::TerminalView::NonTerminal => {
            "{\"terminal\":false,\"winner\":null,\"draw\":false}".to_owned()
        }
        token_bazaar::TerminalView::Win { winning_seat } => format!(
            "{{\"terminal\":true,\"winner\":\"{}\",\"draw\":false}}",
            winning_seat.as_str()
        ),
        token_bazaar::TerminalView::Draw => {
            "{\"terminal\":true,\"winner\":null,\"draw\":true}".to_owned()
        }
    }
}

fn token_effect_view_json(effect: &token_bazaar::EffectView) -> String {
    format!(
        "{{\"kind\":\"{}\",\"summary\":\"{}\"}}",
        escape_json(&effect.kind),
        escape_json(&effect.summary)
    )
}

fn token_ui_json(ui: &token_bazaar::UiMetadata) -> String {
    format!(
        "{{\"table_label\":\"{}\",\"supply_label\":\"{}\",\"inventory_label\":\"{}\",\"market_label\":\"{}\",\"score_label\":\"{}\",\"turn_counter_label\":\"{}\",\"reduced_motion_token\":\"{}\"}}",
        escape_json(&ui.table_label),
        escape_json(&ui.supply_label),
        escape_json(&ui.inventory_label),
        escape_json(&ui.market_label),
        escape_json(&ui.score_label),
        escape_json(&ui.turn_counter_label),
        escape_json(&ui.reduced_motion_token)
    )
}

fn token_effect_json(effect: &EffectEnvelope<TokenBazaarEffect>) -> String {
    let payload = match &effect.payload {
        TokenBazaarEffect::ResourceCollected {
            seat,
            bundle,
            gain,
            inventory_after,
            supply_after,
        } => format!(
            "{{\"type\":\"resource_collected\",\"seat\":\"{}\",\"bundle\":\"{}\",\"gain\":{},\"inventory_after\":{},\"supply_after\":{}}}",
            seat.as_str(),
            bundle.as_str(),
            token_counts_json(*gain),
            token_counts_json(*inventory_after),
            token_counts_json(*supply_after)
        ),
        TokenBazaarEffect::ResourceExchanged {
            seat,
            paid_resource,
            taken_resource,
            cost,
            gain,
            inventory_after,
            supply_after,
        } => format!(
            "{{\"type\":\"resource_exchanged\",\"seat\":\"{}\",\"paid_resource\":\"{}\",\"taken_resource\":\"{}\",\"cost\":{},\"gain\":{},\"inventory_after\":{},\"supply_after\":{}}}",
            seat.as_str(),
            paid_resource.as_str(),
            taken_resource.as_str(),
            token_counts_json(*cost),
            token_counts_json(*gain),
            token_counts_json(*inventory_after),
            token_counts_json(*supply_after)
        ),
        TokenBazaarEffect::ContractFulfilled {
            seat,
            slot,
            contract,
            cost,
            points,
            score_after,
            fulfilled_count_after,
        } => format!(
            "{{\"type\":\"contract_fulfilled\",\"seat\":\"{}\",\"slot\":\"{}\",\"contract\":\"{}\",\"cost\":{},\"points\":{},\"score_after\":{},\"fulfilled_count_after\":{}}}",
            seat.as_str(),
            slot.as_str(),
            contract.as_str(),
            token_counts_json(*cost),
            points,
            score_after,
            fulfilled_count_after
        ),
        TokenBazaarEffect::SlotRefilled {
            slot,
            contract,
            remaining_queue_len,
        } => format!(
            "{{\"type\":\"slot_refilled\",\"slot\":\"{}\",\"contract\":\"{}\",\"remaining_queue_len\":{}}}",
            slot.as_str(),
            contract.as_str(),
            remaining_queue_len
        ),
        TokenBazaarEffect::SlotEmptied {
            slot,
            remaining_queue_len,
        } => format!(
            "{{\"type\":\"slot_emptied\",\"slot\":\"{}\",\"remaining_queue_len\":{}}}",
            slot.as_str(),
            remaining_queue_len
        ),
        TokenBazaarEffect::PassAccepted { seat } => {
            format!("{{\"type\":\"pass_accepted\",\"seat\":\"{}\"}}", seat.as_str())
        }
        TokenBazaarEffect::TurnAdvanced {
            previous_seat,
            active_seat,
            turns_taken,
        } => format!(
            "{{\"type\":\"turn_advanced\",\"previous_seat\":\"{}\",\"active_seat\":\"{}\",\"turns_taken\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            previous_seat.as_str(),
            active_seat.as_str(),
            turns_taken[0],
            turns_taken[1]
        ),
        TokenBazaarEffect::Terminal {
            outcome,
            scores,
            fulfilled_counts,
            inventory_totals,
        } => format!(
            "{{\"type\":\"terminal\",\"outcome\":{},\"scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"fulfilled_counts\":{{\"seat_0\":{},\"seat_1\":{}}},\"inventory_totals\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            token_terminal_outcome_json(*outcome),
            scores[0],
            scores[1],
            fulfilled_counts[0],
            fulfilled_counts[1],
            inventory_totals[0],
            inventory_totals[1]
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn token_counts_json(counts: token_bazaar::ResourceCounts) -> String {
    format!(
        "{{\"amber\":{},\"jade\":{},\"iron\":{}}}",
        counts.amber, counts.jade, counts.iron
    )
}

fn token_terminal_outcome_json(outcome: token_bazaar::TerminalOutcome) -> String {
    match outcome {
        token_bazaar::TerminalOutcome::Win { seat } => {
            format!("{{\"kind\":\"win\",\"winner\":\"{}\"}}", seat.as_str())
        }
        token_bazaar::TerminalOutcome::Draw => "{\"kind\":\"draw\",\"winner\":null}".to_owned(),
    }
}

fn high_card_effect_json(effect: &EffectEnvelope<HighCardDuelEffect>) -> String {
    let payload = match &effect.payload {
        HighCardDuelEffect::DealPrivateCard { owner, card } => format!(
            "{{\"type\":\"deal_private_card\",\"owner\":\"{}\",\"card_id\":\"{}\"}}",
            owner.as_str(),
            card.stable_id()
        ),
        HighCardDuelEffect::HandCountChanged {
            seat_0_count,
            seat_1_count,
            deck_count,
        } => format!(
            "{{\"type\":\"hand_count_changed\",\"seat_0_count\":{},\"seat_1_count\":{},\"deck_count\":{}}}",
            seat_0_count, seat_1_count, deck_count
        ),
        HighCardDuelEffect::CommitFaceDown { seat, round_number } => format!(
            "{{\"type\":\"commit_face_down\",\"seat\":\"{}\",\"round_number\":{}}}",
            seat.as_str(),
            round_number
        ),
        HighCardDuelEffect::OwnCommitConfirmed {
            owner,
            card,
            round_number,
        } => format!(
            "{{\"type\":\"own_commit_confirmed\",\"owner\":\"{}\",\"card_id\":\"{}\",\"round_number\":{}}}",
            owner.as_str(),
            card.stable_id(),
            round_number
        ),
        HighCardDuelEffect::CardsRevealed {
            round_number,
            seat_0_card,
            seat_1_card,
        } => format!(
            "{{\"type\":\"cards_revealed\",\"round_number\":{},\"seat_0_card\":\"{}\",\"seat_1_card\":\"{}\"}}",
            round_number,
            seat_0_card.stable_id(),
            seat_1_card.stable_id()
        ),
        HighCardDuelEffect::RoundScored {
            round_number,
            winner,
            score,
        } => format!(
            "{{\"type\":\"round_scored\",\"round_number\":{},\"winner\":{},\"score\":{}}}",
            round_number,
            option_high_card_seat_json(*winner),
            high_card_score_json(*score)
        ),
        HighCardDuelEffect::RefillStarted {
            next_round_number,
            next_lead_seat,
        } => format!(
            "{{\"type\":\"refill_started\",\"next_round_number\":{},\"next_lead_seat\":\"{}\"}}",
            next_round_number,
            next_lead_seat.as_str()
        ),
        HighCardDuelEffect::Terminal { winner, score } => format!(
            "{{\"type\":\"terminal\",\"winner\":{},\"score\":{}}}",
            option_high_card_seat_json(*winner),
            high_card_score_json(*score)
        ),
        HighCardDuelEffect::PrivateDiagnostic {
            owner,
            code,
            private_message,
        } => format!(
            "{{\"type\":\"private_diagnostic\",\"owner\":\"{}\",\"code\":\"{}\",\"private_message\":\"{}\"}}",
            owner.as_str(),
            escape_json(code),
            escape_json(private_message)
        ),
        HighCardDuelEffect::PublicDiagnostic {
            code,
            public_message,
        } => format!(
            "{{\"type\":\"public_diagnostic\",\"code\":\"{}\",\"public_message\":\"{}\"}}",
            escape_json(code),
            escape_json(public_message)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn high_card_terminal_kind(terminal: &high_card_duel::TerminalView) -> &'static str {
    match terminal {
        high_card_duel::TerminalView::NonTerminal => "non_terminal",
        high_card_duel::TerminalView::Win { .. } => "win",
        high_card_duel::TerminalView::Draw => "draw",
    }
}

fn high_card_terminal_winner(terminal: &high_card_duel::TerminalView) -> Option<HighCardDuelSeat> {
    match terminal {
        high_card_duel::TerminalView::Win { winning_seat } => Some(*winning_seat),
        _ => None,
    }
}

fn draughts_terminal_kind(terminal: &draughts_lite::TerminalView) -> &'static str {
    match terminal {
        draughts_lite::TerminalView::NonTerminal => "non_terminal",
        draughts_lite::TerminalView::Win { .. } => "win",
    }
}

fn draughts_terminal_winner(terminal: &draughts_lite::TerminalView) -> Option<DraughtsLiteSeat> {
    match terminal {
        draughts_lite::TerminalView::Win { winning_seat } => Some(*winning_seat),
        _ => None,
    }
}

fn draughts_move_kind(kind: draughts_lite::MoveKind) -> &'static str {
    match kind {
        draughts_lite::MoveKind::Quiet => "quiet",
        draughts_lite::MoveKind::Capture => "capture",
    }
}

fn draughts_terminal_reason(reason: draughts_lite::TerminalWinReason) -> &'static str {
    match reason {
        draughts_lite::TerminalWinReason::OpponentNoPieces => "opponent_no_pieces",
        draughts_lite::TerminalWinReason::OpponentNoLegalMove => "opponent_no_legal_move",
    }
}

fn option_draughts_seat_json(seat: Option<DraughtsLiteSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

fn option_high_card_seat_json(seat: Option<HighCardDuelSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

fn option_token_seat_json(seat: Option<TokenBazaarSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
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

fn parse_public_replay_steps(input: &str) -> Result<Vec<PublicReplayStep>, String> {
    array_items(input, "steps")?
        .into_iter()
        .map(|item| parse_public_replay_step(&item))
        .collect()
}

fn parse_public_replay_step(input: &str) -> Result<PublicReplayStep, String> {
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
    Ok(PublicReplayStep {
        step_index,
        public_view_summary: string_field(input, "public_view_summary")?,
        public_effects: string_array_field(input, "public_effects")?,
        redacted_command_summary: string_field(input, "redacted_command_summary")?,
        terminal: bool_field(input, "terminal")?,
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
    let start = field_value_start(input, key)?;
    parse_string_at(input, start).ok_or_else(|| format!("field `{key}` must be a string"))
}

fn number_field(input: &str, key: &str) -> Result<u64, String> {
    let start = field_value_start(input, key)?;
    parse_number_at(input, start).ok_or_else(|| format!("field `{key}` must be a number"))
}

fn bool_field(input: &str, key: &str) -> Result<bool, String> {
    let start = field_value_start(input, key)?;
    let tail = input[start..].trim_start();
    if tail.starts_with("true") {
        Ok(true)
    } else if tail.starts_with("false") {
        Ok(false)
    } else {
        Err(format!("field `{key}` must be a boolean"))
    }
}

fn string_array_field(input: &str, key: &str) -> Result<Vec<String>, String> {
    array_items(input, key)?
        .into_iter()
        .map(|item| parse_json_string(item.trim()))
        .collect()
}

fn array_items(input: &str, key: &str) -> Result<Vec<String>, String> {
    let start = field_value_start(input, key)?;
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

fn field_value_start(input: &str, key: &str) -> Result<usize, String> {
    let trimmed = input.trim();
    let body = trimmed
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .ok_or_else(|| "malformed JSON object".to_owned())?;
    let body_start = input
        .find('{')
        .ok_or_else(|| "malformed JSON object".to_owned())?
        + 1;
    let mut index = 0;
    while index < body.len() {
        let rest = body[index..].trim_start();
        if rest.is_empty() {
            break;
        }
        index += body[index..].len() - rest.len();
        if body[index..].starts_with(',') {
            index += 1;
            continue;
        }
        let (field_key, next) = parse_json_string_at(body, index)?;
        index = next;
        let after_key = body[index..].trim_start();
        if !after_key.starts_with(':') {
            return Err(format!("malformed field `{field_key}`"));
        }
        let value_start = index + body[index..].len() - after_key.len() + 1;
        if field_key == key {
            return Ok(body_start + value_start);
        }
        index = skip_json_value(body, value_start)?;
    }
    Err(format!("missing `{key}`"))
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
/// `match_ptr..match_ptr + match_len` must be a valid UTF-8 buffer. If
/// `viewer_len` is nonzero, `viewer_ptr..viewer_ptr + viewer_len` must also be
/// a valid UTF-8 buffer for the duration of the call.
pub unsafe extern "C" fn rulepath_get_view_for_viewer(
    match_ptr: *const u8,
    match_len: usize,
    viewer_ptr: *const u8,
    viewer_len: usize,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let viewer = if viewer_len == 0 {
        Ok(None)
    } else {
        unsafe { read_string(viewer_ptr, viewer_len) }.map(Some)
    };
    write_result(
        match_id
            .and_then(|match_id| viewer.and_then(|viewer| get_view(&match_id, viewer.as_deref()))),
    )
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
/// `match_ptr..match_ptr + match_len` and `seat_ptr..seat_ptr + seat_len` must
/// be valid UTF-8 buffers. If `viewer_len` is nonzero,
/// `viewer_ptr..viewer_ptr + viewer_len` must also be a valid UTF-8 buffer for
/// the duration of the call.
pub unsafe extern "C" fn rulepath_get_action_tree_for_viewer(
    match_ptr: *const u8,
    match_len: usize,
    seat_ptr: *const u8,
    seat_len: usize,
    viewer_ptr: *const u8,
    viewer_len: usize,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let seat = unsafe { read_string(seat_ptr, seat_len) };
    let viewer = if viewer_len == 0 {
        Ok(None)
    } else {
        unsafe { read_string(viewer_ptr, viewer_len) }.map(Some)
    };
    write_result(match_id.and_then(|match_id| {
        seat.and_then(|seat| {
            viewer
                .and_then(|viewer| get_action_tree_for_viewer(&match_id, &seat, viewer.as_deref()))
        })
    }))
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
        assert!(games.contains("\"game_id\":\"draughts_lite\""));
        assert!(games.contains("\"game_id\":\"high_card_duel\""));
        assert!(games.contains("\"game_id\":\"token_bazaar\""));
        assert!(games.contains("\"variants\":[\"three_marks_standard\"]"));
        assert!(games.contains("\"variants\":[\"column_four_standard\"]"));
        assert!(games.contains("\"variants\":[\"directional_flip_standard\"]"));
        assert!(games.contains("\"variants\":[\"draughts_lite_standard\"]"));
        assert!(games.contains("\"variants\":[\"high_card_duel_standard\"]"));
        assert!(games.contains("\"variants\":[\"token_bazaar_standard\"]"));
        assert!(games.contains("\"hidden_information\":true"));
        assert!(games.contains("\"public_replay_export\""));
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
    fn get_view_honors_viewer_for_existing_perfect_information_games() {
        let created = new_match("column_four", 42).expect("match created");
        let match_id = extract_match_id(&created);

        let observer = get_view(&match_id, None).expect("observer view returned");
        let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat_0 view returned");
        let seat_1 = get_view(&match_id, Some("seat_1")).expect("seat_1 view returned");

        assert_eq!(observer, seat_0);
        assert_eq!(observer, seat_1);
        assert!(get_view(&match_id, Some("seat_2")).is_err());
    }

    #[test]
    fn get_action_tree_requires_viewer_authorization() {
        let created = new_match("three_marks", 32).expect("match created");
        let match_id = extract_match_id(&created);

        let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
            .expect("authorized action tree returned");
        let unauthorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_1"))
            .expect("unauthorized action tree redacted");
        let observer = get_action_tree_for_viewer(&match_id, "seat_0", None)
            .expect("observer action tree redacted");

        assert!(authorized.contains("\"segment\":\"place/r1c1\""));
        assert!(unauthorized.contains("\"choices\":[]"));
        assert!(observer.contains("\"choices\":[]"));
        assert!(get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_2")).is_err());
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
    fn draughts_lite_surface_preserves_multi_segment_paths() {
        let created = new_match("draughts_lite", 61).expect("match created");
        let match_id = extract_match_id(&created);
        assert!(created.contains("\"variant_id\":\"draughts_lite_standard\""));

        let view = get_view(&match_id, None).expect("view returned");
        assert!(view.contains("\"game_id\":\"draughts_lite\""));
        assert!(view.contains("\"variant_id\":\"draughts_lite_standard\""));
        assert!(view.contains("\"board_rows\":8"));
        assert!(view.contains("\"board_columns\":8"));
        assert!(view.contains("\"freshness_token\":0"));
        assert!(view.contains("\"private_view_status\":\"not_applicable_perfect_information\""));
        assert!(view.contains("\"hidden_fields\":[]"));

        let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
        assert!(tree.contains("\"segment\":\"from/"));
        assert!(tree.contains("\"next\":{\"choices\":["));
        assert!(tree.contains("\"segment\":\"to/"));
        assert!(tree.contains("\"freshness_token\":0"));

        let applied = apply_action(&match_id, "seat_0", "from/r3c2>to/r4c1", 0)
            .expect("multi-segment human action applies");
        assert!(applied.contains("\"type\":\"move_committed\""));
        assert!(applied.contains("\"action_path\":[\"from/r3c2\",\"to/r4c1\"]"));
        assert!(applied.contains("\"active_seat\":\"seat_1\""));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        assert!(bot.contains("\"type\":\"bot_chose_action\""));
        assert!(bot.contains("\"ply_count\":2"));

        let effects = get_effects(&match_id, 0, None).expect("effects returned");
        assert!(effects.contains("\"type\":\"move_committed\""));
        assert!(effects.contains("\"type\":\"bot_chose_action\""));

        let exported = export_replay(&match_id).expect("replay exported");
        assert!(exported.contains("\"game_id\":\"draughts_lite\""));
        assert!(exported.contains("\"rules_version\":\"draughts_lite-rules-v1\""));
        assert!(exported.contains("\"expected_replay_hashes\""));
        assert!(exported.contains("\"action_path\":[\"from/r3c2\",\"to/r4c1\"]"));
        assert!(
            exported.contains("\"private_view_hashes\":\"draughts_lite has no private-view API.\"")
        );
        assert!(!exported.contains("initial_snapshot"));

        let imported = import_replay(&exported).expect("replay imported");
        let replay_id = extract_replay_id(&imported);
        assert!(imported.contains("\"game_id\":\"draughts_lite\""));

        let reset = replay_reset(&replay_id).expect("replay reset returned");
        assert!(reset.contains("\"cursor\":0"));
        assert!(reset.contains("\"ply_count\":0"));

        let step = replay_step(&replay_id, 1).expect("replay stepped");
        assert!(step.contains("\"cursor\":1"));
        assert!(step.contains("\"ply_count\":1"));
    }

    #[test]
    fn high_card_duel_surface_filters_hidden_information() {
        let created = new_match("high_card_duel", 71).expect("match created");
        let match_id = extract_match_id(&created);
        assert!(created.contains("\"variant_id\":\"high_card_duel_standard\""));

        let observer = get_view(&match_id, None).expect("observer view returned");
        assert!(observer.contains("\"game_id\":\"high_card_duel\""));
        assert!(observer.contains("\"private_view\":{\"status\":\"observer\""));
        assert!(!observer.contains("hcd:r"));

        let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat view returned");
        assert!(seat_0.contains("\"private_view\":{\"status\":\"seat\",\"seat\":\"seat_0\""));
        assert!(seat_0.contains("hcd:r"));

        let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
            .expect("authorized tree returned");
        let unauthorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_1"))
            .expect("unauthorized tree returned");
        let observer_tree =
            get_action_tree_for_viewer(&match_id, "seat_0", None).expect("observer tree returned");

        assert!(authorized.contains("\"segment\":\"commit/hcd:r"));
        assert!(unauthorized.contains("\"choices\":[]"));
        assert!(observer_tree.contains("\"choices\":[]"));

        let action_segment = authorized
            .split("\"segment\":\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .expect("commit segment present")
            .to_owned();
        let applied =
            apply_action(&match_id, "seat_0", &action_segment, 0).expect("commit applies");
        assert!(applied.contains("\"type\":\"own_commit_confirmed\""));
        assert!(applied.contains("hcd:r"));
        assert!(applied.contains("\"private_to_seat\":\"seat-0\""));

        let observer_effects = get_effects(&match_id, 0, None).expect("observer effects");
        assert!(observer_effects.contains("\"type\":\"commit_face_down\""));
        assert!(!observer_effects.contains("hcd:r"));

        let seat_0_effects = get_effects(&match_id, 0, Some("seat_0")).expect("seat effects");
        assert!(seat_0_effects.contains("\"type\":\"own_commit_confirmed\""));
        assert!(seat_0_effects.contains("hcd:r"));

        let seat_1_effects = get_effects(&match_id, 0, Some("seat_1")).expect("other effects");
        assert!(seat_1_effects.contains("\"type\":\"commit_face_down\""));
        assert!(!seat_1_effects.contains("hcd:r"));

        let exported = export_replay(&match_id).expect("public replay exported");
        assert!(exported.contains("\"export_class\":\"public_observer_projection_v1\""));
        assert!(exported.contains("\"viewer\":\"observer\""));
        assert!(!exported.contains("\"seed\""));
        assert!(!exported.contains("hcd:r"));

        let imported = import_replay(&exported).expect("public replay imported");
        let replay_id = extract_replay_id(&imported);
        assert!(imported.contains("\"public_export\":true"));
        assert!(imported.contains("\"game_id\":\"high_card_duel\""));
        let reset = replay_reset(&replay_id).expect("public replay reset returned");
        assert!(reset.contains("\"public_export\":true"));
        assert!(reset.contains("\"view\":null"));

        let pretty_exported = pretty_json_layout(&exported);
        assert!(pretty_exported.contains("\"export_class\": \"public_observer_projection_v1\""));
        assert!(pretty_exported.contains("\"game_id\": \"high_card_duel\""));
        let pretty_imported =
            import_replay(&pretty_exported).expect("pretty public replay imported");
        let pretty_replay_id = extract_replay_id(&pretty_imported);
        assert!(pretty_imported.contains("\"public_export\":true"));
        assert!(pretty_imported.contains("\"game_id\":\"high_card_duel\""));
        assert!(!pretty_imported.contains("hcd:r"));
        let pretty_reset = replay_reset(&pretty_replay_id).expect("pretty public replay reset");
        assert!(pretty_reset.contains("\"public_export\":true"));
        assert!(!pretty_reset.contains("hcd:r"));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        assert!(bot.contains("\"ok\":true"));
        assert!(bot.contains("\"type\":\"cards_revealed\""));
    }

    #[test]
    fn token_bazaar_surface_drives_public_accounting_group() {
        let created = new_match("token_bazaar", 81).expect("match created");
        let match_id = extract_match_id(&created);
        assert!(created.contains("\"variant_id\":\"token_bazaar_standard\""));

        let observer = get_view(&match_id, None).expect("observer view returned");
        let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat view returned");
        assert_eq!(observer, seat_0);
        assert!(observer.contains("\"game_id\":\"token_bazaar\""));
        assert!(observer.contains("\"variant_id\":\"token_bazaar_standard\""));
        assert!(observer.contains("\"supply\":{\"amber\":14,\"jade\":14,\"iron\":14}"));
        assert!(observer.contains("\"hidden_fields\":[]"));
        assert!(!observer.contains("candidate"));
        assert!(!observer.contains("debug"));

        let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
        assert!(tree.contains("\"segment\":\"collect/amber\""));
        assert!(tree.contains("\"segment\":\"fulfill/slot_0\""));
        assert!(tree.contains("\"freshness_token\":0"));

        let applied =
            apply_action(&match_id, "seat_0", "collect/amber", 0).expect("collect applies");
        assert!(applied.contains("\"type\":\"resource_collected\""));
        assert!(applied.contains("\"active_seat\":\"seat_1\""));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        assert!(bot.contains("\"ok\":true"));
        assert!(bot.contains("\"active_seat\":\"seat_0\""));

        let effects = get_effects(&match_id, 0, None).expect("effects returned");
        assert!(effects.contains("\"type\":\"resource_collected\""));
        assert!(effects.contains("\"visibility\":\"public\""));
        assert!(!effects.contains("candidate"));
        assert!(!effects.contains("debug"));

        let exported = export_replay(&match_id).expect("replay exported");
        assert!(exported.contains("\"game_id\":\"token_bazaar\""));
        assert!(exported.contains("\"rules_version\":\"token-bazaar-rules-v1\""));
        assert!(exported.contains("\"expected_public_export_hashes\""));
        assert!(!exported.contains("\"state\":"));
        assert!(!exported.contains("candidate"));
        assert!(!exported.contains("debug"));

        let imported = import_replay(&exported).expect("replay imported");
        let replay_id = extract_replay_id(&imported);
        assert!(imported.contains("\"game_id\":\"token_bazaar\""));

        let reset = replay_reset(&replay_id).expect("replay reset returned");
        assert!(reset.contains("\"cursor\":0"));
        assert!(reset.contains("\"game_id\":\"token_bazaar\""));

        let step = replay_step(&replay_id, 1).expect("replay stepped");
        assert!(step.contains("\"cursor\":1"));
        assert!(step.contains("\"type\":\"resource_collected\""));
    }

    #[test]
    fn high_card_public_import_replays_ordered_public_effects() {
        let source_export = high_card_export_public_observer_replay(
            &high_card_duel::generate_internal_full_trace(55),
        );
        let exported = pretty_json_layout(&source_export.to_json());
        let imported = import_replay(&exported).expect("public replay imported");
        let replay_id = extract_replay_id(&imported);

        for source_step in &source_export.steps {
            let step = replay_step(&replay_id, source_step.step_index).expect("step returned");
            assert!(step.contains(&format!("\"cursor\":{}", source_step.step_index)));
            assert!(step.contains(&format!(
                "\"public_effects\":{}",
                json_string_array(&source_step.public_effects)
            )));
            assert!(step.contains(&format!(
                "\"redacted_command_summary\":\"{}\"",
                escape_json(&source_step.redacted_command_summary)
            )));
        }

        let initial = replay_step(&replay_id, 0).expect("initial step returned");
        assert!(initial.contains("\"public_effects\":[]"));

        let reveal = replay_step(&replay_id, 2).expect("reveal step returned");
        assert!(reveal.contains("hcd_cards_revealed:round=1;"));
        assert!(reveal.contains("hcd_round_scored:round=1;"));
        assert_ordered(
            &reveal,
            "hcd_cards_revealed:round=1;",
            "hcd_round_scored:round=1;",
        );

        let terminal_index = source_export.steps.len() - 1;
        let terminal = replay_step(&replay_id, terminal_index).expect("terminal step returned");
        assert!(terminal.contains("hcd_terminal:winner="));
        assert!(terminal.contains(&format!("\"cursor\":{terminal_index}")));

        let clamped = replay_step(&replay_id, terminal_index + 99).expect("clamped step returned");
        assert!(clamped.contains(&format!("\"cursor\":{terminal_index}")));
        assert!(clamped.contains("hcd_terminal:winner="));
    }

    #[test]
    fn high_card_public_import_step_json_adds_no_hidden_facts() {
        let source_export = high_card_export_public_observer_replay(
            &high_card_duel::generate_internal_full_trace(55),
        );
        let exported = source_export.to_json();
        let imported = import_replay(&exported).expect("public replay imported");
        let replay_id = extract_replay_id(&imported);
        let public_card_ids = card_ids_in(&exported);

        for cursor in 0..source_export.steps.len() {
            let step = replay_step(&replay_id, cursor).expect("step returned");
            assert!(
                !step.contains("\"seed\""),
                "seed leaked at cursor {cursor}: {step}"
            );
            assert!(
                !step.contains("commit/hcd:r"),
                "private command path leaked at cursor {cursor}: {step}"
            );
            for card_id in card_ids_in(&step) {
                assert!(
                    public_card_ids.contains(&card_id),
                    "step introduced card id {card_id} absent from source public export"
                );
            }
        }
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

        let unexpected_export_class = exported.replacen(
            '{',
            "{\"export_class\":\"public_observer_projection_v1\",",
            1,
        );
        assert!(import_replay(&unexpected_export_class)
            .expect_err("unknown export_class rejected on generic path")
            .contains("unknown field `export_class`"));

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

    #[test]
    fn draughts_lite_wasm_export_matches_golden_fixture() {
        let commands = vec![AppliedCommand {
            actor_seat: "seat-0".to_owned(),
            action_path: vec!["from/r3c2".to_owned(), "to/r4c1".to_owned()],
            freshness_token: 0,
        }];
        let exported =
            draughts_replay_document_json("wasm-exported", 1, &commands).expect("fixture exported");
        let fixture = include_str!(
            "../../../games/draughts_lite/tests/golden_traces/wasm-exported.trace.json"
        );

        assert_eq!(compact_json_layout(fixture), exported);
    }

    #[test]
    fn token_bazaar_wasm_export_matches_golden_fixture() {
        let commands = vec![
            AppliedCommand {
                actor_seat: "seat_0".to_owned(),
                action_path: vec!["collect/amber".to_owned()],
                freshness_token: 0,
            },
            AppliedCommand {
                actor_seat: "seat_1".to_owned(),
                action_path: vec!["collect/jade".to_owned()],
                freshness_token: 1,
            },
            AppliedCommand {
                actor_seat: "seat_0".to_owned(),
                action_path: vec!["fulfill/slot_0".to_owned()],
                freshness_token: 2,
            },
        ];
        let exported =
            token_replay_document_json("wasm-exported", 1, &commands).expect("fixture exported");
        let fixture = include_str!(
            "../../../games/token_bazaar/tests/golden_traces/wasm-exported.trace.json"
        );

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

    fn json_string_array(values: &[String]) -> String {
        let body = values
            .iter()
            .map(|value| format!("\"{}\"", escape_json(value)))
            .collect::<Vec<_>>()
            .join(",");
        format!("[{body}]")
    }

    fn assert_ordered(input: &str, first: &str, second: &str) {
        let first_index = input.find(first).expect("first value present");
        let second_index = input.find(second).expect("second value present");
        assert!(
            first_index < second_index,
            "`{first}` must appear before `{second}` in {input}"
        );
    }

    fn card_ids_in(input: &str) -> Vec<String> {
        let mut ids = Vec::new();
        for part in input.split("hcd:r").skip(1) {
            let suffix = part
                .chars()
                .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == ':')
                .collect::<String>();
            ids.push(format!("hcd:r{suffix}"));
        }
        ids.sort();
        ids.dedup();
        ids
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

    fn pretty_json_layout(input: &str) -> String {
        let mut output = String::new();
        let mut in_string = false;
        let mut escaped = false;
        let mut depth = 0_usize;
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
                continue;
            }
            match ch {
                '"' => {
                    in_string = true;
                    output.push(ch);
                }
                '{' | '[' => {
                    output.push(ch);
                    depth += 1;
                    output.push('\n');
                    output.push_str(&"  ".repeat(depth));
                }
                '}' | ']' => {
                    depth = depth.saturating_sub(1);
                    output.push('\n');
                    output.push_str(&"  ".repeat(depth));
                    output.push(ch);
                }
                ':' => output.push_str(": "),
                ',' => {
                    output.push(ch);
                    output.push('\n');
                    output.push_str(&"  ".repeat(depth));
                }
                _ => output.push(ch),
            }
        }
        output
    }
}
