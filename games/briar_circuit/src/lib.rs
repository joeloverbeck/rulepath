//! `briar_circuit` official-game crate scaffold for Briar Circuit.

pub mod actions;
pub mod bots;
pub mod cards;
pub mod effects;
pub mod ids;
pub mod replay_support;
pub mod rules;
pub mod scoring;
pub mod setup;
pub mod state;
pub mod ui;
pub mod variants;
pub mod visibility;

pub use actions::{
    apply_pass_action, apply_play_action, parse_pass_action_path, parse_play_action_path,
    validate_pass_command, validate_play_command, PassAction, PassActionResult, PlayAction,
};
pub use cards::{canonical_deck, Card, CardId, Deck, Rank, Suit};
pub use effects::{BriarCircuitEffect, PassCommitmentStatus};
pub use ids::{
    canonical_seat_ids, BriarCircuitSeat, ACTION_PASS, ACTION_PASS_CONFIRM, ACTION_PASS_SELECT,
    ACTION_PASS_UNSELECT, ACTION_PLAY, GAME_ID, RULES_VERSION_LABEL, STANDARD_CARD_COUNT,
    STANDARD_DEFAULT_SEATS, STANDARD_HAND_SIZE, STANDARD_MAX_SEATS, STANDARD_MIN_SEATS,
    STANDARD_PASS_SIZE, STANDARD_RANK_COUNT, STANDARD_SEAT_COUNT, STANDARD_SUIT_COUNT,
    STANDARD_TRICKS_PER_HAND, VARIANT_ID,
};
pub use rules::{
    apply_play_card, is_point_card, legal_cards_for_playing_state, legal_play_cards,
    play_legality_reason, trick_winner, validate_play_card, PlayActionResult, PlayLegalityReason,
};
pub use setup::{setup_match, SetupOptions};
pub use state::{
    BriarCircuitState, CapturedTrick, CurrentTrick, HandScoreBreakdown, PassDirection, PassState,
    Phase, PlayingTrickState, TerminalOutcome, TrickPlay,
};
pub use variants::{Manifest, Variant, VariantCatalog};
pub use visibility::{project_pass_view, PassView};

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
    fn static_data_parses_and_rejects_unknown_or_behavior_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.display_name, "Briar Circuit");
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.min_seats, STANDARD_MIN_SEATS);
        assert_eq!(manifest.default_seats, STANDARD_DEFAULT_SEATS);
        assert_eq!(manifest.max_seats, STANDARD_MAX_SEATS);
        assert_eq!(manifest.card_count, STANDARD_CARD_COUNT);
        assert_eq!(manifest.hand_size, STANDARD_HAND_SIZE);
        assert_eq!(variants.selected, Variant::briar_circuit_standard());

        assert!(Manifest::parse("game_id = \"briar_circuit\"\ntrigger = \"bad\"\n").is_err());
        assert!(Manifest::parse("game_id = \"briar_circuit\"\nunknown = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"briar_circuit_standard\"\nvalid_if = \"bad\"\n"
        )
        .is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"briar_circuit_standard\"\nfollow_suit_formula = \"bad\"\n"
        )
        .is_err());
    }
}
