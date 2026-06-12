use crate::ids::{CellId, ThreeMarksSeat};

pub const SEAT_LABEL_AUDIT: &str =
    "Three Marks is factionless; keep existing first-player/second-player mark labels.";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarkTokenMetadata {
    pub seat: ThreeMarksSeat,
    pub token_key: &'static str,
    pub shape_label: &'static str,
    pub color_role: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CellLayoutMetadata {
    pub cell: CellId,
    pub row: u8,
    pub column: u8,
}

pub fn mark_token(seat: ThreeMarksSeat) -> MarkTokenMetadata {
    match seat {
        ThreeMarksSeat::Seat0 => MarkTokenMetadata {
            seat,
            token_key: "first_mark_loop",
            shape_label: "loop mark",
            color_role: "first-player",
        },
        ThreeMarksSeat::Seat1 => MarkTokenMetadata {
            seat,
            token_key: "second_mark_spark",
            shape_label: "spark mark",
            color_role: "second-player",
        },
    }
}

pub fn cell_layout(cell: CellId) -> CellLayoutMetadata {
    let index = cell.index() as u8;
    CellLayoutMetadata {
        cell,
        row: (index / 3) + 1,
        column: (index % 3) + 1,
    }
}
