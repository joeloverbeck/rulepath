use engine_core::{SeatId, StableSerialize, Viewer};
use poker_lite::{
    load_manifest, load_standard_fixture, load_variants,
    replay_support::{
        export_public_replay, generate_internal_full_trace, PokerLiteInternalTrace,
        PublicReplayExport,
    },
    setup_match, Fixture, Manifest, SetupOptions, VariantCatalog, GAME_ID, RULES_VERSION_LABEL,
    VARIANT_ID,
};

#[test]
fn static_data_and_fixture_match_setup_and_reject_unknown_fields() {
    let manifest = load_manifest().expect("manifest parses");
    let variants = load_variants().expect("variants parse");
    let fixture = load_standard_fixture().expect("fixture parses");
    let state = setup_match(
        engine_core::Seed(0),
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds");

    assert_eq!(manifest.game_id, GAME_ID);
    assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
    assert_eq!(variants.selected.id, VARIANT_ID);
    assert_eq!(fixture.game_id, GAME_ID);
    assert_eq!(fixture.variant, VARIANT_ID);
    assert_eq!(fixture.rules_version, RULES_VERSION_LABEL);
    assert_eq!(fixture.opening_shared_pool, state.shared_pool);
    assert_eq!(fixture.center_status, "hidden");

    assert!(Manifest::parse("game_id = \"poker_lite\"\ndebug = \"bad\"\n").is_err());
    assert!(
        VariantCatalog::parse("variant_id = \"poker_lite_standard\"\nformula = \"bad\"\n").is_err()
    );
    assert!(Fixture::parse(
        "{\n  \"fixture_id\": \"x\",\n  \"game_id\": \"poker_lite\",\n  \"debug\": \"bad\"\n}"
    )
    .is_err());
}

#[test]
fn internal_trace_json_round_trips_stably_and_rejects_unknown_fields() {
    let trace = generate_internal_full_trace();
    let json = trace.to_json();
    let reparsed = PokerLiteInternalTrace::from_json(&json).expect("trace parses");

    assert_eq!(reparsed, trace);
    assert_eq!(trace.stable_bytes(), json.as_bytes());
    assert_eq!(
        PokerLiteInternalTrace::from_json(
            &json.replace("\"commands\"", "\"debug\":\"bad\",\"commands\"")
        )
        .expect_err("top-level unknown rejected"),
        "unknown field `debug`"
    );
    assert!(PokerLiteInternalTrace::from_json(
        &json.replace("\"actor\"", "\"selector\":\"bad\",\"actor\"")
    )
    .is_err());
}

#[test]
fn public_export_json_round_trips_stably_and_rejects_unknown_fields() {
    let trace = generate_internal_full_trace();
    let export = export_public_replay(&trace, &Viewer { seat_id: None });
    let json = export.to_json();
    let reparsed = PublicReplayExport::from_json(&json).expect("export parses");

    assert_eq!(reparsed, export);
    assert_eq!(export.stable_bytes(), json.as_bytes());
    assert_eq!(
        PublicReplayExport::from_json(&json.replace("\"steps\"", "\"debug\":\"bad\",\"steps\""))
            .expect_err("top-level unknown rejected"),
        "unknown field `debug`"
    );
    assert!(PublicReplayExport::from_json(
        &json.replace("\"step_index\"", "\"formula\":\"bad\",\"step_index\"")
    )
    .is_err());
}

#[test]
fn state_and_view_summaries_are_deterministic() {
    let seats = [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())];
    let first =
        setup_match(engine_core::Seed(42), &seats, &SetupOptions::default()).expect("setup");
    let second =
        setup_match(engine_core::Seed(42), &seats, &SetupOptions::default()).expect("setup");
    let viewer = Viewer { seat_id: None };

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
    assert_eq!(
        poker_lite::project_view(&first, &viewer).stable_summary(),
        poker_lite::project_view(&second, &viewer).stable_summary()
    );
}
