use engine_core::SeatId;

pub const GAME_ID: &str = "secret_draft";
pub const VARIANT_ID: &str = "secret_draft_standard";
pub const RULES_VERSION_LABEL: &str = "secret-draft-rules-v1";
pub const STANDARD_SEAT_COUNT: u8 = 2;
pub const STANDARD_ROUND_COUNT: u8 = 6;
pub const STANDARD_ITEM_COUNT: u8 = 12;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SecretDraftSeat {
    Seat0,
    Seat1,
}

impl SecretDraftSeat {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum DraftThread {
    Ember,
    Tide,
    Grove,
}

impl DraftThread {
    pub const ALL: [Self; 3] = [Self::Ember, Self::Tide, Self::Grove];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ember => "ember",
            Self::Tide => "tide",
            Self::Grove => "grove",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "ember" => Some(Self::Ember),
            "tide" => Some(Self::Tide),
            "grove" => Some(Self::Grove),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seats_are_stable() {
        assert_eq!(SecretDraftSeat::from_index(0), Some(SecretDraftSeat::Seat0));
        assert_eq!(SecretDraftSeat::from_index(1), Some(SecretDraftSeat::Seat1));
        assert_eq!(SecretDraftSeat::from_index(2), None);
        assert_eq!(SecretDraftSeat::Seat0.other(), SecretDraftSeat::Seat1);
        assert_eq!(SecretDraftSeat::Seat1.as_str(), "seat_1");
        assert_eq!(
            SecretDraftSeat::parse("seat_0"),
            Some(SecretDraftSeat::Seat0)
        );
        assert_eq!(
            SecretDraftSeat::parse("seat_1"),
            Some(SecretDraftSeat::Seat1)
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
            assert_eq!(SecretDraftSeat::parse(rejected), None, "{rejected}");
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum DraftItemId {
    Ember1,
    Ember2,
    Ember3,
    Ember4,
    Tide1,
    Tide2,
    Tide3,
    Tide4,
    Grove1,
    Grove2,
    Grove3,
    Grove4,
}

impl DraftItemId {
    pub const ALL: [Self; 12] = [
        Self::Ember1,
        Self::Ember2,
        Self::Ember3,
        Self::Ember4,
        Self::Tide1,
        Self::Tide2,
        Self::Tide3,
        Self::Tide4,
        Self::Grove1,
        Self::Grove2,
        Self::Grove3,
        Self::Grove4,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ember1 => "ember_1",
            Self::Ember2 => "ember_2",
            Self::Ember3 => "ember_3",
            Self::Ember4 => "ember_4",
            Self::Tide1 => "tide_1",
            Self::Tide2 => "tide_2",
            Self::Tide3 => "tide_3",
            Self::Tide4 => "tide_4",
            Self::Grove1 => "grove_1",
            Self::Grove2 => "grove_2",
            Self::Grove3 => "grove_3",
            Self::Grove4 => "grove_4",
        }
    }

    pub const fn thread(self) -> DraftThread {
        match self {
            Self::Ember1 | Self::Ember2 | Self::Ember3 | Self::Ember4 => DraftThread::Ember,
            Self::Tide1 | Self::Tide2 | Self::Tide3 | Self::Tide4 => DraftThread::Tide,
            Self::Grove1 | Self::Grove2 | Self::Grove3 | Self::Grove4 => DraftThread::Grove,
        }
    }

    pub const fn value(self) -> u8 {
        match self {
            Self::Ember1 | Self::Tide1 | Self::Grove1 => 1,
            Self::Ember2 | Self::Tide2 | Self::Grove2 => 2,
            Self::Ember3 | Self::Tide3 | Self::Grove3 => 3,
            Self::Ember4 | Self::Tide4 | Self::Grove4 => 4,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Ember1 => "Ember One",
            Self::Ember2 => "Ember Two",
            Self::Ember3 => "Ember Three",
            Self::Ember4 => "Ember Four",
            Self::Tide1 => "Tide One",
            Self::Tide2 => "Tide Two",
            Self::Tide3 => "Tide Three",
            Self::Tide4 => "Tide Four",
            Self::Grove1 => "Grove One",
            Self::Grove2 => "Grove Two",
            Self::Grove3 => "Grove Three",
            Self::Grove4 => "Grove Four",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "ember_1" => Some(Self::Ember1),
            "ember_2" => Some(Self::Ember2),
            "ember_3" => Some(Self::Ember3),
            "ember_4" => Some(Self::Ember4),
            "tide_1" => Some(Self::Tide1),
            "tide_2" => Some(Self::Tide2),
            "tide_3" => Some(Self::Tide3),
            "tide_4" => Some(Self::Tide4),
            "grove_1" => Some(Self::Grove1),
            "grove_2" => Some(Self::Grove2),
            "grove_3" => Some(Self::Grove3),
            "grove_4" => Some(Self::Grove4),
            _ => None,
        }
    }
}
