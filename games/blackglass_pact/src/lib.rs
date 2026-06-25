//! `blackglass_pact` official-game crate scaffold for Blackglass Pact.

pub mod bidding;
pub mod bots;
pub mod cards;
pub mod effects;
pub mod ids;
pub mod partnerships;
pub mod replay_support;
pub mod rules;
pub mod scoring;
pub mod setup;
pub mod state;
pub mod ui;
pub mod variants;
pub mod visibility;

pub use bidding::{
    active_bid_seat, active_play_seat, apply_bid_action, apply_bid_choice, apply_blind_nil_action,
    apply_blind_nil_choice, legal_action_tree, opening_blind_nil_effect, parse_bid_action_path,
    parse_blind_nil_action_path, BidAction, BlindNilAction, ACTION_BID, ACTION_BID_NIL,
    ACTION_BLIND_DECLARE, ACTION_BLIND_DECLINE, ACTION_BLIND_NIL, MAX_NUMERIC_BID, MIN_NUMERIC_BID,
};
pub use cards::{canonical_deck, Card, CardId, Deck, Rank, Suit};
pub use effects::BlackglassPactEffect;
pub use ids::{
    canonical_seat_ids, BlackglassSeat, TeamId, DATA_VERSION_LABEL, GAME_ID, RULES_VERSION_LABEL,
    STANDARD_CARD_COUNT, STANDARD_DEFAULT_SEATS, STANDARD_HAND_SIZE, STANDARD_MAX_SEATS,
    STANDARD_MIN_SEATS, STANDARD_RANK_COUNT, STANDARD_SEAT_COUNT, STANDARD_SUIT_COUNT,
    STANDARD_TRICKS_PER_HAND, VARIANT_ID,
};
pub use partnerships::{
    canonical_team_ids, members_for_team, partner_for, team_for_seat, team_id_for_index,
};
pub use rules::{
    apply_play_action, apply_play_choice, lead_is_legal, legal_leads, legal_play_cards,
    parse_play_action_path, trick_winner, PlayAction, ACTION_PLAY,
};
pub use scoring::{
    match_outcome, score_completed_hand, score_hand, terminal_winner, BAG_PENALTY_POINTS,
    BAG_THRESHOLD, TARGET_SCORE,
};
pub use setup::{
    deal_for_hand, deal_order_after, eligible_blind_nil_order, seed_for_hand, setup_match,
    setup_match_with_scores, validate_standard_seat_count, HandDeal, SetupOptions,
    BLIND_NIL_DEFICIT_THRESHOLD, HAND_SEED_DERIVATION_V1,
};
pub use state::{
    Bid, BlackglassPactState, BlindNilChoice, HandScoreBreakdown, MatchOutcome, NilResult, Phase,
    PlayedCard, SeatScoreBreakdown, SeatStanding, TeamScoreBreakdown, TeamStanding,
};
pub use variants::{Manifest, Variant, VariantCatalog};
pub use visibility::{
    public_bid_rows, public_bidding_projection, public_team_contracts, PublicBidRow,
    PublicBiddingProjection, PublicTeamContract,
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
    fn static_data_parses_and_rejects_unknown_or_behavior_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");

        assert_eq!(manifest.game_id, GAME_ID);
        assert_eq!(manifest.display_name, "Blackglass Pact");
        assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
        assert_eq!(manifest.min_seats, STANDARD_MIN_SEATS);
        assert_eq!(manifest.default_seats, STANDARD_DEFAULT_SEATS);
        assert_eq!(manifest.max_seats, STANDARD_MAX_SEATS);
        assert_eq!(manifest.card_count, STANDARD_CARD_COUNT);
        assert_eq!(manifest.hand_size, STANDARD_HAND_SIZE);
        assert_eq!(variants.selected, Variant::blackglass_pact_standard());

        assert!(Manifest::parse("game_id = \"blackglass_pact\"\ntrigger = \"bad\"\n").is_err());
        assert!(Manifest::parse("game_id = \"blackglass_pact\"\nunknown = \"bad\"\n").is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"blackglass_pact_standard\"\nformula = \"bad\"\n"
        )
        .is_err());
        assert!(VariantCatalog::parse(
            "variant_id = \"blackglass_pact_standard\"\nscore_formula = \"bad\"\n"
        )
        .is_err());
    }
}
