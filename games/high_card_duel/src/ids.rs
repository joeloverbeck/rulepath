pub const GAME_ID: &str = "high_card_duel";
pub const VARIANT_ID: &str = "high_card_duel_standard";
pub const RULES_VERSION_LABEL: &str = "high-card-duel-rules-v1";
pub const STANDARD_ROUND_LIMIT: u8 = 6;
pub const STANDARD_HAND_SIZE: u8 = 3;
pub const STANDARD_RANK_COUNT: u8 = 12;
pub const STANDARD_SIGILS_PER_RANK: u8 = 2;
pub const STANDARD_DECK_CARD_COUNT: u8 = STANDARD_RANK_COUNT * STANDARD_SIGILS_PER_RANK;
pub const SHUFFLE_ALGORITHM: &str = "hcd-shuffle-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum HighCardDuelSeat {
    Seat0,
    Seat1,
}

impl HighCardDuelSeat {
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
        match value {
            "seat_0" => Some(Self::Seat0),
            "seat_1" => Some(Self::Seat1),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seats_are_stable() {
        assert_eq!(
            HighCardDuelSeat::from_index(0),
            Some(HighCardDuelSeat::Seat0)
        );
        assert_eq!(
            HighCardDuelSeat::from_index(1),
            Some(HighCardDuelSeat::Seat1)
        );
        assert_eq!(HighCardDuelSeat::from_index(2), None);
        assert_eq!(HighCardDuelSeat::Seat0.other(), HighCardDuelSeat::Seat1);
        assert_eq!(HighCardDuelSeat::Seat1.as_str(), "seat_1");
    }
}
