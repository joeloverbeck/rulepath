use blackglass_pact::{
    canonical_deck, canonical_seat_ids, canonical_team_ids, load_manifest, load_variants, Card,
    CardId, Rank, Suit, TeamId, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID,
};
use engine_core::SeatId;

#[test]
fn static_data_matches_constants_and_rejects_behavior_fields() {
    let manifest = load_manifest().expect("manifest parses");
    let variants = load_variants().expect("variants parse");

    assert_eq!(manifest.game_id, GAME_ID);
    assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
    assert_eq!(variants.selected.id, VARIANT_ID);
    assert_eq!(variants.selected.deck_order, manifest.deck_order);
    assert_eq!(variants.selected.card_count, manifest.card_count);
    assert_eq!(
        variants.selected.team_layout,
        "team_0_seat_0_seat_2__team_1_seat_1_seat_3"
    );
}

#[test]
fn canonical_card_seat_and_team_order_is_stable() {
    let card_ids: Vec<String> = canonical_deck()
        .into_iter()
        .map(|card_id| card_id.as_str())
        .collect();
    let seat_ids: Vec<SeatId> = canonical_seat_ids().into_iter().collect();
    let team_ids: Vec<&'static str> = canonical_team_ids()
        .into_iter()
        .map(TeamId::as_str)
        .collect();

    assert_eq!(card_ids[0], "two_clubs");
    assert_eq!(card_ids[12], "ace_clubs");
    assert_eq!(card_ids[13], "two_diamonds");
    assert_eq!(card_ids[26], "two_hearts");
    assert_eq!(card_ids[39], "two_spades");
    assert_eq!(card_ids[51], "ace_spades");
    assert_eq!(
        CardId::parse("queen_spades").expect("known card").index(),
        49
    );
    assert_eq!(
        seat_ids,
        vec![
            SeatId("seat_0".to_owned()),
            SeatId("seat_1".to_owned()),
            SeatId("seat_2".to_owned()),
            SeatId("seat_3".to_owned()),
        ]
    );
    assert_eq!(team_ids, vec!["team_0", "team_1"]);
    assert_eq!(Card::new(Rank::Ace, Suit::Spades).public_label(), "AS");
}
