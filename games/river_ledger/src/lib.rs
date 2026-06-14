//! `river_ledger` official-game crate scaffold for River Ledger.

pub mod actions;
pub mod betting;
pub mod cards;
pub mod evaluator;
pub mod ids;
pub mod rules;
pub mod setup;
pub mod state;
pub mod ui;
pub mod variants;

pub use actions::{
    actor_seat, legal_action_tree, parse_action_segment, validate_command, RiverLedgerAction,
    ValidatedAction,
};
pub use betting::{call_price, first_live_after, live_seats, next_live_after};
pub use cards::{canonical_deck, Card, Deck, Rank, Suit, STANDARD_CARD_COUNT};
pub use evaluator::{
    best_five_from_seven, compare_evaluations, evaluate_five, HandCategory, HandEvaluation,
};
pub use ids::{
    actor_for_seat, seat_id_for_index, seat_viewer_for_index, RiverLedgerSeat, ACTION_BET,
    ACTION_CALL, ACTION_CHECK, ACTION_FOLD, ACTION_RAISE, GAME_ID, MAX_RAISES_PER_STREET,
    RULES_VERSION_LABEL, RULE_ID_PREFIX, STANDARD_BIG_BET_UNIT, STANDARD_BIG_BLIND,
    STANDARD_DEFAULT_SEATS, STANDARD_MAX_SEATS, STANDARD_MIN_SEATS, STANDARD_SMALL_BET_UNIT,
    STANDARD_SMALL_BLIND, STANDARD_STREET_COUNT, VARIANT_ID,
};
pub use rules::apply_action;
pub use setup::{setup_match, shuffle_deck, SetupOptions};
pub use state::{
    BettingRoundState, ContributionLedger, Phase, RiverLedgerState, SeatLedger, SeatStatus,
    ShowdownReveal, ShowdownSeatExplanation, Street, TerminalOutcome,
};
pub use ui::{ui_metadata, UiMetadata};
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
        assert_eq!(manifest.display_name, "River Ledger");
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.min_seats, STANDARD_MIN_SEATS);
        assert_eq!(manifest.default_seats, STANDARD_DEFAULT_SEATS);
        assert_eq!(manifest.max_seats, STANDARD_MAX_SEATS);
        assert_eq!(manifest.card_count, STANDARD_CARD_COUNT);
        assert_eq!(variants.selected, Variant::river_ledger_standard());

        assert!(Manifest::parse("game_id = \"river_ledger\"\ntrigger = \"bad\"\n").is_err());
        assert!(Manifest::parse("game_id = \"river_ledger\"\nunknown = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"river_ledger_standard\"\nvalid_if = \"bad\"\n"
        )
        .is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"river_ledger_standard\"\nshowdown_formula = \"bad\"\n"
        )
        .is_err());
    }
}
