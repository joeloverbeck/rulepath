//! `token_bazaar` official-game crate skeleton.

pub mod ids;
pub mod variants;

pub use ids::{
    CollectBundleId, ContractId, ResourceId, TokenBazaarSeat, TokenBazaarSlot, GAME_ID,
    RULES_VERSION_LABEL, STANDARD_CONTRACT_COUNT, STANDARD_MARKET_SLOT_COUNT,
    STANDARD_RESOURCE_SUPPLY, STANDARD_SEAT_COUNT, STANDARD_STARTING_RESOURCE_COUNT,
    STANDARD_TURNS_PER_SEAT, VARIANT_ID,
};
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
    fn static_data_parses_and_rejects_unknown_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.resource_supply, STANDARD_RESOURCE_SUPPLY);
        assert_eq!(
            manifest.starting_resource_count,
            STANDARD_STARTING_RESOURCE_COUNT
        );
        assert_eq!(manifest.market_slot_count, STANDARD_MARKET_SLOT_COUNT);
        assert_eq!(manifest.contract_count, STANDARD_CONTRACT_COUNT);
        assert_eq!(manifest.turns_per_seat, STANDARD_TURNS_PER_SEAT);
        assert_eq!(variants.selected.id, VARIANT_ID);
        assert_eq!(variants.selected.seat_count, STANDARD_SEAT_COUNT);
        assert!(Manifest::parse("game_id = \"token_bazaar\"\ntrigger = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"token_bazaar_standard\"\nselector = \"bad\"\n"
        )
        .is_err());
    }
}
