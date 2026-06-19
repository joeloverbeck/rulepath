//! Browser-facing Rulepath API surface.

use std::{cell::RefCell, slice, str};

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
use engine_core::{
    ActionPath, CommandEnvelope, EffectCursor, EffectEnvelope, EffectLog, RulesVersion, Seed,
    Viewer, VisibilityScope,
};
#[cfg(test)]
use engine_core::{Actor, HashValue, SeatId, StableSerialize};
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
use plain_tricks::{
    apply_action as plain_apply_action, legal_action_tree as plain_legal_action_tree,
    project_view as plain_project_view,
    replay_support::{
        export_public_replay as plain_export_public_replay,
        import_public_export as plain_import_public_export, PlainTricksInternalTrace,
        PublicReplayStep as PlainPublicReplayStep, ReplayCommand as PlainReplayCommand,
    },
    setup_match as plain_setup_match, PlainTricksEffect, PlainTricksLevel2Bot, PlainTricksSeat,
    PlainTricksState,
};
use poker_lite::{
    apply_action as poker_apply_action, legal_action_tree as poker_legal_action_tree,
    project_view as poker_project_view,
    replay_support::{
        export_public_replay as poker_export_public_replay,
        import_public_export as poker_import_public_export, PokerLiteInternalTrace,
        PublicReplayStep as PokerPublicReplayStep, ReplayCommand as PokerReplayCommand,
    },
    setup_match as poker_setup_match, PokerLiteEffect, PokerLiteLevel2Bot, PokerLiteSeat,
    PokerLiteState,
};
#[cfg(test)]
use race_to_n::replay_support::replay_commands as race_replay_commands;
use race_to_n::{
    apply_action as race_apply_action, legal_action_tree, project_view,
    setup_match as race_setup_match, RaceEffect, RaceRandomBot, RaceState,
    SetupOptions as RaceSetupOptions,
};
use river_ledger::{
    apply_action as river_apply_action, legal_action_tree as river_legal_action_tree,
    project_view as river_project_view,
    replay_support::{
        export_public_replay as river_export_public_replay, ReplayCommand as RiverReplayCommand,
        RiverLedgerInternalTrace,
    },
    setup_match as river_setup_match, RiverLedgerEffect, RiverLedgerLevel2Bot, RiverLedgerSeat,
    RiverLedgerState,
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
use games::race::*;
use games::secret::*;
use games::three::*;
use games::token::*;
use json::*;
use json_parse::*;
use replay::*;
use seats::*;
use store::*;

pub use catalog::{feature_report, list_games};

thread_local! {
    /// Scratch buffer backing the WASM pointer/length output ABI.
    static LAST_OUTPUT: RefCell<String> = const { RefCell::new(String::new()) };
}

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

fn is_poker_lite_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("viewer_scoped_observation_v1")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(poker_lite::GAME_ID)
    )
}

fn is_plain_tricks_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("viewer_scoped_observation_v1")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(plain_tricks::GAME_ID)
    )
}

fn import_poker_lite_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export =
        poker_lite::replay_support::PublicReplayExport::from_json(doc).map_err(|message| {
            diagnostic_string(
                "invalid_replay",
                &format!("invalid public replay document: {message}"),
            )
        })?;
    if export.rules_version != poker_lite::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {}", export.rules_version),
        ));
    }
    if export.variant != poker_lite::VARIANT_ID {
        return Err(diagnostic_string(
            "unsupported_replay_variant",
            &format!("unsupported replay variant: {}", export.variant),
        ));
    }
    if export.viewer != "observer" {
        return Err(diagnostic_string(
            "unsupported_replay_viewer",
            &format!("unsupported replay viewer: {}", export.viewer),
        ));
    }
    let timeline = poker_import_public_export(&export);
    let replay_id = next_replay_id(GAME_POKER_LITE);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_POKER_LITE.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: timeline.viewer.clone(),
                    steps: timeline
                        .steps
                        .iter()
                        .map(public_timeline_step_from_poker)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_POKER_LITE),
        escape_json(&timeline.viewer),
        timeline.steps.len()
    ))
}

fn import_plain_tricks_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export =
        plain_tricks::replay_support::PublicReplayExport::from_json(doc).map_err(|message| {
            diagnostic_string(
                "invalid_replay",
                &format!("invalid public replay document: {message}"),
            )
        })?;
    if export.rules_version != plain_tricks::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {}", export.rules_version),
        ));
    }
    if export.variant != plain_tricks::VARIANT_ID {
        return Err(diagnostic_string(
            "unsupported_replay_variant",
            &format!("unsupported replay variant: {}", export.variant),
        ));
    }
    if export.viewer != "observer" {
        return Err(diagnostic_string(
            "unsupported_replay_viewer",
            &format!("unsupported replay viewer: {}", export.viewer),
        ));
    }
    let timeline = plain_import_public_export(&export);
    let replay_id = next_replay_id(GAME_PLAIN_TRICKS);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_PLAIN_TRICKS.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: timeline.viewer.clone(),
                    steps: timeline
                        .steps
                        .iter()
                        .map(public_timeline_step_from_plain)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_PLAIN_TRICKS),
        escape_json(&timeline.viewer),
        timeline.steps.len()
    ))
}

fn public_timeline_step_from_poker(step: &PokerPublicReplayStep) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

fn public_timeline_step_from_plain(step: &PlainPublicReplayStep) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
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

fn poker_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(PokerLiteState, Vec<EffectEnvelope<PokerLiteEffect>>), String> {
    let seats = seats();
    let mut state = poker_setup_match(Seed(seed), &seats, &poker_lite::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_poker_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: poker_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = poker_lite::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(poker_apply_action(&mut state, action).map_err(diagnostic_json)?);
    }
    Ok((state, all_effects))
}

fn plain_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(PlainTricksState, Vec<EffectEnvelope<PlainTricksEffect>>), String> {
    let seats = plain_seats();
    let mut state = plain_setup_match(Seed(seed), &seats, &plain_tricks::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_plain_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: plain_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = plain_tricks::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(plain_apply_action(&mut state, action).map_err(diagnostic_json)?);
    }
    Ok((state, all_effects))
}

fn river_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(RiverLedgerState, Vec<EffectEnvelope<RiverLedgerEffect>>), String> {
    let seats = river_seats_for_count(6);
    let mut state = river_setup_match(Seed(seed), &seats, &river_ledger::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_river_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: river_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = river_ledger::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(river_apply_action(&mut state, action).map_err(diagnostic_json)?);
    }
    Ok((state, all_effects))
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

#[cfg(test)]
fn plain_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let mut state = plain_setup_match(
        Seed(seed),
        &plain_seats(),
        &plain_tricks::SetupOptions::default(),
    )
    .map_err(diagnostic_json)?;
    let mut effects = plain_tricks::setup_effects(&state);
    let mut replay_commands = Vec::new();

    for command in commands {
        let seat = parse_plain_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: plain_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = plain_tricks::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        effects.extend(plain_apply_action(&mut state, action).map_err(diagnostic_json)?);
        replay_commands.push(PlainReplayCommand {
            actor: trace_plain_seat(seat).to_owned(),
            path: command.action_path.clone(),
        });
    }

    let trace = PlainTricksInternalTrace {
        schema_version: SCHEMA_VERSION,
        game_id: plain_tricks::GAME_ID.to_owned(),
        rules_version: plain_tricks::RULES_VERSION_LABEL.to_owned(),
        variant: plain_tricks::VARIANT_ID.to_owned(),
        seed_evidence: seed,
        commands: replay_commands,
    };
    let public_export = plain_export_public_replay(&trace, &Viewer { seat_id: None });
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
    let (terminal, winner, draw) = match state.terminal_outcome {
        Some(plain_tricks::TerminalOutcome::TrickWin { winner, .. }) => {
            (true, format!("\"{}\"", winner.as_str()), false)
        }
        Some(plain_tricks::TerminalOutcome::Split { .. }) => (true, "null".to_owned(), true),
        None => (false, "null".to_owned(), false),
    };

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"plain-tricks-{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"commands\":[{}],\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"observer\":{},\"seat_0\":{},\"seat_1\":{}}},\"expected_private_view_hashes\":{{\"seat_0\":{},\"seat_1\":{}}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_diagnostic_hashes\":null,\"expected_public_export_hashes\":{{\"final\":{}}},\"expected_diagnostics\":null,\"expected_terminal_state\":{{\"terminal\":{},\"winner\":{},\"draw\":{}}},\"note\":\"Plain Tricks wasm-exported fixture generated by the Rulepath WASM API.\",\"migration_update_note\":\"Refreshed with real WASM export evidence by GAT101PLATRI-016.\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat_0\",\"player_id\":\"player_0\"}},{{\"seat_id\":\"seat_1\",\"player_id\":\"player_1\"}}],\"checkpoints\":{},\"expected_outcome\":{{\"terminal\":{},\"winner\":{},\"draw\":{}}},\"not_applicable\":{{\"hidden_information\":\"Plain Tricks has hidden private hands and an internal tail; viewer-scoped traces and no-leak tests verify redaction.\",\"stochastic_game_events\":\"Setup uses deterministic seeded shuffle; no later stochastic rule events occur.\",\"preview_hashes\":\"Plain Tricks uses action metadata rather than a separate preview hash.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_PLAIN_TRICKS),
        escape_json(PLAIN_TRICKS_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_PLAIN_TRICKS_STANDARD),
        commands_json,
        plain_tricks::replay_support::state_hash(&state).0,
        plain_tricks::replay_support::effect_hash(&effects).0,
        plain_action_tree_hash(&state).0,
        plain_tricks::replay_support::view_hash(&state, &Viewer { seat_id: None }).0,
        plain_tricks::replay_support::view_hash(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_0".to_owned()))
            }
        )
        .0,
        plain_tricks::replay_support::view_hash(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_1".to_owned()))
            }
        )
        .0,
        plain_tricks::replay_support::view_hash(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_0".to_owned()))
            }
        )
        .0,
        plain_tricks::replay_support::view_hash(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_1".to_owned()))
            }
        )
        .0,
        trace.stable_hash().0,
        public_export.stable_hash().0,
        terminal,
        winner,
        draw,
        checkpoints,
        terminal,
        winner,
        draw
    ))
}

#[cfg(test)]
fn plain_action_tree_hash(state: &PlainTricksState) -> HashValue {
    let parts = PlainTricksSeat::ALL
        .iter()
        .map(|seat| {
            let actor = Actor {
                seat_id: state.seats[seat.index()].clone(),
            };
            plain_tricks::replay_support::action_tree_hash(&plain_legal_action_tree(state, &actor))
                .0
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(parts.as_bytes())
}

fn poker_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &PokerLiteState,
    effects: &[EffectEnvelope<PokerLiteEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        poker_view_json(&poker_project_view(state, &viewer)),
        poker_effects_json(effects, &viewer)
    )
}

fn plain_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &PlainTricksState,
    effects: &[EffectEnvelope<PlainTricksEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        plain_view_json(&plain_project_view(state, &viewer)),
        plain_effects_json(effects, &viewer)
    )
}

fn river_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &RiverLedgerState,
    effects: &[EffectEnvelope<RiverLedgerEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        river_view_json(&river_project_view(state, &viewer)),
        river_effects_json(effects, &viewer)
    )
}

fn poker_view_json(view: &poker_lite::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"phase\":\"{}\",\"active_seat\":{},\"shared_pool\":{},\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}},\"round\":{},\"private_counts\":{{\"seat_0\":{},\"seat_1\":{}}},\"center\":{},\"showdown\":{},\"terminal\":{},\"terminal_rationale\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.phase.as_str(),
        poker_optional_seat_json(view.active_seat),
        view.shared_pool,
        view.contributions[0],
        view.contributions[1],
        poker_round_json(&view.round),
        view.private_counts[0],
        view.private_counts[1],
        poker_center_json(&view.center),
        view.showdown
            .as_ref()
            .map_or_else(|| "null".to_owned(), poker_showdown_json),
        poker_terminal_json(&view.terminal),
        poker_terminal_rationale(&view.terminal)
            .map_or_else(|| "null".to_owned(), poker_rationale_json),
        view.freshness_token.0,
        poker_private_view_json(&view.private_view),
        poker_ui_json(&view.ui)
    )
}

fn river_view_json(view: &river_ledger::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"phase\":\"{}\",\"active_seat\":{},\"active_seat_labels\":[{}],\"button\":\"{}\",\"small_blind\":\"{}\",\"big_blind\":\"{}\",\"pot_total\":{},\"seats\":[{}],\"board\":[{}],\"board_slots\":[{}],\"terminal\":{},\"terminal_rationale\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        river_phase_label(view.phase),
        option_river_seat_json(view.active_seat),
        view.active_seat_labels
            .iter()
            .map(river_seat_display_label_json)
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&view.button.as_str()),
        escape_json(&view.small_blind.as_str()),
        escape_json(&view.big_blind.as_str()),
        view.pot_total,
        view.seats
            .iter()
            .map(river_seat_view_json)
            .collect::<Vec<_>>()
            .join(","),
        view.board
            .iter()
            .map(river_card_json)
            .collect::<Vec<_>>()
            .join(","),
        view.board_slots
            .iter()
            .map(river_board_slot_json)
            .collect::<Vec<_>>()
            .join(","),
        river_terminal_json(&view.terminal),
        view.terminal_rationale
            .as_ref()
            .map_or_else(|| "null".to_owned(), river_rationale_json),
        view.freshness_token.0,
        river_private_view_json(&view.private_view),
        river_ui_json(&view.ui)
    )
}

fn river_phase_label(phase: river_ledger::Phase) -> &'static str {
    match phase {
        river_ledger::Phase::Setup => "setup",
        river_ledger::Phase::Betting { street } => street.as_str(),
        river_ledger::Phase::Showdown => "showdown",
        river_ledger::Phase::Terminal => "terminal",
    }
}

fn option_river_seat_json(seat: Option<RiverLedgerSeat>) -> String {
    seat.map(|seat| format!("\"{}\"", escape_json(&seat.as_str())))
        .unwrap_or_else(|| "null".to_owned())
}

fn river_seat_view_json(seat: &river_ledger::visibility::SeatView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"status\":\"{}\",\"street_contribution\":{},\"total_contribution\":{},\"hidden_hole_count\":{},\"ledger_display\":{}}}",
        escape_json(&seat.seat.as_str()),
        river_seat_status_label(seat.status),
        seat.street_contribution,
        seat.total_contribution,
        seat.hidden_hole_count,
        river_seat_ledger_display_json(&seat.ledger_display)
    )
}

fn river_seat_ledger_display_json(
    display: &river_ledger::ui::RiverLedgerSeatLedgerDisplay,
) -> String {
    format!(
        "{{\"round_contribution\":{},\"hand_contribution\":{},\"hole_card_summary\":{},\"role_badges\":[{}],\"status_label\":\"{}\"}}",
        river_seat_ledger_field_json(&display.round_contribution),
        river_seat_ledger_field_json(&display.hand_contribution),
        river_seat_ledger_field_json(&display.hole_card_summary),
        display
            .role_badges
            .iter()
            .map(|badge| format!("\"{}\"", escape_json(badge)))
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&display.status_label)
    )
}

fn river_seat_ledger_field_json(field: &river_ledger::ui::RiverLedgerSeatLedgerField) -> String {
    format!(
        "{{\"label\":\"{}\",\"value\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&field.label),
        escape_json(&field.value),
        escape_json(&field.accessibility_label)
    )
}

fn river_seat_status_label(status: river_ledger::SeatStatus) -> &'static str {
    match status {
        river_ledger::SeatStatus::Live => "live",
        river_ledger::SeatStatus::Folded => "folded",
        river_ledger::SeatStatus::ShowdownEligible => "showdown_eligible",
    }
}

fn river_card_json(card: &river_ledger::CardView) -> String {
    format!(
        "{{\"card_id\":\"{}\",\"rank\":\"{}\",\"rank_value\":{},\"suit\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.card_id),
        escape_json(&card.rank),
        card.rank_value,
        escape_json(&card.suit),
        escape_json(&card.label),
        escape_json(&format!("{} of {}", card.rank, card.suit))
    )
}

fn river_board_slot_json(slot: &river_ledger::visibility::BoardSlotView) -> String {
    format!(
        "{{\"slot\":\"{}\",\"reveal_state\":\"{}\",\"street_label\":\"{}\",\"visual_placeholder_label\":\"{}\",\"accessibility_label\":\"{}\",\"card\":{}}}",
        escape_json(&slot.slot),
        escape_json(&slot.reveal_state),
        escape_json(&slot.street_label),
        escape_json(&slot.visual_placeholder_label),
        escape_json(&slot.accessibility_label),
        slot.card
            .as_ref()
            .map_or_else(|| "null".to_owned(), river_card_json)
    )
}

fn river_terminal_json(terminal: &river_ledger::visibility::TerminalView) -> String {
    match terminal {
        river_ledger::visibility::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"terminal\":false,\"winners\":[],\"pot_total\":0,\"allocations\":[],\"explanations\":[]}".to_owned()
        }
        river_ledger::visibility::TerminalView::LastLiveHand { winner, pot_total } => format!(
            "{{\"kind\":\"last_live_hand\",\"terminal\":true,\"winners\":[\"{}\"],\"pot_total\":{},\"allocations\":[{{\"seat\":\"{}\",\"amount\":{}}}],\"explanations\":[]}}",
            escape_json(&winner.as_str()),
            pot_total,
            escape_json(&winner.as_str()),
            pot_total
        ),
        river_ledger::visibility::TerminalView::Showdown {
            winners,
            pot_total,
            allocations,
            explanations,
            presentation_v2,
        } => format!(
            "{{\"kind\":\"showdown\",\"terminal\":true,\"winners\":[{}],\"pot_total\":{},\"allocations\":[{}],\"explanations\":[{}],\"presentation_v2\":{}}}",
            winners
                .iter()
                .map(|seat| format!("\"{}\"", escape_json(&seat.as_str())))
                .collect::<Vec<_>>()
                .join(","),
            pot_total,
            allocations
                .iter()
                .map(|(seat, amount)| format!(
                    "{{\"seat\":\"{}\",\"amount\":{}}}",
                    escape_json(&seat.as_str()),
                    amount
                ))
                .collect::<Vec<_>>()
                .join(","),
            explanations
                .iter()
                .map(|explanation| format!("\"{}\"", escape_json(explanation)))
                .collect::<Vec<_>>()
                .join(","),
            river_showdown_presentation_v2_json(presentation_v2)
        ),
    }
}

fn river_showdown_presentation_v2_json(
    presentation: &river_ledger::visibility::ShowdownPresentationV2View,
) -> String {
    format!(
        "{{\"result_banner\":{},\"decisive_reason\":{},\"board_cards\":[{}],\"standings\":[{}],\"folded_rows\":[{}]}}",
        river_showdown_result_banner_json(&presentation.result_banner),
        river_showdown_decisive_reason_json(&presentation.decisive_reason),
        presentation
            .board_cards
            .iter()
            .map(river_showdown_board_card_json)
            .collect::<Vec<_>>()
            .join(","),
        presentation
            .standings
            .iter()
            .map(river_showdown_standing_json)
            .collect::<Vec<_>>()
            .join(","),
        presentation
            .folded_rows
            .iter()
            .map(river_showdown_folded_row_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn river_showdown_result_banner_json(
    banner: &river_ledger::visibility::ShowdownResultBannerView,
) -> String {
    format!(
        "{{\"headline\":\"{}\",\"subheadline\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&banner.headline),
        escape_json(&banner.subheadline),
        escape_json(&banner.accessibility_label)
    )
}

fn river_showdown_decisive_reason_json(
    reason: &river_ledger::visibility::ShowdownDecisiveReasonView,
) -> String {
    format!(
        "{{\"short_text\":\"{}\",\"contrast_seat\":{},\"contrast_seat_label\":{},\"rule_refs\":[{}]}}",
        escape_json(&reason.short_text),
        reason
            .contrast_seat
            .map(|seat| format!("\"{}\"", escape_json(&seat.as_str())))
            .unwrap_or_else(|| "null".to_owned()),
        reason
            .contrast_seat_label
            .as_ref()
            .map(|label| format!("\"{}\"", escape_json(label)))
            .unwrap_or_else(|| "null".to_owned()),
        reason
            .rule_refs
            .iter()
            .map(|rule| format!("\"{}\"", escape_json(rule)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn river_showdown_board_card_json(
    card: &river_ledger::visibility::ShowdownBoardCardPresentationView,
) -> String {
    format!(
        "{{\"slot\":\"{}\",\"card\":{},\"public_label\":\"{}\",\"used_by_selected\":[{}]}}",
        escape_json(&card.slot),
        river_card_json(&card.card),
        escape_json(&card.public_label),
        card.used_by_selected
            .iter()
            .map(|seat| format!("\"{}\"", escape_json(seat)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn river_showdown_standing_json(
    standing: &river_ledger::visibility::ShowdownStandingPresentationView,
) -> String {
    format!(
        "{{\"seat\":\"{}\",\"seat_label\":\"{}\",\"rank\":{},\"result_label\":\"{}\",\"allocation_label\":\"{}\",\"hand_name\":\"{}\",\"short_comparison_note\":\"{}\",\"rank_ladder_label\":\"{}\",\"hole_cards\":[{}],\"board_cards\":[{}],\"best_five\":[{}],\"best_five_accessibility_label\":\"{}\",\"detail_rows\":[{}],\"default_expanded\":{}}}",
        escape_json(&standing.seat.as_str()),
        escape_json(&standing.seat_label),
        standing.rank,
        escape_json(&standing.result_label),
        escape_json(&standing.allocation_label),
        escape_json(&standing.hand_name),
        escape_json(&standing.short_comparison_note),
        escape_json(&standing.rank_ladder_label),
        standing
            .hole_cards
            .iter()
            .map(river_showdown_card_usage_json)
            .collect::<Vec<_>>()
            .join(","),
        standing
            .board_cards
            .iter()
            .map(river_showdown_card_usage_json)
            .collect::<Vec<_>>()
            .join(","),
        standing
            .best_five
            .iter()
            .map(river_card_json)
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&standing.best_five_accessibility_label),
        standing
            .detail_rows
            .iter()
            .map(river_showdown_detail_row_json)
            .collect::<Vec<_>>()
            .join(","),
        standing.default_expanded
    )
}

fn river_showdown_card_usage_json(
    mark: &river_ledger::visibility::ShowdownCardUsageMarkView,
) -> String {
    format!(
        "{{\"card\":{},\"public_label\":\"{}\",\"used_in_best_five\":{}}}",
        river_card_json(&mark.card),
        escape_json(&mark.public_label),
        mark.used_in_best_five
    )
}

fn river_showdown_detail_row_json(row: &river_ledger::visibility::ShowdownDetailRowView) -> String {
    format!(
        "{{\"label\":\"{}\",\"value\":\"{}\"}}",
        escape_json(&row.label),
        escape_json(&row.value)
    )
}

fn river_showdown_folded_row_json(
    row: &river_ledger::visibility::ShowdownFoldedRowPresentationView,
) -> String {
    format!(
        "{{\"seat\":\"{}\",\"seat_label\":\"{}\",\"redaction_label\":\"{}\"}}",
        escape_json(&row.seat.as_str()),
        escape_json(&row.seat_label),
        escape_json(&row.redaction_label)
    )
}

fn river_rationale_json(rationale: &river_ledger::visibility::OutcomeRationaleView) -> String {
    let decisive_rule_ids = rationale
        .decisive_rule_ids
        .iter()
        .map(|rule_id| format!("\"{}\"", escape_json(rule_id)))
        .collect::<Vec<_>>()
        .join(",");
    let final_standing = rationale
        .per_seat
        .iter()
        .map(river_rationale_standing_json)
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"result_kind\":\"{}\",\"decisive_cause\":\"{}\",\"template_key\":\"{}\",\"headline\":{},\"decisive_comparison\":{},\"comparison_basis\":{},\"decisive_rule_ids\":[{}],\"final_standing\":[{}]}}",
        escape_json(&rationale.result_kind),
        escape_json(&rationale.decisive_cause),
        escape_json(&rationale.template_key),
        option_string_json(rationale.headline.as_deref()),
        option_string_json(rationale.decisive_comparison.as_deref()),
        option_string_json(rationale.comparison_basis.as_deref()),
        decisive_rule_ids,
        final_standing
    )
}

fn river_rationale_standing_json(
    breakdown: &river_ledger::visibility::SeatOutcomeBreakdownView,
) -> String {
    let mut values = vec![
        format!(
            "{{\"label\":\"Contribution\",\"value\":{}}}",
            breakdown.contribution
        ),
        format!(
            "{{\"label\":\"Allocation\",\"value\":{}}}",
            breakdown.allocation
        ),
    ];
    if let Some(strength) = &breakdown.strength {
        values.push(format!(
            "{{\"label\":\"Result label\",\"value\":\"{}\"}}",
            escape_json(&strength.result_label)
        ));
        values.push(format!(
            "{{\"label\":\"Hand\",\"value\":\"{}\"}}",
            escape_json(&strength.hand_name)
        ));
        values.push(format!(
            "{{\"label\":\"Rank explanation\",\"value\":\"{}\"}}",
            escape_json(&strength.rank_explanation)
        ));
        values.push(format!(
            "{{\"label\":\"Comparison\",\"value\":\"{}\"}}",
            escape_json(&strength.comparison_note)
        ));
        values.push(format!(
            "{{\"label\":\"Category\",\"value\":\"{}\"}}",
            escape_json(&strength.category)
        ));
        values.push(format!(
            "{{\"label\":\"Tie break\",\"value\":\"{}\"}}",
            escape_json(
                &strength
                    .tie_break_vector
                    .iter()
                    .map(u8::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            )
        ));
        values.push(format!(
            "{{\"label\":\"Best five\",\"value\":\"{}\"}}",
            escape_json(
                &strength
                    .best_five
                    .iter()
                    .map(|card| card.label.as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        ));
    }

    format!(
        "{{\"id\":\"{}\",\"label\":\"{}\",\"result\":\"{}\",\"emphasized\":{},\"strength\":{},\"values\":[{}]}}",
        breakdown.seat.as_str(),
        breakdown.seat.as_str(),
        escape_json(&breakdown.result),
        matches!(breakdown.result.as_str(), "win" | "split"),
        breakdown
            .strength
            .as_ref()
            .map_or_else(|| "null".to_owned(), river_showdown_strength_json),
        values.join(",")
    )
}

fn river_showdown_strength_json(
    strength: &river_ledger::visibility::ShowdownStrengthView,
) -> String {
    format!(
        "{{\"category\":\"{}\",\"tie_break_vector\":[{}],\"best_five\":[{}],\"category_ladder_position\":{},\"result_label\":\"{}\",\"hand_name\":\"{}\",\"rank_explanation\":\"{}\",\"comparison_note\":\"{}\",\"best_five_accessibility_label\":\"{}\"}}",
        escape_json(&strength.category),
        strength
            .tie_break_vector
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(","),
        strength
            .best_five
            .iter()
            .map(river_card_json)
            .collect::<Vec<_>>()
            .join(","),
        river_category_ladder_position_json(&strength.category_ladder_position),
        escape_json(&strength.result_label),
        escape_json(&strength.hand_name),
        escape_json(&strength.rank_explanation),
        escape_json(&strength.comparison_note),
        escape_json(&strength.best_five_accessibility_label)
    )
}

fn river_category_ladder_position_json(
    position: &river_ledger::state::CategoryLadderPosition,
) -> String {
    format!(
        "{{\"position\":{},\"total\":{},\"description\":\"{}\"}}",
        position.position,
        position.total,
        escape_json(&position.description)
    )
}

fn river_private_view_json(private_view: &river_ledger::PrivateView) -> String {
    match private_view {
        river_ledger::PrivateView::Observer => {
            "{\"status\":\"observer\",\"seat\":null,\"hole_cards\":[]}".to_owned()
        }
        river_ledger::PrivateView::Seat(view) => format!(
            "{{\"status\":\"seat\",\"seat\":\"{}\",\"hole_cards\":[{}]}}",
            escape_json(&view.seat.as_str()),
            view.hole_cards
                .iter()
                .map(river_card_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

fn river_ui_json(ui: &river_ledger::ui::UiMetadata) -> String {
    format!(
        "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"surface_label\":\"{}\",\"viewer_modes\":[{}],\"min_seats\":{},\"default_seats\":{},\"max_seats\":{},\"seat_labels\":[{}],\"seat_metadata_label\":\"{}\",\"action_hint_label\":\"{}\",\"outcome_explanation_label\":\"{}\",\"contribution_label\":\"{}\",\"board_label\":\"{}\",\"hidden_hole_label\":\"{}\",\"reduced_motion_note\":\"{}\",\"hand_rankings\":[{}]}}",
        escape_json(&ui.game_id),
        escape_json(&ui.display_name),
        escape_json(&ui.surface_label),
        ui.viewer_modes
            .iter()
            .map(|mode| format!("\"{}\"", escape_json(mode)))
            .collect::<Vec<_>>()
            .join(","),
        ui.min_seats,
        ui.default_seats,
        ui.max_seats,
        ui.seat_labels
            .iter()
            .map(river_seat_display_label_json)
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&ui.seat_metadata_label),
        escape_json(&ui.action_hint_label),
        escape_json(&ui.outcome_explanation_label),
        escape_json(&ui.contribution_label),
        escape_json(&ui.board_label),
        escape_json(&ui.hidden_hole_label),
        escape_json(&ui.reduced_motion_note),
        ui.hand_rankings
            .iter()
            .map(river_hand_ranking_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn river_seat_display_label_json(label: &river_ledger::ui::SeatDisplayLabel) -> String {
    format!(
        "{{\"seat\":\"{}\",\"label\":\"{}\"}}",
        escape_json(&label.seat),
        escape_json(&label.label)
    )
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

fn river_hand_ranking_json(row: &river_ledger::ui::HandRankingMetadata) -> String {
    format!(
        "{{\"category\":\"{}\",\"label\":\"{}\",\"definition\":\"{}\"}}",
        escape_json(&row.category),
        escape_json(&row.label),
        escape_json(&row.definition)
    )
}

fn river_bot_decision_public_explanation_json(
    explanation: &river_ledger::visibility::BotDecisionPublicExplanationView,
) -> String {
    format!(
        "{{\"seat\":\"{}\",\"seat_label\":\"{}\",\"action_label\":\"{}\",\"short_reason\":\"{}\",\"public_facts\":[{}],\"hidden_information_notice\":\"{}\"}}",
        escape_json(&explanation.seat.as_str()),
        escape_json(&explanation.seat_label),
        escape_json(&explanation.action_label),
        escape_json(&explanation.short_reason),
        explanation
            .public_facts
            .iter()
            .map(river_bot_decision_public_fact_json)
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&explanation.hidden_information_notice)
    )
}

fn river_bot_decision_public_fact_json(
    fact: &river_ledger::visibility::BotDecisionPublicFactView,
) -> String {
    format!(
        "{{\"label\":\"{}\",\"value\":\"{}\"}}",
        escape_json(&fact.label),
        escape_json(&fact.value)
    )
}

fn plain_view_json(view: &plain_tricks::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"phase\":\"{}\",\"active_seat\":{},\"round_index\":{},\"trick_index\":{},\"round_leader\":\"{}\",\"current_leader\":\"{}\",\"hand_counts\":{},\"current_trick\":{},\"trick_history\":[{}],\"round_trick_counts\":{},\"total_trick_counts\":{},\"terminal\":{},\"terminal_rationale\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.phase.as_str(),
        option_plain_seat_json(view.active_seat),
        view.round_index,
        view.trick_index,
        view.round_leader.as_str(),
        view.current_leader.as_str(),
        plain_counts_json(view.hand_counts.seat_0, view.hand_counts.seat_1),
        plain_current_trick_json(&view.current_trick),
        view.trick_history
            .iter()
            .map(plain_completed_trick_json)
            .collect::<Vec<_>>()
            .join(","),
        plain_counts_json(view.round_trick_counts.seat_0, view.round_trick_counts.seat_1),
        plain_counts_json(view.total_trick_counts.seat_0, view.total_trick_counts.seat_1),
        plain_terminal_json(&view.terminal),
        plain_terminal_rationale(&view.terminal)
            .map_or_else(|| "null".to_owned(), plain_outcome_rationale_json),
        view.freshness_token.0,
        plain_private_view_json(&view.private_view),
        plain_ui_json(&view.ui)
    )
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

fn plain_counts_json(seat_0: u8, seat_1: u8) -> String {
    format!("{{\"seat_0\":{},\"seat_1\":{}}}", seat_0, seat_1)
}

fn plain_current_trick_json(trick: &plain_tricks::CurrentTrickView) -> String {
    format!(
        "{{\"led_suit\":{},\"plays\":[{}]}}",
        option_string_json(trick.led_suit.as_deref()),
        trick
            .plays
            .iter()
            .map(plain_played_card_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn plain_completed_trick_json(trick: &plain_tricks::CompletedTrickView) -> String {
    format!(
        "{{\"round_index\":{},\"trick_index\":{},\"leader\":\"{}\",\"plays\":[{},{}],\"winner\":\"{}\",\"trick_counts_after\":{}}}",
        trick.round_index,
        trick.trick_index,
        trick.leader.as_str(),
        plain_played_card_json(&trick.plays[0]),
        plain_played_card_json(&trick.plays[1]),
        trick.winner.as_str(),
        plain_counts_json(
            trick.trick_counts_after.seat_0,
            trick.trick_counts_after.seat_1
        )
    )
}

fn plain_played_card_json(play: &plain_tricks::PlayedCardView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"card\":{}}}",
        play.seat.as_str(),
        plain_card_json(&play.card)
    )
}

fn plain_card_json(card: &plain_tricks::CardView) -> String {
    format!(
        "{{\"card_id\":\"{}\",\"suit\":\"{}\",\"rank\":\"{}\",\"rank_value\":{},\"label\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.card_id),
        escape_json(&card.suit),
        escape_json(&card.rank),
        card.rank_value,
        escape_json(&card.label),
        escape_json(&card.accessibility_label)
    )
}

fn plain_terminal_json(terminal: &plain_tricks::TerminalView) -> String {
    match terminal {
        plain_tricks::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"winner\":null,\"draw\":false}".to_owned()
        }
        plain_tricks::TerminalView::TrickWin { winner, totals, .. } => format!(
            "{{\"kind\":\"trick_win\",\"winner\":\"{}\",\"draw\":false,\"totals\":{}}}",
            winner.as_str(),
            plain_counts_json(totals.seat_0, totals.seat_1)
        ),
        plain_tricks::TerminalView::Split { each, totals, .. } => format!(
            "{{\"kind\":\"split\",\"winner\":null,\"draw\":true,\"each\":{},\"totals\":{}}}",
            each,
            plain_counts_json(totals.seat_0, totals.seat_1)
        ),
    }
}

fn plain_terminal_rationale(
    terminal: &plain_tricks::TerminalView,
) -> Option<&plain_tricks::OutcomeRationaleView> {
    match terminal {
        plain_tricks::TerminalView::NonTerminal => None,
        plain_tricks::TerminalView::TrickWin { rationale, .. }
        | plain_tricks::TerminalView::Split { rationale, .. } => Some(rationale),
    }
}

fn plain_outcome_rationale_json(rationale: &plain_tricks::OutcomeRationaleView) -> String {
    format!(
        "{{\"result_kind\":\"{}\",\"decisive_cause\":\"{}\",\"template_key\":\"{}\",\"decisive_rule_ids\":[{}],\"per_seat\":[{},{}]}}",
        escape_json(&rationale.result_kind),
        escape_json(&rationale.decisive_cause),
        escape_json(&rationale.template_key),
        string_array(&rationale.decisive_rule_ids),
        plain_outcome_breakdown_json(&rationale.per_seat[0]),
        plain_outcome_breakdown_json(&rationale.per_seat[1])
    )
}

fn plain_outcome_breakdown_json(breakdown: &plain_tricks::SeatOutcomeBreakdownView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"total_tricks\":{},\"result\":\"{}\"}}",
        breakdown.seat.as_str(),
        breakdown.total_tricks,
        escape_json(&breakdown.result)
    )
}

fn plain_private_view_json(private: &plain_tricks::PrivateView) -> String {
    match private {
        plain_tricks::PrivateView::Observer => {
            "{\"status\":\"observer\",\"own_hand\":[]}".to_owned()
        }
        plain_tricks::PrivateView::Seat(view) => format!(
            "{{\"status\":\"seat\",\"seat\":\"{}\",\"own_hand\":[{}]}}",
            view.seat.as_str(),
            view.own_hand
                .iter()
                .map(plain_card_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

fn plain_ui_json(ui: &plain_tricks::UiMetadata) -> String {
    format!(
        "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"table_label\":\"{}\",\"own_hand_label\":\"{}\",\"opponent_hand_label\":\"{}\",\"current_trick_label\":\"{}\",\"trick_history_label\":\"{}\",\"score_label\":\"{}\",\"play_action_label\":\"{}\",\"observer_disabled_reason\":\"{}\",\"reduced_motion_note\":\"{}\",\"rules_summary\":[{}]}}",
        escape_json(&ui.game_id),
        escape_json(&ui.display_name),
        escape_json(&ui.table_label),
        escape_json(&ui.own_hand_label),
        escape_json(&ui.opponent_hand_label),
        escape_json(&ui.current_trick_label),
        escape_json(&ui.trick_history_label),
        escape_json(&ui.score_label),
        escape_json(&ui.play_action_label),
        escape_json(&ui.observer_disabled_reason),
        escape_json(&ui.reduced_motion_note),
        string_array(&ui.rules_summary)
    )
}

fn poker_round_json(round: &poker_lite::visibility::RoundView) -> String {
    format!(
        "{{\"round_index\":{},\"round_unit\":{},\"outstanding_actor\":{},\"outstanding_amount\":{},\"lift_cap_remaining\":{}}}",
        round.round_index,
        round.round_unit,
        poker_optional_seat_json(round.outstanding_actor),
        round.outstanding_amount,
        round.lift_cap_remaining
    )
}

fn poker_center_json(center: &poker_lite::visibility::CenterView) -> String {
    match center {
        poker_lite::visibility::CenterView::Hidden { status } => {
            format!("{{\"status\":\"{}\",\"card\":null}}", escape_json(status))
        }
        poker_lite::visibility::CenterView::Revealed(card) => {
            format!(
                "{{\"status\":\"revealed\",\"card\":{}}}",
                poker_card_json(card)
            )
        }
    }
}

fn poker_showdown_json(showdown: &poker_lite::visibility::ShowdownView) -> String {
    format!(
        "{{\"seat_0_private\":{},\"seat_1_private\":{},\"center\":{},\"winner\":{}}}",
        poker_card_json(&showdown.seat_0_private),
        poker_card_json(&showdown.seat_1_private),
        poker_card_json(&showdown.center),
        poker_optional_seat_json(showdown.winner)
    )
}

fn poker_card_json(card: &poker_lite::visibility::CardView) -> String {
    format!(
        "{{\"card_id\":\"{}\",\"rank\":\"{}\",\"rank_value\":{},\"copy\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.card_id),
        escape_json(&card.rank),
        card.rank_value,
        escape_json(&card.copy),
        escape_json(&card.label),
        escape_json(&card.accessibility_label)
    )
}

fn poker_terminal_json(terminal: &poker_lite::visibility::TerminalView) -> String {
    match terminal {
        poker_lite::visibility::TerminalView::NonTerminal => {
            "{\"terminal\":false,\"kind\":\"non_terminal\",\"winner\":null,\"draw\":false}".to_owned()
        }
        poker_lite::visibility::TerminalView::YieldWin {
            winner,
            loser,
            shared_pool,
            ..
        } => format!(
            "{{\"terminal\":true,\"kind\":\"yield_win\",\"winner\":\"{}\",\"loser\":\"{}\",\"draw\":false,\"shared_pool\":{}}}",
            winner.as_str(),
            loser.as_str(),
            shared_pool
        ),
        poker_lite::visibility::TerminalView::ShowdownWin {
            winner,
            shared_pool,
            ..
        } => format!(
            "{{\"terminal\":true,\"kind\":\"showdown_win\",\"winner\":\"{}\",\"draw\":false,\"shared_pool\":{}}}",
            winner.as_str(),
            shared_pool
        ),
        poker_lite::visibility::TerminalView::Split {
            shared_pool, each, ..
        } => format!(
            "{{\"terminal\":true,\"kind\":\"split\",\"winner\":null,\"draw\":true,\"shared_pool\":{},\"each\":{}}}",
            shared_pool, each
        ),
    }
}

fn poker_terminal_rationale(
    terminal: &poker_lite::visibility::TerminalView,
) -> Option<&poker_lite::visibility::OutcomeRationaleView> {
    match terminal {
        poker_lite::visibility::TerminalView::NonTerminal => None,
        poker_lite::visibility::TerminalView::YieldWin { rationale, .. }
        | poker_lite::visibility::TerminalView::ShowdownWin { rationale, .. }
        | poker_lite::visibility::TerminalView::Split { rationale, .. } => Some(rationale),
    }
}

fn poker_rationale_json(rationale: &poker_lite::visibility::OutcomeRationaleView) -> String {
    let decisive_rule_ids = rationale
        .decisive_rule_ids
        .iter()
        .map(|rule_id| format!("\"{}\"", escape_json(rule_id)))
        .collect::<Vec<_>>()
        .join(",");
    let final_standing = rationale
        .per_seat
        .iter()
        .map(poker_rationale_standing_json)
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"result_kind\":\"{}\",\"decisive_cause\":\"{}\",\"template_key\":\"{}\",\"decisive_rule_ids\":[{}],\"final_standing\":[{}]}}",
        escape_json(&rationale.result_kind),
        escape_json(&rationale.decisive_cause),
        escape_json(&rationale.template_key),
        decisive_rule_ids,
        final_standing
    )
}

fn poker_rationale_standing_json(
    breakdown: &poker_lite::visibility::SeatOutcomeBreakdownView,
) -> String {
    let mut values = vec![
        format!(
            "{{\"label\":\"Contribution\",\"value\":{}}}",
            breakdown.contribution
        ),
        format!(
            "{{\"label\":\"Allocation\",\"value\":{}}}",
            breakdown.allocation
        ),
    ];
    if let Some(strength) = &breakdown.strength {
        values.push(format!(
            "{{\"label\":\"Pair\",\"value\":\"{}\"}}",
            escape_json(&strength.pair_bucket)
        ));
        values.push(format!(
            "{{\"label\":\"Private rank\",\"value\":\"{}\"}}",
            escape_json(&strength.private_rank)
        ));
    }

    format!(
        "{{\"id\":\"{}\",\"label\":\"{}\",\"result\":\"{}\",\"emphasized\":{},\"values\":[{}]}}",
        breakdown.seat.as_str(),
        breakdown.seat.as_str(),
        escape_json(&breakdown.result),
        breakdown.result == "win",
        values.join(",")
    )
}

fn poker_private_view_json(private_view: &poker_lite::visibility::PrivateView) -> String {
    match private_view {
        poker_lite::visibility::PrivateView::Observer => {
            "{\"status\":\"observer\",\"own_private\":null,\"own_strength_bucket\":null}".to_owned()
        }
        poker_lite::visibility::PrivateView::Seat(private) => format!(
            "{{\"status\":\"seat\",\"seat\":\"{}\",\"own_private\":{},\"own_strength_bucket\":{}}}",
            private.seat.as_str(),
            private
                .own_private
                .as_ref()
                .map_or_else(|| "null".to_owned(), poker_card_json),
            private.own_strength_bucket.as_ref().map_or_else(
                || "null".to_owned(),
                |bucket| format!("\"{}\"", escape_json(bucket))
            )
        ),
    }
}

fn poker_ui_json(ui: &poker_lite::UiMetadata) -> String {
    format!(
        "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"surface_label\":\"{}\",\"shared_pool_label\":\"{}\",\"hidden_center_label\":\"{}\",\"hidden_private_label\":\"{}\",\"hold_label\":\"{}\",\"press_label\":\"{}\",\"lift_label\":\"{}\",\"match_label\":\"{}\",\"yield_label\":\"{}\",\"reduced_motion_note\":\"{}\"}}",
        escape_json(&ui.game_id),
        escape_json(&ui.display_name),
        escape_json(&ui.surface_label),
        escape_json(&ui.shared_pool_label),
        escape_json(&ui.hidden_center_label),
        escape_json(&ui.hidden_private_label),
        escape_json(&ui.hold_label),
        escape_json(&ui.press_label),
        escape_json(&ui.lift_label),
        escape_json(&ui.match_label),
        escape_json(&ui.yield_label),
        escape_json(&ui.reduced_motion_note)
    )
}

fn poker_optional_seat_json(seat: Option<PokerLiteSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

fn poker_effects_json(effects: &[EffectEnvelope<PokerLiteEffect>], viewer: &Viewer) -> String {
    let body = poker_lite::visibility::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(poker_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn river_effects_json(effects: &[EffectEnvelope<RiverLedgerEffect>], viewer: &Viewer) -> String {
    let body = river_ledger::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(river_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn river_effect_json(effect: &EffectEnvelope<RiverLedgerEffect>) -> String {
    let payload = match &effect.payload {
        RiverLedgerEffect::PrivateCardsDealt { owner, cards } => format!(
            "{{\"type\":\"river_ledger_private_cards_dealt\",\"owner\":\"{}\",\"cards\":[{}]}}",
            escape_json(&river_effect_seat_label(*owner)),
            cards
                .iter()
                .map(|card| format!("\"{}\"", escape_json(&card.id())))
                .collect::<Vec<_>>()
                .join(",")
        ),
        RiverLedgerEffect::DealStarted {
            private_count_per_seat,
            reserved_community_count,
            deck_tail_count,
        } => format!(
            "{{\"type\":\"river_ledger_deal_started\",\"private_count_per_seat\":{},\"reserved_community_count\":{},\"deck_tail_count\":{}}}",
            private_count_per_seat, reserved_community_count, deck_tail_count
        ),
        RiverLedgerEffect::ContributionChanged {
            seat,
            amount_added,
            pot_total,
        } => format!(
            "{{\"type\":\"river_ledger_contribution_changed\",\"actor\":\"{}\",\"amount_added\":{},\"pot_total\":{}}}",
            escape_json(&river_effect_seat_label(*seat)),
            amount_added,
            pot_total
        ),
        RiverLedgerEffect::StreetAdvanced {
            street,
            public_board,
        } => format!(
            "{{\"type\":\"river_ledger_street_advanced\",\"street\":\"{}\",\"public_board_count\":{},\"public_board\":[{}]}}",
            street.as_str(),
            public_board.len(),
            public_board
                .iter()
                .map(|card| format!("\"{}\"", escape_json(&card.id())))
                .collect::<Vec<_>>()
                .join(",")
        ),
        RiverLedgerEffect::ShowdownResolved { outcome } => {
            let (kind, winner_count, pot_total) = match outcome {
                river_ledger::TerminalOutcome::LastLiveHand { pot_total, .. } => {
                    ("last_live_hand", 1usize, *pot_total)
                }
                river_ledger::TerminalOutcome::Showdown {
                    winners,
                    pot_total,
                    ..
                } => ("showdown", winners.len(), *pot_total),
            };
            format!(
                "{{\"type\":\"river_ledger_showdown_resolved\",\"kind\":\"{}\",\"winner_count\":{},\"pot_total\":{}}}",
                kind, winner_count, pot_total
            )
        }
    };
    format!("{{\"payload\":{payload}}}")
}

fn river_effect_seat_label(seat: RiverLedgerSeat) -> String {
    format!("Seat {}", seat.index())
}

fn plain_effects_json(effects: &[EffectEnvelope<PlainTricksEffect>], viewer: &Viewer) -> String {
    let body = effects
        .iter()
        .filter(|effect| effect_visible_to_viewer(&effect.visibility, viewer))
        .map(plain_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn plain_effect_json(effect: &EffectEnvelope<PlainTricksEffect>) -> String {
    let payload = match &effect.payload {
        PlainTricksEffect::DealStarted {
            round_index,
            cards_per_seat,
            tail_count,
        } => format!(
            "{{\"type\":\"deal_started\",\"round_index\":{},\"cards_per_seat\":{},\"tail_count\":{}}}",
            round_index, cards_per_seat, tail_count
        ),
        PlainTricksEffect::HandDealt { owner, cards } => format!(
            "{{\"type\":\"hand_dealt\",\"owner\":\"{}\",\"cards\":[{}]}}",
            owner.as_str(),
            cards
                .iter()
                .map(|card| format!("\"{}\"", card.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        PlainTricksEffect::DealCompleted {
            round_index,
            cards_per_seat,
            tail_count,
            leader,
        } => format!(
            "{{\"type\":\"deal_completed\",\"round_index\":{},\"cards_per_seat\":{},\"tail_count\":{},\"leader\":\"{}\"}}",
            round_index, cards_per_seat, tail_count, leader.as_str()
        ),
        PlainTricksEffect::CardPlayed {
            seat,
            card,
            round_index,
            trick_index,
            led,
        } => format!(
            "{{\"type\":\"card_played\",\"seat\":\"{}\",\"card\":\"{}\",\"round_index\":{},\"trick_index\":{},\"led\":{}}}",
            seat.as_str(),
            card.as_str(),
            round_index,
            trick_index,
            led
        ),
        PlainTricksEffect::TrickResolved {
            round_index,
            trick_index,
            plays,
            winner,
            trick_counts,
        } => format!(
            "{{\"type\":\"trick_resolved\",\"round_index\":{},\"trick_index\":{},\"plays\":[{},{}],\"winner\":\"{}\",\"trick_counts\":{}}}",
            round_index,
            trick_index,
            plain_trick_play_json(plays[0]),
            plain_trick_play_json(plays[1]),
            winner.as_str(),
            plain_counts_json(trick_counts.seat_0, trick_counts.seat_1)
        ),
        PlainTricksEffect::RoundScored {
            round_index,
            round_counts,
            total_counts,
        } => format!(
            "{{\"type\":\"round_scored\",\"round_index\":{},\"round_counts\":{},\"total_counts\":{}}}",
            round_index,
            plain_counts_json(round_counts.seat_0, round_counts.seat_1),
            plain_counts_json(total_counts.seat_0, total_counts.seat_1)
        ),
        PlainTricksEffect::DealRotated {
            round_index,
            leader,
        } => format!(
            "{{\"type\":\"deal_rotated\",\"round_index\":{},\"leader\":\"{}\"}}",
            round_index,
            leader.as_str()
        ),
        PlainTricksEffect::MatchResolved {
            totals,
            decisive_cause,
        } => format!(
            "{{\"type\":\"match_resolved\",\"totals\":{},\"decisive_cause\":\"{}\"}}",
            plain_counts_json(totals.seat_0, totals.seat_1),
            escape_json(decisive_cause)
        ),
        PlainTricksEffect::Terminal {
            outcome,
            decisive_cause,
        } => format!(
            "{{\"type\":\"terminal\",\"outcome\":{},\"decisive_cause\":\"{}\"}}",
            plain_terminal_outcome_json(*outcome),
            escape_json(decisive_cause)
        ),
        PlainTricksEffect::BotChoseActionPublic {
            policy_id,
            action_family,
        } => format!(
            "{{\"type\":\"bot_chose_action_public\",\"policy_id\":\"{}\",\"action_family\":\"{}\"}}",
            escape_json(policy_id),
            escape_json(action_family)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn plain_trick_play_json(play: plain_tricks::TrickPlay) -> String {
    format!(
        "{{\"seat\":\"{}\",\"card\":\"{}\"}}",
        play.seat.as_str(),
        play.card.as_str()
    )
}

fn plain_terminal_outcome_json(outcome: plain_tricks::TerminalOutcome) -> String {
    match outcome {
        plain_tricks::TerminalOutcome::TrickWin { winner, totals } => format!(
            "{{\"kind\":\"trick_win\",\"winner\":\"{}\",\"totals\":{}}}",
            winner.as_str(),
            plain_counts_json(totals.seat_0, totals.seat_1)
        ),
        plain_tricks::TerminalOutcome::Split { each, totals } => format!(
            "{{\"kind\":\"split\",\"winner\":null,\"each\":{},\"totals\":{}}}",
            each,
            plain_counts_json(totals.seat_0, totals.seat_1)
        ),
    }
}

fn poker_effect_json(effect: &EffectEnvelope<PokerLiteEffect>) -> String {
    let payload = match &effect.payload {
        PokerLiteEffect::PrivateCrestDealt { owner, card } => format!(
            "{{\"type\":\"private_crest_dealt\",\"owner\":\"{}\",\"card\":{}}}",
            owner.as_str(),
            poker_raw_card_json(*card)
        ),
        PokerLiteEffect::CrestDealStarted {
            private_count_per_seat,
            center_count,
            deck_tail_count,
        } => format!(
            "{{\"type\":\"crest_deal_started\",\"private_count_per_seat\":{},\"center_count\":{},\"deck_tail_count\":{}}}",
            private_count_per_seat, center_count, deck_tail_count
        ),
        PokerLiteEffect::OpeningPoolSet {
            contributions,
            shared_pool,
        } => format!(
            "{{\"type\":\"opening_pool_set\",\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}},\"shared_pool\":{}}}",
            contributions[0], contributions[1], shared_pool
        ),
        PokerLiteEffect::PledgeHeld { actor, round_index } => format!(
            "{{\"type\":\"pledge_held\",\"actor\":\"{}\",\"round_index\":{}}}",
            actor.as_str(),
            round_index
        ),
        PokerLiteEffect::PledgePressed {
            actor,
            round_index,
            amount,
            shared_pool_after,
        } => format!(
            "{{\"type\":\"pledge_pressed\",\"actor\":\"{}\",\"round_index\":{},\"amount\":{},\"shared_pool_after\":{}}}",
            actor.as_str(),
            round_index,
            amount,
            shared_pool_after
        ),
        PokerLiteEffect::PledgeLifted {
            actor,
            round_index,
            amount,
            shared_pool_after,
            lift_cap_consumed,
        } => format!(
            "{{\"type\":\"pledge_lifted\",\"actor\":\"{}\",\"round_index\":{},\"amount\":{},\"shared_pool_after\":{},\"lift_cap_consumed\":{}}}",
            actor.as_str(),
            round_index,
            amount,
            shared_pool_after,
            lift_cap_consumed
        ),
        PokerLiteEffect::PledgeMatched {
            actor,
            round_index,
            amount,
            shared_pool_after,
        } => format!(
            "{{\"type\":\"pledge_matched\",\"actor\":\"{}\",\"round_index\":{},\"amount\":{},\"shared_pool_after\":{}}}",
            actor.as_str(),
            round_index,
            amount,
            shared_pool_after
        ),
        PokerLiteEffect::SeatYielded {
            actor,
            winner,
            shared_pool,
        } => format!(
            "{{\"type\":\"seat_yielded\",\"actor\":\"{}\",\"winner\":\"{}\",\"shared_pool\":{}}}",
            actor.as_str(),
            winner.as_str(),
            shared_pool
        ),
        PokerLiteEffect::CenterRevealStarted { group_id } => format!(
            "{{\"type\":\"center_reveal_started\",\"group_id\":\"{}\"}}",
            escape_json(group_id)
        ),
        PokerLiteEffect::CenterRevealed { group_id, center } => format!(
            "{{\"type\":\"center_revealed\",\"group_id\":\"{}\",\"center\":{}}}",
            escape_json(group_id),
            poker_raw_card_json(*center)
        ),
        PokerLiteEffect::ShowdownRevealStarted { group_id } => format!(
            "{{\"type\":\"showdown_reveal_started\",\"group_id\":\"{}\"}}",
            escape_json(group_id)
        ),
        PokerLiteEffect::ShowdownRevealed { group_id, reveal } => format!(
            "{{\"type\":\"showdown_revealed\",\"group_id\":\"{}\",\"reveal\":{}}}",
            escape_json(group_id),
            poker_reveal_json(*reveal)
        ),
        PokerLiteEffect::LedgerResolved {
            shared_pool,
            contributions,
            allocation,
        } => format!(
            "{{\"type\":\"ledger_resolved\",\"shared_pool\":{},\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}},\"allocation\":{}}}",
            shared_pool,
            contributions[0],
            contributions[1],
            poker_allocation_json(*allocation)
        ),
        PokerLiteEffect::Terminal { outcome } => format!(
            "{{\"type\":\"terminal\",\"outcome\":{}}}",
            poker_terminal_outcome_json(*outcome)
        ),
        PokerLiteEffect::BotChoseActionPublic {
            policy_id,
            action_family,
        } => format!(
            "{{\"type\":\"bot_chose_action\",\"policy_id\":\"{}\",\"action_family\":\"{}\"}}",
            escape_json(policy_id),
            escape_json(action_family)
        ),
        PokerLiteEffect::BotChoseActionPrivate {
            owner,
            policy_id,
            action_family,
            strength_bucket,
        } => format!(
            "{{\"type\":\"bot_chose_action_private\",\"owner\":\"{}\",\"policy_id\":\"{}\",\"action_family\":\"{}\",\"strength_bucket\":\"{}\"}}",
            owner.as_str(),
            escape_json(policy_id),
            escape_json(action_family),
            escape_json(strength_bucket)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn poker_raw_card_json(card: poker_lite::CrestCardId) -> String {
    format!(
        "{{\"card_id\":\"{}\",\"rank\":\"{}\",\"rank_value\":{},\"copy\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\"}}",
        card.as_str(),
        card.rank().as_str(),
        card.rank().value(),
        card.rank_copy().as_str(),
        escape_json(&card.label()),
        escape_json(&poker_lite::card_accessibility_label(card))
    )
}

fn poker_reveal_json(reveal: poker_lite::ShowdownReveal) -> String {
    format!(
        "{{\"seat_0_private\":{},\"seat_1_private\":{},\"center\":{}}}",
        poker_raw_card_json(reveal.seat_0_private),
        poker_raw_card_json(reveal.seat_1_private),
        poker_raw_card_json(reveal.center)
    )
}

fn poker_allocation_json(allocation: poker_lite::effects::LedgerAllocation) -> String {
    match allocation {
        poker_lite::effects::LedgerAllocation::Winner { seat, amount } => format!(
            "{{\"kind\":\"winner\",\"seat\":\"{}\",\"amount\":{}}}",
            seat.as_str(),
            amount
        ),
        poker_lite::effects::LedgerAllocation::Split { each } => {
            format!("{{\"kind\":\"split\",\"each\":{each}}}")
        }
    }
}

fn poker_terminal_outcome_json(outcome: poker_lite::TerminalOutcome) -> String {
    match outcome {
        poker_lite::TerminalOutcome::YieldWin {
            winner,
            loser,
            shared_pool,
            contributions,
        } => format!(
            "{{\"kind\":\"yield_win\",\"winner\":\"{}\",\"loser\":\"{}\",\"shared_pool\":{},\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            winner.as_str(),
            loser.as_str(),
            shared_pool,
            contributions[0],
            contributions[1]
        ),
        poker_lite::TerminalOutcome::ShowdownWin {
            winner,
            shared_pool,
            contributions,
            reveal,
        } => format!(
            "{{\"kind\":\"showdown_win\",\"winner\":\"{}\",\"shared_pool\":{},\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}},\"reveal\":{}}}",
            winner.as_str(),
            shared_pool,
            contributions[0],
            contributions[1],
            poker_reveal_json(reveal)
        ),
        poker_lite::TerminalOutcome::Split {
            shared_pool,
            each,
            contributions,
            reveal,
        } => format!(
            "{{\"kind\":\"split\",\"shared_pool\":{},\"each\":{},\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}},\"reveal\":{}}}",
            shared_pool,
            each,
            contributions[0],
            contributions[1],
            poker_reveal_json(reveal)
        ),
    }
}

fn option_plain_seat_json(seat: Option<PlainTricksSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
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
/// `game_ptr..game_ptr + game_len` must be a valid UTF-8 buffer for the duration
/// of the call.
pub unsafe extern "C" fn rulepath_new_match_with_seat_count(
    game_ptr: *const u8,
    game_len: usize,
    seed: u64,
    seat_count: usize,
) -> i32 {
    let game_id = unsafe { read_string(game_ptr, game_len) };
    write_result(game_id.and_then(|game_id| new_match_with_seat_count(&game_id, seed, seat_count)))
}

#[no_mangle]
/// # Safety
///
/// `game_ptr..game_ptr + game_len` and
/// `variant_ptr..variant_ptr + variant_len` must be valid UTF-8 buffers for
/// the duration of the call.
pub unsafe extern "C" fn rulepath_new_match_with_variant(
    game_ptr: *const u8,
    game_len: usize,
    variant_ptr: *const u8,
    variant_len: usize,
    seed: u64,
) -> i32 {
    let game_id = unsafe { read_string(game_ptr, game_len) };
    let variant_id = unsafe { read_string(variant_ptr, variant_len) };
    write_result(game_id.and_then(|game_id| {
        variant_id.and_then(|variant_id| new_match_for_variant(&game_id, Some(&variant_id), seed))
    }))
}

#[no_mangle]
/// # Safety
///
/// `game_ptr..game_ptr + game_len` and
/// `variant_ptr..variant_ptr + variant_len` must be valid UTF-8 buffers for
/// the duration of the call.
pub unsafe extern "C" fn rulepath_new_match_with_variant_and_seat_count(
    game_ptr: *const u8,
    game_len: usize,
    variant_ptr: *const u8,
    variant_len: usize,
    seed: u64,
    seat_count: usize,
) -> i32 {
    let game_id = unsafe { read_string(game_ptr, game_len) };
    let variant_id = unsafe { read_string(variant_ptr, variant_len) };
    write_result(game_id.and_then(|game_id| {
        variant_id.and_then(|variant_id| {
            new_match_for_variant_with_seat_count(&game_id, Some(&variant_id), seed, seat_count)
        })
    }))
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
    use std::collections::BTreeMap;

    #[derive(Clone, Debug)]
    struct NoLeakSurface {
        viewer: Option<String>,
        name: &'static str,
        payload: String,
    }

    #[derive(Clone, Debug)]
    struct PairwiseNoLeakCase {
        seats: Vec<String>,
        private_terms_by_seat: BTreeMap<String, Vec<String>>,
        surfaces: Vec<NoLeakSurface>,
    }

    impl PairwiseNoLeakCase {
        fn deterministic_summary(&self) -> String {
            let mut output = String::new();
            output.push_str(&self.seats.join("|"));
            for (seat, terms) in &self.private_terms_by_seat {
                output.push_str(seat);
                output.push(':');
                output.push_str(&terms.join(","));
                output.push(';');
            }
            for surface in &self.surfaces {
                output.push_str(surface.name);
                output.push('@');
                output.push_str(surface.viewer.as_deref().unwrap_or("observer"));
                output.push('=');
                output.push_str(&surface.payload);
                output.push(';');
            }
            output
        }
    }

    fn assert_pairwise_no_leak(case: &PairwiseNoLeakCase) {
        if let Err(message) = pairwise_no_leak_result(case) {
            panic!("{message}");
        }
    }

    fn pairwise_no_leak_result(case: &PairwiseNoLeakCase) -> Result<(), String> {
        for source_seat in &case.seats {
            let Some(private_terms) = case.private_terms_by_seat.get(source_seat) else {
                return Err(format!("missing private terms for {source_seat}"));
            };
            if private_terms.is_empty() {
                return Err(format!("no private terms registered for {source_seat}"));
            }
            for surface in &case.surfaces {
                if surface.viewer.as_deref() == Some(source_seat.as_str()) {
                    continue;
                }
                for term in private_terms {
                    if surface.payload.contains(term) {
                        return Err(format!(
                            "private term {term} for {source_seat} leaked to {} via {}",
                            surface.viewer.as_deref().unwrap_or("observer"),
                            surface.name
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn synthetic_n_seat_no_leak_case(seat_count: usize) -> PairwiseNoLeakCase {
        let seats = (0..seat_count)
            .map(|index| format!("seat_{index}"))
            .collect::<Vec<_>>();
        let mut private_terms_by_seat = BTreeMap::new();
        for seat in &seats {
            private_terms_by_seat.insert(seat.clone(), vec![format!("private::{seat}::seed-1701")]);
        }

        let mut surfaces = vec![NoLeakSurface {
            viewer: None,
            name: "replay_export",
            payload: format!(
                "viewer=observer;seat_count={seat_count};redacted=true;dom_test_id=seat-frame"
            ),
        }];
        for viewer in &seats {
            let own = private_terms_by_seat
                .get(viewer)
                .and_then(|terms| terms.first())
                .expect("synthetic private term");
            for name in [
                "payload",
                "action_tree",
                "preview",
                "effect_log",
                "bot_explanation",
                "candidate_ranking",
                "dom_test_id",
                "storage",
                "log",
            ] {
                surfaces.push(NoLeakSurface {
                    viewer: Some(viewer.clone()),
                    name,
                    payload: format!(
                        "viewer={viewer};seat_count={seat_count};own_private={own};other_private=redacted"
                    ),
                });
            }
        }

        PairwiseNoLeakCase {
            seats,
            private_terms_by_seat,
            surfaces,
        }
    }

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
        assert!(games.contains("\"game_id\":\"masked_claims\""));
        assert!(games.contains("\"game_id\":\"flood_watch\""));
        assert!(games.contains("\"game_id\":\"token_bazaar\""));
        assert!(games.contains("\"game_id\":\"poker_lite\""));
        assert!(games.contains("\"game_id\":\"plain_tricks\""));
        assert!(games.contains("\"min_seats\":2"));
        assert!(games.contains("\"max_seats\":2"));
        assert!(games.contains("\"default_seats\":2"));
        assert!(games.contains("\"supported_seats\":[2]"));
        assert!(games.contains(
            "\"seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Seat 0\"},{\"seat\":\"seat_1\",\"label\":\"Seat 1\"}]"
        ));
        assert!(games.contains(
            "\"seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Charter\"},{\"seat\":\"seat_1\",\"label\":\"Freeholders\"}]"
        ));
        assert!(games.contains("\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"]"));
        assert!(games.contains(
            "\"variants\":[{\"id\":\"three_marks_standard\",\"label\":\"Three Marks\"}]"
        ));
        assert!(games.contains(
            "\"variants\":[{\"id\":\"column_four_standard\",\"label\":\"Column Four\"}]"
        ));
        assert!(games.contains(
            "\"variants\":[{\"id\":\"directional_flip_standard\",\"label\":\"Directional Flip\"}]"
        ));
        assert!(games.contains(
            "\"variants\":[{\"id\":\"draughts_lite_standard\",\"label\":\"Draughts Lite\"}]"
        ));
        assert!(games.contains(
            "\"variants\":[{\"id\":\"high_card_duel_standard\",\"label\":\"High Card Duel\"}]"
        ));
        assert!(games.contains(
            "\"variants\":[{\"id\":\"masked_claims_standard\",\"label\":\"Masked Claims\"}]"
        ));
        assert!(games.contains("\"id\":\"flood_watch_deluge\""));
        assert!(games.contains("\"label\":\"Flood Watch: Deluge\""));
        assert!(games.contains("\"description\":\"Higher water starts and heavier surges create a tighter shared rescue.\""));
        assert!(games.contains("\"id\":\"frontier_control_highlands\""));
        assert!(games.contains("\"label\":\"Frontier Control: Highlands\""));
        assert!(games.contains("\"description\":\"Highlands shifts table pressure toward quarry routes and high ground.\""));
        assert!(games.contains("\"id\":\"event_frontier_hard_winter\""));
        assert!(games.contains("\"label\":\"Event Frontier: Hard Winter\""));
        assert!(games.contains("\"description\":\"Leaner opening resources make recovery feel tighter from the first turn.\""));
        assert!(games.contains("\"id\":\"event_frontier_land_rush\""));
        assert!(games.contains("\"label\":\"Event Frontier: Land Rush\""));
        assert!(games.contains(
            "\"description\":\"Broader opening reach creates a faster public buildup.\""
        ));
        assert!(games.contains(
            "\"variants\":[{\"id\":\"token_bazaar_standard\",\"label\":\"Token Bazaar\"}]"
        ));
        assert!(games.contains(
            "\"variants\":[{\"id\":\"poker_lite_standard\",\"label\":\"Crest Ledger\"}]"
        ));
        assert!(games.contains(
            "\"variants\":[{\"id\":\"plain_tricks_standard\",\"label\":\"Plain Tricks\"}]"
        ));
        assert!(games.contains("\"hidden_information\":true"));
        assert!(games.contains("\"public_replay_export\""));
        assert!(games.contains("\"reaction_window\""));
        assert!(games.contains("\"cooperative\":true"));
        assert!(games.contains("\"environment_automation\""));
        assert!(games.contains("\"bounded_pledge\""));
        assert!(games.contains("\"trick_taking\""));
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
    fn new_match_for_variant_starts_multi_variant_games() {
        let event_created =
            new_match_for_variant(GAME_EVENT_FRONTIER, Some("event_frontier_hard_winter"), 7)
                .expect("event frontier variant match created");
        let event_match_id = extract_match_id(&event_created);
        assert!(event_created.contains("\"variant_id\":\"event_frontier_hard_winter\""));
        let event_view = get_view(&event_match_id, Some("seat_0")).expect("event variant view");
        assert!(event_view.contains("\"variant_id\":\"event_frontier_hard_winter\""));

        let flood_created = new_match_for_variant(GAME_FLOOD_WATCH, Some("flood_watch_deluge"), 7)
            .expect("flood watch variant match created");
        assert!(flood_created.contains("\"variant_id\":\"flood_watch_deluge\""));

        let frontier_created =
            new_match_for_variant(GAME_FRONTIER_CONTROL, Some("frontier_control_highlands"), 7)
                .expect("frontier control variant match created");
        assert!(frontier_created.contains("\"variant_id\":\"frontier_control_highlands\""));
    }

    #[test]
    fn bridge_seat_builder_uses_deterministic_labels() {
        let seats = seats_for_count(3);
        assert_eq!(
            seats,
            vec![
                SeatId("seat-0".to_owned()),
                SeatId("seat-1".to_owned()),
                SeatId("seat-2".to_owned())
            ]
        );

        let underscore = masked_seats_for_count(3);
        assert_eq!(
            underscore,
            vec![
                SeatId("seat_0".to_owned()),
                SeatId("seat_1".to_owned()),
                SeatId("seat_2".to_owned())
            ]
        );
    }

    #[test]
    fn new_match_with_seat_count_surfaces_game_setup_diagnostic() {
        let created =
            new_match_with_seat_count(GAME_RACE_TO_N, 11, 2).expect("two-seat setup succeeds");
        assert!(created.contains("\"game_id\":\"race_to_n\""));

        let diagnostic = new_match_with_seat_count(GAME_RACE_TO_N, 11, 3)
            .expect_err("three-seat setup is rejected by the game");
        assert!(diagnostic.contains("\"code\":\"invalid_seat_count\""));
        assert!(diagnostic.contains("race_to_n requires exactly two seats"));
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
    fn pairwise_no_leak_harness_covers_high_card_and_synthetic_n_seats() {
        let high_card = high_card_pairwise_no_leak_case();
        assert_pairwise_no_leak(&high_card);

        let synthetic = synthetic_n_seat_no_leak_case(4);
        assert_pairwise_no_leak(&synthetic);
        assert_eq!(
            synthetic.deterministic_summary(),
            synthetic_n_seat_no_leak_case(4).deterministic_summary()
        );
    }

    #[test]
    fn hidden_info_bridge_games_invoke_pairwise_no_leak_harness() {
        for case in [
            high_card_pairwise_no_leak_case(),
            poker_lite_pairwise_no_leak_case(),
            plain_tricks_pairwise_no_leak_case(),
            masked_claims_pairwise_no_leak_case(),
            river_ledger_pairwise_no_leak_case(),
        ] {
            assert_pairwise_no_leak(&case);
        }
    }

    #[test]
    fn pairwise_no_leak_harness_negative_fixture_fails() {
        let mut synthetic = synthetic_n_seat_no_leak_case(4);
        let leaked = synthetic
            .private_terms_by_seat
            .get("seat_0")
            .and_then(|terms| terms.first())
            .expect("seat_0 private term")
            .clone();
        synthetic.surfaces.push(NoLeakSurface {
            viewer: Some("seat_2".to_owned()),
            name: "negative_induced_leak",
            payload: format!("viewer=seat_2;leaked={leaked}"),
        });

        let message = pairwise_no_leak_result(&synthetic).expect_err("induced leak is caught");
        assert!(message.contains("seat_0"));
        assert!(message.contains("seat_2"));
        assert!(message.contains("negative_induced_leak"));
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
    fn secret_draft_surface_filters_hidden_commitments() {
        let created = new_match("secret_draft", 91).expect("match created");
        let match_id = extract_match_id(&created);
        assert!(created.contains("\"variant_id\":\"secret_draft_standard\""));

        let observer = get_view(&match_id, None).expect("observer view returned");
        let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat view returned");
        assert!(observer.contains("\"game_id\":\"secret_draft\""));
        assert!(observer.contains("\"private_view\":{\"status\":\"observer\""));
        assert!(seat_0.contains("\"private_view\":{\"status\":\"seat\",\"seat\":\"seat_0\""));
        assert!(!seat_0.contains("own_commitment"));

        let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
            .expect("authorized tree returned");
        let unauthorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_1"))
            .expect("unauthorized tree returned");
        let observer_tree =
            get_action_tree_for_viewer(&match_id, "seat_0", None).expect("observer tree returned");
        assert!(authorized.contains("\"segment\":\"commit/"));
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
        assert!(applied.contains("\"type\":\"own_commit_accepted\""));
        assert!(applied.contains("\"own_committed\":true"));
        assert!(!applied.contains("\"item_id\":\"commit/"));

        let observer_effects = get_effects(&match_id, 0, None).expect("observer effects");
        assert!(observer_effects.contains("\"type\":\"commitment_placed\""));
        assert!(observer_effects.contains("\"type\":\"own_commit_accepted\""));
        assert!(!observer_effects.contains(&action_segment));

        let seat_0_effects = get_effects(&match_id, 0, Some("seat_0")).expect("seat effects");
        assert!(seat_0_effects.contains("\"type\":\"own_commit_accepted\""));
        assert!(!seat_0_effects.contains(&action_segment));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        assert!(bot.contains("\"ok\":true"));
        assert!(bot.contains("\"type\":\"choices_revealed\""));
        assert!(!bot.contains("candidate"));
        assert!(!bot.contains("debug"));

        let exported = export_replay(&match_id).expect("public replay exported");
        assert!(exported.contains("\"export_class\":\"viewer_scoped_observation_v1\""));
        assert!(exported.contains("\"viewer\":\"observer\""));
        assert!(!exported.contains("\"commands\""));
        assert!(!exported.contains("\"path\""));
        assert!(!exported.contains("\"seed_evidence\""));

        let imported = import_replay(&exported).expect("public replay imported");
        let replay_id = extract_replay_id(&imported);
        assert!(imported.contains("\"public_export\":true"));
        assert!(imported.contains("\"game_id\":\"secret_draft\""));

        let reset = replay_reset(&replay_id).expect("public replay reset returned");
        assert!(reset.contains("\"public_export\":true"));
        assert!(reset.contains("\"view\":null"));
    }

    #[test]
    fn poker_lite_surface_filters_hidden_cards() {
        let created = new_match("poker_lite", 101).expect("match created");
        let match_id = extract_match_id(&created);
        assert!(created.contains("\"variant_id\":\"poker_lite_standard\""));

        let internal = poker_setup_match(Seed(101), &seats(), &poker_lite::SetupOptions::default())
            .expect("setup succeeds");
        let seat_0_card = internal.private_card_for_internal(PokerLiteSeat::Seat0);
        let seat_1_card = internal.private_card_for_internal(PokerLiteSeat::Seat1);
        let center_card = internal.center_card_internal();

        let observer = get_view(&match_id, None).expect("observer view returned");
        let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat view returned");
        let seat_1 = get_view(&match_id, Some("seat_1")).expect("seat view returned");
        assert!(observer.contains("\"game_id\":\"poker_lite\""));
        assert!(observer.contains("\"private_view\":{\"status\":\"observer\""));
        assert!(seat_0.contains("\"private_view\":{\"status\":\"seat\",\"seat\":\"seat_0\""));
        assert!(seat_0.contains(seat_0_card.as_str()));
        assert!(seat_1.contains(seat_1_card.as_str()));
        assert!(!observer.contains("\"rank\":"));
        assert_no_poker_cards(&observer, &[seat_0_card, seat_1_card, center_card]);
        assert!(!seat_1.contains(seat_0_card.as_str()));
        assert!(!seat_0.contains(seat_1_card.as_str()));
        assert!(!seat_0.contains(center_card.as_str()));
        assert!(!seat_1.contains(center_card.as_str()));
        if seat_0_card.rank() != seat_1_card.rank() {
            assert!(!seat_1.contains(&format!("\"rank\":\"{}\"", seat_0_card.rank().as_str())));
            assert!(!seat_0.contains(&format!("\"rank\":\"{}\"", seat_1_card.rank().as_str())));
        }
        if center_card.rank() != seat_0_card.rank() {
            assert!(!seat_0.contains(&format!("\"rank\":\"{}\"", center_card.rank().as_str())));
        }
        if center_card.rank() != seat_1_card.rank() {
            assert!(!seat_1.contains(&format!("\"rank\":\"{}\"", center_card.rank().as_str())));
        }

        let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
            .expect("authorized tree returned");
        let unauthorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_1"))
            .expect("unauthorized tree returned");
        let observer_tree =
            get_action_tree_for_viewer(&match_id, "seat_0", None).expect("observer tree returned");
        assert!(authorized.contains("\"segment\":\"hold\""));
        assert!(authorized.contains("\"segment\":\"press\""));
        assert!(unauthorized.contains("\"choices\":[]"));
        assert!(observer_tree.contains("\"choices\":[]"));

        let applied = apply_action(&match_id, "seat_0", "press", 0).expect("press applies");
        assert!(applied.contains("\"type\":\"pledge_pressed\""));
        assert!(!applied.contains(seat_1_card.as_str()));
        assert!(!applied.contains(center_card.as_str()));

        let observer_effects = get_effects(&match_id, 0, None).expect("observer effects");
        assert!(observer_effects.contains("\"type\":\"pledge_pressed\""));
        assert_no_poker_cards(&observer_effects, &[seat_0_card, seat_1_card, center_card]);

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        assert!(bot.contains("\"ok\":true"));
        assert!(bot.contains("\"policy_id\":\"poker-lite-crest-ledger-level2-v1\""));
        assert!(!bot.contains(seat_0_card.as_str()));

        let seat_1_effects = get_effects(&match_id, 0, Some("seat_1")).expect("seat effects");
        assert!(seat_1_effects.contains("\"type\":\"bot_chose_action_private\""));
        assert!(seat_1_effects.contains("\"strength_bucket\""));
        assert!(!seat_1_effects.contains(seat_0_card.as_str()));

        let exported = export_replay(&match_id).expect("public replay exported");
        assert!(exported.contains("\"export_class\":\"viewer_scoped_observation_v1\""));
        assert!(exported.contains("\"viewer\":\"observer\""));
        assert!(!exported.contains("\"commands\""));
        assert!(!exported.contains("\"path\""));
        assert!(!exported.contains("\"seed_evidence\""));
        assert_no_poker_cards(&exported, &[seat_0_card, seat_1_card]);

        let imported = import_replay(&exported).expect("public replay imported");
        let replay_id = extract_replay_id(&imported);
        assert!(imported.contains("\"public_export\":true"));
        assert!(imported.contains("\"game_id\":\"poker_lite\""));

        let reset = replay_reset(&replay_id).expect("public replay reset returned");
        assert!(reset.contains("\"public_export\":true"));
        assert!(reset.contains("\"view\":null"));
    }

    #[test]
    fn plain_tricks_surface_filters_hidden_cards_and_authorizes_actor() {
        let seed = 101;
        let created = new_match("plain_tricks", seed).expect("match created");
        let match_id = extract_match_id(&created);
        assert!(created.contains("\"variant_id\":\"plain_tricks_standard\""));

        let internal = plain_setup_match(
            Seed(seed),
            &plain_seats(),
            &plain_tricks::SetupOptions::default(),
        )
        .expect("setup succeeds");
        let seat_0_view = plain_project_view(
            &internal,
            &Viewer {
                seat_id: Some(SeatId("seat_0".to_owned())),
            },
        );
        let seat_1_view = plain_project_view(
            &internal,
            &Viewer {
                seat_id: Some(SeatId("seat_1".to_owned())),
            },
        );
        let seat_0_cards = plain_private_cards(&seat_0_view);
        let seat_1_cards = plain_private_cards(&seat_1_view);
        let hidden_cards = plain_hidden_cards_except(&[]);
        let seat_0_private = plain_cards_except(&seat_0_cards, &[]);
        let seat_1_private = plain_cards_except(&seat_1_cards, &[]);

        let observer = get_view(&match_id, None).expect("observer view returned");
        let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat_0 view returned");
        let seat_1 = get_view(&match_id, Some("seat_1")).expect("seat_1 view returned");
        assert!(observer.contains("\"game_id\":\"plain_tricks\""));
        assert!(observer.contains("\"private_view\":{\"status\":\"observer\""));
        assert!(seat_0.contains("\"private_view\":{\"status\":\"seat\",\"seat\":\"seat_0\""));
        assert!(seat_1.contains("\"private_view\":{\"status\":\"seat\",\"seat\":\"seat_1\""));
        assert_no_plain_cards(&observer, &hidden_cards);
        for card in &seat_0_cards {
            assert!(seat_0.contains(card.as_str()));
        }
        for card in &seat_1_cards {
            assert!(seat_1.contains(card.as_str()));
        }
        assert_no_plain_cards(&seat_0, &seat_1_private);
        assert_no_plain_cards(&seat_1, &seat_0_private);

        let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
            .expect("authorized tree returned");
        let unauthorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_1"))
            .expect("unauthorized tree returned");
        let observer_tree =
            get_action_tree_for_viewer(&match_id, "seat_0", None).expect("observer tree returned");
        let first_card = seat_0_cards[0];
        assert!(authorized.contains("\"segment\":\"play\""));
        assert!(authorized.contains(first_card.as_str()));
        assert!(unauthorized.contains("\"choices\":[]"));
        assert!(observer_tree.contains("\"choices\":[]"));

        let applied = apply_action(
            &match_id,
            "seat_0",
            &format!("play>{}", first_card.as_str()),
            0,
        )
        .expect("plain trick card applies");
        assert!(applied.contains("\"type\":\"card_played\""));
        assert!(applied.contains(first_card.as_str()));
        assert_no_plain_cards(&applied, &seat_1_private);

        let observer_effects = get_effects(&match_id, 0, None).expect("observer effects");
        assert!(observer_effects.contains("\"type\":\"card_played\""));
        assert_no_plain_cards(&observer_effects, &plain_hidden_cards_except(&[first_card]));
    }

    #[test]
    fn masked_claims_bridge_filters_unrevealed_masks_and_exports_redacted_claims() {
        let created = new_match("masked_claims", 41).expect("match created");
        let match_id = extract_match_id(&created);
        assert!(created.contains("\"variant_id\":\"masked_claims_standard\""));

        let observer = get_view(&match_id, None).expect("observer view returned");
        assert!(observer.contains("\"game_id\":\"masked_claims\""));
        assert!(observer.contains("\"status\":\"observer\""));
        assert!(
            !observer.contains("mask_g"),
            "observer view leaked hidden mask id: {observer}"
        );

        let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat view returned");
        let first_mask = first_mask_segment(&seat_0);
        assert!(seat_0.contains(&first_mask));

        let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
        assert!(tree.contains("\"segment\":\"claim\""));
        assert!(tree.contains(&first_mask));
        let applied = apply_action(&match_id, "seat_0", &format!("claim>{first_mask}>5"), 0)
            .expect("claim applies");
        assert!(applied.contains("reaction"));
        assert!(
            !applied.contains(&format!("\"tile_id\":\"{first_mask}\"")),
            "pending claim leaked pedestal tile id: {applied}"
        );

        let exported = export_replay(&match_id).expect("export succeeds");
        assert!(exported.contains("\"game_id\":\"masked_claims\""));
        assert!(exported.contains("claim/grade-5"));
        assert!(
            !exported.contains("claim/mask_g"),
            "export leaked raw claim path: {exported}"
        );
        let imported = import_replay(&exported).expect("public export imports");
        assert!(imported.contains("\"game_id\":\"masked_claims\""));
        assert!(imported.contains("\"public_export\":true"));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot response applies");
        assert!(bot.contains("\"policy_id\":\"masked-claims-level1-v1\""));
        assert!(!bot.contains("reserve"));
    }

    #[test]
    fn flood_watch_bridge_projects_public_view_effects_bot_and_export_without_deck_order() {
        let created = new_match("flood_watch", 41).expect("match created");
        let match_id = extract_match_id(&created);
        assert!(created.contains("\"variant_id\":\"flood_watch_standard\""));

        let observer = get_view(&match_id, None).expect("observer view returned");
        assert!(observer.contains("\"game_id\":\"flood_watch\""));
        assert!(observer.contains("\"variant_id\":\"flood_watch_standard\""));
        assert!(observer.contains("\"undrawn_count\":"));
        assert!(!observer.contains("full_deck_order"));
        assert!(!observer.contains("\"event_deck\":"));

        let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
        assert!(tree.contains("\"segment\":\"end_turn\""));
        assert!(tree.contains("\"freshness_token\":0"));

        let applied =
            apply_action(&match_id, "seat_0", "end_turn", 0).expect("turn-ending action applies");
        assert!(applied.contains("\"type\":\"environment_phase_began\""));
        assert!(applied.contains("\"type\":\"event_drawn\""));
        assert!(applied.contains("\"active_seat\":\"seat_1\""));
        assert!(!applied.contains("full_deck_order"));
        assert!(!applied.contains("\"event_deck\":"));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot action applies");
        assert!(bot.contains("\"policy_id\":\"flood_watch_level1_public_priority_v1\""));
        assert!(!bot.contains("full_deck_order"));
        assert!(!bot.contains("\"event_deck\":"));

        let exported = export_replay(&match_id).expect("public replay exported");
        assert!(exported.contains("\"game_id\":\"flood_watch\""));
        assert!(exported.contains("\"rules_version_label\":\"flood-watch-rules-v1\""));
        assert!(exported.contains("\"viewer\":\"observer\""));
        assert!(exported.contains("\"redacted_command_summary\""));
        assert!(!exported.contains("\"commands\""));
        assert!(!exported.contains("\"full_deck_order\""));
        assert!(!exported.contains("\"event_deck\""));

        let imported = import_replay(&exported).expect("public replay imported");
        let replay_id = extract_replay_id(&imported);
        assert!(imported.contains("\"game_id\":\"flood_watch\""));
        assert!(imported.contains("\"public_export\":true"));

        let reset = replay_reset(&replay_id).expect("public replay reset returned");
        assert!(reset.contains("\"public_export\":true"));
        assert!(reset.contains("\"view\":null"));
    }

    #[test]
    fn plain_tricks_public_export_omits_seed_tail_and_unplayed_cards() {
        let seed = 0;
        let created = new_match("plain_tricks", seed).expect("match created");
        let match_id = extract_match_id(&created);

        let internal = plain_setup_match(
            Seed(seed),
            &plain_seats(),
            &plain_tricks::SetupOptions::default(),
        )
        .expect("setup succeeds");
        let seat_0_view = plain_project_view(
            &internal,
            &Viewer {
                seat_id: Some(SeatId("seat_0".to_owned())),
            },
        );
        let seat_0_cards = plain_private_cards(&seat_0_view);
        let played_card = seat_0_cards[0];
        apply_action(
            &match_id,
            "seat_0",
            &format!("play>{}", played_card.as_str()),
            0,
        )
        .expect("card applies");

        let exported = export_replay(&match_id).expect("public replay exported");
        assert!(exported.contains("\"game_id\":\"plain_tricks\""));
        assert!(exported.contains("\"export_class\":\"viewer_scoped_observation_v1\""));
        assert!(exported.contains("\"viewer\":\"observer\""));
        assert!(!exported.contains("\"commands\""));
        assert!(!exported.contains("\"seed\""));
        assert!(!exported.contains("\"seed_evidence\""));
        assert_no_plain_cards(&exported, &plain_hidden_cards_except(&[played_card]));

        let imported = import_replay(&exported).expect("public replay imported");
        let replay_id = extract_replay_id(&imported);
        assert!(imported.contains("\"public_export\":true"));
        assert!(imported.contains("\"game_id\":\"plain_tricks\""));

        let reset = replay_reset(&replay_id).expect("public replay reset returned");
        assert!(reset.contains("\"public_export\":true"));
        assert!(reset.contains("\"view\":null"));
    }

    #[test]
    fn poker_lite_view_projects_terminal_rationale_template_keys() {
        let non_terminal = get_terminal_poker_view(0, &[]);
        assert!(non_terminal.contains("\"terminal_rationale\":null"));

        let private_rank_showdown = get_terminal_poker_view(2, &["hold", "hold", "hold", "hold"]);
        assert!(private_rank_showdown.contains(
            "\"terminal_rationale\":{\"result_kind\":\"showdown_win\",\"decisive_cause\":\"higher_private_rank\",\"template_key\":\"poker_lite.private_rank_tiebreak\""
        ));
        assert!(private_rank_showdown
            .contains("\"decisive_rule_ids\":[\"CL-REVEAL-002\",\"CL-SCORE-004\",\"CL-END-002\"]"));
        assert!(private_rank_showdown.contains("\"label\":\"Private rank\""));
        assert!(private_rank_showdown.contains("\"label\":\"Pair\""));

        let pair_showdown = get_terminal_poker_view(0, &["hold", "hold", "hold", "hold"]);
        assert!(pair_showdown.contains(
            "\"terminal_rationale\":{\"result_kind\":\"showdown_win\",\"decisive_cause\":\"pair_beats_high_card\",\"template_key\":\"poker_lite.pair_beats_high_card\""
        ));

        let split = get_terminal_poker_view(1, &["hold", "hold", "hold", "hold"]);
        assert!(split.contains(
            "\"terminal_rationale\":{\"result_kind\":\"split\",\"decisive_cause\":\"equal_strength_split\",\"template_key\":\"poker_lite.equal_strength_split\""
        ));

        let yield_win = get_terminal_poker_view(11, &["press", "yield"]);
        assert!(yield_win.contains(
            "\"terminal_rationale\":{\"result_kind\":\"yield_win\",\"decisive_cause\":\"opponent_yielded\",\"template_key\":\"poker_lite.yield_win_no_reveal\""
        ));
    }

    #[test]
    fn river_ledger_view_projects_terminal_rationale_template_keys() {
        let non_terminal = get_terminal_river_view(21, 4, &[]);
        assert!(non_terminal.contains("\"terminal_rationale\":null"));
        assert!(non_terminal.contains(
            "\"hand_rankings\":[{\"category\":\"straight_flush\",\"label\":\"Straight flush\""
        ));
        assert!(non_terminal.contains("\"category\":\"high_card\",\"label\":\"High card\""));
        assert!(non_terminal.contains(
            "\"seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Seat 1\"},{\"seat\":\"seat_1\",\"label\":\"Seat 2\""
        ));
        assert!(non_terminal.contains("{\"seat\":\"seat_5\",\"label\":\"Seat 6\"}"));
        assert!(non_terminal.contains(
            "\"active_seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Seat 1\"},{\"seat\":\"seat_1\",\"label\":\"Seat 2\"},{\"seat\":\"seat_2\",\"label\":\"Seat 3\"},{\"seat\":\"seat_3\",\"label\":\"Seat 4\"}]"
        ));
        assert!(!non_terminal.contains("\"active_seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Seat 1\"},{\"seat\":\"seat_1\",\"label\":\"Seat 2\"},{\"seat\":\"seat_2\",\"label\":\"Seat 3\"},{\"seat\":\"seat_3\",\"label\":\"Seat 4\"},{\"seat\":\"seat_4\""));

        let foldout = get_terminal_river_view(21, 3, &[("seat_0", "fold"), ("seat_1", "fold")]);
        assert!(foldout.contains(
            "\"active_seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Seat 1\"},{\"seat\":\"seat_1\",\"label\":\"Seat 2\"},{\"seat\":\"seat_2\",\"label\":\"Seat 3\"}]"
        ));
        assert!(foldout.contains(
            "\"terminal_rationale\":{\"result_kind\":\"last_live_hand\",\"decisive_cause\":\"last_live_after_folds\",\"template_key\":\"river_ledger.last_live_fold_win\""
        ));
        assert!(
            foldout.contains("\"decisive_rule_ids\":[\"RL-END-LAST-LIVE\",\"RL-SCORE-POT-AWARD\"]")
        );
        assert!(!foldout.contains("\"label\":\"Category\""));
        assert!(!foldout.contains("\"label\":\"Tie break\""));

        let internal = river_setup_match(
            Seed(21),
            &river_seats_for_count(3),
            &river_ledger::SetupOptions::default(),
        )
        .expect("setup succeeds");
        let hidden_cards = (0..3)
            .flat_map(|seat_index| {
                internal
                    .private_hand_for_internal(
                        RiverLedgerSeat::from_index(seat_index).expect("valid seat"),
                    )
                    .expect("private hand")
            })
            .collect::<Vec<_>>();
        assert_no_river_cards(&foldout, &hidden_cards);

        let showdown = get_terminal_river_view(
            0,
            4,
            &[
                ("seat_3", "call"),
                ("seat_0", "call"),
                ("seat_1", "call"),
                ("seat_2", "check"),
                ("seat_1", "check"),
                ("seat_2", "check"),
                ("seat_3", "check"),
                ("seat_0", "check"),
                ("seat_1", "check"),
                ("seat_2", "check"),
                ("seat_3", "check"),
                ("seat_0", "check"),
                ("seat_1", "check"),
                ("seat_2", "check"),
                ("seat_3", "check"),
                ("seat_0", "check"),
            ],
        );
        assert!(showdown.contains(
            "\"terminal_rationale\":{\"result_kind\":\"showdown_win\",\"decisive_cause\":\"best_showdown_hand\",\"template_key\":\"river_ledger.showdown_best_hand_win\""
        ));
        assert!(
            showdown.contains("\"decisive_rule_ids\":[\"RL-SCORE-SHOWDOWN\",\"RL-END-SHOWDOWN\"]")
        );
        assert!(showdown.contains("\"label\":\"Category\""));
        assert!(showdown.contains("\"label\":\"Best five\""));
        assert!(showdown.contains("\"headline\":\""));
        assert!(showdown.contains("\"decisive_comparison\":\""));
        assert!(showdown.contains("\"comparison_basis\":\""));
        assert!(showdown.contains("\"strength\":{\"category\":\""));
        assert!(showdown.contains("\"result_label\":\""));
        assert!(showdown.contains("\"hand_name\":\""));
        assert!(showdown.contains("\"rank_explanation\":\""));
        assert!(showdown.contains("\"comparison_note\":\""));
        assert!(showdown.contains("\"category_ladder_position\":{\"position\":"));
        assert!(showdown.contains("\"description\":\""));
        assert!(showdown.contains("\"best_five_accessibility_label\":\""));
        assert!(showdown.contains("\"presentation_v2\":{\"result_banner\":{\"headline\":\""));
        assert!(showdown.contains("\"decisive_reason\":{\"short_text\":\""));
        assert!(showdown.contains("\"standings\":[{\"seat\":\""));
        assert!(showdown.contains("\"hole_cards\":[{\"card\":"));
        assert!(showdown.contains("\"used_in_best_five\":"));
        assert!(showdown.contains("\"folded_rows\":["));
    }

    #[test]
    fn river_ledger_bridge_redacts_folded_showdown_explanation_fields() {
        let seat = |index| RiverLedgerSeat::from_index(index).expect("valid seat");
        let board = [
            river_ledger::Card::new(river_ledger::Rank::Ten, river_ledger::Suit::Hearts),
            river_ledger::Card::new(river_ledger::Rank::Jack, river_ledger::Suit::Hearts),
            river_ledger::Card::new(river_ledger::Rank::Queen, river_ledger::Suit::Hearts),
            river_ledger::Card::new(river_ledger::Rank::King, river_ledger::Suit::Hearts),
            river_ledger::Card::new(river_ledger::Rank::Ace, river_ledger::Suit::Hearts),
        ];
        let mut state = river_ledger::RiverLedgerState::new_after_setup(
            river_ledger::Variant::river_ledger_standard(),
            river_seats_for_count(3),
            river_ledger::state::SeatRoles {
                button: seat(0),
                small_blind: seat(1),
                big_blind: seat(2),
                active_seat: seat(0),
            },
            vec![
                [
                    river_ledger::Card::new(river_ledger::Rank::Two, river_ledger::Suit::Diamonds),
                    river_ledger::Card::new(
                        river_ledger::Rank::Three,
                        river_ledger::Suit::Diamonds,
                    ),
                ],
                [
                    river_ledger::Card::new(river_ledger::Rank::Two, river_ledger::Suit::Clubs),
                    river_ledger::Card::new(river_ledger::Rank::Three, river_ledger::Suit::Clubs),
                ],
                [
                    river_ledger::Card::new(river_ledger::Rank::Four, river_ledger::Suit::Clubs),
                    river_ledger::Card::new(river_ledger::Rank::Five, river_ledger::Suit::Clubs),
                ],
            ],
            board,
            Vec::new(),
        );
        state.board = board.to_vec();
        state.ledger.seats = (0..3)
            .map(|index| river_ledger::SeatLedger {
                seat: seat(index),
                status: if index == 0 {
                    river_ledger::SeatStatus::Folded
                } else {
                    river_ledger::SeatStatus::ShowdownEligible
                },
                street_contribution: 0,
                total_contribution: 3,
            })
            .collect();
        state.ledger.pot_total = 9;
        state.terminal_outcome = Some(river_ledger::resolve_showdown(&state));

        let json = river_view_json(&river_project_view(&state, &Viewer { seat_id: None }));
        assert!(json.contains("\"headline\":\""));
        assert!(json.contains("\"id\":\"seat_0\",\"label\":\"seat_0\",\"result\":\"folded\",\"emphasized\":false,\"strength\":null"));
        assert!(json.contains("\"id\":\"seat_1\""));
        assert!(json.contains("\"strength\":{\"category\":\"straight_flush\""));
        assert!(!json.contains("two_diamonds"));
        assert!(!json.contains("three_diamonds"));
    }

    #[test]
    fn plain_tricks_view_projects_terminal_rationale_template_keys() {
        let non_terminal = get_terminal_plain_view(0, &[]);
        assert!(non_terminal.contains("\"terminal_rationale\":null"));
        assert!(non_terminal
            .contains("\"terminal\":{\"kind\":\"non_terminal\",\"winner\":null,\"draw\":false}"));

        let trick_win = get_terminal_plain_view(0, &PLAIN_TRICKS_WIN_ACTIONS);
        assert!(trick_win.contains("\"terminal\":{\"kind\":\"trick_win\""));
        assert!(trick_win.contains("\"terminal_rationale\":{"));
        assert!(!trick_win.contains("\"rationale\":{"));
        assert!(trick_win.contains("\"result_kind\":\"trick_win\""));
        assert!(trick_win.contains("\"template_key\":\"plain_tricks.trick_win\""));
        assert!(trick_win
            .contains("\"decisive_rule_ids\":[\"PT-SCORE-002\",\"PT-END-001\",\"PT-END-002\"]"));
        assert!(trick_win.contains("\"total_tricks\":"));

        let split = get_terminal_plain_view(5, &PLAIN_TRICKS_SPLIT_ACTIONS);
        assert!(split.contains("\"terminal\":{\"kind\":\"split\""));
        assert!(split.contains("\"terminal_rationale\":{"));
        assert!(!split.contains("\"rationale\":{"));
        assert!(split.contains("\"result_kind\":\"split\""));
        assert!(split.contains("\"decisive_cause\":\"split:6-6\""));
        assert!(split.contains("\"template_key\":\"plain_tricks.split\""));
        assert!(split
            .contains("\"decisive_rule_ids\":[\"PT-SCORE-002\",\"PT-END-001\",\"PT-END-002\"]"));
    }

    #[test]
    fn plain_tricks_terminal_rationale_does_not_reveal_unplayed_cards() {
        let view = get_terminal_plain_view(0, &PLAIN_TRICKS_WIN_ACTIONS[..2]);
        assert!(view.contains("\"terminal_rationale\":null"));

        let terminal = get_terminal_plain_view(0, &PLAIN_TRICKS_WIN_ACTIONS);
        assert!(terminal.contains("\"terminal_rationale\":{"));
        assert!(terminal.contains("\"template_key\":\"plain_tricks.trick_win\""));
        assert_no_plain_cards(
            &terminal,
            &plain_hidden_cards_except(&PLAIN_TRICKS_WIN_PLAYED_CARDS),
        );
    }

    #[test]
    fn poker_lite_yield_terminal_rationale_does_not_reveal_private_strength() {
        let seed = 11;
        let view = get_terminal_poker_view(seed, &["press", "yield"]);
        let internal =
            poker_setup_match(Seed(seed), &seats(), &poker_lite::SetupOptions::default())
                .expect("setup succeeds");

        assert!(view.contains("\"terminal_rationale\":{"));
        assert!(view.contains("\"template_key\":\"poker_lite.yield_win_no_reveal\""));
        assert!(!view.contains("\"label\":\"Pair\""));
        assert!(!view.contains("\"label\":\"Private rank\""));
        assert!(!view.contains("\"rank\":"));
        assert_no_poker_cards(
            &view,
            &[
                internal.private_card_for_internal(PokerLiteSeat::Seat0),
                internal.private_card_for_internal(PokerLiteSeat::Seat1),
                internal.center_card_internal(),
            ],
        );
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

    #[test]
    fn poker_lite_wasm_public_export_matches_golden_fixture() {
        let created = new_match("poker_lite", 101).expect("match created");
        let match_id = extract_match_id(&created);
        apply_action(&match_id, "seat_0", "press", 0).expect("press applies");
        run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        let exported = export_replay(&match_id).expect("public replay exported");
        let fixture =
            include_str!("../../../games/poker_lite/tests/golden_traces/wasm-exported.trace.json");

        assert_eq!(compact_json_layout(fixture), exported);
    }

    #[test]
    fn plain_tricks_wasm_export_matches_golden_fixture() {
        let commands = [
            AppliedCommand {
                actor_seat: "seat_0".to_owned(),
                action_path: vec!["play".to_owned(), "gale_1".to_owned()],
                freshness_token: 0,
            },
            AppliedCommand {
                actor_seat: "seat_1".to_owned(),
                action_path: vec!["play".to_owned(), "gale_2".to_owned()],
                freshness_token: 1,
            },
        ];
        let exported =
            plain_replay_document_json("wasm-exported", 0, &commands).expect("fixture exported");
        let fixture = include_str!(
            "../../../games/plain_tricks/tests/golden_traces/wasm-exported.trace.json"
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

    fn high_card_pairwise_no_leak_case() -> PairwiseNoLeakCase {
        let seats = vec!["seat_0".to_owned(), "seat_1".to_owned()];
        let created = new_match("high_card_duel", 707).expect("match created");
        let match_id = extract_match_id(&created);
        let mut private_terms_by_seat = BTreeMap::new();
        let mut surfaces = Vec::new();

        for viewer in [None, Some("seat_0"), Some("seat_1")] {
            surfaces.push(NoLeakSurface {
                viewer: viewer.map(ToOwned::to_owned),
                name: "payload",
                payload: get_view(&match_id, viewer).expect("viewer payload returned"),
            });
        }

        for seat in &seats {
            let view = get_view(&match_id, Some(seat)).expect("seat payload returned");
            let terms = collect_prefixed_terms(&view, "hcd:r");
            assert!(
                !terms.is_empty(),
                "expected private high-card token for {seat}"
            );
            private_terms_by_seat.insert(seat.clone(), terms);
        }

        for actor in &seats {
            for viewer in [None, Some("seat_0"), Some("seat_1")] {
                surfaces.push(NoLeakSurface {
                    viewer: viewer.map(ToOwned::to_owned),
                    name: "action_tree",
                    payload: get_action_tree_for_viewer(&match_id, actor, viewer)
                        .expect("viewer action tree returned"),
                });
            }
        }

        let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
            .expect("authorized action tree");
        let action_segment = first_segment(&authorized);
        let applied =
            apply_action(&match_id, "seat_0", &action_segment, 0).expect("commit applies");
        surfaces.push(NoLeakSurface {
            viewer: Some("seat_0".to_owned()),
            name: "payload",
            payload: applied,
        });

        for viewer in [None, Some("seat_0"), Some("seat_1")] {
            surfaces.push(NoLeakSurface {
                viewer: viewer.map(ToOwned::to_owned),
                name: "effect_log",
                payload: get_effects(&match_id, 0, viewer).expect("viewer effects returned"),
            });
        }

        surfaces.push(NoLeakSurface {
            viewer: None,
            name: "replay_export",
            payload: export_replay(&match_id).expect("public replay exported"),
        });
        for name in [
            "preview",
            "bot_explanation",
            "candidate_ranking",
            "dom_test_id",
            "storage",
            "log",
        ] {
            surfaces.push(NoLeakSurface {
                viewer: None,
                name,
                payload: format!("high_card_duel {name} not_applicable_or_redacted"),
            });
        }

        PairwiseNoLeakCase {
            seats,
            private_terms_by_seat,
            surfaces,
        }
    }

    fn poker_lite_pairwise_no_leak_case() -> PairwiseNoLeakCase {
        let seed = 727;
        let created = new_match("poker_lite", seed).expect("match created");
        let match_id = extract_match_id(&created);
        let internal =
            poker_setup_match(Seed(seed), &seats(), &poker_lite::SetupOptions::default())
                .expect("setup succeeds");
        let mut private_terms_by_seat = BTreeMap::new();
        private_terms_by_seat.insert(
            "seat_0".to_owned(),
            vec![internal
                .private_card_for_internal(PokerLiteSeat::Seat0)
                .as_str()
                .to_owned()],
        );
        private_terms_by_seat.insert(
            "seat_1".to_owned(),
            vec![internal
                .private_card_for_internal(PokerLiteSeat::Seat1)
                .as_str()
                .to_owned()],
        );
        bridge_pairwise_no_leak_case(&match_id, private_terms_by_seat)
    }

    fn plain_tricks_pairwise_no_leak_case() -> PairwiseNoLeakCase {
        let seed = 737;
        let created = new_match("plain_tricks", seed).expect("match created");
        let match_id = extract_match_id(&created);
        let internal = plain_setup_match(
            Seed(seed),
            &plain_seats(),
            &plain_tricks::SetupOptions::default(),
        )
        .expect("setup succeeds");
        let seat_0_view = plain_project_view(
            &internal,
            &Viewer {
                seat_id: Some(SeatId("seat_0".to_owned())),
            },
        );
        let seat_1_view = plain_project_view(
            &internal,
            &Viewer {
                seat_id: Some(SeatId("seat_1".to_owned())),
            },
        );
        let mut private_terms_by_seat = BTreeMap::new();
        private_terms_by_seat.insert(
            "seat_0".to_owned(),
            plain_private_cards(&seat_0_view)
                .iter()
                .map(|card| card.as_str().to_owned())
                .collect(),
        );
        private_terms_by_seat.insert(
            "seat_1".to_owned(),
            plain_private_cards(&seat_1_view)
                .iter()
                .map(|card| card.as_str().to_owned())
                .collect(),
        );
        bridge_pairwise_no_leak_case(&match_id, private_terms_by_seat)
    }

    fn masked_claims_pairwise_no_leak_case() -> PairwiseNoLeakCase {
        let created = new_match("masked_claims", 747).expect("match created");
        let match_id = extract_match_id(&created);
        let mut private_terms_by_seat = BTreeMap::new();
        for seat in ["seat_0", "seat_1"] {
            let view = get_view(&match_id, Some(seat)).expect("seat view returned");
            let terms = collect_prefixed_terms(&view, "mask_g");
            assert!(!terms.is_empty(), "expected private mask ids for {seat}");
            private_terms_by_seat.insert(seat.to_owned(), terms);
        }
        bridge_pairwise_no_leak_case(&match_id, private_terms_by_seat)
    }

    fn river_ledger_pairwise_no_leak_case() -> PairwiseNoLeakCase {
        let seed = 757;
        let created = new_match_with_seat_count("river_ledger", seed, 6).expect("match created");
        let match_id = extract_match_id(&created);
        let internal = river_setup_match(
            Seed(seed),
            &river_seats_for_count(6),
            &river_ledger::SetupOptions::default(),
        )
        .expect("setup succeeds");
        let mut private_terms_by_seat = BTreeMap::new();
        for seat_index in 0..6 {
            let seat = RiverLedgerSeat::from_index(seat_index).expect("valid seat");
            let cards = internal
                .private_hand_for_internal(seat)
                .expect("seat has private hand")
                .iter()
                .map(|card| card.id().to_owned())
                .collect::<Vec<_>>();
            private_terms_by_seat.insert(seat.as_str(), cards);
        }
        bridge_pairwise_no_leak_case(&match_id, private_terms_by_seat)
    }

    fn bridge_pairwise_no_leak_case(
        match_id: &str,
        private_terms_by_seat: BTreeMap<String, Vec<String>>,
    ) -> PairwiseNoLeakCase {
        let seats = private_terms_by_seat.keys().cloned().collect::<Vec<_>>();
        let viewer_options = std::iter::once(None)
            .chain(seats.iter().map(|seat| Some(seat.as_str())))
            .collect::<Vec<_>>();
        let mut surfaces = Vec::new();
        for viewer in &viewer_options {
            surfaces.push(NoLeakSurface {
                viewer: viewer.map(ToOwned::to_owned),
                name: "payload",
                payload: get_view(match_id, *viewer).expect("viewer payload returned"),
            });
            surfaces.push(NoLeakSurface {
                viewer: viewer.map(ToOwned::to_owned),
                name: "effect_log",
                payload: get_effects(match_id, 0, *viewer).expect("viewer effects returned"),
            });
        }
        for actor in &seats {
            for viewer in &viewer_options {
                surfaces.push(NoLeakSurface {
                    viewer: viewer.map(ToOwned::to_owned),
                    name: "action_tree",
                    payload: get_action_tree_for_viewer(match_id, actor, *viewer)
                        .expect("viewer action tree returned"),
                });
            }
        }
        surfaces.push(NoLeakSurface {
            viewer: None,
            name: "replay_export",
            payload: export_replay(match_id).expect("public replay exported"),
        });
        for name in [
            "preview",
            "bot_explanation",
            "candidate_ranking",
            "dom_test_id",
            "storage",
            "log",
        ] {
            surfaces.push(NoLeakSurface {
                viewer: None,
                name,
                payload: format!("bridge {name} not_applicable_or_redacted"),
            });
        }

        PairwiseNoLeakCase {
            seats,
            private_terms_by_seat,
            surfaces,
        }
    }

    fn collect_prefixed_terms(input: &str, prefix: &str) -> Vec<String> {
        let mut terms = Vec::new();
        for (index, _) in input.match_indices(prefix) {
            let token = input[index..]
                .chars()
                .take_while(|character| {
                    character.is_ascii_alphanumeric() || matches!(character, ':' | '_' | '-' | '/')
                })
                .collect::<String>();
            if !token.is_empty() {
                terms.push(token);
            }
        }
        terms.sort();
        terms.dedup();
        terms
    }

    fn first_segment(tree: &str) -> String {
        tree.split("\"segment\":\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .expect("segment present")
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

    fn get_terminal_poker_view(seed: u64, action_paths: &[&str]) -> String {
        let created = new_match("poker_lite", seed).expect("match created");
        let match_id = extract_match_id(&created);

        for (freshness_token, action_path) in action_paths.iter().enumerate() {
            let tree = get_action_tree(&match_id, "seat_0").expect("seat_0 tree returned");
            let actor = if tree.contains("\"choices\":[]") {
                "seat_1"
            } else {
                "seat_0"
            };
            apply_action(&match_id, actor, action_path, freshness_token as u64)
                .expect("poker action applies");
        }

        get_view(&match_id, None).expect("observer view returned")
    }

    fn get_terminal_river_view(seed: u64, seat_count: u8, actions: &[(&str, &str)]) -> String {
        let created = new_match_with_seat_count("river_ledger", seed, usize::from(seat_count))
            .expect("match created");
        let match_id = extract_match_id(&created);

        for (freshness_token, (actor, action_path)) in actions.iter().enumerate() {
            apply_action(&match_id, actor, action_path, freshness_token as u64)
                .expect("river action applies");
        }

        get_view(&match_id, None).expect("observer view returned")
    }

    const PLAIN_TRICKS_WIN_ACTIONS: [(&str, &str); 24] = [
        ("seat_0", "play>gale_1"),
        ("seat_1", "play>gale_2"),
        ("seat_1", "play>ember_3"),
        ("seat_0", "play>ember_6"),
        ("seat_0", "play>river_3"),
        ("seat_1", "play>river_6"),
        ("seat_1", "play>gale_3"),
        ("seat_0", "play>river_5"),
        ("seat_1", "play>ember_2"),
        ("seat_0", "play>ember_5"),
        ("seat_0", "play>river_1"),
        ("seat_1", "play>gale_6"),
        ("seat_1", "play>ember_4"),
        ("seat_0", "play>ember_2"),
        ("seat_1", "play>gale_1"),
        ("seat_0", "play>river_5"),
        ("seat_1", "play>gale_6"),
        ("seat_0", "play>river_2"),
        ("seat_1", "play>ember_6"),
        ("seat_0", "play>river_3"),
        ("seat_1", "play>gale_3"),
        ("seat_0", "play>river_1"),
        ("seat_1", "play>gale_5"),
        ("seat_0", "play>river_6"),
    ];

    const PLAIN_TRICKS_SPLIT_ACTIONS: [(&str, &str); 24] = [
        ("seat_0", "play>river_6"),
        ("seat_1", "play>river_5"),
        ("seat_0", "play>river_1"),
        ("seat_1", "play>river_4"),
        ("seat_1", "play>gale_5"),
        ("seat_0", "play>gale_2"),
        ("seat_1", "play>ember_6"),
        ("seat_0", "play>ember_1"),
        ("seat_1", "play>gale_1"),
        ("seat_0", "play>gale_6"),
        ("seat_0", "play>gale_3"),
        ("seat_1", "play>ember_5"),
        ("seat_1", "play>ember_5"),
        ("seat_0", "play>ember_2"),
        ("seat_1", "play>gale_2"),
        ("seat_0", "play>gale_5"),
        ("seat_0", "play>river_1"),
        ("seat_1", "play>river_2"),
        ("seat_1", "play>river_4"),
        ("seat_0", "play>river_6"),
        ("seat_0", "play>ember_6"),
        ("seat_1", "play>gale_1"),
        ("seat_0", "play>gale_3"),
        ("seat_1", "play>gale_4"),
    ];

    const PLAIN_TRICKS_WIN_PLAYED_CARDS: [plain_tricks::TrickCardId; 24] = [
        plain_tricks::TrickCardId::Gale1,
        plain_tricks::TrickCardId::Gale2,
        plain_tricks::TrickCardId::Ember3,
        plain_tricks::TrickCardId::Ember6,
        plain_tricks::TrickCardId::River3,
        plain_tricks::TrickCardId::River6,
        plain_tricks::TrickCardId::Gale3,
        plain_tricks::TrickCardId::River5,
        plain_tricks::TrickCardId::Ember2,
        plain_tricks::TrickCardId::Ember5,
        plain_tricks::TrickCardId::River1,
        plain_tricks::TrickCardId::Gale6,
        plain_tricks::TrickCardId::Ember4,
        plain_tricks::TrickCardId::Ember2,
        plain_tricks::TrickCardId::Gale1,
        plain_tricks::TrickCardId::River5,
        plain_tricks::TrickCardId::Gale6,
        plain_tricks::TrickCardId::River2,
        plain_tricks::TrickCardId::Ember6,
        plain_tricks::TrickCardId::River3,
        plain_tricks::TrickCardId::Gale3,
        plain_tricks::TrickCardId::River1,
        plain_tricks::TrickCardId::Gale5,
        plain_tricks::TrickCardId::River6,
    ];

    fn get_terminal_plain_view(seed: u64, actions: &[(&str, &str)]) -> String {
        let created = new_match("plain_tricks", seed).expect("match created");
        let match_id = extract_match_id(&created);

        for (freshness_token, (actor, action_path)) in actions.iter().enumerate() {
            apply_action(&match_id, actor, action_path, freshness_token as u64)
                .expect("plain_tricks action applies");
        }

        get_view(&match_id, None).expect("observer view returned")
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

    fn assert_no_poker_cards(input: &str, cards: &[poker_lite::CrestCardId]) {
        for card in cards {
            assert!(
                !input.contains(card.as_str()),
                "hidden poker_lite card {} leaked in {input}",
                card.as_str()
            );
        }
    }

    fn assert_no_river_cards(input: &str, cards: &[river_ledger::Card]) {
        for card in cards {
            assert!(
                !input.contains(&card.id()),
                "hidden river_ledger card {} leaked in {input}",
                card.id()
            );
        }
    }

    fn first_mask_segment(input: &str) -> String {
        let start = input.find("mask_g").expect("mask id is present");
        let rest = &input[start..];
        let end = rest.find('"').expect("mask id terminates");
        rest[..end].to_owned()
    }

    fn plain_private_cards(view: &plain_tricks::PublicView) -> Vec<plain_tricks::TrickCardId> {
        match &view.private_view {
            plain_tricks::PrivateView::Seat(private) => private
                .own_hand
                .iter()
                .map(|card| plain_tricks::TrickCardId::parse(&card.card_id).expect("known card"))
                .collect(),
            plain_tricks::PrivateView::Observer => panic!("expected private seat view"),
        }
    }

    fn plain_cards_except(
        cards: &[plain_tricks::TrickCardId],
        exceptions: &[plain_tricks::TrickCardId],
    ) -> Vec<plain_tricks::TrickCardId> {
        cards
            .iter()
            .copied()
            .filter(|card| !exceptions.contains(card))
            .collect()
    }

    fn plain_hidden_cards_except(
        exceptions: &[plain_tricks::TrickCardId],
    ) -> Vec<plain_tricks::TrickCardId> {
        plain_tricks::TrickCardId::ALL
            .iter()
            .copied()
            .filter(|card| !exceptions.contains(card))
            .collect()
    }

    fn assert_no_plain_cards(input: &str, cards: &[plain_tricks::TrickCardId]) {
        for card in cards {
            assert!(
                !input.contains(card.as_str()),
                "hidden plain_tricks card {} leaked in {input}",
                card.as_str()
            );
        }
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
