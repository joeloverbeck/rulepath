pub const GAME_ID: &str = "frontier_control";
pub const VARIANT_STANDARD_ID: &str = "frontier_control_standard";
pub const VARIANT_HIGHLANDS_ID: &str = "frontier_control_highlands";
pub const RULES_VERSION_LABEL: &str = "frontier-control-rules-v1";

pub const STANDARD_SEAT_COUNT: u8 = 2;
pub const STANDARD_SITE_COUNT: u8 = 7;
pub const STANDARD_ACTION_BUDGET: u8 = 2;
pub const STANDARD_ROUND_COUNT: u8 = 8;
pub const UNIT_CAP_PER_SITE: u8 = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SiteId {
    Gatehouse,
    SignalHill,
    BaseCamp,
    Ford,
    Quarry,
    Timberline,
    Goldfield,
}

impl SiteId {
    pub const ALL: [Self; 7] = [
        Self::Gatehouse,
        Self::SignalHill,
        Self::BaseCamp,
        Self::Ford,
        Self::Quarry,
        Self::Timberline,
        Self::Goldfield,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gatehouse => "site_gatehouse",
            Self::SignalHill => "site_signal_hill",
            Self::BaseCamp => "site_base_camp",
            Self::Ford => "site_ford",
            Self::Quarry => "site_quarry",
            Self::Timberline => "site_timberline",
            Self::Goldfield => "site_goldfield",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Gatehouse => "Gatehouse",
            Self::SignalHill => "Signal Hill",
            Self::BaseCamp => "Base Camp",
            Self::Ford => "Ford",
            Self::Quarry => "Quarry",
            Self::Timberline => "Timberline",
            Self::Goldfield => "Goldfield",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "site_gatehouse" => Some(Self::Gatehouse),
            "site_signal_hill" => Some(Self::SignalHill),
            "site_base_camp" => Some(Self::BaseCamp),
            "site_ford" => Some(Self::Ford),
            "site_quarry" => Some(Self::Quarry),
            "site_timberline" => Some(Self::Timberline),
            "site_goldfield" => Some(Self::Goldfield),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FactionId {
    Garrison,
    Prospectors,
}

impl FactionId {
    pub const ALL: [Self; 2] = [Self::Garrison, Self::Prospectors];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Garrison => "faction_garrison",
            Self::Prospectors => "faction_prospectors",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Garrison => "Garrison",
            Self::Prospectors => "Prospectors",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "faction_garrison" => Some(Self::Garrison),
            "faction_prospectors" => Some(Self::Prospectors),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_parse_and_format_stably() {
        assert_eq!(SiteId::parse("site_quarry"), Some(SiteId::Quarry));
        assert_eq!(SiteId::SignalHill.as_str(), "site_signal_hill");
        assert_eq!(FactionId::Prospectors.label(), "Prospectors");
        assert_eq!(
            FactionId::parse("faction_garrison"),
            Some(FactionId::Garrison)
        );
    }
}
