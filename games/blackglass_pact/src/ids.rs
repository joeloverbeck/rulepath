use engine_core::SeatId;
use std::sync::LazyLock;

pub const GAME_ID: &str = "blackglass_pact";
pub const VARIANT_ID: &str = "blackglass_pact_standard";
pub const RULES_VERSION_LABEL: &str = "blackglass-pact-rules-v1";
pub const DATA_VERSION_LABEL: &str = "blackglass-pact-data-v1";
pub const STANDARD_SEAT_COUNT: u8 = 4;
pub const STANDARD_MIN_SEATS: u8 = STANDARD_SEAT_COUNT;
pub const STANDARD_DEFAULT_SEATS: u8 = STANDARD_SEAT_COUNT;
pub const STANDARD_MAX_SEATS: u8 = STANDARD_SEAT_COUNT;
pub const STANDARD_SUIT_COUNT: u8 = 4;
pub const STANDARD_RANK_COUNT: u8 = 13;
pub const STANDARD_CARD_COUNT: u8 = STANDARD_SUIT_COUNT * STANDARD_RANK_COUNT;
pub const STANDARD_HAND_SIZE: u8 = 13;
pub const STANDARD_TRICKS_PER_HAND: u8 = 13;

static CANONICAL_BLACKGLASS_SEAT_IDS: LazyLock<[SeatId; 4]> = LazyLock::new(|| {
    [
        SeatId::from_zero_based_index(0),
        SeatId::from_zero_based_index(1),
        SeatId::from_zero_based_index(2),
        SeatId::from_zero_based_index(3),
    ]
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum BlackglassSeat {
    North,
    East,
    South,
    West,
}

impl BlackglassSeat {
    pub const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::North),
            1 => Some(Self::East),
            2 => Some(Self::South),
            3 => Some(Self::West),
            _ => None,
        }
    }

    pub const fn index(self) -> usize {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        }
    }

    pub fn as_str(self) -> &'static str {
        &CANONICAL_BLACKGLASS_SEAT_IDS[self.index()].0
    }

    pub const fn public_label(self) -> &'static str {
        match self {
            Self::North => "North",
            Self::East => "East",
            Self::South => "South",
            Self::West => "West",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        let raw_index = SeatId::parse_canonical(value)
            .ok()?
            .canonical_zero_based_index()
            .ok()? as usize;
        Self::from_index(raw_index)
    }

    pub const fn next_clockwise(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TeamId {
    NorthSouth,
    EastWest,
}

impl TeamId {
    pub const ALL: [Self; 2] = [Self::NorthSouth, Self::EastWest];

    pub const fn index(self) -> usize {
        match self {
            Self::NorthSouth => 0,
            Self::EastWest => 1,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NorthSouth => "team_0",
            Self::EastWest => "team_1",
        }
    }

    pub const fn public_label(self) -> &'static str {
        match self {
            Self::NorthSouth => "North-South",
            Self::EastWest => "East-West",
        }
    }
}

pub fn seat_id_for_index(index: usize) -> SeatId {
    SeatId::from_zero_based_index(index.try_into().expect("seat index must fit u32"))
}

pub fn canonical_seat_ids() -> [SeatId; 4] {
    CANONICAL_BLACKGLASS_SEAT_IDS.clone()
}
