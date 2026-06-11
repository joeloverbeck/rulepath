pub const GAME_ID: &str = "flood_watch";
pub const VARIANT_STANDARD_ID: &str = "flood_watch_standard";
pub const VARIANT_DELUGE_ID: &str = "flood_watch_deluge";
pub const RULES_VERSION_LABEL: &str = "flood-watch-rules-v1";

pub const STANDARD_SEAT_COUNT: u8 = 2;
pub const STANDARD_DISTRICT_COUNT: u8 = 5;
pub const STANDARD_MAX_FLOOD_LEVEL: u8 = 3;
pub const STANDARD_LEVEE_CAP: u8 = 2;
pub const STANDARD_ACTION_BUDGET: u8 = 3;
pub const STANDARD_DRAWS_PER_PHASE: u8 = 2;
pub const STANDARD_DOWNPOURS_PER_DISTRICT: u8 = 3;
pub const STANDARD_SURGES_PER_DISTRICT: u8 = 1;
pub const STANDARD_REPRIEVE_COUNT: u8 = 4;
pub const STANDARD_DECK_SIZE: u8 = (STANDARD_DOWNPOURS_PER_DISTRICT + STANDARD_SURGES_PER_DISTRICT)
    * STANDARD_DISTRICT_COUNT
    + STANDARD_REPRIEVE_COUNT;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum DistrictId {
    Riverside,
    OldDocks,
    Market,
    Terraces,
    Gardens,
}

impl DistrictId {
    pub const ALL: [Self; 5] = [
        Self::Riverside,
        Self::OldDocks,
        Self::Market,
        Self::Terraces,
        Self::Gardens,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Riverside => "district_riverside",
            Self::OldDocks => "district_old_docks",
            Self::Market => "district_market",
            Self::Terraces => "district_terraces",
            Self::Gardens => "district_gardens",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Riverside => "Riverside",
            Self::OldDocks => "Old Docks",
            Self::Market => "Market",
            Self::Terraces => "Terraces",
            Self::Gardens => "Gardens",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "district_riverside" => Some(Self::Riverside),
            "district_old_docks" => Some(Self::OldDocks),
            "district_market" => Some(Self::Market),
            "district_terraces" => Some(Self::Terraces),
            "district_gardens" => Some(Self::Gardens),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FloodWatchRole {
    Pumpwright,
    LeveeWarden,
}

impl FloodWatchRole {
    pub const ALL: [Self; 2] = [Self::Pumpwright, Self::LeveeWarden];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pumpwright => "pumpwright",
            Self::LeveeWarden => "levee_warden",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Pumpwright => "Pumpwright",
            Self::LeveeWarden => "Levee Warden",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "pumpwright" => Some(Self::Pumpwright),
            "levee_warden" => Some(Self::LeveeWarden),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum EventKind {
    Downpour { district: DistrictId },
    StormSurge { district: DistrictId },
    Reprieve,
}

impl EventKind {
    pub fn id(self) -> String {
        match self {
            Self::Downpour { district } => format!("downpour/{}", district.as_str()),
            Self::StormSurge { district } => format!("storm_surge/{}", district.as_str()),
            Self::Reprieve => "reprieve".to_owned(),
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        if value == "reprieve" {
            return Some(Self::Reprieve);
        }
        if let Some(district) = value.strip_prefix("downpour/") {
            return DistrictId::parse(district).map(|district| Self::Downpour { district });
        }
        if let Some(district) = value.strip_prefix("storm_surge/") {
            return DistrictId::parse(district).map(|district| Self::StormSurge { district });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_parse_and_format_stably() {
        assert_eq!(
            DistrictId::parse("district_old_docks"),
            Some(DistrictId::OldDocks)
        );
        assert_eq!(FloodWatchRole::LeveeWarden.as_str(), "levee_warden");
        assert_eq!(
            EventKind::parse("storm_surge/district_market"),
            Some(EventKind::StormSurge {
                district: DistrictId::Market
            })
        );
        assert_eq!(
            EventKind::Downpour {
                district: DistrictId::Gardens
            }
            .id(),
            "downpour/district_gardens"
        );
    }
}
