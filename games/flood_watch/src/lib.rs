//! `flood_watch` official-game crate skeleton for Flood Watch.

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

pub use ids::{
    DistrictId, EventKind, FloodWatchRole, GAME_ID, RULES_VERSION_LABEL, STANDARD_ACTION_BUDGET,
    STANDARD_DECK_SIZE, STANDARD_DISTRICT_COUNT, STANDARD_DRAWS_PER_PHASE, STANDARD_LEVEE_CAP,
    STANDARD_SEAT_COUNT, VARIANT_DELUGE_ID, VARIANT_STANDARD_ID,
};
pub use setup::SetupOptions;
pub use state::FloodWatchState;
pub use variants::{EventComposition, Fixture, Manifest, ScenarioVariant, VariantCatalog};

pub fn load_manifest() -> Result<Manifest, String> {
    Manifest::parse(include_str!("../data/manifest.toml"))
}

pub fn load_variants() -> Result<VariantCatalog, String> {
    VariantCatalog::parse(include_str!("../data/variants.toml"))
}

pub fn load_standard_fixture() -> Result<Fixture, String> {
    Fixture::parse(include_str!(
        "../data/fixtures/flood_watch_standard.fixture.json"
    ))
}

pub fn load_deluge_fixture() -> Result<Fixture, String> {
    Fixture::parse(include_str!(
        "../data/fixtures/flood_watch_deluge.fixture.json"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_data_parses_and_rejects_unknown_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");
        let standard = load_standard_fixture().expect("standard fixture parses");
        let deluge = load_deluge_fixture().expect("deluge fixture parses");

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.display_name, "Flood Watch");
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.district_count, STANDARD_DISTRICT_COUNT);
        assert_eq!(variants.standard.id, VARIANT_STANDARD_ID);
        assert_eq!(variants.deluge.id, VARIANT_DELUGE_ID);
        assert_eq!(
            variants.standard.event_composition.total_cards(),
            STANDARD_DECK_SIZE
        );
        assert_eq!(standard.variant, VARIANT_STANDARD_ID);
        assert_eq!(deluge.variant, VARIANT_DELUGE_ID);
        assert_eq!(standard.event_deck_order_status, "computed_from_seed");
        assert_eq!(deluge.event_deck_order_status, "computed_from_seed");

        assert!(Manifest::parse("game_id = \"flood_watch\"\ntrigger = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "standard_variant_id = \"flood_watch_standard\"\nselector = \"bad\"\n"
        )
        .is_err());
        assert!(Fixture::parse("{\"game_id\":\"flood_watch\",\"valid_if\":\"bad\"}").is_err());
    }
}
