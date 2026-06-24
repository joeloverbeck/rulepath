use engine_core::SeatId;

pub const GAME_ID: &str = "briar_circuit";
pub const VARIANT_ID: &str = "briar_circuit_standard";
pub const RULES_VERSION_LABEL: &str = "briar-circuit-rules-v1";
pub const DATA_VERSION_LABEL: &str = "briar-circuit-data-v1";
pub const STANDARD_SEAT_COUNT: u8 = 4;
pub const STANDARD_MIN_SEATS: u8 = STANDARD_SEAT_COUNT;
pub const STANDARD_DEFAULT_SEATS: u8 = STANDARD_SEAT_COUNT;
pub const STANDARD_MAX_SEATS: u8 = STANDARD_SEAT_COUNT;
pub const STANDARD_SUIT_COUNT: u8 = 4;
pub const STANDARD_RANK_COUNT: u8 = 13;
pub const STANDARD_CARD_COUNT: u8 = STANDARD_SUIT_COUNT * STANDARD_RANK_COUNT;
pub const STANDARD_HAND_SIZE: u8 = 13;
pub const STANDARD_TRICKS_PER_HAND: u8 = 13;
pub const STANDARD_RAW_POINTS_PER_HAND: u8 = 26;
pub const STANDARD_MATCH_THRESHOLD: u16 = 100;
pub const STANDARD_PASS_SIZE: u8 = 3;
pub const ACTION_PASS: &str = "pass";
pub const ACTION_PASS_SELECT: &str = "select";
pub const ACTION_PASS_UNSELECT: &str = "unselect";
pub const ACTION_PASS_CONFIRM: &str = "confirm";
pub const ACTION_PLAY: &str = "play";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum BriarCircuitSeat {
    Seat0,
    Seat1,
    Seat2,
    Seat3,
}

impl BriarCircuitSeat {
    pub const ALL: [Self; 4] = [Self::Seat0, Self::Seat1, Self::Seat2, Self::Seat3];

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Seat0),
            1 => Some(Self::Seat1),
            2 => Some(Self::Seat2),
            3 => Some(Self::Seat3),
            _ => None,
        }
    }

    pub const fn index(self) -> usize {
        match self {
            Self::Seat0 => 0,
            Self::Seat1 => 1,
            Self::Seat2 => 2,
            Self::Seat3 => 3,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Seat0 => "seat_0",
            Self::Seat1 => "seat_1",
            Self::Seat2 => "seat_2",
            Self::Seat3 => "seat_3",
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
            Self::Seat0 => Self::Seat1,
            Self::Seat1 => Self::Seat2,
            Self::Seat2 => Self::Seat3,
            Self::Seat3 => Self::Seat0,
        }
    }

    pub const fn pass_left_target(self) -> Self {
        self.next_clockwise()
    }

    pub const fn pass_right_target(self) -> Self {
        match self {
            Self::Seat0 => Self::Seat3,
            Self::Seat1 => Self::Seat0,
            Self::Seat2 => Self::Seat1,
            Self::Seat3 => Self::Seat2,
        }
    }

    pub const fn pass_across_target(self) -> Self {
        match self {
            Self::Seat0 => Self::Seat2,
            Self::Seat1 => Self::Seat3,
            Self::Seat2 => Self::Seat0,
            Self::Seat3 => Self::Seat1,
        }
    }
}

pub fn seat_id_for_index(index: usize) -> SeatId {
    SeatId(format!("seat_{index}"))
}

pub fn canonical_seat_ids() -> [SeatId; 4] {
    [
        seat_id_for_index(0),
        seat_id_for_index(1),
        seat_id_for_index(2),
        seat_id_for_index(3),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_parser_accepts_only_bounded_canonical_ids() {
        let accepted = [
            ("seat_0", BriarCircuitSeat::Seat0),
            ("seat_1", BriarCircuitSeat::Seat1),
            ("seat_2", BriarCircuitSeat::Seat2),
            ("seat_3", BriarCircuitSeat::Seat3),
        ];
        for (input, expected) in accepted {
            assert_eq!(BriarCircuitSeat::parse(input), Some(expected));
        }

        for rejected in [
            "seat_4", "seat-0", "seat-a", "seat_", "seat_01", "seat_0 ", " seat_0", "Seat_0", "",
        ] {
            assert_eq!(BriarCircuitSeat::parse(rejected), None, "{rejected}");
        }
    }
}
