use engine_core::StableSerialize;
use starbridge_crossing::{
    project_view, setup_match, SetupOptions, StarbridgeSnapshot, Variant, VariantCatalog,
};

#[test]
fn state_snapshot_round_trip_preserves_stable_bytes() {
    let seats = vec![
        engine_core::SeatId::from_zero_based_index(0),
        engine_core::SeatId::from_zero_based_index(1),
    ];
    let state = setup_match(engine_core::Seed(5), &seats, &SetupOptions::default()).unwrap();
    let snapshot = StarbridgeSnapshot::from_state(&state);
    let round_trip = StarbridgeSnapshot::from_state(&snapshot.clone().into_state());

    assert_eq!(snapshot.stable_bytes(), round_trip.stable_bytes());
}

#[test]
fn public_view_serialization_is_all_public_and_stable() {
    let seats = vec![
        engine_core::SeatId::from_zero_based_index(0),
        engine_core::SeatId::from_zero_based_index(1),
    ];
    let state = setup_match(engine_core::Seed(5), &seats, &SetupOptions::default()).unwrap();
    let left = project_view(&state, &engine_core::Viewer { seat_id: None });
    let right = project_view(
        &state,
        &engine_core::Viewer {
            seat_id: Some(seats[0].clone()),
        },
    );

    assert_eq!(left.stable_bytes(), right.stable_bytes());
    assert!(!String::from_utf8(left.stable_bytes())
        .unwrap()
        .contains("private"));
}

#[test]
fn variant_catalog_round_trip_matches_static_variant() {
    let raw = include_str!("../data/variants.toml");
    let parsed = VariantCatalog::parse(raw).unwrap();

    assert_eq!(parsed.selected, Variant::starbridge_classic());
}

#[test]
fn fixture_and_trace_receipts_are_versioned_static_content() {
    for raw in [
        include_str!("../data/fixtures/starbridge_crossing_2p_standard.fixture.json"),
        include_str!("../data/fixtures/starbridge_crossing_3p_standard.fixture.json"),
        include_str!("../data/fixtures/starbridge_crossing_4p_standard.fixture.json"),
        include_str!("../data/fixtures/starbridge_crossing_6p_standard.fixture.json"),
    ] {
        assert!(raw.contains("\"game_id\":\"starbridge_crossing\""));
        assert!(raw.contains("\"hidden_information\":\"not_applicable\""));
    }

    let traces = [
        include_str!("golden_traces/setup-2p-standard.trace.json"),
        include_str!("golden_traces/setup-3p-standard.trace.json"),
        include_str!("golden_traces/setup-4p-standard.trace.json"),
        include_str!("golden_traces/setup-6p-standard.trace.json"),
        include_str!("golden_traces/single-step-move.trace.json"),
        include_str!("golden_traces/one-hop-move.trace.json"),
        include_str!("golden_traces/multi-hop-change-direction.trace.json"),
        include_str!("golden_traces/jump-chain-stop-midway.trace.json"),
        include_str!("golden_traces/repeat-landing-rejected.trace.json"),
        include_str!("golden_traces/invalid-mixed-step-jump.trace.json"),
        include_str!("golden_traces/blocked-forced-pass.trace.json"),
        include_str!("golden_traces/reach-home-first-finish.trace.json"),
        include_str!("golden_traces/finish-order-continues.trace.json"),
        include_str!("golden_traces/terminal-full-standings.trace.json"),
        include_str!("golden_traces/turn-limit-cutoff.trace.json"),
        include_str!("golden_traces/public-observer-all-public.trace.json"),
        include_str!("golden_traces/seat-viewer-parity.trace.json"),
        include_str!("golden_traces/public-replay-round-trip.trace.json"),
    ];

    assert_eq!(traces.len(), 18);
    assert!(traces
        .iter()
        .all(|raw| raw.contains("\"schema_version\":1")));
}
