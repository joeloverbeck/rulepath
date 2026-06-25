//! `blackglass_pact` official-game crate scaffold for Blackglass Pact.

pub mod bidding;
pub mod bots;
pub mod cards;
pub mod effects;
pub mod ids;
pub mod partnerships;
pub mod replay_support;
pub mod rules;
pub mod scoring;
pub mod setup;
pub mod state;
pub mod ui;
pub mod variants;
pub mod visibility;

pub use cards::{canonical_deck, Card, CardId, Deck, Rank, Suit};
pub use ids::{
    canonical_seat_ids, BlackglassSeat, TeamId, DATA_VERSION_LABEL, GAME_ID, RULES_VERSION_LABEL,
    STANDARD_CARD_COUNT, STANDARD_DEFAULT_SEATS, STANDARD_HAND_SIZE, STANDARD_MAX_SEATS,
    STANDARD_MIN_SEATS, STANDARD_RANK_COUNT, STANDARD_SEAT_COUNT, STANDARD_SUIT_COUNT,
    STANDARD_TRICKS_PER_HAND, VARIANT_ID,
};
pub use partnerships::{
    canonical_team_ids, members_for_team, partner_for, team_for_seat, team_id_for_index,
};
pub use setup::{setup_match, validate_standard_seat_count, SetupOptions};
pub use state::{Bid, BlackglassPactState, BlindNilChoice, Phase, PlayedCard};
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
    fn static_data_parses_and_rejects_unknown_or_behavior_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.display_name, "Blackglass Pact");
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.min_seats, STANDARD_MIN_SEATS);
        assert_eq!(manifest.default_seats, STANDARD_DEFAULT_SEATS);
        assert_eq!(manifest.max_seats, STANDARD_MAX_SEATS);
        assert_eq!(manifest.card_count, STANDARD_CARD_COUNT);
        assert_eq!(manifest.hand_size, STANDARD_HAND_SIZE);
        assert_eq!(variants.selected, Variant::blackglass_pact_standard());

        assert!(Manifest::parse("game_id = \"blackglass_pact\"\ntrigger = \"bad\"\n").is_err());
        assert!(Manifest::parse("game_id = \"blackglass_pact\"\nunknown = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"blackglass_pact_standard\"\nformula = \"bad\"\n"
        )
        .is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"blackglass_pact_standard\"\nscore_formula = \"bad\"\n"
        )
        .is_err());
    }
}
