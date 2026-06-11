//! `frontier_control` official-game crate skeleton for Frontier Control.

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
    FactionId, SiteId, GAME_ID, RULES_VERSION_LABEL, STANDARD_ACTION_BUDGET, STANDARD_ROUND_COUNT,
    STANDARD_SEAT_COUNT, STANDARD_SITE_COUNT, UNIT_CAP_PER_SITE, VARIANT_HIGHLANDS_ID,
    VARIANT_STANDARD_ID,
};
pub use setup::SetupOptions;
pub use variants::{Fixture, Manifest, SiteDefinition, StartUnits, VariantCatalog, VariantMap};

pub fn load_manifest() -> Result<Manifest, String> {
    Manifest::parse(include_str!("../data/manifest.toml"))
}

pub fn load_variants() -> Result<VariantCatalog, String> {
    VariantCatalog::parse(include_str!("../data/variants.toml"))
}

pub fn load_standard_fixture() -> Result<Fixture, String> {
    Fixture::parse(include_str!(
        "../data/fixtures/frontier_control_standard.fixture.json"
    ))
}

pub fn load_highlands_fixture() -> Result<Fixture, String> {
    Fixture::parse(include_str!(
        "../data/fixtures/frontier_control_highlands.fixture.json"
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
        let highlands = load_highlands_fixture().expect("highlands fixture parses");

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.display_name, "Frontier Control");
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.site_count, STANDARD_SITE_COUNT);
        assert_eq!(variants.standard.id, VARIANT_STANDARD_ID);
        assert_eq!(variants.highlands.id, VARIANT_HIGHLANDS_ID);
        assert_eq!(variants.standard.action_budget, STANDARD_ACTION_BUDGET);
        assert_eq!(variants.standard.round_count, STANDARD_ROUND_COUNT);
        assert_eq!(standard.variant, VARIANT_STANDARD_ID);
        assert_eq!(highlands.variant, VARIANT_HIGHLANDS_ID);

        assert!(Manifest::parse("game_id = \"frontier_control\"\ntrigger = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "standard_variant_id = \"frontier_control_standard\"\nselector = \"bad\"\n"
        )
        .is_err());
        assert!(Fixture::parse("{\"game_id\":\"frontier_control\",\"valid_if\":\"bad\"}").is_err());
    }
}
