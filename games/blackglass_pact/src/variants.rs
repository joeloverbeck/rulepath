use std::collections::BTreeMap;

use crate::ids::{
    DATA_VERSION_LABEL, RULES_VERSION_LABEL, STANDARD_CARD_COUNT, STANDARD_HAND_SIZE,
    STANDARD_RANK_COUNT, STANDARD_SEAT_COUNT, STANDARD_SUIT_COUNT, STANDARD_TRICKS_PER_HAND,
    VARIANT_ID,
};

const BEHAVIOR_KEYS: &[&str] = &[
    "when",
    "if",
    "then",
    "else",
    "selector",
    "condition",
    "trigger",
    "script",
    "loop",
    "foreach",
    "priority_expression",
    "ai_condition",
    "effect_script",
    "rule",
    "requires",
    "valid_if",
    "on_play",
    "on_reveal",
    "formula",
    "score_formula",
    "tie_break_formula",
    "trick_winner_formula",
    "follow_suit_formula",
    "deal_formula",
    "rotation_formula",
    "bid_formula",
    "contract_formula",
    "bot_policy",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Manifest {
    pub game_id: String,
    pub display_name: String,
    pub readiness: String,
    pub rules_version: u32,
    pub rules_version_label: String,
    pub data_version: u32,
    pub data_version_label: String,
    pub schema_version: u32,
    pub theme_name: String,
    pub min_seats: u8,
    pub default_seats: u8,
    pub max_seats: u8,
    pub supported_seats: String,
    pub suit_count: u8,
    pub rank_count: u8,
    pub card_count: u8,
    pub hand_size: u8,
    pub tricks_per_hand: u8,
    pub team_layout: String,
    pub deck_order: String,
}

impl Manifest {
    pub fn parse(input: &str) -> Result<Self, String> {
        let values = parse_flat_toml(input)?;
        reject_unknown_keys(
            &values,
            &[
                "game_id",
                "display_name",
                "readiness",
                "rules_version",
                "rules_version_label",
                "data_version",
                "data_version_label",
                "schema_version",
                "theme_name",
                "min_seats",
                "default_seats",
                "max_seats",
                "supported_seats",
                "suit_count",
                "rank_count",
                "card_count",
                "hand_size",
                "tricks_per_hand",
                "team_layout",
                "deck_order",
            ],
        )?;

        Ok(Self {
            game_id: required_string(&values, "game_id")?,
            display_name: required_string(&values, "display_name")?,
            readiness: required_string(&values, "readiness")?,
            rules_version: required_u32(&values, "rules_version")?,
            rules_version_label: required_string(&values, "rules_version_label")?,
            data_version: required_u32(&values, "data_version")?,
            data_version_label: required_string(&values, "data_version_label")?,
            schema_version: required_u32(&values, "schema_version")?,
            theme_name: required_string(&values, "theme_name")?,
            min_seats: required_u8(&values, "min_seats")?,
            default_seats: required_u8(&values, "default_seats")?,
            max_seats: required_u8(&values, "max_seats")?,
            supported_seats: required_string(&values, "supported_seats")?,
            suit_count: required_u8(&values, "suit_count")?,
            rank_count: required_u8(&values, "rank_count")?,
            card_count: required_u8(&values, "card_count")?,
            hand_size: required_u8(&values, "hand_size")?,
            tricks_per_hand: required_u8(&values, "tricks_per_hand")?,
            team_layout: required_string(&values, "team_layout")?,
            deck_order: required_string(&values, "deck_order")?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantCatalog {
    pub selected: Variant,
}

impl VariantCatalog {
    pub fn parse(input: &str) -> Result<Self, String> {
        let values = parse_flat_toml(input)?;
        reject_unknown_keys(
            &values,
            &[
                "variant_id",
                "display_name",
                "rules_version_label",
                "data_version_label",
                "seat_count",
                "suit_count",
                "rank_count",
                "card_count",
                "hand_size",
                "tricks_per_hand",
                "team_layout",
                "deck_order",
                "terminal_outcomes",
            ],
        )?;

        Ok(Self {
            selected: Variant {
                id: required_string(&values, "variant_id")?,
                display_name: required_string(&values, "display_name")?,
                rules_version_label: required_string(&values, "rules_version_label")?,
                data_version_label: required_string(&values, "data_version_label")?,
                seat_count: required_u8(&values, "seat_count")?,
                suit_count: required_u8(&values, "suit_count")?,
                rank_count: required_u8(&values, "rank_count")?,
                card_count: required_u8(&values, "card_count")?,
                hand_size: required_u8(&values, "hand_size")?,
                tricks_per_hand: required_u8(&values, "tricks_per_hand")?,
                team_layout: required_string(&values, "team_layout")?,
                deck_order: required_string(&values, "deck_order")?,
                terminal_outcomes: required_string(&values, "terminal_outcomes")?,
            },
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variant {
    pub id: String,
    pub display_name: String,
    pub rules_version_label: String,
    pub data_version_label: String,
    pub seat_count: u8,
    pub suit_count: u8,
    pub rank_count: u8,
    pub card_count: u8,
    pub hand_size: u8,
    pub tricks_per_hand: u8,
    pub team_layout: String,
    pub deck_order: String,
    pub terminal_outcomes: String,
}

impl Variant {
    pub fn blackglass_pact_standard() -> Self {
        Self {
            id: VARIANT_ID.to_owned(),
            display_name: "Blackglass Pact".to_owned(),
            rules_version_label: RULES_VERSION_LABEL.to_owned(),
            data_version_label: DATA_VERSION_LABEL.to_owned(),
            seat_count: STANDARD_SEAT_COUNT,
            suit_count: STANDARD_SUIT_COUNT,
            rank_count: STANDARD_RANK_COUNT,
            card_count: STANDARD_CARD_COUNT,
            hand_size: STANDARD_HAND_SIZE,
            tricks_per_hand: STANDARD_TRICKS_PER_HAND,
            team_layout: "team_0_seat_0_seat_2__team_1_seat_1_seat_3".to_owned(),
            deck_order: "standard_52_cards_clubs_diamonds_hearts_spades_rank_ascending_v1"
                .to_owned(),
            terminal_outcomes: "unique_higher_team_500_exact_tie_continues".to_owned(),
        }
    }

    pub fn resolve(id: &str) -> Result<Self, String> {
        match id {
            VARIANT_ID => Ok(Self::blackglass_pact_standard()),
            _ => Err(format!("unsupported blackglass_pact variant `{id}`")),
        }
    }
}

fn parse_flat_toml(input: &str) -> Result<BTreeMap<String, String>, String> {
    let mut values = BTreeMap::new();
    for (line_index, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("line {} is not a key/value pair", line_index + 1));
        };
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() {
            return Err(format!("line {} has an empty key", line_index + 1));
        }
        if BEHAVIOR_KEYS.contains(&key) {
            return Err(format!("behavior-looking key `{key}` is not allowed"));
        }
        if values.contains_key(key) {
            return Err(format!("duplicate key `{key}`"));
        }
        values.insert(key.to_owned(), parse_value(value, line_index + 1)?);
    }
    Ok(values)
}

fn parse_value(value: &str, line_number: usize) -> Result<String, String> {
    if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
        Ok(value[1..value.len() - 1].to_owned())
    } else if value.chars().all(|ch| ch.is_ascii_digit()) {
        Ok(value.to_owned())
    } else {
        Err(format!(
            "line {line_number} value must be a string or integer"
        ))
    }
}

fn reject_unknown_keys(values: &BTreeMap<String, String>, allowed: &[&str]) -> Result<(), String> {
    for key in values.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(format!("unknown key `{key}`"));
        }
    }
    Ok(())
}

fn required_string(values: &BTreeMap<String, String>, key: &str) -> Result<String, String> {
    values
        .get(key)
        .cloned()
        .ok_or_else(|| format!("missing key `{key}`"))
}

fn required_u32(values: &BTreeMap<String, String>, key: &str) -> Result<u32, String> {
    required_string(values, key)?
        .parse::<u32>()
        .map_err(|_| format!("key `{key}` must be u32"))
}

fn required_u8(values: &BTreeMap<String, String>, key: &str) -> Result<u8, String> {
    required_string(values, key)?
        .parse::<u8>()
        .map_err(|_| format!("key `{key}` must be u8"))
}
