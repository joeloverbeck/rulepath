use crate::ids::{CellId, ThreeMarksSeat};

pub const SEAT_LABEL_AUDIT: &str =
    "Three Marks is factionless; keep existing first-player/second-player mark labels.";
pub const TURN_REPORT_AUDIT: &str =
    "Not adopted by ACTCONMAT-006; Three Marks has no automated non-interactive burst surface in scope.";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CatalogThemeMetadata {
    pub icon_id: &'static str,
    pub theme_key: &'static str,
    pub accent_token: &'static str,
    pub secondary_accent_token: &'static str,
    pub shape_token: &'static str,
    pub accessibility_label: &'static str,
}

pub fn catalog_theme() -> CatalogThemeMetadata {
    CatalogThemeMetadata {
        icon_id: "three_marks",
        theme_key: "three-marks",
        accent_token: "--game-accent",
        secondary_accent_token: "--game-accent-2",
        shape_token: "loop-and-spark-grid",
        accessibility_label: "Three Marks abstract grid with loop and spark marks",
    }
}

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
