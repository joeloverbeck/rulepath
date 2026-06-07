//! `draughts_lite` official-game crate skeleton.

pub mod actions;
pub mod bots;
pub mod effects;
pub mod ids;
pub mod replay_support;
pub mod rules;
pub mod setup;
pub mod state;
pub mod ui;
pub mod variants;
pub mod visibility;

pub use actions::{
    actor_seat, legal_action_tree, FROM_SEGMENT_PREFIX, JUMP_SEGMENT_PREFIX, TO_SEGMENT_PREFIX,
};
pub use bots::{
    BotDecision, DraughtsLiteLevel1Bot, DraughtsLiteRandomBot, LEVEL1_POLICY_ID, RANDOM_POLICY_ID,
};
pub use effects::{
    bot_chose_action_effect, display_anchor, forced_capture_available_effect,
    invalid_command_effect, public_effect, terminal_win_effect, DraughtsLiteEffect,
    TerminalWinReason,
};
pub use ids::{
    is_playable_cell, DraughtsLiteSeat, PieceId, BOARD_COLS, BOARD_ROWS, GAME_ID,
    RULES_VERSION_LABEL, STANDARD_PIECES_PER_SEAT, TOTAL_STANDARD_PIECES, VARIANT_ID,
};
pub use replay_support::{
    action_tree_hash, actor_for_state, command_for_state, default_seats, diagnostic_hash,
    effect_hash, effect_stable_string, hashes_for_state, replay_commands, replay_from_state,
    replay_invalid, DraughtsLiteReplayJson, ReplayCommandPath, ReplayHashes, ReplayStepProjection,
};
pub use rules::{
    apply_action, has_legal_move, legal_moves, legal_moves_for, terminal_outcome_for_active_player,
    validate_command, CaptureDetail, Diagonal, LegalMove, MoveKind, MoveStep, ValidatedAction,
};
pub use setup::{setup_match, SetupOptions};
pub use state::{
    CellOccupancy, DraughtsLiteSnapshot, DraughtsLiteState, Piece, PieceKind, TerminalOutcome,
};
pub use ui::{
    board_presentation, cell_layout, piece_label, piece_token, BoardPresentationMetadata,
    CellLayoutMetadata, PieceLabelMetadata, PieceTokenMetadata,
};
pub use variants::{Manifest, Variant, VariantCatalog};
pub use visibility::{project_view, CellView, PrivateView, PublicView, TerminalView, UiMetadata};

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
