use std::collections::BTreeMap;

use crate::ids::{
    DistrictId, FloodWatchRole, RULES_VERSION_LABEL, STANDARD_ACTION_BUDGET,
    STANDARD_DISTRICT_COUNT, STANDARD_DOWNPOURS_PER_DISTRICT, STANDARD_DRAWS_PER_PHASE,
    STANDARD_LEVEE_CAP, STANDARD_MAX_FLOOD_LEVEL, STANDARD_REPRIEVE_COUNT, STANDARD_SEAT_COUNT,
    STANDARD_SURGES_PER_DISTRICT, VARIANT_DELUGE_ID, VARIANT_STANDARD_ID,
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
    "event_formula",
    "automation_formula",
    "role_formula",
    "budget_formula",
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
    pub district_count: u8,
    pub district_ids: Vec<DistrictId>,
    pub district_labels: Vec<String>,
    pub role_ids: Vec<FloodWatchRole>,
    pub role_labels: Vec<String>,
    pub event_kinds: Vec<String>,
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
                "district_count",
                "district_ids",
                "district_labels",
                "role_ids",
                "role_labels",
                "event_kinds",
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
            district_count: required_u8(&values, "district_count")?,
            district_ids: parse_district_list(&required_string(&values, "district_ids")?)?,
            district_labels: parse_string_list(&required_string(&values, "district_labels")?),
            role_ids: parse_role_list(&required_string(&values, "role_ids")?)?,
            role_labels: parse_string_list(&required_string(&values, "role_labels")?),
            event_kinds: parse_string_list(&required_string(&values, "event_kinds")?),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantCatalog {
    pub standard: ScenarioVariant,
    pub deluge: ScenarioVariant,
}

impl VariantCatalog {
    pub fn parse(input: &str) -> Result<Self, String> {
        let values = parse_flat_toml(input)?;
        reject_unknown_keys(
            &values,
            &[
                "standard_variant_id",
                "standard_display_name",
                "standard_rules_version_label",
                "standard_seat_count",
                "standard_action_budget",
                "standard_draws_per_phase",
                "standard_levee_cap",
                "standard_max_flood_level",
                "standard_starting_levels",
                "standard_downpours_per_district",
                "standard_surges_per_district",
                "standard_reprieves",
                "standard_role_order",
                "standard_terminal_outcomes",
                "deluge_variant_id",
                "deluge_display_name",
                "deluge_rules_version_label",
                "deluge_seat_count",
                "deluge_action_budget",
                "deluge_draws_per_phase",
                "deluge_levee_cap",
                "deluge_max_flood_level",
                "deluge_starting_levels",
                "deluge_downpours_per_district",
                "deluge_surges_per_district",
                "deluge_reprieves",
                "deluge_role_order",
                "deluge_terminal_outcomes",
            ],
        )?;

        Ok(Self {
            standard: parse_variant(&values, "standard")?,
            deluge: parse_variant(&values, "deluge")?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScenarioVariant {
    pub id: String,
    pub display_name: String,
    pub rules_version_label: String,
    pub seat_count: u8,
    pub action_budget: u8,
    pub draws_per_phase: u8,
    pub levee_cap: u8,
    pub max_flood_level: u8,
    pub starting_levels: [u8; 5],
    pub event_composition: EventComposition,
    pub role_order: [FloodWatchRole; 2],
    pub terminal_outcomes: String,
}

impl ScenarioVariant {
    pub fn standard() -> Self {
        Self {
            id: VARIANT_STANDARD_ID.to_owned(),
            display_name: "Flood Watch".to_owned(),
            rules_version_label: RULES_VERSION_LABEL.to_owned(),
            seat_count: STANDARD_SEAT_COUNT,
            action_budget: STANDARD_ACTION_BUDGET,
            draws_per_phase: STANDARD_DRAWS_PER_PHASE,
            levee_cap: STANDARD_LEVEE_CAP,
            max_flood_level: STANDARD_MAX_FLOOD_LEVEL,
            starting_levels: [0, 1, 0, 1, 0],
            event_composition: EventComposition::standard(),
            role_order: [FloodWatchRole::Pumpwright, FloodWatchRole::LeveeWarden],
            terminal_outcomes: "shared_win_shared_loss".to_owned(),
        }
    }

    pub fn deluge() -> Self {
        Self {
            id: VARIANT_DELUGE_ID.to_owned(),
            display_name: "Flood Watch: Deluge".to_owned(),
            rules_version_label: RULES_VERSION_LABEL.to_owned(),
            seat_count: STANDARD_SEAT_COUNT,
            action_budget: STANDARD_ACTION_BUDGET,
            draws_per_phase: STANDARD_DRAWS_PER_PHASE,
            levee_cap: STANDARD_LEVEE_CAP,
            max_flood_level: STANDARD_MAX_FLOOD_LEVEL,
            starting_levels: [1, 1, 1, 2, 1],
            event_composition: EventComposition {
                downpours_per_district: 3,
                surges_per_district: 2,
                reprieves: 2,
            },
            role_order: [FloodWatchRole::Pumpwright, FloodWatchRole::LeveeWarden],
            terminal_outcomes: "shared_win_shared_loss".to_owned(),
        }
    }

    pub fn resolve(id: &str) -> Result<Self, String> {
        match id {
            VARIANT_STANDARD_ID => Ok(Self::standard()),
            VARIANT_DELUGE_ID => Ok(Self::deluge()),
            _ => Err(format!("unsupported flood_watch variant `{id}`")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventComposition {
    pub downpours_per_district: u8,
    pub surges_per_district: u8,
    pub reprieves: u8,
}

impl EventComposition {
    pub const fn standard() -> Self {
        Self {
            downpours_per_district: STANDARD_DOWNPOURS_PER_DISTRICT,
            surges_per_district: STANDARD_SURGES_PER_DISTRICT,
            reprieves: STANDARD_REPRIEVE_COUNT,
        }
    }

    pub const fn total_cards(&self) -> u8 {
        (self.downpours_per_district + self.surges_per_district) * STANDARD_DISTRICT_COUNT
            + self.reprieves
    }
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
    pub draws_per_phase: u8,
    pub levee_cap: u8,
    pub max_flood_level: u8,
    pub starting_levels: [u8; 5],
    pub event_composition: EventComposition,
    pub event_deck_order_status: String,
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
                "draws_per_phase",
                "levee_cap",
                "max_flood_level",
                "starting_levels",
                "downpours_per_district",
                "surges_per_district",
                "reprieves",
                "event_deck_order_status",
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
            draws_per_phase: required_u8(&values, "draws_per_phase")?,
            levee_cap: required_u8(&values, "levee_cap")?,
            max_flood_level: required_u8(&values, "max_flood_level")?,
            starting_levels: parse_levels(&required_string(&values, "starting_levels")?)?,
            event_composition: EventComposition {
                downpours_per_district: required_u8(&values, "downpours_per_district")?,
                surges_per_district: required_u8(&values, "surges_per_district")?,
                reprieves: required_u8(&values, "reprieves")?,
            },
            event_deck_order_status: required_string(&values, "event_deck_order_status")?,
            terminal_outcome: required_string(&values, "terminal_outcome")?,
            purpose: required_string(&values, "purpose")?,
            notes: required_string(&values, "notes")?,
        })
    }
}

fn parse_variant(
    values: &BTreeMap<String, String>,
    prefix: &str,
) -> Result<ScenarioVariant, String> {
    Ok(ScenarioVariant {
        id: required_string(values, &format!("{prefix}_variant_id"))?,
        display_name: required_string(values, &format!("{prefix}_display_name"))?,
        rules_version_label: required_string(values, &format!("{prefix}_rules_version_label"))?,
        seat_count: required_u8(values, &format!("{prefix}_seat_count"))?,
        action_budget: required_u8(values, &format!("{prefix}_action_budget"))?,
        draws_per_phase: required_u8(values, &format!("{prefix}_draws_per_phase"))?,
        levee_cap: required_u8(values, &format!("{prefix}_levee_cap"))?,
        max_flood_level: required_u8(values, &format!("{prefix}_max_flood_level"))?,
        starting_levels: parse_levels(&required_string(
            values,
            &format!("{prefix}_starting_levels"),
        )?)?,
        event_composition: EventComposition {
            downpours_per_district: required_u8(
                values,
                &format!("{prefix}_downpours_per_district"),
            )?,
            surges_per_district: required_u8(values, &format!("{prefix}_surges_per_district"))?,
            reprieves: required_u8(values, &format!("{prefix}_reprieves"))?,
        },
        role_order: parse_role_order(&required_string(values, &format!("{prefix}_role_order"))?)?,
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

fn parse_district_list(value: &str) -> Result<Vec<DistrictId>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| DistrictId::parse(&part).ok_or_else(|| format!("unknown district `{part}`")))
        .collect()
}

fn parse_role_list(value: &str) -> Result<Vec<FloodWatchRole>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| FloodWatchRole::parse(&part).ok_or_else(|| format!("unknown role `{part}`")))
        .collect()
}

fn parse_levels(value: &str) -> Result<[u8; 5], String> {
    let parsed = value
        .split(',')
        .map(|part| {
            part.trim()
                .parse::<u8>()
                .map_err(|_| format!("invalid level `{}`", part.trim()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    parsed
        .try_into()
        .map_err(|_| "starting_levels must contain exactly five levels".to_owned())
}

fn parse_role_order(value: &str) -> Result<[FloodWatchRole; 2], String> {
    let parsed = parse_role_list(value)?;
    parsed
        .try_into()
        .map_err(|_| "role_order must contain exactly two roles".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{GAME_ID, STANDARD_DECK_SIZE};

    #[test]
    fn behavior_looking_keys_are_rejected() {
        assert!(VariantCatalog::parse(
            "standard_variant_id = \"flood_watch_standard\"\nvalid_if = \"bad\"\n"
        )
        .is_err());
        assert!(Manifest::parse("game_id = \"flood_watch\"\nevent_formula = \"bad\"\n").is_err());
        assert!(
            Fixture::parse("{\n  \"fixture_id\": \"x\",\n  \"on_reveal\": \"bad\"\n}").is_err()
        );
    }

    #[test]
    fn unknown_fields_are_rejected() {
        assert!(Manifest::parse("game_id = \"flood_watch\"\nunknown = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "standard_variant_id = \"flood_watch_standard\"\nextra = \"bad\"\n"
        )
        .is_err());
        assert!(Fixture::parse("{\n  \"fixture_id\": \"x\",\n  \"extra\": \"bad\"\n}").is_err());
    }

    #[test]
    fn constants_match_static_data() {
        let manifest = Manifest::parse(include_str!("../data/manifest.toml")).unwrap();
        let variants = VariantCatalog::parse(include_str!("../data/variants.toml")).unwrap();

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.district_count, STANDARD_DISTRICT_COUNT);
        assert_eq!(manifest.district_ids, DistrictId::ALL);
        assert_eq!(manifest.role_ids, FloodWatchRole::ALL);
        assert_eq!(variants.standard, ScenarioVariant::standard());
        assert_eq!(variants.deluge, ScenarioVariant::deluge());
        assert_eq!(
            variants.standard.event_composition.total_cards(),
            STANDARD_DECK_SIZE
        );
    }

    #[test]
    fn variants_resolve_by_id() {
        assert_eq!(
            ScenarioVariant::resolve(VARIANT_STANDARD_ID).expect("standard variant resolves"),
            ScenarioVariant::standard()
        );
        assert_eq!(
            ScenarioVariant::resolve(VARIANT_DELUGE_ID).expect("deluge variant resolves"),
            ScenarioVariant::deluge()
        );
        assert!(ScenarioVariant::resolve("other").is_err());
    }
}
