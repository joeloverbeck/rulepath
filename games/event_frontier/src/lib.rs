//! `event_frontier` official-game crate skeleton for Event Frontier.

pub mod actions;
pub mod bots;
pub mod cards;
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
    legal_action_metadata, legal_action_tree, parse_action_path, validate_command,
    EventFrontierAction, MenuEntry, OperationKind, OperationSelection, ValidatedAction,
    ACTION_EVENT, ACTION_LIMITED_OPERATION, ACTION_OPERATION, ACTION_PASS,
};
pub use cards::{CardCatalog, CardData, CardId};
pub use effects::{EventFrontierEffect, EventFrontierEffectEnvelope, SiteScoreBreakdown};
pub use ids::{
    FactionId, SiteId, GAME_ID, RULES_VERSION_LABEL, STANDARD_CARD_COUNT, STANDARD_EPOCH_COUNT,
    STANDARD_RESOURCE_CAP, STANDARD_SEAT_COUNT, STANDARD_SITE_COUNT, VARIANT_HARD_WINTER_ID,
    VARIANT_LAND_RUSH_ID, VARIANT_STANDARD_ID,
};
pub use rules::{
    apply_command, apply_validated_action, initialize_card_phase, resolve_reckoning, AppliedAction,
};
pub use setup::{setup_match, validate_variant, SetupOptions};
pub use state::{
    ActiveEdict, AdjacencyEntry, CardPhase, DeckState, Eligibility, EventFrontierSnapshot,
    EventFrontierState, FactionScores, FirstChoice, ResourcePools, SiteState, TerminalOutcome,
    VictoryType,
};
pub use variants::{Manifest, ScenarioVariant, VariantCatalog};

pub fn load_manifest() -> Result<Manifest, String> {
    Manifest::parse(include_str!("../data/manifest.toml"))
}

pub fn load_variants() -> Result<VariantCatalog, String> {
    VariantCatalog::parse(include_str!("../data/variants.toml"))
}

pub fn load_cards() -> Result<CardCatalog, String> {
    CardCatalog::parse(include_str!("../data/cards.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_data_parses_and_rejects_unknown_or_behavior_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");
        let cards = load_cards().expect("cards parse");

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.display_name, "Event Frontier");
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.site_count, STANDARD_SITE_COUNT);
        assert_eq!(variants.standard.id, VARIANT_STANDARD_ID);
        assert_eq!(variants.hard_winter.id, VARIANT_HARD_WINTER_ID);
        assert_eq!(variants.land_rush.id, VARIANT_LAND_RUSH_ID);
        assert_eq!(cards.cards.len(), STANDARD_CARD_COUNT as usize);
        assert_eq!(cards.cards[0].id, CardId::BorderSurvey);

        assert!(Manifest::parse("game_id = \"event_frontier\"\ntrigger = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "standard_variant_id = \"event_frontier_standard\"\neffect = \"bad\"\n"
        )
        .is_err());
        assert!(CardCatalog::parse("card_ids = \"ef_border_survey\"\nscript = \"bad\"\n").is_err());
        assert!(
            CardCatalog::parse("card_ids = \"ef_border_survey\"\nunknown = \"bad\"\n").is_err()
        );
    }
}
