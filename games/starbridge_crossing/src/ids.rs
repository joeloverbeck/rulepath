use engine_core::SeatId;
use game_stdlib::SeatCount;
use std::{fmt, sync::LazyLock};

pub const GAME_ID: &str = "starbridge_crossing";
pub const VARIANT_ID: &str = "starbridge_crossing_classic_star_v1";
pub const RULES_VERSION_LABEL: &str = "starbridge-crossing-rules-v1";
pub const DATA_VERSION_LABEL: &str = "starbridge-crossing-data-v1";
pub const STANDARD_MIN_SEATS: u8 = 2;
pub const STANDARD_DEFAULT_SEATS: u8 = 2;
pub const STANDARD_MAX_SEATS: u8 = 6;
pub const SUPPORTED_SEAT_COUNTS: [u8; 4] = [2, 3, 4, 6];
pub const STANDARD_PEGS_PER_SEAT: u8 = 10;
pub const SPACE_COUNT: u16 = 121;
pub const MAX_SPACE_INDEX: u16 = SPACE_COUNT - 1;

static SUPPORTED_SEAT_COUNTS_USIZE: LazyLock<[usize; 4]> =
    LazyLock::new(|| SUPPORTED_SEAT_COUNTS.map(usize::from));

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StarSpaceId(u16);

impl StarSpaceId {
    pub const fn new(index: u16) -> Result<Self, StarSpaceIdError> {
        if index < SPACE_COUNT {
            Ok(Self(index))
        } else {
            Err(StarSpaceIdError::OutOfRange { index })
        }
    }

    pub const fn index(self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for StarSpaceId {
    type Error = StarSpaceIdError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<StarSpaceId> for u16 {
    fn from(value: StarSpaceId) -> Self {
        value.index()
    }
}

impl fmt::Display for StarSpaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "s{:03}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StarSpaceIdError {
    OutOfRange { index: u16 },
}

impl fmt::Display for StarSpaceIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange { index } => {
                write!(
                    f,
                    "star space index {index} is outside 0..={MAX_SPACE_INDEX}"
                )
            }
        }
    }
}

impl std::error::Error for StarSpaceIdError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StarPoint {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl StarPoint {
    pub const ALL: [Self; 6] = [
        Self::North,
        Self::NorthEast,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::NorthWest,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::NorthEast => "north_east",
            Self::SouthEast => "south_east",
            Self::South => "south",
            Self::SouthWest => "south_west",
            Self::NorthWest => "north_west",
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::NorthWest => Self::SouthEast,
        }
    }

    pub const fn clockwise_index(self) -> usize {
        match self {
            Self::North => 0,
            Self::NorthEast => 1,
            Self::SouthEast => 2,
            Self::South => 3,
            Self::SouthWest => 4,
            Self::NorthWest => 5,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StarZone {
    Home(StarPoint),
    Target(StarPoint),
    Neutral,
}

impl StarZone {
    pub const fn point(self) -> Option<StarPoint> {
        match self {
            Self::Home(point) | Self::Target(point) => Some(point),
            Self::Neutral => None,
        }
    }
}

pub fn supported_seat_count(seat_count: usize) -> bool {
    SeatCount::new(seat_count).is_ok() && SUPPORTED_SEAT_COUNTS_USIZE.contains(&seat_count)
}

pub fn active_points_for_seat_count(seat_count: usize) -> Option<&'static [StarPoint]> {
    match seat_count {
        2 => Some(&[StarPoint::North, StarPoint::South]),
        3 => Some(&[StarPoint::North, StarPoint::SouthEast, StarPoint::SouthWest]),
        4 => Some(&[
            StarPoint::North,
            StarPoint::NorthEast,
            StarPoint::South,
            StarPoint::SouthWest,
        ]),
        6 => Some(&StarPoint::ALL),
        _ => None,
    }
}

pub fn seat_id_for_index(index: usize) -> SeatId {
    SeatId::from_zero_based_index(index.try_into().expect("seat index must fit u32"))
}

pub fn canonical_seat_ids(seat_count: usize) -> Option<Vec<SeatId>> {
    supported_seat_count(seat_count).then(|| (0..seat_count).map(seat_id_for_index).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_space_ids_accept_exact_manifest_range() {
        assert_eq!(StarSpaceId::new(0).unwrap().index(), 0);
        assert_eq!(
            StarSpaceId::new(MAX_SPACE_INDEX).unwrap().index(),
            MAX_SPACE_INDEX
        );
        assert_eq!(
            StarSpaceId::new(SPACE_COUNT),
            Err(StarSpaceIdError::OutOfRange { index: SPACE_COUNT })
        );
        assert_eq!(StarSpaceId::new(7).unwrap().to_string(), "s007");
    }

    #[test]
    fn star_points_have_stable_clockwise_labels_and_opposites() {
        let labels: Vec<_> = StarPoint::ALL
            .iter()
            .copied()
            .map(StarPoint::label)
            .collect();
        assert_eq!(
            labels,
            vec![
                "north",
                "north_east",
                "south_east",
                "south",
                "south_west",
                "north_west",
            ]
        );

        for (index, point) in StarPoint::ALL.iter().copied().enumerate() {
            assert_eq!(point.clockwise_index(), index);
            assert_eq!(point.opposite().opposite(), point);
        }
        assert_eq!(StarPoint::North.opposite(), StarPoint::South);
        assert_eq!(StarPoint::NorthEast.opposite(), StarPoint::SouthWest);
        assert_eq!(StarPoint::SouthEast.opposite(), StarPoint::NorthWest);
    }

    #[test]
    fn star_zones_expose_point_only_for_home_or_target() {
        assert_eq!(
            StarZone::Home(StarPoint::North).point(),
            Some(StarPoint::North)
        );
        assert_eq!(
            StarZone::Target(StarPoint::South).point(),
            Some(StarPoint::South)
        );
        assert_eq!(StarZone::Neutral.point(), None);
    }

    #[test]
    fn supported_seat_counts_are_discontinuous_and_points_are_stable() {
        for rejected in [0, 1, 5, 7, 8] {
            assert!(!supported_seat_count(rejected));
            assert_eq!(active_points_for_seat_count(rejected), None);
            assert_eq!(canonical_seat_ids(rejected), None);
        }

        for accepted in [2, 3, 4, 6] {
            assert!(supported_seat_count(accepted));
            assert_eq!(canonical_seat_ids(accepted).unwrap().len(), accepted);
        }

        assert_eq!(
            active_points_for_seat_count(2).unwrap(),
            &[StarPoint::North, StarPoint::South]
        );
        assert_eq!(
            active_points_for_seat_count(3).unwrap(),
            &[StarPoint::North, StarPoint::SouthEast, StarPoint::SouthWest]
        );
        assert_eq!(
            active_points_for_seat_count(4).unwrap(),
            &[
                StarPoint::North,
                StarPoint::NorthEast,
                StarPoint::South,
                StarPoint::SouthWest,
            ]
        );
        assert_eq!(active_points_for_seat_count(6).unwrap(), &StarPoint::ALL);
    }
}
