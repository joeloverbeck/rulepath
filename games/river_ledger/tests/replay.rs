use engine_core::{HashValue, SeatId, StableSerialize, Viewer};
use game_test_support::no_leak::{assert_pairwise_no_leak, ExposureExpectation, LeakProbe};
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind, PublicExportV1Driver,
    ReplayCommandV1Driver, SetupEvidenceV1Driver, PROFILE_VERSION_V1, PUBLIC_EXPORT_V1,
    REPLAY_COMMAND_V1, SETUP_EVIDENCE_V1,
};
use river_ledger::{
    replay_support::{
        export_public_replay, import_public_export, replay_internal_full_trace,
        replay_internal_full_trace_result, trace_from_commands,
    },
    setup_match, PotShare, RiverLedgerSeat, SetupOptions, TerminalOutcome,
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

const REPLAY_COMMAND_PROFILE_FIELDS: &[&str] = &[
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
    "commands",
    "checkpoints",
    "expected_hashes",
];

const PUBLIC_EXPORT_PROFILE_FIELDS: &[&str] = &[
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
    "export_steps",
    "import_round_trip",
    "hidden_absence_tokens",
];

const FOUR_PLAYER_CHECKDOWN: &[(usize, &str)] = &[
    (3, "call"),
    (0, "call"),
    (1, "call"),
    (2, "check"),
    (1, "check"),
    (2, "check"),
    (3, "check"),
    (0, "check"),
    (1, "check"),
    (2, "check"),
    (3, "check"),
    (0, "check"),
    (1, "check"),
    (2, "check"),
    (3, "check"),
    (0, "check"),
];

const GATE_15_1_GOLDEN_TRACE_FILES: &[&str] = &[
    "setup-equal-default-stacks-3p.trace.json",
    "setup-asymmetric-stacks-6p.trace.json",
    "short-small-blind-all-in.trace.json",
    "short-big-blind-all-in.trace.json",
    "call-all-in-below-price.trace.json",
    "exact-call-exhausts-stack.trace.json",
    "short-open-bet-all-in.trace.json",
    "short-raise-all-in.trace.json",
    "cumulative-reopen.trace.json",
    "full-all-in-raise.trace.json",
    "cap-blocks-short-raise.trace.json",
    "three-way-main-two-side-pots.trace.json",
    "folded-contribution-retained.trace.json",
    "uncalled-return.trace.json",
    "sole-eligible-pot.trace.json",
    "different-winners-across-pots.trace.json",
    "tied-winners-in-pot.trace.json",
    "per-pot-remainder-button-order.trace.json",
    "all-all-in-runout.trace.json",
    "public-observer-multipot-no-leak.trace.json",
    "seat-private-multipot-no-leak.trace.json",
    "wasm-exported-side-pot-terminal.trace.json",
];

fn hidden_ids(seed: u64, seat_count: usize) -> Vec<String> {
    let seats = (0..seat_count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect::<Vec<_>>();
    let state =
        setup_match(engine_core::Seed(seed), &seats, &SetupOptions::default()).expect("setup");
    state
        .private_hands_internal()
        .iter()
        .flatten()
        .chain(state.community_deck_internal().iter())
        .chain(state.deck_tail_internal().iter())
        .map(|card| card.id())
        .collect()
}

fn private_ids_by_seat(seed: u64, seat_count: usize) -> Vec<Vec<String>> {
    let seats = (0..seat_count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect::<Vec<_>>();
    let state =
        setup_match(engine_core::Seed(seed), &seats, &SetupOptions::default()).expect("setup");
    state
        .private_hands_internal()
        .iter()
        .map(|hand| hand.iter().map(|card| card.id()).collect())
        .collect()
}

fn future_ids(seed: u64, seat_count: usize) -> Vec<String> {
    let seats = (0..seat_count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect::<Vec<_>>();
    let state =
        setup_match(engine_core::Seed(seed), &seats, &SetupOptions::default()).expect("setup");
    state
        .community_deck_internal()
        .iter()
        .chain(state.deck_tail_internal().iter())
        .map(|card| card.id())
        .collect()
}

fn read_golden_trace(file_name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden_traces")
        .join(file_name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read golden trace {}: {error}", path.display()))
}

fn expected_trace_id(file_name: &str) -> String {
    format!(
        "river-ledger-{}",
        file_name
            .strip_suffix(".trace.json")
            .expect("golden trace suffix")
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum ExportViewer {
    Observer,
    Seat(usize),
}

impl ExportViewer {
    fn as_viewer(self) -> Viewer {
        match self {
            ExportViewer::Observer => Viewer { seat_id: None },
            ExportViewer::Seat(index) => Viewer {
                seat_id: Some(SeatId(format!("seat_{index}"))),
            },
        }
    }

    const fn seat(self) -> Option<usize> {
        match self {
            ExportViewer::Observer => None,
            ExportViewer::Seat(index) => Some(index),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum ExportSurface {
    PublicReplayExport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum ExportCanary {
    PrivateHand,
    FutureCard,
}

fn export_viewers(seat_count: usize) -> Vec<ExportViewer> {
    let mut viewers = vec![ExportViewer::Observer];
    viewers.extend((0..seat_count).map(ExportViewer::Seat));
    viewers
}

fn export_probes(seed: u64, seat_count: usize) -> Vec<LeakProbe<usize, ExportCanary, String>> {
    let mut probes = Vec::new();
    for (source, private_ids) in private_ids_by_seat(seed, seat_count)
        .into_iter()
        .enumerate()
    {
        probes.push(LeakProbe {
            source_seat: source,
            canary_id: ExportCanary::PrivateHand,
            canary: private_ids.first().cloned().expect("private card"),
        });
    }
    probes.push(LeakProbe {
        source_seat: 0,
        canary_id: ExportCanary::FutureCard,
        canary: future_ids(seed, seat_count)
            .first()
            .cloned()
            .expect("future card"),
    });
    probes
}

fn export_expectation(
    source: &usize,
    viewer: &ExportViewer,
    _surface: &ExportSurface,
    canary: &ExportCanary,
) -> ExposureExpectation {
    match canary {
        ExportCanary::FutureCard => ExposureExpectation::MustBeAbsent,
        ExportCanary::PrivateHand => {
            if viewer.seat() == Some(*source) {
                ExposureExpectation::MustBePresent
            } else {
                ExposureExpectation::MustBeAbsent
            }
        }
    }
}

fn setup_profile_artifact() -> ProfileArtifact<'static> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: SETUP_EVIDENCE_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("public"),
            validator_owner: "fixture-check",
            canonical_byte_authority: "none",
            migration_update_note: Some("UNI8CMECSCA-024 virtual setup-evidence classification"),
        },
        fields: SETUP_EVIDENCE_PROFILE_FIELDS,
        canonical_byte_claim: false,
    }
}

fn replay_command_profile_artifact() -> ProfileArtifact<'static> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: REPLAY_COMMAND_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("internal-dev"),
            validator_owner: "replay-check",
            canonical_byte_authority: "none",
            migration_update_note: Some("8CR4NSEAPRITRI-025 virtual replay-command profile"),
        },
        fields: REPLAY_COMMAND_PROFILE_FIELDS,
        canonical_byte_claim: false,
    }
}

fn public_export_profile_artifact() -> ProfileArtifact<'static> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: PUBLIC_EXPORT_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("public"),
            validator_owner: "wasm-export",
            canonical_byte_authority: "none",
            migration_update_note: Some("8CR4NSEAPRITRI-026 virtual public-export profile"),
        },
        fields: PUBLIC_EXPORT_PROFILE_FIELDS,
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

fn assert_standard_3p_setup_fixture(input: &str) {
    assert_eq!(
        fixture_string_field(input, "fixture_id"),
        "river_ledger_3p_standard"
    );
    assert_eq!(fixture_string_field(input, "game_id"), "river_ledger");
    assert_eq!(
        fixture_string_field(input, "variant"),
        "river_ledger_standard"
    );
    assert_eq!(
        fixture_string_field(input, "rules_version"),
        "river-ledger-rules-v1"
    );
    assert_eq!(fixture_number_field(input, "seat_count"), 3);

    let seed = fixture_number_field(input, "seed") as u64;
    let seats = (0..fixture_number_field(input, "seat_count"))
        .map(|index| SeatId(format!("seat_{index}")))
        .collect::<Vec<_>>();
    let state =
        setup_match(engine_core::Seed(seed), &seats, &SetupOptions::default()).expect("setup");

    assert_eq!(state.seats.len(), fixture_number_field(input, "seat_count"));
    assert_eq!(state.button.as_str(), fixture_string_field(input, "button"));
    assert_eq!(
        state.small_blind.as_str(),
        fixture_string_field(input, "small_blind")
    );
    assert_eq!(
        state.big_blind.as_str(),
        fixture_string_field(input, "big_blind")
    );
    assert_eq!(
        state.active_seat.map(|seat| seat.as_str()),
        Some(fixture_string_field(input, "initial_active"))
    );
    assert_eq!(
        state.board.len(),
        fixture_number_field(input, "public_board_count")
    );
    assert_eq!(
        state.private_hands_internal()[0].len(),
        fixture_number_field(input, "private_hole_cards_per_seat")
    );
    assert_eq!(
        state.community_deck_internal().len(),
        fixture_number_field(input, "reserved_community_count")
    );
    assert_eq!(
        state.deck_tail_internal().len(),
        fixture_number_field(input, "deck_tail_count")
    );
}

#[test]
fn setup_evidence_v1_driver_validates_standard_3p_setup_fixture() {
    let setup_fixture = include_str!("../data/fixtures/river_ledger_3p_standard.fixture.json");
    assert!(!setup_fixture.contains("\"profile_id\""));
    assert!(!setup_fixture.contains("\"canonical_byte_authority\""));
    let driver = SetupEvidenceV1Driver::new("fixture-check");
    let profile = setup_profile_artifact();

    driver
        .validate_with(&profile, |_| {
            assert_standard_3p_setup_fixture(setup_fixture)
        })
        .expect("setup-evidence-v1 driver accepts River setup fixture");
}

#[test]
fn replay_command_v1_driver_validates_river_internal_command_traces() {
    let trace = trace_from_commands(21, 4, FOUR_PLAYER_CHECKDOWN);
    let result = replay_internal_full_trace(&trace);
    let driver = ReplayCommandV1Driver::new("replay-check");
    let profile = replay_command_profile_artifact();

    let delegated = driver
        .validate_with(&profile, |report| {
            assert_eq!(report.profile_id, REPLAY_COMMAND_V1);
            assert_eq!(report.visibility_class, "internal-dev");
            assert_eq!(
                result.trace_hash,
                replay_internal_full_trace(&trace).trace_hash
            );
            format!("{}:river-ledger", report.profile_id)
        })
        .expect("replay-command-v1 driver accepts River internal trace adapter");

    assert_eq!(delegated, "replay-command-v1:river-ledger");

    for file_name in [
        "all-all-in-runout.trace.json",
        "three-way-main-two-side-pots.trace.json",
        "seat-private-multipot-no-leak.trace.json",
    ] {
        let contents = read_golden_trace(file_name);
        assert!(
            !contents.contains("\"profile_id\""),
            "{file_name} must not be rewritten with profile metadata"
        );
        assert!(
            !contents.contains("\"canonical_byte_authority\""),
            "{file_name} must keep legacy replay trace bytes authoritative"
        );
        driver
            .validate_with(&profile, |_| {
                assert!(contents.contains("\"schema_version\": 1"));
                assert!(contents.contains("\"rules_version\""));
            })
            .expect("virtual replay-command profile validates selected golden trace");
    }
}

#[test]
fn replay_command_v1_driver_rejects_wrong_metadata_without_reading_commands() {
    let driver = ReplayCommandV1Driver::new("replay-check");
    let valid = replay_command_profile_artifact();

    let mut wrong_profile = valid.clone();
    wrong_profile.metadata.profile_id = SETUP_EVIDENCE_V1;
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

    let mut unknown_field = valid.clone();
    unknown_field.fields = &["profile_id", "commands", "procedural_setup"];
    assert_eq!(
        driver
            .validate(&unknown_field)
            .expect_err("unknown field")
            .kind,
        ProfileValidationErrorKind::UnknownField
    );

    let mut byte_claim = valid;
    byte_claim.canonical_byte_claim = true;
    assert_eq!(
        driver
            .validate(&byte_claim)
            .expect_err("canonical byte claim")
            .kind,
        ProfileValidationErrorKind::IllegalCanonicalByteClaim
    );
}

#[test]
fn public_export_v1_driver_validates_observer_round_trip_and_no_leak() {
    let driver = PublicExportV1Driver::new("wasm-export");
    let profile = public_export_profile_artifact();
    let cases = [
        (
            "public-replay-export-import.trace.json",
            trace_from_commands(21, 4, &[(3, "call"), (0, "call")]),
            21,
            4,
        ),
        (
            "three-way-main-two-side-pots.trace.json",
            trace_from_commands(1521, 3, &[]),
            1521,
            3,
        ),
    ];

    for (file_name, trace, seed, seat_count) in cases {
        let fixture = read_golden_trace(file_name);
        assert!(
            !fixture.contains("\"profile_id\""),
            "{file_name} must not embed public-export profile metadata"
        );
        let delegated_hash = driver
            .validate_with(&profile, |report| {
                assert_eq!(report.profile_id, PUBLIC_EXPORT_V1);
                assert_eq!(report.visibility_class, "public");

                let export = export_public_replay(&trace, &Viewer { seat_id: None });
                let imported = import_public_export(&export);
                assert_eq!(imported.viewer, "observer");
                assert_eq!(imported.steps, export.steps);

                let json = export.to_json();
                for hidden in hidden_ids(seed, seat_count) {
                    assert!(!json.contains(&hidden), "{file_name} leaked {hidden}");
                }
                assert!(!json.contains("seed_evidence"));
                assert!(!json.contains("\"seed\""));
                export.stable_hash()
            })
            .expect("public-export-v1 driver accepts River observer export adapter");

        assert_eq!(
            delegated_hash,
            export_public_replay(&trace, &Viewer { seat_id: None }).stable_hash()
        );
    }
}

#[test]
fn public_export_v1_driver_rejects_wrong_metadata() {
    let driver = PublicExportV1Driver::new("wasm-export");
    let valid = public_export_profile_artifact();

    let mut wrong_owner = valid.clone();
    wrong_owner.metadata.validator_owner = "replay-check";
    assert_eq!(
        driver.validate(&wrong_owner).expect_err("wrong owner").kind,
        ProfileValidationErrorKind::WrongValidatorOwner
    );

    let mut wrong_visibility = valid.clone();
    wrong_visibility.metadata.visibility_class = Some("seat-private");
    assert_eq!(
        driver
            .validate(&wrong_visibility)
            .expect_err("wrong visibility")
            .kind,
        ProfileValidationErrorKind::InvalidVisibility
    );

    let mut unknown_field = valid;
    unknown_field.fields = &["profile_id", "viewer_seat"];
    assert_eq!(
        driver
            .validate(&unknown_field)
            .expect_err("unknown field")
            .kind,
        ProfileValidationErrorKind::UnknownField
    );
}

#[test]
fn internal_trace_replays_to_same_hashes_and_state() {
    let trace = trace_from_commands(
        21,
        4,
        &[(3, "call"), (0, "call"), (1, "call"), (2, "check")],
    );
    let first = replay_internal_full_trace(&trace);
    let second = replay_internal_full_trace(&trace);

    assert_eq!(first.trace_hash, second.trace_hash);
    assert_eq!(first.state_hash, second.state_hash);
    assert_eq!(first.effect_hash, second.effect_hash);
    assert_eq!(first.view_hash, second.view_hash);
    assert_eq!(first.action_tree_hashes, second.action_tree_hashes);
    assert_eq!(
        first.final_state.stable_internal_summary(),
        second.final_state.stable_internal_summary()
    );
}

#[test]
fn gate_15_1_golden_trace_set_is_present_and_reviewed() {
    assert_eq!(GATE_15_1_GOLDEN_TRACE_FILES.len(), 22);

    let mut combined = String::new();
    for file_name in GATE_15_1_GOLDEN_TRACE_FILES {
        let contents = read_golden_trace(file_name);
        let trace_id = expected_trace_id(file_name);

        assert!(
            contents.contains(&format!("\"trace_id\": \"{trace_id}\"")),
            "{file_name} must use its canonical trace_id"
        );
        assert!(
            contents.contains("\"schema_version\": 1"),
            "{file_name} must stay on the replay-check schema"
        );
        assert!(
            contents.contains("\"rules_version\": \"river-ledger-rules-v1\""),
            "{file_name} must declare the current replay-check rule version"
        );
        assert!(
            contents.contains("\"migration_review\""),
            "{file_name} must carry an individual v2 migration review note"
        );
        assert!(
            contents.contains("\"expected_public_result\"")
                || contents.contains("\"expected_public_setup\"")
                || contents.contains("\"expected_diagnostics\"")
                || contents.contains("\"forbidden_public_facts\"")
                || contents.contains("\"forbidden_cross_seat_facts\"")
                || contents.contains("\"public_export\""),
            "{file_name} must record a reviewable public expectation"
        );

        combined.push_str(&contents);
        combined.push('\n');
    }

    for required_marker in [
        "\"starting_stacks\": [24, 24, 24]",
        "\"starting_stacks\": [4, 8, 12, 16, 20, 24]",
        "\"starting_stacks\": [8, 3, 2]",
        "\"starting_stacks\": [2, 5, 9]",
        "\"raise_cap_reached\"",
        "\"reopen_after_full_unit_pressure\"",
        "\"is_full_raise\"",
        "\"folded_contribution_retained\"",
        "\"returned\"",
        "\"sole_eligible_pot\"",
        "\"pot_winners\"",
        "\"odd_units_in_multiple_pots\"",
        "\"viewer\": \"observer\"",
        "\"viewer\": \"seat_0\"",
        "\"public_export\"",
        "\"terminal\"",
    ] {
        assert!(
            combined.contains(required_marker),
            "Gate 15.1 trace set is missing marker {required_marker}"
        );
    }
}

#[test]
fn v1_internal_trace_is_rejected_with_stable_diagnostic() {
    let mut trace = trace_from_commands(21, 4, &[(3, "call")]);
    trace.rules_version = "river-ledger-rules-v1".to_owned();

    let diagnostic = replay_internal_full_trace_result(&trace).expect_err("v1 rejects");

    assert_eq!(diagnostic.code, "river_ledger_rules_version_mismatch");
    assert_eq!(
        diagnostic.message,
        "River Ledger replay uses river-ledger-rules-v1; expected river-ledger-rules-v2"
    );
}

#[test]
fn public_export_import_round_trips_for_observer_and_seat_viewer() {
    let trace = trace_from_commands(21, 4, &[(3, "call"), (0, "call")]);
    for viewer in [
        Viewer { seat_id: None },
        Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        },
    ] {
        let export = export_public_replay(&trace, &viewer);
        let imported = import_public_export(&export);

        assert_eq!(imported.viewer, export.viewer);
        assert_eq!(imported.steps, export.steps);
        assert_eq!(
            HashValue::from_stable_bytes(export.to_json().as_bytes()),
            export.stable_hash()
        );
    }
}

#[test]
fn characterization_setup_export_and_visibility_artifacts_are_pinned() {
    let setup_fixture = include_str!("../data/fixtures/river_ledger_3p_standard.fixture.json");
    let public_trace = include_str!("golden_traces/public-replay-export-import.trace.json");
    let seat_private_trace = include_str!("golden_traces/seat-private-view.trace.json");
    let trace = trace_from_commands(21, 3, &[]);
    let observer_export = export_public_replay(&trace, &Viewer { seat_id: None });
    let seat_export = export_public_replay(
        &trace,
        &Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        },
    );

    assert_eq!(
        HashValue::from_stable_bytes(setup_fixture.as_bytes()),
        HashValue(2633580370171550625)
    );
    assert_eq!(
        HashValue::from_stable_bytes(public_trace.as_bytes()),
        HashValue(11946834064931283956)
    );
    assert_eq!(
        HashValue::from_stable_bytes(seat_private_trace.as_bytes()),
        HashValue(6382002720248622821)
    );
    assert_eq!(
        observer_export.stable_hash(),
        HashValue(2482097568303728278)
    );
    assert_eq!(seat_export.stable_hash(), HashValue(7443748736294317283));
    assert_eq!(observer_export.viewer, "observer");
    assert_eq!(seat_export.viewer, "seat_0");
}

#[test]
fn observer_public_export_omits_hidden_facts_and_seed() {
    let trace = trace_from_commands(21, 4, &[(3, "call"), (0, "call")]);
    let export = export_public_replay(&trace, &Viewer { seat_id: None });
    let json = export.to_json();

    for hidden in hidden_ids(21, 4) {
        assert!(!json.contains(&hidden), "public export leaked {hidden}");
    }
    assert!(!json.contains("seed_evidence"));
    assert!(!json.contains("\"seed\""));
}

#[test]
fn runout_and_multipot_exports_hide_future_and_cross_seat_cards() {
    let cases = [
        (
            "all-all-in-runout.trace.json",
            1528,
            3,
            vec![(0, "raise"), (1, "call")],
        ),
        (
            "three-way-main-two-side-pots.trace.json",
            1521,
            3,
            Vec::new(),
        ),
        (
            "uncalled-return.trace.json",
            1523,
            3,
            vec![(0, "raise"), (1, "call")],
        ),
        (
            "public-observer-multipot-no-leak.trace.json",
            1530,
            3,
            Vec::new(),
        ),
        (
            "seat-private-multipot-no-leak.trace.json",
            1530,
            3,
            Vec::new(),
        ),
    ];

    for (file_name, seed, seat_count, commands) in cases {
        let fixture = read_golden_trace(file_name);
        assert!(fixture.contains(&format!(
            "\"trace_id\": \"{}\"",
            expected_trace_id(file_name)
        )));
        let trace = trace_from_commands(seed, seat_count, &commands);

        assert_pairwise_no_leak(
            export_viewers(seat_count),
            [ExportSurface::PublicReplayExport],
            export_probes(seed, seat_count),
            |viewer, _surface| export_public_replay(&trace, &viewer.as_viewer()).to_json(),
            export_expectation,
            |snapshot, canary| snapshot.contains(canary),
        )
        .unwrap_or_else(|failures| {
            panic!("{file_name} River Ledger export no-leak matrix: {failures}")
        });

        let observer_json = export_public_replay(&trace, &Viewer { seat_id: None }).to_json();
        assert!(
            observer_json.contains("pot")
                || observer_json.contains("setup")
                || observer_json.contains("showdown"),
            "{file_name} keeps public accounting/export steps visible"
        );
    }
}

#[test]
fn terminal_public_export_keeps_v2_showdown_surface_public_and_deterministic() {
    let trace = trace_from_commands(79, 4, FOUR_PLAYER_CHECKDOWN);
    let first = export_public_replay(&trace, &Viewer { seat_id: None });
    let second = export_public_replay(&trace, &Viewer { seat_id: None });
    let json = first.to_json();

    assert_eq!(first.steps, second.steps);
    assert!(json.contains("wins with"));
    assert!(json.contains("showdown:"));
    assert!(!json.contains("private_hands"));
    assert!(!json.contains("seed_evidence"));
    assert!(!json.contains("\"seed\""));
}

#[test]
fn seed_10018_public_replay_uses_one_based_unique_winner_label() {
    let trace = trace_from_commands(10018, 4, FOUR_PLAYER_CHECKDOWN);
    let result = replay_internal_full_trace(&trace);
    let TerminalOutcome::Showdown {
        winners,
        allocations,
        headline,
        presentation_v2,
        ..
    } = result
        .final_state
        .terminal_outcome
        .as_ref()
        .expect("terminal outcome")
    else {
        panic!("showdown terminal expected");
    };

    assert_eq!(winners, &vec![RiverLedgerSeat::from_index(0).unwrap()]);
    assert_eq!(
        allocations,
        &vec![PotShare {
            seat: RiverLedgerSeat::from_index(0).unwrap(),
            amount: result.final_state.ledger.pot_total,
        }]
    );
    assert_eq!(headline, "Seat 1 wins with Two pair, Queens and Fives.");
    assert_eq!(presentation_v2.standings[0].seat_label, "Seat 1");

    let export = export_public_replay(&trace, &Viewer { seat_id: None });
    let json = export.to_json();
    assert!(json.contains("Seat 1 wins with Two pair, Queens and Fives."));
    assert!(!json.contains("Seat 0 wins"));
    assert!(!json.contains("seat_0 wins"));
}

#[test]
fn seed_31_public_replay_keeps_split_winners_in_canonical_order() {
    let trace = trace_from_commands(31, 4, FOUR_PLAYER_CHECKDOWN);
    let result = replay_internal_full_trace(&trace);
    let TerminalOutcome::Showdown {
        winners,
        allocations,
        headline,
        presentation_v2,
        ..
    } = result
        .final_state
        .terminal_outcome
        .as_ref()
        .expect("terminal outcome")
    else {
        panic!("showdown terminal expected");
    };

    assert_eq!(
        winners,
        &vec![
            RiverLedgerSeat::from_index(1).unwrap(),
            RiverLedgerSeat::from_index(2).unwrap(),
            RiverLedgerSeat::from_index(3).unwrap(),
        ]
    );
    assert_eq!(
        allocations
            .iter()
            .map(|share| share.seat)
            .collect::<Vec<_>>(),
        *winners
    );
    assert!(headline.starts_with("Seat 2, Seat 3, and Seat 4 split the ledger"));
    assert_eq!(
        presentation_v2
            .standings
            .iter()
            .filter(|standing| standing.result_label == "Split win")
            .map(|standing| standing.seat)
            .collect::<Vec<_>>(),
        *winners
    );
}
