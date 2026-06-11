use engine_core::{
    ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, StableSerialize, Viewer,
};
use flood_watch::{
    apply_command, export_public_replay, generate_internal_full_trace, import_public_export_json,
    load_deluge_fixture, load_manifest, load_standard_fixture, load_variants, public_replay_step,
    setup_match, Fixture, Manifest, ScenarioVariant, SetupOptions, VariantCatalog, ACTION_END_TURN,
    GAME_ID, RULES_VERSION_LABEL, STANDARD_DECK_SIZE, VARIANT_DELUGE_ID, VARIANT_STANDARD_ID,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn viewer() -> Viewer {
    Viewer { seat_id: None }
}

fn end_turn_command(state: &flood_watch::FloodWatchState) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: SeatId("seat_0".to_owned()),
        },
        action_path: ActionPath {
            segments: vec![ACTION_END_TURN.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

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

#[test]
fn public_export_serializes_stably_and_rejects_unknown_fields() {
    let mut state = setup_match(Seed(11), &seats(), &SetupOptions::default()).unwrap();
    let command = end_turn_command(&state);
    let applied = apply_command(&mut state, &command).unwrap();
    let step = public_replay_step(0, &state, &command, &applied.effects, &viewer());
    let export = export_public_replay(state.variant.id.clone(), &viewer(), vec![step]);
    let json = export.to_json();

    assert_eq!(
        export.stable_hash(),
        export_public_replay(state.variant.id.clone(), &viewer(), export.steps.clone())
            .stable_hash()
    );
    assert!(import_public_export_json(&json).is_ok());
    assert!(import_public_export_json(
        "{\"schema_version\":1,\"game_id\":\"flood_watch\",\"unknown\":true}"
    )
    .is_err());
}

#[test]
fn internal_trace_is_separate_full_order_authority() {
    let state = setup_match(Seed(11), &seats(), &SetupOptions::default()).unwrap();
    let trace = generate_internal_full_trace(11, &state);
    let json = trace.to_json();

    assert_eq!(trace.full_deck_order.len(), STANDARD_DECK_SIZE as usize);
    assert!(json.contains("full_deck_order"));
    assert!(json.contains("#1"));
    assert_eq!(
        trace.stable_hash(),
        generate_internal_full_trace(11, &state).stable_hash()
    );
}
