use engine_core::SeatId;

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
pub enum Sigil {
    A,
    B,
}

impl Sigil {
    pub const ALL: [Self; 2] = [Self::A, Self::B];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A => "a",
            Self::B => "b",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "a" => Some(Self::A),
            "b" => Some(Self::B),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CardId {
    rank: u8,
    sigil: Sigil,
}

impl CardId {
    pub fn new(rank: u8, sigil: Sigil) -> Option<Self> {
        (1..=STANDARD_RANK_COUNT)
            .contains(&rank)
            .then_some(Self { rank, sigil })
    }

    pub const fn rank(self) -> u8 {
        self.rank
    }

    pub const fn sigil(self) -> Sigil {
        self.sigil
    }

    pub fn stable_id(self) -> String {
        format!("hcd:r{:02}:{}", self.rank, self.sigil.as_str())
    }

    pub fn parse(value: &str) -> Option<Self> {
        let mut parts = value.split(':');
        let prefix = parts.next()?;
        let rank = parts.next()?;
        let sigil = parts.next()?;
        if parts.next().is_some() || prefix != "hcd" {
            return None;
        }
        let rank = rank.strip_prefix('r')?.parse::<u8>().ok()?;
        Self::new(rank, Sigil::parse(sigil)?)
    }
}

pub fn canonical_deck() -> Vec<CardId> {
    let mut cards = Vec::with_capacity(STANDARD_DECK_CARD_COUNT as usize);
    for rank in 1..=STANDARD_RANK_COUNT {
        for sigil in Sigil::ALL {
            cards.push(CardId::new(rank, sigil).expect("canonical rank is valid"));
        }
    }
    cards
}

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
        let seat_id = SeatId::parse_canonical(value).ok()?;
        let index = usize::try_from(seat_id.canonical_zero_based_index().ok()?).ok()?;
        Self::from_index(index)
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
        assert_eq!(
            HighCardDuelSeat::parse("seat_0"),
            Some(HighCardDuelSeat::Seat0)
        );
        assert_eq!(
            HighCardDuelSeat::parse("seat_1"),
            Some(HighCardDuelSeat::Seat1)
        );
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
            assert_eq!(HighCardDuelSeat::parse(rejected), None, "{rejected}");
        }
    }

    #[test]
    fn canonical_card_ids_are_stable_and_bounded() {
        let first = CardId::new(1, Sigil::A).unwrap();
        let last = CardId::new(12, Sigil::B).unwrap();

        assert_eq!(first.stable_id(), "hcd:r01:a");
        assert_eq!(last.stable_id(), "hcd:r12:b");
        assert_eq!(CardId::new(0, Sigil::A), None);
        assert_eq!(CardId::new(13, Sigil::A), None);

        let deck = canonical_deck();
        assert_eq!(deck.len(), STANDARD_DECK_CARD_COUNT as usize);
        assert_eq!(deck.first().copied(), Some(first));
        assert_eq!(deck.last().copied(), Some(last));
    }
}
