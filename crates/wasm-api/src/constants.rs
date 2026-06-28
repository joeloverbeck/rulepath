//! Shared compile-time constants for the browser-facing API: API/engine/data
//! version tags, per-game id and display-name strings, supported-operation and
//! feature-flag lists, trace rules-version labels, and variant ids.
//!
//! These are glob-imported (`use crate::constants::*;`) at the crate root so the
//! rest of the bridge can reference them by bare name.

pub(crate) const API_VERSION: &str = "rulepath-wasm-api/0.1.0";
pub(crate) const DEFAULT_SEAT_COUNT: usize = 2;
pub(crate) const GAME_RACE_TO_N: &str = "race_to_n";
pub(crate) const GAME_RACE_TO_N_DISPLAY_NAME: &str = "Race to 21";
pub(crate) const GAME_THREE_MARKS: &str = "three_marks";
pub(crate) const GAME_THREE_MARKS_DISPLAY_NAME: &str = "Three Marks";
pub(crate) const GAME_COLUMN_FOUR: &str = "column_four";
pub(crate) const GAME_COLUMN_FOUR_DISPLAY_NAME: &str = "Column Four";
pub(crate) const GAME_DIRECTIONAL_FLIP: &str = "directional_flip";
pub(crate) const GAME_DIRECTIONAL_FLIP_DISPLAY_NAME: &str = "Directional Flip";
pub(crate) const GAME_DRAUGHTS_LITE: &str = "draughts_lite";
pub(crate) const GAME_DRAUGHTS_LITE_DISPLAY_NAME: &str = "Draughts Lite";
pub(crate) const GAME_HIGH_CARD_DUEL: &str = "high_card_duel";
pub(crate) const GAME_HIGH_CARD_DUEL_DISPLAY_NAME: &str = "High Card Duel";
pub(crate) const GAME_MASKED_CLAIMS: &str = "masked_claims";
pub(crate) const GAME_MASKED_CLAIMS_DISPLAY_NAME: &str = "Masked Claims";
pub(crate) const GAME_MELDFALL_LEDGER: &str = "meldfall_ledger";
pub(crate) const GAME_MELDFALL_LEDGER_DISPLAY_NAME: &str = "Meldfall Ledger";
pub(crate) const GAME_FLOOD_WATCH: &str = "flood_watch";
pub(crate) const GAME_FLOOD_WATCH_DISPLAY_NAME: &str = "Flood Watch";
pub(crate) const GAME_FRONTIER_CONTROL: &str = "frontier_control";
pub(crate) const GAME_FRONTIER_CONTROL_DISPLAY_NAME: &str = "Frontier Control";
pub(crate) const GAME_EVENT_FRONTIER: &str = "event_frontier";
pub(crate) const GAME_EVENT_FRONTIER_DISPLAY_NAME: &str = "Event Frontier";
pub(crate) const GAME_TOKEN_BAZAAR: &str = "token_bazaar";
pub(crate) const GAME_TOKEN_BAZAAR_DISPLAY_NAME: &str = "Token Bazaar";
pub(crate) const GAME_SECRET_DRAFT: &str = "secret_draft";
pub(crate) const GAME_SECRET_DRAFT_DISPLAY_NAME: &str = "Veiled Draft";
pub(crate) const GAME_POKER_LITE: &str = "poker_lite";
pub(crate) const GAME_POKER_LITE_DISPLAY_NAME: &str = "Crest Ledger";
pub(crate) const GAME_PLAIN_TRICKS: &str = "plain_tricks";
pub(crate) const GAME_PLAIN_TRICKS_DISPLAY_NAME: &str = "Plain Tricks";
pub(crate) const GAME_RIVER_LEDGER: &str = "river_ledger";
pub(crate) const GAME_RIVER_LEDGER_DISPLAY_NAME: &str = "River Ledger";
pub(crate) const GAME_BRIAR_CIRCUIT: &str = "briar_circuit";
pub(crate) const GAME_BRIAR_CIRCUIT_DISPLAY_NAME: &str = "Briar Circuit";
pub(crate) const GAME_VOW_TIDE: &str = "vow_tide";
pub(crate) const GAME_VOW_TIDE_DISPLAY_NAME: &str = "Vow Tide";
pub(crate) const GAME_BLACKGLASS_PACT: &str = "blackglass_pact";
pub(crate) const GAME_BLACKGLASS_PACT_DISPLAY_NAME: &str = "Blackglass Pact";
pub(crate) const GAME_STARBRIDGE_CROSSING: &str = "starbridge_crossing";
pub(crate) const GAME_STARBRIDGE_CROSSING_DISPLAY_NAME: &str = "Starbridge Crossing";
pub(crate) const RULES_VERSION: u32 = 1;
pub(crate) const SCHEMA_VERSION: u32 = 1;
pub(crate) const SUPPORTED_OPERATIONS: &[&str] = &[
    "feature_report",
    "list_games",
    "new_match",
    "new_match_with_seat_count",
    "new_match_with_options",
    "new_match_with_variant",
    "new_match_with_variant_and_seat_count",
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
pub(crate) const FEATURE_FLAGS: &[&str] =
    &["catalog", "match_store", "legal_action_tree", "effects"];
pub(crate) const RACE_TRACE_RULES_VERSION: &str = "race_to_n-rules-v1";
pub(crate) const THREE_MARKS_TRACE_RULES_VERSION: &str = "three_marks-rules-v1";
pub(crate) const COLUMN_FOUR_TRACE_RULES_VERSION: &str = "column_four-rules-v1";
pub(crate) const DIRECTIONAL_FLIP_TRACE_RULES_VERSION: &str = "directional_flip-rules-v1";
pub(crate) const DRAUGHTS_LITE_TRACE_RULES_VERSION: &str = "draughts_lite-rules-v1";
pub(crate) const HIGH_CARD_DUEL_TRACE_RULES_VERSION: &str = "high-card-duel-rules-v1";
pub(crate) const MASKED_CLAIMS_TRACE_RULES_VERSION: &str = "masked-claims-rules-v1";
pub(crate) const MELDFALL_LEDGER_TRACE_RULES_VERSION: &str = "meldfall-ledger-rules-v1";
pub(crate) const FLOOD_WATCH_TRACE_RULES_VERSION: &str = "flood-watch-rules-v1";
pub(crate) const FRONTIER_CONTROL_TRACE_RULES_VERSION: &str = "frontier-control-rules-v1";
pub(crate) const EVENT_FRONTIER_TRACE_RULES_VERSION: &str = "event-frontier-rules-v1";
pub(crate) const TOKEN_BAZAAR_TRACE_RULES_VERSION: &str = "token-bazaar-rules-v1";
pub(crate) const SECRET_DRAFT_TRACE_RULES_VERSION: &str = "secret-draft-rules-v1";
pub(crate) const POKER_LITE_TRACE_RULES_VERSION: &str = "poker-lite-rules-v1";
pub(crate) const PLAIN_TRICKS_TRACE_RULES_VERSION: &str = "plain-tricks-rules-v1";
pub(crate) const RIVER_LEDGER_TRACE_RULES_VERSION: &str = "river-ledger-rules-v1";
pub(crate) const BRIAR_CIRCUIT_TRACE_RULES_VERSION: &str = "briar-circuit-rules-v1";
pub(crate) const VOW_TIDE_TRACE_RULES_VERSION: &str = "vow-tide-rules-v1";
pub(crate) const BLACKGLASS_PACT_TRACE_RULES_VERSION: &str = "blackglass-pact-rules-v1";
pub(crate) const STARBRIDGE_CROSSING_TRACE_RULES_VERSION: &str = "starbridge-crossing-rules-v1";
pub(crate) const ENGINE_VERSION: &str = "engine-core-0.1.0";
pub(crate) const DATA_VERSION: &str = "1";
pub(crate) const VARIANT_RACE_TO_21: &str = "race_to_21";
pub(crate) const VARIANT_THREE_MARKS_STANDARD: &str = "three_marks_standard";
pub(crate) const VARIANT_COLUMN_FOUR_STANDARD: &str = "column_four_standard";
pub(crate) const VARIANT_DIRECTIONAL_FLIP_STANDARD: &str = "directional_flip_standard";
pub(crate) const VARIANT_DRAUGHTS_LITE_STANDARD: &str = "draughts_lite_standard";
pub(crate) const VARIANT_HIGH_CARD_DUEL_STANDARD: &str = "high_card_duel_standard";
pub(crate) const VARIANT_MASKED_CLAIMS_STANDARD: &str = "masked_claims_standard";
pub(crate) const VARIANT_MELDFALL_LEDGER_STANDARD: &str = "classic_500_single_deck_v1";
pub(crate) const VARIANT_FLOOD_WATCH_STANDARD: &str = "flood_watch_standard";
pub(crate) const VARIANT_FRONTIER_CONTROL_STANDARD: &str = "frontier_control_standard";
pub(crate) const VARIANT_EVENT_FRONTIER_STANDARD: &str = "event_frontier_standard";
pub(crate) const VARIANT_TOKEN_BAZAAR_STANDARD: &str = "token_bazaar_standard";
pub(crate) const VARIANT_SECRET_DRAFT_STANDARD: &str = "secret_draft_standard";
pub(crate) const VARIANT_POKER_LITE_STANDARD: &str = "poker_lite_standard";
pub(crate) const VARIANT_PLAIN_TRICKS_STANDARD: &str = "plain_tricks_standard";
pub(crate) const VARIANT_RIVER_LEDGER_STANDARD: &str = "river_ledger_standard";
pub(crate) const VARIANT_BRIAR_CIRCUIT_STANDARD: &str = "briar_circuit_standard";
pub(crate) const VARIANT_VOW_TIDE_STANDARD: &str = "vow_tide_standard";
pub(crate) const VARIANT_BLACKGLASS_PACT_STANDARD: &str = "blackglass_pact_standard";
pub(crate) const VARIANT_STARBRIDGE_CROSSING_STANDARD: &str = "starbridge_crossing_classic_star_v1";
// The browser shell must be able to import any replay it can legitimately
// export. Starbridge Crossing 6-seat, 2000-ply public exports are about 549 KiB
// today; 8 MiB leaves order-of-magnitude catalog headroom while still rejecting
// pathological local paste/import payloads before parsing.
pub(crate) const MAX_REPLAY_IMPORT_BYTES: usize = 8 * 1024 * 1024;
