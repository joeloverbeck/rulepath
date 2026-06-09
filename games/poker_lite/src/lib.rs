//! `poker_lite` official-game crate skeleton for Crest Ledger.

pub mod ids;
pub mod variants;

pub use ids::{
    canonical_deck, CrestCardId, CrestRank, CrestRankCopy, PokerLiteSeat, ACTION_HOLD, ACTION_LIFT,
    ACTION_MATCH, ACTION_PRESS, ACTION_YIELD, GAME_ID, RULES_VERSION_LABEL, STANDARD_CARD_COUNT,
    STANDARD_COPY_COUNT, STANDARD_MAX_CONTRIBUTION, STANDARD_RANK_COUNT, STANDARD_ROUND_COUNT,
    STANDARD_ROUND_UNITS, STANDARD_SEAT_COUNT, VARIANT_ID,
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
        "../data/fixtures/poker_lite_standard.fixture.json"
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
        assert_eq!(manifest.display_name, "Crest Ledger");
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.round_count, STANDARD_ROUND_COUNT);
        assert_eq!(manifest.card_count, STANDARD_CARD_COUNT);
        assert_eq!(
            manifest.max_contribution_per_seat,
            STANDARD_MAX_CONTRIBUTION
        );
        assert_eq!(variants.selected.id, VARIANT_ID);
        assert_eq!(variants.selected.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(variants.selected.round_units, "1,2");
        assert_eq!(fixture.game_id, GAME_ID);
        assert_eq!(fixture.variant, VARIANT_ID);
        assert_eq!(fixture.opening_shared_pool, 2);
        assert_eq!(fixture.deck_order, CrestCardId::ALL);
        assert_eq!(fixture.center_status, "hidden");
        assert_eq!(fixture.terminal_outcome, "none");

        assert!(Manifest::parse("game_id = \"poker_lite\"\ntrigger = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"poker_lite_standard\"\nselector = \"bad\"\n"
        )
        .is_err());
        assert!(Fixture::parse("{\"game_id\":\"poker_lite\",\"valid_if\":\"bad\"}").is_err());
    }
}
