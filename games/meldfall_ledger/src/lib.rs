//! Meldfall Ledger game crate.
//!
//! This crate keeps all Five Hundred Rummy-family nouns local to the game
//! module. The shared engine sees only generic Rulepath contracts.

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

pub use ids::{
    canonical_seat_ids, hand_size_for_seats, seat_id_for_index, supported_seat_count,
    DATA_VERSION_LABEL, GAME_ID, RULES_VERSION_LABEL, STANDARD_CARD_COUNT, STANDARD_DEFAULT_SEATS,
    STANDARD_MAX_SEATS, STANDARD_MIN_SEATS, STANDARD_MULTI_SEAT_HAND_SIZE, STANDARD_RANK_COUNT,
    STANDARD_SUIT_COUNT, STANDARD_TARGET_SCORE, STANDARD_TWO_SEAT_HAND_SIZE, VARIANT_ID,
};
pub use variants::{load_manifest, load_variants, Manifest, Variant, VariantCatalog};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_data_parses_and_rejects_unknown_or_behavior_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.display_name, "Meldfall Ledger");
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.data_version_label, DATA_VERSION_LABEL);
        assert_eq!(manifest.min_seats, STANDARD_MIN_SEATS);
        assert_eq!(manifest.default_seats, STANDARD_DEFAULT_SEATS);
        assert_eq!(manifest.max_seats, STANDARD_MAX_SEATS);
        assert_eq!(manifest.target_score, STANDARD_TARGET_SCORE);
        assert_eq!(manifest.two_seat_hand_size, STANDARD_TWO_SEAT_HAND_SIZE);
        assert_eq!(manifest.multi_seat_hand_size, STANDARD_MULTI_SEAT_HAND_SIZE);
        assert_eq!(manifest.card_count, STANDARD_CARD_COUNT);
        assert_eq!(variants.selected, Variant::classic_500_single_deck_v1());

        assert!(Manifest::parse("game_id = \"meldfall_ledger\"\ntrigger = \"bad\"\n").is_err());
        assert!(Manifest::parse("game_id = \"meldfall_ledger\"\nunknown = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"classic_500_single_deck_v1\"\nformula = \"bad\"\n"
        )
        .is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"classic_500_single_deck_v1\"\nscore_formula = \"bad\"\n"
        )
        .is_err());
    }
}
