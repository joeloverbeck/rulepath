use std::collections::BTreeMap;

use crate::ids::{
    DATA_VERSION_LABEL, RULES_VERSION_LABEL, STANDARD_CARD_COUNT, STANDARD_DEFAULT_SEATS,
    STANDARD_MAX_SEATS, STANDARD_MIN_SEATS, STANDARD_MULTI_SEAT_HAND_SIZE, STANDARD_RANK_COUNT,
    STANDARD_SUIT_COUNT, STANDARD_TARGET_SCORE, STANDARD_TWO_SEAT_HAND_SIZE, VARIANT_ID,
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
    "meld_formula",
    "layoff_formula",
    "discard_pickup_formula",
    "deal_formula",
    "rotation_formula",
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
    pub seat_label_prefix: String,
    pub public_family_label: String,
    pub card_back_style: String,
    pub stock_label: String,
    pub discard_label: String,
    pub tableau_label: String,
    pub target_score: i32,
    pub two_seat_hand_size: u8,
    pub multi_seat_hand_size: u8,
    pub suit_count: u8,
    pub rank_count: u8,
    pub card_count: u8,
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
                "seat_label_prefix",
                "public_family_label",
                "card_back_style",
                "stock_label",
                "discard_label",
                "tableau_label",
                "target_score",
                "two_seat_hand_size",
                "multi_seat_hand_size",
                "suit_count",
                "rank_count",
                "card_count",
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
            seat_label_prefix: required_string(&values, "seat_label_prefix")?,
            public_family_label: required_string(&values, "public_family_label")?,
            card_back_style: required_string(&values, "card_back_style")?,
            stock_label: required_string(&values, "stock_label")?,
            discard_label: required_string(&values, "discard_label")?,
            tableau_label: required_string(&values, "tableau_label")?,
            target_score: required_i32(&values, "target_score")?,
            two_seat_hand_size: required_u8(&values, "two_seat_hand_size")?,
            multi_seat_hand_size: required_u8(&values, "multi_seat_hand_size")?,
            suit_count: required_u8(&values, "suit_count")?,
            rank_count: required_u8(&values, "rank_count")?,
            card_count: required_u8(&values, "card_count")?,
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
                "min_seats",
                "default_seats",
                "max_seats",
                "supported_seats",
                "seat_label_prefix",
                "public_family_label",
                "card_back_style",
                "stock_label",
                "discard_label",
                "tableau_label",
                "target_score",
                "two_seat_hand_size",
                "multi_seat_hand_size",
                "suit_count",
                "rank_count",
                "card_count",
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
                min_seats: required_u8(&values, "min_seats")?,
                default_seats: required_u8(&values, "default_seats")?,
                max_seats: required_u8(&values, "max_seats")?,
                supported_seats: required_string(&values, "supported_seats")?,
                seat_label_prefix: required_string(&values, "seat_label_prefix")?,
                public_family_label: required_string(&values, "public_family_label")?,
                card_back_style: required_string(&values, "card_back_style")?,
                stock_label: required_string(&values, "stock_label")?,
                discard_label: required_string(&values, "discard_label")?,
                tableau_label: required_string(&values, "tableau_label")?,
                target_score: required_i32(&values, "target_score")?,
                two_seat_hand_size: required_u8(&values, "two_seat_hand_size")?,
                multi_seat_hand_size: required_u8(&values, "multi_seat_hand_size")?,
                suit_count: required_u8(&values, "suit_count")?,
                rank_count: required_u8(&values, "rank_count")?,
                card_count: required_u8(&values, "card_count")?,
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
    pub min_seats: u8,
    pub default_seats: u8,
    pub max_seats: u8,
    pub supported_seats: String,
    pub seat_label_prefix: String,
    pub public_family_label: String,
    pub card_back_style: String,
    pub stock_label: String,
    pub discard_label: String,
    pub tableau_label: String,
    pub target_score: i32,
    pub two_seat_hand_size: u8,
    pub multi_seat_hand_size: u8,
    pub suit_count: u8,
    pub rank_count: u8,
    pub card_count: u8,
    pub deck_order: String,
    pub terminal_outcomes: String,
}

impl Variant {
    pub fn classic_500_single_deck_v1() -> Self {
        Self {
            id: VARIANT_ID.to_owned(),
            display_name: "Meldfall Ledger".to_owned(),
            rules_version_label: RULES_VERSION_LABEL.to_owned(),
            data_version_label: DATA_VERSION_LABEL.to_owned(),
            min_seats: STANDARD_MIN_SEATS,
            default_seats: STANDARD_DEFAULT_SEATS,
            max_seats: STANDARD_MAX_SEATS,
            supported_seats: "2,3,4,5,6".to_owned(),
            seat_label_prefix: "Seat".to_owned(),
            public_family_label: "Five Hundred Rummy family".to_owned(),
            card_back_style: "rulepath-meldfall-ledger-back-v1".to_owned(),
            stock_label: "Stock".to_owned(),
            discard_label: "Discard pile".to_owned(),
            tableau_label: "Meld tableau".to_owned(),
            target_score: STANDARD_TARGET_SCORE,
            two_seat_hand_size: STANDARD_TWO_SEAT_HAND_SIZE,
            multi_seat_hand_size: STANDARD_MULTI_SEAT_HAND_SIZE,
            suit_count: STANDARD_SUIT_COUNT,
            rank_count: STANDARD_RANK_COUNT,
            card_count: STANDARD_CARD_COUNT,
            deck_order: "standard_52_cards_clubs_diamonds_hearts_spades_rank_ascending_v1"
                .to_owned(),
            terminal_outcomes: "unique_highest_seat_500_exact_tie_continues".to_owned(),
        }
    }

    pub fn resolve(id: &str) -> Result<Self, String> {
        match id {
            VARIANT_ID => Ok(Self::classic_500_single_deck_v1()),
            _ => Err(format!("unsupported meldfall_ledger variant `{id}`")),
        }
    }
}

pub fn load_manifest() -> Result<Manifest, String> {
    Manifest::parse(include_str!("../data/manifest.toml"))
}

pub fn load_variants() -> Result<VariantCatalog, String> {
    VariantCatalog::parse(include_str!("../data/variants.toml"))
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
        let parsed_value = if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
            value[1..value.len() - 1].to_owned()
        } else if value.chars().all(|ch| ch.is_ascii_digit()) {
            value.to_owned()
        } else {
            return Err(format!(
                "line {} value must be a string or integer",
                line_index + 1
            ));
        };
        if values.insert(key.to_owned(), parsed_value).is_some() {
            return Err(format!("duplicate field `{key}`"));
        }
    }
    Ok(values)
}

fn reject_unknown_keys(
    values: &BTreeMap<String, String>,
    allowed: &[&'static str],
) -> Result<(), String> {
    for key in values.keys() {
        if BEHAVIOR_KEYS.contains(&key.as_str()) {
            return Err(format!("behavior-looking field `{key}` is not allowed"));
        }
        if !allowed.contains(&key.as_str()) {
            return Err(format!("unknown field `{key}`"));
        }
    }
    Ok(())
}

fn required_string(values: &BTreeMap<String, String>, key: &str) -> Result<String, String> {
    values
        .get(key)
        .cloned()
        .ok_or_else(|| format!("missing required field `{key}`"))
}

fn required_u32(values: &BTreeMap<String, String>, key: &str) -> Result<u32, String> {
    required_string(values, key)?
        .parse::<u32>()
        .map_err(|_| format!("field `{key}` must be an unsigned integer"))
}

fn required_u8(values: &BTreeMap<String, String>, key: &str) -> Result<u8, String> {
    required_string(values, key)?
        .parse::<u8>()
        .map_err(|_| format!("field `{key}` must be an unsigned integer"))
}

fn required_i32(values: &BTreeMap<String, String>, key: &str) -> Result<i32, String> {
    required_string(values, key)?
        .parse::<i32>()
        .map_err(|_| format!("field `{key}` must be a signed integer"))
}
