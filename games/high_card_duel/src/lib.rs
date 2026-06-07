//! `high_card_duel` official-game crate skeleton.

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
    active_commit_seat, actor_seat, commit_segment, legal_action_tree, parse_commit_segment,
    COMMIT_SEGMENT_PREFIX,
};
pub use effects::{
    cards_revealed_effect, commit_face_down_effect, deal_private_card_effect,
    hand_count_changed_effect, own_commit_confirmed_effect, private_diagnostic_effect,
    public_diagnostic_effect, public_effect, refill_started_effect, round_scored_effect,
    terminal_effect, HighCardDuelEffect,
};
pub use ids::{
    canonical_deck, CardId, HighCardDuelSeat, Sigil, GAME_ID, RULES_VERSION_LABEL,
    SHUFFLE_ALGORITHM, STANDARD_DECK_CARD_COUNT, STANDARD_HAND_SIZE, STANDARD_RANK_COUNT,
    STANDARD_ROUND_LIMIT, STANDARD_SIGILS_PER_RANK, VARIANT_ID,
};
pub use rules::{
    apply_action, commitment_conflict_diagnostic, invalid_private_card_diagnostic, lead_for_round,
    round_winner, stale_action_diagnostic, validate_command, wrong_phase_diagnostic,
    wrong_seat_diagnostic, ValidatedAction,
};
pub use setup::{next_bounded_index_unbiased, setup_match, shuffle_deck, SetupOptions};
pub use state::{HighCardDuelState, Phase, RevealedRound, Score, TerminalOutcome};
pub use ui::{
    card_accessibility_label, face_down_commitment_label, revealed_card_accessibility_label,
    ui_metadata, UiMetadata,
};
pub use variants::{Manifest, Variant, VariantCatalog};
pub use visibility::{
    project_view, CardView, CommitmentView, CommitmentViews, HandCountsView, PrivateView,
    PublicView, RevealedRoundView, TerminalView,
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

    #[test]
    fn static_data_parses_and_rejects_unknown_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.deck_card_count, STANDARD_DECK_CARD_COUNT);
        assert_eq!(manifest.hand_size, STANDARD_HAND_SIZE);
        assert_eq!(manifest.round_limit, STANDARD_ROUND_LIMIT);
        assert_eq!(variants.selected.id, VARIANT_ID);
        assert_eq!(variants.selected.seat_count, 2);
        assert!(Manifest::parse("game_id = \"high_card_duel\"\ntrigger = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"high_card_duel_standard\"\nselector = \"bad\"\n"
        )
        .is_err());
    }
}
