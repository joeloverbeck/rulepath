//! Game catalog and feature-report endpoints.
//!
//! [`list_games`] renders the browser-facing catalog (one entry per registered
//! game: ids, display names, variants, viewer modes, seat metadata, and tags),
//! and [`feature_report`] advertises the supported operations and feature
//! flags. Per-game catalog details that live alongside game-specific code
//! (event-frontier UI/seat labels, river-ledger seat labels) are pulled in from
//! the crate root.

use crate::constants::*;
use crate::json::{escape_json, string_array_json};
use crate::{
    event_frontier_catalog_seat_labels_json, event_frontier_catalog_ui_json,
    river_catalog_seat_labels_json, RegisteredGame,
};

fn variant_json(id: &str, label: &str, description: Option<&str>) -> String {
    let description_field = description
        .map(|value| format!(",\"description\":\"{}\"", escape_json(value)))
        .unwrap_or_default();
    format!(
        "{{\"id\":\"{}\",\"label\":\"{}\"{}}}",
        escape_json(id),
        escape_json(label),
        description_field
    )
}

fn variants_json(variants: &[(&str, &str, Option<&str>)]) -> String {
    format!(
        "[{}]",
        variants
            .iter()
            .map(|(id, label, description)| variant_json(id, label, *description))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn with_catalog_seat_metadata(
    mut catalog_json: String,
    seat_count: usize,
    seat_labels_json: Option<&str>,
) -> String {
    let seat_fields = catalog_seat_metadata_fields(seat_count, seat_labels_json);
    if catalog_json.ends_with('}') {
        catalog_json.pop();
        catalog_json.push_str(&seat_fields);
        catalog_json.push('}');
    }
    catalog_json
}

fn catalog_seat_metadata_fields(seat_count: usize, seat_labels_json: Option<&str>) -> String {
    let seat_labels = seat_labels_json
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| catalog_seat_labels_json(seat_count));
    format!(
        ",\"min_seats\":{seat_count},\"max_seats\":{seat_count},\"default_seats\":{seat_count},\"supported_seats\":[{seat_count}],\"seat_labels\":{},\"viewer_modes\":{}",
        seat_labels,
        catalog_viewer_modes_json(seat_count)
    )
}

fn catalog_seat_labels_json(seat_count: usize) -> String {
    format!(
        "[{}]",
        (0..seat_count)
            .map(|index| { format!("{{\"seat\":\"seat_{index}\",\"label\":\"Seat {index}\"}}") })
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn catalog_viewer_modes_json(seat_count: usize) -> String {
    let mut modes = vec!["\"observer\"".to_owned()];
    modes.extend((0..seat_count).map(|index| format!("\"seat_{index}\"")));
    format!("[{}]", modes.join(","))
}

pub fn list_games() -> Result<String, String> {
    let flood_variants = flood_watch::load_variants()?;
    let frontier_variants = frontier_control::load_variants()?;
    let event_variants = event_frontier::load_variants()?;
    let games = [
        RegisteredGame::RaceToN,
        RegisteredGame::ThreeMarks,
        RegisteredGame::ColumnFour,
        RegisteredGame::DirectionalFlip,
        RegisteredGame::DraughtsLite,
        RegisteredGame::HighCardDuel,
        RegisteredGame::MaskedClaims,
        RegisteredGame::FloodWatch,
        RegisteredGame::FrontierControl,
        RegisteredGame::EventFrontier,
        RegisteredGame::TokenBazaar,
        RegisteredGame::SecretDraft,
        RegisteredGame::PokerLite,
        RegisteredGame::PlainTricks,
        RegisteredGame::RiverLedger,
        RegisteredGame::BriarCircuit,
    ]
        .iter()
        .map(|game| {
            let catalog_json = match game {
            RegisteredGame::RaceToN => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{}}}",
                escape_json(GAME_RACE_TO_N),
                escape_json(GAME_RACE_TO_N_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION
            ),
            RegisteredGame::ThreeMarks => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{}}}",
                escape_json(GAME_THREE_MARKS),
                escape_json(GAME_THREE_MARKS_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_THREE_MARKS_STANDARD, GAME_THREE_MARKS_DISPLAY_NAME, None)])
            ),
            RegisteredGame::ColumnFour => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{}}}",
                escape_json(GAME_COLUMN_FOUR),
                escape_json(GAME_COLUMN_FOUR_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_COLUMN_FOUR_STANDARD, GAME_COLUMN_FOUR_DISPLAY_NAME, None)])
            ),
            RegisteredGame::DirectionalFlip => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{}}}",
                escape_json(GAME_DIRECTIONAL_FLIP),
                escape_json(GAME_DIRECTIONAL_FLIP_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_DIRECTIONAL_FLIP_STANDARD, GAME_DIRECTIONAL_FLIP_DISPLAY_NAME, None)])
            ),
            RegisteredGame::DraughtsLite => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{}}}",
                escape_json(GAME_DRAUGHTS_LITE),
                escape_json(GAME_DRAUGHTS_LITE_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_DRAUGHTS_LITE_STANDARD, GAME_DRAUGHTS_LITE_DISPLAY_NAME, None)])
            ),
            RegisteredGame::HighCardDuel => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{},\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"],\"hidden_information\":true,\"tags\":[\"hidden_info\",\"viewer_filtered\",\"public_replay_export\"]}}",
                escape_json(GAME_HIGH_CARD_DUEL),
                escape_json(GAME_HIGH_CARD_DUEL_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_HIGH_CARD_DUEL_STANDARD, GAME_HIGH_CARD_DUEL_DISPLAY_NAME, None)])
            ),
            RegisteredGame::MaskedClaims => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{},\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"],\"hidden_information\":true,\"tags\":[\"hidden_info\",\"viewer_filtered\",\"public_replay_export\",\"reaction_window\",\"bluffing\"]}}",
                escape_json(GAME_MASKED_CLAIMS),
                escape_json(GAME_MASKED_CLAIMS_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_MASKED_CLAIMS_STANDARD, GAME_MASKED_CLAIMS_DISPLAY_NAME, None)])
            ),
            RegisteredGame::FloodWatch => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{},\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"],\"hidden_information\":true,\"cooperative\":true,\"tags\":[\"hidden_info\",\"viewer_filtered\",\"public_replay_export\",\"cooperative\",\"environment_automation\"],\"docs\":[\"games/flood_watch/docs/RULES.md\",\"games/flood_watch/docs/SOURCES.md\"]}}",
                escape_json(GAME_FLOOD_WATCH),
                escape_json(GAME_FLOOD_WATCH_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[
                    (&flood_variants.standard.id, &flood_variants.standard.display_name, flood_variants.standard.description.as_deref()),
                    (&flood_variants.deluge.id, &flood_variants.deluge.display_name, flood_variants.deluge.description.as_deref()),
                ])
            ),
            RegisteredGame::FrontierControl => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{},\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"],\"hidden_information\":false,\"tags\":[\"perfect_information\",\"graph_map\",\"asymmetric_factions\",\"public_replay_export\"],\"docs\":[\"games/frontier_control/docs/RULES.md\",\"games/frontier_control/docs/SOURCES.md\",\"games/frontier_control/docs/COMPETENT-PLAYER.md\",\"games/frontier_control/docs/BOT-STRATEGY-EVIDENCE-PACK.md\"]}}",
                escape_json(GAME_FRONTIER_CONTROL),
                escape_json(GAME_FRONTIER_CONTROL_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[
                    (&frontier_variants.standard.id, &frontier_variants.standard.display_name, frontier_variants.standard.description.as_deref()),
                    (&frontier_variants.highlands.id, &frontier_variants.highlands.display_name, frontier_variants.highlands.description.as_deref()),
                ])
            ),
            RegisteredGame::EventFrontier => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{},\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"],\"hidden_information\":true,\"tags\":[\"hidden_info\",\"event_deck\",\"graph_map\",\"asymmetric_factions\",\"public_replay_export\"],\"docs\":[\"games/event_frontier/docs/RULES.md\",\"games/event_frontier/docs/SOURCES.md\",\"games/event_frontier/docs/COMPETENT-PLAYER.md\",\"games/event_frontier/docs/BOT-STRATEGY-EVIDENCE-PACK.md\"],\"ui\":{}}}",
                escape_json(GAME_EVENT_FRONTIER),
                escape_json(GAME_EVENT_FRONTIER_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[
                    (&event_variants.standard.id, &event_variants.standard.display_name, event_variants.standard.description.as_deref()),
                    (&event_variants.hard_winter.id, &event_variants.hard_winter.display_name, event_variants.hard_winter.description.as_deref()),
                    (&event_variants.land_rush.id, &event_variants.land_rush.display_name, event_variants.land_rush.description.as_deref()),
                ]),
                event_frontier_catalog_ui_json()
            ),
            RegisteredGame::TokenBazaar => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{},\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"],\"hidden_information\":false,\"tags\":[\"public_accounting\",\"economy\",\"public_replay_export\"]}}",
                escape_json(GAME_TOKEN_BAZAAR),
                escape_json(GAME_TOKEN_BAZAAR_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_TOKEN_BAZAAR_STANDARD, GAME_TOKEN_BAZAAR_DISPLAY_NAME, None)])
            ),
            RegisteredGame::SecretDraft => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{},\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"],\"hidden_information\":true,\"tags\":[\"hidden_info\",\"simultaneous_commit\",\"viewer_filtered\",\"public_replay_export\"]}}",
                escape_json(GAME_SECRET_DRAFT),
                escape_json(GAME_SECRET_DRAFT_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_SECRET_DRAFT_STANDARD, GAME_SECRET_DRAFT_DISPLAY_NAME, None)])
            ),
            RegisteredGame::PokerLite => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{},\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"],\"hidden_information\":true,\"tags\":[\"hidden_info\",\"viewer_filtered\",\"public_replay_export\",\"public_accounting\",\"bounded_pledge\"]}}",
                escape_json(GAME_POKER_LITE),
                escape_json(GAME_POKER_LITE_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_POKER_LITE_STANDARD, GAME_POKER_LITE_DISPLAY_NAME, None)])
            ),
            RegisteredGame::PlainTricks => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{},\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"],\"hidden_information\":true,\"tags\":[\"hidden_info\",\"viewer_filtered\",\"public_replay_export\",\"trick_taking\"]}}",
                escape_json(GAME_PLAIN_TRICKS),
                escape_json(GAME_PLAIN_TRICKS_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_PLAIN_TRICKS_STANDARD, GAME_PLAIN_TRICKS_DISPLAY_NAME, None)])
            ),
            RegisteredGame::RiverLedger => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{},\"hidden_information\":true,\"tags\":[\"hidden_info\",\"viewer_filtered\",\"public_replay_export\",\"public_accounting\",\"fixed_limit\",\"multi_seat\"],\"min_seats\":3,\"max_seats\":6,\"default_seats\":6,\"supported_seats\":[3,4,5,6],\"seat_labels\":{},\"viewer_modes\":{}}}",
                escape_json(GAME_RIVER_LEDGER),
                escape_json(GAME_RIVER_LEDGER_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_RIVER_LEDGER_STANDARD, GAME_RIVER_LEDGER_DISPLAY_NAME, None)]),
                river_catalog_seat_labels_json(),
                catalog_viewer_modes_json(6)
            ),
            RegisteredGame::BriarCircuit => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{},\"variants\":{},\"hidden_information\":true,\"tags\":[\"hidden_info\",\"viewer_filtered\",\"public_replay_export\",\"trick_taking\",\"multi_seat\"],\"min_seats\":4,\"max_seats\":4,\"default_seats\":4,\"supported_seats\":[4],\"seat_labels\":{},\"viewer_modes\":{}}}",
                escape_json(GAME_BRIAR_CIRCUIT),
                escape_json(GAME_BRIAR_CIRCUIT_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION,
                variants_json(&[(VARIANT_BRIAR_CIRCUIT_STANDARD, GAME_BRIAR_CIRCUIT_DISPLAY_NAME, None)]),
                catalog_seat_labels_json(4),
                catalog_viewer_modes_json(4)
            ),
        };
            if matches!(
                game,
                RegisteredGame::RiverLedger | RegisteredGame::BriarCircuit
            ) {
                return catalog_json;
            }
            let seat_labels_json = match game {
                RegisteredGame::EventFrontier => Some(event_frontier_catalog_seat_labels_json()),
                _ => None,
            };
            with_catalog_seat_metadata(catalog_json, DEFAULT_SEAT_COUNT, seat_labels_json.as_deref())
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
