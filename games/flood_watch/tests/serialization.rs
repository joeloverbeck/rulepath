use flood_watch::{
    load_deluge_fixture, load_manifest, load_standard_fixture, load_variants, Fixture, Manifest,
    ScenarioVariant, VariantCatalog, GAME_ID, RULES_VERSION_LABEL, STANDARD_DECK_SIZE,
    VARIANT_DELUGE_ID, VARIANT_STANDARD_ID,
};

#[test]
fn static_data_parses_and_rejects_unknown_fields() {
    let manifest = load_manifest().expect("manifest parses");
    assert_eq!(manifest.game_id, GAME_ID);
    assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
    assert!(Manifest::parse("game_id = \"flood_watch\"\nunknown = true\n").is_err());

    let variants = load_variants().expect("variants parse");
    assert_eq!(variants.standard.id, VARIANT_STANDARD_ID);
    assert_eq!(variants.deluge.id, VARIANT_DELUGE_ID);
    assert_eq!(
        variants.standard.event_composition.total_cards(),
        STANDARD_DECK_SIZE
    );
    assert!(VariantCatalog::parse("standard_variant_id = \"x\"\nunknown = true\n").is_err());
    assert!(ScenarioVariant::resolve("unknown").is_err());

    let standard = load_standard_fixture().expect("standard fixture parses");
    let deluge = load_deluge_fixture().expect("deluge fixture parses");
    assert_eq!(standard.game_id, GAME_ID);
    assert_eq!(deluge.game_id, GAME_ID);
    assert!(Fixture::parse("{\"game_id\":\"flood_watch\",\"unknown\":true}").is_err());
}

#[test]
fn static_data_rejects_behavior_looking_fields() {
    assert!(Manifest::parse("game_id = \"flood_watch\"\ntrigger = \"bad\"\n").is_err());
    assert!(VariantCatalog::parse(
        "standard_variant_id = \"flood_watch_standard\"\nselector = \"bad\"\n"
    )
    .is_err());
    assert!(Fixture::parse("{\"game_id\":\"flood_watch\",\"valid_if\":\"bad\"}").is_err());
}

#[test]
fn fixtures_do_not_embed_ordered_event_decks() {
    let standard = include_str!("../data/fixtures/flood_watch_standard.fixture.json");
    let deluge = include_str!("../data/fixtures/flood_watch_deluge.fixture.json");

    assert!(!standard.contains("event_deck\":"));
    assert!(!standard.contains("deck_order\":"));
    assert!(!deluge.contains("event_deck\":"));
    assert!(!deluge.contains("deck_order\":"));
}
