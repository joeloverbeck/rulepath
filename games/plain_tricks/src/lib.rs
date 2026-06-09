//! `plain_tricks` official-game crate skeleton for Plain Tricks.

pub mod ids;
pub mod variants;

pub use ids::{
    canonical_deck, PlainTricksSeat, TrickCardId, TrickRank, TrickSuit, ACTION_PLAY, GAME_ID,
    RULES_VERSION_LABEL, STANDARD_CARD_COUNT, STANDARD_HAND_SIZE, STANDARD_MAX_PLAYS,
    STANDARD_RANK_COUNT, STANDARD_ROUND_COUNT, STANDARD_SEAT_COUNT, STANDARD_SUIT_COUNT,
    STANDARD_TAIL_SIZE, STANDARD_TOTAL_TRICKS, STANDARD_TRICKS_PER_ROUND, VARIANT_ID,
};
pub use variants::{Fixture, Manifest, Variant, VariantCatalog};

pub fn load_manifest() -> Result<Manifest, String> {
    Manifest::parse(include_str!("../data/manifest.toml"))
}

pub fn load_variants() -> Result<VariantCatalog, String> {
    VariantCatalog::parse(include_str!("../data/variants.toml"))
}

pub fn load_standard_fixture() -> Result<Fixture, String> {
    Fixture::parse(include_str!(
        "../data/fixtures/plain_tricks_standard.fixture.json"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_data_parses_and_rejects_unknown_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");
        let fixture = load_standard_fixture().expect("fixture parses");

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.display_name, "Plain Tricks");
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.suit_count, STANDARD_SUIT_COUNT);
        assert_eq!(manifest.rank_count, STANDARD_RANK_COUNT);
        assert_eq!(manifest.card_count, STANDARD_CARD_COUNT);
        assert_eq!(manifest.hand_size, STANDARD_HAND_SIZE);
        assert_eq!(manifest.tail_size, STANDARD_TAIL_SIZE);
        assert_eq!(manifest.tricks_per_round, STANDARD_TRICKS_PER_ROUND);
        assert_eq!(variants.selected.id, VARIANT_ID);
        assert_eq!(variants.selected.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(fixture.game_id, GAME_ID);
        assert_eq!(fixture.variant, VARIANT_ID);
        assert_eq!(fixture.deck_order, TrickCardId::ALL);
        assert_eq!(fixture.hand_status, "hidden_by_setup");
        assert_eq!(fixture.tail_status, "internal_only");
        assert_eq!(fixture.terminal_outcome, "none");

        assert!(Manifest::parse("game_id = \"plain_tricks\"\ntrigger = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"plain_tricks_standard\"\nselector = \"bad\"\n"
        )
        .is_err());
        assert!(Fixture::parse("{\"game_id\":\"plain_tricks\",\"valid_if\":\"bad\"}").is_err());
    }
}
