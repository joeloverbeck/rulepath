#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ColumnFourSeat {
    Seat0,
    Seat1,
}

impl ColumnFourSeat {
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
pub enum ColumnId {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

impl ColumnId {
    pub const ALL: [Self; 7] = [
        Self::C1,
        Self::C2,
        Self::C3,
        Self::C4,
        Self::C5,
        Self::C6,
        Self::C7,
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
}

impl RowId {
    pub const ALL: [Self; 6] = [Self::R1, Self::R2, Self::R3, Self::R4, Self::R5, Self::R6];

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::R1),
            1 => Some(Self::R2),
            2 => Some(Self::R3),
            3 => Some(Self::R4),
            4 => Some(Self::R5),
            5 => Some(Self::R6),
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
    pub const ALL: [Self; 42] = [
        Self::new(RowId::R1, ColumnId::C1),
        Self::new(RowId::R1, ColumnId::C2),
        Self::new(RowId::R1, ColumnId::C3),
        Self::new(RowId::R1, ColumnId::C4),
        Self::new(RowId::R1, ColumnId::C5),
        Self::new(RowId::R1, ColumnId::C6),
        Self::new(RowId::R1, ColumnId::C7),
        Self::new(RowId::R2, ColumnId::C1),
        Self::new(RowId::R2, ColumnId::C2),
        Self::new(RowId::R2, ColumnId::C3),
        Self::new(RowId::R2, ColumnId::C4),
        Self::new(RowId::R2, ColumnId::C5),
        Self::new(RowId::R2, ColumnId::C6),
        Self::new(RowId::R2, ColumnId::C7),
        Self::new(RowId::R3, ColumnId::C1),
        Self::new(RowId::R3, ColumnId::C2),
        Self::new(RowId::R3, ColumnId::C3),
        Self::new(RowId::R3, ColumnId::C4),
        Self::new(RowId::R3, ColumnId::C5),
        Self::new(RowId::R3, ColumnId::C6),
        Self::new(RowId::R3, ColumnId::C7),
        Self::new(RowId::R4, ColumnId::C1),
        Self::new(RowId::R4, ColumnId::C2),
        Self::new(RowId::R4, ColumnId::C3),
        Self::new(RowId::R4, ColumnId::C4),
        Self::new(RowId::R4, ColumnId::C5),
        Self::new(RowId::R4, ColumnId::C6),
        Self::new(RowId::R4, ColumnId::C7),
        Self::new(RowId::R5, ColumnId::C1),
        Self::new(RowId::R5, ColumnId::C2),
        Self::new(RowId::R5, ColumnId::C3),
        Self::new(RowId::R5, ColumnId::C4),
        Self::new(RowId::R5, ColumnId::C5),
        Self::new(RowId::R5, ColumnId::C6),
        Self::new(RowId::R5, ColumnId::C7),
        Self::new(RowId::R6, ColumnId::C1),
        Self::new(RowId::R6, ColumnId::C2),
        Self::new(RowId::R6, ColumnId::C3),
        Self::new(RowId::R6, ColumnId::C4),
        Self::new(RowId::R6, ColumnId::C5),
        Self::new(RowId::R6, ColumnId::C6),
        Self::new(RowId::R6, ColumnId::C7),
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

pub const GAME_ID: &str = "column_four";
pub const RULES_VERSION_LABEL: &str = "column_four-rules-v1";
pub const VARIANT_ID: &str = "column_four_standard";
