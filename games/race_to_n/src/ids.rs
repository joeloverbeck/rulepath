use engine_core::SeatId;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RaceSeat {
    Seat0,
    Seat1,
}

impl RaceSeat {
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
        let index = SeatId::parse_canonical(value)
            .ok()?
            .canonical_zero_based_index()
            .ok()?;
        Self::from_index(index as usize)
    }
}
