use engine_core::{SeatId, StableSerialize};
use frontier_control::{
    load_highlands_fixture, load_manifest, load_standard_fixture, load_variants, setup_match,
    Fixture, Manifest, SetupOptions, VariantCatalog,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

#[test]
fn static_content_parses_and_rejects_unknown_or_behavior_fields() {
    load_manifest().unwrap();
    load_variants().unwrap();
    load_standard_fixture().unwrap();
    load_highlands_fixture().unwrap();

    assert!(Manifest::parse("game_id = \"frontier_control\"\nunknown = \"x\"\n").is_err());
    assert!(VariantCatalog::parse("standard_variant_id = \"x\"\nwhen = \"bad\"\n").is_err());
    assert!(Fixture::parse("{\"game_id\":\"frontier_control\",\"if\":\"bad\"}").is_err());
}

#[test]
fn stable_state_serialization_is_repeatable() {
    let first = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let second = setup_match(&seats(), &SetupOptions::default()).unwrap();

    assert_eq!(first.stable_bytes(), second.stable_bytes());
    assert_eq!(first.stable_hash(), second.stable_hash());
}
