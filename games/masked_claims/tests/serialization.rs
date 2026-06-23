use engine_core::{HashValue, SeatId, Seed, StableSerialize, Viewer};
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind, SetupEvidenceV1Driver,
    PROFILE_VERSION_V1, SETUP_EVIDENCE_V1,
};
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
fn setup_evidence_v1_profile_driver_wraps_mask_fixture_metadata() {
    let fixture = include_str!("../data/fixtures/masked_claims_standard.fixture.json");
    assert!(fixture.contains("\"fixture_id\": \"masked_claims_standard_gate11\""));
    assert!(fixture.contains("\"game_id\": \"masked_claims\""));
    assert!(fixture.contains("\"variant\": \"masked_claims_standard\""));
    assert!(fixture.contains("\"rules_version\": \"masked-claims-rules-v1\""));
    assert!(fixture.contains("\"mask_order\""));
    assert!(fixture.contains("\"hand_status\": \"hidden_by_setup\""));
    assert!(fixture.contains("\"reserve_status\": \"internal_only\""));
    assert!(!fixture.contains("selector"));
    assert!(!fixture.contains("trigger"));
    assert!(!fixture.contains("reaction"));

    let driver = SetupEvidenceV1Driver::new("masked_claims");
    let artifact = setup_evidence_profile_artifact(
        SETUP_EVIDENCE_V1,
        Some("public"),
        "masked_claims",
        &["seat_grammar_version", "setup_options", "expected_setup"],
    );

    let report = driver
        .validate(&artifact)
        .expect("profile metadata validates");
    assert_eq!(report.profile_id, SETUP_EVIDENCE_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "public");
    assert_eq!(report.validator_owner, "masked_claims");

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
        "masked_claims",
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
        "masked_claims",
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
        "masked_claims",
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
