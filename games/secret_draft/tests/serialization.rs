use engine_core::{HashValue, SeatId, StableSerialize, Viewer};
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind, SetupEvidenceV1Driver,
    PROFILE_VERSION_V1, SETUP_EVIDENCE_V1,
};
use secret_draft::{
    load_manifest, load_standard_fixture, load_variants,
    replay_support::{
        export_public_replay, generate_internal_full_trace, PublicReplayExport,
        SecretDraftInternalTrace,
    },
    setup_match, DraftItemId, Fixture, Manifest, SetupOptions, VariantCatalog, GAME_ID,
    RULES_VERSION_LABEL, VARIANT_ID,
};

#[test]
fn static_data_and_fixture_match_setup_and_reject_unknown_fields() {
    let manifest = load_manifest().expect("manifest parses");
    let variants = load_variants().expect("variants parse");
    let fixture = load_standard_fixture().expect("fixture parses");
    let state = setup_match(
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
    assert_eq!(fixture.visible_pool.to_vec(), state.visible_pool);
    assert_eq!(fixture.visible_pool.len(), DraftItemId::ALL.len());
    assert_eq!(fixture.seat_0_commitment, "none");
    assert_eq!(fixture.seat_1_commitment, "none");

    assert!(Manifest::parse("game_id = \"secret_draft\"\ndebug = \"bad\"\n").is_err());
    assert!(
        VariantCatalog::parse("variant_id = \"secret_draft_standard\"\nformula = \"bad\"\n")
            .is_err()
    );
    assert!(Fixture::parse(
        "{\n  \"fixture_id\": \"x\",\n  \"game_id\": \"secret_draft\",\n  \"debug\": \"bad\"\n}"
    )
    .is_err());
}

#[test]
fn setup_evidence_v1_profile_driver_wraps_public_fixture_metadata() {
    let fixture = include_str!("../data/fixtures/secret_draft_standard.fixture.json");
    assert!(fixture.contains("\"fixture_id\": \"secret_draft_standard_gate9_1\""));
    assert!(fixture.contains("\"game_id\": \"secret_draft\""));
    assert!(fixture.contains("\"variant\": \"secret_draft_standard\""));
    assert!(fixture.contains("\"rules_version\": \"secret-draft-rules-v1\""));
    assert!(fixture.contains("\"visible_pool\""));
    assert!(fixture.contains("\"seat_0_commitment\": \"none\""));
    assert!(fixture.contains("\"seat_1_commitment\": \"none\""));
    assert!(!fixture.contains("selector"));
    assert!(!fixture.contains("trigger"));
    assert!(!fixture.contains("reveal"));

    let driver = SetupEvidenceV1Driver::new("secret_draft");
    let artifact = setup_evidence_profile_artifact(
        SETUP_EVIDENCE_V1,
        Some("public"),
        "secret_draft",
        &["seat_grammar_version", "setup_options", "expected_setup"],
    );

    let report = driver
        .validate(&artifact)
        .expect("profile metadata validates");
    assert_eq!(report.profile_id, SETUP_EVIDENCE_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "public");
    assert_eq!(report.validator_owner, "secret_draft");

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
        "secret_draft",
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
        "secret_draft",
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
        "secret_draft",
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
fn internal_trace_json_round_trips_stably_and_rejects_unknown_fields() {
    let trace = generate_internal_full_trace();
    let json = trace.to_json();
    let reparsed = SecretDraftInternalTrace::from_json(&json).expect("trace parses");

    assert_eq!(reparsed, trace);
    assert_eq!(trace.stable_bytes(), json.as_bytes());
    assert_eq!(
        SecretDraftInternalTrace::from_json(
            &json.replace("\"commands\"", "\"debug\":\"bad\",\"commands\"")
        )
        .expect_err("top-level unknown rejected"),
        "unknown field `debug`"
    );
    assert!(SecretDraftInternalTrace::from_json(
        &json.replace("\"actor\"", "\"debug\":\"bad\",\"actor\"")
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
        &json.replace("\"step_index\"", "\"debug\":\"bad\",\"step_index\"")
    )
    .is_err());
}

#[test]
fn state_and_view_summaries_are_deterministic() {
    let seats = [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())];
    let first = setup_match(&seats, &SetupOptions::default()).expect("first setup succeeds");
    let second = setup_match(&seats, &SetupOptions::default()).expect("second setup succeeds");
    let viewer = Viewer { seat_id: None };

    assert_eq!(first.stable_summary(), second.stable_summary());
    assert_eq!(
        secret_draft::project_view(&first, &viewer).stable_summary(),
        secret_draft::project_view(&second, &viewer).stable_summary()
    );
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
