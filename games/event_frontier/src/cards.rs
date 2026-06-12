use crate::ids::{FactionId, STANDARD_CARD_COUNT};
use crate::variants::{parse_flat_toml, parse_string_list, reject_unknown_keys, required_string};

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

fn parse_card_list(value: &str) -> Result<Vec<CardId>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| CardId::parse(&part).ok_or_else(|| format!("unknown card `{part}`")))
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
}
