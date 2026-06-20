use std::collections::BTreeMap;

use crate::ids::{
    RULES_VERSION_LABEL, STANDARD_BIG_BET_UNIT, STANDARD_BIG_BLIND, STANDARD_DEFAULT_SEATS,
    STANDARD_SMALL_BET_UNIT, STANDARD_SMALL_BLIND, STANDARD_STARTING_STACK, VARIANT_ID,
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
    "showdown_formula",
    "betting_formula",
    "accounting_formula",
    "visibility_formula",
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
    pub schema_version: u32,
    pub theme_name: String,
    pub min_seats: u8,
    pub default_seats: u8,
    pub max_seats: u8,
    pub street_count: u8,
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
                "schema_version",
                "theme_name",
                "min_seats",
                "default_seats",
                "max_seats",
                "street_count",
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
            schema_version: required_u32(&values, "schema_version")?,
            theme_name: required_string(&values, "theme_name")?,
            min_seats: required_u8(&values, "min_seats")?,
            default_seats: required_u8(&values, "default_seats")?,
            max_seats: required_u8(&values, "max_seats")?,
            street_count: required_u8(&values, "street_count")?,
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
                "supported_seat_counts",
                "default_seats",
                "street_sequence",
                "small_blind",
                "big_blind",
                "small_bet_unit",
                "big_bet_unit",
                "default_starting_stack",
                "stack_presets",
                "raise_cap_per_street",
                "deck_order",
                "terminal_outcomes",
            ],
        )?;

        Ok(Self {
            selected: Variant {
                id: required_string(&values, "variant_id")?,
                display_name: required_string(&values, "display_name")?,
                rules_version_label: required_string(&values, "rules_version_label")?,
                supported_seat_counts: required_string(&values, "supported_seat_counts")?,
                default_seats: required_u8(&values, "default_seats")?,
                street_sequence: required_string(&values, "street_sequence")?,
                small_blind: required_u8(&values, "small_blind")?,
                big_blind: required_u8(&values, "big_blind")?,
                small_bet_unit: required_u8(&values, "small_bet_unit")?,
                big_bet_unit: required_u8(&values, "big_bet_unit")?,
                default_starting_stack: required_u16(&values, "default_starting_stack")?,
                stack_presets: required_string(&values, "stack_presets")?,
                raise_cap_per_street: required_u8(&values, "raise_cap_per_street")?,
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
    pub supported_seat_counts: String,
    pub default_seats: u8,
    pub street_sequence: String,
    pub small_blind: u8,
    pub big_blind: u8,
    pub small_bet_unit: u8,
    pub big_bet_unit: u8,
    pub default_starting_stack: u16,
    pub stack_presets: String,
    pub raise_cap_per_street: u8,
    pub deck_order: String,
    pub terminal_outcomes: String,
}

impl Variant {
    pub fn river_ledger_standard() -> Self {
        Self {
            id: VARIANT_ID.to_owned(),
            display_name: "River Ledger".to_owned(),
            rules_version_label: RULES_VERSION_LABEL.to_owned(),
            supported_seat_counts: "3,4,5,6".to_owned(),
            default_seats: STANDARD_DEFAULT_SEATS,
            street_sequence: "preflop,flop,turn,river,showdown".to_owned(),
            small_blind: STANDARD_SMALL_BLIND,
            big_blind: STANDARD_BIG_BLIND,
            small_bet_unit: STANDARD_SMALL_BET_UNIT,
            big_bet_unit: STANDARD_BIG_BET_UNIT,
            default_starting_stack: STANDARD_STARTING_STACK,
            stack_presets: "equal_24;three_seat_8_16_24;six_seat_4_8_12_16_20_24".to_owned(),
            raise_cap_per_street: 3,
            deck_order: "standard_52_rank_suit_v1".to_owned(),
            terminal_outcomes: "last_live_hand_showdown_win_showdown_split".to_owned(),
        }
    }

    pub fn resolve(id: &str) -> Result<Self, String> {
        match id {
            VARIANT_ID => Ok(Self::river_ledger_standard()),
            _ => Err(format!("unsupported river_ledger variant `{id}`")),
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
        if line.starts_with('[') {
            return Err(format!(
                "sections are not allowed on line {}",
                line_index + 1
            ));
        }

        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("expected key/value on line {}", line_index + 1))?;
        let key = key.trim();
        reject_behavior_key(key)?;

        if values.contains_key(key) {
            return Err(format!("duplicate key `{key}`"));
        }

        values.insert(key.to_owned(), parse_value(value.trim())?);
    }

    Ok(values)
}

fn parse_value(value: &str) -> Result<String, String> {
    if let Some(stripped) = value
        .strip_prefix('"')
        .and_then(|inner| inner.strip_suffix('"'))
    {
        return Ok(stripped.to_owned());
    }

    if value.chars().all(|ch| ch.is_ascii_digit()) {
        return Ok(value.to_owned());
    }

    Err(format!("unsupported value `{value}`"))
}

fn reject_behavior_key(key: &str) -> Result<(), String> {
    if BEHAVIOR_KEYS.contains(&key) {
        return Err(format!("behavior-looking key `{key}` is not allowed"));
    }
    Ok(())
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
        .parse()
        .map_err(|_| format!("key `{key}` must be an integer"))
}

fn required_u8(values: &BTreeMap<String, String>, key: &str) -> Result<u8, String> {
    required_string(values, key)?
        .parse()
        .map_err(|_| format!("key `{key}` must fit u8"))
}

fn required_u16(values: &BTreeMap<String, String>, key: &str) -> Result<u16, String> {
    required_string(values, key)?
        .parse()
        .map_err(|_| format!("key `{key}` must fit u16"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cards::STANDARD_CARD_COUNT,
        ids::{
            GAME_ID, MAX_RAISES_PER_STREET, STANDARD_MAX_SEATS, STANDARD_MIN_SEATS,
            STANDARD_STREET_COUNT,
        },
    };

    #[test]
    fn behavior_looking_keys_are_rejected() {
        assert!(
            Manifest::parse("game_id = \"river_ledger\"\nvisibility_formula = \"bad\"\n").is_err()
        );
        assert!(VariantCatalog::parse(
            "variant_id = \"river_ledger_standard\"\nselector = \"bad\"\n"
        )
        .is_err());
    }

    #[test]
    fn constants_match_static_data() {
        let manifest = Manifest::parse(include_str!("../data/manifest.toml")).unwrap();
        let variants = VariantCatalog::parse(include_str!("../data/variants.toml")).unwrap();

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.min_seats, STANDARD_MIN_SEATS);
        assert_eq!(manifest.default_seats, STANDARD_DEFAULT_SEATS);
        assert_eq!(manifest.max_seats, STANDARD_MAX_SEATS);
        assert_eq!(manifest.street_count, STANDARD_STREET_COUNT);
        assert_eq!(manifest.card_count, STANDARD_CARD_COUNT);
        assert_eq!(variants.selected, Variant::river_ledger_standard());
        assert_eq!(
            variants.selected.default_starting_stack,
            STANDARD_STARTING_STACK
        );
        assert!(variants
            .selected
            .stack_presets
            .contains("three_seat_8_16_24"));
        assert_eq!(
            variants.selected.raise_cap_per_street,
            MAX_RAISES_PER_STREET
        );
    }

    #[test]
    fn standard_variant_resolves_by_id() {
        assert_eq!(
            Variant::resolve(VARIANT_ID).expect("standard variant resolves"),
            Variant::river_ledger_standard()
        );
        assert!(Variant::resolve("other").is_err());
    }
}
