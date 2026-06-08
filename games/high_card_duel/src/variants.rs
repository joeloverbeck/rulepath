use std::collections::BTreeMap;

use crate::ids::{
    RULES_VERSION_LABEL, SHUFFLE_ALGORITHM, STANDARD_DECK_CARD_COUNT, STANDARD_HAND_SIZE,
    STANDARD_RANK_COUNT, STANDARD_ROUND_LIMIT, STANDARD_SIGILS_PER_RANK, VARIANT_ID,
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
    pub seat_count: u8,
    pub deck_card_count: u8,
    pub rank_count: u8,
    pub sigils_per_rank: u8,
    pub hand_size: u8,
    pub round_limit: u8,
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
                "seat_count",
                "deck_card_count",
                "rank_count",
                "sigils_per_rank",
                "hand_size",
                "round_limit",
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
            seat_count: required_u8(&values, "seat_count")?,
            deck_card_count: required_u8(&values, "deck_card_count")?,
            rank_count: required_u8(&values, "rank_count")?,
            sigils_per_rank: required_u8(&values, "sigils_per_rank")?,
            hand_size: required_u8(&values, "hand_size")?,
            round_limit: required_u8(&values, "round_limit")?,
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
                "seat_count",
                "first_lead_seat",
                "round_limit",
                "hand_size",
                "deck_card_count",
                "rank_count",
                "sigils_per_rank",
                "shuffle_algorithm",
                "commitment_model",
                "terminal_scoring",
            ],
        )?;

        Ok(Self {
            selected: Variant {
                id: required_string(&values, "variant_id")?,
                display_name: required_string(&values, "display_name")?,
                rules_version_label: required_string(&values, "rules_version_label")?,
                seat_count: required_u8(&values, "seat_count")?,
                first_lead_seat: required_string(&values, "first_lead_seat")?,
                round_limit: required_u8(&values, "round_limit")?,
                hand_size: required_u8(&values, "hand_size")?,
                deck_card_count: required_u8(&values, "deck_card_count")?,
                rank_count: required_u8(&values, "rank_count")?,
                sigils_per_rank: required_u8(&values, "sigils_per_rank")?,
                shuffle_algorithm: required_string(&values, "shuffle_algorithm")?,
                commitment_model: required_string(&values, "commitment_model")?,
                terminal_scoring: required_string(&values, "terminal_scoring")?,
            },
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variant {
    pub id: String,
    pub display_name: String,
    pub rules_version_label: String,
    pub seat_count: u8,
    pub first_lead_seat: String,
    pub round_limit: u8,
    pub hand_size: u8,
    pub deck_card_count: u8,
    pub rank_count: u8,
    pub sigils_per_rank: u8,
    pub shuffle_algorithm: String,
    pub commitment_model: String,
    pub terminal_scoring: String,
}

impl Variant {
    pub fn high_card_duel_standard() -> Self {
        Self {
            id: VARIANT_ID.to_owned(),
            display_name: "High Card Duel".to_owned(),
            rules_version_label: RULES_VERSION_LABEL.to_owned(),
            seat_count: 2,
            first_lead_seat: "seat_0".to_owned(),
            round_limit: STANDARD_ROUND_LIMIT,
            hand_size: STANDARD_HAND_SIZE,
            deck_card_count: STANDARD_DECK_CARD_COUNT,
            rank_count: STANDARD_RANK_COUNT,
            sigils_per_rank: STANDARD_SIGILS_PER_RANK,
            shuffle_algorithm: SHUFFLE_ALGORITHM.to_owned(),
            commitment_model: "lead_reply_face_down_simultaneous_reveal".to_owned(),
            terminal_scoring: "higher_score_after_six_rounds_else_draw".to_owned(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::GAME_ID;

    #[test]
    fn behavior_looking_keys_are_rejected() {
        assert!(VariantCatalog::parse(
            "variant_id = \"high_card_duel_standard\"\nvalid_if = \"bad\"\n"
        )
        .is_err());
    }

    #[test]
    fn constants_match_static_data() {
        let manifest = Manifest::parse(include_str!("../data/manifest.toml")).unwrap();
        let variants = VariantCatalog::parse(include_str!("../data/variants.toml")).unwrap();

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.deck_card_count, STANDARD_DECK_CARD_COUNT);
        assert_eq!(manifest.rank_count, STANDARD_RANK_COUNT);
        assert_eq!(manifest.sigils_per_rank, STANDARD_SIGILS_PER_RANK);
        assert_eq!(manifest.hand_size, STANDARD_HAND_SIZE);
        assert_eq!(manifest.round_limit, STANDARD_ROUND_LIMIT);
        assert_eq!(variants.selected, Variant::high_card_duel_standard());
    }
}
