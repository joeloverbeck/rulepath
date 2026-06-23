use engine_core::{HashValue, SeatId, Seed, StableSerialize, Viewer, VisibilityScope};
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind, SetupEvidenceV1Driver,
    PROFILE_VERSION_V1, SETUP_EVIDENCE_V1,
};
use high_card_duel::{
    export_public_observer_replay, generate_internal_full_trace, import_public_export,
    project_view, public_effect, setup_match, HighCardDuelEffect, HighCardDuelSeat, Manifest,
    SetupOptions, VariantCatalog,
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
fn public_effect_constructor_preserves_public_scope_and_payload_text() {
    let effect = public_effect(HighCardDuelEffect::HandCountChanged {
        seat_0_count: 2,
        seat_1_count: 3,
        deck_count: 18,
    });

    assert_eq!(effect.visibility, VisibilityScope::Public);
    assert_eq!(
        effect.payload.public_payload_text(),
        "hcd_hand_count_changed:seat_0_count=2;seat_1_count=3;deck_count=18"
    );
}

#[test]
fn setup_evidence_v1_profile_driver_wraps_public_fixture_metadata() {
    let fixture = include_str!("../data/fixtures/high_card_duel_standard.fixture.json");
    assert!(fixture.contains("\"fixture_id\": \"high_card_duel_standard_gate8\""));
    assert!(fixture.contains("\"game_id\": \"high_card_duel\""));
    assert!(fixture.contains("\"variant\": \"high_card_duel_standard\""));
    assert!(fixture.contains("\"rules_version\": \"high-card-duel-rules-v1\""));
    assert!(fixture.contains(
        "\"fixture_kinds\": [\"setup\", \"commands\", \"terminal\", \"diagnostic\", \"bot\"]"
    ));
    assert!(!fixture.contains("private_deal"));
    assert!(!fixture.contains("hcd:r"));

    let driver = SetupEvidenceV1Driver::new("high_card_duel");
    let artifact = setup_evidence_profile_artifact(
        SETUP_EVIDENCE_V1,
        Some("public"),
        "high_card_duel",
        &["seat_grammar_version", "setup_options", "expected_setup"],
    );

    let report = driver
        .validate(&artifact)
        .expect("profile metadata validates");
    assert_eq!(report.profile_id, SETUP_EVIDENCE_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "public");
    assert_eq!(report.validator_owner, "high_card_duel");

    let fixture_hash = driver
        .validate_with(&artifact, |_| {
            HashValue::from_stable_bytes(fixture.as_bytes())
        })
        .expect("profile delegates to fixture metadata validator");
    assert_eq!(
        fixture_hash,
        HashValue::from_stable_bytes(fixture.as_bytes())
    );
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let wrong_profile = setup_evidence_profile_artifact(
        "replay-command-v1",
        Some("public"),
        "high_card_duel",
        &["expected_setup"],
    );
    assert_eq!(
        driver
            .validate(&wrong_profile)
            .expect_err("wrong profile id rejects")
            .kind,
        ProfileValidationErrorKind::WrongProfileId
    );

    let wrong_owner = setup_evidence_profile_artifact(
        SETUP_EVIDENCE_V1,
        Some("public"),
        "other",
        &["expected_setup"],
    );
    assert_eq!(
        driver
            .validate(&wrong_owner)
            .expect_err("wrong owner rejects")
            .kind,
        ProfileValidationErrorKind::WrongValidatorOwner
    );

    let wrong_visibility = setup_evidence_profile_artifact(
        SETUP_EVIDENCE_V1,
        Some("private-source"),
        "high_card_duel",
        &["expected_setup"],
    );
    assert_eq!(
        driver
            .validate(&wrong_visibility)
            .expect_err("wrong visibility rejects")
            .kind,
        ProfileValidationErrorKind::InvalidVisibility
    );

    let wrong_field = setup_evidence_profile_artifact(
        SETUP_EVIDENCE_V1,
        Some("public"),
        "high_card_duel",
        &["expected_setup", "commands"],
    );
    assert_eq!(
        driver
            .validate(&wrong_field)
            .expect_err("wrong field rejects")
            .kind,
        ProfileValidationErrorKind::UnknownField
    );
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

fn setup_evidence_profile_artifact<'a>(
    profile_id: &'a str,
    visibility_class: Option<&'a str>,
    validator_owner: &'a str,
    fields: &'a [&'a str],
) -> ProfileArtifact<'a> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id,
            profile_version: PROFILE_VERSION_V1,
            visibility_class,
            validator_owner,
            canonical_byte_authority: "none",
            migration_update_note: Some("profile migration reviewed"),
        },
        fields,
        canonical_byte_claim: false,
    }
}
