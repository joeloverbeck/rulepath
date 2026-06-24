use briar_circuit::{
    canonical_deck, canonical_seat_ids, load_manifest, load_variants, parse_export_header,
    setup_match, CardId, Manifest, SetupOptions, VariantCatalog, GAME_ID, RULES_VERSION_LABEL,
    TRACE_SCHEMA_VERSION, VARIANT_ID, VIEWER_EXPORT_VERSION,
};
use engine_core::{SeatId, Seed};
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind, SetupEvidenceV1Driver,
    PROFILE_VERSION_V1, SETUP_EVIDENCE_V1,
};

const SETUP_EVIDENCE_PROFILE_FIELDS: &[&str] = &[
    "profile_id",
    "profile_version",
    "visibility_class",
    "validator_owner",
    "game_id",
    "rules_version",
    "data_version",
    "hash_surface_version",
    "canonical_byte_authority",
    "migration_update_note",
    "not_applicable",
    "seat_grammar_version",
    "setup_options",
    "expected_setup",
];

fn setup_evidence_profile_artifact() -> ProfileArtifact<'static> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: SETUP_EVIDENCE_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("public"),
            validator_owner: "fixture-check",
            canonical_byte_authority: "none",
            migration_update_note: Some("8CR4NSEAPRITRI-030 virtual setup-evidence profile"),
        },
        fields: SETUP_EVIDENCE_PROFILE_FIELDS,
        canonical_byte_claim: false,
    }
}

#[test]
fn static_data_matches_constants_and_rejects_unknown_fields() {
    let manifest = load_manifest().expect("manifest parses");
    let variants = load_variants().expect("variants parse");

    assert_eq!(manifest.game_id, GAME_ID);
    assert_eq!(manifest.rules_version_label, RULES_VERSION_LABEL);
    assert_eq!(variants.selected.id, VARIANT_ID);
    assert_eq!(variants.selected.deck_order, manifest.deck_order);
    assert_eq!(variants.selected.card_count, manifest.card_count);

    assert!(Manifest::parse("game_id = \"briar_circuit\"\ndebug = \"bad\"\n").is_err());
    assert!(
        VariantCatalog::parse("variant_id = \"briar_circuit_standard\"\nformula = \"bad\"\n")
            .is_err()
    );
}

#[test]
fn canonical_card_and_seat_serial_order_is_stable() {
    let card_ids: Vec<String> = canonical_deck()
        .into_iter()
        .map(|card_id| card_id.as_str())
        .collect();
    let seat_ids: Vec<SeatId> = canonical_seat_ids().into_iter().collect();

    assert_eq!(card_ids[0], "two_clubs");
    assert_eq!(card_ids[51], "ace_spades");
    assert_eq!(
        CardId::parse("queen_spades").expect("known card").index(),
        49
    );
    assert_eq!(
        seat_ids,
        vec![
            SeatId("seat_0".to_owned()),
            SeatId("seat_1".to_owned()),
            SeatId("seat_2".to_owned()),
            SeatId("seat_3".to_owned()),
        ]
    );
}

#[test]
fn setup_state_summary_is_deterministic_for_same_inputs() {
    let seats = canonical_seat_ids();
    let first = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("setup");
    let second = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("setup");

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
}

#[test]
fn setup_evidence_v1_driver_validates_standard_fixture_and_deterministic_deal() {
    let fixture = include_str!("../data/fixtures/briar_circuit_standard.fixture.json");
    assert!(!fixture.contains("\"profile_id\""));
    assert!(!fixture.contains("\"canonical_byte_authority\""));
    assert!(fixture.contains("\"fixture_kind\": \"setup\""));
    assert!(fixture.contains("\"trace_id\": \"briar_circuit_standard\""));
    assert!(fixture.contains("\"seats\": [\"seat_0\", \"seat_1\", \"seat_2\", \"seat_3\"]"));

    let driver = SetupEvidenceV1Driver::new("fixture-check");
    driver
        .validate_with(&setup_evidence_profile_artifact(), |report| {
            assert_eq!(report.profile_id, SETUP_EVIDENCE_V1);
            assert_eq!(report.visibility_class, "public");

            let seats = canonical_seat_ids();
            let first = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("setup");
            let second = setup_match(Seed(1600), &seats, &SetupOptions::default()).expect("setup");
            assert_eq!(
                first.stable_internal_summary(),
                second.stable_internal_summary()
            );
            assert_eq!(first.seats, seats);
            assert_eq!(
                first
                    .hand_for_internal(briar_circuit::BriarCircuitSeat::Seat0)
                    .len(),
                13
            );
            assert_eq!(first.pass_direction().as_str(), "left");
        })
        .expect("setup-evidence-v1 driver accepts Briar standard setup adapter");
}

#[test]
fn setup_evidence_v1_driver_rejects_briar_wrong_metadata() {
    let driver = SetupEvidenceV1Driver::new("fixture-check");
    let valid = setup_evidence_profile_artifact();

    let mut wrong_version = valid.clone();
    wrong_version.metadata.profile_version = "v2";
    assert_eq!(
        driver
            .validate(&wrong_version)
            .expect_err("wrong version")
            .kind,
        ProfileValidationErrorKind::WrongProfileVersion
    );

    let mut wrong_owner = valid.clone();
    wrong_owner.metadata.validator_owner = "replay-check";
    assert_eq!(
        driver.validate(&wrong_owner).expect_err("wrong owner").kind,
        ProfileValidationErrorKind::WrongValidatorOwner
    );

    let mut unknown_field = valid;
    unknown_field.fields = &["profile_id", "deal_formula"];
    assert_eq!(
        driver
            .validate(&unknown_field)
            .expect_err("unknown field")
            .kind,
        ProfileValidationErrorKind::UnknownField
    );
}

#[test]
fn replay_export_header_has_version_anchors_and_rejects_unknown_fields() {
    let parsed = parse_export_header(
        "game_id=briar_circuit\nrules_version=briar-circuit-rules-v1\nexport_version=1\nviewer=public\nclass=public\n",
    )
    .expect("header parses");

    assert_eq!(parsed.game_id, GAME_ID);
    assert_eq!(parsed.rules_version, RULES_VERSION_LABEL);
    assert_eq!(parsed.export_version, VIEWER_EXPORT_VERSION);
    assert_eq!(TRACE_SCHEMA_VERSION, 1);

    assert!(parse_export_header(
        "game_id=briar_circuit\nrules_version=briar-circuit-rules-v1\nexport_version=1\nviewer=public\nclass=public\ndebug=bad\n",
    )
    .is_err());
    assert!(parse_export_header(
        "game_id=briar_circuit\nrules_version=briar-circuit-rules-v1\nexport_version=2\nviewer=public\nclass=public\n",
    )
    .is_err());
}
