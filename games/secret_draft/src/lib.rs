//! `secret_draft` official-game crate skeleton.

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

pub use actions::{legal_action_tree, SecretDraftAction};
pub use bots::{SecretDraftLevel1Bot, SecretDraftRandomBot, LEVEL1_POLICY_ID, RANDOM_POLICY_ID};
pub use effects::{SecretDraftEffect, TieBreakSummary};
pub use ids::{
    DraftItemId, DraftThread, SecretDraftSeat, GAME_ID, RULES_VERSION_LABEL, STANDARD_ITEM_COUNT,
    STANDARD_ROUND_COUNT, STANDARD_SEAT_COUNT, VARIANT_ID,
};
pub use replay_support::{action_tree_v1_bytes, action_tree_v1_hash, state_hash, ReplayResult};
pub use rules::{
    apply_action, determine_terminal_outcome_from_summary, legal_actions,
    terminal_tie_break_summary, validate_action, ValidatedAction,
};
pub use setup::{setup_match, SetupOptions};
pub use state::{
    DraftItemSpec, Phase, RevealedRound, ScoreSummary, SecretDraftState, TerminalOutcome,
};
pub use ui::{ui_metadata, UiMetadata};
pub use variants::{Fixture, Manifest, Variant, VariantCatalog};
pub use visibility::{
    project_view, OutcomeRationaleView, OutcomeStandingView, PublicView, TerminalView,
    TiebreakLadderRungView,
};

pub fn load_manifest() -> Result<Manifest, String> {
    Manifest::parse(include_str!("../data/manifest.toml"))
}

pub fn load_variants() -> Result<VariantCatalog, String> {
    VariantCatalog::parse(include_str!("../data/variants.toml"))
}

pub fn load_standard_fixture() -> Result<Fixture, String> {
    Fixture::parse(include_str!(
        "../data/fixtures/secret_draft_standard.fixture.json"
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
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.round_count, STANDARD_ROUND_COUNT);
        assert_eq!(manifest.item_count, STANDARD_ITEM_COUNT);
        assert_eq!(variants.selected.id, VARIANT_ID);
        assert_eq!(variants.selected.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(fixture.game_id, GAME_ID);
        assert_eq!(fixture.variant, VARIANT_ID);
        assert_eq!(fixture.round_number, 1);
        assert_eq!(fixture.visible_pool, DraftItemId::ALL);
        assert_eq!(fixture.seat_0_commitment, "none");
        assert_eq!(fixture.seat_1_commitment, "none");
        assert!(fixture.seat_0_drafted.is_empty());
        assert!(fixture.seat_1_drafted.is_empty());
        assert_eq!(fixture.seat_0_score, 0);
        assert_eq!(fixture.seat_1_score, 0);

        assert!(Manifest::parse("game_id = \"secret_draft\"\ntrigger = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"secret_draft_standard\"\nselector = \"bad\"\n"
        )
        .is_err());
        assert!(Fixture::parse("{\"game_id\":\"secret_draft\",\"valid_if\":\"bad\"}").is_err());
    }
}
