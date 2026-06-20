//! Browser-facing Rulepath API surface.

use column_four::{
    apply_action as column_apply_action, legal_action_tree as column_legal_action_tree,
    project_view as column_project_view, setup_match as column_setup_match, ColumnFourEffect,
    ColumnFourLevel2Bot, ColumnFourState,
};
#[cfg(test)]
use commands::command_record_json;
use directional_flip::{
    apply_action as directional_apply_action, legal_action_tree as directional_legal_action_tree,
    project_view as directional_project_view, setup_match as directional_setup_match,
    DirectionalFlipEffect, DirectionalFlipLevel2Bot, DirectionalFlipState,
};
use draughts_lite::{
    apply_action as draughts_apply_action, legal_action_tree as draughts_legal_action_tree,
    project_view as draughts_project_view, setup_match as draughts_setup_match, DraughtsLiteEffect,
    DraughtsLiteLevel1Bot, DraughtsLiteState,
};
#[cfg(test)]
use engine_core::{ActionPath, Actor, HashValue, SeatId, StableSerialize};
use engine_core::{
    CommandEnvelope, EffectCursor, EffectLog, RulesVersion, Seed, Viewer, VisibilityScope,
};
use event_frontier::visibility::public_effect_text as event_frontier_public_effect_text;
use event_frontier::{
    apply_command as event_frontier_apply_command,
    command_for_decision as event_frontier_command_for_decision,
    legal_action_tree as event_frontier_legal_action_tree,
    project_view as event_frontier_project_view, setup_match as event_frontier_setup_match,
    validate_command as event_frontier_validate_command, EventCharterLevel1Bot,
    EventFreeholdersLevel1Bot, EventFrontierEffect, EventFrontierState,
    FactionId as EventFrontierFactionId,
};
use flood_watch::{
    apply_validated_action as flood_apply_action, legal_action_tree as flood_legal_action_tree,
    project_view as flood_project_view, setup_match as flood_setup_match, FloodWatchEffect,
    FloodWatchLevel1Bot, FloodWatchState,
};
use frontier_control::{
    apply_command as frontier_apply_command, command_for_decision as frontier_command_for_decision,
    legal_action_tree as frontier_legal_action_tree, project_view as frontier_project_view,
    public_effect_text as frontier_public_effect_text, setup_match as frontier_setup_match,
    validate_command as frontier_validate_command, FactionId as FrontierFactionId,
    FrontierControlEffect, FrontierControlState, FrontierGarrisonLevel1Bot,
    FrontierProspectorLevel1Bot,
};
use high_card_duel::{
    apply_action as high_card_apply_action,
    export_public_observer_replay as high_card_export_public_observer_replay,
    legal_action_tree as high_card_legal_action_tree, project_view as high_card_project_view,
    setup_match as high_card_setup_match, validate_command as high_card_validate_command,
    HighCardDuelEffect, HighCardDuelInternalTrace, HighCardDuelRandomBot, HighCardDuelState,
};
use masked_claims::{
    apply_action as masked_apply_action, legal_action_tree as masked_legal_action_tree,
    project_view as masked_project_view, setup_match as masked_setup_match, MaskedClaimsEffect,
    MaskedClaimsLevel1Bot, MaskedClaimsState,
};
#[cfg(test)]
use plain_tricks::PlainTricksSeat;
use plain_tricks::{
    apply_action as plain_apply_action, legal_action_tree as plain_legal_action_tree,
    project_view as plain_project_view,
    replay_support::{
        export_public_replay as plain_export_public_replay, PlainTricksInternalTrace,
        ReplayCommand as PlainReplayCommand,
    },
    setup_match as plain_setup_match, PlainTricksEffect, PlainTricksLevel2Bot, PlainTricksState,
};
#[cfg(test)]
use poker_lite::PokerLiteSeat;
use poker_lite::{
    apply_action as poker_apply_action, legal_action_tree as poker_legal_action_tree,
    project_view as poker_project_view,
    replay_support::{
        export_public_replay as poker_export_public_replay, PokerLiteInternalTrace,
        ReplayCommand as PokerReplayCommand,
    },
    setup_match as poker_setup_match, PokerLiteEffect, PokerLiteLevel2Bot, PokerLiteState,
};
#[cfg(test)]
use race_to_n::replay_support::replay_commands as race_replay_commands;
use race_to_n::{
    apply_action as race_apply_action, legal_action_tree, project_view,
    setup_match as race_setup_match, RaceEffect, RaceRandomBot, RaceState,
    SetupOptions as RaceSetupOptions,
};
#[cfg(test)]
use river_ledger::RiverLedgerSeat;
use river_ledger::{
    apply_action as river_apply_action, legal_action_tree as river_legal_action_tree,
    project_view as river_project_view,
    replay_support::{
        export_public_replay as river_export_public_replay, ReplayCommand as RiverReplayCommand,
        RiverLedgerInternalTrace,
    },
    setup_match as river_setup_match, RiverLedgerEffect, RiverLedgerLevel2Bot, RiverLedgerState,
};
use secret_draft::{
    apply_action as secret_apply_action, legal_action_tree as secret_legal_action_tree,
    project_view as secret_project_view,
    replay_support::{
        export_public_replay as secret_export_public_replay, ReplayCommand as SecretReplayCommand,
        SecretDraftInternalTrace,
    },
    setup_match as secret_setup_match, SecretDraftEffect, SecretDraftLevel1Bot, SecretDraftState,
};
use three_marks::{
    apply_action as three_apply_action, legal_action_tree as three_legal_action_tree,
    project_view as three_project_view, setup_match as three_setup_match, ThreeMarksEffect,
    ThreeMarksLevel1Bot, ThreeMarksState,
};
use token_bazaar::{
    apply_action as token_apply_action, legal_action_tree as token_legal_action_tree,
    project_view as token_project_view, setup_match as token_setup_match, TokenBazaarEffect,
    TokenBazaarLevel1Bot, TokenBazaarState,
};

mod action_path;
mod action_tree;
mod actors;
mod catalog;
mod commands;
mod constants;
mod games;
mod json;
mod json_parse;
mod replay;
mod seats;
mod store;
mod wasm_abi;

use action_path::*;
use action_tree::*;
use actors::*;
use constants::*;
use games::column::*;
use games::directional::*;
use games::draughts::*;
use games::event::*;
use games::flood::*;
use games::frontier::*;
use games::high_card::*;
use games::masked::*;
use games::plain::*;
use games::poker::*;
use games::race::*;
use games::river::*;
use games::secret::*;
use games::three::*;
use games::token::*;
use json::*;
use replay::*;
use seats::*;
use store::*;

pub use catalog::{feature_report, list_games};

#[derive(Clone, Debug)]
pub(crate) enum MatchRecord {
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
    MaskedClaims {
        game_id: String,
        state: MaskedClaimsState,
        effects: EffectLog<MaskedClaimsEffect>,
        commands: Vec<AppliedCommand>,
    },
    FloodWatch {
        game_id: String,
        state: FloodWatchState,
        effects: EffectLog<FloodWatchEffect>,
        commands: Vec<AppliedCommand>,
    },
    FrontierControl {
        game_id: String,
        state: FrontierControlState,
        effects: EffectLog<FrontierControlEffect>,
        commands: Vec<AppliedCommand>,
    },
    EventFrontier {
        game_id: String,
        state: EventFrontierState,
        effects: EffectLog<EventFrontierEffect>,
        commands: Vec<AppliedCommand>,
    },
    TokenBazaar {
        game_id: String,
        seed: u64,
        state: TokenBazaarState,
        effects: EffectLog<TokenBazaarEffect>,
        commands: Vec<AppliedCommand>,
    },
    SecretDraft {
        game_id: String,
        seed: u64,
        state: SecretDraftState,
        effects: EffectLog<SecretDraftEffect>,
        commands: Vec<AppliedCommand>,
    },
    PokerLite {
        game_id: String,
        seed: u64,
        state: PokerLiteState,
        effects: EffectLog<PokerLiteEffect>,
        commands: Vec<AppliedCommand>,
    },
    PlainTricks {
        game_id: String,
        seed: u64,
        state: PlainTricksState,
        effects: EffectLog<PlainTricksEffect>,
        commands: Vec<AppliedCommand>,
    },
    RiverLedger {
        game_id: String,
        seed: u64,
        state: RiverLedgerState,
        effects: EffectLog<RiverLedgerEffect>,
        commands: Vec<AppliedCommand>,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct ReplayRecord {
    game_id: String,
    seed: u64,
    commands: Vec<AppliedCommand>,
    public_timeline: Option<PublicTimelineReplay>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AppliedCommand {
    pub(crate) actor_seat: String,
    pub(crate) action_path: Vec<String>,
    pub(crate) freshness_token: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RegisteredGame {
    RaceToN,
    ThreeMarks,
    ColumnFour,
    DirectionalFlip,
    DraughtsLite,
    HighCardDuel,
    MaskedClaims,
    FloodWatch,
    FrontierControl,
    EventFrontier,
    TokenBazaar,
    SecretDraft,
    PokerLite,
    PlainTricks,
    RiverLedger,
}

pub fn placeholder_version() -> &'static str {
    API_VERSION
}

pub fn new_match(game_id: &str, seed: u64) -> Result<String, String> {
    new_match_with_seat_count(game_id, seed, DEFAULT_SEAT_COUNT)
}

pub fn new_match_with_seat_count(
    game_id: &str,
    seed: u64,
    seat_count: usize,
) -> Result<String, String> {
    new_match_for_variant_with_seat_count(game_id, None, seed, seat_count)
}

pub fn new_match_for_variant(
    game_id: &str,
    variant_id: Option<&str>,
    seed: u64,
) -> Result<String, String> {
    new_match_for_variant_with_seat_count(game_id, variant_id, seed, DEFAULT_SEAT_COUNT)
}

pub fn new_match_for_variant_with_seat_count(
    game_id: &str,
    variant_id: Option<&str>,
    seed: u64,
    seat_count: usize,
) -> Result<String, String> {
    match resolve_game(game_id)? {
        RegisteredGame::RaceToN => {
            let seats = seats_for_count(seat_count);
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
            let seats = seats_for_count(seat_count);
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
            let seats = seats_for_count(seat_count);
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
            let seats = seats_for_count(seat_count);
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
            let seats = seats_for_count(seat_count);
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
            let seats = seats_for_count(seat_count);
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
        RegisteredGame::MaskedClaims => {
            let seats = masked_seats_for_count(seat_count);
            let state =
                masked_setup_match(Seed(seed), &seats, &masked_claims::SetupOptions::default())
                    .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::MaskedClaims {
                        game_id: GAME_MASKED_CLAIMS.to_owned(),
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
                escape_json(VARIANT_MASKED_CLAIMS_STANDARD)
            ))
        }
        RegisteredGame::FloodWatch => {
            let seats = flood_seats_for_count(seat_count);
            let selected_variant = variant_id.unwrap_or(VARIANT_FLOOD_WATCH_STANDARD);
            let variant = flood_watch::ScenarioVariant::resolve(selected_variant)
                .map_err(|message| unsupported_variant_json(game_id, &message))?;
            let state =
                flood_setup_match(Seed(seed), &seats, &flood_watch::SetupOptions { variant })
                    .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::FloodWatch {
                        game_id: GAME_FLOOD_WATCH.to_owned(),
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
                escape_json(selected_variant)
            ))
        }
        RegisteredGame::FrontierControl => {
            let seats = frontier_seats_for_count(seat_count);
            let selected_variant = variant_id.unwrap_or(VARIANT_FRONTIER_CONTROL_STANDARD);
            let variant = frontier_control::VariantMap::resolve(selected_variant)
                .map_err(|message| unsupported_variant_json(game_id, &message))?;
            let state = frontier_setup_match(&seats, &frontier_control::SetupOptions { variant })
                .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::FrontierControl {
                        game_id: GAME_FRONTIER_CONTROL.to_owned(),
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
                escape_json(selected_variant)
            ))
        }
        RegisteredGame::EventFrontier => {
            let seats = event_frontier_seats_for_count(seat_count);
            let selected_variant = variant_id.unwrap_or(VARIANT_EVENT_FRONTIER_STANDARD);
            let variant = event_frontier::ScenarioVariant::resolve(selected_variant)
                .map_err(|message| unsupported_variant_json(game_id, &message))?;
            let state = event_frontier_setup_match(
                Seed(seed),
                &seats,
                &event_frontier::SetupOptions { variant },
            )
            .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::EventFrontier {
                        game_id: GAME_EVENT_FRONTIER.to_owned(),
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
                escape_json(selected_variant)
            ))
        }
        RegisteredGame::TokenBazaar => {
            let seats = seats_for_count(seat_count);
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
        RegisteredGame::SecretDraft => {
            let seats = seats_for_count(seat_count);
            let state = secret_setup_match(&seats, &secret_draft::SetupOptions::default())
                .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::SecretDraft {
                        game_id: GAME_SECRET_DRAFT.to_owned(),
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
                escape_json(VARIANT_SECRET_DRAFT_STANDARD)
            ))
        }
        RegisteredGame::PokerLite => {
            let seats = seats_for_count(seat_count);
            let state = poker_setup_match(Seed(seed), &seats, &poker_lite::SetupOptions::default())
                .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::PokerLite {
                        game_id: GAME_POKER_LITE.to_owned(),
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
                escape_json(VARIANT_POKER_LITE_STANDARD)
            ))
        }
        RegisteredGame::PlainTricks => {
            let seats = plain_seats_for_count(seat_count);
            let state =
                plain_setup_match(Seed(seed), &seats, &plain_tricks::SetupOptions::default())
                    .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::PlainTricks {
                        game_id: GAME_PLAIN_TRICKS.to_owned(),
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
                escape_json(VARIANT_PLAIN_TRICKS_STANDARD)
            ))
        }
        RegisteredGame::RiverLedger => {
            let seats = river_seats_for_count(seat_count);
            let state =
                river_setup_match(Seed(seed), &seats, &river_ledger::SetupOptions::default())
                    .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord::RiverLedger {
                        game_id: GAME_RIVER_LEDGER.to_owned(),
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
                escape_json(VARIANT_RIVER_LEDGER_STANDARD)
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
        MatchRecord::MaskedClaims { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = masked_viewer_for_seat(state, viewer_seat)?;
            Ok(masked_view_json(&masked_project_view(state, &viewer)))
        }
        MatchRecord::FloodWatch { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = flood_viewer_for_seat(state, viewer_seat)?;
            Ok(flood_view_json(&flood_project_view(state, &viewer)))
        }
        MatchRecord::FrontierControl { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = frontier_viewer_for_seat(state, viewer_seat)?;
            Ok(frontier_view_json(&frontier_project_view(state, &viewer)))
        }
        MatchRecord::EventFrontier { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = event_frontier_viewer_for_seat(state, viewer_seat)?;
            Ok(event_frontier_view_json(&event_frontier_project_view(
                state, &viewer,
            )))
        }
        MatchRecord::TokenBazaar { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = token_viewer_for_seat(state, viewer_seat)?;
            Ok(token_view_json(&token_project_view(state, &viewer)))
        }
        MatchRecord::SecretDraft { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = secret_viewer_for_seat(state, viewer_seat)?;
            Ok(secret_view_json(&secret_project_view(state, &viewer)))
        }
        MatchRecord::PokerLite { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = poker_viewer_for_seat(state, viewer_seat)?;
            Ok(poker_view_json(&poker_project_view(state, &viewer)))
        }
        MatchRecord::PlainTricks { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = plain_viewer_for_seat(state, viewer_seat)?;
            Ok(plain_view_json(&plain_project_view(state, &viewer)))
        }
        MatchRecord::RiverLedger { game_id, state, .. } => {
            resolve_game(game_id)?;
            let viewer = river_viewer_for_seat(state, viewer_seat)?;
            Ok(river_view_json(&river_project_view(state, &viewer)))
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
        MatchRecord::MaskedClaims { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_masked_seat(actor_seat)?;
            if !masked_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = masked_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&masked_legal_action_tree(state, &actor)))
        }
        MatchRecord::FloodWatch { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_flood_seat(actor_seat)?;
            if !flood_viewer_authorizes_actor(viewer_seat, &seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = flood_actor_for_seat(state, &seat)?;
            Ok(action_tree_json(&flood_legal_action_tree(state, &actor)))
        }
        MatchRecord::FrontierControl { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_frontier_seat(actor_seat)?;
            if !frontier_viewer_authorizes_actor(viewer_seat, &seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = frontier_actor_for_seat(state, &seat)?;
            Ok(action_tree_json(&frontier_legal_action_tree(state, &actor)))
        }
        MatchRecord::EventFrontier { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_event_frontier_seat(actor_seat)?;
            if !event_frontier_viewer_authorizes_actor(viewer_seat, &seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = event_frontier_actor_for_seat(state, &seat)?;
            Ok(action_tree_json(&event_frontier_legal_action_tree(
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
        MatchRecord::SecretDraft { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_secret_seat(actor_seat)?;
            if !secret_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = secret_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&secret_legal_action_tree(state, &actor)))
        }
        MatchRecord::PokerLite { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_poker_seat(actor_seat)?;
            if !poker_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = poker_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&poker_legal_action_tree(state, &actor)))
        }
        MatchRecord::PlainTricks { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_plain_seat(actor_seat)?;
            if !plain_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = plain_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&plain_legal_action_tree(state, &actor)))
        }
        MatchRecord::RiverLedger { game_id, state, .. } => {
            resolve_game(game_id)?;
            let seat = parse_river_seat(actor_seat)?;
            if !river_viewer_authorizes_actor(viewer_seat, seat)? {
                return Ok(empty_action_tree_json(state.freshness_token));
            }
            let actor = river_actor_for_seat(state, seat)?;
            Ok(action_tree_json(&river_legal_action_tree(state, &actor)))
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
        MatchRecord::MaskedClaims {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_masked_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: masked_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                masked_claims::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = masked_apply_action(state, action).map_err(diagnostic_json)?;
            let viewer = masked_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = masked_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_masked_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                masked_view_json(&masked_project_view(state, &viewer))
            ))
        }
        MatchRecord::FloodWatch {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_flood_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: flood_actor_for_seat(state, &seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = flood_watch::validate_command(state, &command).map_err(diagnostic_json)?;
            let applied = flood_apply_action(state, action).map_err(diagnostic_json)?;
            let effects = applied.effects;
            let viewer = flood_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = flood_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: seat.0,
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                flood_view_json(&flood_project_view(state, &viewer))
            ))
        }
        MatchRecord::FrontierControl {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_frontier_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: frontier_actor_for_seat(state, &seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            frontier_validate_command(state, &command).map_err(diagnostic_json)?;
            let applied = frontier_apply_command(state, &command).map_err(diagnostic_json)?;
            let effects = applied.effects;
            let viewer = frontier_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = frontier_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: seat.0,
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                frontier_view_json(&frontier_project_view(state, &viewer))
            ))
        }
        MatchRecord::EventFrontier {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_event_frontier_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: event_frontier_actor_for_seat(state, &seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            event_frontier_validate_command(state, &command).map_err(diagnostic_json)?;
            let applied = event_frontier_apply_command(state, &command).map_err(diagnostic_json)?;
            let mut effects = applied.effects;
            effects.extend(event_frontier_finish_automated_phases(state)?);
            let viewer = event_frontier_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = event_frontier_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: seat.0,
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                event_frontier_view_json(&event_frontier_project_view(state, &viewer))
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
        MatchRecord::SecretDraft {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_secret_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: secret_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = secret_draft::actions::validate_command(state, &command)
                .map_err(diagnostic_json)?;
            let effects = secret_apply_action(state, action).map_err(diagnostic_json)?;
            let viewer = secret_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = secret_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_secret_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                secret_view_json(&secret_project_view(state, &viewer))
            ))
        }
        MatchRecord::PokerLite {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_poker_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: poker_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = poker_lite::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = poker_apply_action(state, action).map_err(diagnostic_json)?;
            let viewer = poker_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = poker_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_poker_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                poker_view_json(&poker_project_view(state, &viewer))
            ))
        }
        MatchRecord::PlainTricks {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_plain_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: plain_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                plain_tricks::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = plain_apply_action(state, action).map_err(diagnostic_json)?;
            let viewer = plain_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = plain_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_plain_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                plain_view_json(&plain_project_view(state, &viewer))
            ))
        }
        MatchRecord::RiverLedger {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_river_seat(actor_seat)?;
            let command = CommandEnvelope {
                actor: river_actor_for_seat(state, seat)?,
                action_path: parse_action_path(action_path),
                freshness_token: engine_core::FreshnessToken(freshness_token),
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                river_ledger::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = river_apply_action(state, action).map_err(diagnostic_json)?;
            let viewer = river_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = river_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_river_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token,
            });
            Ok(format!(
                "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
                effect_json,
                river_view_json(&river_project_view(state, &viewer))
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
        MatchRecord::MaskedClaims {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_masked_seat(actor_seat)?;
            let decision = MaskedClaimsLevel1Bot::new(Seed(bot_seed))
                .select_decision(state, seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: masked_actor_for_seat(state, seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                masked_claims::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = masked_apply_action(state, action).map_err(diagnostic_json)?;
            let viewer = masked_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = masked_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_masked_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"policy_id\":\"{}\",\"policy_version\":{},\"rationale\":\"{}\",\"effects\":{},\"view\":{}}}",
                escape_json(&decision.policy_id),
                decision.policy_version,
                escape_json(&decision.rationale),
                effect_json,
                masked_view_json(&masked_project_view(state, &viewer))
            ))
        }
        MatchRecord::FloodWatch {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_flood_seat(actor_seat)?;
            let decision = FloodWatchLevel1Bot::new(Seed(bot_seed))
                .select_decision(state, &seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: flood_actor_for_seat(state, &seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = flood_watch::validate_command(state, &command).map_err(diagnostic_json)?;
            let applied = flood_apply_action(state, action).map_err(diagnostic_json)?;
            let effects = applied.effects;
            let viewer = flood_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = flood_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: seat.0,
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"policy_id\":\"{}\",\"policy_version\":{},\"rationale\":\"{}\",\"effects\":{},\"view\":{}}}",
                escape_json(&decision.policy_id),
                decision.policy_version,
                escape_json(&decision.rationale),
                effect_json,
                flood_view_json(&flood_project_view(state, &viewer))
            ))
        }
        MatchRecord::FrontierControl {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_frontier_seat(actor_seat)?;
            let faction = state.faction_for_seat(&seat).ok_or_else(|| {
                format!(
                    "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                    escape_json(actor_seat)
                )
            })?;
            let decision = match faction {
                FrontierFactionId::Garrison => FrontierGarrisonLevel1Bot::new(Seed(bot_seed))
                    .select_decision(state, &seat)
                    .map_err(diagnostic_json)?,
                FrontierFactionId::Prospectors => FrontierProspectorLevel1Bot::new(Seed(bot_seed))
                    .select_decision(state, &seat)
                    .map_err(diagnostic_json)?,
            };
            let command = frontier_command_for_decision(state, &seat, &decision);
            let applied = frontier_apply_command(state, &command).map_err(diagnostic_json)?;
            let effects = applied.effects;
            let viewer = frontier_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = frontier_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: seat.0,
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"policy_id\":\"{}\",\"policy_version\":{},\"rationale\":\"{}\",\"effects\":{},\"view\":{}}}",
                escape_json(&decision.policy_id),
                decision.policy_version,
                escape_json(&decision.rationale),
                effect_json,
                frontier_view_json(&frontier_project_view(state, &viewer))
            ))
        }
        MatchRecord::EventFrontier {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_event_frontier_seat(actor_seat)?;
            let faction = state.faction_for_seat(&seat).ok_or_else(|| {
                format!(
                    "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                    escape_json(actor_seat)
                )
            })?;
            let decision = match faction {
                EventFrontierFactionId::Charter => EventCharterLevel1Bot::new(Seed(bot_seed))
                    .select_decision(state, &seat)
                    .map_err(diagnostic_json)?,
                EventFrontierFactionId::Freeholders => {
                    EventFreeholdersLevel1Bot::new(Seed(bot_seed))
                        .select_decision(state, &seat)
                        .map_err(diagnostic_json)?
                }
            };
            let command = event_frontier_command_for_decision(state, &seat, &decision);
            let applied = event_frontier_apply_command(state, &command).map_err(diagnostic_json)?;
            let mut effects = applied.effects;
            effects.extend(event_frontier_finish_automated_phases(state)?);
            let viewer = event_frontier_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = event_frontier_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: seat.0,
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"policy_id\":\"{}\",\"policy_version\":{},\"rationale\":\"{}\",\"effects\":{},\"view\":{}}}",
                escape_json(&decision.policy_id),
                decision.policy_version,
                escape_json(&decision.rationale),
                effect_json,
                event_frontier_view_json(&event_frontier_project_view(state, &viewer))
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
        MatchRecord::SecretDraft {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_secret_seat(actor_seat)?;
            let decision = SecretDraftLevel1Bot::new(Seed(bot_seed))
                .select_decision(state, seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: secret_actor_for_seat(state, seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = secret_draft::actions::validate_command(state, &command)
                .map_err(diagnostic_json)?;
            let effects = secret_apply_action(state, action).map_err(diagnostic_json)?;
            let viewer = secret_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = secret_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_secret_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"policy_id\":\"{}\",\"policy_version\":{},\"rationale\":\"{}\",\"effects\":{},\"view\":{}}}",
                escape_json(&decision.policy_id),
                decision.policy_version,
                escape_json(&decision.rationale),
                effect_json,
                secret_view_json(&secret_project_view(state, &viewer))
            ))
        }
        MatchRecord::PokerLite {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_poker_seat(actor_seat)?;
            let decision = PokerLiteLevel2Bot::new(Seed(bot_seed))
                .select_decision(state, seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: poker_actor_for_seat(state, seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action = poker_lite::validate_command(state, &command).map_err(diagnostic_json)?;
            let mut effects = decision.effects;
            effects.extend(poker_apply_action(state, action).map_err(diagnostic_json)?);
            let viewer = poker_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = poker_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_poker_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"policy_id\":\"{}\",\"policy_version\":{},\"rationale\":\"{}\",\"effects\":{},\"view\":{}}}",
                escape_json(&decision.policy_id),
                decision.policy_version,
                escape_json(&decision.rationale),
                effect_json,
                poker_view_json(&poker_project_view(state, &viewer))
            ))
        }
        MatchRecord::PlainTricks {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_plain_seat(actor_seat)?;
            let decision = PlainTricksLevel2Bot::new(Seed(bot_seed))
                .select_decision(state, seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: plain_actor_for_seat(state, seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                plain_tricks::validate_command(state, &command).map_err(diagnostic_json)?;
            let mut effects = decision.effects;
            effects.extend(plain_apply_action(state, action).map_err(diagnostic_json)?);
            let viewer = plain_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = plain_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_plain_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            Ok(format!(
                "{{\"ok\":true,\"policy_id\":\"{}\",\"policy_version\":{},\"rationale\":\"{}\",\"effects\":{},\"view\":{}}}",
                escape_json(&decision.policy_id),
                decision.policy_version,
                escape_json(&decision.rationale),
                effect_json,
                plain_view_json(&plain_project_view(state, &viewer))
            ))
        }
        MatchRecord::RiverLedger {
            game_id,
            state,
            effects: effect_log,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let seat = parse_river_seat(actor_seat)?;
            let decision = RiverLedgerLevel2Bot::new(Seed(bot_seed))
                .select_decision(state, seat)
                .map_err(diagnostic_json)?;
            let command = CommandEnvelope {
                actor: river_actor_for_seat(state, seat)?,
                action_path: decision.action_path,
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(RULES_VERSION),
            };
            let action =
                river_ledger::validate_command(state, &command).map_err(diagnostic_json)?;
            let effects = river_apply_action(state, action).map_err(diagnostic_json)?;
            let viewer = river_viewer_for_seat(state, Some(actor_seat))?;
            let effect_json = river_effects_json(&effects, &viewer);
            for effect in effects {
                effect_log.push(effect);
            }
            commands.push(AppliedCommand {
                actor_seat: trace_river_seat(seat).to_owned(),
                action_path: command.action_path.segments,
                freshness_token: command.freshness_token.0,
            });
            let bot_explanation = decision.public_explanation.as_ref().map_or_else(
                || "null".to_owned(),
                |explanation| {
                    river_bot_decision_public_explanation_json(
                        &river_ledger::visibility::project_bot_decision_public_explanation(
                            explanation,
                        ),
                    )
                },
            );
            Ok(format!(
                "{{\"ok\":true,\"policy_id\":\"{}\",\"policy_version\":{},\"rationale\":\"{}\",\"bot_explanation\":{},\"effects\":{},\"view\":{}}}",
                escape_json(&decision.policy_id),
                decision.policy_version,
                escape_json(&decision.rationale),
                bot_explanation,
                effect_json,
                river_view_json(&river_project_view(state, &viewer))
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
        MatchRecord::MaskedClaims {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = masked_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        masked_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
        MatchRecord::FloodWatch {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = flood_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        flood_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
        MatchRecord::FrontierControl {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = frontier_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        frontier_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
        MatchRecord::EventFrontier {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = event_frontier_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        event_frontier_effect_json(&logged.envelope)
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
        MatchRecord::SecretDraft {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = secret_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        secret_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
        MatchRecord::PokerLite {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = poker_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        poker_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
        MatchRecord::PlainTricks {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = plain_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        plain_effect_json(&logged.envelope)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{effects}]"))
        }
        MatchRecord::RiverLedger {
            game_id,
            state,
            effects,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = river_viewer_for_seat(state, viewer_seat)?;
            let effects = effects
                .since(EffectCursor(since_cursor), &viewer)
                .into_iter()
                .map(|logged| {
                    format!(
                        "{{\"cursor\":{},\"effect\":{}}}",
                        logged.cursor.0,
                        river_effect_json(&logged.envelope)
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
        MatchRecord::MaskedClaims {
            game_id,
            state,
            effects,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let view = masked_project_view(state, &Viewer { seat_id: None });
            let public_effects = effects
                .since(EffectCursor(0), &Viewer { seat_id: None })
                .into_iter()
                .map(|logged| format!("{:?}", logged.envelope.payload))
                .collect::<Vec<_>>();
            let steps = vec![masked_claims::PublicReplayStep::from_view(
                0,
                &view,
                public_effects,
                commands
                    .last()
                    .map_or_else(|| "setup".to_owned(), masked_redacted_command_summary),
                matches!(state.phase, masked_claims::Phase::Terminal),
            )];
            Ok(masked_claims::PublicReplayExport::new("observer", steps).to_json())
        }
        MatchRecord::FloodWatch {
            game_id,
            state,
            effects,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = Viewer { seat_id: None };
            let public_effects = effects
                .since(EffectCursor(0), &viewer)
                .into_iter()
                .map(|logged| flood_watch::public_effect_text(&logged.envelope.payload))
                .collect::<Vec<_>>();
            let steps = vec![flood_watch::PublicReplayStep {
                step_index: 0,
                public_view_summary: flood_project_view(state, &viewer).stable_summary(),
                public_effects,
                redacted_command_summary: commands
                    .last()
                    .map_or_else(|| "setup".to_owned(), flood_redacted_command_summary),
                terminal: state.terminal_outcome.is_some(),
            }];
            Ok(
                flood_watch::export_public_replay(state.variant.id.clone(), &viewer, steps)
                    .to_json(),
            )
        }
        MatchRecord::FrontierControl {
            game_id,
            state,
            effects,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = Viewer { seat_id: None };
            let public_effects = effects
                .since(EffectCursor(0), &viewer)
                .into_iter()
                .map(|logged| frontier_public_effect_text(&logged.envelope.payload))
                .collect::<Vec<_>>();
            let steps = vec![frontier_control::PublicReplayStep {
                step_index: 0,
                public_view_summary: frontier_project_view(state, &viewer).stable_summary(),
                public_effects,
                command_summary: commands
                    .last()
                    .map_or_else(|| "setup".to_owned(), frontier_command_summary),
                terminal: state.terminal_outcome.is_some(),
            }];
            Ok(frontier_control::export_public_replay(state.variant.id.clone(), steps).to_json())
        }
        MatchRecord::EventFrontier {
            game_id,
            state,
            effects,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let viewer = Viewer { seat_id: None };
            let public_effects = effects
                .since(EffectCursor(0), &viewer)
                .into_iter()
                .map(|logged| event_frontier_public_effect_text(&logged.envelope.payload))
                .collect::<Vec<_>>();
            let step = event_frontier::PublicReplayStep {
                step_index: 0,
                public_view_summary: event_frontier_project_view(state, &viewer).stable_summary(),
                public_effects,
                redacted_command_summary: commands
                    .last()
                    .map_or_else(|| "setup".to_owned(), event_frontier_command_summary),
                terminal: state.terminal_outcome.is_some(),
            };
            Ok(
                event_frontier::export_public_replay(state.variant.id.clone(), &viewer, vec![step])
                    .to_json(),
            )
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
        MatchRecord::SecretDraft {
            game_id,
            seed,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let trace = SecretDraftInternalTrace {
                schema_version: SCHEMA_VERSION,
                game_id: secret_draft::GAME_ID.to_owned(),
                rules_version: secret_draft::RULES_VERSION_LABEL.to_owned(),
                variant: secret_draft::VARIANT_ID.to_owned(),
                seed_evidence: *seed,
                commands: commands
                    .iter()
                    .map(|command| SecretReplayCommand {
                        actor: command.actor_seat.clone(),
                        path: command.action_path.clone(),
                    })
                    .collect(),
            };
            Ok(secret_export_public_replay(&trace, &Viewer { seat_id: None }).to_json())
        }
        MatchRecord::PokerLite {
            game_id,
            seed,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let trace = PokerLiteInternalTrace {
                schema_version: SCHEMA_VERSION,
                game_id: poker_lite::GAME_ID.to_owned(),
                rules_version: poker_lite::RULES_VERSION_LABEL.to_owned(),
                variant: poker_lite::VARIANT_ID.to_owned(),
                seed_evidence: *seed,
                commands: commands
                    .iter()
                    .map(|command| PokerReplayCommand {
                        actor: command.actor_seat.clone(),
                        path: command.action_path.clone(),
                    })
                    .collect(),
            };
            Ok(poker_export_public_replay(&trace, &Viewer { seat_id: None }).to_json())
        }
        MatchRecord::PlainTricks {
            game_id,
            seed,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let trace = PlainTricksInternalTrace {
                schema_version: SCHEMA_VERSION,
                game_id: plain_tricks::GAME_ID.to_owned(),
                rules_version: plain_tricks::RULES_VERSION_LABEL.to_owned(),
                variant: plain_tricks::VARIANT_ID.to_owned(),
                seed_evidence: *seed,
                commands: commands
                    .iter()
                    .map(|command| PlainReplayCommand {
                        actor: command.actor_seat.clone(),
                        path: command.action_path.clone(),
                    })
                    .collect(),
            };
            Ok(plain_export_public_replay(&trace, &Viewer { seat_id: None }).to_json())
        }
        MatchRecord::RiverLedger {
            game_id,
            seed,
            state,
            commands,
            ..
        } => {
            resolve_game(game_id)?;
            let trace = RiverLedgerInternalTrace {
                schema_version: SCHEMA_VERSION,
                game_id: river_ledger::GAME_ID.to_owned(),
                rules_version: river_ledger::RULES_VERSION_LABEL.to_owned(),
                variant: river_ledger::VARIANT_ID.to_owned(),
                seed_evidence: *seed,
                seat_count: state.seats.len(),
                commands: commands
                    .iter()
                    .map(|command| RiverReplayCommand {
                        actor: command.actor_seat.clone(),
                        path: command.action_path.clone(),
                    })
                    .collect(),
            };
            Ok(river_export_public_replay(&trace, &Viewer { seat_id: None }).to_json())
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
    if is_masked_public_export(doc) {
        return import_masked_public_replay(doc);
    }
    if is_flood_watch_public_export(doc) {
        return import_flood_watch_public_replay(doc);
    }
    if is_frontier_control_public_export(doc) {
        return import_frontier_control_public_replay(doc);
    }
    if is_event_frontier_public_export(doc) {
        return import_event_frontier_public_replay(doc);
    }
    if is_secret_draft_public_export(doc) {
        return import_secret_draft_public_replay(doc);
    }
    if is_poker_lite_public_export(doc) {
        return import_poker_lite_public_replay(doc);
    }
    if is_plain_tricks_public_export(doc) {
        return import_plain_tricks_public_replay(doc);
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
        && parsed.game_id != GAME_MASKED_CLAIMS
        && parsed.game_id != GAME_FLOOD_WATCH
        && parsed.game_id != GAME_FRONTIER_CONTROL
        && parsed.game_id != GAME_EVENT_FRONTIER
        && parsed.game_id != GAME_TOKEN_BAZAAR
        && parsed.game_id != GAME_SECRET_DRAFT
        && parsed.game_id != GAME_POKER_LITE
        && parsed.game_id != GAME_PLAIN_TRICKS
        && parsed.game_id != GAME_RIVER_LEDGER
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
        RegisteredGame::MaskedClaims => {
            if command_count != 0 {
                return Err(diagnostic_string(
                    "unsupported_replay_commands",
                    "masked_claims command replay uses viewer-scoped public export/import in this bridge",
                ));
            }
            let state = masked_setup_match(
                Seed(parsed.seed),
                &masked_seats(),
                &masked_claims::SetupOptions::default(),
            )
            .map_err(diagnostic_json)?;
            (
                masked_view_json(&masked_project_view(&state, &Viewer { seat_id: None })),
                0,
            )
        }
        RegisteredGame::FloodWatch => {
            if command_count != 0 {
                return Err(diagnostic_string(
                    "unsupported_replay_commands",
                    "flood_watch command replay uses viewer-scoped public export/import in this bridge",
                ));
            }
            let state = flood_setup_match(
                Seed(parsed.seed),
                &flood_seats(),
                &flood_watch::SetupOptions::default(),
            )
            .map_err(diagnostic_json)?;
            (
                flood_view_json(&flood_project_view(&state, &Viewer { seat_id: None })),
                0,
            )
        }
        RegisteredGame::FrontierControl => {
            if command_count != 0 {
                return Err(diagnostic_string(
                    "unsupported_replay_commands",
                    "frontier_control command replay uses public export/import in this bridge",
                ));
            }
            let state = frontier_setup_match(
                &frontier_seats(),
                &frontier_control::SetupOptions::default(),
            )
            .map_err(diagnostic_json)?;
            (
                frontier_view_json(&frontier_project_view(&state, &Viewer { seat_id: None })),
                0,
            )
        }
        RegisteredGame::EventFrontier => {
            if command_count != 0 {
                return Err(diagnostic_string(
                    "unsupported_replay_commands",
                    "event_frontier command replay uses public export/import in this bridge",
                ));
            }
            let state = event_frontier_setup_match(
                Seed(parsed.seed),
                &event_frontier_seats(),
                &event_frontier::SetupOptions::default(),
            )
            .map_err(diagnostic_json)?;
            (
                event_frontier_view_json(&event_frontier_project_view(
                    &state,
                    &Viewer { seat_id: None },
                )),
                0,
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
        RegisteredGame::SecretDraft => {
            let (state, effects) =
                secret_replay_to_cursor(parsed.seed, &parsed.commands, command_count)?;
            (
                secret_view_json(&secret_project_view(&state, &Viewer { seat_id: None })),
                effects.len(),
            )
        }
        RegisteredGame::PokerLite => {
            let (state, effects) =
                poker_replay_to_cursor(parsed.seed, &parsed.commands, command_count)?;
            (
                poker_view_json(&poker_project_view(&state, &Viewer { seat_id: None })),
                effects.len(),
            )
        }
        RegisteredGame::PlainTricks => {
            let (state, effects) =
                plain_replay_to_cursor(parsed.seed, &parsed.commands, command_count)?;
            (
                plain_view_json(&plain_project_view(&state, &Viewer { seat_id: None })),
                effects.len(),
            )
        }
        RegisteredGame::RiverLedger => {
            let (state, effects) =
                river_replay_to_cursor(parsed.seed, &parsed.commands, command_count)?;
            (
                river_view_json(&river_project_view(&state, &Viewer { seat_id: None })),
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
            RegisteredGame::MaskedClaims => {
                let state = masked_setup_match(
                    Seed(record.seed),
                    &masked_seats(),
                    &masked_claims::SetupOptions::default(),
                )
                .map_err(diagnostic_json)?;
                Ok(format!(
                    "{{\"replay_id\":\"{}\",\"cursor\":0,\"total_commands\":0,\"view\":{},\"effects\":[]}}",
                    escape_json(replay_id),
                    masked_view_json(&masked_project_view(&state, &Viewer { seat_id: None }))
                ))
            }
            RegisteredGame::FloodWatch => {
                let state = flood_setup_match(
                    Seed(record.seed),
                    &flood_seats(),
                    &flood_watch::SetupOptions::default(),
                )
                .map_err(diagnostic_json)?;
                Ok(format!(
                    "{{\"replay_id\":\"{}\",\"cursor\":0,\"total_commands\":0,\"view\":{},\"effects\":[]}}",
                    escape_json(replay_id),
                    flood_view_json(&flood_project_view(&state, &Viewer { seat_id: None }))
                ))
            }
            RegisteredGame::FrontierControl => {
                let state = frontier_setup_match(
                    &frontier_seats(),
                    &frontier_control::SetupOptions::default(),
                )
                .map_err(diagnostic_json)?;
                Ok(format!(
                    "{{\"replay_id\":\"{}\",\"cursor\":0,\"total_commands\":0,\"view\":{},\"effects\":[]}}",
                    escape_json(replay_id),
                    frontier_view_json(&frontier_project_view(&state, &Viewer { seat_id: None }))
                ))
            }
            RegisteredGame::EventFrontier => {
                let state = event_frontier_setup_match(
                    Seed(record.seed),
                    &event_frontier_seats(),
                    &event_frontier::SetupOptions::default(),
                )
                .map_err(diagnostic_json)?;
                Ok(format!(
                    "{{\"replay_id\":\"{}\",\"cursor\":0,\"total_commands\":0,\"view\":{},\"effects\":[]}}",
                    escape_json(replay_id),
                    event_frontier_view_json(&event_frontier_project_view(
                        &state,
                        &Viewer { seat_id: None }
                    ))
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
            RegisteredGame::SecretDraft => {
                let bounded_cursor = cursor.min(record.commands.len());
                let (state, effects) =
                    secret_replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
                Ok(secret_replay_step_json(
                    replay_id,
                    bounded_cursor,
                    record.commands.len(),
                    &state,
                    &effects,
                ))
            }
            RegisteredGame::PokerLite => {
                let bounded_cursor = cursor.min(record.commands.len());
                let (state, effects) =
                    poker_replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
                Ok(poker_replay_step_json(
                    replay_id,
                    bounded_cursor,
                    record.commands.len(),
                    &state,
                    &effects,
                ))
            }
            RegisteredGame::PlainTricks => {
                let bounded_cursor = cursor.min(record.commands.len());
                let (state, effects) =
                    plain_replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
                Ok(plain_replay_step_json(
                    replay_id,
                    bounded_cursor,
                    record.commands.len(),
                    &state,
                    &effects,
                ))
            }
            RegisteredGame::RiverLedger => {
                let bounded_cursor = cursor.min(record.commands.len());
                let (state, effects) =
                    river_replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
                Ok(river_replay_step_json(
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
        GAME_MASKED_CLAIMS => Ok(RegisteredGame::MaskedClaims),
        GAME_FLOOD_WATCH => Ok(RegisteredGame::FloodWatch),
        GAME_FRONTIER_CONTROL => Ok(RegisteredGame::FrontierControl),
        GAME_EVENT_FRONTIER => Ok(RegisteredGame::EventFrontier),
        GAME_TOKEN_BAZAAR => Ok(RegisteredGame::TokenBazaar),
        GAME_SECRET_DRAFT => Ok(RegisteredGame::SecretDraft),
        GAME_POKER_LITE => Ok(RegisteredGame::PokerLite),
        GAME_PLAIN_TRICKS => Ok(RegisteredGame::PlainTricks),
        GAME_RIVER_LEDGER => Ok(RegisteredGame::RiverLedger),
        _ => Err(format!(
            "{{\"code\":\"unknown_game\",\"message\":\"unsupported game id: {}\"}}",
            escape_json(game_id)
        )),
    }
}

fn trace_rules_version(game: RegisteredGame) -> &'static str {
    match game {
        RegisteredGame::RaceToN => RACE_TRACE_RULES_VERSION,
        RegisteredGame::ThreeMarks => THREE_MARKS_TRACE_RULES_VERSION,
        RegisteredGame::ColumnFour => COLUMN_FOUR_TRACE_RULES_VERSION,
        RegisteredGame::DirectionalFlip => DIRECTIONAL_FLIP_TRACE_RULES_VERSION,
        RegisteredGame::DraughtsLite => DRAUGHTS_LITE_TRACE_RULES_VERSION,
        RegisteredGame::HighCardDuel => HIGH_CARD_DUEL_TRACE_RULES_VERSION,
        RegisteredGame::MaskedClaims => MASKED_CLAIMS_TRACE_RULES_VERSION,
        RegisteredGame::FloodWatch => FLOOD_WATCH_TRACE_RULES_VERSION,
        RegisteredGame::FrontierControl => FRONTIER_CONTROL_TRACE_RULES_VERSION,
        RegisteredGame::EventFrontier => EVENT_FRONTIER_TRACE_RULES_VERSION,
        RegisteredGame::TokenBazaar => TOKEN_BAZAAR_TRACE_RULES_VERSION,
        RegisteredGame::SecretDraft => SECRET_DRAFT_TRACE_RULES_VERSION,
        RegisteredGame::PokerLite => POKER_LITE_TRACE_RULES_VERSION,
        RegisteredGame::PlainTricks => PLAIN_TRICKS_TRACE_RULES_VERSION,
        RegisteredGame::RiverLedger => RIVER_LEDGER_TRACE_RULES_VERSION,
    }
}

pub(crate) fn effect_visible_to_viewer(visibility: &VisibilityScope, viewer: &Viewer) -> bool {
    match visibility {
        VisibilityScope::Public => true,
        VisibilityScope::PrivateToSeat(seat) => viewer.seat_id.as_ref() == Some(seat),
    }
}

pub(crate) fn visibility_json(visibility: &VisibilityScope) -> String {
    match visibility {
        VisibilityScope::Public => "\"public\"".to_owned(),
        VisibilityScope::PrivateToSeat(seat) => {
            format!("{{\"private_to_seat\":\"{}\"}}", escape_json(&seat.0))
        }
    }
}

pub(crate) fn river_catalog_seat_labels_json() -> String {
    let labels = river_ledger::ui_metadata()
        .seat_labels
        .iter()
        .map(river_seat_display_label_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{labels}]")
}

pub(crate) fn event_frontier_catalog_ui_json() -> String {
    let ui = event_frontier::ui_metadata();
    format!(
        "{{\"seat_labels\":[{}],\"faction_labels\":[{}]}}",
        ui.seat_labels
            .iter()
            .map(event_frontier_seat_display_label_json)
            .collect::<Vec<_>>()
            .join(","),
        ui.faction_labels
            .iter()
            .map(event_frontier_faction_display_label_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn event_frontier_catalog_seat_labels_json() -> String {
    let ui = event_frontier::ui_metadata();
    format!(
        "[{}]",
        ui.seat_labels
            .iter()
            .map(event_frontier_seat_display_label_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn option_string_json(value: Option<&str>) -> String {
    value.map_or_else(
        || "null".to_owned(),
        |value| format!("\"{}\"", escape_json(value)),
    )
}

pub(crate) fn option_bool_json(value: Option<bool>) -> String {
    value.map_or_else(|| "null".to_owned(), |value| value.to_string())
}

pub(crate) fn string_array(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests;
