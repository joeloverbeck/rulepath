use std::collections::BTreeMap;

use crate::ids::{
    DATA_VERSION_LABEL, RULES_VERSION_LABEL, STANDARD_DEFAULT_SEATS, STANDARD_MAX_SEATS,
    STANDARD_MIN_SEATS, STANDARD_PEGS_PER_SEAT, SUPPORTED_SEAT_COUNTS, VARIANT_ID,
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
    "path_formula",
    "jump_formula",
    "adjacency_rule",
    "legal_if",
    "bot_policy",
];

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
                "max_plies",
                "pegs_per_seat",
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
                supported_seats: required_seat_set(&values, "supported_seats")?,
                max_plies: required_u32(&values, "max_plies")?,
                pegs_per_seat: required_u8(&values, "pegs_per_seat")?,
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
    pub supported_seats: Vec<u8>,
    pub max_plies: u32,
    pub pegs_per_seat: u8,
}

impl Variant {
    pub fn starbridge_classic() -> Self {
        Self {
            id: VARIANT_ID.to_owned(),
            display_name: "Starbridge Crossing".to_owned(),
            rules_version_label: RULES_VERSION_LABEL.to_owned(),
            data_version_label: DATA_VERSION_LABEL.to_owned(),
            min_seats: STANDARD_MIN_SEATS,
            default_seats: STANDARD_DEFAULT_SEATS,
            max_seats: STANDARD_MAX_SEATS,
            supported_seats: SUPPORTED_SEAT_COUNTS.to_vec(),
            max_plies: 2000,
            pegs_per_seat: STANDARD_PEGS_PER_SEAT,
        }
    }

    pub fn resolve(id: &str) -> Result<Self, String> {
        match id {
            VARIANT_ID => Ok(Self::starbridge_classic()),
            _ => Err(format!("unsupported starbridge_crossing variant `{id}`")),
        }
    }

    pub fn supports_seat_count(&self, seat_count: usize) -> bool {
        u8::try_from(seat_count)
            .ok()
            .is_some_and(|count| self.supported_seats.contains(&count))
    }
}

pub fn load_variants() -> Result<VariantCatalog, String> {
    VariantCatalog::parse(include_str!("../data/variants.toml"))
}

fn parse_flat_toml(input: &str) -> Result<BTreeMap<String, String>, String> {
    let mut values = BTreeMap::new();
    for (line_index, raw_line) in input.lines().enumerate() {
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((raw_key, raw_value)) = line.split_once('=') else {
            return Err(format!("line {} is not key = value", line_index + 1));
        };
        let key = raw_key.trim();
        if key.is_empty() {
            return Err(format!("line {} has an empty key", line_index + 1));
        }
        reject_behavior_key(key)?;
        if values
            .insert(
                key.to_owned(),
                parse_value(raw_value.trim(), line_index + 1)?,
            )
            .is_some()
        {
            return Err(format!("duplicate field `{key}`"));
        }
    }
    Ok(values)
}

fn parse_value(raw: &str, line: usize) -> Result<String, String> {
    if raw.starts_with('"') {
        if !raw.ends_with('"') || raw.len() == 1 {
            return Err(format!("line {line} has malformed quoted value"));
        }
        Ok(raw[1..raw.len() - 1].to_owned())
    } else {
        Ok(raw.to_owned())
    }
}

fn reject_behavior_key(key: &str) -> Result<(), String> {
    if key == "rules_version_label" {
        return Ok(());
    }
    if BEHAVIOR_KEYS.iter().any(|token| key.contains(token)) {
        return Err(format!("behavior-looking field `{key}` is not allowed"));
    }
    Ok(())
}

fn reject_unknown_keys(values: &BTreeMap<String, String>, allowed: &[&str]) -> Result<(), String> {
    for key in values.keys() {
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
        .ok_or_else(|| format!("missing `{key}`"))
}

fn required_u8(values: &BTreeMap<String, String>, key: &str) -> Result<u8, String> {
    required_string(values, key)?
        .parse()
        .map_err(|_| format!("`{key}` must be a u8"))
}

fn required_u32(values: &BTreeMap<String, String>, key: &str) -> Result<u32, String> {
    required_string(values, key)?
        .parse()
        .map_err(|_| format!("`{key}` must be a u32"))
}

fn required_seat_set(values: &BTreeMap<String, String>, key: &str) -> Result<Vec<u8>, String> {
    let raw = required_string(values, key)?;
    let seats = raw
        .split(',')
        .map(|part| {
            part.trim()
                .parse::<u8>()
                .map_err(|_| format!("`{key}` must be a comma-delimited u8 list"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if seats.is_empty() {
        return Err(format!("`{key}` must not be empty"));
    }
    Ok(seats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_manifest_matches_classic_variant() {
        let catalog = load_variants().expect("variant data parses");

        assert_eq!(catalog.selected, Variant::starbridge_classic());
    }

    #[test]
    fn variant_manifest_rejects_unknown_and_behavior_fields() {
        assert!(VariantCatalog::parse(
            "variant_id = \"starbridge_crossing_classic_star_v1\"\nunknown = \"bad\"\n"
        )
        .is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"starbridge_crossing_classic_star_v1\"\nlegal_if = \"bad\"\n"
        )
        .is_err());
    }
}
