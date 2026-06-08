use engine_core::{SeatId, Seed, StableSerialize, Viewer};
use high_card_duel::{
    export_public_observer_replay, generate_internal_full_trace, import_public_export,
    project_view, setup_match, HighCardDuelSeat, Manifest, SetupOptions, VariantCatalog,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn viewer(index: Option<u8>) -> Viewer {
    Viewer {
        seat_id: index.map(|seat| SeatId(format!("seat-{seat}"))),
    }
}

#[test]
fn static_data_rejects_unknown_fields_in_integration_schema_surface() {
    assert!(Manifest::parse("game_id = \"high_card_duel\"\ntrigger = \"bad\"\n").is_err());
    assert!(VariantCatalog::parse(
        "variant_id = \"high_card_duel_standard\"\nselector = \"bad\"\n"
    )
    .is_err());
}

#[test]
fn public_and_seat_view_serialization_is_stable_for_same_seed() {
    let left = setup_match(Seed(31), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let right = setup_match(Seed(31), &seats(), &SetupOptions::default()).expect("setup succeeds");

    let left_public = project_view(&left, &viewer(None));
    let right_public = project_view(&right, &viewer(None));
    let left_seat = project_view(&left, &viewer(Some(0)));
    let right_seat = project_view(&right, &viewer(Some(0)));

    assert_eq!(left_public.stable_summary(), right_public.stable_summary());
    assert_eq!(left_public.stable_hash(), right_public.stable_hash());
    assert_eq!(left_seat.stable_summary(), right_seat.stable_summary());
    assert_eq!(left_seat.stable_hash(), right_seat.stable_hash());
    assert!(left_seat
        .stable_summary()
        .contains(HighCardDuelSeat::Seat0.as_str()));
}

#[test]
fn internal_trace_and_public_export_serialization_are_stable() {
    let left_trace = generate_internal_full_trace(44);
    let right_trace = generate_internal_full_trace(44);
    let left_export = export_public_observer_replay(&left_trace);
    let right_export = export_public_observer_replay(&right_trace);

    assert_eq!(left_trace.to_json(), right_trace.to_json());
    assert_eq!(left_trace.stable_hash(), right_trace.stable_hash());
    assert_eq!(left_export.to_json(), right_export.to_json());
    assert_eq!(left_export.stable_hash(), right_export.stable_hash());
    assert!(!left_export.to_json().contains("\"seed\""));
    assert!(!left_export.to_json().contains("commit/hcd:r"));
}

#[test]
fn public_export_import_preserves_redacted_timeline_only() {
    let trace = generate_internal_full_trace(55);
    let export = export_public_observer_replay(&trace);
    let timeline = import_public_export(&export);

    assert_eq!(timeline.viewer, "observer");
    assert_eq!(timeline.steps, export.steps);
    assert!(timeline
        .steps
        .iter()
        .all(|step| !step.redacted_command_summary.contains("hcd:r")));
}
