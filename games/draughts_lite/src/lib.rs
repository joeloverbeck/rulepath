//! `draughts_lite` official-game crate skeleton.

pub mod ids;
pub mod rules;
pub mod setup;
pub mod state;
pub mod variants;

pub use ids::{
    is_playable_cell, DraughtsLiteSeat, PieceId, BOARD_COLS, BOARD_ROWS, GAME_ID,
    RULES_VERSION_LABEL, STANDARD_PIECES_PER_SEAT, TOTAL_STANDARD_PIECES, VARIANT_ID,
};
pub use rules::{
    has_legal_move, legal_moves, legal_moves_for, terminal_outcome_for_active_player,
    CaptureDetail, Diagonal, LegalMove, MoveKind, MoveStep,
};
pub use setup::{setup_match, SetupOptions};
pub use state::{
    CellOccupancy, DraughtsLiteSnapshot, DraughtsLiteState, Piece, PieceKind, TerminalOutcome,
};
pub use variants::{Manifest, Variant, VariantCatalog};

pub fn load_manifest() -> Result<Manifest, String> {
    Manifest::parse(include_str!("../data/manifest.toml"))
}

pub fn load_variants() -> Result<VariantCatalog, String> {
    VariantCatalog::parse(include_str!("../data/variants.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_data_parses_and_rejects_unknown_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.board_columns, BOARD_COLS);
        assert_eq!(manifest.board_rows, BOARD_ROWS);
        assert_eq!(variants.selected.id, VARIANT_ID);
        assert_eq!(variants.selected.seat_count, 2);
        assert!(Manifest::parse("game_id = \"draughts_lite\"\nextra = \"nope\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"draughts_lite_standard\"\ntrigger = \"bad\"\n"
        )
        .is_err());
    }
}
