use engine_core::SeatId;
use game_stdlib::board_space::{Coord, Dimensions};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DirectionalFlipSeat {
    Seat0,
    Seat1,
}

impl DirectionalFlipSeat {
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
pub enum RowId {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

pub fn board_dimensions() -> Dimensions {
    Dimensions::checked(8, 8).expect("directional_flip board dimensions are valid")
}

impl RowId {
    pub const ALL: [Self; 8] = [
        Self::R1,
        Self::R2,
        Self::R3,
        Self::R4,
        Self::R5,
        Self::R6,
        Self::R7,
        Self::R8,
    ];

    pub fn from_index(index: usize) -> Option<Self> {
        let row = u8::try_from(index + 1).ok()?;
        let coord = board_dimensions().coord(row, 1)?;
        Self::from_board_row(coord.row())
    }

    pub fn index(self) -> usize {
        board_dimensions()
            .coord(self.board_row(), 1)
            .expect("RowId is within the directional_flip board")
            .row_index()
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::R1 => "r1",
            Self::R2 => "r2",
            Self::R3 => "r3",
            Self::R4 => "r4",
            Self::R5 => "r5",
            Self::R6 => "r6",
            Self::R7 => "r7",
            Self::R8 => "r8",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        let row = value.strip_prefix('r')?.parse::<u8>().ok()?;
        let coord = board_dimensions().coord(row, 1)?;
        (value == format!("r{}", coord.row())).then(|| Self::from_board_row(coord.row()))?
    }

    fn board_row(self) -> u8 {
        match self {
            Self::R1 => 1,
            Self::R2 => 2,
            Self::R3 => 3,
            Self::R4 => 4,
            Self::R5 => 5,
            Self::R6 => 6,
            Self::R7 => 7,
            Self::R8 => 8,
        }
    }

    fn from_board_row(row: u8) -> Option<Self> {
        match row {
            1 => Some(Self::R1),
            2 => Some(Self::R2),
            3 => Some(Self::R3),
            4 => Some(Self::R4),
            5 => Some(Self::R5),
            6 => Some(Self::R6),
            7 => Some(Self::R7),
            8 => Some(Self::R8),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ColumnId {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
}

impl ColumnId {
    pub const ALL: [Self; 8] = [
        Self::C1,
        Self::C2,
        Self::C3,
        Self::C4,
        Self::C5,
        Self::C6,
        Self::C7,
        Self::C8,
    ];

    pub fn from_index(index: usize) -> Option<Self> {
        let col = u8::try_from(index + 1).ok()?;
        let coord = board_dimensions().coord(1, col)?;
        Self::from_board_col(coord.col())
    }

    pub fn index(self) -> usize {
        board_dimensions()
            .coord(1, self.board_col())
            .expect("ColumnId is within the directional_flip board")
            .col_index()
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::C1 => "c1",
            Self::C2 => "c2",
            Self::C3 => "c3",
            Self::C4 => "c4",
            Self::C5 => "c5",
            Self::C6 => "c6",
            Self::C7 => "c7",
            Self::C8 => "c8",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        let col = value.strip_prefix('c')?.parse::<u8>().ok()?;
        let coord = board_dimensions().coord(1, col)?;
        (value == format!("c{}", coord.col())).then(|| Self::from_board_col(coord.col()))?
    }

    fn board_col(self) -> u8 {
        match self {
            Self::C1 => 1,
            Self::C2 => 2,
            Self::C3 => 3,
            Self::C4 => 4,
            Self::C5 => 5,
            Self::C6 => 6,
            Self::C7 => 7,
            Self::C8 => 8,
        }
    }

    fn from_board_col(col: u8) -> Option<Self> {
        match col {
            1 => Some(Self::C1),
            2 => Some(Self::C2),
            3 => Some(Self::C3),
            4 => Some(Self::C4),
            5 => Some(Self::C5),
            6 => Some(Self::C6),
            7 => Some(Self::C7),
            8 => Some(Self::C8),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CellId {
    pub row: RowId,
    pub column: ColumnId,
}

impl CellId {
    pub const ALL: [Self; 64] = [
        Self::new(RowId::R1, ColumnId::C1),
        Self::new(RowId::R1, ColumnId::C2),
        Self::new(RowId::R1, ColumnId::C3),
        Self::new(RowId::R1, ColumnId::C4),
        Self::new(RowId::R1, ColumnId::C5),
        Self::new(RowId::R1, ColumnId::C6),
        Self::new(RowId::R1, ColumnId::C7),
        Self::new(RowId::R1, ColumnId::C8),
        Self::new(RowId::R2, ColumnId::C1),
        Self::new(RowId::R2, ColumnId::C2),
        Self::new(RowId::R2, ColumnId::C3),
        Self::new(RowId::R2, ColumnId::C4),
        Self::new(RowId::R2, ColumnId::C5),
        Self::new(RowId::R2, ColumnId::C6),
        Self::new(RowId::R2, ColumnId::C7),
        Self::new(RowId::R2, ColumnId::C8),
        Self::new(RowId::R3, ColumnId::C1),
        Self::new(RowId::R3, ColumnId::C2),
        Self::new(RowId::R3, ColumnId::C3),
        Self::new(RowId::R3, ColumnId::C4),
        Self::new(RowId::R3, ColumnId::C5),
        Self::new(RowId::R3, ColumnId::C6),
        Self::new(RowId::R3, ColumnId::C7),
        Self::new(RowId::R3, ColumnId::C8),
        Self::new(RowId::R4, ColumnId::C1),
        Self::new(RowId::R4, ColumnId::C2),
        Self::new(RowId::R4, ColumnId::C3),
        Self::new(RowId::R4, ColumnId::C4),
        Self::new(RowId::R4, ColumnId::C5),
        Self::new(RowId::R4, ColumnId::C6),
        Self::new(RowId::R4, ColumnId::C7),
        Self::new(RowId::R4, ColumnId::C8),
        Self::new(RowId::R5, ColumnId::C1),
        Self::new(RowId::R5, ColumnId::C2),
        Self::new(RowId::R5, ColumnId::C3),
        Self::new(RowId::R5, ColumnId::C4),
        Self::new(RowId::R5, ColumnId::C5),
        Self::new(RowId::R5, ColumnId::C6),
        Self::new(RowId::R5, ColumnId::C7),
        Self::new(RowId::R5, ColumnId::C8),
        Self::new(RowId::R6, ColumnId::C1),
        Self::new(RowId::R6, ColumnId::C2),
        Self::new(RowId::R6, ColumnId::C3),
        Self::new(RowId::R6, ColumnId::C4),
        Self::new(RowId::R6, ColumnId::C5),
        Self::new(RowId::R6, ColumnId::C6),
        Self::new(RowId::R6, ColumnId::C7),
        Self::new(RowId::R6, ColumnId::C8),
        Self::new(RowId::R7, ColumnId::C1),
        Self::new(RowId::R7, ColumnId::C2),
        Self::new(RowId::R7, ColumnId::C3),
        Self::new(RowId::R7, ColumnId::C4),
        Self::new(RowId::R7, ColumnId::C5),
        Self::new(RowId::R7, ColumnId::C6),
        Self::new(RowId::R7, ColumnId::C7),
        Self::new(RowId::R7, ColumnId::C8),
        Self::new(RowId::R8, ColumnId::C1),
        Self::new(RowId::R8, ColumnId::C2),
        Self::new(RowId::R8, ColumnId::C3),
        Self::new(RowId::R8, ColumnId::C4),
        Self::new(RowId::R8, ColumnId::C5),
        Self::new(RowId::R8, ColumnId::C6),
        Self::new(RowId::R8, ColumnId::C7),
        Self::new(RowId::R8, ColumnId::C8),
    ];

    pub const fn new(row: RowId, column: ColumnId) -> Self {
        Self { row, column }
    }

    pub fn index(self) -> usize {
        self.to_coord()
            .row_col_index(board_dimensions())
            .expect("CellId is within the directional_flip board")
    }

    pub fn as_string(self) -> String {
        self.to_coord().id()
    }

    pub fn parse(value: &str) -> Option<Self> {
        let coord = board_dimensions()
            .parse_coord_id(value)
            .ok()
            .filter(|coord| coord.id() == value)?;
        Self::from_coord(coord)
    }

    pub fn to_coord(self) -> Coord {
        board_dimensions()
            .coord(self.row.board_row(), self.column.board_col())
            .expect("CellId coordinates are within the directional_flip board")
    }

    pub fn from_coord(coord: Coord) -> Option<Self> {
        if !board_dimensions().contains(coord) {
            return None;
        }

        Some(Self {
            row: RowId::from_board_row(coord.row())?,
            column: ColumnId::from_board_col(coord.col())?,
        })
    }
}

pub const GAME_ID: &str = "directional_flip";
pub const RULES_VERSION_LABEL: &str = "directional_flip-rules-v1";
pub const VARIANT_ID: &str = "directional_flip_standard";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_ids_parse_only_canonical_bounded_ids() {
        for seat in [DirectionalFlipSeat::Seat0, DirectionalFlipSeat::Seat1] {
            assert_eq!(DirectionalFlipSeat::parse(seat.as_str()), Some(seat));
        }
    }

    #[test]
    fn seat_ids_reject_non_canonical_and_out_of_range_ids() {
        for value in [
            "seat_00", "seat_01", "seat_１", "seat-0", "seat-a", "seat_2",
        ] {
            assert_eq!(DirectionalFlipSeat::parse(value), None, "{value}");
        }
    }

    #[test]
    fn cell_ids_round_trip_through_board_space_coords() {
        for cell in CellId::ALL {
            let coord = cell.to_coord();

            assert_eq!(CellId::from_coord(coord), Some(cell));
            assert_eq!(CellId::parse(&coord.id()), Some(cell));
            assert_eq!(cell.as_string(), coord.id());
        }

        assert_eq!(CellId::from_coord(Coord::checked(9, 1).unwrap()), None);
        assert_eq!(CellId::parse("r0c1"), None);
        assert_eq!(CellId::parse("r01c1"), None);
        assert_eq!(CellId::parse("r1c9"), None);
        assert_eq!(CellId::parse("c1"), None);
    }

    #[test]
    fn cell_ids_preserve_public_row_major_order_and_indices() {
        let expected = CellId::ALL;

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
        assert_eq!(CellId::ALL.first().unwrap().as_string(), "r1c1");
        assert_eq!(CellId::ALL.last().unwrap().as_string(), "r8c8");
    }

    #[test]
    fn row_and_column_ids_preserve_public_order_and_canonical_parse() {
        assert_eq!(
            RowId::ALL,
            [
                RowId::R1,
                RowId::R2,
                RowId::R3,
                RowId::R4,
                RowId::R5,
                RowId::R6,
                RowId::R7,
                RowId::R8
            ]
        );
        assert_eq!(
            ColumnId::ALL,
            [
                ColumnId::C1,
                ColumnId::C2,
                ColumnId::C3,
                ColumnId::C4,
                ColumnId::C5,
                ColumnId::C6,
                ColumnId::C7,
                ColumnId::C8
            ]
        );

        for (index, row) in RowId::ALL.into_iter().enumerate() {
            assert_eq!(RowId::from_index(index), Some(row));
            assert_eq!(row.index(), index);
            assert_eq!(RowId::parse(row.as_str()), Some(row));
        }
        for (index, column) in ColumnId::ALL.into_iter().enumerate() {
            assert_eq!(ColumnId::from_index(index), Some(column));
            assert_eq!(column.index(), index);
            assert_eq!(ColumnId::parse(column.as_str()), Some(column));
        }

        assert_eq!(RowId::parse("r01"), None);
        assert_eq!(ColumnId::parse("c01"), None);
        assert_eq!(RowId::from_index(8), None);
        assert_eq!(ColumnId::from_index(8), None);
    }
}
