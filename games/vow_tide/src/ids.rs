use engine_core::SeatId;

pub const GAME_ID: &str = "vow_tide";
pub const VARIANT_ID: &str = "vow_tide_standard";
pub const RULES_VERSION_LABEL: &str = "vow-tide-rules-v1";
pub const DATA_VERSION_LABEL: &str = "vow-tide-data-v1";
pub const STANDARD_MIN_SEATS: u8 = 3;
pub const STANDARD_DEFAULT_SEATS: u8 = 4;
pub const STANDARD_MAX_SEATS: u8 = 7;
pub const STANDARD_SUIT_COUNT: u8 = 4;
pub const STANDARD_RANK_COUNT: u8 = 13;
pub const STANDARD_CARD_COUNT: u8 = STANDARD_SUIT_COUNT * STANDARD_RANK_COUNT;
pub const STANDARD_MAX_HAND_SIZE: u8 = 10;
pub const ACTION_BID: &str = "bid";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum VowTideSeat {
    Seat0,
    Seat1,
    Seat2,
    Seat3,
    Seat4,
    Seat5,
    Seat6,
}

impl VowTideSeat {
    pub const ALL: [Self; 7] = [
        Self::Seat0,
        Self::Seat1,
        Self::Seat2,
        Self::Seat3,
        Self::Seat4,
        Self::Seat5,
        Self::Seat6,
    ];

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Seat0),
            1 => Some(Self::Seat1),
            2 => Some(Self::Seat2),
            3 => Some(Self::Seat3),
            4 => Some(Self::Seat4),
            5 => Some(Self::Seat5),
            6 => Some(Self::Seat6),
            _ => None,
        }
    }

    pub const fn index(self) -> usize {
        match self {
            Self::Seat0 => 0,
            Self::Seat1 => 1,
            Self::Seat2 => 2,
            Self::Seat3 => 3,
            Self::Seat4 => 4,
            Self::Seat5 => 5,
            Self::Seat6 => 6,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Seat0 => "seat_0",
            Self::Seat1 => "seat_1",
            Self::Seat2 => "seat_2",
            Self::Seat3 => "seat_3",
            Self::Seat4 => "seat_4",
            Self::Seat5 => "seat_5",
            Self::Seat6 => "seat_6",
        }
    }

    pub const fn fallback_label(self) -> &'static str {
        match self {
            Self::Seat0 => "Tide 1",
            Self::Seat1 => "Tide 2",
            Self::Seat2 => "Tide 3",
            Self::Seat3 => "Tide 4",
            Self::Seat4 => "Tide 5",
            Self::Seat5 => "Tide 6",
            Self::Seat6 => "Tide 7",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        let raw_index = SeatId::parse_canonical(value)
            .ok()?
            .canonical_zero_based_index()
            .ok()? as usize;
        Self::from_index(raw_index)
    }

    pub fn next_clockwise(self, seat_count: usize) -> Self {
        debug_assert!(
            (STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize).contains(&seat_count)
        );
        let next = (self.index() + 1) % seat_count;
        Self::from_index(next).expect("validated seat count keeps next seat in range")
    }
}

pub fn supported_seat_count(seat_count: usize) -> bool {
    (STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize).contains(&seat_count)
}

pub fn seat_id_for_index(index: usize) -> SeatId {
    SeatId(format!("seat_{index}"))
}

pub fn canonical_seat_ids(seat_count: usize) -> Vec<SeatId> {
    (0..seat_count).map(seat_id_for_index).collect()
}

pub fn max_hand_size_for_seats(seat_count: usize) -> Option<u8> {
    if !supported_seat_count(seat_count) {
        return None;
    }
    let deck_without_trump_reveal = (STANDARD_CARD_COUNT - 1) as usize;
    Some(STANDARD_MAX_HAND_SIZE.min((deck_without_trump_reveal / seat_count) as u8))
}

pub fn hand_schedule_for_seats(seat_count: usize) -> Option<Vec<u8>> {
    let max_hand_size = max_hand_size_for_seats(seat_count)?;
    let mut schedule = Vec::with_capacity((max_hand_size as usize * 2) - 1);
    for hand_size in (1..=max_hand_size).rev() {
        schedule.push(hand_size);
    }
    for hand_size in 2..=max_hand_size {
        schedule.push(hand_size);
    }
    Some(schedule)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_parser_accepts_only_bounded_canonical_ids() {
        let accepted = [
            ("seat_0", VowTideSeat::Seat0),
            ("seat_1", VowTideSeat::Seat1),
            ("seat_2", VowTideSeat::Seat2),
            ("seat_3", VowTideSeat::Seat3),
            ("seat_4", VowTideSeat::Seat4),
            ("seat_5", VowTideSeat::Seat5),
            ("seat_6", VowTideSeat::Seat6),
        ];
        for (input, expected) in accepted {
            assert_eq!(VowTideSeat::parse(input), Some(expected));
        }

        for rejected in [
            "seat_7", "seat-0", "seat-a", "seat_", "seat_01", "seat_0 ", " seat_0", "Seat_0", "",
        ] {
            assert_eq!(VowTideSeat::parse(rejected), None, "{rejected}");
        }
    }
}
