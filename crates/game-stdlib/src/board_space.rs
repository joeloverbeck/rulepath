//! Rule-agnostic rectangular board-space helpers.

use std::{fmt, num::NonZeroU8};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dimensions {
    rows: NonZeroU8,
    cols: NonZeroU8,
}

impl Dimensions {
    pub const fn new(rows: NonZeroU8, cols: NonZeroU8) -> Self {
        Self { rows, cols }
    }

    pub fn checked(rows: u8, cols: u8) -> Option<Self> {
        Some(Self::new(NonZeroU8::new(rows)?, NonZeroU8::new(cols)?))
    }

    pub const fn rows(self) -> u8 {
        self.rows.get()
    }

    pub const fn cols(self) -> u8 {
        self.cols.get()
    }

    pub fn contains(self, coord: Coord) -> bool {
        coord.row <= self.rows() && coord.col <= self.cols()
    }

    pub fn coord(self, row: u8, col: u8) -> Option<Coord> {
        let coord = Coord::checked(row, col)?;
        self.contains(coord).then_some(coord)
    }

    pub fn parse_coord_id(self, value: &str) -> Result<Coord, CoordIdError> {
        let coord = Coord::parse_id(value)?;
        if self.contains(coord) {
            Ok(coord)
        } else {
            Err(CoordIdError::OutOfBounds)
        }
    }

    pub fn offset(self, coord: Coord, d_row: i16, d_col: i16) -> Option<Coord> {
        if !self.contains(coord) {
            return None;
        }

        let row = i16::from(coord.row) + d_row;
        let col = i16::from(coord.col) + d_col;
        if row < 1 || col < 1 {
            return None;
        }

        let row = u8::try_from(row).ok()?;
        let col = u8::try_from(col).ok()?;
        self.coord(row, col)
    }

    pub const fn coord_count(self) -> usize {
        self.rows.get() as usize * self.cols.get() as usize
    }

    pub const fn row_major(self) -> RowMajor {
        RowMajor {
            dimensions: self,
            next_row: 1,
            next_col: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Coord {
    row: u8,
    col: u8,
}

impl Coord {
    pub fn checked(row: u8, col: u8) -> Option<Self> {
        (row != 0 && col != 0).then_some(Self { row, col })
    }

    pub const fn row(self) -> u8 {
        self.row
    }

    pub const fn col(self) -> u8 {
        self.col
    }

    pub fn row_index(self) -> usize {
        usize::from(self.row - 1)
    }

    pub fn col_index(self) -> usize {
        usize::from(self.col - 1)
    }

    pub fn row_col_index(self, dimensions: Dimensions) -> Option<usize> {
        dimensions
            .contains(self)
            .then_some(self.row_index() * usize::from(dimensions.cols()) + self.col_index())
    }

    pub fn id(self) -> String {
        format!("r{}c{}", self.row, self.col)
    }

    pub fn parse_id(value: &str) -> Result<Self, CoordIdError> {
        let Some(after_r) = value.strip_prefix('r') else {
            return Err(CoordIdError::Malformed);
        };
        let Some((row, col)) = after_r.split_once('c') else {
            return Err(CoordIdError::Malformed);
        };
        if row.is_empty() || col.is_empty() || col.contains('c') {
            return Err(CoordIdError::Malformed);
        }

        let row = row.parse::<u8>().map_err(|_| CoordIdError::Malformed)?;
        let col = col.parse::<u8>().map_err(|_| CoordIdError::Malformed)?;
        Self::checked(row, col).ok_or(CoordIdError::Zero)
    }

    pub const fn parity(self) -> Parity {
        Parity::of(self.row, self.col)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}c{}", self.row, self.col)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordIdError {
    Malformed,
    Zero,
    OutOfBounds,
}

impl fmt::Display for CoordIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Malformed => f.write_str("malformed coordinate id"),
            Self::Zero => f.write_str("coordinate id uses zero"),
            Self::OutOfBounds => f.write_str("coordinate id is out of bounds"),
        }
    }
}

impl std::error::Error for CoordIdError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Parity {
    Even,
    Odd,
}

impl Parity {
    pub const fn of(row: u8, col: u8) -> Self {
        if (row as u16 + col as u16) % 2 == 0 {
            Self::Even
        } else {
            Self::Odd
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RowMajor {
    dimensions: Dimensions,
    next_row: u8,
    next_col: u8,
}

impl Iterator for RowMajor {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row > self.dimensions.rows() {
            return None;
        }

        let coord = Coord {
            row: self.next_row,
            col: self.next_col,
        };

        if self.next_col == self.dimensions.cols() {
            self.next_row += 1;
            self.next_col = 1;
        } else {
            self.next_col += 1;
        }

        Some(coord)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = if self.next_row > self.dimensions.rows() {
            0
        } else {
            let completed = (usize::from(self.next_row - 1) * usize::from(self.dimensions.cols()))
                + usize::from(self.next_col - 1);
            self.dimensions.coord_count() - completed
        };
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for RowMajor {}

#[cfg(test)]
mod tests {
    use super::*;

    fn dims(rows: u8, cols: u8) -> Dimensions {
        Dimensions::checked(rows, cols).unwrap()
    }

    fn coord(row: u8, col: u8) -> Coord {
        Coord::checked(row, col).unwrap()
    }

    #[test]
    fn dimensions_require_nonzero_axes() {
        assert_eq!(Dimensions::checked(0, 8), None);
        assert_eq!(Dimensions::checked(8, 0), None);
        assert_eq!(dims(8, 8).rows(), 8);
        assert_eq!(dims(8, 8).cols(), 8);
    }

    #[test]
    fn coordinates_require_one_based_values() {
        assert_eq!(Coord::checked(0, 1), None);
        assert_eq!(Coord::checked(1, 0), None);
        assert_eq!(coord(2, 3).row_index(), 1);
        assert_eq!(coord(2, 3).col_index(), 2);
    }

    #[test]
    fn bounds_checks_coordinates() {
        let dimensions = dims(2, 3);

        assert!(dimensions.contains(coord(1, 1)));
        assert!(dimensions.contains(coord(2, 3)));
        assert!(!dimensions.contains(coord(3, 1)));
        assert!(!dimensions.contains(coord(1, 4)));
        assert_eq!(dimensions.coord(2, 3), Some(coord(2, 3)));
        assert_eq!(dimensions.coord(3, 3), None);
    }

    #[test]
    fn row_major_iteration_is_stable() {
        let ids: Vec<String> = dims(2, 3).row_major().map(Coord::id).collect();

        assert_eq!(ids, ["r1c1", "r1c2", "r1c3", "r2c1", "r2c2", "r2c3"]);
    }

    #[test]
    fn row_major_size_hint_tracks_remaining_items() {
        let mut iter = dims(2, 2).row_major();

        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(coord(1, 1)));
        assert_eq!(iter.len(), 3);
        assert_eq!(
            iter.collect::<Vec<_>>(),
            [coord(1, 2), coord(2, 1), coord(2, 2)]
        );
    }

    #[test]
    fn coord_ids_round_trip_and_validate_bounds() {
        let dimensions = dims(8, 8);
        let parsed = dimensions.parse_coord_id("r7c8").unwrap();

        assert_eq!(parsed, coord(7, 8));
        assert_eq!(parsed.id(), "r7c8");
        assert_eq!(parsed.to_string(), "r7c8");
        assert_eq!(
            dimensions.parse_coord_id("r9c1"),
            Err(CoordIdError::OutOfBounds)
        );
    }

    #[test]
    fn coord_id_parser_rejects_invalid_values() {
        assert_eq!(Coord::parse_id("1c2"), Err(CoordIdError::Malformed));
        assert_eq!(Coord::parse_id("r1"), Err(CoordIdError::Malformed));
        assert_eq!(Coord::parse_id("r1c2c3"), Err(CoordIdError::Malformed));
        assert_eq!(Coord::parse_id("r0c1"), Err(CoordIdError::Zero));
        assert_eq!(Coord::parse_id("r1c0"), Err(CoordIdError::Zero));
        assert_eq!(Coord::parse_id("r256c1"), Err(CoordIdError::Malformed));
    }

    #[test]
    fn signed_offsets_are_bounded() {
        let dimensions = dims(8, 8);

        assert_eq!(dimensions.offset(coord(4, 4), -1, 1), Some(coord(3, 5)));
        assert_eq!(dimensions.offset(coord(1, 1), -1, 0), None);
        assert_eq!(dimensions.offset(coord(8, 8), 1, 0), None);
        assert_eq!(dimensions.offset(coord(9, 1), 0, 0), None);
    }

    #[test]
    fn row_column_index_uses_row_major_order() {
        let dimensions = dims(3, 3);

        assert_eq!(coord(1, 1).row_col_index(dimensions), Some(0));
        assert_eq!(coord(2, 1).row_col_index(dimensions), Some(3));
        assert_eq!(coord(3, 3).row_col_index(dimensions), Some(8));
        assert_eq!(coord(4, 1).row_col_index(dimensions), None);
    }

    #[test]
    fn parity_is_generic() {
        assert_eq!(coord(1, 1).parity(), Parity::Even);
        assert_eq!(coord(1, 2).parity(), Parity::Odd);
        assert_eq!(Parity::of(2, 2), Parity::Even);
        assert_eq!(Parity::of(2, 3), Parity::Odd);
    }
}
