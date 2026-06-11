//! `masked_claims` official-game crate skeleton for Masked Claims.

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
    actor_seat, claimable_tiles, command_public_summary, legal_action_metadata, legal_action_tree,
    parse_action_path, response_public_summary, validate_command, MaskedClaimsAction,
    ResponseChoice, ValidatedAction, ValidatedClaim, ValidatedResponse,
};
pub use bots::{
    action_from_decision, actor_for_seat, BotDecision, MaskedClaimsBotInput, MaskedClaimsLevel1Bot,
    MaskedClaimsRandomBot, LEVEL1_POLICY_ID, RANDOM_POLICY_ID,
};
pub use effects::{
    challenge_declared_effect, challenge_resolved_effect, claim_accepted_effect,
    claim_placed_effect, mask_revealed_effect, public_effect, reaction_window_opened_effect,
    score_changed_effect, terminal_effect, turn_advanced_effect, ChallengeOutcome,
    MaskedClaimsEffect,
};
pub use ids::{
    canonical_masks, Grade, MaskTileId, MaskedClaimsSeat, ACTION_CLAIM, ACTION_RESPOND_ACCEPT,
    ACTION_RESPOND_CHALLENGE, GAME_ID, RULES_VERSION_LABEL, STANDARD_CLAIMS_PER_SEAT,
    STANDARD_GRADE_COUNT, STANDARD_HAND_SIZE, STANDARD_MASK_COUNT, STANDARD_MAX_TURNS,
    STANDARD_RESERVE_SIZE, STANDARD_SEAT_COUNT, STANDARD_TILES_PER_GRADE, VARIANT_ID,
};
pub use replay_support::{PublicReplayExport, PublicReplayStep, PublicReplayTimeline};
pub use rules::apply_action;
pub use setup::{setup_match, shuffle_masks, SetupOptions};
pub use state::{
    ChallengeCounters, ExposedMask, MaskedClaimsState, PendingClaim, Phase, TerminalOutcome,
    VeiledClaim,
};
pub use ui::{grade_accessibility_label, grade_label, ui_metadata, UiMetadata};
pub use variants::{Fixture, Manifest, Variant, VariantCatalog};
pub use visibility::{
    filter_effects_for_viewer, project_view, CounterView, ExposedMaskView, HandCountsView,
    MaskView, OutcomeRationaleView, PedestalView, PrivateView, PublicView, SeatPrivateView,
    TerminalView, VeiledClaimView,
};

pub fn load_manifest() -> Result<Manifest, String> {
    Manifest::parse(include_str!("../data/manifest.toml"))
}

pub fn load_variants() -> Result<VariantCatalog, String> {
    VariantCatalog::parse(include_str!("../data/variants.toml"))
}

pub fn load_standard_fixture() -> Result<Fixture, String> {
    Fixture::parse(include_str!(
        "../data/fixtures/masked_claims_standard.fixture.json"
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
        assert_eq!(manifest.display_name, "Masked Claims");
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(manifest.grade_count, STANDARD_GRADE_COUNT);
        assert_eq!(manifest.tiles_per_grade, STANDARD_TILES_PER_GRADE);
        assert_eq!(manifest.mask_count, STANDARD_MASK_COUNT);
        assert_eq!(manifest.hand_size, STANDARD_HAND_SIZE);
        assert_eq!(manifest.reserve_size, STANDARD_RESERVE_SIZE);
        assert_eq!(manifest.claims_per_seat, STANDARD_CLAIMS_PER_SEAT);
        assert_eq!(manifest.max_turns, STANDARD_MAX_TURNS);
        assert_eq!(variants.selected.id, VARIANT_ID);
        assert_eq!(variants.selected.seat_count, STANDARD_SEAT_COUNT);
        assert_eq!(fixture.game_id, GAME_ID);
        assert_eq!(fixture.variant, VARIANT_ID);
        assert_eq!(fixture.mask_order, MaskTileId::ALL);
        assert_eq!(fixture.hand_status, "hidden_by_setup");
        assert_eq!(fixture.reserve_status, "internal_only");
        assert_eq!(fixture.terminal_outcome, "none");

        assert!(Manifest::parse("game_id = \"masked_claims\"\ntrigger = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"masked_claims_standard\"\nselector = \"bad\"\n"
        )
        .is_err());
        assert!(Fixture::parse("{\"game_id\":\"masked_claims\",\"valid_if\":\"bad\"}").is_err());
    }
}
