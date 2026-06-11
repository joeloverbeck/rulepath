pub const GAME_ID: &str = "masked_claims";
pub const VARIANT_ID: &str = "masked_claims_standard";
pub const RULES_VERSION_LABEL: &str = "masked-claims-rules-v1";
pub const STANDARD_SEAT_COUNT: u8 = 2;
pub const STANDARD_GRADE_COUNT: u8 = 5;
pub const STANDARD_TILES_PER_GRADE: u8 = 3;
pub const STANDARD_MASK_COUNT: u8 = STANDARD_GRADE_COUNT * STANDARD_TILES_PER_GRADE;
pub const STANDARD_HAND_SIZE: u8 = 5;
pub const STANDARD_RESERVE_SIZE: u8 = 5;
pub const STANDARD_CLAIMS_PER_SEAT: u8 = 4;
pub const STANDARD_MAX_TURNS: u8 = STANDARD_CLAIMS_PER_SEAT * STANDARD_SEAT_COUNT;

pub const ACTION_CLAIM: &str = "claim";
pub const ACTION_RESPOND_ACCEPT: &str = "respond/accept";
pub const ACTION_RESPOND_CHALLENGE: &str = "respond/challenge";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum MaskedClaimsSeat {
    Seat0,
    Seat1,
}

impl MaskedClaimsSeat {
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
pub enum Grade {
    Plain,
    Trimmed,
    Gilded,
    Jeweled,
    Master,
}

impl Grade {
    pub const ALL: [Self; 5] = [
        Self::Plain,
        Self::Trimmed,
        Self::Gilded,
        Self::Jeweled,
        Self::Master,
    ];

    pub const fn value(self) -> u8 {
        match self {
            Self::Plain => 1,
            Self::Trimmed => 2,
            Self::Gilded => 3,
            Self::Jeweled => 4,
            Self::Master => 5,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "1",
            Self::Trimmed => "2",
            Self::Gilded => "3",
            Self::Jeweled => "4",
            Self::Master => "5",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Plain => "Plain",
            Self::Trimmed => "Trimmed",
            Self::Gilded => "Gilded",
            Self::Jeweled => "Jeweled",
            Self::Master => "Master",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "1" | "plain" => Some(Self::Plain),
            "2" | "trimmed" => Some(Self::Trimmed),
            "3" | "gilded" => Some(Self::Gilded),
            "4" | "jeweled" => Some(Self::Jeweled),
            "5" | "master" => Some(Self::Master),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum MaskTileId {
    MaskG1A,
    MaskG1B,
    MaskG1C,
    MaskG2A,
    MaskG2B,
    MaskG2C,
    MaskG3A,
    MaskG3B,
    MaskG3C,
    MaskG4A,
    MaskG4B,
    MaskG4C,
    MaskG5A,
    MaskG5B,
    MaskG5C,
}

impl MaskTileId {
    pub const ALL: [Self; 15] = [
        Self::MaskG1A,
        Self::MaskG1B,
        Self::MaskG1C,
        Self::MaskG2A,
        Self::MaskG2B,
        Self::MaskG2C,
        Self::MaskG3A,
        Self::MaskG3B,
        Self::MaskG3C,
        Self::MaskG4A,
        Self::MaskG4B,
        Self::MaskG4C,
        Self::MaskG5A,
        Self::MaskG5B,
        Self::MaskG5C,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MaskG1A => "mask_g1_a",
            Self::MaskG1B => "mask_g1_b",
            Self::MaskG1C => "mask_g1_c",
            Self::MaskG2A => "mask_g2_a",
            Self::MaskG2B => "mask_g2_b",
            Self::MaskG2C => "mask_g2_c",
            Self::MaskG3A => "mask_g3_a",
            Self::MaskG3B => "mask_g3_b",
            Self::MaskG3C => "mask_g3_c",
            Self::MaskG4A => "mask_g4_a",
            Self::MaskG4B => "mask_g4_b",
            Self::MaskG4C => "mask_g4_c",
            Self::MaskG5A => "mask_g5_a",
            Self::MaskG5B => "mask_g5_b",
            Self::MaskG5C => "mask_g5_c",
        }
    }

    pub const fn grade(self) -> Grade {
        match self {
            Self::MaskG1A | Self::MaskG1B | Self::MaskG1C => Grade::Plain,
            Self::MaskG2A | Self::MaskG2B | Self::MaskG2C => Grade::Trimmed,
            Self::MaskG3A | Self::MaskG3B | Self::MaskG3C => Grade::Gilded,
            Self::MaskG4A | Self::MaskG4B | Self::MaskG4C => Grade::Jeweled,
            Self::MaskG5A | Self::MaskG5B | Self::MaskG5C => Grade::Master,
        }
    }

    pub fn label(self) -> String {
        format!("{} {}", self.grade().label(), copy_label(self))
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "mask_g1_a" => Some(Self::MaskG1A),
            "mask_g1_b" => Some(Self::MaskG1B),
            "mask_g1_c" => Some(Self::MaskG1C),
            "mask_g2_a" => Some(Self::MaskG2A),
            "mask_g2_b" => Some(Self::MaskG2B),
            "mask_g2_c" => Some(Self::MaskG2C),
            "mask_g3_a" => Some(Self::MaskG3A),
            "mask_g3_b" => Some(Self::MaskG3B),
            "mask_g3_c" => Some(Self::MaskG3C),
            "mask_g4_a" => Some(Self::MaskG4A),
            "mask_g4_b" => Some(Self::MaskG4B),
            "mask_g4_c" => Some(Self::MaskG4C),
            "mask_g5_a" => Some(Self::MaskG5A),
            "mask_g5_b" => Some(Self::MaskG5B),
            "mask_g5_c" => Some(Self::MaskG5C),
            _ => None,
        }
    }
}

const fn copy_label(mask: MaskTileId) -> &'static str {
    match mask {
        MaskTileId::MaskG1A
        | MaskTileId::MaskG2A
        | MaskTileId::MaskG3A
        | MaskTileId::MaskG4A
        | MaskTileId::MaskG5A => "A",
        MaskTileId::MaskG1B
        | MaskTileId::MaskG2B
        | MaskTileId::MaskG3B
        | MaskTileId::MaskG4B
        | MaskTileId::MaskG5B => "B",
        MaskTileId::MaskG1C
        | MaskTileId::MaskG2C
        | MaskTileId::MaskG3C
        | MaskTileId::MaskG4C
        | MaskTileId::MaskG5C => "C",
    }
}

pub const fn canonical_masks() -> [MaskTileId; 15] {
    MaskTileId::ALL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seats_are_stable() {
        assert_eq!(
            MaskedClaimsSeat::from_index(0),
            Some(MaskedClaimsSeat::Seat0)
        );
        assert_eq!(
            MaskedClaimsSeat::from_index(1),
            Some(MaskedClaimsSeat::Seat1)
        );
        assert_eq!(MaskedClaimsSeat::from_index(2), None);
        assert_eq!(MaskedClaimsSeat::Seat0.index(), 0);
        assert_eq!(MaskedClaimsSeat::Seat1.other(), MaskedClaimsSeat::Seat0);
        assert_eq!(MaskedClaimsSeat::Seat1.as_str(), "seat_1");
        assert_eq!(
            MaskedClaimsSeat::parse("seat_0"),
            Some(MaskedClaimsSeat::Seat0)
        );
    }

    #[test]
    fn grades_are_stable() {
        assert_eq!(Grade::Plain.as_str(), "1");
        assert_eq!(Grade::Master.value(), 5);
        assert_eq!(Grade::Jeweled.label(), "Jeweled");
        assert_eq!(Grade::parse("gilded"), Some(Grade::Gilded));
        assert_eq!(Grade::parse("3"), Some(Grade::Gilded));
        assert_eq!(Grade::parse("6"), None);
    }

    #[test]
    fn canonical_mask_ids_match_rules_order() {
        let masks = canonical_masks();

        assert_eq!(masks.len(), STANDARD_MASK_COUNT as usize);
        assert_eq!(masks[0].as_str(), "mask_g1_a");
        assert_eq!(masks[14].as_str(), "mask_g5_c");
        assert_eq!(MaskTileId::MaskG4B.grade(), Grade::Jeweled);
        assert_eq!(MaskTileId::MaskG5C.label(), "Master C");
        assert_eq!(MaskTileId::parse("mask_g3_a"), Some(MaskTileId::MaskG3A));
        assert_eq!(MaskTileId::parse("bad"), None);
    }

    #[test]
    fn action_segments_are_neutral_and_stable() {
        assert_eq!(ACTION_CLAIM, "claim");
        assert_eq!(ACTION_RESPOND_ACCEPT, "respond/accept");
        assert_eq!(ACTION_RESPOND_CHALLENGE, "respond/challenge");
    }
}
