use std::collections::BTreeMap;

use crate::ids::{
    FactionId, SiteId, RULES_VERSION_LABEL, STANDARD_ACTION_BUDGET, STANDARD_ROUND_COUNT,
    STANDARD_SEAT_COUNT, UNIT_CAP_PER_SITE, VARIANT_HIGHLANDS_ID, VARIANT_STANDARD_ID,
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
    "effect",
    "on_play",
    "on_reveal",
    "formula",
    "score_formula",
    "movement_formula",
    "clash_formula",
    "supply_formula",
    "control_formula",
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
    pub site_ids: Vec<SiteId>,
    pub site_labels: Vec<String>,
    pub faction_ids: Vec<FactionId>,
    pub faction_labels: Vec<String>,
    pub unit_cap_per_site: u8,
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
                "site_ids",
                "site_labels",
                "faction_ids",
                "faction_labels",
                "unit_cap_per_site",
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
            site_ids: parse_site_list(&required_string(&values, "site_ids")?)?,
            site_labels: parse_string_list(&required_string(&values, "site_labels")?),
            faction_ids: parse_faction_list(&required_string(&values, "faction_ids")?)?,
            faction_labels: parse_string_list(&required_string(&values, "faction_labels")?),
            unit_cap_per_site: required_u8(&values, "unit_cap_per_site")?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantCatalog {
    pub standard: VariantMap,
    pub highlands: VariantMap,
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
                "standard_action_budget",
                "standard_round_count",
                "standard_unit_cap_per_site",
                "standard_edges",
                "standard_fort_sites",
                "standard_base_camp",
                "standard_stake_values",
                "standard_start_guards",
                "standard_start_crews",
                "standard_faction_order",
                "standard_terminal_outcomes",
                "highlands_variant_id",
                "highlands_display_name",
                "highlands_description",
                "highlands_rules_version_label",
                "highlands_seat_count",
                "highlands_action_budget",
                "highlands_round_count",
                "highlands_unit_cap_per_site",
                "highlands_edges",
                "highlands_fort_sites",
                "highlands_base_camp",
                "highlands_stake_values",
                "highlands_start_guards",
                "highlands_start_crews",
                "highlands_faction_order",
                "highlands_terminal_outcomes",
            ],
        )?;

        Ok(Self {
            standard: parse_variant(&values, "standard")?,
            highlands: parse_variant(&values, "highlands")?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantMap {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub rules_version_label: String,
    pub seat_count: u8,
    pub action_budget: u8,
    pub round_count: u8,
    pub unit_cap_per_site: u8,
    pub edges: Vec<(SiteId, SiteId)>,
    pub fort_sites: Vec<SiteId>,
    pub base_camp: SiteId,
    pub stake_values: Vec<(SiteId, u8)>,
    pub start_units: StartUnits,
    pub faction_order: [FactionId; 2],
    pub terminal_outcomes: String,
}

impl VariantMap {
    pub fn standard() -> Self {
        Self {
            id: VARIANT_STANDARD_ID.to_owned(),
            display_name: "Frontier Control".to_owned(),
            description: Some(
                "Classic asymmetric map fight with clear stakes and balanced pressure.".to_owned(),
            ),
            rules_version_label: RULES_VERSION_LABEL.to_owned(),
            seat_count: STANDARD_SEAT_COUNT,
            action_budget: STANDARD_ACTION_BUDGET,
            round_count: STANDARD_ROUND_COUNT,
            unit_cap_per_site: UNIT_CAP_PER_SITE,
            edges: parse_edges(STANDARD_EDGES).expect("standard edges are valid"),
            fort_sites: parse_site_list("site_gatehouse,site_signal_hill")
                .expect("standard fort sites are valid"),
            base_camp: SiteId::BaseCamp,
            stake_values: parse_stake_values(STANDARD_STAKE_VALUES)
                .expect("standard stake values are valid"),
            start_units: StartUnits {
                guards: parse_unit_counts(STANDARD_START_GUARDS)
                    .expect("standard guard starts are valid"),
                crews: parse_unit_counts(STANDARD_START_CREWS)
                    .expect("standard crew starts are valid"),
            },
            faction_order: [FactionId::Garrison, FactionId::Prospectors],
            terminal_outcomes: "score_compare_garrison_tiebreak".to_owned(),
        }
    }

    pub fn highlands() -> Self {
        Self {
            id: VARIANT_HIGHLANDS_ID.to_owned(),
            display_name: "Frontier Control: Highlands".to_owned(),
            description: Some(
                "Highlands shifts table pressure toward quarry routes and high ground.".to_owned(),
            ),
            rules_version_label: RULES_VERSION_LABEL.to_owned(),
            seat_count: STANDARD_SEAT_COUNT,
            action_budget: STANDARD_ACTION_BUDGET,
            round_count: 7,
            unit_cap_per_site: UNIT_CAP_PER_SITE,
            edges: parse_edges(HIGHLANDS_EDGES).expect("highlands edges are valid"),
            fort_sites: parse_site_list("site_gatehouse,site_quarry")
                .expect("highlands fort sites are valid"),
            base_camp: SiteId::BaseCamp,
            stake_values: parse_stake_values(HIGHLANDS_STAKE_VALUES)
                .expect("highlands stake values are valid"),
            start_units: StartUnits {
                guards: parse_unit_counts(HIGHLANDS_START_GUARDS)
                    .expect("highlands guard starts are valid"),
                crews: parse_unit_counts(HIGHLANDS_START_CREWS)
                    .expect("highlands crew starts are valid"),
            },
            faction_order: [FactionId::Garrison, FactionId::Prospectors],
            terminal_outcomes: "score_compare_garrison_tiebreak".to_owned(),
        }
    }

    pub fn resolve(id: &str) -> Result<Self, String> {
        match id {
            VARIANT_STANDARD_ID => Ok(Self::standard()),
            VARIANT_HIGHLANDS_ID => Ok(Self::highlands()),
            _ => Err(format!("unsupported frontier_control variant `{id}`")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartUnits {
    pub guards: Vec<(SiteId, u8)>,
    pub crews: Vec<(SiteId, u8)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SiteDefinition {
    pub id: SiteId,
    pub label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fixture {
    pub fixture_id: String,
    pub game_id: String,
    pub variant: String,
    pub rules_version: String,
    pub phase: String,
    pub active_seat: String,
    pub action_budget: u8,
    pub round_count: u8,
    pub unit_cap_per_site: u8,
    pub edges: Vec<(SiteId, SiteId)>,
    pub fort_sites: Vec<SiteId>,
    pub base_camp: SiteId,
    pub stake_values: Vec<(SiteId, u8)>,
    pub start_units: StartUnits,
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
                "phase",
                "active_seat",
                "action_budget",
                "round_count",
                "unit_cap_per_site",
                "edges",
                "fort_sites",
                "base_camp",
                "stake_values",
                "start_guards",
                "start_crews",
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
            phase: required_string(&values, "phase")?,
            active_seat: required_string(&values, "active_seat")?,
            action_budget: required_u8(&values, "action_budget")?,
            round_count: required_u8(&values, "round_count")?,
            unit_cap_per_site: required_u8(&values, "unit_cap_per_site")?,
            edges: parse_edges(&required_string(&values, "edges")?)?,
            fort_sites: parse_site_list(&required_string(&values, "fort_sites")?)?,
            base_camp: parse_site(&required_string(&values, "base_camp")?)?,
            stake_values: parse_stake_values(&required_string(&values, "stake_values")?)?,
            start_units: StartUnits {
                guards: parse_unit_counts(&required_string(&values, "start_guards")?)?,
                crews: parse_unit_counts(&required_string(&values, "start_crews")?)?,
            },
            terminal_outcome: required_string(&values, "terminal_outcome")?,
            purpose: required_string(&values, "purpose")?,
            notes: required_string(&values, "notes")?,
        })
    }
}

const STANDARD_EDGES: &str = "site_gatehouse-site_ford,site_gatehouse-site_quarry,site_gatehouse-site_signal_hill,site_signal_hill-site_quarry,site_signal_hill-site_goldfield,site_quarry-site_ford,site_quarry-site_timberline,site_ford-site_base_camp,site_timberline-site_base_camp,site_timberline-site_goldfield";
const STANDARD_STAKE_VALUES: &str = "site_goldfield:3,site_quarry:2,site_ford:1,site_timberline:1";
const STANDARD_START_GUARDS: &str = "site_gatehouse:2,site_signal_hill:2";
const STANDARD_START_CREWS: &str = "site_base_camp:3";
const HIGHLANDS_EDGES: &str = "site_gatehouse-site_signal_hill,site_gatehouse-site_ford,site_signal_hill-site_quarry,site_signal_hill-site_goldfield,site_quarry-site_goldfield,site_quarry-site_timberline,site_quarry-site_ford,site_ford-site_base_camp,site_timberline-site_base_camp,site_timberline-site_goldfield";
const HIGHLANDS_STAKE_VALUES: &str = "site_goldfield:2,site_quarry:3,site_ford:1,site_timberline:2";
const HIGHLANDS_START_GUARDS: &str = "site_gatehouse:2,site_quarry:1";
const HIGHLANDS_START_CREWS: &str = "site_base_camp:2,site_timberline:1";

fn parse_variant(values: &BTreeMap<String, String>, prefix: &str) -> Result<VariantMap, String> {
    Ok(VariantMap {
        id: required_string(values, &format!("{prefix}_variant_id"))?,
        display_name: required_string(values, &format!("{prefix}_display_name"))?,
        description: optional_description(values, prefix)?,
        rules_version_label: required_string(values, &format!("{prefix}_rules_version_label"))?,
        seat_count: required_u8(values, &format!("{prefix}_seat_count"))?,
        action_budget: required_u8(values, &format!("{prefix}_action_budget"))?,
        round_count: required_u8(values, &format!("{prefix}_round_count"))?,
        unit_cap_per_site: required_u8(values, &format!("{prefix}_unit_cap_per_site"))?,
        edges: parse_edges(&required_string(values, &format!("{prefix}_edges"))?)?,
        fort_sites: parse_site_list(&required_string(values, &format!("{prefix}_fort_sites"))?)?,
        base_camp: parse_site(&required_string(values, &format!("{prefix}_base_camp"))?)?,
        stake_values: parse_stake_values(&required_string(
            values,
            &format!("{prefix}_stake_values"),
        )?)?,
        start_units: StartUnits {
            guards: parse_unit_counts(&required_string(
                values,
                &format!("{prefix}_start_guards"),
            )?)?,
            crews: parse_unit_counts(&required_string(values, &format!("{prefix}_start_crews"))?)?,
        },
        faction_order: parse_faction_order(&required_string(
            values,
            &format!("{prefix}_faction_order"),
        )?)?,
        terminal_outcomes: required_string(values, &format!("{prefix}_terminal_outcomes"))?,
    })
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

fn parse_string_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_owned)
        .collect()
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

fn parse_stake_values(value: &str) -> Result<Vec<(SiteId, u8)>, String> {
    parse_unit_counts(value)
}

fn parse_unit_counts(value: &str) -> Result<Vec<(SiteId, u8)>, String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{GAME_ID, STANDARD_SITE_COUNT};

    #[test]
    fn behavior_looking_keys_are_rejected() {
        assert!(VariantCatalog::parse(
            "standard_variant_id = \"frontier_control_standard\"\nvalid_if = \"bad\"\n"
        )
        .is_err());
        assert!(
            Manifest::parse("game_id = \"frontier_control\"\nscore_formula = \"bad\"\n").is_err()
        );
        assert!(
            Fixture::parse("{\n  \"fixture_id\": \"x\",\n  \"on_reveal\": \"bad\"\n}").is_err()
        );
    }

    #[test]
    fn unknown_fields_are_rejected() {
        assert!(Manifest::parse("game_id = \"frontier_control\"\nunknown = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "standard_variant_id = \"frontier_control_standard\"\nextra = \"bad\"\n"
        )
        .is_err());
        assert!(Fixture::parse("{\n  \"fixture_id\": \"x\",\n  \"extra\": \"bad\"\n}").is_err());
    }

    #[test]
    fn variant_descriptions_are_validated() {
        let variants = VariantCatalog::parse(include_str!("../data/variants.toml")).unwrap();
        assert!(variants.standard.description.as_deref().unwrap().len() <= 120);
        assert!(VariantCatalog::parse(
            "standard_variant_id = \"frontier_control_standard\"\nstandard_display_name = \"Frontier Control\"\nstandard_description = \"Choose this when control is easy.\"\n"
        )
        .is_err());
        assert!(VariantCatalog::parse(&format!(
            "standard_variant_id = \"frontier_control_standard\"\nstandard_display_name = \"Frontier Control\"\nstandard_description = \"{}\"\n",
            "a".repeat(121)
        ))
        .is_err());
    }

    #[test]
    fn constants_match_static_data() {
        let manifest = Manifest::parse(include_str!("../data/manifest.toml")).unwrap();
        let variants = VariantCatalog::parse(include_str!("../data/variants.toml")).unwrap();

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.site_count, STANDARD_SITE_COUNT);
        assert_eq!(manifest.site_ids, SiteId::ALL);
        assert_eq!(manifest.faction_ids, FactionId::ALL);
        assert_eq!(variants.standard, VariantMap::standard());
        assert_eq!(variants.highlands, VariantMap::highlands());
    }

    #[test]
    fn variants_resolve_by_id() {
        assert_eq!(
            VariantMap::resolve(VARIANT_STANDARD_ID).expect("standard variant resolves"),
            VariantMap::standard()
        );
        assert_eq!(
            VariantMap::resolve(VARIANT_HIGHLANDS_ID).expect("highlands variant resolves"),
            VariantMap::highlands()
        );
        assert!(VariantMap::resolve("other").is_err());
    }
}
