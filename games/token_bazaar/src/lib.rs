//! `token_bazaar` official-game crate skeleton.

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
    actor_seat, legal_action_tree, parse_action_segment, ActionFamily, TokenBazaarAction,
    COLLECT_SEGMENT_PREFIX, EXCHANGE_SEGMENT_PREFIX, FULFILL_SEGMENT_PREFIX, PASS_SEGMENT,
};
pub use bots::{
    action_from_decision, BotDecision, TokenBazaarLevel1Bot, TokenBazaarRandomBot,
    LEVEL1_POLICY_ID, RANDOM_POLICY_ID,
};
pub use effects::{public_effect, TokenBazaarEffect};
pub use ids::{
    CollectBundleId, ContractId, ResourceId, TokenBazaarSeat, TokenBazaarSlot, GAME_ID,
    RULES_VERSION_LABEL, STANDARD_CONTRACT_COUNT, STANDARD_MARKET_SLOT_COUNT,
    STANDARD_RESOURCE_SUPPLY, STANDARD_SEAT_COUNT, STANDARD_STARTING_RESOURCE_COUNT,
    STANDARD_TURNS_PER_SEAT, VARIANT_ID,
};
pub use replay_support::{
    action_tree_hash, actor_for_state, command_for_state, default_seats, effect_hash,
    effect_stable_string, export_public_replay, import_public_export, replay_commands, state_hash,
    PublicReplayExport, PublicReplayStep, PublicReplayTimeline, ReplayCommandPath, ReplayResult,
    ReplayStepProjection,
};
pub use rules::{
    apply_action, determine_terminal_outcome, diagnostic, legal_actions, validate_command,
    wrong_seat_diagnostic, ValidatedAction,
};
pub use setup::{setup_match, SetupOptions};
pub use state::{
    ContractSpec, ResourceCounts, TerminalOutcome, TerminalTrigger, TiebreakRung,
    TokenBazaarSnapshot, TokenBazaarState,
};
pub use ui::{
    action_preview_copy, contract_accessibility_label, resource_accessibility_label, ui_metadata,
    UiMetadata,
};
pub use variants::{Manifest, Variant, VariantCatalog};
pub use visibility::{
    project_view, project_view_with_effects, ContractView, EffectView, InventoryView,
    LegalActionView, MarketSlotView, OutcomeRationaleView, OutcomeStandingView, PublicView,
    ResourceSupplyView, TerminalView, TiebreakLadderRungView,
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
