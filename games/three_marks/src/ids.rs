use engine_core::SeatId;
use game_stdlib::board_space::{Coord, Dimensions};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ThreeMarksSeat {
    Seat0,
    Seat1,
}

impl ThreeMarksSeat {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Seat0),
            1 => Some(Self::Seat1),
            _ => None,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::Seat0 => 0,
            Self::Seat1 => 1,
        }
    }

    pub fn other(self) -> Self {
        match self {
            Self::Seat0 => Self::Seat1,
            Self::Seat1 => Self::Seat0,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Seat0 => "seat_0",
            Self::Seat1 => "seat_1",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        let index = SeatId::parse_canonical(value)
            .ok()?
            .canonical_zero_based_index()
            .ok()?;
        Self::from_index(index as usize)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CellId {
    R1C1,
    R1C2,
    R1C3,
    R2C1,
    R2C2,
    R2C3,
    R3C1,
    R3C2,
    R3C3,
}

pub fn board_dimensions() -> Dimensions {
    Dimensions::checked(3, 3).expect("three_marks board dimensions are valid")
}

impl CellId {
    pub const ALL: [Self; 9] = [
        Self::R1C1,
        Self::R1C2,
        Self::R1C3,
        Self::R2C1,
        Self::R2C2,
        Self::R2C3,
        Self::R3C1,
        Self::R3C2,
        Self::R3C3,
    ];

    pub fn index(self) -> usize {
        self.to_coord()
            .row_col_index(board_dimensions())
            .expect("CellId is within the three_marks board")
    }

    pub fn to_coord(self) -> Coord {
        match self {
            Self::R1C1 => board_dimensions().coord(1, 1),
            Self::R1C2 => board_dimensions().coord(1, 2),
            Self::R1C3 => board_dimensions().coord(1, 3),
            Self::R2C1 => board_dimensions().coord(2, 1),
            Self::R2C2 => board_dimensions().coord(2, 2),
            Self::R2C3 => board_dimensions().coord(2, 3),
            Self::R3C1 => board_dimensions().coord(3, 1),
            Self::R3C2 => board_dimensions().coord(3, 2),
            Self::R3C3 => board_dimensions().coord(3, 3),
        }
        .expect("CellId coordinates are within the three_marks board")
    }

    pub fn from_coord(coord: Coord) -> Option<Self> {
        let index = coord.row_col_index(board_dimensions())?;
        Self::ALL.get(index).copied()
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::R1C1 => "r1c1",
            Self::R1C2 => "r1c2",
            Self::R1C3 => "r1c3",
            Self::R2C1 => "r2c1",
            Self::R2C2 => "r2c2",
            Self::R2C3 => "r2c3",
            Self::R3C1 => "r3c1",
            Self::R3C2 => "r3c2",
            Self::R3C3 => "r3c3",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        let coord = board_dimensions()
            .parse_coord_id(value)
            .ok()
            .filter(|coord| coord.id() == value)?;
        Self::from_coord(coord)
    }
}

pub const GAME_ID: &str = "three_marks";
pub const RULES_VERSION_LABEL: &str = "three_marks-rules-v1";
pub const VARIANT_ID: &str = "three_marks_standard";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_ids_parse_only_canonical_bounded_ids() {
        for seat in [ThreeMarksSeat::Seat0, ThreeMarksSeat::Seat1] {
            assert_eq!(ThreeMarksSeat::parse(seat.as_str()), Some(seat));
        }
    }

    #[test]
    fn seat_ids_reject_non_canonical_and_out_of_range_ids() {
        for value in [
            "seat_00", "seat_01", "seat_１", "seat-0", "seat-a", "seat_2",
        ] {
            assert_eq!(ThreeMarksSeat::parse(value), None, "{value}");
        }
    }

    #[test]
    fn cell_ids_round_trip_through_board_space_coords() {
        for cell in CellId::ALL {
            let coord = cell.to_coord();

            assert_eq!(CellId::from_coord(coord), Some(cell));
            assert_eq!(CellId::parse(&coord.id()), Some(cell));
            assert_eq!(cell.as_str(), coord.id());
        }

        assert_eq!(CellId::from_coord(Coord::checked(4, 1).unwrap()), None);
        assert_eq!(CellId::parse("r0c1"), None);
        assert_eq!(CellId::parse("r01c1"), None);
        assert_eq!(CellId::parse("r1c4"), None);
        assert_eq!(CellId::parse("cell-1"), None);
    }

    #[test]
    fn cell_ids_preserve_public_row_major_order_and_indices() {
        let expected = [
            CellId::R1C1,
            CellId::R1C2,
            CellId::R1C3,
            CellId::R2C1,
            CellId::R2C2,
            CellId::R2C3,
            CellId::R3C1,
            CellId::R3C2,
            CellId::R3C3,
        ];

        assert_eq!(CellId::ALL, expected);
        assert_eq!(
            board_dimensions()
                .row_major()
                .map(CellId::from_coord)
                .collect::<Option<Vec<_>>>()
                .unwrap(),
            expected
        );

        for (expected_index, cell) in expected.into_iter().enumerate() {
            assert_eq!(cell.index(), expected_index);
        }
    }
}
