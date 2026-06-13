use crate::ids::{CellId, ColumnFourSeat, ColumnId};

pub const SEAT_LABEL_AUDIT: &str =
    "Column Four is factionless; keep existing first-player/second-player token labels.";
pub const TURN_REPORT_AUDIT: &str =
    "Not adopted by ACTCONMAT-006; Column Four has no automated non-interactive burst surface in scope.";

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
        icon_id: "column_four",
        theme_key: "column-four",
        accent_token: "--game-accent",
        secondary_accent_token: "--game-accent-2",
        shape_token: "stacked-column-tokens",
        accessibility_label: "Column Four abstract vertical token stack",
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PieceTokenMetadata {
    pub seat: ColumnFourSeat,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnControlMetadata {
    pub column: ColumnId,
    pub label: String,
    pub accessibility_label: String,
}

pub fn piece_token(seat: ColumnFourSeat) -> PieceTokenMetadata {
    match seat {
        ColumnFourSeat::Seat0 => PieceTokenMetadata {
            seat,
            token_key: "first_piece_ring",
            shape_label: "ring piece",
            color_role: "first-player",
        },
        ColumnFourSeat::Seat1 => PieceTokenMetadata {
            seat,
            token_key: "second_piece_star",
            shape_label: "star piece",
            color_role: "second-player",
        },
    }
}

pub fn cell_layout(cell: CellId) -> CellLayoutMetadata {
    CellLayoutMetadata {
        cell,
        row: (cell.row.index() + 1) as u8,
        column: (cell.column.index() + 1) as u8,
    }
}

pub fn column_control(column: ColumnId) -> ColumnControlMetadata {
    let number = column.index() + 1;
    ColumnControlMetadata {
        column,
        label: format!("Column {number}"),
        accessibility_label: format!("Drop a piece in column {number}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_controls_use_neutral_labels_and_accessibility_names() {
        let first = column_control(ColumnId::C1);
        let last = column_control(ColumnId::C7);

        assert_eq!(first.label, "Column 1");
        assert_eq!(first.accessibility_label, "Drop a piece in column 1");
        assert_eq!(last.label, "Column 7");
        assert_eq!(last.accessibility_label, "Drop a piece in column 7");
    }
}
