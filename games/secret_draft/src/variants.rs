use std::collections::BTreeMap;

use crate::ids::{
    DraftItemId, RULES_VERSION_LABEL, STANDARD_ITEM_COUNT, STANDARD_ROUND_COUNT,
    STANDARD_SEAT_COUNT, VARIANT_ID,
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
    "conflict_formula",
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
    pub round_count: u8,
    pub item_count: u8,
    pub first_priority_seat: String,
    pub item_order: String,
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
                "round_count",
                "item_count",
                "first_priority_seat",
                "item_order",
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
            round_count: required_u8(&values, "round_count")?,
            item_count: required_u8(&values, "item_count")?,
            first_priority_seat: required_string(&values, "first_priority_seat")?,
            item_order: required_string(&values, "item_order")?,
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
                "round_count",
                "item_count",
                "first_priority_seat",
                "item_order",
                "terminal_scoring",
            ],
        )?;

        Ok(Self {
            selected: Variant {
                id: required_string(&values, "variant_id")?,
                display_name: required_string(&values, "display_name")?,
                rules_version_label: required_string(&values, "rules_version_label")?,
                seat_count: required_u8(&values, "seat_count")?,
                round_count: required_u8(&values, "round_count")?,
                item_count: required_u8(&values, "item_count")?,
                first_priority_seat: required_string(&values, "first_priority_seat")?,
                item_order: required_string(&values, "item_order")?,
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
    pub round_count: u8,
    pub item_count: u8,
    pub first_priority_seat: String,
    pub item_order: String,
    pub terminal_scoring: String,
}

impl Variant {
    pub fn secret_draft_standard() -> Self {
        Self {
            id: VARIANT_ID.to_owned(),
            display_name: "Veiled Draft".to_owned(),
            rules_version_label: RULES_VERSION_LABEL.to_owned(),
            seat_count: STANDARD_SEAT_COUNT,
            round_count: STANDARD_ROUND_COUNT,
            item_count: STANDARD_ITEM_COUNT,
            first_priority_seat: "seat_0".to_owned(),
            item_order: "standard_fixed_v1".to_owned(),
            terminal_scoring: "score_sets_highest_threads_conflict_wins_else_draw".to_owned(),
        }
    }

    pub fn resolve(id: &str) -> Result<Self, String> {
        match id {
            VARIANT_ID => Ok(Self::secret_draft_standard()),
            _ => Err(format!("unsupported secret_draft variant `{id}`")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fixture {
    pub fixture_id: String,
    pub game_id: String,
    pub variant: String,
    pub rules_version: String,
    pub round_number: u8,
    pub priority_seat: String,
    pub visible_pool: [DraftItemId; 12],
    pub seat_0_commitment: String,
    pub seat_1_commitment: String,
    pub seat_0_drafted: String,
    pub seat_1_drafted: String,
    pub seat_0_score: u8,
    pub seat_1_score: u8,
    pub terminal_outcome: String,
    pub purpose: String,
    pub notes: String,
}

impl Fixture {
    pub fn parse(input: &str) -> Result<Self, String> {
        let values = parse_flat_json_object(input)?;
        reject_unknown_keys(
            &values,
            &[
                "fixture_id",
                "game_id",
                "variant",
                "rules_version",
                "round_number",
                "priority_seat",
                "visible_pool",
                "seat_0_commitment",
                "seat_1_commitment",
                "seat_0_drafted",
                "seat_1_drafted",
                "seat_0_score",
                "seat_1_score",
                "terminal_outcome",
                "purpose",
                "notes",
            ],
        )?;

        Ok(Self {
            fixture_id: required_string(&values, "fixture_id")?,
            game_id: required_string(&values, "game_id")?,
            variant: required_string(&values, "variant")?,
            rules_version: required_string(&values, "rules_version")?,
            round_number: required_u8(&values, "round_number")?,
            priority_seat: required_string(&values, "priority_seat")?,
            visible_pool: parse_item_order(&required_string(&values, "visible_pool")?)?,
            seat_0_commitment: required_string(&values, "seat_0_commitment")?,
            seat_1_commitment: required_string(&values, "seat_1_commitment")?,
            seat_0_drafted: required_string(&values, "seat_0_drafted")?,
            seat_1_drafted: required_string(&values, "seat_1_drafted")?,
            seat_0_score: required_u8(&values, "seat_0_score")?,
            seat_1_score: required_u8(&values, "seat_1_score")?,
            terminal_outcome: required_string(&values, "terminal_outcome")?,
            purpose: required_string(&values, "purpose")?,
            notes: required_string(&values, "notes")?,
        })
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

fn parse_flat_json_object(input: &str) -> Result<BTreeMap<String, String>, String> {
    let trimmed = input.trim();
    let body = trimmed
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .ok_or_else(|| "fixture must be a flat JSON object".to_owned())?;
    let mut values = BTreeMap::new();

    for (line_index, raw_line) in body.lines().enumerate() {
        let line = raw_line.trim().trim_end_matches(',').trim();
        if line.is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once(':')
            .ok_or_else(|| format!("expected key/value on JSON line {}", line_index + 1))?;
        let key = parse_json_string(key.trim())?;
        reject_behavior_key(&key)?;
        if values.contains_key(&key) {
            return Err(format!("duplicate key `{key}`"));
        }
        values.insert(key, parse_json_scalar(value.trim())?);
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

fn parse_json_scalar(value: &str) -> Result<String, String> {
    if value.starts_with('"') {
        return parse_json_string(value);
    }
    if value.chars().all(|ch| ch.is_ascii_digit()) {
        return Ok(value.to_owned());
    }
    Err(format!("unsupported JSON value `{value}`"))
}

fn parse_json_string(value: &str) -> Result<String, String> {
    value
        .strip_prefix('"')
        .and_then(|inner| inner.strip_suffix('"'))
        .map(str::to_owned)
        .ok_or_else(|| format!("expected JSON string, got `{value}`"))
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

fn parse_item_order(value: &str) -> Result<[DraftItemId; 12], String> {
    let parsed = value
        .split(',')
        .map(|part| {
            DraftItemId::parse(part.trim())
                .ok_or_else(|| format!("unknown draft item id `{}`", part.trim()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    parsed
        .try_into()
        .map_err(|_| "visible_pool must contain exactly twelve items".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::GAME_ID;

    #[test]
    fn behavior_looking_keys_are_rejected() {
        assert!(VariantCatalog::parse(
            "variant_id = \"secret_draft_standard\"\nvalid_if = \"bad\"\n"
        )
        .is_err());
        assert!(
            Fixture::parse("{\n  \"fixture_id\": \"x\",\n  \"on_reveal\": \"bad\"\n}").is_err()
        );
    }

    #[test]
    fn constants_match_static_data() {
        let manifest = Manifest::parse(include_str!("../data/manifest.toml")).unwrap();
        let variants = VariantCatalog::parse(include_str!("../data/variants.toml")).unwrap();

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.round_count, STANDARD_ROUND_COUNT);
        assert_eq!(manifest.item_count, STANDARD_ITEM_COUNT);
        assert_eq!(variants.selected, Variant::secret_draft_standard());
    }

    #[test]
    fn standard_variant_resolves_by_id() {
        assert_eq!(
            Variant::resolve(VARIANT_ID).expect("standard variant resolves"),
            Variant::secret_draft_standard()
        );
        assert!(Variant::resolve("other").is_err());
    }
}
