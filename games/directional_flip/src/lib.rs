//! `directional_flip` official-game crate skeleton.

pub mod actions;
pub mod effects;
pub mod ids;
pub mod rules;
pub mod setup;
pub mod state;
pub mod ui;
pub mod variants;
pub mod visibility;

pub use actions::{
    legal_action_tree, placement_preview, DirectionPreview, PlacementPreview, PASS_REASON_NO_MOVES,
};
pub use effects::{
    bot_chose_action_effect, display_to_anchor, public_effect, DirectionalFlipEffect, FlipEntry,
    TerminalReason,
};
pub use ids::{CellId, ColumnId, DirectionalFlipSeat, RowId};
pub use rules::{
    apply_action, disc_counts, legal_placements, placement_flips, validate_command, Direction,
    FlipRun, ForcedPass, Placement, Score, ValidatedAction,
};
pub use setup::{setup_match, SetupOptions};
pub use state::{CellOccupancy, DirectionalFlipSnapshot, DirectionalFlipState, TerminalOutcome};
pub use ui::{cell_layout, disc_token, legal_cell_control, DiscTokenMetadata};
pub use variants::{Manifest, Variant, VariantCatalog};
pub use visibility::{
    project_view, CellView, LegalTargetView, PlacementPreviewView, PrivateView, PublicView,
    ScoreView, TerminalView, UiMetadata,
};

pub fn load_manifest() -> Result<Manifest, String> {
    Manifest::parse(include_str!("../data/manifest.toml"))
}

pub fn load_variants() -> Result<VariantCatalog, String> {
    VariantCatalog::parse(include_str!("../data/variants.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::{FreshnessToken, SeatId};

    #[test]
    fn static_data_parses_and_rejects_unknown_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");

        assert_eq!(manifest.game_id, "directional_flip");
        assert_eq!(manifest.rules_version_label, "directional_flip-rules-v1");
        assert_eq!(manifest.board_columns, 8);
        assert_eq!(manifest.board_rows, 8);
        assert_eq!(variants.selected.id, "directional_flip_standard");
        assert_eq!(variants.selected.seat_count, 2);
        assert!(Manifest::parse("game_id = \"directional_flip\"\nextra = \"nope\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"directional_flip_standard\"\nwhen = \"bad\"\n"
        )
        .is_err());
    }

    #[test]
    fn setup_wires_standard_center_position() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let state = setup_match(engine_core::Seed(1), &seats, &SetupOptions::default())
            .expect("setup succeeds");

        assert_eq!(state.active_seat, DirectionalFlipSeat::Seat0);
        assert_eq!(state.seats[0], seats[0]);
        assert_eq!(state.seats[1], seats[1]);
        assert_eq!(state.ply_count, 0);
        assert_eq!(state.consecutive_forced_passes, 0);
        assert_eq!(state.terminal_outcome, None);
        assert_eq!(state.freshness_token, FreshnessToken(0));
        assert_eq!(
            state.occupancy(CellId::new(RowId::R4, ColumnId::C5)),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat0)
        );
        assert_eq!(
            state.occupancy(CellId::new(RowId::R5, ColumnId::C4)),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat0)
        );
        assert_eq!(
            state.occupancy(CellId::new(RowId::R4, ColumnId::C4)),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat1)
        );
        assert_eq!(
            state.occupancy(CellId::new(RowId::R5, ColumnId::C5)),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat1)
        );
        assert_eq!(
            state.cells.iter().filter(|cell| !cell.is_empty()).count(),
            4
        );
    }
}
