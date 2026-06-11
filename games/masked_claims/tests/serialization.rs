use engine_core::{HashValue, SeatId, Seed, StableSerialize, Viewer};
use masked_claims::{
    setup_match, Fixture, Manifest, PublicReplayExport, PublicReplayStep, SetupOptions, Variant,
    VariantCatalog, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

#[test]
fn static_data_parses_and_rejects_unknown_fields() {
    let manifest = Manifest::parse(include_str!("../data/manifest.toml")).expect("manifest parses");
    assert_eq!(manifest.game_id, GAME_ID);
    assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
    assert!(Manifest::parse("game_id = \"masked_claims\"\nunknown = true\n").is_err());

    let variants =
        VariantCatalog::parse(include_str!("../data/variants.toml")).expect("variants parse");
    assert_eq!(variants.selected.id, VARIANT_ID);
    assert!(VariantCatalog::parse("variant_id = \"x\"\nunknown = true\n").is_err());
    assert!(Variant::resolve("unknown").is_err());

    let fixture = Fixture::parse(include_str!(
        "../data/fixtures/masked_claims_standard.fixture.json"
    ))
    .expect("fixture parses");
    assert_eq!(fixture.game_id, GAME_ID);
    assert!(Fixture::parse("{\"game_id\":\"masked_claims\",\"unknown\":true}").is_err());
}

#[test]
fn public_view_and_export_have_stable_hashes_and_round_trip() {
    let state = setup_match(Seed(41), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let view = masked_claims::project_view(&state, &Viewer { seat_id: None });
    let first = view.stable_hash();
    let second = HashValue::from_stable_bytes(&view.stable_summary().into_bytes());
    assert_eq!(first, second);

    let step = PublicReplayStep::from_view(0, &view, Vec::new(), "setup", false);
    let export = PublicReplayExport::new("observer", vec![step]);
    let json = export.to_json();
    assert_eq!(PublicReplayExport::from_json(&json).unwrap(), export);
    assert!(PublicReplayExport::from_json("{\"viewer\":\"observer\"}").is_err());
}
