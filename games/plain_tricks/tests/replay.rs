use engine_core::{
    ActionPath, Actor, CommandEnvelope, FreshnessToken, HashValue, RulesVersion, SeatId,
    StableSerialize, Viewer,
};
use game_test_support::no_leak::{assert_pairwise_no_leak, ExposureExpectation, LeakProbe};
use game_test_support::profiles::{
    DomainEvidenceV1Driver, ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind,
    ReplayCommandV1Driver, SetupEvidenceV1Driver, DOMAIN_EVIDENCE_V1, PROFILE_VERSION_V1,
    REPLAY_COMMAND_V1, SETUP_EVIDENCE_V1,
};
use plain_tricks::{
    apply_action, legal_action_tree, load_standard_fixture,
    replay_support::{
        action_tree_hash, action_tree_v1_bytes, action_tree_v1_hash, effect_hash,
        export_public_replay, import_public_export, state_hash,
        trace_from_seeded_first_legal_playout, view_hash, PlainTricksInternalTrace, ReplayCommand,
    },
    setup_effects, setup_match, validate_command, Phase, PlainTricksSeat, PlainTricksState,
    SetupOptions, TerminalOutcome, TrickCardId, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID,
};

#[derive(Debug)]
struct TraceFixture {
    id: String,
    purpose: String,
    seed: u64,
    commands: Vec<TraceCommand>,
    expected_state_hash: u64,
    expected_effect_hash: u64,
    expected_action_tree_hash: u64,
    expected_observer_view_hash: u64,
    expected_seat_0_view_hash: u64,
    expected_seat_1_view_hash: u64,
    expected_replay_hash: u64,
    expected_public_export_hash: u64,
    expected_diagnostic_hash: Option<u64>,
    expected_diagnostic_code: Option<String>,
    terminal: bool,
    winner: Option<String>,
    draw: bool,
}

#[derive(Debug)]
struct TraceCommand {
    actor_seat: PlainTricksSeat,
    action_path: Vec<String>,
    freshness_token: u64,
    expect: String,
    expected_diagnostic_code: Option<String>,
}

#[derive(Debug)]
struct ReplayActual {
    state_hash: HashValue,
    effect_hash: HashValue,
    action_tree_hash: HashValue,
    observer_view_hash: HashValue,
    seat_0_view_hash: HashValue,
    seat_1_view_hash: HashValue,
    replay_hash: HashValue,
    public_export_hash: HashValue,
    diagnostic_hash: Option<HashValue>,
    diagnostic_code: Option<String>,
    terminal: bool,
    outcome: Option<TerminalOutcome>,
    export_json: String,
}

const GOLDEN_TRACE_INPUTS: [&str; 16] = [
    include_str!("golden_traces/deal-private-no-leak.trace.json"),
    include_str!("golden_traces/follow-suit-forced.trace.json"),
    include_str!("golden_traces/void-free-discard.trace.json"),
    include_str!("golden_traces/off-suit-never-wins.trace.json"),
    include_str!("golden_traces/trick-winner-leads-next.trace.json"),
    include_str!("golden_traces/round-close-deal-rotation.trace.json"),
    include_str!("golden_traces/terminal-most-points-win.trace.json"),
    include_str!("golden_traces/tie-split.trace.json"),
    include_str!("golden_traces/no-leak-public-observer.trace.json"),
    include_str!("golden_traces/seat-private-view.trace.json"),
    include_str!("golden_traces/invalid-wrong-seat-diagnostic.trace.json"),
    include_str!("golden_traces/invalid-stale-diagnostic.trace.json"),
    include_str!("golden_traces/invalid-must-follow-diagnostic.trace.json"),
    include_str!("golden_traces/bot-action.trace.json"),
    include_str!("golden_traces/public-replay-export-import.trace.json"),
    include_str!("golden_traces/wasm-exported.trace.json"),
];

#[test]
fn golden_traces_match_expected_replay_hashes_diagnostics_and_no_leak_surfaces() {
    for input in GOLDEN_TRACE_INPUTS {
        let fixture = parse_trace_fixture(input);
        assert_trace_fixture(&fixture);
    }
}

#[test]
fn replay_command_v1_profile_driver_wraps_native_replay_validator() {
    let driver = ReplayCommandV1Driver::new("plain_tricks");
    let artifact = replay_command_profile_artifact(
        REPLAY_COMMAND_V1,
        PROFILE_VERSION_V1,
        Some("internal-dev"),
        "plain_tricks",
        &["commands", "checkpoints", "expected_hashes"],
    );

    let report = driver
        .validate(&artifact)
        .expect("profile metadata validates");
    assert_eq!(report.profile_id, REPLAY_COMMAND_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "internal-dev");
    assert_eq!(report.validator_owner, "plain_tricks");
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let checked = driver
        .validate_with(&artifact, |_| {
            GOLDEN_TRACE_INPUTS
                .iter()
                .map(|input| {
                    let fixture = parse_trace_fixture(input);
                    assert_trace_fixture(&fixture);
                    fixture.id
                })
                .collect::<Vec<_>>()
        })
        .expect("profile delegates to native replay validator");
    assert_eq!(checked.len(), GOLDEN_TRACE_INPUTS.len());

    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            "public-export-v1",
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "plain_tricks",
            &["commands"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            "v2",
            Some("internal-dev"),
            "plain_tricks",
            &["commands"],
        ),
        ProfileValidationErrorKind::WrongProfileVersion,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("seat-private"),
            "plain_tricks",
            &["commands"],
        ),
        ProfileValidationErrorKind::InvalidVisibility,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "replay-check",
            &["commands"],
        ),
        ProfileValidationErrorKind::WrongValidatorOwner,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "plain_tricks",
            &["commands", "export_steps"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn setup_evidence_v1_profile_driver_wraps_standard_fixture_validator() {
    let driver = SetupEvidenceV1Driver::new("fixture-check");
    let artifact = setup_evidence_profile_artifact(
        SETUP_EVIDENCE_V1,
        PROFILE_VERSION_V1,
        Some("internal-dev"),
        "fixture-check",
        &["seat_grammar_version", "setup_options", "expected_setup"],
    );

    let report = driver
        .validate(&artifact)
        .expect("setup metadata validates");
    assert_eq!(report.profile_id, SETUP_EVIDENCE_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "internal-dev");
    assert_eq!(report.validator_owner, "fixture-check");
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let summary = driver
        .validate_with(&artifact, |_| validate_standard_setup_fixture())
        .expect("profile delegates to setup fixture validator");
    assert_eq!(summary.variant, VARIANT_ID);
    assert_eq!(summary.hand_count_per_seat, 6);
    assert_eq!(summary.tail_count, 6);

    assert_setup_profile_rejects(
        setup_evidence_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "fixture-check",
            &["expected_setup"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_setup_profile_rejects(
        setup_evidence_profile_artifact(
            SETUP_EVIDENCE_V1,
            "v2",
            Some("internal-dev"),
            "fixture-check",
            &["expected_setup"],
        ),
        ProfileValidationErrorKind::WrongProfileVersion,
    );
    assert_setup_profile_rejects(
        setup_evidence_profile_artifact(
            SETUP_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("private-source"),
            "fixture-check",
            &["expected_setup"],
        ),
        ProfileValidationErrorKind::InvalidVisibility,
    );
    assert_setup_profile_rejects(
        setup_evidence_profile_artifact(
            SETUP_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "plain_tricks",
            &["expected_setup"],
        ),
        ProfileValidationErrorKind::WrongValidatorOwner,
    );
    assert_setup_profile_rejects(
        setup_evidence_profile_artifact(
            SETUP_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "fixture-check",
            &["expected_setup", "domain_input"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn domain_evidence_v1_profile_driver_wraps_plain_tricks_domain_validator() {
    let driver = DomainEvidenceV1Driver::new("plain_tricks");
    let artifact = domain_evidence_profile_artifact(
        DOMAIN_EVIDENCE_V1,
        PROFILE_VERSION_V1,
        Some("internal-dev"),
        "plain_tricks",
        &["domain_schema_version", "domain_input", "expected_domain"],
    );

    let report = driver
        .validate(&artifact)
        .expect("domain metadata validates");
    assert_eq!(report.profile_id, DOMAIN_EVIDENCE_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "internal-dev");
    assert_eq!(report.validator_owner, "plain_tricks");
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let summary = driver
        .validate_with(&artifact, |_| validate_plain_tricks_domain_evidence())
        .expect("profile delegates to Plain Tricks domain validator");
    assert_eq!(summary.deck_size, 18);
    assert_eq!(summary.hand_count_per_seat, 6);
    assert_eq!(summary.tail_count, 6);
    assert_eq!(summary.round_one_completed_tricks, 6);
    assert_eq!(summary.terminal_winner, Some("seat_1"));
    assert_eq!(summary.terminal_split_each, Some(6));

    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            SETUP_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "plain_tricks",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            "v2",
            Some("internal-dev"),
            "plain_tricks",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::WrongProfileVersion,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("unsupported"),
            "plain_tricks",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::InvalidVisibility,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "fixture-check",
            &["expected_domain"],
        ),
        ProfileValidationErrorKind::WrongValidatorOwner,
    );
    assert_domain_profile_rejects(
        domain_evidence_profile_artifact(
            DOMAIN_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "plain_tricks",
            &["expected_domain", "setup_options"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn internal_trace_replays_to_the_same_hashes_and_terminal() {
    let trace = trace_from_seeded_first_legal_playout(0);
    let first = plain_tricks::replay_support::replay_internal_full_trace(&trace);
    let second = plain_tricks::replay_support::replay_internal_full_trace(&trace);

    assert_eq!(first.trace_hash, second.trace_hash);
    assert_eq!(first.state_hash, second.state_hash);
    assert_eq!(first.effect_hash, second.effect_hash);
    assert_eq!(first.view_hash, second.view_hash);
    assert_eq!(first.action_tree_hashes, second.action_tree_hashes);
    assert_eq!(first.final_state.phase, Phase::Terminal);
}

#[test]
fn public_export_import_round_trips_for_observer_and_seat_viewer() {
    let trace = trace_from_seeded_first_legal_playout(0);
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
            plain_tricks::replay_support::PublicReplayExport::from_json(&export.to_json())
                .expect("export parses"),
            export
        );
    }
}

#[test]
fn terminal_public_export_cannot_reconstruct_tail_or_seed() {
    let trace = trace_from_seeded_first_legal_playout(0);
    let export = export_public_replay(&trace, &Viewer { seat_id: None });
    let json = export.to_json();

    assert!(!json.contains("seed_evidence"));
    assert!(!json.contains("\"seed\""));
    assert!(!json.contains("tail="));
    assert!(!json.contains("tail_cards"));
    assert!(!json.contains("tail_ids"));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExportViewer {
    Observer,
    Seat0,
    Seat1,
}

impl ExportViewer {
    fn viewer(self) -> Viewer {
        match self {
            Self::Observer => Viewer { seat_id: None },
            Self::Seat0 => Viewer {
                seat_id: Some(SeatId("seat_0".to_owned())),
            },
            Self::Seat1 => Viewer {
                seat_id: Some(SeatId("seat_1".to_owned())),
            },
        }
    }
}

#[test]
fn replay_exports_pairwise_private_cards_only_to_authorized_viewer() {
    let state = setup_state(0);
    let probes = PlainTricksSeat::ALL
        .into_iter()
        .flat_map(|source_seat| {
            visible_hand_for_export_probe(&state, source_seat)
                .into_iter()
                .map(move |card| LeakProbe {
                    source_seat,
                    canary_id: card.as_str(),
                    canary: card,
                })
        })
        .collect::<Vec<_>>();
    let trace = PlainTricksInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        seed_evidence: 0,
        commands: Vec::new(),
    };

    assert_pairwise_no_leak(
        [
            ExportViewer::Observer,
            ExportViewer::Seat0,
            ExportViewer::Seat1,
        ],
        ["public_export_json"],
        probes,
        |viewer, _surface| export_public_replay(&trace, &viewer.viewer()).to_json(),
        |source_seat, viewer, _surface, _canary_id| match (source_seat, viewer) {
            (PlainTricksSeat::Seat0, ExportViewer::Seat0)
            | (PlainTricksSeat::Seat1, ExportViewer::Seat1) => ExposureExpectation::MustBePresent,
            _ => ExposureExpectation::MustBeAbsent,
        },
        |snapshot, card| export_contains_card(snapshot, *card),
    )
    .expect("export pairwise no-leak matrix has no failures");

    let canary = ["R3", "PLAIN", "NOLEAK", "CANARY"].join("_");
    for viewer in [
        ExportViewer::Observer,
        ExportViewer::Seat0,
        ExportViewer::Seat1,
    ] {
        let json = export_public_replay(&trace, &viewer.viewer()).to_json();
        assert!(!json.contains(&canary), "{json}");
        for tail in tail_cards(&state) {
            assert!(!export_contains_card(&json, tail), "{json}");
        }
    }
}

#[test]
fn action_tree_v1_parallel_vectors_cover_representative_trees() {
    let vectors = action_tree_v1_vectors();
    for vector in vectors {
        assert_eq!(vector.hash, vector.expected_hash, "{} hash", vector.name);
        assert_eq!(
            vector.bytes.len(),
            vector.expected_bytes_len,
            "{} bytes length",
            vector.name
        );
        assert_eq!(
            HashValue::from_stable_bytes(&vector.bytes),
            vector.hash,
            "{} hash derives from bytes",
            vector.name
        );
        assert_eq!(
            vector.local_hash, vector.expected_local_hash,
            "{} local hash",
            vector.name
        );
        assert_eq!(
            vector.paths, vector.expected_paths,
            "{} legal paths",
            vector.name
        );
    }
}

fn assert_trace_fixture(fixture: &TraceFixture) {
    let first = replay_fixture(fixture);
    let second = replay_fixture(fixture);

    assert_eq!(first.state_hash, second.state_hash, "{} state", fixture.id);
    assert_eq!(
        first.effect_hash, second.effect_hash,
        "{} effects",
        fixture.id
    );
    assert_eq!(
        first.action_tree_hash, second.action_tree_hash,
        "{} action tree",
        fixture.id
    );
    assert_eq!(
        first.observer_view_hash, second.observer_view_hash,
        "{} observer view",
        fixture.id
    );
    assert_eq!(
        first.state_hash,
        HashValue(fixture.expected_state_hash),
        "{} state actual {}",
        fixture.id,
        first.state_hash.0
    );
    assert_eq!(
        first.effect_hash,
        HashValue(fixture.expected_effect_hash),
        "{} effects actual {}",
        fixture.id,
        first.effect_hash.0
    );
    assert_eq!(
        first.action_tree_hash,
        HashValue(fixture.expected_action_tree_hash),
        "{} action tree actual {}",
        fixture.id,
        first.action_tree_hash.0
    );
    assert_eq!(
        first.observer_view_hash,
        HashValue(fixture.expected_observer_view_hash),
        "{} observer view actual {}",
        fixture.id,
        first.observer_view_hash.0
    );
    assert_eq!(
        first.seat_0_view_hash,
        HashValue(fixture.expected_seat_0_view_hash),
        "{} seat 0 view actual {}",
        fixture.id,
        first.seat_0_view_hash.0
    );
    assert_eq!(
        first.seat_1_view_hash,
        HashValue(fixture.expected_seat_1_view_hash),
        "{} seat 1 view actual {}",
        fixture.id,
        first.seat_1_view_hash.0
    );
    assert_eq!(
        first.replay_hash,
        HashValue(fixture.expected_replay_hash),
        "{} replay actual {}",
        fixture.id,
        first.replay_hash.0
    );
    assert_eq!(
        first.public_export_hash,
        HashValue(fixture.expected_public_export_hash),
        "{} public export actual {}",
        fixture.id,
        first.public_export_hash.0
    );
    assert_eq!(
        first.diagnostic_hash,
        fixture.expected_diagnostic_hash.map(HashValue),
        "{} diagnostic hash actual {:?}",
        fixture.id,
        first.diagnostic_hash.map(|hash| hash.0)
    );
    assert_eq!(
        first.diagnostic_code.as_deref(),
        fixture.expected_diagnostic_code.as_deref(),
        "{} diagnostic code",
        fixture.id
    );
    assert_eq!(first.terminal, fixture.terminal, "{} terminal", fixture.id);
    assert_eq!(
        winner(&first.outcome),
        fixture.winner.as_deref(),
        "{} winner",
        fixture.id
    );
    assert_eq!(
        matches!(first.outcome, Some(TerminalOutcome::Split { .. })),
        fixture.draw,
        "{} draw",
        fixture.id
    );

    if fixture.purpose.contains("no_leak")
        || fixture.purpose == "seat_private_view"
        || fixture.purpose == "public_replay_export_import"
    {
        let initial_state = setup_state(fixture.seed);
        if fixture.purpose == "seat_private_view" {
            assert_no_tail_cards(&first.export_json, &initial_state);
        } else {
            assert_no_unobserved_cards(&first.export_json, &initial_state, fixture);
        }
        assert!(!first.export_json.contains("seed_evidence"));
        assert!(!first.export_json.contains("\"seed\""));
    }

    if fixture.purpose == "public_replay_export_import" {
        let trace = internal_trace_from_fixture(fixture);
        let export = export_public_replay(&trace, &Viewer { seat_id: None });
        let imported = import_public_export(&export);
        assert_eq!(imported.viewer, "observer");
        assert_eq!(imported.steps, export.steps);
    }
}

fn replay_command_profile_artifact<'a>(
    profile_id: &'a str,
    profile_version: &'a str,
    visibility_class: Option<&'a str>,
    validator_owner: &'a str,
    fields: &'a [&'a str],
) -> ProfileArtifact<'a> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id,
            profile_version,
            visibility_class,
            validator_owner,
            canonical_byte_authority: "none",
            migration_update_note: Some("profile migration reviewed"),
        },
        fields,
        canonical_byte_claim: false,
    }
}

fn assert_replay_profile_rejects(
    artifact: ProfileArtifact<'_>,
    expected: ProfileValidationErrorKind,
) {
    let driver = ReplayCommandV1Driver::new("plain_tricks");
    assert_eq!(
        driver
            .validate(&artifact)
            .expect_err("invalid replay-command-v1 metadata rejects")
            .kind,
        expected
    );
}

#[derive(Debug, Eq, PartialEq)]
struct SetupEvidenceSummary {
    variant: String,
    hand_count_per_seat: usize,
    tail_count: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct DomainEvidenceSummary {
    deck_size: usize,
    hand_count_per_seat: usize,
    tail_count: usize,
    round_one_completed_tricks: usize,
    terminal_winner: Option<&'static str>,
    terminal_split_each: Option<u8>,
}

fn setup_evidence_profile_artifact<'a>(
    profile_id: &'a str,
    profile_version: &'a str,
    visibility_class: Option<&'a str>,
    validator_owner: &'a str,
    fields: &'a [&'a str],
) -> ProfileArtifact<'a> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id,
            profile_version,
            visibility_class,
            validator_owner,
            canonical_byte_authority: "none",
            migration_update_note: Some("profile migration reviewed"),
        },
        fields,
        canonical_byte_claim: false,
    }
}

fn domain_evidence_profile_artifact<'a>(
    profile_id: &'a str,
    profile_version: &'a str,
    visibility_class: Option<&'a str>,
    validator_owner: &'a str,
    fields: &'a [&'a str],
) -> ProfileArtifact<'a> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id,
            profile_version,
            visibility_class,
            validator_owner,
            canonical_byte_authority: "none",
            migration_update_note: Some("profile migration reviewed"),
        },
        fields,
        canonical_byte_claim: false,
    }
}

fn assert_setup_profile_rejects(
    artifact: ProfileArtifact<'_>,
    expected: ProfileValidationErrorKind,
) {
    let driver = SetupEvidenceV1Driver::new("fixture-check");
    assert_eq!(
        driver
            .validate(&artifact)
            .expect_err("invalid setup-evidence-v1 metadata rejects")
            .kind,
        expected
    );
}

fn assert_domain_profile_rejects(
    artifact: ProfileArtifact<'_>,
    expected: ProfileValidationErrorKind,
) {
    let driver = DomainEvidenceV1Driver::new("plain_tricks");
    assert_eq!(
        driver
            .validate(&artifact)
            .expect_err("invalid domain-evidence-v1 metadata rejects")
            .kind,
        expected
    );
}

fn validate_standard_setup_fixture() -> SetupEvidenceSummary {
    let fixture = load_standard_fixture().expect("fixture parses");
    let state = setup_state(0);
    let seat_0_hand = visible_hand_for_export_probe(&state, PlainTricksSeat::Seat0);
    let seat_1_hand = visible_hand_for_export_probe(&state, PlainTricksSeat::Seat1);
    let tail = tail_cards(&state);

    assert_eq!(fixture.game_id, GAME_ID);
    assert_eq!(fixture.variant, VARIANT_ID);
    assert_eq!(fixture.rules_version, RULES_VERSION_LABEL);
    assert_eq!(fixture.deck_order, TrickCardId::ALL);
    assert_eq!(fixture.hand_status, "hidden_by_setup");
    assert_eq!(fixture.tail_status, "internal_only");
    assert_eq!(
        state.seats,
        [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
    );
    assert_eq!(state.round_index, 0);
    assert_eq!(state.trick_index, 0);
    assert_eq!(state.active_seat, Some(PlainTricksSeat::Seat0));
    assert_eq!(seat_0_hand.len(), 6);
    assert_eq!(seat_1_hand.len(), 6);
    assert_eq!(tail.len(), 6);

    SetupEvidenceSummary {
        variant: fixture.variant,
        hand_count_per_seat: seat_0_hand.len(),
        tail_count: tail.len(),
    }
}

fn validate_plain_tricks_domain_evidence() -> DomainEvidenceSummary {
    let fixture = load_standard_fixture().expect("fixture parses");
    let initial = setup_state(0);
    let seat_0_hand = visible_hand_for_export_probe(&initial, PlainTricksSeat::Seat0);
    let seat_1_hand = visible_hand_for_export_probe(&initial, PlainTricksSeat::Seat1);
    let tail = tail_cards(&initial);

    assert_eq!(fixture.game_id, GAME_ID);
    assert_eq!(fixture.variant, VARIANT_ID);
    assert_eq!(fixture.rules_version, RULES_VERSION_LABEL);
    assert_eq!(fixture.deck_order, TrickCardId::ALL);
    assert_eq!(fixture.hand_status, "hidden_by_setup");
    assert_eq!(fixture.tail_status, "internal_only");
    assert_eq!(seat_0_hand.len(), 6);
    assert_eq!(seat_1_hand.len(), 6);
    assert_eq!(tail.len(), 6);
    assert_complete_deck_partition(&fixture.deck_order, &seat_0_hand, &seat_1_hand, &tail);

    let first_trick = state_after_commands(
        0,
        &[
            (PlainTricksSeat::Seat0, TrickCardId::Gale1),
            (PlainTricksSeat::Seat1, TrickCardId::Gale2),
        ],
    );
    let completed = first_trick.completed_tricks[0];
    assert_eq!(completed.round_index, 0);
    assert_eq!(completed.trick_index, 0);
    assert_eq!(completed.leader, PlainTricksSeat::Seat0);
    assert_eq!(completed.plays[0].card, TrickCardId::Gale1);
    assert_eq!(completed.plays[1].card, TrickCardId::Gale2);
    assert_eq!(completed.winner, PlainTricksSeat::Seat1);
    assert_eq!(first_trick.current_leader, PlainTricksSeat::Seat1);
    assert_eq!(first_trick.active_seat, Some(PlainTricksSeat::Seat1));
    assert_eq!(first_trick.round_trick_counts.seat_0, 0);
    assert_eq!(first_trick.round_trick_counts.seat_1, 1);

    let round_close = state_after_commands(0, &round_one_terminal_commands());
    assert_eq!(round_close.completed_tricks.len(), 6);
    assert_eq!(round_close.round_index, 1);
    assert_eq!(round_close.trick_index, 0);
    assert_eq!(
        round_close.phase,
        Phase::Playing {
            round_index: 1,
            trick_index: 0,
        }
    );
    assert_eq!(round_close.total_trick_counts.seat_0, 3);
    assert_eq!(round_close.total_trick_counts.seat_1, 3);
    assert_eq!(round_close.round_trick_counts.seat_0, 0);
    assert_eq!(round_close.round_trick_counts.seat_1, 0);

    let terminal_fixture = parse_trace_fixture(include_str!(
        "golden_traces/terminal-most-points-win.trace.json"
    ));
    assert_trace_fixture(&terminal_fixture);
    let terminal = state_after_trace_commands(&terminal_fixture);
    assert_eq!(terminal.phase, Phase::Terminal);
    assert_eq!(terminal.completed_tricks.len(), 12);
    assert_eq!(
        terminal.terminal_outcome,
        Some(TerminalOutcome::TrickWin {
            winner: PlainTricksSeat::Seat1,
            totals: terminal.total_trick_counts,
        })
    );
    assert!(terminal.total_trick_counts.seat_1 > terminal.total_trick_counts.seat_0);

    let split_fixture = parse_trace_fixture(include_str!("golden_traces/tie-split.trace.json"));
    assert_trace_fixture(&split_fixture);
    let split = state_after_trace_commands(&split_fixture);
    assert_eq!(
        split.terminal_outcome,
        Some(TerminalOutcome::Split {
            each: 6,
            totals: split.total_trick_counts,
        })
    );

    DomainEvidenceSummary {
        deck_size: fixture.deck_order.len(),
        hand_count_per_seat: seat_0_hand.len(),
        tail_count: tail.len(),
        round_one_completed_tricks: round_close.completed_tricks.len(),
        terminal_winner: winner(&terminal.terminal_outcome),
        terminal_split_each: split_each(&split.terminal_outcome),
    }
}

fn assert_complete_deck_partition(
    deck_order: &[TrickCardId],
    seat_0_hand: &[TrickCardId],
    seat_1_hand: &[TrickCardId],
    tail: &[TrickCardId],
) {
    let mut partition = Vec::new();
    partition.extend_from_slice(seat_0_hand);
    partition.extend_from_slice(seat_1_hand);
    partition.extend_from_slice(tail);
    assert_eq!(partition.len(), deck_order.len());
    for card in deck_order {
        assert_eq!(
            partition
                .iter()
                .filter(|candidate| *candidate == card)
                .count(),
            1,
            "{card:?} appears exactly once in the dealt partition"
        );
    }
}

fn round_one_terminal_commands() -> Vec<(PlainTricksSeat, TrickCardId)> {
    vec![
        (PlainTricksSeat::Seat0, TrickCardId::Gale1),
        (PlainTricksSeat::Seat1, TrickCardId::Gale2),
        (PlainTricksSeat::Seat1, TrickCardId::Ember3),
        (PlainTricksSeat::Seat0, TrickCardId::Ember6),
        (PlainTricksSeat::Seat0, TrickCardId::River3),
        (PlainTricksSeat::Seat1, TrickCardId::River6),
        (PlainTricksSeat::Seat1, TrickCardId::Gale3),
        (PlainTricksSeat::Seat0, TrickCardId::River5),
        (PlainTricksSeat::Seat1, TrickCardId::Ember2),
        (PlainTricksSeat::Seat0, TrickCardId::Ember5),
        (PlainTricksSeat::Seat0, TrickCardId::River1),
        (PlainTricksSeat::Seat1, TrickCardId::Gale6),
    ]
}

fn visible_hand_for_export_probe(
    state: &PlainTricksState,
    seat: PlainTricksSeat,
) -> Vec<TrickCardId> {
    let view = plain_tricks::project_view(
        state,
        &Viewer {
            seat_id: Some(state.seats[seat.index()].clone()),
        },
    );
    let plain_tricks::PrivateView::Seat(private) = view.private_view else {
        panic!("seat viewer gets private view");
    };
    private
        .own_hand
        .iter()
        .map(|card| TrickCardId::parse(&card.card_id).expect("known card"))
        .collect()
}

fn export_contains_card(text: &str, card: TrickCardId) -> bool {
    text.contains(card.as_str())
        || text.contains(&card.label())
        || text.contains(&format!("{card:?}"))
}

struct ActionTreeV1Vector {
    name: &'static str,
    bytes: Vec<u8>,
    hash: HashValue,
    local_hash: HashValue,
    paths: Vec<Vec<String>>,
    expected_bytes_len: usize,
    expected_hash: HashValue,
    expected_local_hash: HashValue,
    expected_paths: Vec<Vec<String>>,
}

fn action_tree_v1_vectors() -> Vec<ActionTreeV1Vector> {
    let opening = setup_state(0);

    let forced_follow = state_after_commands(0, &[(PlainTricksSeat::Seat0, TrickCardId::Gale1)]);

    let void_free_discard = state_after_commands(
        0,
        &[
            (PlainTricksSeat::Seat0, TrickCardId::Gale1),
            (PlainTricksSeat::Seat1, TrickCardId::Gale2),
            (PlainTricksSeat::Seat1, TrickCardId::Ember3),
            (PlainTricksSeat::Seat0, TrickCardId::Ember6),
            (PlainTricksSeat::Seat0, TrickCardId::River3),
            (PlainTricksSeat::Seat1, TrickCardId::River6),
            (PlainTricksSeat::Seat1, TrickCardId::Gale3),
        ],
    );

    let final_play = state_after_commands(
        0,
        &[
            (PlainTricksSeat::Seat0, TrickCardId::Gale1),
            (PlainTricksSeat::Seat1, TrickCardId::Gale2),
            (PlainTricksSeat::Seat1, TrickCardId::Ember3),
            (PlainTricksSeat::Seat0, TrickCardId::Ember6),
            (PlainTricksSeat::Seat0, TrickCardId::River3),
            (PlainTricksSeat::Seat1, TrickCardId::River6),
            (PlainTricksSeat::Seat1, TrickCardId::Gale3),
            (PlainTricksSeat::Seat0, TrickCardId::River5),
            (PlainTricksSeat::Seat1, TrickCardId::Ember2),
            (PlainTricksSeat::Seat0, TrickCardId::Ember5),
            (PlainTricksSeat::Seat0, TrickCardId::River1),
            (PlainTricksSeat::Seat1, TrickCardId::Gale6),
            (PlainTricksSeat::Seat1, TrickCardId::Ember4),
            (PlainTricksSeat::Seat0, TrickCardId::Ember2),
            (PlainTricksSeat::Seat1, TrickCardId::Gale1),
            (PlainTricksSeat::Seat0, TrickCardId::River5),
            (PlainTricksSeat::Seat1, TrickCardId::Gale6),
            (PlainTricksSeat::Seat0, TrickCardId::River2),
            (PlainTricksSeat::Seat1, TrickCardId::Ember6),
            (PlainTricksSeat::Seat0, TrickCardId::River3),
            (PlainTricksSeat::Seat1, TrickCardId::Gale3),
            (PlainTricksSeat::Seat0, TrickCardId::River1),
            (PlainTricksSeat::Seat1, TrickCardId::Gale5),
        ],
    );

    let terminal = state_after_commands(
        0,
        &[
            (PlainTricksSeat::Seat0, TrickCardId::Gale1),
            (PlainTricksSeat::Seat1, TrickCardId::Gale2),
            (PlainTricksSeat::Seat1, TrickCardId::Ember3),
            (PlainTricksSeat::Seat0, TrickCardId::Ember6),
            (PlainTricksSeat::Seat0, TrickCardId::River3),
            (PlainTricksSeat::Seat1, TrickCardId::River6),
            (PlainTricksSeat::Seat1, TrickCardId::Gale3),
            (PlainTricksSeat::Seat0, TrickCardId::River5),
            (PlainTricksSeat::Seat1, TrickCardId::Ember2),
            (PlainTricksSeat::Seat0, TrickCardId::Ember5),
            (PlainTricksSeat::Seat0, TrickCardId::River1),
            (PlainTricksSeat::Seat1, TrickCardId::Gale6),
            (PlainTricksSeat::Seat1, TrickCardId::Ember4),
            (PlainTricksSeat::Seat0, TrickCardId::Ember2),
            (PlainTricksSeat::Seat1, TrickCardId::Gale1),
            (PlainTricksSeat::Seat0, TrickCardId::River5),
            (PlainTricksSeat::Seat1, TrickCardId::Gale6),
            (PlainTricksSeat::Seat0, TrickCardId::River2),
            (PlainTricksSeat::Seat1, TrickCardId::Ember6),
            (PlainTricksSeat::Seat0, TrickCardId::River3),
            (PlainTricksSeat::Seat1, TrickCardId::Gale3),
            (PlainTricksSeat::Seat0, TrickCardId::River1),
            (PlainTricksSeat::Seat1, TrickCardId::Gale5),
            (PlainTricksSeat::Seat0, TrickCardId::River6),
        ],
    );

    vec![
        vector(
            "opening_trick",
            &opening,
            PlainTricksSeat::Seat0,
            3209,
            HashValue(10760653848758353227),
            HashValue(9608973152758876482),
            vec![
                vec!["play".to_owned(), "gale_1".to_owned()],
                vec!["play".to_owned(), "river_3".to_owned()],
                vec!["play".to_owned(), "river_5".to_owned()],
                vec!["play".to_owned(), "river_1".to_owned()],
                vec!["play".to_owned(), "ember_6".to_owned()],
                vec!["play".to_owned(), "ember_5".to_owned()],
            ],
        ),
        vector(
            "forced_follow_suit",
            &forced_follow,
            PlainTricksSeat::Seat1,
            1850,
            HashValue(10249125325511701213),
            HashValue(11988930228804901292),
            vec![
                vec!["play".to_owned(), "gale_2".to_owned()],
                vec!["play".to_owned(), "gale_3".to_owned()],
                vec!["play".to_owned(), "gale_6".to_owned()],
            ],
        ),
        vector(
            "void_free_discard",
            &void_free_discard,
            PlainTricksSeat::Seat0,
            1874,
            HashValue(13864411618449214495),
            HashValue(2830033628787621803),
            vec![
                vec!["play".to_owned(), "river_5".to_owned()],
                vec!["play".to_owned(), "river_1".to_owned()],
                vec!["play".to_owned(), "ember_5".to_owned()],
            ],
        ),
        vector(
            "final_play",
            &final_play,
            PlainTricksSeat::Seat0,
            932,
            HashValue(10622526245863211658),
            HashValue(12733681326737878192),
            vec![vec!["play".to_owned(), "river_6".to_owned()]],
        ),
        vector(
            "terminal_empty_tree",
            &terminal,
            PlainTricksSeat::Seat0,
            64,
            HashValue(17407510006563527667),
            HashValue(117586594652395198),
            Vec::new(),
        ),
    ]
}

fn vector(
    name: &'static str,
    state: &PlainTricksState,
    seat: PlainTricksSeat,
    expected_bytes_len: usize,
    expected_hash: HashValue,
    expected_local_hash: HashValue,
    expected_paths: Vec<Vec<String>>,
) -> ActionTreeV1Vector {
    let actor = Actor {
        seat_id: state.seats[seat.index()].clone(),
    };
    let tree = legal_action_tree(state, &actor);
    let bytes = action_tree_v1_bytes(&tree);
    ActionTreeV1Vector {
        name,
        hash: action_tree_v1_hash(&tree),
        local_hash: action_tree_hash(&tree),
        paths: action_paths(&tree.root.choices),
        bytes,
        expected_bytes_len,
        expected_hash,
        expected_local_hash,
        expected_paths,
    }
}

fn state_after_commands(
    seed: u64,
    commands: &[(PlainTricksSeat, TrickCardId)],
) -> PlainTricksState {
    let mut state = setup_state(seed);
    for (seat, card) in commands {
        let command = command_for_state_for_test(&state, *seat, *card);
        let action = validate_command(&state, &command).expect("test command validates");
        apply_action(&mut state, action).expect("test command applies");
    }
    state
}

fn state_after_trace_commands(fixture: &TraceFixture) -> PlainTricksState {
    let mut state = setup_state(fixture.seed);
    for command in &fixture.commands {
        if command.expect != "applied" {
            continue;
        }
        let envelope = command_envelope(&state, command);
        let action = validate_command(&state, &envelope).expect("fixture command validates");
        apply_action(&mut state, action).expect("fixture command applies");
    }
    state
}

fn command_for_state_for_test(
    state: &PlainTricksState,
    actor: PlainTricksSeat,
    card: TrickCardId,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[actor.index()].clone(),
        },
        action_path: ActionPath {
            segments: vec!["play".to_owned(), card.as_str().to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn action_paths(choices: &[engine_core::ActionChoice]) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    collect_action_paths(choices, &mut Vec::new(), &mut paths);
    paths
}

fn collect_action_paths(
    choices: &[engine_core::ActionChoice],
    prefix: &mut Vec<String>,
    paths: &mut Vec<Vec<String>>,
) {
    for choice in choices {
        prefix.push(choice.segment.clone());
        if let Some(next) = &choice.next {
            collect_action_paths(&next.choices, prefix, paths);
        } else {
            paths.push(prefix.clone());
        }
        prefix.pop();
    }
}

fn replay_fixture(fixture: &TraceFixture) -> ReplayActual {
    let mut state = setup_state(fixture.seed);
    let mut effects = setup_effects(&state);
    let mut applied = Vec::new();
    let mut diagnostic_code = None;
    let mut diagnostic_hash = None;

    for command in &fixture.commands {
        let envelope = command_envelope(&state, command);
        match command.expect.as_str() {
            "applied" => {
                let action =
                    validate_command(&state, &envelope).expect("fixture command validates");
                effects.extend(apply_action(&mut state, action).expect("fixture command applies"));
                applied.push(ReplayCommand {
                    actor: command.actor_seat.as_str().to_owned(),
                    path: command.action_path.clone(),
                });
            }
            "diagnostic" => {
                let before = state.clone();
                let diagnostic = validate_command(&state, &envelope)
                    .expect_err("fixture diagnostic command rejects");
                let expected = command
                    .expected_diagnostic_code
                    .as_deref()
                    .expect("diagnostic command includes expected code");
                assert_eq!(diagnostic.code, expected);
                assert_eq!(state, before);
                diagnostic_code = Some(diagnostic.code.clone());
                diagnostic_hash = Some(HashValue::from_stable_bytes(
                    format!("{diagnostic:?}").as_bytes(),
                ));
            }
            other => panic!("unsupported command expectation `{other}`"),
        }
    }

    let trace = PlainTricksInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        seed_evidence: fixture.seed,
        commands: applied,
    };
    let export = export_public_replay(&trace, &export_viewer_for_fixture(fixture));

    ReplayActual {
        state_hash: state_hash(&state),
        effect_hash: effect_hash(&effects),
        action_tree_hash: combined_action_tree_hash(&state),
        observer_view_hash: view_hash(&state, &Viewer { seat_id: None }),
        seat_0_view_hash: view_hash(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_0".to_owned())),
            },
        ),
        seat_1_view_hash: view_hash(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_1".to_owned())),
            },
        ),
        replay_hash: trace.stable_hash(),
        public_export_hash: export.stable_hash(),
        diagnostic_hash,
        diagnostic_code,
        terminal: state.phase == Phase::Terminal,
        outcome: state.terminal_outcome,
        export_json: export.to_json(),
    }
}

fn setup_state(seed: u64) -> PlainTricksState {
    setup_match(
        engine_core::Seed(seed),
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn export_viewer_for_fixture(fixture: &TraceFixture) -> Viewer {
    if fixture.purpose == "seat_private_view" {
        Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        }
    } else {
        Viewer { seat_id: None }
    }
}

fn command_envelope(state: &PlainTricksState, command: &TraceCommand) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[command.actor_seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: command.action_path.clone(),
        },
        freshness_token: FreshnessToken(command.freshness_token),
        rules_version: RulesVersion(1),
    }
}

fn combined_action_tree_hash(state: &PlainTricksState) -> HashValue {
    let parts = PlainTricksSeat::ALL
        .iter()
        .map(|seat| {
            let actor = Actor {
                seat_id: state.seats[seat.index()].clone(),
            };
            action_tree_hash(&legal_action_tree(state, &actor))
                .0
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(parts.as_bytes())
}

fn internal_trace_from_fixture(fixture: &TraceFixture) -> PlainTricksInternalTrace {
    PlainTricksInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        seed_evidence: fixture.seed,
        commands: fixture
            .commands
            .iter()
            .filter(|command| command.expect == "applied")
            .map(|command| ReplayCommand {
                actor: command.actor_seat.as_str().to_owned(),
                path: command.action_path.clone(),
            })
            .collect(),
    }
}

fn assert_no_tail_cards(text: &str, initial_state: &PlainTricksState) {
    for card in tail_cards(initial_state) {
        assert!(!text.contains(card.as_str()), "{text}");
        assert!(!text.contains(&format!("{card:?}")), "{text}");
    }
}

fn assert_no_unobserved_cards(
    text: &str,
    initial_state: &PlainTricksState,
    fixture: &TraceFixture,
) {
    let played = fixture
        .commands
        .iter()
        .filter(|command| command.expect == "applied")
        .filter_map(|command| command.action_path.get(1))
        .filter_map(|segment| TrickCardId::parse(segment))
        .collect::<Vec<_>>();
    for card in TrickCardId::ALL {
        if played.contains(&card) {
            continue;
        }
        assert!(!text.contains(card.as_str()), "{text}");
        assert!(!text.contains(&format!("{card:?}")), "{text}");
    }
    assert_no_tail_cards(text, initial_state);
}

fn tail_cards(initial_state: &PlainTricksState) -> Vec<TrickCardId> {
    let mut observed = Vec::new();
    for seat in PlainTricksSeat::ALL {
        let view = plain_tricks::project_view(
            initial_state,
            &Viewer {
                seat_id: Some(initial_state.seats[seat.index()].clone()),
            },
        );
        let plain_tricks::PrivateView::Seat(private) = view.private_view else {
            panic!("seat viewer gets private view");
        };
        observed.extend(
            private
                .own_hand
                .iter()
                .map(|card| TrickCardId::parse(&card.card_id).expect("known card")),
        );
    }
    TrickCardId::ALL
        .into_iter()
        .filter(|card| !observed.contains(card))
        .collect()
}

fn winner(outcome: &Option<TerminalOutcome>) -> Option<&'static str> {
    match outcome {
        Some(TerminalOutcome::TrickWin { winner, .. }) => Some(winner.as_str()),
        Some(TerminalOutcome::Split { .. }) | None => None,
    }
}

fn split_each(outcome: &Option<TerminalOutcome>) -> Option<u8> {
    match outcome {
        Some(TerminalOutcome::Split { each, .. }) => Some(*each),
        Some(TerminalOutcome::TrickWin { .. }) | None => None,
    }
}

fn parse_trace_fixture(input: &str) -> TraceFixture {
    assert_eq!(number_field(input, "schema_version"), 1);
    assert_eq!(string_field(input, "game_id"), GAME_ID);
    assert_eq!(string_field(input, "rules_version"), RULES_VERSION_LABEL);
    assert_eq!(string_field(input, "variant"), VARIANT_ID);
    let public_view_hashes = object_body(input, "expected_public_view_hashes");
    let diagnostic_body = input.find("\"expected_diagnostics\":").and_then(|start| {
        let marker = "\"expected_diagnostics\":";
        let rest = input[start + marker.len()..].trim_start();
        (!rest.starts_with("null")).then(|| object_body(input, "expected_diagnostics"))
    });
    TraceFixture {
        id: string_field(input, "trace_id"),
        purpose: string_field(input, "purpose"),
        seed: number_field(input, "seed"),
        commands: commands(input),
        expected_state_hash: final_hash(input, "expected_state_hashes"),
        expected_effect_hash: final_hash(input, "expected_effect_hashes"),
        expected_action_tree_hash: final_hash(input, "expected_action_tree_hashes"),
        expected_observer_view_hash: number_field(&public_view_hashes, "observer"),
        expected_seat_0_view_hash: number_field(&public_view_hashes, "seat_0"),
        expected_seat_1_view_hash: number_field(&public_view_hashes, "seat_1"),
        expected_replay_hash: final_hash(input, "expected_replay_hashes"),
        expected_public_export_hash: final_hash(input, "expected_public_export_hashes"),
        expected_diagnostic_hash: optional_diagnostic_hash(input),
        expected_diagnostic_code: diagnostic_body.map(|body| string_field(&body, "code")),
        terminal: bool_field(&object_body(input, "expected_terminal_state"), "terminal"),
        winner: nullable_string_field(&object_body(input, "expected_terminal_state"), "winner"),
        draw: bool_field(&object_body(input, "expected_terminal_state"), "draw"),
    }
}

fn commands(input: &str) -> Vec<TraceCommand> {
    let body = array_body(input, "commands");
    if body.trim().is_empty() {
        return Vec::new();
    }
    split_top_level(&body, ',')
        .into_iter()
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let expected_code = optional_string_field(&entry, "expected_diagnostic_code");
            TraceCommand {
                actor_seat: PlainTricksSeat::parse(&string_field(&entry, "actor_seat"))
                    .expect("trace seat is valid"),
                action_path: string_array_field(&entry, "action_path"),
                freshness_token: string_field(&entry, "freshness_token").parse().unwrap(),
                expect: if expected_code.is_some() {
                    "diagnostic".to_owned()
                } else {
                    string_field(&entry, "expect")
                },
                expected_diagnostic_code: expected_code,
            }
        })
        .collect()
}

fn final_hash(input: &str, key: &str) -> u64 {
    number_field(&object_body(input, key), "final")
}

fn optional_diagnostic_hash(input: &str) -> Option<u64> {
    let marker = "\"expected_diagnostic_hashes\":";
    let start = input.find(marker)?;
    let rest = input[start + marker.len()..].trim_start();
    if rest.starts_with("null") {
        None
    } else {
        Some(number_field(
            &object_body(input, "expected_diagnostic_hashes"),
            "final",
        ))
    }
}

fn object_body(input: &str, key: &str) -> String {
    nested_body(input, key, '{', '}')
}

fn array_body(input: &str, key: &str) -> String {
    nested_body(input, key, '[', ']')
}

fn nested_body(input: &str, key: &str, open: char, close: char) -> String {
    let marker = format!("\"{key}\":");
    let start = input
        .find(&marker)
        .unwrap_or_else(|| panic!("missing field `{key}`"));
    let after = &input[start + marker.len()..];
    let open_index = after
        .find(open)
        .unwrap_or_else(|| panic!("missing `{open}` for `{key}`"));
    let value = &after[open_index..];
    let mut depth = 0_i32;
    let mut in_string = false;
    let mut previous_escape = false;
    for (index, ch) in value.char_indices() {
        match ch {
            '"' if !previous_escape => in_string = !in_string,
            c if c == open && !in_string => depth += 1,
            c if c == close && !in_string => {
                depth -= 1;
                if depth == 0 {
                    return value[1..index].to_owned();
                }
            }
            _ => {}
        }
        previous_escape = ch == '\\' && !previous_escape;
        if ch != '\\' {
            previous_escape = false;
        }
    }
    panic!("unterminated nested field `{key}`")
}

fn string_array_field(input: &str, key: &str) -> Vec<String> {
    let body = array_body(input, key);
    if body.trim().is_empty() {
        return Vec::new();
    }
    split_top_level(&body, ',')
        .into_iter()
        .map(|value| parse_json_string(value.trim()))
        .collect()
}

fn string_field(input: &str, key: &str) -> String {
    optional_string_field(input, key).unwrap_or_else(|| panic!("missing string field `{key}`"))
}

fn optional_string_field(input: &str, key: &str) -> Option<String> {
    let marker = format!("\"{key}\":");
    let start = input.find(&marker)?;
    let rest = input[start + marker.len()..].trim_start();
    let end = end_of_scalar(rest);
    Some(parse_json_string(rest[..end].trim()))
}

fn nullable_string_field(input: &str, key: &str) -> Option<String> {
    let marker = format!("\"{key}\":");
    let start = input
        .find(&marker)
        .unwrap_or_else(|| panic!("missing nullable `{key}`"));
    let rest = input[start + marker.len()..].trim_start();
    if rest.starts_with("null") {
        None
    } else {
        let end = end_of_scalar(rest);
        Some(parse_json_string(rest[..end].trim()))
    }
}

fn number_field(input: &str, key: &str) -> u64 {
    let marker = format!("\"{key}\":");
    let start = input
        .find(&marker)
        .unwrap_or_else(|| panic!("missing number `{key}`"));
    let rest = input[start + marker.len()..].trim_start();
    let end = end_of_scalar(rest);
    rest[..end].trim().parse().expect("number parses")
}

fn bool_field(input: &str, key: &str) -> bool {
    let marker = format!("\"{key}\":");
    let start = input
        .find(&marker)
        .unwrap_or_else(|| panic!("missing bool `{key}`"));
    let rest = input[start + marker.len()..].trim_start();
    rest.starts_with("true")
}

fn end_of_scalar(input: &str) -> usize {
    let mut in_string = false;
    let mut previous_escape = false;
    for (index, ch) in input.char_indices() {
        match ch {
            '"' if !previous_escape => in_string = !in_string,
            ',' | '}' | ']' if !in_string => return index,
            _ => {}
        }
        previous_escape = ch == '\\' && !previous_escape;
        if ch != '\\' {
            previous_escape = false;
        }
    }
    input.len()
}

fn split_top_level(input: &str, delimiter: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut depth = 0_i32;
    let mut in_string = false;
    let mut previous_escape = false;

    for (index, ch) in input.char_indices() {
        match ch {
            '"' if !previous_escape => in_string = !in_string,
            '[' | '{' if !in_string => depth += 1,
            ']' | '}' if !in_string => depth -= 1,
            _ => {}
        }
        if ch == delimiter && depth == 0 && !in_string {
            result.push(input[start..index].to_owned());
            start = index + ch.len_utf8();
        }
        previous_escape = ch == '\\' && !previous_escape;
        if ch != '\\' {
            previous_escape = false;
        }
    }
    result.push(input[start..].to_owned());
    result
}

fn parse_json_string(raw: &str) -> String {
    raw.trim()
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .expect("expected JSON string")
        .replace("\\n", "\n")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
}
