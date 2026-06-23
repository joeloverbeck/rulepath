use engine_core::SeatId;

pub const GAME_ID: &str = "poker_lite";
pub const VARIANT_ID: &str = "poker_lite_standard";
pub const RULES_VERSION_LABEL: &str = "poker-lite-rules-v1";
pub const STANDARD_SEAT_COUNT: u8 = 2;
pub const STANDARD_RANK_COUNT: u8 = 3;
pub const STANDARD_COPY_COUNT: u8 = 2;
pub const STANDARD_CARD_COUNT: u8 = STANDARD_RANK_COUNT * STANDARD_COPY_COUNT;
pub const STANDARD_ROUND_COUNT: u8 = 2;
pub const STANDARD_ROUND_UNITS: [u8; 2] = [1, 2];
pub const STANDARD_MAX_CONTRIBUTION: u8 = 7;

pub const ACTION_HOLD: &str = "hold";
pub const ACTION_PRESS: &str = "press";
pub const ACTION_LIFT: &str = "lift";
pub const ACTION_MATCH: &str = "match";
pub const ACTION_YIELD: &str = "yield";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum PokerLiteSeat {
    Seat0,
    Seat1,
}

impl PokerLiteSeat {
    pub const ALL: [Self; 2] = [Self::Seat0, Self::Seat1];

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Seat0),
            1 => Some(Self::Seat1),
            _ => None,
        }
    }

    pub const fn index(self) -> usize {
        match self {
            Self::Seat0 => 0,
            Self::Seat1 => 1,
        }
    }

    pub const fn other(self) -> Self {
        match self {
            Self::Seat0 => Self::Seat1,
            Self::Seat1 => Self::Seat0,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Seat0 => "seat_0",
            Self::Seat1 => "seat_1",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        let seat_id = SeatId::parse_canonical(value).ok()?;
        let index = usize::try_from(seat_id.canonical_zero_based_index().ok()?).ok()?;
        Self::from_index(index)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CrestRank {
    Low,
    Middle,
    High,
}

impl CrestRank {
    pub const ALL: [Self; 3] = [Self::Low, Self::Middle, Self::High];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Middle => "middle",
            Self::High => "high",
        }
    }

    pub const fn value(self) -> u8 {
        match self {
            Self::Low => 1,
            Self::Middle => 2,
            Self::High => 3,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Low => "Sprout",
            Self::Middle => "Current",
            Self::High => "Crown",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "low" => Some(Self::Low),
            "middle" => Some(Self::Middle),
            "high" => Some(Self::High),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CrestRankCopy {
    Dawn,
    Dusk,
}

impl CrestRankCopy {
    pub const ALL: [Self; 2] = [Self::Dawn, Self::Dusk];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dawn => "dawn",
            Self::Dusk => "dusk",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Dawn => "Dawn",
            Self::Dusk => "Dusk",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "dawn" => Some(Self::Dawn),
            "dusk" => Some(Self::Dusk),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CrestCardId {
    LowDawn,
    LowDusk,
    MiddleDawn,
    MiddleDusk,
    HighDawn,
    HighDusk,
}

impl CrestCardId {
    pub const ALL: [Self; 6] = [
        Self::LowDawn,
        Self::LowDusk,
        Self::MiddleDawn,
        Self::MiddleDusk,
        Self::HighDawn,
        Self::HighDusk,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LowDawn => "low_dawn",
            Self::LowDusk => "low_dusk",
            Self::MiddleDawn => "middle_dawn",
            Self::MiddleDusk => "middle_dusk",
            Self::HighDawn => "high_dawn",
            Self::HighDusk => "high_dusk",
        }
    }

    pub const fn rank(self) -> CrestRank {
        match self {
            Self::LowDawn | Self::LowDusk => CrestRank::Low,
            Self::MiddleDawn | Self::MiddleDusk => CrestRank::Middle,
            Self::HighDawn | Self::HighDusk => CrestRank::High,
        }
    }

    pub const fn rank_copy(self) -> CrestRankCopy {
        match self {
            Self::LowDawn | Self::MiddleDawn | Self::HighDawn => CrestRankCopy::Dawn,
            Self::LowDusk | Self::MiddleDusk | Self::HighDusk => CrestRankCopy::Dusk,
        }
    }

    pub fn label(self) -> String {
        format!("{} {}", self.rank().label(), self.rank_copy().label())
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "low_dawn" => Some(Self::LowDawn),
            "low_dusk" => Some(Self::LowDusk),
            "middle_dawn" => Some(Self::MiddleDawn),
            "middle_dusk" => Some(Self::MiddleDusk),
            "high_dawn" => Some(Self::HighDawn),
            "high_dusk" => Some(Self::HighDusk),
            _ => None,
        }
    }
}

pub const fn canonical_deck() -> [CrestCardId; 6] {
    CrestCardId::ALL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seats_are_stable() {
        assert_eq!(PokerLiteSeat::from_index(0), Some(PokerLiteSeat::Seat0));
        assert_eq!(PokerLiteSeat::from_index(1), Some(PokerLiteSeat::Seat1));
        assert_eq!(PokerLiteSeat::from_index(2), None);
        assert_eq!(PokerLiteSeat::Seat0.index(), 0);
        assert_eq!(PokerLiteSeat::Seat1.other(), PokerLiteSeat::Seat0);
        assert_eq!(PokerLiteSeat::Seat1.as_str(), "seat_1");
        assert_eq!(PokerLiteSeat::parse("seat_0"), Some(PokerLiteSeat::Seat0));
        assert_eq!(PokerLiteSeat::parse("seat_1"), Some(PokerLiteSeat::Seat1));
    }

    #[test]
    fn seat_parser_rejects_non_canonical_and_out_of_range_ids() {
        for rejected in [
            "seat_2",
            "seat_01",
            "seat_001",
            "seat-0",
            "seat-a",
            "seat-b",
            "seat_a",
            "seat_one",
            "seat_１",
            "ѕeat_0",
            "Seat_0",
            "player_0",
            "dealer",
            "leader",
            "challenger",
            "responder",
        ] {
            assert_eq!(PokerLiteSeat::parse(rejected), None, "{rejected}");
        }
    }

    #[test]
    fn crest_rank_labels_and_values_are_stable() {
        assert_eq!(CrestRank::Low.as_str(), "low");
        assert_eq!(CrestRank::Middle.value(), 2);
        assert_eq!(CrestRank::High.label(), "Crown");
        assert_eq!(CrestRank::parse("middle"), Some(CrestRank::Middle));
        assert_eq!(CrestRank::parse("unknown"), None);
    }

    #[test]
    fn canonical_card_ids_match_rules_order() {
        let deck = canonical_deck();

        assert_eq!(deck.len(), STANDARD_CARD_COUNT as usize);
        assert_eq!(deck[0].as_str(), "low_dawn");
        assert_eq!(deck[5].as_str(), "high_dusk");
        assert_eq!(CrestCardId::MiddleDusk.rank(), CrestRank::Middle);
        assert_eq!(CrestCardId::MiddleDusk.rank_copy(), CrestRankCopy::Dusk);
        assert_eq!(CrestCardId::HighDawn.label(), "Crown Dawn");
        assert_eq!(
            CrestCardId::parse("middle_dawn"),
            Some(CrestCardId::MiddleDawn)
        );
        assert_eq!(CrestCardId::parse("bad"), None);
    }

    #[test]
    fn action_segments_are_neutral_and_stable() {
        assert_eq!(
            [
                ACTION_HOLD,
                ACTION_PRESS,
                ACTION_LIFT,
                ACTION_MATCH,
                ACTION_YIELD
            ],
            ["hold", "press", "lift", "match", "yield"]
        );
    }
}
