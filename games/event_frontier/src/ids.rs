pub const GAME_ID: &str = "event_frontier";
pub const VARIANT_STANDARD_ID: &str = "event_frontier_standard";
pub const VARIANT_HARD_WINTER_ID: &str = "event_frontier_hard_winter";
pub const VARIANT_LAND_RUSH_ID: &str = "event_frontier_land_rush";
pub const RULES_VERSION_LABEL: &str = "event-frontier-rules-v1";

pub const STANDARD_SEAT_COUNT: u8 = 2;
pub const STANDARD_SITE_COUNT: u8 = 6;
pub const STANDARD_CARD_COUNT: u8 = 21;
pub const STANDARD_EPOCH_COUNT: u8 = 3;
pub const STANDARD_RESOURCE_CAP: u8 = 9;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FactionId {
    Charter,
    Freeholders,
}

impl FactionId {
    pub const ALL: [Self; 2] = [Self::Charter, Self::Freeholders];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Charter => "faction_charter",
            Self::Freeholders => "faction_freeholders",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Charter => "Charter",
            Self::Freeholders => "Freeholders",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "faction_charter" => Some(Self::Charter),
            "faction_freeholders" => Some(Self::Freeholders),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SiteId {
    Charterhouse,
    Landing,
    Crossing,
    GranitePass,
    HighMeadow,
    OldMill,
}

impl SiteId {
    pub const ALL: [Self; 6] = [
        Self::Charterhouse,
        Self::Landing,
        Self::Crossing,
        Self::GranitePass,
        Self::HighMeadow,
        Self::OldMill,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Charterhouse => "site_charterhouse",
            Self::Landing => "site_landing",
            Self::Crossing => "site_crossing",
            Self::GranitePass => "site_granite_pass",
            Self::HighMeadow => "site_high_meadow",
            Self::OldMill => "site_old_mill",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Charterhouse => "Charterhouse",
            Self::Landing => "Landing",
            Self::Crossing => "Crossing",
            Self::GranitePass => "Granite Pass",
            Self::HighMeadow => "High Meadow",
            Self::OldMill => "Old Mill",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "site_charterhouse" => Some(Self::Charterhouse),
            "site_landing" => Some(Self::Landing),
            "site_crossing" => Some(Self::Crossing),
            "site_granite_pass" => Some(Self::GranitePass),
            "site_high_meadow" => Some(Self::HighMeadow),
            "site_old_mill" => Some(Self::OldMill),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_parse_and_format_stably() {
        assert_eq!(FactionId::Charter.as_str(), "faction_charter");
        assert_eq!(
            FactionId::parse("faction_freeholders"),
            Some(FactionId::Freeholders)
        );
        assert_eq!(SiteId::GranitePass.label(), "Granite Pass");
        assert_eq!(SiteId::parse("site_old_mill"), Some(SiteId::OldMill));
    }
}
