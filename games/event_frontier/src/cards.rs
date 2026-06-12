use std::collections::BTreeSet;

use crate::variants::{parse_flat_toml, parse_string_list, reject_unknown_keys, required_string};
use crate::{
    effects::{public_effect, EventFrontierEffect, EventFrontierEffectEnvelope},
    ids::{FactionId, SiteId, STANDARD_CARD_COUNT, STANDARD_SITE_COUNT},
    state::{ActiveEdict, EventFrontierState},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CardId {
    BorderSurvey,
    TollRoads,
    RiverMists,
    StorehouseFire,
    SurveyBan,
    HighMeadowFair,
    ReckoningOne,
    DepotGrants,
    LongSeason,
    TrailWashout,
    CharterAudit,
    FreeholderMoot,
    Requisition,
    ReckoningTwo,
    OldMillStrike,
    CrossingMarket,
    GranitePassSnows,
    CacheBoom,
    AgentsRecall,
    LastLight,
    ReckoningThree,
}

impl CardId {
    pub const ALL: [Self; 21] = [
        Self::BorderSurvey,
        Self::TollRoads,
        Self::RiverMists,
        Self::StorehouseFire,
        Self::SurveyBan,
        Self::HighMeadowFair,
        Self::ReckoningOne,
        Self::DepotGrants,
        Self::LongSeason,
        Self::TrailWashout,
        Self::CharterAudit,
        Self::FreeholderMoot,
        Self::Requisition,
        Self::ReckoningTwo,
        Self::OldMillStrike,
        Self::CrossingMarket,
        Self::GranitePassSnows,
        Self::CacheBoom,
        Self::AgentsRecall,
        Self::LastLight,
        Self::ReckoningThree,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BorderSurvey => "ef_border_survey",
            Self::TollRoads => "ef_toll_roads",
            Self::RiverMists => "ef_river_mists",
            Self::StorehouseFire => "ef_storehouse_fire",
            Self::SurveyBan => "ef_survey_ban",
            Self::HighMeadowFair => "ef_high_meadow_fair",
            Self::ReckoningOne => "ef_reckoning_one",
            Self::DepotGrants => "ef_depot_grants",
            Self::LongSeason => "ef_long_season",
            Self::TrailWashout => "ef_trail_washout",
            Self::CharterAudit => "ef_charter_audit",
            Self::FreeholderMoot => "ef_freeholder_moot",
            Self::Requisition => "ef_requisition",
            Self::ReckoningTwo => "ef_reckoning_two",
            Self::OldMillStrike => "ef_old_mill_strike",
            Self::CrossingMarket => "ef_crossing_market",
            Self::GranitePassSnows => "ef_granite_pass_snows",
            Self::CacheBoom => "ef_cache_boom",
            Self::AgentsRecall => "ef_agents_recall",
            Self::LastLight => "ef_last_light",
            Self::ReckoningThree => "ef_reckoning_three",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "ef_border_survey" => Some(Self::BorderSurvey),
            "ef_toll_roads" => Some(Self::TollRoads),
            "ef_river_mists" => Some(Self::RiverMists),
            "ef_storehouse_fire" => Some(Self::StorehouseFire),
            "ef_survey_ban" => Some(Self::SurveyBan),
            "ef_high_meadow_fair" => Some(Self::HighMeadowFair),
            "ef_reckoning_one" => Some(Self::ReckoningOne),
            "ef_depot_grants" => Some(Self::DepotGrants),
            "ef_long_season" => Some(Self::LongSeason),
            "ef_trail_washout" => Some(Self::TrailWashout),
            "ef_charter_audit" => Some(Self::CharterAudit),
            "ef_freeholder_moot" => Some(Self::FreeholderMoot),
            "ef_requisition" => Some(Self::Requisition),
            "ef_reckoning_two" => Some(Self::ReckoningTwo),
            "ef_old_mill_strike" => Some(Self::OldMillStrike),
            "ef_crossing_market" => Some(Self::CrossingMarket),
            "ef_granite_pass_snows" => Some(Self::GranitePassSnows),
            "ef_cache_boom" => Some(Self::CacheBoom),
            "ef_agents_recall" => Some(Self::AgentsRecall),
            "ef_last_light" => Some(Self::LastLight),
            "ef_reckoning_three" => Some(Self::ReckoningThree),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EdictKind {
    TollRoads,
    SurveyBan,
    LongSeason,
    Requisition,
}

impl EdictKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TollRoads => "toll_roads",
            Self::SurveyBan => "survey_ban",
            Self::LongSeason => "long_season",
            Self::Requisition => "requisition",
        }
    }

    pub const fn stable_order(self) -> u8 {
        match self {
            Self::TollRoads => 0,
            Self::SurveyBan => 1,
            Self::LongSeason => 2,
            Self::Requisition => 3,
        }
    }

    pub fn for_card(card: CardId) -> Option<Self> {
        match card {
            CardId::TollRoads => Some(Self::TollRoads),
            CardId::SurveyBan => Some(Self::SurveyBan),
            CardId::LongSeason => Some(Self::LongSeason),
            CardId::Requisition => Some(Self::Requisition),
            _ => None,
        }
    }
}

pub fn resolve_event_card(
    state: &mut EventFrontierState,
    card: CardId,
) -> Vec<EventFrontierEffectEnvelope> {
    let mut effects = Vec::new();
    match card {
        CardId::BorderSurvey => {
            place_agent(state, SiteId::Crossing, &mut effects);
            resolved(card, "placed a Charter agent at Crossing", &mut effects);
        }
        CardId::TollRoads | CardId::SurveyBan | CardId::LongSeason | CardId::Requisition => {
            activate_edict(state, card, &mut effects);
            resolved(card, "activated an edict", &mut effects);
        }
        CardId::RiverMists => {
            move_settler_if_possible(state, SiteId::Landing, SiteId::HighMeadow, &mut effects);
            resolved(card, "moved a settler through the mists", &mut effects);
        }
        CardId::StorehouseFire => {
            change_resource(
                state,
                FactionId::Charter,
                -1,
                "storehouse_fire",
                &mut effects,
            );
            resolved(card, "reduced Charter funds", &mut effects);
        }
        CardId::HighMeadowFair => {
            change_resource(
                state,
                FactionId::Freeholders,
                1,
                "high_meadow_fair",
                &mut effects,
            );
            rally_settler(state, SiteId::HighMeadow, &mut effects);
            resolved(card, "fair raised provisions and a settler", &mut effects);
        }
        CardId::ReckoningOne | CardId::ReckoningTwo | CardId::ReckoningThree => {
            resolved(
                card,
                "Reckoning resolution is handled by the Reckoning pipeline",
                &mut effects,
            );
        }
        CardId::DepotGrants => {
            change_resource(state, FactionId::Charter, 2, "depot_grants", &mut effects);
            resolved(card, "granted Charter funds", &mut effects);
        }
        CardId::TrailWashout => {
            move_settler_if_possible(state, SiteId::Crossing, SiteId::Landing, &mut effects);
            resolved(card, "washed a settler back toward Landing", &mut effects);
        }
        CardId::CharterAudit => {
            change_resource(state, FactionId::Charter, 1, "charter_audit", &mut effects);
            remove_cache_if_present(state, SiteId::Landing, &mut effects);
            resolved(card, "audited caches and raised funds", &mut effects);
        }
        CardId::FreeholderMoot => {
            change_resource(
                state,
                FactionId::Freeholders,
                2,
                "freeholder_moot",
                &mut effects,
            );
            resolved(card, "raised Freeholder provisions", &mut effects);
        }
        CardId::OldMillStrike => {
            remove_agent_if_present(state, SiteId::Charterhouse, &mut effects);
            resolved(card, "pulled back a Charter agent", &mut effects);
        }
        CardId::CrossingMarket => {
            change_resource(
                state,
                FactionId::Charter,
                1,
                "crossing_market",
                &mut effects,
            );
            change_resource(
                state,
                FactionId::Freeholders,
                1,
                "crossing_market",
                &mut effects,
            );
            resolved(card, "market raised both resources", &mut effects);
        }
        CardId::GranitePassSnows => {
            change_resource(
                state,
                FactionId::Charter,
                -1,
                "granite_pass_snows",
                &mut effects,
            );
            resolved(card, "snows reduced Charter funds", &mut effects);
        }
        CardId::CacheBoom => {
            lay_cache(state, SiteId::HighMeadow, &mut effects);
            resolved(card, "laid a cache at High Meadow", &mut effects);
        }
        CardId::AgentsRecall => {
            remove_agent_if_present(state, SiteId::Crossing, &mut effects);
            place_agent(state, SiteId::Charterhouse, &mut effects);
            resolved(card, "recalled agents toward Charterhouse", &mut effects);
        }
        CardId::LastLight => {
            change_resource(state, FactionId::Charter, 1, "last_light", &mut effects);
            change_resource(state, FactionId::Freeholders, 1, "last_light", &mut effects);
            resolved(card, "last light paid both factions", &mut effects);
        }
    }
    effects
}

pub fn expire_all_edicts(state: &mut EventFrontierState) -> Vec<EventFrontierEffectEnvelope> {
    let mut active = sorted_active_edicts(state);
    state.active_edicts.clear();
    active
        .drain(..)
        .map(|edict| {
            public_effect(EventFrontierEffect::EdictExpired {
                edict: edict.kind.as_str().to_owned(),
            })
        })
        .collect()
}

pub fn sorted_active_edicts(state: &EventFrontierState) -> Vec<ActiveEdict> {
    let mut edicts = state.active_edicts.clone();
    edicts.sort_by_key(|edict| (edict.kind.stable_order(), edict.activation_index));
    edicts
}

fn activate_edict(
    state: &mut EventFrontierState,
    card: CardId,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    let Some(kind) = EdictKind::for_card(card) else {
        return;
    };
    let activation_index = state.active_edicts.len() as u8;
    state.active_edicts.push(ActiveEdict {
        kind,
        card,
        activation_index,
        expires_at_reckoning: state.reckoning_count.saturating_add(1),
    });
    effects.push(public_effect(EventFrontierEffect::EdictActivated {
        card,
        edict: kind.as_str().to_owned(),
    }));
}

fn resolved(card: CardId, summary: &str, effects: &mut Vec<EventFrontierEffectEnvelope>) {
    effects.push(public_effect(EventFrontierEffect::EventResolved {
        card,
        summary: summary.to_owned(),
    }));
}

fn change_resource(
    state: &mut EventFrontierState,
    faction: FactionId,
    delta: i8,
    reason: &str,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    let cap = state.variant.resource_cap;
    let (previous, new) = match faction {
        FactionId::Charter => {
            let previous = state.resources.funds;
            state.resources.funds = apply_delta(previous, delta, cap);
            (previous, state.resources.funds)
        }
        FactionId::Freeholders => {
            let previous = state.resources.provisions;
            state.resources.provisions = apply_delta(previous, delta, cap);
            (previous, state.resources.provisions)
        }
    };
    effects.push(public_effect(EventFrontierEffect::ResourcesChanged {
        faction,
        previous,
        new,
        reason: reason.to_owned(),
    }));
}

fn apply_delta(value: u8, delta: i8, cap: u8) -> u8 {
    if delta.is_negative() {
        value.saturating_sub(delta.unsigned_abs())
    } else {
        value.saturating_add(delta as u8).min(cap)
    }
}

fn place_agent(
    state: &mut EventFrontierState,
    site: SiteId,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    if let Some(site_state) = state.site_mut(site) {
        let previous = site_state.agents;
        site_state.agents = site_state.agents.saturating_add(1).min(3);
        if site_state.agents != previous {
            effects.push(public_effect(EventFrontierEffect::AgentPlaced {
                site,
                new_count: site_state.agents,
            }));
        }
    }
}

fn remove_agent_if_present(
    state: &mut EventFrontierState,
    site: SiteId,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    if let Some(site_state) = state.site_mut(site) {
        if site_state.agents > 0 {
            site_state.agents -= 1;
            effects.push(public_effect(EventFrontierEffect::AgentRemoved {
                site,
                new_count: site_state.agents,
            }));
        }
    }
}

fn rally_settler(
    state: &mut EventFrontierState,
    site: SiteId,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    if let Some(site_state) = state.site_mut(site) {
        let previous = site_state.settlers;
        site_state.settlers = site_state.settlers.saturating_add(1).min(3);
        if site_state.settlers != previous {
            effects.push(public_effect(EventFrontierEffect::SettlerRallied {
                site,
                new_count: site_state.settlers,
            }));
        }
    }
}

fn move_settler_if_possible(
    state: &mut EventFrontierState,
    from: SiteId,
    to: SiteId,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    let can_move = state.site(from).is_some_and(|site| site.settlers > 0)
        && state.site(to).is_some_and(|site| site.settlers < 3);
    if !can_move {
        return;
    }
    {
        let from_site = state.site_mut(from).expect("checked source site");
        from_site.settlers -= 1;
    }
    let from_count = state.site(from).map(|site| site.settlers).unwrap_or(0);
    let to_count = {
        let to_site = state.site_mut(to).expect("checked target site");
        to_site.settlers += 1;
        to_site.settlers
    };
    effects.push(public_effect(EventFrontierEffect::SettlerMoved {
        from,
        to,
        from_count,
        to_count,
    }));
}

fn lay_cache(
    state: &mut EventFrontierState,
    site: SiteId,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    if let Some(site_state) = state.site_mut(site) {
        let previous = site_state.cache_count;
        site_state.cache_count = site_state.cache_count.saturating_add(1).min(2);
        if site_state.cache_count != previous {
            effects.push(public_effect(EventFrontierEffect::CacheLaid {
                site,
                new_count: site_state.cache_count,
            }));
        }
    }
}

fn remove_cache_if_present(
    state: &mut EventFrontierState,
    site: SiteId,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    if let Some(site_state) = state.site_mut(site) {
        if site_state.cache_count > 0 {
            site_state.cache_count -= 1;
            effects.push(public_effect(EventFrontierEffect::CacheRemoved {
                site,
                new_count: site_state.cache_count,
            }));
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardData {
    pub id: CardId,
    pub label: String,
    pub epoch_pool: u8,
    pub first_eligible: FactionId,
    pub ops_value: u8,
    pub edict: bool,
    pub ui_family: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardCatalog {
    pub cards: Vec<CardData>,
}

impl CardCatalog {
    pub fn parse(input: &str) -> Result<Self, String> {
        let values = parse_flat_toml(input)?;
        reject_unknown_keys(
            &values,
            &[
                "card_ids",
                "labels",
                "epoch_pools",
                "first_eligible",
                "ops_values",
                "edict_flags",
                "ui_families",
            ],
        )?;

        let ids = parse_card_list(&required_string(&values, "card_ids")?)?;
        let labels = parse_string_list(&required_string(&values, "labels")?);
        let epoch_pools = parse_u8_list(&required_string(&values, "epoch_pools")?)?;
        let first_eligible = parse_faction_list(&required_string(&values, "first_eligible")?)?;
        let ops_values = parse_u8_list(&required_string(&values, "ops_values")?)?;
        let edict_flags = parse_bool_list(&required_string(&values, "edict_flags")?)?;
        let ui_families = parse_string_list(&required_string(&values, "ui_families")?);

        let len = ids.len();
        if len != STANDARD_CARD_COUNT as usize {
            return Err(format!(
                "card inventory must contain {STANDARD_CARD_COUNT} cards"
            ));
        }
        for (field, field_len) in [
            ("labels", labels.len()),
            ("epoch_pools", epoch_pools.len()),
            ("first_eligible", first_eligible.len()),
            ("ops_values", ops_values.len()),
            ("edict_flags", edict_flags.len()),
            ("ui_families", ui_families.len()),
        ] {
            if field_len != len {
                return Err(format!("{field} must contain {len} entries"));
            }
        }

        let cards = ids
            .into_iter()
            .enumerate()
            .map(|(index, id)| CardData {
                id,
                label: labels[index].clone(),
                epoch_pool: epoch_pools[index],
                first_eligible: first_eligible[index],
                ops_value: ops_values[index],
                edict: edict_flags[index],
                ui_family: ui_families[index].clone(),
            })
            .collect();

        Ok(Self { cards })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardPresentation {
    pub id: CardId,
    pub label: String,
    pub summary: String,
    pub details: Option<String>,
    pub family: String,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardPresentationCatalog {
    pub cards: Vec<CardPresentation>,
}

impl CardPresentationCatalog {
    pub fn parse(input: &str) -> Result<Self, String> {
        let values = parse_flat_toml(input)?;
        reject_unknown_keys(
            &values,
            &[
                "card_ids",
                "labels",
                "summaries",
                "details",
                "families",
                "accessibility_labels",
            ],
        )?;

        let ids = parse_card_list(&required_string(&values, "card_ids")?)?;
        let labels = parse_non_empty_string_list(&required_string(&values, "labels")?, "labels")?;
        let summaries =
            parse_non_empty_string_list(&required_string(&values, "summaries")?, "summaries")?;
        let details = values
            .get("details")
            .map(|value| parse_optional_string_list(value, "details"))
            .transpose()?;
        let families =
            parse_non_empty_string_list(&required_string(&values, "families")?, "families")?;
        let accessibility_labels = parse_non_empty_string_list(
            &required_string(&values, "accessibility_labels")?,
            "accessibility_labels",
        )?;

        validate_complete_unique_ids(&ids)?;
        let len = ids.len();
        for (field, field_len) in [
            ("labels", labels.len()),
            ("summaries", summaries.len()),
            ("details", details.as_ref().map_or(len, Vec::len)),
            ("families", families.len()),
            ("accessibility_labels", accessibility_labels.len()),
        ] {
            if field_len != len {
                return Err(format!("{field} must contain {len} entries"));
            }
        }

        let cards = ids
            .into_iter()
            .enumerate()
            .map(|(index, id)| CardPresentation {
                id,
                label: labels[index].clone(),
                summary: summaries[index].clone(),
                details: details.as_ref().and_then(|entries| entries[index].clone()),
                family: families[index].clone(),
                accessibility_label: accessibility_labels[index].clone(),
            })
            .collect();

        Ok(Self { cards })
    }

    pub fn get(&self, id: CardId) -> Option<&CardPresentation> {
        self.cards.iter().find(|card| card.id == id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SitePresentation {
    pub id: SiteId,
    pub label: String,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SitePresentationCatalog {
    pub sites: Vec<SitePresentation>,
}

impl SitePresentationCatalog {
    pub fn parse(input: &str) -> Result<Self, String> {
        let values = parse_flat_toml(input)?;
        reject_unknown_keys(&values, &["site_ids", "labels", "accessibility_labels"])?;

        let ids = parse_site_list(&required_string(&values, "site_ids")?)?;
        let labels = parse_non_empty_string_list(&required_string(&values, "labels")?, "labels")?;
        let accessibility_labels = parse_non_empty_string_list(
            &required_string(&values, "accessibility_labels")?,
            "accessibility_labels",
        )?;

        validate_complete_unique_site_ids(&ids)?;
        let len = ids.len();
        for (field, field_len) in [
            ("labels", labels.len()),
            ("accessibility_labels", accessibility_labels.len()),
        ] {
            if field_len != len {
                return Err(format!("{field} must contain {len} entries"));
            }
        }

        let sites = ids
            .into_iter()
            .enumerate()
            .map(|(index, id)| SitePresentation {
                id,
                label: labels[index].clone(),
                accessibility_label: accessibility_labels[index].clone(),
            })
            .collect();

        Ok(Self { sites })
    }

    pub fn get(&self, id: SiteId) -> Option<&SitePresentation> {
        self.sites.iter().find(|site| site.id == id)
    }
}

fn parse_card_list(value: &str) -> Result<Vec<CardId>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| CardId::parse(&part).ok_or_else(|| format!("unknown card `{part}`")))
        .collect()
}

fn parse_site_list(value: &str) -> Result<Vec<SiteId>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| SiteId::parse(&part).ok_or_else(|| format!("unknown site `{part}`")))
        .collect()
}

fn parse_faction_list(value: &str) -> Result<Vec<FactionId>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| FactionId::parse(&part).ok_or_else(|| format!("unknown faction `{part}`")))
        .collect()
}

fn parse_u8_list(value: &str) -> Result<Vec<u8>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| {
            part.parse::<u8>()
                .map_err(|_| format!("invalid integer `{part}`"))
        })
        .collect()
}

fn parse_bool_list(value: &str) -> Result<Vec<bool>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| match part.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(format!("invalid bool `{part}`")),
        })
        .collect()
}

fn parse_non_empty_string_list(value: &str, field: &str) -> Result<Vec<String>, String> {
    let entries = parse_string_list(value);
    if let Some(empty_index) = entries.iter().position(|entry| entry.trim().is_empty()) {
        return Err(format!("{field} entry {empty_index} must not be empty"));
    }
    Ok(entries)
}

fn parse_optional_string_list(value: &str, field: &str) -> Result<Vec<Option<String>>, String> {
    Ok(parse_non_empty_string_list(value, field)?
        .into_iter()
        .map(|entry| {
            let trimmed = entry.trim();
            if trimmed == "-" {
                None
            } else {
                Some(trimmed.to_owned())
            }
        })
        .collect())
}

fn validate_complete_unique_ids(ids: &[CardId]) -> Result<(), String> {
    if ids.len() != STANDARD_CARD_COUNT as usize {
        return Err(format!(
            "card_ids must contain {STANDARD_CARD_COUNT} presentation rows"
        ));
    }

    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(*id) {
            return Err(format!("duplicate presentation row for `{}`", id.as_str()));
        }
    }
    for expected in CardId::ALL {
        if !seen.contains(&expected) {
            return Err(format!(
                "missing presentation row for `{}`",
                expected.as_str()
            ));
        }
    }
    Ok(())
}

fn validate_complete_unique_site_ids(ids: &[SiteId]) -> Result<(), String> {
    if ids.len() != STANDARD_SITE_COUNT as usize {
        return Err(format!(
            "site_ids must contain {STANDARD_SITE_COUNT} presentation rows"
        ));
    }

    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(*id) {
            return Err(format!("duplicate presentation row for `{}`", id.as_str()));
        }
    }
    for expected in SiteId::ALL {
        if !seen.contains(&expected) {
            return Err(format!(
                "missing presentation row for `{}`",
                expected.as_str()
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_ids_are_closed_and_stable() {
        assert_eq!(CardId::BorderSurvey.as_str(), "ef_border_survey");
        assert_eq!(
            CardId::parse("ef_reckoning_three"),
            Some(CardId::ReckoningThree)
        );
        assert_eq!(CardId::ALL.len(), STANDARD_CARD_COUNT as usize);
    }

    #[test]
    fn card_data_rejects_behavior_and_unknown_keys() {
        assert!(
            CardCatalog::parse("card_ids = \"ef_border_survey\"\ntrigger = \"bad\"\n").is_err()
        );
        assert!(CardCatalog::parse("card_ids = \"ef_border_survey\"\nextra = \"bad\"\n").is_err());
    }

    #[test]
    fn static_cards_parse() {
        let cards = CardCatalog::parse(include_str!("../data/cards.toml")).unwrap();
        assert_eq!(cards.cards.len(), STANDARD_CARD_COUNT as usize);
        assert_eq!(cards.cards.iter().filter(|card| card.edict).count(), 4);
    }

    #[test]
    fn card_presentation_parse_is_complete_and_fail_closed() {
        let presentation =
            CardPresentationCatalog::parse(include_str!("../data/cards_presentation.toml"))
                .unwrap();
        assert_eq!(presentation.cards.len(), STANDARD_CARD_COUNT as usize);
        assert_eq!(
            presentation.get(CardId::HighMeadowFair).unwrap().label,
            "High Meadow Fair"
        );
        assert!(presentation
            .get(CardId::SurveyBan)
            .unwrap()
            .details
            .as_deref()
            .unwrap()
            .contains("contested site"));
        assert!(CardPresentationCatalog::parse(
            "card_ids = \"ef_border_survey\"\nlabels = \"Border Survey\"\nsummaries = \"x\"\nfamilies = \"ordinary\"\naccessibility_labels = \"x\"\ntrigger = \"bad\"\n"
        )
        .is_err());
        assert!(CardPresentationCatalog::parse(
            "card_ids = \"ef_border_survey\"\nlabels = \"Border Survey\"\nsummaries = \"x\"\ndetails = \"x\"\nfamilies = \"ordinary\"\naccessibility_labels = \"x\"\nselector = \"bad\"\n"
        )
        .is_err());
        assert!(CardPresentationCatalog::parse(
            "card_ids = \"ef_border_survey\"\nlabels = \"Border Survey\"\nsummaries = \"x\"\ndetails = \"x,y\"\nfamilies = \"ordinary\"\naccessibility_labels = \"x\"\n"
        )
        .is_err());
        assert!(CardPresentationCatalog::parse(
            "card_ids = \"ef_border_survey,ef_border_survey\"\nlabels = \"A,B\"\nsummaries = \"A,B\"\nfamilies = \"ordinary,ordinary\"\naccessibility_labels = \"A,B\"\n"
        )
        .is_err());
        assert!(CardPresentationCatalog::parse(
            "card_ids = \"ef_border_survey\"\nlabels = \"\"\nsummaries = \"A\"\nfamilies = \"ordinary\"\naccessibility_labels = \"A\"\n"
        )
        .is_err());
    }

    #[test]
    fn site_presentation_parse_is_complete_and_fail_closed() {
        let presentation =
            SitePresentationCatalog::parse(include_str!("../data/sites_presentation.toml"))
                .unwrap();
        assert_eq!(presentation.sites.len(), STANDARD_SITE_COUNT as usize);
        assert_eq!(
            presentation.get(SiteId::GranitePass).unwrap().label,
            "Granite Pass"
        );
        assert!(SitePresentationCatalog::parse(
            "site_ids = \"site_charterhouse\"\nlabels = \"Charterhouse\"\naccessibility_labels = \"Charterhouse site\"\ntrigger = \"bad\"\n"
        )
        .is_err());
        assert!(SitePresentationCatalog::parse(
            "site_ids = \"site_charterhouse,site_charterhouse,site_crossing,site_landing,site_granite_pass,site_high_meadow\"\nlabels = \"A,B,C,D,E,F\"\naccessibility_labels = \"A,B,C,D,E,F\"\n"
        )
        .is_err());
        assert!(SitePresentationCatalog::parse(
            "site_ids = \"site_charterhouse\"\nlabels = \"\"\naccessibility_labels = \"Charterhouse site\"\n"
        )
        .is_err());
    }
}
