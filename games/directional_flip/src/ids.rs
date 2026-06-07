#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DirectionalFlipSeat {
    Seat0,
    Seat1,
}

impl DirectionalFlipSeat {
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
pub enum RowId {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl RowId {
    pub const ALL: [Self; 8] = [
        Self::R1,
        Self::R2,
        Self::R3,
        Self::R4,
        Self::R5,
        Self::R6,
        Self::R7,
        Self::R8,
    ];

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::R1),
            1 => Some(Self::R2),
            2 => Some(Self::R3),
            3 => Some(Self::R4),
            4 => Some(Self::R5),
            5 => Some(Self::R6),
            6 => Some(Self::R7),
            7 => Some(Self::R8),
            _ => None,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::R1 => 0,
            Self::R2 => 1,
            Self::R3 => 2,
            Self::R4 => 3,
            Self::R5 => 4,
            Self::R6 => 5,
            Self::R7 => 6,
            Self::R8 => 7,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::R1 => "r1",
            Self::R2 => "r2",
            Self::R3 => "r3",
            Self::R4 => "r4",
            Self::R5 => "r5",
            Self::R6 => "r6",
            Self::R7 => "r7",
            Self::R8 => "r8",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "r1" => Some(Self::R1),
            "r2" => Some(Self::R2),
            "r3" => Some(Self::R3),
            "r4" => Some(Self::R4),
            "r5" => Some(Self::R5),
            "r6" => Some(Self::R6),
            "r7" => Some(Self::R7),
            "r8" => Some(Self::R8),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ColumnId {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
}

impl ColumnId {
    pub const ALL: [Self; 8] = [
        Self::C1,
        Self::C2,
        Self::C3,
        Self::C4,
        Self::C5,
        Self::C6,
        Self::C7,
        Self::C8,
    ];

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::C1),
            1 => Some(Self::C2),
            2 => Some(Self::C3),
            3 => Some(Self::C4),
            4 => Some(Self::C5),
            5 => Some(Self::C6),
            6 => Some(Self::C7),
            7 => Some(Self::C8),
            _ => None,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::C1 => 0,
            Self::C2 => 1,
            Self::C3 => 2,
            Self::C4 => 3,
            Self::C5 => 4,
            Self::C6 => 5,
            Self::C7 => 6,
            Self::C8 => 7,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::C1 => "c1",
            Self::C2 => "c2",
            Self::C3 => "c3",
            Self::C4 => "c4",
            Self::C5 => "c5",
            Self::C6 => "c6",
            Self::C7 => "c7",
            Self::C8 => "c8",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "c1" => Some(Self::C1),
            "c2" => Some(Self::C2),
            "c3" => Some(Self::C3),
            "c4" => Some(Self::C4),
            "c5" => Some(Self::C5),
            "c6" => Some(Self::C6),
            "c7" => Some(Self::C7),
            "c8" => Some(Self::C8),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CellId {
    pub row: RowId,
    pub column: ColumnId,
}

impl CellId {
    pub const ALL: [Self; 64] = [
        Self::new(RowId::R1, ColumnId::C1),
        Self::new(RowId::R1, ColumnId::C2),
        Self::new(RowId::R1, ColumnId::C3),
        Self::new(RowId::R1, ColumnId::C4),
        Self::new(RowId::R1, ColumnId::C5),
        Self::new(RowId::R1, ColumnId::C6),
        Self::new(RowId::R1, ColumnId::C7),
        Self::new(RowId::R1, ColumnId::C8),
        Self::new(RowId::R2, ColumnId::C1),
        Self::new(RowId::R2, ColumnId::C2),
        Self::new(RowId::R2, ColumnId::C3),
        Self::new(RowId::R2, ColumnId::C4),
        Self::new(RowId::R2, ColumnId::C5),
        Self::new(RowId::R2, ColumnId::C6),
        Self::new(RowId::R2, ColumnId::C7),
        Self::new(RowId::R2, ColumnId::C8),
        Self::new(RowId::R3, ColumnId::C1),
        Self::new(RowId::R3, ColumnId::C2),
        Self::new(RowId::R3, ColumnId::C3),
        Self::new(RowId::R3, ColumnId::C4),
        Self::new(RowId::R3, ColumnId::C5),
        Self::new(RowId::R3, ColumnId::C6),
        Self::new(RowId::R3, ColumnId::C7),
        Self::new(RowId::R3, ColumnId::C8),
        Self::new(RowId::R4, ColumnId::C1),
        Self::new(RowId::R4, ColumnId::C2),
        Self::new(RowId::R4, ColumnId::C3),
        Self::new(RowId::R4, ColumnId::C4),
        Self::new(RowId::R4, ColumnId::C5),
        Self::new(RowId::R4, ColumnId::C6),
        Self::new(RowId::R4, ColumnId::C7),
        Self::new(RowId::R4, ColumnId::C8),
        Self::new(RowId::R5, ColumnId::C1),
        Self::new(RowId::R5, ColumnId::C2),
        Self::new(RowId::R5, ColumnId::C3),
        Self::new(RowId::R5, ColumnId::C4),
        Self::new(RowId::R5, ColumnId::C5),
        Self::new(RowId::R5, ColumnId::C6),
        Self::new(RowId::R5, ColumnId::C7),
        Self::new(RowId::R5, ColumnId::C8),
        Self::new(RowId::R6, ColumnId::C1),
        Self::new(RowId::R6, ColumnId::C2),
        Self::new(RowId::R6, ColumnId::C3),
        Self::new(RowId::R6, ColumnId::C4),
        Self::new(RowId::R6, ColumnId::C5),
        Self::new(RowId::R6, ColumnId::C6),
        Self::new(RowId::R6, ColumnId::C7),
        Self::new(RowId::R6, ColumnId::C8),
        Self::new(RowId::R7, ColumnId::C1),
        Self::new(RowId::R7, ColumnId::C2),
        Self::new(RowId::R7, ColumnId::C3),
        Self::new(RowId::R7, ColumnId::C4),
        Self::new(RowId::R7, ColumnId::C5),
        Self::new(RowId::R7, ColumnId::C6),
        Self::new(RowId::R7, ColumnId::C7),
        Self::new(RowId::R7, ColumnId::C8),
        Self::new(RowId::R8, ColumnId::C1),
        Self::new(RowId::R8, ColumnId::C2),
        Self::new(RowId::R8, ColumnId::C3),
        Self::new(RowId::R8, ColumnId::C4),
        Self::new(RowId::R8, ColumnId::C5),
        Self::new(RowId::R8, ColumnId::C6),
        Self::new(RowId::R8, ColumnId::C7),
        Self::new(RowId::R8, ColumnId::C8),
    ];

    pub const fn new(row: RowId, column: ColumnId) -> Self {
        Self { row, column }
    }

    pub fn index(self) -> usize {
        self.row.index() * ColumnId::ALL.len() + self.column.index()
    }

    pub fn as_string(self) -> String {
        format!("{}{}", self.row.as_str(), self.column.as_str())
    }

    pub fn parse(value: &str) -> Option<Self> {
        if value.len() != 4 {
            return None;
        }

        let row = RowId::parse(&value[0..2])?;
        let column = ColumnId::parse(&value[2..4])?;
        Some(Self { row, column })
    }
}

pub const GAME_ID: &str = "directional_flip";
pub const RULES_VERSION_LABEL: &str = "directional_flip-rules-v1";
pub const VARIANT_ID: &str = "directional_flip_standard";
