use briar_circuit::{
    canonical_deck, canonical_seat_ids, load_manifest, load_variants, parse_export_header,
    setup_match, CardId, Manifest, SetupOptions, VariantCatalog, GAME_ID, RULES_VERSION_LABEL,
    TRACE_SCHEMA_VERSION, VARIANT_ID, VIEWER_EXPORT_VERSION,
};
use engine_core::{SeatId, Seed};

#[test]
fn static_data_matches_constants_and_rejects_unknown_fields() {
    let manifest = load_manifest().expect("manifest parses");
    let variants = load_variants().expect("variants parse");

    assert_eq!(manifest.game_id, GAME_ID);
    assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
    assert_eq!(variants.selected.id, VARIANT_ID);
    assert_eq!(variants.selected.deck_order, manifest.deck_order);
    assert_eq!(variants.selected.card_count, manifest.card_count);

    assert!(Manifest::parse("game_id = \"briar_circuit\"\ndebug = \"bad\"\n").is_err());
    assert!(
        VariantCatalog::parse("variant_id = \"briar_circuit_standard\"\nformula = \"bad\"\n")
            .is_err()
    );
}

#[test]
fn canonical_card_and_seat_serial_order_is_stable() {
    let card_ids: Vec<String> = canonical_deck()
        .into_iter()
        .map(|card_id| card_id.as_str())
        .collect();
    let seat_ids: Vec<SeatId> = canonical_seat_ids().into_iter().collect();

    assert_eq!(card_ids[0], "two_clubs");
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
}

#[test]
fn setup_state_summary_is_deterministic_for_same_inputs() {
    let seats = canonical_seat_ids();
    let first = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("setup");
    let second = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("setup");

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
}

#[test]
fn replay_export_header_has_version_anchors_and_rejects_unknown_fields() {
    let parsed = parse_export_header(
        "game_id=briar_circuit\nrules_version=briar-circuit-rules-v1\nexport_version=1\nviewer=public\nclass=public\n",
    )
    .expect("header parses");

    assert_eq!(parsed.game_id, GAME_ID);
    assert_eq!(parsed.rules_version, RULES_VERSION_LABEL);
    assert_eq!(parsed.export_version, VIEWER_EXPORT_VERSION);
    assert_eq!(TRACE_SCHEMA_VERSION, 1);

    assert!(parse_export_header(
        "game_id=briar_circuit\nrules_version=briar-circuit-rules-v1\nexport_version=1\nviewer=public\nclass=public\ndebug=bad\n",
    )
    .is_err());
    assert!(parse_export_header(
        "game_id=briar_circuit\nrules_version=briar-circuit-rules-v1\nexport_version=2\nviewer=public\nclass=public\n",
    )
    .is_err());
}
