use std::collections::BTreeMap;

use crate::ids::{
    FactionId, SiteId, VARIANT_HARD_WINTER_ID, VARIANT_LAND_RUSH_ID, VARIANT_STANDARD_ID,
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
    "action",
    "legal",
    "on_play",
    "on_reveal",
    "effect",
    "formula",
    "score_formula",
    "event_formula",
    "edict_formula",
    "eligibility_formula",
    "victory_formula",
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
    pub seat_count: u8,
    pub site_count: u8,
    pub card_count: u8,
    pub epoch_count: u8,
    pub resource_cap: u8,
    pub site_ids: Vec<SiteId>,
    pub site_labels: Vec<String>,
    pub faction_ids: Vec<FactionId>,
    pub faction_labels: Vec<String>,
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
                "site_count",
                "card_count",
                "epoch_count",
                "resource_cap",
                "site_ids",
                "site_labels",
                "faction_ids",
                "faction_labels",
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
            site_count: required_u8(&values, "site_count")?,
            card_count: required_u8(&values, "card_count")?,
            epoch_count: required_u8(&values, "epoch_count")?,
            resource_cap: required_u8(&values, "resource_cap")?,
            site_ids: parse_site_list(&required_string(&values, "site_ids")?)?,
            site_labels: parse_string_list(&required_string(&values, "site_labels")?),
            faction_ids: parse_faction_list(&required_string(&values, "faction_ids")?)?,
            faction_labels: parse_string_list(&required_string(&values, "faction_labels")?),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantCatalog {
    pub standard: ScenarioVariant,
    pub hard_winter: ScenarioVariant,
    pub land_rush: ScenarioVariant,
}

impl VariantCatalog {
    pub fn parse(input: &str) -> Result<Self, String> {
        let values = parse_flat_toml(input)?;
        reject_unknown_keys(
            &values,
            &[
                "standard_variant_id",
                "standard_display_name",
                "standard_description",
                "standard_rules_version_label",
                "standard_seat_count",
                "standard_starting_resources",
                "standard_resource_cap",
                "standard_charter_site_threshold",
                "standard_freeholder_cache_threshold",
                "standard_epoch_composition",
                "standard_start_agents",
                "standard_start_depots",
                "standard_start_settlers",
                "standard_start_caches",
                "standard_edges",
                "standard_faction_order",
                "hard_winter_variant_id",
                "hard_winter_display_name",
                "hard_winter_description",
                "hard_winter_rules_version_label",
                "hard_winter_seat_count",
                "hard_winter_starting_resources",
                "hard_winter_resource_cap",
                "hard_winter_charter_site_threshold",
                "hard_winter_freeholder_cache_threshold",
                "hard_winter_epoch_composition",
                "hard_winter_start_agents",
                "hard_winter_start_depots",
                "hard_winter_start_settlers",
                "hard_winter_start_caches",
                "hard_winter_edges",
                "hard_winter_faction_order",
                "land_rush_variant_id",
                "land_rush_display_name",
                "land_rush_description",
                "land_rush_rules_version_label",
                "land_rush_seat_count",
                "land_rush_starting_resources",
                "land_rush_resource_cap",
                "land_rush_charter_site_threshold",
                "land_rush_freeholder_cache_threshold",
                "land_rush_epoch_composition",
                "land_rush_start_agents",
                "land_rush_start_depots",
                "land_rush_start_settlers",
                "land_rush_start_caches",
                "land_rush_edges",
                "land_rush_faction_order",
            ],
        )?;

        Ok(Self {
            standard: parse_variant(&values, "standard")?,
            hard_winter: parse_variant(&values, "hard_winter")?,
            land_rush: parse_variant(&values, "land_rush")?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScenarioVariant {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub rules_version_label: String,
    pub seat_count: u8,
    pub starting_resources: (u8, u8),
    pub resource_cap: u8,
    pub charter_site_threshold: u8,
    pub freeholder_cache_threshold: u8,
    pub epoch_composition: Vec<String>,
    pub start_agents: Vec<(SiteId, u8)>,
    pub start_depots: Vec<SiteId>,
    pub start_settlers: Vec<(SiteId, u8)>,
    pub start_caches: Vec<(SiteId, u8)>,
    pub edges: Vec<(SiteId, SiteId)>,
    pub faction_order: [FactionId; 2],
}

impl ScenarioVariant {
    pub fn resolve(id: &str) -> Result<Self, String> {
        let variants = VariantCatalog::parse(include_str!("../data/variants.toml"))?;
        match id {
            VARIANT_STANDARD_ID => Ok(variants.standard),
            VARIANT_HARD_WINTER_ID => Ok(variants.hard_winter),
            VARIANT_LAND_RUSH_ID => Ok(variants.land_rush),
            _ => Err(format!("unsupported event_frontier variant `{id}`")),
        }
    }
}

fn parse_variant(
    values: &BTreeMap<String, String>,
    prefix: &str,
) -> Result<ScenarioVariant, String> {
    Ok(ScenarioVariant {
        id: required_string(values, &format!("{prefix}_variant_id"))?,
        display_name: required_string(values, &format!("{prefix}_display_name"))?,
        description: optional_description(values, prefix)?,
        rules_version_label: required_string(values, &format!("{prefix}_rules_version_label"))?,
        seat_count: required_u8(values, &format!("{prefix}_seat_count"))?,
        starting_resources: parse_pair(&required_string(
            values,
            &format!("{prefix}_starting_resources"),
        )?)?,
        resource_cap: required_u8(values, &format!("{prefix}_resource_cap"))?,
        charter_site_threshold: required_u8(values, &format!("{prefix}_charter_site_threshold"))?,
        freeholder_cache_threshold: required_u8(
            values,
            &format!("{prefix}_freeholder_cache_threshold"),
        )?,
        epoch_composition: parse_string_list(&required_string(
            values,
            &format!("{prefix}_epoch_composition"),
        )?),
        start_agents: parse_site_counts(&required_string(
            values,
            &format!("{prefix}_start_agents"),
        )?)?,
        start_depots: parse_site_list(&required_string(
            values,
            &format!("{prefix}_start_depots"),
        )?)?,
        start_settlers: parse_site_counts(&required_string(
            values,
            &format!("{prefix}_start_settlers"),
        )?)?,
        start_caches: parse_site_counts(&required_string(
            values,
            &format!("{prefix}_start_caches"),
        )?)?,
        edges: parse_edges(&required_string(values, &format!("{prefix}_edges"))?)?,
        faction_order: parse_faction_order(&required_string(
            values,
            &format!("{prefix}_faction_order"),
        )?)?,
    })
}

pub fn parse_flat_toml(input: &str) -> Result<BTreeMap<String, String>, String> {
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

fn optional_description(
    values: &BTreeMap<String, String>,
    prefix: &str,
) -> Result<Option<String>, String> {
    let key = format!("{prefix}_description");
    values
        .get(&key)
        .map(|value| validate_description(value, &key))
        .transpose()
}

fn validate_description(value: &str, key: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("key `{key}` must not be empty"));
    }
    if trimmed.chars().count() > 120 {
        return Err(format!("key `{key}` must be at most 120 characters"));
    }
    if trimmed.contains('_') {
        return Err(format!("key `{key}` must not contain raw identifiers"));
    }
    let lower = trimmed.to_ascii_lowercase();
    for token in [
        "if", "when", "then", "selector", "trigger", "valid_if", "legal", "effect", "action",
    ] {
        if lower
            .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
            .any(|part| part == token)
        {
            return Err(format!(
                "key `{key}` contains behavior-looking prose token `{token}`"
            ));
        }
    }
    Ok(trimmed.to_owned())
}

pub fn reject_unknown_keys(
    values: &BTreeMap<String, String>,
    allowed: &[&str],
) -> Result<(), String> {
    for key in values.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(format!("unknown key `{key}`"));
        }
    }
    Ok(())
}

pub fn required_string(values: &BTreeMap<String, String>, key: &str) -> Result<String, String> {
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

pub fn parse_string_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_owned)
        .collect()
}

fn parse_pair(value: &str) -> Result<(u8, u8), String> {
    let parts = parse_string_list(value);
    if parts.len() != 2 {
        return Err("pair must contain exactly two values".to_owned());
    }
    Ok((
        parts[0]
            .parse::<u8>()
            .map_err(|_| format!("invalid integer `{}`", parts[0]))?,
        parts[1]
            .parse::<u8>()
            .map_err(|_| format!("invalid integer `{}`", parts[1]))?,
    ))
}

fn parse_site(value: &str) -> Result<SiteId, String> {
    SiteId::parse(value).ok_or_else(|| format!("unknown site `{value}`"))
}

fn parse_site_list(value: &str) -> Result<Vec<SiteId>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| parse_site(&part))
        .collect()
}

fn parse_faction_list(value: &str) -> Result<Vec<FactionId>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| FactionId::parse(&part).ok_or_else(|| format!("unknown faction `{part}`")))
        .collect()
}

fn parse_faction_order(value: &str) -> Result<[FactionId; 2], String> {
    let parsed = parse_faction_list(value)?;
    parsed
        .try_into()
        .map_err(|_| "faction_order must contain exactly two factions".to_owned())
}

fn parse_site_counts(value: &str) -> Result<Vec<(SiteId, u8)>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| {
            let (site, count) = part
                .split_once(':')
                .ok_or_else(|| format!("site count `{part}` must use `site:value`"))?;
            let count = count
                .parse::<u8>()
                .map_err(|_| format!("invalid count `{count}`"))?;
            Ok((parse_site(site)?, count))
        })
        .collect()
}

fn parse_edges(value: &str) -> Result<Vec<(SiteId, SiteId)>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| {
            let (left, right) = part
                .split_once('-')
                .ok_or_else(|| format!("edge `{part}` must use `site_a-site_b`"))?;
            Ok((parse_site(left)?, parse_site(right)?))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{
        RULES_VERSION_LABEL, STANDARD_CARD_COUNT, STANDARD_EPOCH_COUNT, STANDARD_RESOURCE_CAP,
        STANDARD_SEAT_COUNT, STANDARD_SITE_COUNT,
    };

    #[test]
    fn behavior_and_unknown_keys_are_rejected() {
        assert!(Manifest::parse("game_id = \"event_frontier\"\nselector = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "standard_variant_id = \"event_frontier_standard\"\nvalid_if = \"bad\"\n"
        )
        .is_err());
        assert!(Manifest::parse("game_id = \"event_frontier\"\nextra = \"bad\"\n").is_err());
    }

    #[test]
    fn variant_descriptions_are_validated() {
        let variants = VariantCatalog::parse(include_str!("../data/variants.toml")).unwrap();
        assert!(variants.standard.description.as_deref().unwrap().len() <= 120);
        assert!(VariantCatalog::parse(
            "standard_variant_id = \"event_frontier_standard\"\nstandard_display_name = \"Event Frontier\"\nstandard_description = \"Play this when the event trigger is visible.\"\n"
        )
        .is_err());
        assert!(VariantCatalog::parse(&format!(
            "standard_variant_id = \"event_frontier_standard\"\nstandard_display_name = \"Event Frontier\"\nstandard_description = \"{}\"\n",
            "a".repeat(121)
        ))
        .is_err());
    }

    #[test]
    fn constants_match_static_data() {
        let manifest = Manifest::parse(include_str!("../data/manifest.toml")).unwrap();
        let variants = VariantCatalog::parse(include_str!("../data/variants.toml")).unwrap();

        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.site_count, STANDARD_SITE_COUNT);
        assert_eq!(manifest.card_count, STANDARD_CARD_COUNT);
        assert_eq!(manifest.epoch_count, STANDARD_EPOCH_COUNT);
        assert_eq!(manifest.resource_cap, STANDARD_RESOURCE_CAP);
        assert_eq!(manifest.site_ids, SiteId::ALL);
        assert_eq!(manifest.faction_ids, FactionId::ALL);
        assert_eq!(variants.standard.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(variants.standard.resource_cap, STANDARD_RESOURCE_CAP);
    }

    #[test]
    fn variants_resolve_by_id() {
        assert_eq!(
            ScenarioVariant::resolve(VARIANT_STANDARD_ID)
                .expect("standard resolves")
                .id,
            VARIANT_STANDARD_ID
        );
        assert_eq!(
            ScenarioVariant::resolve(VARIANT_HARD_WINTER_ID)
                .expect("hard winter resolves")
                .id,
            VARIANT_HARD_WINTER_ID
        );
        assert!(ScenarioVariant::resolve("other").is_err());
    }
}
