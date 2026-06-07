use game_stdlib::board_space::{Coord, Dimensions, Parity};

pub const GAME_ID: &str = "draughts_lite";
pub const VARIANT_ID: &str = "draughts_lite_standard";
pub const RULES_VERSION_LABEL: &str = "draughts_lite-rules-v1";
pub const BOARD_ROWS: u8 = 8;
pub const BOARD_COLS: u8 = 8;
pub const STANDARD_PIECES_PER_SEAT: u8 = 12;
pub const TOTAL_STANDARD_PIECES: usize = 24;

pub fn board_dimensions() -> Dimensions {
    Dimensions::checked(BOARD_ROWS, BOARD_COLS).expect("standard board dimensions are nonzero")
}

pub fn is_playable_cell(cell: Coord) -> bool {
    cell.parity() == Parity::Odd
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum DraughtsLiteSeat {
    Seat0,
    Seat1,
}

impl DraughtsLiteSeat {
    pub const ALL: [Self; 2] = [Self::Seat0, Self::Seat1];

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
        match value {
            "seat_0" => Some(Self::Seat0),
            "seat_1" => Some(Self::Seat1),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PieceId {
    owner: DraughtsLiteSeat,
    ordinal: u8,
}

impl PieceId {
    pub fn new(owner: DraughtsLiteSeat, ordinal: u8) -> Option<Self> {
        (1..=STANDARD_PIECES_PER_SEAT)
            .contains(&ordinal)
            .then_some(Self { owner, ordinal })
    }

    pub const fn owner(self) -> DraughtsLiteSeat {
        self.owner
    }

    pub const fn ordinal(self) -> u8 {
        self.ordinal
    }

    pub fn stable_id(self) -> String {
        format!("{}-p{:02}", self.owner.as_str(), self.ordinal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_ids_are_stable_and_bounded() {
        let id = PieceId::new(DraughtsLiteSeat::Seat1, 7).unwrap();

        assert_eq!(id.owner(), DraughtsLiteSeat::Seat1);
        assert_eq!(id.ordinal(), 7);
        assert_eq!(id.stable_id(), "seat_1-p07");
        assert_eq!(PieceId::new(DraughtsLiteSeat::Seat0, 0), None);
        assert_eq!(PieceId::new(DraughtsLiteSeat::Seat0, 13), None);
    }

    #[test]
    fn playable_cell_parity_is_game_local() {
        assert!(!is_playable_cell(Coord::checked(1, 1).unwrap()));
        assert!(is_playable_cell(Coord::checked(1, 2).unwrap()));
    }
}
