use crate::ids::{CellId, DirectionalFlipSeat};

pub const SEAT_LABEL_AUDIT: &str =
    "Directional Flip is factionless; keep existing first-player/second-player token labels.";
pub const TURN_REPORT_AUDIT: &str =
    "Not adopted by ACTCONMAT-006; Directional Flip has no automated non-interactive burst surface in scope.";

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
        icon_id: "directional_flip",
        theme_key: "directional-flip",
        accent_token: "--game-accent",
        secondary_accent_token: "--game-accent-2",
        shape_token: "opposing-rotation-arrows",
        accessibility_label: "Directional Flip abstract opposing rotation arrows",
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscTokenMetadata {
    pub seat: DirectionalFlipSeat,
    pub token_key: &'static str,
    pub shape_label: &'static str,
    pub pattern_label: &'static str,
    pub color_role: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CellLayoutMetadata {
    pub cell: CellId,
    pub row: u8,
    pub column: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegalCellControlMetadata {
    pub cell: CellId,
    pub label: String,
    pub accessibility_label: String,
}

pub fn disc_token(seat: DirectionalFlipSeat) -> DiscTokenMetadata {
    match seat {
        DirectionalFlipSeat::Seat0 => DiscTokenMetadata {
            seat,
            token_key: "first_disc_ring",
            shape_label: "ring disc",
            pattern_label: "single-ring pattern",
            color_role: "first-player",
        },
        DirectionalFlipSeat::Seat1 => DiscTokenMetadata {
            seat,
            token_key: "second_disc_cross",
            shape_label: "cross disc",
            pattern_label: "cross-hatch pattern",
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

pub fn legal_cell_control(cell: CellId, flip_count: usize) -> LegalCellControlMetadata {
    let cell_id = cell.as_string();
    LegalCellControlMetadata {
        cell,
        label: format!("Place at {cell_id}"),
        accessibility_label: format!(
            "Place at {cell_id}, flipping {flip_count} {}",
            if flip_count == 1 { "disc" } else { "discs" }
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ColumnId, RowId};

    #[test]
    fn disc_tokens_use_non_color_only_metadata() {
        let first = disc_token(DirectionalFlipSeat::Seat0);
        let second = disc_token(DirectionalFlipSeat::Seat1);

        assert_eq!(first.shape_label, "ring disc");
        assert_eq!(first.pattern_label, "single-ring pattern");
        assert_eq!(second.shape_label, "cross disc");
        assert_eq!(second.pattern_label, "cross-hatch pattern");
    }

    #[test]
    fn cell_controls_use_neutral_labels_and_accessibility_names() {
        let cell = CellId::new(RowId::R3, ColumnId::C4);
        let control = legal_cell_control(cell, 2);

        assert_eq!(control.label, "Place at r3c4");
        assert_eq!(
            control.accessibility_label,
            "Place at r3c4, flipping 2 discs"
        );
    }
}
