//! `column_four` public-polish crate.

pub mod actions;
pub mod effects;
pub mod ids;
pub mod replay_support;
pub mod rules;
pub mod setup;
pub mod state;
pub mod ui;
pub mod variants;
pub mod visibility;

pub use actions::{actor_seat, legal_action_tree};
pub use effects::{bot_chose_action_effect, public_effect, ColumnFourEffect};
pub use ids::{CellId, ColumnFourSeat, ColumnId, RowId};
pub use rules::{apply_action, validate_command, ValidatedAction};
pub use setup::{setup_match, SetupOptions};
pub use state::{CellOccupancy, ColumnFourSnapshot, ColumnFourState, TerminalOutcome, WinningLine};
pub use ui::{cell_layout, column_control, piece_token};
pub use variants::{Manifest, Variant, VariantCatalog};
pub use visibility::{
    project_view, CellView, ColumnSummaryView, LegalColumnTargetView, PublicView, TerminalView,
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

        assert_eq!(manifest.game_id, "column_four");
        assert_eq!(manifest.rules_version_label, "column_four-rules-v1");
        assert_eq!(manifest.board_columns, 7);
        assert_eq!(manifest.board_rows, 6);
        assert_eq!(variants.selected.id, "column_four_standard");
        assert_eq!(variants.selected.seat_count, 2);
        assert!(Manifest::parse("game_id = \"column_four\"\nextra = \"nope\"\n").is_err());
        assert!(
            VariantCatalog::parse("variant_id = \"column_four_standard\"\nwhen = \"bad\"\n")
                .is_err()
        );
    }

    #[test]
    fn setup_wires_initial_state() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let state = setup_match(engine_core::Seed(1), &seats, &SetupOptions::default())
            .expect("setup succeeds");

        assert_eq!(state.active_seat, ColumnFourSeat::Seat0);
        assert_eq!(state.seats[0], seats[0]);
        assert_eq!(state.seats[1], seats[1]);
        assert_eq!(state.ply_count, 0);
        assert_eq!(state.terminal_outcome, None);
        assert_eq!(state.freshness_token, FreshnessToken(0));
        assert!(state.cells.iter().all(|cell| cell.is_empty()));
    }
}
