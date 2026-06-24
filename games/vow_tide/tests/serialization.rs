use std::collections::BTreeSet;

use engine_core::Seed;
use engine_core::StableSerialize;
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind, SetupEvidenceV1Driver,
    PROFILE_VERSION_V1, PUBLIC_EXPORT_V1, SETUP_EVIDENCE_V1,
};
use vow_tide::{
    cards::{canonical_deck, Card, CardId, Rank, Suit},
    ids::{canonical_seat_ids, hand_schedule_for_seats, VowTideSeat, STANDARD_CARD_COUNT},
    replay_support::{export_for_viewer, import_viewer_export, observer},
    scoring::terminal_outcome,
    setup::{setup_match, SetupOptions},
    variants::{
        expected_manifest, load_manifest, load_variants, Manifest, Variant, VariantCatalog,
    },
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
            migration_update_note: Some("8CR4NSEAPRITRI-034 virtual setup-evidence profile"),
        },
        fields: SETUP_EVIDENCE_PROFILE_FIELDS,
        canonical_byte_claim: false,
    }
}

fn fixture_string_field(input: &str, key: &str) -> String {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .unwrap_or_else(|| panic!("missing `{key}`"))
        + needle.len();
    let tail = input[start..].trim_start();
    let tail = tail
        .strip_prefix('"')
        .unwrap_or_else(|| panic!("field `{key}` must be a string"));
    let end = tail
        .find('"')
        .unwrap_or_else(|| panic!("field `{key}` string must close"));
    tail[..end].to_owned()
}

fn fixture_number_field(input: &str, key: &str) -> usize {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .unwrap_or_else(|| panic!("missing `{key}`"))
        + needle.len();
    let digits = input[start..]
        .trim_start()
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    digits
        .parse()
        .unwrap_or_else(|error| panic!("field `{key}` must be a number: {error}"))
}

fn assert_standard_setup_fixture(input: &str, trace_id: &str, expected_seat_count: usize) {
    assert_eq!(fixture_string_field(input, "trace_id"), trace_id);
    assert_eq!(fixture_string_field(input, "fixture_kind"), "setup");
    assert_eq!(fixture_string_field(input, "game_id"), "vow_tide");
    assert_eq!(
        fixture_string_field(input, "rules_version"),
        "vow-tide-rules-v1"
    );
    assert_eq!(fixture_string_field(input, "variant"), "vow_tide_standard");
    assert_eq!(
        fixture_number_field(input, "seat_count"),
        expected_seat_count
    );
    assert!(input.contains("\"expected_trump_public\": true"));

    let seed = fixture_number_field(input, "seed") as u64;
    let seats = canonical_seat_ids(expected_seat_count);
    let state = setup_match(Seed(seed), &seats, &SetupOptions::default()).expect("setup succeeds");
    let schedule = hand_schedule_for_seats(expected_seat_count).expect("schedule exists");
    let expected_dealer =
        VowTideSeat::parse(&fixture_string_field(input, "expected_dealer")).expect("dealer");
    let expected_first_leader =
        VowTideSeat::parse(&fixture_string_field(input, "expected_first_leader"))
            .expect("first leader");

    assert_eq!(state.seat_count(), expected_seat_count);
    assert_eq!(state.hand_schedule, schedule);
    assert_eq!(state.dealer, expected_dealer);
    assert_eq!(state.active_seat(), Some(expected_first_leader));
    assert_eq!(
        state.current_hand_size(),
        Some(fixture_number_field(input, "expected_hand_size") as u8)
    );
    assert_eq!(
        state.hand_schedule.len(),
        fixture_number_field(input, "expected_hand_count")
    );
    assert_eq!(
        state.hidden_stock_internal().len(),
        fixture_number_field(input, "expected_hidden_stock_count")
    );
    assert_eq!(state.private_hands.len(), expected_seat_count);
    assert!(state
        .private_hands
        .iter()
        .all(|(_, hand)| hand.len() == state.current_hand_size().expect("hand size") as usize));
    assert_eq!(
        state.deal_order.first().copied(),
        Some(expected_first_leader)
    );
}

#[test]
fn canonical_deck_order_is_complete_and_stable() {
    let deck = canonical_deck();

    assert_eq!(deck.len(), STANDARD_CARD_COUNT as usize);
    assert_eq!(
        deck.iter().copied().collect::<BTreeSet<_>>().len(),
        deck.len()
    );
    assert_eq!(
        deck.first().copied(),
        Some(Card::new(Rank::Two, Suit::Clubs).id())
    );
    assert_eq!(
        deck.last().copied(),
        Some(Card::new(Rank::Ace, Suit::Spades).id())
    );

    for (index, card_id) in deck.iter().enumerate() {
        assert_eq!(card_id.index(), index as u8);
        assert_eq!(CardId::parse(&card_id.as_str()), Some(*card_id));
    }
}

#[test]
fn setup_evidence_v1_driver_validates_three_and_seven_seat_fixtures() {
    let driver = SetupEvidenceV1Driver::new("fixture-check");
    let profile = setup_evidence_profile_artifact();

    for (fixture, trace_id, seat_count) in [
        (
            include_str!("../data/fixtures/vow_tide_3p_standard.fixture.json"),
            "vow_tide_3p_standard",
            3,
        ),
        (
            include_str!("../data/fixtures/vow_tide_7p_standard.fixture.json"),
            "vow_tide_7p_standard",
            7,
        ),
    ] {
        assert!(!fixture.contains("\"profile_id\""));
        assert!(!fixture.contains("\"canonical_byte_authority\""));
        driver
            .validate_with(&profile, |report| {
                assert_eq!(report.profile_id, SETUP_EVIDENCE_V1);
                assert_eq!(report.visibility_class, "public");
                assert_standard_setup_fixture(fixture, trace_id, seat_count);
            })
            .expect("setup-evidence-v1 driver accepts Vow setup fixture adapter");
    }
}

#[test]
fn setup_evidence_v1_driver_rejects_vow_wrong_metadata() {
    let driver = SetupEvidenceV1Driver::new("fixture-check");
    let valid = setup_evidence_profile_artifact();

    let mut wrong_profile = valid.clone();
    wrong_profile.metadata.profile_id = PUBLIC_EXPORT_V1;
    assert_eq!(
        driver
            .validate(&wrong_profile)
            .expect_err("wrong profile")
            .kind,
        ProfileValidationErrorKind::WrongProfileId
    );

    let mut wrong_version = valid.clone();
    wrong_version.metadata.profile_version = "v2";
    assert_eq!(
        driver
            .validate(&wrong_version)
            .expect_err("wrong version")
            .kind,
        ProfileValidationErrorKind::WrongProfileVersion
    );

    let mut unknown_field = valid;
    unknown_field.fields = &["profile_id", "setup_formula"];
    assert_eq!(
        driver
            .validate(&unknown_field)
            .expect_err("unknown field")
            .kind,
        ProfileValidationErrorKind::UnknownField
    );
}

#[test]
fn setup_state_summary_is_deterministic_and_ordered() {
    let options = SetupOptions::default();
    let seats = canonical_seat_ids(5);

    let first = setup_match(Seed(42), &seats, &options).expect("first setup succeeds");
    let second = setup_match(Seed(42), &seats, &options).expect("second setup succeeds");

    assert_eq!(first, second);
    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
    assert!(first
        .stable_internal_summary()
        .contains("schedule=[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"));
}

#[test]
fn metadata_stubs_load_inert_standard_content() {
    let manifest = load_manifest().expect("manifest parses");
    let variants = load_variants().expect("variants parse");

    assert_eq!(manifest, expected_manifest());
    assert_eq!(variants.selected, Variant::vow_tide_standard());
}

#[test]
fn metadata_rejects_unknown_and_behavior_looking_fields() {
    let manifest = include_str!("../data/manifest.toml");
    let unknown_manifest = format!("{manifest}\nunknown_field = \"not allowed\"\n");
    let behavior_manifest = format!("{manifest}\nscore_formula = \"10 + bid\"\n");
    assert!(Manifest::parse(&unknown_manifest)
        .expect_err("unknown field rejected")
        .contains("unknown field"));
    assert!(Manifest::parse(&behavior_manifest)
        .expect_err("behavior field rejected")
        .contains("behavior-looking field"));

    let variants = include_str!("../data/variants.toml");
    let unknown_variants = format!("{variants}\nunknown_field = \"not allowed\"\n");
    let behavior_variants = format!("{variants}\nbot_policy = \"peek\"\n");
    assert!(VariantCatalog::parse(&unknown_variants)
        .expect_err("unknown field rejected")
        .contains("unknown field"));
    assert!(VariantCatalog::parse(&behavior_variants)
        .expect_err("behavior field rejected")
        .contains("behavior-looking field"));
}

#[test]
fn terminal_outcome_order_is_deterministic_for_serialization() {
    let options = SetupOptions::default();
    let seats = canonical_seat_ids(4);
    let mut state = setup_match(Seed(5), &seats, &options).expect("setup succeeds");
    state.cumulative_scores = vec![
        (vow_tide::ids::VowTideSeat::Seat0, 10),
        (vow_tide::ids::VowTideSeat::Seat1, 30),
        (vow_tide::ids::VowTideSeat::Seat2, 30),
        (vow_tide::ids::VowTideSeat::Seat3, 0),
    ];

    let first = terminal_outcome(&state);
    let second = terminal_outcome(&state);

    assert_eq!(first, second);
    assert_eq!(
        first
            .standings
            .iter()
            .map(|standing| (standing.seat.as_str(), standing.rank))
            .collect::<Vec<_>>(),
        vec![("seat_1", 1), ("seat_2", 1), ("seat_0", 3), ("seat_3", 4)]
    );
}

#[test]
fn viewer_export_serialization_is_stable_and_versioned() {
    let options = SetupOptions::default();
    let seats = canonical_seat_ids(4);
    let state = setup_match(Seed(17), &seats, &options).expect("setup succeeds");
    let export = export_for_viewer(&state, &[], &observer());
    let imported = import_viewer_export(&export).expect("import succeeds");

    assert_eq!(export, imported);
    assert_eq!(export.schema_version, 1);
    assert_eq!(export.game_id, "vow_tide");
    assert_eq!(export.stable_bytes(), imported.stable_bytes());
}
