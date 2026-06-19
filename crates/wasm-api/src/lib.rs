//! Browser-facing Rulepath API surface.

use std::{cell::RefCell, slice, str};

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
    ActionPath, CommandEnvelope, EffectCursor, EffectEnvelope, EffectLog, RulesVersion, Seed,
    Viewer, VisibilityScope,
};
#[cfg(test)]
use engine_core::{Actor, HashValue, SeatId, StableSerialize};
use event_frontier::visibility::{
    public_effect_text as event_frontier_public_effect_text,
    reason_label as event_frontier_reason_label,
};
use event_frontier::{
    apply_command as event_frontier_apply_command,
    command_for_decision as event_frontier_command_for_decision,
    legal_action_tree as event_frontier_legal_action_tree,
    project_view as event_frontier_project_view,
    resolve_reckoning as event_frontier_resolve_reckoning,
    setup_match as event_frontier_setup_match, validate_command as event_frontier_validate_command,
    EventCharterLevel1Bot, EventFreeholdersLevel1Bot, EventFrontierEffect, EventFrontierState,
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
    import_public_export as high_card_import_public_export,
    legal_action_tree as high_card_legal_action_tree, project_view as high_card_project_view,
    setup_match as high_card_setup_match, validate_command as high_card_validate_command,
    HighCardDuelEffect, HighCardDuelInternalTrace, HighCardDuelRandomBot, HighCardDuelSeat,
    HighCardDuelState, PublicReplayExport, PublicReplayStep,
};
use masked_claims::{
    apply_action as masked_apply_action, legal_action_tree as masked_legal_action_tree,
    project_view as masked_project_view, setup_match as masked_setup_match, MaskedClaimsEffect,
    MaskedClaimsLevel1Bot, MaskedClaimsSeat, MaskedClaimsState,
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
        export_public_replay as secret_export_public_replay,
        import_public_export as secret_import_public_export,
        PublicReplayStep as SecretPublicReplayStep, ReplayCommand as SecretReplayCommand,
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
    project_view as token_project_view, replay_support::replay_commands as token_replay_commands,
    setup_match as token_setup_match, TokenBazaarEffect, TokenBazaarLevel1Bot, TokenBazaarSeat,
    TokenBazaarState,
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
use commands::*;
use constants::*;
use games::race::*;
use games::three::*;
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

fn is_high_card_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("public_observer_projection_v1")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(high_card_duel::GAME_ID)
    )
}

fn is_secret_draft_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("viewer_scoped_observation_v1")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(secret_draft::GAME_ID)
    )
}

fn is_masked_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("viewer_scoped_observation")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(masked_claims::GAME_ID)
    )
}

fn is_flood_watch_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(flood_watch::GAME_ID)
    ) && string_field(doc, "rules_version_label").is_ok()
        && string_field(doc, "viewer").is_ok()
}

fn is_frontier_control_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(frontier_control::GAME_ID)
    ) && string_field(doc, "rules_version_label").is_ok()
        && doc.contains("\"not_applicable\"")
}

fn is_event_frontier_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(event_frontier::GAME_ID)
    ) && string_field(doc, "rules_version_label").is_ok()
        && string_field(doc, "viewer").is_ok()
        && string_field(doc, "hidden_information_redaction").is_ok()
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
        steps: steps
            .iter()
            .map(high_card_step_from_public_timeline)
            .collect(),
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
                    steps: timeline
                        .steps
                        .iter()
                        .map(public_timeline_step_from_high_card)
                        .collect(),
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

fn import_secret_draft_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export =
        secret_draft::replay_support::PublicReplayExport::from_json(doc).map_err(|message| {
            diagnostic_string(
                "invalid_replay",
                &format!("invalid public replay document: {message}"),
            )
        })?;
    if export.rules_version != secret_draft::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {}", export.rules_version),
        ));
    }
    if export.variant != secret_draft::VARIANT_ID {
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
    let timeline = secret_import_public_export(&export);
    let replay_id = next_replay_id(GAME_SECRET_DRAFT);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_SECRET_DRAFT.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: timeline.viewer.clone(),
                    steps: timeline
                        .steps
                        .iter()
                        .map(public_timeline_step_from_secret)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_SECRET_DRAFT),
        escape_json(&timeline.viewer),
        timeline.steps.len()
    ))
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

fn import_masked_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export = masked_claims::PublicReplayExport::from_json(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    if export.rules_version != masked_claims::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {}", export.rules_version),
        ));
    }
    if export.variant != masked_claims::VARIANT_ID {
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
    let replay_id = next_replay_id(GAME_MASKED_CLAIMS);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_MASKED_CLAIMS.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: export.viewer.clone(),
                    steps: export
                        .steps
                        .iter()
                        .map(public_timeline_step_from_masked)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_MASKED_CLAIMS),
        escape_json(&export.viewer),
        export.steps.len()
    ))
}

fn import_flood_watch_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let rules_version = string_field(doc, "rules_version_label")?;
    if rules_version != flood_watch::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {rules_version}"),
        ));
    }
    let variant = string_field(doc, "variant")?;
    if variant != flood_watch::VARIANT_STANDARD_ID && variant != flood_watch::VARIANT_DELUGE_ID {
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
    let export = flood_watch::PublicReplayExport {
        schema_version: SCHEMA_VERSION,
        game_id: flood_watch::GAME_ID.to_owned(),
        rules_version_label: rules_version,
        variant,
        viewer,
        steps: steps.iter().map(flood_step_from_public_timeline).collect(),
    };
    let _timeline = flood_watch::import_public_export(&export);
    let viewer = export.viewer.clone();
    let step_count = export.steps.len();
    let replay_id = next_replay_id(GAME_FLOOD_WATCH);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_FLOOD_WATCH.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: viewer.clone(),
                    steps: export
                        .steps
                        .iter()
                        .map(public_timeline_step_from_flood)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_FLOOD_WATCH),
        escape_json(&viewer),
        step_count
    ))
}

fn import_frontier_control_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    frontier_control::import_public_export_json(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let rules_version = string_field(doc, "rules_version_label")?;
    if rules_version != frontier_control::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {rules_version}"),
        ));
    }
    let variant = string_field(doc, "variant")?;
    if variant != frontier_control::VARIANT_STANDARD_ID
        && variant != frontier_control::VARIANT_HIGHLANDS_ID
    {
        return Err(diagnostic_string(
            "unsupported_replay_variant",
            &format!("unsupported replay variant: {variant}"),
        ));
    }
    let steps = parse_frontier_public_replay_steps(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export = frontier_control::PublicReplayExport {
        schema_version: SCHEMA_VERSION,
        game_id: frontier_control::GAME_ID.to_owned(),
        rules_version_label: rules_version,
        variant,
        not_applicable: frontier_control::trace_not_applicable(),
        steps: steps
            .iter()
            .map(frontier_step_from_public_timeline)
            .collect(),
    };
    let imported = frontier_control::import_public_export(&export);
    let timeline_steps = export
        .steps
        .iter()
        .map(public_timeline_step_from_frontier)
        .collect::<Vec<_>>();
    let step_count = timeline_steps.len();
    let replay_id = next_replay_id(GAME_FRONTIER_CONTROL);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_FRONTIER_CONTROL.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: "observer".to_owned(),
                    steps: timeline_steps,
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"observer\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0,\"raw_size\":{}}}",
        escape_json(&replay_id),
        escape_json(GAME_FRONTIER_CONTROL),
        step_count,
        imported.raw_json.len()
    ))
}

fn import_event_frontier_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    event_frontier::import_public_export_json(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let rules_version = string_field(doc, "rules_version_label")?;
    if rules_version != event_frontier::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {rules_version}"),
        ));
    }
    let variant = string_field(doc, "variant")?;
    if variant != event_frontier::VARIANT_STANDARD_ID
        && variant != event_frontier::VARIANT_HARD_WINTER_ID
        && variant != event_frontier::VARIANT_LAND_RUSH_ID
    {
        return Err(diagnostic_string(
            "unsupported_replay_variant",
            &format!("unsupported replay variant: {variant}"),
        ));
    }
    let viewer = string_field(doc, "viewer")?;
    let steps = parse_public_replay_steps(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export = event_frontier::PublicReplayExport {
        schema_version: SCHEMA_VERSION,
        game_id: event_frontier::GAME_ID.to_owned(),
        rules_version_label: rules_version,
        variant,
        viewer,
        hidden_information: event_frontier::TRACE_HIDDEN_SURFACE.to_owned(),
        hidden_information_redaction: "undrawn_order_redacted".to_owned(),
        stochastic_game_rule_events: event_frontier::TRACE_STOCHASTIC_SURFACE.to_owned(),
        steps: steps
            .iter()
            .map(event_frontier_step_from_public_timeline)
            .collect(),
    };
    let imported = event_frontier::import_public_export(&export);
    let viewer = export.viewer.clone();
    let step_count = export.steps.len();
    let replay_id = next_replay_id(GAME_EVENT_FRONTIER);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_EVENT_FRONTIER.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: viewer.clone(),
                    steps: export
                        .steps
                        .iter()
                        .map(public_timeline_step_from_event_frontier)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0,\"raw_size\":{}}}",
        escape_json(&replay_id),
        escape_json(GAME_EVENT_FRONTIER),
        escape_json(&viewer),
        step_count,
        imported.raw_json.len()
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

fn high_card_step_from_public_timeline(step: &PublicTimelineStep) -> PublicReplayStep {
    PublicReplayStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

fn public_timeline_step_from_high_card(step: &PublicReplayStep) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

fn public_timeline_step_from_masked(step: &masked_claims::PublicReplayStep) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

fn flood_step_from_public_timeline(step: &PublicTimelineStep) -> flood_watch::PublicReplayStep {
    flood_watch::PublicReplayStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

fn public_timeline_step_from_flood(step: &flood_watch::PublicReplayStep) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

fn event_frontier_step_from_public_timeline(
    step: &PublicTimelineStep,
) -> event_frontier::PublicReplayStep {
    event_frontier::PublicReplayStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

fn public_timeline_step_from_event_frontier(
    step: &event_frontier::PublicReplayStep,
) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

fn frontier_step_from_public_timeline(
    step: &PublicTimelineStep,
) -> frontier_control::PublicReplayStep {
    frontier_control::PublicReplayStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

fn public_timeline_step_from_frontier(
    step: &frontier_control::PublicReplayStep,
) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.command_summary.clone(),
        terminal: step.terminal,
    }
}

fn public_timeline_step_from_secret(step: &SecretPublicReplayStep) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
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

fn masked_redacted_command_summary(command: &AppliedCommand) -> String {
    match command.action_path.as_slice() {
        [family, _tile, declared] if family == masked_claims::ACTION_CLAIM => {
            format!("claim/grade-{declared}")
        }
        _ => command.action_path.join("/"),
    }
}

fn flood_redacted_command_summary(command: &AppliedCommand) -> String {
    command.action_path.join("/")
}

fn frontier_command_summary(command: &AppliedCommand) -> String {
    format!(
        "{}:{}:{}",
        command.actor_seat,
        command.action_path.join("/"),
        command.freshness_token
    )
}

fn event_frontier_command_summary(command: &AppliedCommand) -> String {
    format!(
        "{}:{}:{}",
        command.actor_seat,
        command.action_path.join("/"),
        command.freshness_token
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

fn secret_replay_to_cursor(
    _seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(SecretDraftState, Vec<EffectEnvelope<SecretDraftEffect>>), String> {
    let seats = seats();
    let mut state = secret_setup_match(&seats, &secret_draft::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_secret_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: secret_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action =
            secret_draft::actions::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(secret_apply_action(&mut state, action).map_err(diagnostic_json)?);
    }
    Ok((state, all_effects))
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

fn event_frontier_effects_json(
    effects: &[EffectEnvelope<EventFrontierEffect>],
    viewer: &Viewer,
) -> String {
    let body = event_frontier::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(event_frontier_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn event_frontier_finish_automated_phases(
    state: &mut EventFrontierState,
) -> Result<Vec<EffectEnvelope<EventFrontierEffect>>, String> {
    let mut effects = Vec::new();
    while state.card_phase == event_frontier::CardPhase::Reckoning {
        effects.extend(
            event_frontier_resolve_reckoning(state)
                .map_err(diagnostic_json)?
                .effects,
        );
    }
    Ok(effects)
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
        "{{\"schema_version\":{},\"trace_id\":\"{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported_replay\",\"note\":\"Replay exported by the Rulepath WASM API from the Rust command log with ordered multi-segment action paths preserved.\",\"migration_update_note\":\"Updated expected hashes for VICEXPSHASUR-006 terminal reason projection.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat-0\",\"player_id\":\"player-0\"}},{{\"seat_id\":\"seat-1\",\"player_id\":\"player-1\"}}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_private_view_hashes\":{{\"not_applicable\":\"draughts_lite is perfect-information and has no private-view API.\"}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_outcome\":{},\"expected_terminal_state\":{},\"not_applicable\":{{\"hidden_information\":\"draughts_lite is perfect-information and has no hidden state to redact.\",\"stochastic_game_events\":\"draughts_lite game rules use no randomness; bot RNG is not replayed from exported documents because resolved commands are recorded.\",\"private_view_hashes\":\"draughts_lite has no private-view API.\",\"preview_hashes\":\"draughts_lite uses action-tree metadata and semantic effects rather than a separate preview hash surface in Gate 7.\"}}}}",
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
        "{{\"schema_version\":{},\"trace_id\":\"token-bazaar-{}\",\"fixture_kind\":\"wasm\",\"purpose\":\"public_export_round_trip\",\"note\":\"Token Bazaar public_export_round_trip replay fixture.\",\"migration_update_note\":\"Updated expected hashes for VICEXPSHASUR-007 tiebreak rationale projection.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat-0\",\"player_id\":\"player-0\"}},{{\"seat_id\":\"seat-1\",\"player_id\":\"player-1\"}}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_diagnostic_hashes\":null,\"expected_public_export_hashes\":{{\"final\":{}}},\"expected_outcome\":{{\"terminal\":{},\"winner\":{},\"draw\":{}}},\"expected_terminal_state\":{{\"terminal\":{},\"winner\":{},\"draw\":{}}},\"not_applicable\":{{\"hidden_information\":\"token_bazaar is fully public.\",\"stochastic_game_events\":\"token_bazaar game rules use no randomness.\",\"private_view_hashes\":\"token_bazaar observer and seat views are identical.\",\"preview_hashes\":\"token_bazaar uses legal action metadata rather than a separate preview hash.\"}}}}",
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

fn secret_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &SecretDraftState,
    effects: &[EffectEnvelope<SecretDraftEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        secret_view_json(&secret_project_view(state, &viewer)),
        secret_effects_json(effects, &viewer)
    )
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

pub(crate) fn visibility_json(visibility: &VisibilityScope) -> String {
    match visibility {
        VisibilityScope::Public => "\"public\"".to_owned(),
        VisibilityScope::PrivateToSeat(seat) => {
            format!("{{\"private_to_seat\":\"{}\"}}", escape_json(&seat.0))
        }
    }
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
        column_four::TerminalView::Draw { .. } => "draw",
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
        | directional_flip::TerminalView::Draw { final_score, .. } => {
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
        token_bazaar::TerminalView::Win { winning_seat, .. } => format!(
            "{{\"terminal\":true,\"winner\":\"{}\",\"draw\":false}}",
            winning_seat.as_str()
        ),
        token_bazaar::TerminalView::Draw { .. } => {
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

fn masked_view_json(view: &masked_claims::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"phase\":\"{}\",\"active_seat\":{},\"turn_index\":{},\"claimant\":\"{}\",\"hand_counts\":{},\"pedestal\":{},\"veiled_gallery\":[{},{}],\"exposed_rows\":[{},{}],\"scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"counters\":[{},{}],\"terminal\":{},\"terminal_rationale\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.phase.as_str(),
        option_masked_seat_json(view.active_seat),
        view.turn_index,
        view.claimant.as_str(),
        masked_counts_json(view.hand_counts.seat_0, view.hand_counts.seat_1),
        view.pedestal
            .map_or_else(|| "null".to_owned(), masked_pedestal_json),
        masked_veiled_json(&view.veiled_gallery[0]),
        masked_veiled_json(&view.veiled_gallery[1]),
        masked_exposed_json(&view.exposed_rows[0]),
        masked_exposed_json(&view.exposed_rows[1]),
        view.scores[0],
        view.scores[1],
        masked_counter_json(view.counters[0]),
        masked_counter_json(view.counters[1]),
        masked_terminal_json(&view.terminal),
        masked_terminal_rationale(&view.terminal)
            .map_or_else(|| "null".to_owned(), masked_outcome_rationale_json),
        view.freshness_token.0,
        masked_private_view_json(&view.private_view),
        masked_ui_json(&view.ui)
    )
}

fn flood_view_json(view: &flood_watch::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"seats\":[{}],\"roles\":[{}],\"turn_number\":{},\"active_seat\":\"{}\",\"phase\":{},\"districts\":[{}],\"drawn_cards\":[{}],\"forecast\":{},\"remaining_composition\":{},\"undrawn_count\":{},\"terminal\":{},\"freshness_token\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        string_array(&view.seats),
        view.roles
            .iter()
            .map(flood_role_json)
            .collect::<Vec<_>>()
            .join(","),
        view.turn_number,
        escape_json(&view.active_seat),
        flood_phase_json(view.phase),
        view.districts
            .iter()
            .map(flood_district_json)
            .collect::<Vec<_>>()
            .join(","),
        view.drawn_cards
            .iter()
            .map(flood_card_face_json)
            .collect::<Vec<_>>()
            .join(","),
        option_flood_card_face_json(view.forecast.as_ref()),
        flood_composition_json(&view.remaining_composition),
        view.undrawn_count,
        flood_terminal_json(&view.terminal),
        view.freshness_token,
        flood_ui_json(&view.ui)
    )
}

fn option_flood_card_face_json(card: Option<&flood_watch::CardFaceView>) -> String {
    card.map(flood_card_face_json)
        .unwrap_or_else(|| "null".to_owned())
}

fn flood_card_face_json(card: &flood_watch::CardFaceView) -> String {
    format!(
        "{{\"id\":\"{}\",\"label\":\"{}\",\"summary\":\"{}\",\"details\":{},\"family\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.id),
        escape_json(&card.label),
        escape_json(&card.summary),
        option_string_json(card.details.as_deref()),
        escape_json(&card.family),
        escape_json(&card.accessibility_label)
    )
}

fn frontier_view_json(view: &frontier_control::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"seats\":[{}],\"factions\":[{}],\"round_number\":{},\"active_faction\":\"{}\",\"active_seat\":{},\"phase\":{},\"sites\":[{}],\"scores\":{},\"terminal\":{},\"freshness_token\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        string_array(&view.seats),
        view.factions
            .iter()
            .map(frontier_faction_view_json)
            .collect::<Vec<_>>()
            .join(","),
        view.round_number,
        view.active_faction.as_str(),
        option_string_json(view.active_seat.as_deref()),
        frontier_phase_json(view.phase),
        view.sites
            .iter()
            .map(frontier_site_json)
            .collect::<Vec<_>>()
            .join(","),
        frontier_score_json(&view.scores),
        frontier_terminal_json(&view.terminal),
        view.freshness_token
    )
}

fn frontier_faction_view_json(faction: &frontier_control::FactionView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"faction\":\"{}\",\"label\":\"{}\"}}",
        escape_json(&faction.seat),
        faction.faction.as_str(),
        escape_json(&faction.label)
    )
}

fn frontier_phase_json(phase: frontier_control::PhaseView) -> String {
    match phase {
        frontier_control::PhaseView::Action { budget_remaining } => {
            format!("{{\"kind\":\"action\",\"budget_remaining\":{budget_remaining}}}")
        }
        frontier_control::PhaseView::Terminal => {
            "{\"kind\":\"terminal\",\"budget_remaining\":0}".to_owned()
        }
    }
}

fn frontier_site_json(site: &frontier_control::SiteView) -> String {
    format!(
        "{{\"site\":\"{}\",\"label\":\"{}\",\"guards\":{},\"crews\":{},\"stake\":{},\"fort\":{},\"stake_value\":{},\"supplied\":{}}}",
        site.site.as_str(),
        escape_json(site.label),
        site.guards,
        site.crews,
        site.stake,
        site.fort,
        site.stake_value,
        option_bool_json(site.supplied)
    )
}

fn frontier_score_json(score: &frontier_control::ScoreView) -> String {
    format!(
        "{{\"garrison\":{},\"prospectors\":{}}}",
        score.garrison, score.prospectors
    )
}

fn frontier_terminal_json(terminal: &frontier_control::TerminalView) -> String {
    match terminal {
        frontier_control::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"winner\":null}".to_owned()
        }
        frontier_control::TerminalView::Winner {
            faction,
            scores,
            garrison_tiebreak,
            summary,
        } => format!(
            "{{\"kind\":\"winner\",\"winner\":\"{}\",\"scores\":{},\"garrison_tiebreak\":{},\"summary\":\"{}\"}}",
            faction.as_str(),
            frontier_score_json(scores),
            garrison_tiebreak,
            escape_json(summary)
        ),
    }
}

fn event_frontier_view_json(view: &event_frontier::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"seats\":[{}],\"factions\":[{}],\"active_seat\":{},\"sites\":[{}],\"adjacency\":[{}],\"resources\":{},\"scores\":{},\"eligibility\":[{}],\"current_card\":{},\"next_public_card\":{},\"discard\":[{}],\"active_edicts\":[{}],\"epoch\":{},\"reckoning_count\":{},\"victory_distance\":{},\"terminal\":{},\"terminal_rationale\":{},\"ui\":{},\"freshness_token\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(GAME_EVENT_FRONTIER_DISPLAY_NAME),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        string_array(&view.seats),
        string_array(&view.factions),
        option_string_json(view.active_seat.as_deref()),
        view.sites
            .iter()
            .map(event_frontier_site_json)
            .collect::<Vec<_>>()
            .join(","),
        view.adjacency
            .iter()
            .map(event_frontier_adjacency_json)
            .collect::<Vec<_>>()
            .join(","),
        event_frontier_resources_json(&view.resources),
        event_frontier_scores_json(&view.scores),
        view.eligibility
            .iter()
            .map(event_frontier_eligibility_json)
            .collect::<Vec<_>>()
            .join(","),
        option_event_frontier_card_face_json(view.current_card.as_ref()),
        option_event_frontier_card_face_json(view.next_public_card.as_ref()),
        view.discard
            .iter()
            .map(event_frontier_card_face_json)
            .collect::<Vec<_>>()
            .join(","),
        string_array(&view.active_edicts),
        view.epoch,
        view.reckoning_count,
        event_frontier_victory_distance_json(&view.victory_distance),
        event_frontier_terminal_json(&view.terminal),
        event_frontier_terminal_rationale_json(view),
        event_frontier_ui_json(&view.ui),
        view.freshness_token
    )
}

fn option_event_frontier_card_face_json(card: Option<&event_frontier::CardFaceView>) -> String {
    card.map(event_frontier_card_face_json)
        .unwrap_or_else(|| "null".to_owned())
}

fn event_frontier_card_face_json(card: &event_frontier::CardFaceView) -> String {
    format!(
        "{{\"id\":\"{}\",\"label\":\"{}\",\"summary\":\"{}\",\"details\":{},\"family\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.id),
        escape_json(&card.label),
        escape_json(&card.summary),
        option_string_json(card.details.as_deref()),
        escape_json(&card.family),
        escape_json(&card.accessibility_label)
    )
}

fn event_frontier_ui_json(ui: &event_frontier::UiMetadata) -> String {
    format!(
        "{{\"table_label\":\"{}\",\"event_deck_label\":\"{}\",\"current_card_label\":\"{}\",\"next_card_label\":\"{}\",\"discard_label\":\"{}\",\"face_down_label\":\"{}\",\"face_down_summary\":\"{}\",\"reduced_motion_token\":\"{}\",\"seat_labels\":[{}],\"faction_labels\":[{}],\"action_affordance_templates\":[{}]}}",
        escape_json(&ui.table_label),
        escape_json(&ui.event_deck_label),
        escape_json(&ui.current_card_label),
        escape_json(&ui.next_card_label),
        escape_json(&ui.discard_label),
        escape_json(&ui.face_down_label),
        escape_json(&ui.face_down_summary),
        escape_json(&ui.reduced_motion_token),
        ui.seat_labels
            .iter()
            .map(event_frontier_seat_display_label_json)
            .collect::<Vec<_>>()
            .join(","),
        ui.faction_labels
            .iter()
            .map(event_frontier_faction_display_label_json)
            .collect::<Vec<_>>()
            .join(","),
        ui.action_affordance_templates
            .iter()
            .map(event_frontier_action_affordance_template_json)
            .collect::<Vec<_>>()
            .join(",")
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

fn event_frontier_seat_display_label_json(label: &event_frontier::ui::SeatDisplayLabel) -> String {
    format!(
        "{{\"seat\":\"{}\",\"label\":\"{}\"}}",
        escape_json(&label.seat),
        escape_json(&label.label)
    )
}

fn event_frontier_faction_display_label_json(
    label: &event_frontier::ui::FactionDisplayLabel,
) -> String {
    format!(
        "{{\"faction\":\"{}\",\"label\":\"{}\"}}",
        escape_json(&label.faction),
        escape_json(&label.label)
    )
}

fn event_frontier_action_affordance_template_json(
    template: &event_frontier::ActionAffordanceTemplate,
) -> String {
    format!(
        "{{\"id\":\"{}\",\"text\":\"{}\"}}",
        escape_json(&template.id),
        escape_json(&template.text)
    )
}

fn event_frontier_site_json(site: &event_frontier::visibility::SiteView) -> String {
    format!(
        "{{\"site\":\"{}\",\"label\":\"{}\",\"agents\":{},\"settlers\":{},\"depot\":{},\"cache_count\":{}}}",
        site.site.as_str(),
        escape_json(&site.label),
        site.agents,
        site.settlers,
        site.depot,
        site.cache_count
    )
}

fn event_frontier_adjacency_json(
    (site, neighbors): &(event_frontier::SiteId, Vec<event_frontier::SiteId>),
) -> String {
    format!(
        "{{\"site\":\"{}\",\"neighbors\":[{}]}}",
        site.as_str(),
        neighbors
            .iter()
            .map(|neighbor| format!("\"{}\"", neighbor.as_str()))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn event_frontier_resources_json(resources: &event_frontier::visibility::ResourceView) -> String {
    format!(
        "{{\"funds\":{},\"provisions\":{}}}",
        resources.funds, resources.provisions
    )
}

fn event_frontier_scores_json(scores: &event_frontier::visibility::ScoreView) -> String {
    format!(
        "{{\"charter\":{},\"freeholders\":{}}}",
        scores.charter, scores.freeholders
    )
}

fn event_frontier_eligibility_json(
    (faction, eligibility): &(EventFrontierFactionId, event_frontier::Eligibility),
) -> String {
    format!(
        "{{\"faction\":\"{}\",\"eligible\":\"{}\"}}",
        faction.as_str(),
        eligibility.as_str()
    )
}

fn event_frontier_victory_distance_json(
    distance: &event_frontier::visibility::VictoryDistanceView,
) -> String {
    format!(
        "{{\"charter_sites_needed\":{},\"freeholder_caches_needed\":{}}}",
        distance.charter_sites_needed, distance.freeholder_caches_needed
    )
}

fn event_frontier_terminal_json(terminal: &event_frontier::visibility::TerminalView) -> String {
    match terminal {
        event_frontier::visibility::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"winner\":null}".to_owned()
        }
        event_frontier::visibility::TerminalView::Complete {
            winner,
            victory_type,
            scores,
            decisive_rule,
        } => format!(
            "{{\"kind\":\"winner\",\"winner\":\"{}\",\"victory_type\":\"{}\",\"scores\":{},\"decisive_rule\":\"{}\"}}",
            winner.as_str(),
            escape_json(victory_type),
            event_frontier_scores_json(scores),
            escape_json(decisive_rule)
        ),
    }
}

fn event_frontier_terminal_rationale_json(view: &event_frontier::PublicView) -> String {
    let event_frontier::visibility::TerminalView::Complete {
        winner,
        victory_type,
        scores,
        decisive_rule,
    } = &view.terminal
    else {
        return "null".to_owned();
    };
    let cause = if decisive_rule == "EF-END-003" {
        "both_met_freeholder"
    } else if victory_type == "charter_instant" {
        "charter_instant"
    } else if victory_type == "freeholder_instant" {
        "freeholder_instant"
    } else if scores.charter == scores.freeholders {
        "final_fallback_tiebreak"
    } else {
        "final_fallback_score"
    };
    let template_key = match cause {
        "charter_instant" => "event_frontier.charter_instant",
        "freeholder_instant" => "event_frontier.freeholder_instant",
        "both_met_freeholder" => "event_frontier.both_met_freeholder",
        "final_fallback_tiebreak" => "event_frontier.final_fallback_tiebreak",
        _ => "event_frontier.final_fallback_score",
    };
    format!(
        "{{\"result_kind\":\"win\",\"decisive_cause\":\"{}\",\"template_key\":\"{}\",\"template_params\":{{\"winner\":\"{}\",\"charter_score\":{},\"freeholder_score\":{}}},\"decisive_rule_ids\":[\"{}\"],\"final_standing\":[{{\"seat\":\"faction_charter\",\"label\":\"Charter\",\"result\":\"{}\",\"emphasized\":{},\"values\":[{{\"label\":\"Score\",\"value\":{}}}]}},{{\"seat\":\"faction_freeholders\",\"label\":\"Freeholders\",\"result\":\"{}\",\"emphasized\":{},\"values\":[{{\"label\":\"Score\",\"value\":{}}}]}}],\"breakdown_sections\":[{{\"id\":\"event-frontier-terminal\",\"heading\":\"Rust terminal cause\",\"rows\":[{{\"label\":\"Victory type\",\"value\":\"{}\"}},{{\"label\":\"Decisive rule\",\"value\":\"{}\"}},{{\"label\":\"Reckonings\",\"value\":{}}}]}}]}}",
        cause,
        template_key,
        escape_json(winner.as_str()),
        scores.charter,
        scores.freeholders,
        escape_json(decisive_rule),
        if *winner == EventFrontierFactionId::Charter { "win" } else { "loss" },
        *winner == EventFrontierFactionId::Charter,
        scores.charter,
        if *winner == EventFrontierFactionId::Freeholders { "win" } else { "loss" },
        *winner == EventFrontierFactionId::Freeholders,
        scores.freeholders,
        escape_json(victory_type),
        escape_json(decisive_rule),
        view.reckoning_count
    )
}

fn flood_role_json(role: &flood_watch::visibility::RoleView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"role\":\"{}\",\"label\":\"{}\"}}",
        escape_json(&role.seat),
        role.role.as_str(),
        escape_json(&role.label)
    )
}

fn flood_phase_json(phase: flood_watch::visibility::PhaseView) -> String {
    match phase {
        flood_watch::visibility::PhaseView::Action { budget_remaining } => {
            format!("{{\"kind\":\"action\",\"budget_remaining\":{budget_remaining}}}")
        }
        flood_watch::visibility::PhaseView::Terminal => {
            "{\"kind\":\"terminal\",\"budget_remaining\":0}".to_owned()
        }
    }
}

fn flood_district_json(district: &flood_watch::DistrictView) -> String {
    format!(
        "{{\"district\":\"{}\",\"label\":\"{}\",\"flood_level\":{},\"levees\":{}}}",
        district.district.as_str(),
        escape_json(district.label),
        district.flood_level,
        district.levees
    )
}

fn flood_composition_json(composition: &flood_watch::CompositionView) -> String {
    format!(
        "{{\"downpours_per_district\":[{}],\"surges_per_district\":[{}],\"reprieves\":{}}}",
        composition
            .downpours_per_district
            .iter()
            .map(flood_composition_entry_json)
            .collect::<Vec<_>>()
            .join(","),
        composition
            .surges_per_district
            .iter()
            .map(flood_composition_entry_json)
            .collect::<Vec<_>>()
            .join(","),
        composition.reprieves
    )
}

fn flood_composition_entry_json(entry: &(flood_watch::DistrictId, u8)) -> String {
    format!(
        "{{\"district\":\"{}\",\"count\":{}}}",
        entry.0.as_str(),
        entry.1
    )
}

fn flood_terminal_json(terminal: &flood_watch::TerminalView) -> String {
    match terminal {
        flood_watch::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"outcome\":null,\"summary\":null}".to_owned()
        }
        flood_watch::TerminalView::Complete { outcome, summary } => format!(
            "{{\"kind\":\"complete\",\"outcome\":\"{}\",\"summary\":{}}}",
            escape_json(outcome),
            flood_terminal_summary_json(summary)
        ),
    }
}

fn flood_terminal_summary_json(summary: &flood_watch::TerminalSummary) -> String {
    format!(
        "{{\"rule_id\":\"{}\",\"public_summary\":\"{}\",\"drawn_card_count\":{},\"surviving_levels\":[{}]}}",
        escape_json(&summary.rule_id),
        escape_json(&summary.public_summary),
        summary.drawn_card_count,
        summary
            .surviving_levels
            .iter()
            .map(flood_composition_entry_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn flood_ui_json(ui: &flood_watch::ui::UiMetadata) -> String {
    format!(
        "{{\"display_name\":\"{}\",\"event_deck_label\":\"{}\",\"forecast_label\":\"{}\",\"drawn_label\":\"{}\",\"face_down_label\":\"{}\",\"face_down_summary\":\"{}\",\"reduced_motion_token\":\"{}\"}}",
        escape_json(&ui.display_name),
        escape_json(&ui.event_deck_label),
        escape_json(&ui.forecast_label),
        escape_json(&ui.drawn_label),
        escape_json(&ui.face_down_label),
        escape_json(&ui.face_down_summary),
        escape_json(&ui.reduced_motion_token)
    )
}

fn option_masked_seat_json(seat: Option<MaskedClaimsSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

fn masked_counts_json(seat_0: u8, seat_1: u8) -> String {
    format!("{{\"seat_0\":{},\"seat_1\":{}}}", seat_0, seat_1)
}

fn masked_pedestal_json(pedestal: masked_claims::PedestalView) -> String {
    format!(
        "{{\"claimant\":\"{}\",\"declared_grade\":\"{}\",\"declared_label\":\"{}\"}}",
        pedestal.claimant.as_str(),
        pedestal.declared_grade.as_str(),
        pedestal.declared_grade.label()
    )
}

fn masked_veiled_json(veiled: &[masked_claims::VeiledClaimView]) -> String {
    let body = veiled
        .iter()
        .map(|claim| {
            format!(
                "{{\"declared_grade\":\"{}\",\"declared_label\":\"{}\"}}",
                claim.declared_grade.as_str(),
                claim.declared_grade.label()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn masked_exposed_json(exposed: &[masked_claims::ExposedMaskView]) -> String {
    let body = exposed
        .iter()
        .map(|mask| {
            format!(
                "{{\"tile_id\":\"{}\",\"actual_grade\":\"{}\",\"declared_grade\":\"{}\",\"claimant\":\"{}\",\"challenger\":\"{}\"}}",
                escape_json(&mask.tile_id),
                mask.actual_grade.as_str(),
                mask.declared_grade.as_str(),
                mask.claimant.as_str(),
                mask.challenger.as_str()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn masked_counter_json(counter: masked_claims::CounterView) -> String {
    format!(
        "{{\"exposed_lies\":{},\"successful_challenges\":{},\"challenges_declared\":{}}}",
        counter.exposed_lies, counter.successful_challenges, counter.challenges_declared
    )
}

fn masked_terminal_json(terminal: &masked_claims::TerminalView) -> String {
    match terminal {
        masked_claims::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"winner\":null,\"draw\":false}".to_owned()
        }
        masked_claims::TerminalView::Complete { outcome, .. } => {
            masked_terminal_outcome_json(*outcome)
        }
    }
}

fn masked_terminal_rationale(
    terminal: &masked_claims::TerminalView,
) -> Option<&masked_claims::OutcomeRationaleView> {
    match terminal {
        masked_claims::TerminalView::NonTerminal => None,
        masked_claims::TerminalView::Complete { rationale, .. } => Some(rationale),
    }
}

fn masked_outcome_rationale_json(rationale: &masked_claims::OutcomeRationaleView) -> String {
    format!(
        "{{\"result_kind\":\"{}\",\"decisive_cause\":\"{}\",\"decisive_rule_ids\":[{}],\"final_scores\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
        escape_json(&rationale.result_kind),
        escape_json(&rationale.decisive_cause),
        string_array(&rationale.decisive_rule_ids),
        rationale.final_scores[0],
        rationale.final_scores[1]
    )
}

fn masked_private_view_json(private: &masked_claims::PrivateView) -> String {
    match private {
        masked_claims::PrivateView::Observer => {
            "{\"status\":\"observer\",\"own_hand\":[]}".to_owned()
        }
        masked_claims::PrivateView::Seat(view) => format!(
            "{{\"status\":\"seat\",\"seat\":\"{}\",\"own_hand\":[{}]}}",
            view.seat.as_str(),
            view.own_hand
                .iter()
                .map(masked_mask_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

fn masked_mask_json(mask: &masked_claims::MaskView) -> String {
    format!(
        "{{\"tile_id\":\"{}\",\"grade\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&mask.tile_id),
        mask.grade.as_str(),
        escape_json(&mask.label),
        escape_json(&mask.accessibility_label)
    )
}

fn masked_ui_json(ui: &masked_claims::UiMetadata) -> String {
    format!(
        "{{\"game_id\":\"{}\",\"variant_id\":\"{}\",\"display_name\":\"{}\",\"grade_labels\":[{}],\"claim_preview_template\":\"{}\",\"reaction_prompt_template\":\"{}\"}}",
        escape_json(&ui.game_id),
        escape_json(&ui.variant_id),
        escape_json(&ui.display_name),
        string_array(&ui.grade_labels),
        escape_json(&ui.claim_preview_template),
        escape_json(&ui.reaction_prompt_template)
    )
}

fn masked_terminal_outcome_json(outcome: masked_claims::TerminalOutcome) -> String {
    match outcome {
        masked_claims::TerminalOutcome::ScoreWin { winner, scores } => format!(
            "{{\"kind\":\"score_win\",\"winner\":\"{}\",\"draw\":false,\"scores\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            winner.as_str(), scores[0], scores[1]
        ),
        masked_claims::TerminalOutcome::TiebreakWin {
            winner,
            scores,
            tiebreak,
        } => format!(
            "{{\"kind\":\"tiebreak_win\",\"winner\":\"{}\",\"draw\":false,\"tiebreak\":\"{}\",\"scores\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            winner.as_str(),
            escape_json(tiebreak),
            scores[0],
            scores[1]
        ),
        masked_claims::TerminalOutcome::Draw { scores } => format!(
            "{{\"kind\":\"draw\",\"winner\":null,\"draw\":true,\"scores\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            scores[0], scores[1]
        ),
    }
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

fn secret_view_json(view: &secret_draft::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"round_number\":{},\"round_limit\":{},\"phase\":\"{}\",\"active_seat\":{},\"priority_seat\":\"{}\",\"visible_pool\":[{}],\"drafted\":{},\"commitments\":{},\"scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"revealed_history\":[{}],\"terminal\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.round_number,
        view.round_limit,
        view.phase.as_str(),
        secret_active_seat_json(view),
        view.priority_seat.as_str(),
        view.visible_pool
            .iter()
            .map(secret_item_json)
            .collect::<Vec<_>>()
            .join(","),
        secret_drafted_json(&view.drafted),
        secret_commitments_json(&view.commitments),
        view.scores[0],
        view.scores[1],
        view.revealed_history
            .iter()
            .map(secret_revealed_round_json)
            .collect::<Vec<_>>()
            .join(","),
        secret_terminal_json(&view.terminal),
        view.freshness_token.0,
        secret_private_view_json(&view.private_view),
        secret_ui_json(&view.ui)
    )
}

fn secret_active_seat_json(view: &secret_draft::PublicView) -> String {
    if !matches!(
        view.terminal,
        secret_draft::visibility::TerminalView::NonTerminal
    ) {
        return "null".to_owned();
    }
    if !view.commitments.seat_0.committed {
        return "\"seat_0\"".to_owned();
    }
    if !view.commitments.seat_1.committed {
        return "\"seat_1\"".to_owned();
    }
    "null".to_owned()
}

fn secret_item_json(item: &secret_draft::visibility::DraftItemView) -> String {
    format!(
        "{{\"item_id\":\"{}\",\"label\":\"{}\",\"thread\":\"{}\",\"value\":{},\"accessibility_label\":\"{}\"}}",
        escape_json(&item.item_id),
        escape_json(&item.label),
        escape_json(&item.thread),
        item.value,
        escape_json(&item.accessibility_label)
    )
}

fn secret_drafted_json(drafted: &secret_draft::visibility::DraftedCollectionsView) -> String {
    format!(
        "{{\"seat_0\":[{}],\"seat_1\":[{}]}}",
        drafted
            .seat_0
            .iter()
            .map(secret_item_json)
            .collect::<Vec<_>>()
            .join(","),
        drafted
            .seat_1
            .iter()
            .map(secret_item_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn secret_commitments_json(commitments: &secret_draft::visibility::CommitmentViews) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{},\"copy\":\"{}\"}}",
        secret_commitment_json(&commitments.seat_0),
        secret_commitment_json(&commitments.seat_1),
        escape_json(&commitments.copy)
    )
}

fn secret_commitment_json(commitment: &secret_draft::visibility::CommitmentView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"committed\":{},\"status\":\"{}\",\"accessibility_label\":\"{}\"}}",
        commitment.seat.as_str(),
        commitment.committed,
        escape_json(&commitment.status),
        escape_json(&commitment.accessibility_label)
    )
}

fn secret_revealed_round_json(round: &secret_draft::visibility::RevealedRoundView) -> String {
    format!(
        "{{\"round_number\":{},\"seat_0_choice\":{},\"seat_1_choice\":{},\"seat_0_award\":{},\"seat_1_award\":{},\"priority_seat\":\"{}\",\"contested\":{}}}",
        round.round_number,
        secret_item_json(&round.seat_0_choice),
        secret_item_json(&round.seat_1_choice),
        secret_item_json(&round.seat_0_award),
        secret_item_json(&round.seat_1_award),
        round.priority_seat.as_str(),
        round.contested
    )
}

fn secret_terminal_json(terminal: &secret_draft::visibility::TerminalView) -> String {
    match terminal {
        secret_draft::visibility::TerminalView::NonTerminal => {
            "{\"terminal\":false,\"winner\":null,\"draw\":false}".to_owned()
        }
        secret_draft::visibility::TerminalView::Win { winning_seat, .. } => format!(
            "{{\"terminal\":true,\"winner\":\"{}\",\"draw\":false}}",
            winning_seat.as_str()
        ),
        secret_draft::visibility::TerminalView::Draw { .. } => {
            "{\"terminal\":true,\"winner\":null,\"draw\":true}".to_owned()
        }
    }
}

fn secret_private_view_json(private_view: &secret_draft::visibility::PrivateView) -> String {
    match private_view {
        secret_draft::visibility::PrivateView::Observer => {
            "{\"status\":\"observer\",\"own_committed\":false,\"waiting_copy\":\"\"}".to_owned()
        }
        secret_draft::visibility::PrivateView::Seat {
            seat,
            own_committed,
            waiting_copy,
        } => format!(
            "{{\"status\":\"seat\",\"seat\":\"{}\",\"own_committed\":{},\"waiting_copy\":\"{}\"}}",
            seat.as_str(),
            own_committed,
            escape_json(waiting_copy)
        ),
    }
}

fn secret_ui_json(ui: &secret_draft::UiMetadata) -> String {
    format!(
        "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"table_label\":\"{}\",\"visible_pool_label\":\"{}\",\"drafted_label\":\"{}\",\"pending_label\":\"{}\",\"score_label\":\"{}\",\"reveal_group_token\":\"{}\",\"reduced_motion_token\":\"{}\"}}",
        escape_json(ui.game_id),
        escape_json(ui.display_name),
        escape_json(&ui.table_label),
        escape_json(&ui.visible_pool_label),
        escape_json(&ui.drafted_label),
        escape_json(&ui.pending_label),
        escape_json(&ui.score_label),
        escape_json(&ui.reveal_group_token),
        escape_json(&ui.reduced_motion_token)
    )
}

fn event_frontier_effect_json(effect: &EffectEnvelope<EventFrontierEffect>) -> String {
    let payload = match &effect.payload {
        EventFrontierEffect::EventResolved { card, summary } => format!(
            "{{\"type\":\"event_resolved\",\"card\":\"{}\",\"summary\":\"{}\"}}",
            card.as_str(),
            escape_json(summary)
        ),
        EventFrontierEffect::EdictActivated { card, edict } => format!(
            "{{\"type\":\"edict_activated\",\"card\":\"{}\",\"edict\":\"{}\"}}",
            card.as_str(),
            escape_json(edict)
        ),
        EventFrontierEffect::EdictExpired { edict } => format!(
            "{{\"type\":\"edict_expired\",\"edict\":\"{}\"}}",
            escape_json(edict)
        ),
        EventFrontierEffect::CardRevealed { card, next_public } => format!(
            "{{\"type\":\"card_revealed\",\"card\":\"{}\",\"next_public\":{}}}",
            card.as_str(),
            option_string_json(next_public.map(|card| card.as_str()))
        ),
        EventFrontierEffect::ChoiceTaken { faction, choice } => format!(
            "{{\"type\":\"choice_taken\",\"faction\":\"{}\",\"choice\":\"{}\"}}",
            faction.as_str(),
            escape_json(choice)
        ),
        EventFrontierEffect::CardDiscarded { card, reason } => format!(
            "{{\"type\":\"card_discarded\",\"card\":\"{}\",\"reason\":\"{}\"}}",
            card.as_str(),
            escape_json(event_frontier_reason_label(reason))
        ),
        EventFrontierEffect::EligibilityChanged {
            faction,
            eligible,
            reason,
        } => format!(
            "{{\"type\":\"eligibility_changed\",\"faction\":\"{}\",\"eligible\":{},\"reason\":\"{}\"}}",
            faction.as_str(),
            eligible,
            escape_json(event_frontier_reason_label(reason))
        ),
        EventFrontierEffect::ResourcesChanged {
            faction,
            previous,
            new,
            reason,
        } => format!(
            "{{\"type\":\"resources_changed\",\"faction\":\"{}\",\"previous\":{},\"new\":{},\"reason\":\"{}\"}}",
            faction.as_str(),
            previous,
            new,
            escape_json(event_frontier_reason_label(reason))
        ),
        EventFrontierEffect::OpResolved { faction, op, sites } => format!(
            "{{\"type\":\"op_resolved\",\"faction\":\"{}\",\"op\":\"{}\",\"sites\":[{}]}}",
            faction.as_str(),
            escape_json(op),
            sites
                .iter()
                .map(|site| format!("\"{}\"", site.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        EventFrontierEffect::AgentPlaced { site, new_count } => format!(
            "{{\"type\":\"agent_placed\",\"site\":\"{}\",\"new_count\":{}}}",
            site.as_str(),
            new_count
        ),
        EventFrontierEffect::AgentRemoved { site, new_count } => format!(
            "{{\"type\":\"agent_removed\",\"site\":\"{}\",\"new_count\":{}}}",
            site.as_str(),
            new_count
        ),
        EventFrontierEffect::DepotBuilt { site } => format!(
            "{{\"type\":\"depot_built\",\"site\":\"{}\"}}",
            site.as_str()
        ),
        EventFrontierEffect::CacheRemoved { site, new_count } => format!(
            "{{\"type\":\"cache_removed\",\"site\":\"{}\",\"new_count\":{}}}",
            site.as_str(),
            new_count
        ),
        EventFrontierEffect::SettlerMoved {
            from,
            to,
            from_count,
            to_count,
        } => format!(
            "{{\"type\":\"settler_moved\",\"from\":\"{}\",\"to\":\"{}\",\"from_count\":{},\"to_count\":{}}}",
            from.as_str(),
            to.as_str(),
            from_count,
            to_count
        ),
        EventFrontierEffect::CacheLaid { site, new_count } => format!(
            "{{\"type\":\"cache_laid\",\"site\":\"{}\",\"new_count\":{}}}",
            site.as_str(),
            new_count
        ),
        EventFrontierEffect::SettlerRallied { site, new_count } => format!(
            "{{\"type\":\"settler_rallied\",\"site\":\"{}\",\"new_count\":{}}}",
            site.as_str(),
            new_count
        ),
        EventFrontierEffect::ReckoningResolved {
            round,
            victory_check,
            site_breakdown,
            income,
            expired_edicts,
        } => format!(
            "{{\"type\":\"reckoning_resolved\",\"round\":{},\"victory_check\":\"{}\",\"site_breakdown\":[{}],\"income\":{{\"funds\":{},\"provisions\":{}}},\"expired_edicts\":[{}]}}",
            round,
            escape_json(victory_check),
            site_breakdown
                .iter()
                .map(event_frontier_site_score_json)
                .collect::<Vec<_>>()
                .join(","),
            income.0,
            income.1,
            expired_edicts
                .iter()
                .map(|edict| format!("\"{}\"", escape_json(edict)))
                .collect::<Vec<_>>()
                .join(",")
        ),
        EventFrontierEffect::Terminal {
            winner,
            victory_type,
            totals,
            summary,
        } => format!(
            "{{\"type\":\"terminal\",\"winner\":\"{}\",\"victory_type\":\"{}\",\"totals\":{{\"charter\":{},\"freeholders\":{}}},\"summary\":\"{}\"}}",
            winner.as_str(),
            escape_json(victory_type),
            totals.0,
            totals.1,
            escape_json(summary)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn event_frontier_site_score_json(breakdown: &event_frontier::SiteScoreBreakdown) -> String {
    format!(
        "{{\"site\":\"{}\",\"charter_presence\":{},\"freeholder_presence\":{},\"awarded_to\":{}}}",
        breakdown.site.as_str(),
        breakdown.charter_presence,
        breakdown.freeholder_presence,
        option_string_json(breakdown.awarded_to.map(|faction| faction.as_str()))
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

fn masked_effects_json(effects: &[EffectEnvelope<MaskedClaimsEffect>], viewer: &Viewer) -> String {
    let body = masked_claims::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(masked_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn masked_effect_json(effect: &EffectEnvelope<MaskedClaimsEffect>) -> String {
    let payload = match &effect.payload {
        MaskedClaimsEffect::ClaimPlaced {
            turn,
            claimant,
            declared_grade,
            ..
        } => format!(
            "{{\"type\":\"claim_placed\",\"turn\":{},\"claimant\":\"{}\",\"declared_grade\":\"{}\"}}",
            turn,
            claimant.as_str(),
            declared_grade.as_str()
        ),
        MaskedClaimsEffect::ReactionWindowOpened {
            turn,
            responder,
            declared_grade,
            choices,
            ..
        } => format!(
            "{{\"type\":\"reaction_window_opened\",\"turn\":{},\"responder\":\"{}\",\"declared_grade\":\"{}\",\"choices\":[{}]}}",
            turn,
            responder.as_str(),
            declared_grade.as_str(),
            string_array(choices)
        ),
        MaskedClaimsEffect::ClaimAccepted {
            turn,
            claimant,
            declared_grade,
            score_delta,
            ..
        } => format!(
            "{{\"type\":\"claim_accepted\",\"turn\":{},\"claimant\":\"{}\",\"declared_grade\":\"{}\",\"score_delta\":{}}}",
            turn,
            claimant.as_str(),
            declared_grade.as_str(),
            score_delta
        ),
        MaskedClaimsEffect::ChallengeDeclared {
            turn, responder, ..
        } => format!(
            "{{\"type\":\"challenge_declared\",\"turn\":{},\"responder\":\"{}\"}}",
            turn,
            responder.as_str()
        ),
        MaskedClaimsEffect::MaskRevealed {
            turn,
            tile_id,
            actual_grade,
            ..
        } => format!(
            "{{\"type\":\"mask_revealed\",\"turn\":{},\"tile_id\":\"{}\",\"actual_grade\":\"{}\"}}",
            turn,
            escape_json(tile_id),
            actual_grade.as_str()
        ),
        MaskedClaimsEffect::ChallengeResolved {
            turn,
            outcome,
            claimant,
            responder,
            claimant_award,
            responder_award,
            ..
        } => format!(
            "{{\"type\":\"challenge_resolved\",\"turn\":{},\"outcome\":\"{}\",\"claimant\":\"{}\",\"responder\":\"{}\",\"claimant_award\":{},\"responder_award\":{}}}",
            turn,
            outcome.as_str(),
            claimant.as_str(),
            responder.as_str(),
            claimant_award,
            responder_award
        ),
        MaskedClaimsEffect::ScoreChanged {
            seat,
            delta,
            total,
            reason,
        } => format!(
            "{{\"type\":\"claim_score_changed\",\"seat\":\"{}\",\"delta\":{},\"total\":{},\"reason\":\"{}\"}}",
            seat.as_str(),
            delta,
            total,
            escape_json(reason)
        ),
        MaskedClaimsEffect::TurnAdvanced { turn, claimant, .. } => format!(
            "{{\"type\":\"claim_turn_advanced\",\"turn\":{},\"claimant\":\"{}\"}}",
            turn,
            claimant.as_str()
        ),
        MaskedClaimsEffect::Terminal {
            outcome,
            final_scores,
            tiebreak_summary,
            ..
        } => format!(
            "{{\"type\":\"terminal\",\"outcome\":{},\"final_scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"tiebreak_summary\":\"{}\"}}",
            masked_terminal_outcome_json(*outcome),
            final_scores[0],
            final_scores[1],
            escape_json(tiebreak_summary)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn flood_effects_json(effects: &[EffectEnvelope<FloodWatchEffect>], viewer: &Viewer) -> String {
    let body = flood_watch::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(flood_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn flood_effect_json(effect: &EffectEnvelope<FloodWatchEffect>) -> String {
    let payload = match &effect.payload {
        FloodWatchEffect::DistrictBailed { district, amount } => format!(
            "{{\"type\":\"district_bailed\",\"district\":\"{}\",\"amount\":{}}}",
            district.as_str(),
            amount
        ),
        FloodWatchEffect::LeveePlaced { district, amount } => format!(
            "{{\"type\":\"levee_placed\",\"district\":\"{}\",\"amount\":{}}}",
            district.as_str(),
            amount
        ),
        FloodWatchEffect::ForecastRevealed { card } => format!(
            "{{\"type\":\"forecast_revealed\",\"card\":\"{}\"}}",
            escape_json(&card.id())
        ),
        FloodWatchEffect::EnvironmentPhaseBegan { turn, draws } => format!(
            "{{\"type\":\"environment_phase_began\",\"turn\":{},\"draws\":{}}}",
            turn,
            draws
        ),
        FloodWatchEffect::EventDrawn { index, card } => format!(
            "{{\"type\":\"event_drawn\",\"index\":{},\"card\":\"{}\"}}",
            index,
            escape_json(&card.id())
        ),
        FloodWatchEffect::LeveeAbsorbed {
            district,
            amount,
            remaining_levees,
        } => format!(
            "{{\"type\":\"levee_absorbed\",\"district\":\"{}\",\"amount\":{},\"remaining_levees\":{}}}",
            district.as_str(),
            amount,
            remaining_levees
        ),
        FloodWatchEffect::FloodLevelRose {
            district,
            amount,
            new_level,
        } => format!(
            "{{\"type\":\"flood_level_rose\",\"district\":\"{}\",\"amount\":{},\"new_level\":{}}}",
            district.as_str(),
            amount,
            new_level
        ),
        FloodWatchEffect::DistrictInundated { district } => format!(
            "{{\"type\":\"district_inundated\",\"district\":\"{}\"}}",
            district.as_str()
        ),
        FloodWatchEffect::DeckExhausted => "{\"type\":\"deck_exhausted\"}".to_owned(),
        FloodWatchEffect::Terminal { outcome, summary } => format!(
            "{{\"type\":\"terminal\",\"outcome\":\"{}\",\"summary\":{}}}",
            escape_json(outcome),
            flood_terminal_summary_json(summary)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn frontier_effects_json(
    effects: &[EffectEnvelope<FrontierControlEffect>],
    viewer: &Viewer,
) -> String {
    let body = frontier_control::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(frontier_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn frontier_effect_json(effect: &EffectEnvelope<FrontierControlEffect>) -> String {
    let payload = match &effect.payload {
        FrontierControlEffect::CrewMarched { from, to } => format!(
            "{{\"type\":\"crew_marched\",\"from\":\"{}\",\"to\":\"{}\"}}",
            from.as_str(),
            to.as_str()
        ),
        FrontierControlEffect::GuardPatrolled { from, to } => format!(
            "{{\"type\":\"guard_patrolled\",\"from\":\"{}\",\"to\":\"{}\"}}",
            from.as_str(),
            to.as_str()
        ),
        FrontierControlEffect::ClashResolved {
            site,
            guard_removed,
            crew_removed,
            entering_faction,
        } => format!(
            "{{\"type\":\"clash_resolved\",\"site\":\"{}\",\"guard_removed\":{},\"crew_removed\":{},\"entering_faction\":\"{}\"}}",
            site.as_str(),
            guard_removed,
            crew_removed,
            entering_faction.as_str()
        ),
        FrontierControlEffect::StakePlaced { site } => format!(
            "{{\"type\":\"stake_placed\",\"site\":\"{}\"}}",
            site.as_str()
        ),
        FrontierControlEffect::StakeDismantled { site } => format!(
            "{{\"type\":\"stake_dismantled\",\"site\":\"{}\"}}",
            site.as_str()
        ),
        FrontierControlEffect::CrewMustered { site, crews } => format!(
            "{{\"type\":\"crew_mustered\",\"site\":\"{}\",\"crews\":{}}}",
            site.as_str(),
            crews
        ),
        FrontierControlEffect::GuardReinforced { site, guards } => format!(
            "{{\"type\":\"guard_reinforced\",\"site\":\"{}\",\"guards\":{}}}",
            site.as_str(),
            guards
        ),
        FrontierControlEffect::TurnEnded { faction, round } => format!(
            "{{\"type\":\"turn_ended\",\"faction\":\"{}\",\"round\":{}}}",
            faction.as_str(),
            round
        ),
        FrontierControlEffect::RoundScored {
            round,
            garrison_points,
            prospector_points,
            fort_breakdown,
            stake_breakdown,
        } => format!(
            "{{\"type\":\"round_scored\",\"round\":{},\"garrison_points\":{},\"prospector_points\":{},\"fort_breakdown\":[{}],\"stake_breakdown\":[{}]}}",
            round,
            garrison_points,
            prospector_points,
            fort_breakdown
                .iter()
                .map(frontier_fort_score_json)
                .collect::<Vec<_>>()
                .join(","),
            stake_breakdown
                .iter()
                .map(frontier_stake_score_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
        FrontierControlEffect::Terminal {
            winner,
            garrison_total,
            prospector_total,
            tiebreak_applied,
            summary,
        } => format!(
            "{{\"type\":\"terminal\",\"winner\":\"{}\",\"garrison_total\":{},\"prospector_total\":{},\"tiebreak_applied\":{},\"summary\":\"{}\"}}",
            winner.as_str(),
            garrison_total,
            prospector_total,
            tiebreak_applied,
            escape_json(summary)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn frontier_fort_score_json(breakdown: &frontier_control::FortScoreBreakdown) -> String {
    format!(
        "{{\"site\":\"{}\",\"held\":{},\"points\":{}}}",
        breakdown.site.as_str(),
        breakdown.held,
        breakdown.points
    )
}

fn frontier_stake_score_json(breakdown: &frontier_control::StakeScoreBreakdown) -> String {
    format!(
        "{{\"site\":\"{}\",\"value\":{},\"supplied\":{},\"points\":{}}}",
        breakdown.site.as_str(),
        breakdown.value,
        breakdown.supplied,
        breakdown.points
    )
}

fn secret_effects_json(effects: &[EffectEnvelope<SecretDraftEffect>], viewer: &Viewer) -> String {
    let body = secret_draft::visibility::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(secret_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn secret_effect_json(effect: &EffectEnvelope<SecretDraftEffect>) -> String {
    let payload = match &effect.payload {
        SecretDraftEffect::CommitmentPlaced { seat, round } => format!(
            "{{\"type\":\"commitment_placed\",\"seat\":\"{}\",\"round\":{}}}",
            seat.as_str(),
            round
        ),
        SecretDraftEffect::OwnCommitAccepted { seat, round } => format!(
            "{{\"type\":\"own_commit_accepted\",\"seat\":\"{}\",\"round\":{}}}",
            seat.as_str(),
            round
        ),
        SecretDraftEffect::PendingSeatsChanged {
            round,
            seat_0_committed,
            seat_1_committed,
        } => format!(
            "{{\"type\":\"pending_seats_changed\",\"round\":{},\"seat_0_committed\":{},\"seat_1_committed\":{}}}",
            round, seat_0_committed, seat_1_committed
        ),
        SecretDraftEffect::RevealBatchStarted { round, group_id } => format!(
            "{{\"type\":\"reveal_batch_started\",\"round\":{},\"group_id\":\"{}\"}}",
            round,
            escape_json(group_id)
        ),
        SecretDraftEffect::ChoicesRevealed {
            round,
            seat_0_item,
            seat_1_item,
        } => format!(
            "{{\"type\":\"choices_revealed\",\"round\":{},\"seat_0_item\":\"{}\",\"seat_1_item\":\"{}\"}}",
            round,
            seat_0_item.as_str(),
            seat_1_item.as_str()
        ),
        SecretDraftEffect::DraftResolved {
            round,
            seat_0_award,
            seat_1_award,
            removed_items,
            conflict,
        } => format!(
            "{{\"type\":\"draft_resolved\",\"round\":{},\"seat_0_award\":\"{}\",\"seat_1_award\":\"{}\",\"removed_items\":[\"{}\",\"{}\"],\"conflict\":{}}}",
            round,
            seat_0_award.as_str(),
            seat_1_award.as_str(),
            removed_items[0].as_str(),
            removed_items[1].as_str(),
            conflict.map_or_else(|| "null".to_owned(), secret_conflict_json)
        ),
        SecretDraftEffect::PoolChanged { remaining_count } => format!(
            "{{\"type\":\"pool_changed\",\"remaining_count\":{}}}",
            remaining_count
        ),
        SecretDraftEffect::ScoreChanged {
            scores,
            tie_break_summary,
        } => format!(
            "{{\"type\":\"score_changed\",\"scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"tie_break_summary\":{}}}",
            scores[0],
            scores[1],
            secret_tie_break_json(*tie_break_summary)
        ),
        SecretDraftEffect::RoundAdvanced {
            next_round,
            priority_seat,
        } => format!(
            "{{\"type\":\"round_advanced\",\"next_round\":{},\"priority_seat\":\"{}\"}}",
            next_round,
            priority_seat.as_str()
        ),
        SecretDraftEffect::Terminal {
            outcome,
            final_scores,
            tie_break_summary,
        } => format!(
            "{{\"type\":\"terminal\",\"outcome\":{},\"final_scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"tie_break_summary\":{}}}",
            secret_terminal_outcome_json(*outcome),
            final_scores[0],
            final_scores[1],
            secret_tie_break_json(*tie_break_summary)
        ),
        SecretDraftEffect::PublicDiagnostic { code, message } => format!(
            "{{\"type\":\"public_diagnostic\",\"code\":\"{}\",\"message\":\"{}\"}}",
            escape_json(code),
            escape_json(message)
        ),
        SecretDraftEffect::PrivateDiagnostic {
            seat,
            code,
            message,
        } => format!(
            "{{\"type\":\"private_diagnostic\",\"seat\":\"{}\",\"code\":\"{}\",\"message\":\"{}\"}}",
            seat.as_str(),
            escape_json(code),
            escape_json(message)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn secret_conflict_json(conflict: secret_draft::effects::ConflictSummary) -> String {
    format!(
        "{{\"contested_item\":\"{}\",\"priority_seat\":\"{}\",\"fallback_item\":\"{}\"}}",
        conflict.contested_item.as_str(),
        conflict.priority_seat.as_str(),
        conflict.fallback_item.as_str()
    )
}

fn secret_tie_break_json(summary: secret_draft::effects::TieBreakSummary) -> String {
    format!(
        "{{\"scores\":[{},{}],\"complete_sets\":[{},{}],\"highest_single_values\":[{},{}],\"distinct_threads\":[{},{}],\"priority_conflict_wins\":[{},{}]}}",
        summary.scores[0],
        summary.scores[1],
        summary.complete_sets[0],
        summary.complete_sets[1],
        summary.highest_single_values[0],
        summary.highest_single_values[1],
        summary.distinct_threads[0],
        summary.distinct_threads[1],
        summary.priority_conflict_wins[0],
        summary.priority_conflict_wins[1]
    )
}

fn secret_terminal_outcome_json(outcome: secret_draft::TerminalOutcome) -> String {
    match outcome {
        secret_draft::TerminalOutcome::Win { seat } => {
            format!("{{\"kind\":\"win\",\"winner\":\"{}\"}}", seat.as_str())
        }
        secret_draft::TerminalOutcome::Draw => "{\"kind\":\"draw\",\"winner\":null}".to_owned(),
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
        high_card_duel::TerminalView::Draw { .. } => "draw",
    }
}

fn high_card_terminal_winner(terminal: &high_card_duel::TerminalView) -> Option<HighCardDuelSeat> {
    match terminal {
        high_card_duel::TerminalView::Win { winning_seat, .. } => Some(*winning_seat),
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
        draughts_lite::TerminalView::Win { winning_seat, .. } => Some(*winning_seat),
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

fn option_plain_seat_json(seat: Option<PlainTricksSeat>) -> String {
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

fn option_bool_json(value: Option<bool>) -> String {
    value.map_or_else(|| "null".to_owned(), |value| value.to_string())
}

fn string_array(values: &[String]) -> String {
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
