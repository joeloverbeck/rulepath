pub const GAME_ID: &str = "plain_tricks";
pub const VARIANT_ID: &str = "plain_tricks_standard";
pub const RULES_VERSION_LABEL: &str = "plain-tricks-rules-v1";
pub const STANDARD_SEAT_COUNT: u8 = 2;
pub const STANDARD_SUIT_COUNT: u8 = 3;
pub const STANDARD_RANK_COUNT: u8 = 6;
pub const STANDARD_CARD_COUNT: u8 = STANDARD_SUIT_COUNT * STANDARD_RANK_COUNT;
pub const STANDARD_HAND_SIZE: u8 = 6;
pub const STANDARD_TAIL_SIZE: u8 = 6;
pub const STANDARD_TRICKS_PER_ROUND: u8 = 6;
pub const STANDARD_ROUND_COUNT: u8 = 2;
pub const STANDARD_TOTAL_TRICKS: u8 = STANDARD_TRICKS_PER_ROUND * STANDARD_ROUND_COUNT;
pub const STANDARD_MAX_PLAYS: u8 = STANDARD_TOTAL_TRICKS * 2;

pub const ACTION_PLAY: &str = "play";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum PlainTricksSeat {
    Seat0,
    Seat1,
}

impl PlainTricksSeat {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TrickSuit {
    Gale,
    River,
    Ember,
}

impl TrickSuit {
    pub const ALL: [Self; 3] = [Self::Gale, Self::River, Self::Ember];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gale => "gale",
            Self::River => "river",
            Self::Ember => "ember",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Gale => "Gale",
            Self::River => "River",
            Self::Ember => "Ember",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "gale" => Some(Self::Gale),
            "river" => Some(Self::River),
            "ember" => Some(Self::Ember),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TrickRank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl TrickRank {
    pub const ALL: [Self; 6] = [
        Self::One,
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::One => "1",
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
        }
    }

    pub const fn value(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "1" => Some(Self::One),
            "2" => Some(Self::Two),
            "3" => Some(Self::Three),
            "4" => Some(Self::Four),
            "5" => Some(Self::Five),
            "6" => Some(Self::Six),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TrickCardId {
    Gale1,
    Gale2,
    Gale3,
    Gale4,
    Gale5,
    Gale6,
    River1,
    River2,
    River3,
    River4,
    River5,
    River6,
    Ember1,
    Ember2,
    Ember3,
    Ember4,
    Ember5,
    Ember6,
}

impl TrickCardId {
    pub const ALL: [Self; 18] = [
        Self::Gale1,
        Self::Gale2,
        Self::Gale3,
        Self::Gale4,
        Self::Gale5,
        Self::Gale6,
        Self::River1,
        Self::River2,
        Self::River3,
        Self::River4,
        Self::River5,
        Self::River6,
        Self::Ember1,
        Self::Ember2,
        Self::Ember3,
        Self::Ember4,
        Self::Ember5,
        Self::Ember6,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gale1 => "gale_1",
            Self::Gale2 => "gale_2",
            Self::Gale3 => "gale_3",
            Self::Gale4 => "gale_4",
            Self::Gale5 => "gale_5",
            Self::Gale6 => "gale_6",
            Self::River1 => "river_1",
            Self::River2 => "river_2",
            Self::River3 => "river_3",
            Self::River4 => "river_4",
            Self::River5 => "river_5",
            Self::River6 => "river_6",
            Self::Ember1 => "ember_1",
            Self::Ember2 => "ember_2",
            Self::Ember3 => "ember_3",
            Self::Ember4 => "ember_4",
            Self::Ember5 => "ember_5",
            Self::Ember6 => "ember_6",
        }
    }

    pub const fn suit(self) -> TrickSuit {
        match self {
            Self::Gale1 | Self::Gale2 | Self::Gale3 | Self::Gale4 | Self::Gale5 | Self::Gale6 => {
                TrickSuit::Gale
            }
            Self::River1
            | Self::River2
            | Self::River3
            | Self::River4
            | Self::River5
            | Self::River6 => TrickSuit::River,
            Self::Ember1
            | Self::Ember2
            | Self::Ember3
            | Self::Ember4
            | Self::Ember5
            | Self::Ember6 => TrickSuit::Ember,
        }
    }

    pub const fn rank(self) -> TrickRank {
        match self {
            Self::Gale1 | Self::River1 | Self::Ember1 => TrickRank::One,
            Self::Gale2 | Self::River2 | Self::Ember2 => TrickRank::Two,
            Self::Gale3 | Self::River3 | Self::Ember3 => TrickRank::Three,
            Self::Gale4 | Self::River4 | Self::Ember4 => TrickRank::Four,
            Self::Gale5 | Self::River5 | Self::Ember5 => TrickRank::Five,
            Self::Gale6 | Self::River6 | Self::Ember6 => TrickRank::Six,
        }
    }

    pub fn label(self) -> String {
        format!("{} {}", self.suit().label(), self.rank().as_str())
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "gale_1" => Some(Self::Gale1),
            "gale_2" => Some(Self::Gale2),
            "gale_3" => Some(Self::Gale3),
            "gale_4" => Some(Self::Gale4),
            "gale_5" => Some(Self::Gale5),
            "gale_6" => Some(Self::Gale6),
            "river_1" => Some(Self::River1),
            "river_2" => Some(Self::River2),
            "river_3" => Some(Self::River3),
            "river_4" => Some(Self::River4),
            "river_5" => Some(Self::River5),
            "river_6" => Some(Self::River6),
            "ember_1" => Some(Self::Ember1),
            "ember_2" => Some(Self::Ember2),
            "ember_3" => Some(Self::Ember3),
            "ember_4" => Some(Self::Ember4),
            "ember_5" => Some(Self::Ember5),
            "ember_6" => Some(Self::Ember6),
            _ => None,
        }
    }
}

pub const fn canonical_deck() -> [TrickCardId; 18] {
    TrickCardId::ALL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seats_are_stable() {
        assert_eq!(PlainTricksSeat::from_index(0), Some(PlainTricksSeat::Seat0));
        assert_eq!(PlainTricksSeat::from_index(1), Some(PlainTricksSeat::Seat1));
        assert_eq!(PlainTricksSeat::from_index(2), None);
        assert_eq!(PlainTricksSeat::Seat0.index(), 0);
        assert_eq!(PlainTricksSeat::Seat1.other(), PlainTricksSeat::Seat0);
        assert_eq!(PlainTricksSeat::Seat1.as_str(), "seat_1");
        assert_eq!(
            PlainTricksSeat::parse("seat_0"),
            Some(PlainTricksSeat::Seat0)
        );
    }

    #[test]
    fn suits_and_ranks_are_stable() {
        assert_eq!(TrickSuit::Gale.as_str(), "gale");
        assert_eq!(TrickSuit::River.label(), "River");
        assert_eq!(TrickSuit::parse("ember"), Some(TrickSuit::Ember));
        assert_eq!(TrickRank::One.as_str(), "1");
        assert_eq!(TrickRank::Six.value(), 6);
        assert_eq!(TrickRank::parse("4"), Some(TrickRank::Four));
        assert_eq!(TrickRank::parse("7"), None);
    }

    #[test]
    fn canonical_card_ids_match_rules_order() {
        let deck = canonical_deck();

        assert_eq!(deck.len(), STANDARD_CARD_COUNT as usize);
        assert_eq!(deck[0].as_str(), "gale_1");
        assert_eq!(deck[17].as_str(), "ember_6");
        assert_eq!(TrickCardId::River4.suit(), TrickSuit::River);
        assert_eq!(TrickCardId::River4.rank(), TrickRank::Four);
        assert_eq!(TrickCardId::Ember6.label(), "Ember 6");
        assert_eq!(TrickCardId::parse("gale_3"), Some(TrickCardId::Gale3));
        assert_eq!(TrickCardId::parse("bad"), None);
    }

    #[test]
    fn action_segments_are_neutral_and_stable() {
        assert_eq!(ACTION_PLAY, "play");
    }
}
