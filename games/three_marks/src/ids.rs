#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ThreeMarksSeat {
    Seat0,
    Seat1,
}

impl ThreeMarksSeat {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CellId {
    R1C1,
    R1C2,
    R1C3,
    R2C1,
    R2C2,
    R2C3,
    R3C1,
    R3C2,
    R3C3,
}

impl CellId {
    pub const ALL: [Self; 9] = [
        Self::R1C1,
        Self::R1C2,
        Self::R1C3,
        Self::R2C1,
        Self::R2C2,
        Self::R2C3,
        Self::R3C1,
        Self::R3C2,
        Self::R3C3,
    ];

    pub fn index(self) -> usize {
        match self {
            Self::R1C1 => 0,
            Self::R1C2 => 1,
            Self::R1C3 => 2,
            Self::R2C1 => 3,
            Self::R2C2 => 4,
            Self::R2C3 => 5,
            Self::R3C1 => 6,
            Self::R3C2 => 7,
            Self::R3C3 => 8,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::R1C1 => "r1c1",
            Self::R1C2 => "r1c2",
            Self::R1C3 => "r1c3",
            Self::R2C1 => "r2c1",
            Self::R2C2 => "r2c2",
            Self::R2C3 => "r2c3",
            Self::R3C1 => "r3c1",
            Self::R3C2 => "r3c2",
            Self::R3C3 => "r3c3",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "r1c1" => Some(Self::R1C1),
            "r1c2" => Some(Self::R1C2),
            "r1c3" => Some(Self::R1C3),
            "r2c1" => Some(Self::R2C1),
            "r2c2" => Some(Self::R2C2),
            "r2c3" => Some(Self::R2C3),
            "r3c1" => Some(Self::R3C1),
            "r3c2" => Some(Self::R3C2),
            "r3c3" => Some(Self::R3C3),
            _ => None,
        }
    }
}

pub const GAME_ID: &str = "three_marks";
pub const RULES_VERSION_LABEL: &str = "three_marks-rules-v1";
pub const VARIANT_ID: &str = "three_marks_standard";
