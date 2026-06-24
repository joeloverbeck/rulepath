use engine_core::{
    ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, Seed, StableSerialize,
    Viewer,
};
use flood_watch::{
    action_tree_hash, action_tree_v1_bytes, action_tree_v1_hash, apply_command,
    export_public_replay, generate_internal_full_trace, import_public_export, legal_action_tree,
    load_deluge_fixture, load_standard_fixture, public_replay_step, setup_match, DistrictId,
    EventCard, EventKind, FloodWatchRole, FloodWatchState, Phase, ScenarioVariant, SetupOptions,
    ACTION_END_TURN, ACTION_REINFORCE, GAME_ID, RULES_VERSION_LABEL,
};
use game_test_support::no_leak::{assert_pairwise_no_leak, ExposureExpectation, LeakProbe};
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind, PublicExportV1Driver,
    ReplayCommandV1Driver, SetupEvidenceV1Driver, PROFILE_VERSION_V1, PUBLIC_EXPORT_V1,
    REPLAY_COMMAND_V1, SETUP_EVIDENCE_V1,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
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

fn card(kind: EventKind, copy_index: u8) -> EventCard {
    EventCard { kind, copy_index }
}

fn observer() -> Viewer {
    Viewer { seat_id: None }
}

fn seat_viewer(seat: &str) -> Viewer {
    Viewer {
        seat_id: Some(SeatId(seat.to_owned())),
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PublicExportProfileSummary {
    viewer: String,
    step_count: usize,
    public_export_hash: HashValue,
    hidden_future_cards_checked: usize,
}

#[test]
fn setup_state_hash_is_deterministic_for_same_seed_and_scenario() {
    let options = SetupOptions::default();

    let first = setup_match(Seed(55), &seats(), &options).unwrap();
    let second = setup_match(Seed(55), &seats(), &options).unwrap();

    assert_eq!(first.event_deck_internal(), second.event_deck_internal());
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn terminal_outcome_replays_deterministically() {
    let deck = vec![card(
        EventKind::StormSurge {
            district: DistrictId::OldDocks,
        },
        1,
    )];
    let mut first =
        FloodWatchState::new_after_setup(ScenarioVariant::standard(), seats(), deck.clone());
    let mut second = FloodWatchState::new_after_setup(ScenarioVariant::standard(), seats(), deck);
    let first_command = end_turn_command(&first);
    let second_command = end_turn_command(&second);

    let first_applied = apply_command(&mut first, &first_command).unwrap();
    let second_applied = apply_command(&mut second, &second_command).unwrap();

    assert_eq!(first.terminal_outcome, second.terminal_outcome);
    assert_eq!(first_applied.effects, second_applied.effects);
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn setup_state_hash_changes_with_seed_or_scenario() {
    let standard = setup_match(Seed(55), &seats(), &SetupOptions::default()).unwrap();
    let other_seed = setup_match(Seed(56), &seats(), &SetupOptions::default()).unwrap();
    let deluge = setup_match(
        Seed(55),
        &seats(),
        &SetupOptions {
            variant: ScenarioVariant::deluge(),
        },
    )
    .unwrap();

    assert_ne!(
        standard.event_deck_internal(),
        other_seed.event_deck_internal()
    );
    assert_ne!(standard.stable_hash(), other_seed.stable_hash());
    assert_ne!(standard.stable_hash(), deluge.stable_hash());
}

#[test]
fn replay_command_v1_profile_driver_wraps_native_replay_evidence() {
    let driver = ReplayCommandV1Driver::new("flood_watch");
    let artifact = replay_command_profile_artifact(
        REPLAY_COMMAND_V1,
        PROFILE_VERSION_V1,
        Some("internal-dev"),
        "flood_watch",
        &["commands", "checkpoints", "expected_hashes"],
    );

    let report = driver
        .validate(&artifact)
        .expect("profile metadata validates");
    assert_eq!(report.profile_id, REPLAY_COMMAND_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "internal-dev");
    assert_eq!(report.validator_owner, "flood_watch");
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let hashes = driver
        .validate_with(&artifact, |_| native_replay_profile_hashes())
        .expect("profile delegates to native replay evidence");
    assert_eq!(hashes.len(), 5);

    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            "public-export-v1",
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "flood_watch",
            &["commands"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            "v2",
            Some("internal-dev"),
            "flood_watch",
            &["commands"],
        ),
        ProfileValidationErrorKind::WrongProfileVersion,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("seat-private"),
            "flood_watch",
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
            "flood_watch",
            &["commands", "export_steps"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn public_export_v1_profile_driver_wraps_flood_watch_public_exporter() {
    let driver = PublicExportV1Driver::new("flood_watch");
    let artifact = public_export_profile_artifact(
        PUBLIC_EXPORT_V1,
        PROFILE_VERSION_V1,
        Some("public"),
        "flood_watch",
        &["export_steps", "import_round_trip", "hidden_absence_tokens"],
    );

    let report = driver
        .validate(&artifact)
        .expect("public export metadata validates");
    assert_eq!(report.profile_id, PUBLIC_EXPORT_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "public");
    assert_eq!(report.validator_owner, "flood_watch");
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let summary = driver
        .validate_with(&artifact, |_| validate_public_export_profile())
        .expect("profile delegates to Flood Watch public exporter");
    assert_eq!(summary.viewer, "observer");
    assert_eq!(summary.step_count, 1);
    assert_ne!(summary.public_export_hash.0, 0);
    assert!(summary.hidden_future_cards_checked > 0);

    assert_public_export_profile_rejects(
        public_export_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("public"),
            "flood_watch",
            &["export_steps"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_public_export_profile_rejects(
        public_export_profile_artifact(
            PUBLIC_EXPORT_V1,
            "v2",
            Some("public"),
            "flood_watch",
            &["export_steps"],
        ),
        ProfileValidationErrorKind::WrongProfileVersion,
    );
    assert_public_export_profile_rejects(
        public_export_profile_artifact(
            PUBLIC_EXPORT_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "flood_watch",
            &["export_steps"],
        ),
        ProfileValidationErrorKind::InvalidVisibility,
    );
    assert_public_export_profile_rejects(
        public_export_profile_artifact(
            PUBLIC_EXPORT_V1,
            PROFILE_VERSION_V1,
            Some("public"),
            "replay-check",
            &["export_steps"],
        ),
        ProfileValidationErrorKind::WrongValidatorOwner,
    );
    assert_public_export_profile_rejects(
        public_export_profile_artifact(
            PUBLIC_EXPORT_V1,
            PROFILE_VERSION_V1,
            Some("public"),
            "flood_watch",
            &["export_steps", "commands"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn setup_evidence_v1_profile_driver_wraps_standard_and_deluge_fixtures() {
    let driver = SetupEvidenceV1Driver::new("fixture-check");
    let artifact = setup_evidence_profile_artifact(
        SETUP_EVIDENCE_V1,
        PROFILE_VERSION_V1,
        Some("public"),
        "fixture-check",
        &["seat_grammar_version", "setup_options", "expected_setup"],
    );

    let report = driver
        .validate(&artifact)
        .expect("setup metadata validates");
    assert_eq!(report.profile_id, SETUP_EVIDENCE_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "public");
    assert_eq!(report.validator_owner, "fixture-check");
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let summary = driver
        .validate_with(&artifact, |_| validate_setup_fixtures())
        .expect("profile delegates to setup fixture validator");
    assert_eq!(summary.fixture_count, 2);
    assert_eq!(summary.role_order_count, 2);

    assert_setup_profile_rejects(
        setup_evidence_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("public"),
            "fixture-check",
            &["expected_setup"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_setup_profile_rejects(
        setup_evidence_profile_artifact(
            SETUP_EVIDENCE_V1,
            "v2",
            Some("public"),
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
            Some("public"),
            "flood_watch",
            &["expected_setup"],
        ),
        ProfileValidationErrorKind::WrongValidatorOwner,
    );
    assert_setup_profile_rejects(
        setup_evidence_profile_artifact(
            SETUP_EVIDENCE_V1,
            PROFILE_VERSION_V1,
            Some("public"),
            "fixture-check",
            &["expected_setup", "domain_input"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn environment_effects_and_hashes_replay_deterministically() {
    let options = SetupOptions::default();
    let mut first = setup_match(Seed(91), &seats(), &options).unwrap();
    let mut second = setup_match(Seed(91), &seats(), &options).unwrap();
    let first_command = end_turn_command(&first);
    let second_command = end_turn_command(&second);

    let first_applied = apply_command(&mut first, &first_command).unwrap();
    let second_applied = apply_command(&mut second, &second_command).unwrap();

    assert_eq!(first_applied.effects, second_applied.effects);
    assert_eq!(first.drawn, second.drawn);
    assert_eq!(first.event_deck_internal(), second.event_deck_internal());
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn public_export_import_redacts_undrawn_deck_after_terminal() {
    let deck = vec![
        card(EventKind::Reprieve, 1),
        card(
            EventKind::StormSurge {
                district: DistrictId::Gardens,
            },
            1,
        ),
    ];
    let mut state = FloodWatchState::new_after_setup(ScenarioVariant::standard(), seats(), deck);
    let command = end_turn_command(&state);
    let applied = apply_command(&mut state, &command).unwrap();
    let step = public_replay_step(0, &state, &command, &applied.effects, &observer());
    let export = export_public_replay(state.variant.id.clone(), &observer(), vec![step]);
    let imported = import_public_export(&export);
    let rendered = imported.raw_json;

    assert!(state.terminal_outcome.is_some());
    assert!(rendered.contains("Event 1 drawn: Reprieve"));
    assert!(rendered.contains("Event 2 drawn: Storm Surge at Gardens"));
    assert!(!rendered.contains("storm_surge/district_gardens#1"));
    assert!(!rendered.contains("full_deck_order"));
    assert!(!rendered.contains("deck_order"));
}

#[test]
fn public_exports_pairwise_omit_hidden_future_deck_cards() {
    let mut state = setup_match(Seed(91), &seats(), &SetupOptions::default()).unwrap();
    let command = end_turn_command(&state);
    let applied = apply_command(&mut state, &command).unwrap();
    let step = public_replay_step(0, &state, &command, &applied.effects, &observer());
    let probes = hidden_future_probes(&state);

    assert_pairwise_no_leak(
        [observer(), seat_viewer("seat_0"), seat_viewer("seat_1")],
        ["public_export_json"],
        probes,
        |viewer, _surface| {
            export_public_replay(state.variant.id.clone(), viewer, vec![step.clone()]).to_json()
        },
        |_source, _viewer, _surface, _canary_id| ExposureExpectation::MustBeAbsent,
        |snapshot, card| snapshot_contains_event(snapshot, card),
    )
    .expect("Flood Watch export no-leak matrix has no failures");

    let canary = ["R3", "FLOOD", "NOLEAK", "CANARY"].join("_");
    let export = export_public_replay(state.variant.id.clone(), &observer(), vec![step]).to_json();
    assert!(!export.contains(&canary), "{export}");
}

#[test]
fn action_tree_v1_parallel_vectors_cover_representative_trees() {
    let vectors = action_tree_v1_vectors();
    let missing = vectors
        .iter()
        .filter(|vector| vector.expected_hash == HashValue(0))
        .map(|vector| {
            format!(
                "{} bytes={} hash={} local_hash={} paths={:?}",
                vector.name,
                vector.bytes.len(),
                vector.hash.0,
                vector.local_hash.0,
                vector.paths
            )
        })
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "populate v1 vectors:\n{}",
        missing.join("\n")
    );

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

fn hidden_future_probes(state: &FloodWatchState) -> Vec<LeakProbe<usize, String, EventCard>> {
    state
        .event_deck_internal()
        .iter()
        .enumerate()
        .filter(|(_, card)| state.forecast.as_ref() != Some(*card))
        .map(|(index, card)| LeakProbe {
            source_seat: index,
            canary_id: card.stable_id(),
            canary: card.clone(),
        })
        .collect()
}

fn snapshot_contains_event(snapshot: &str, card: &EventCard) -> bool {
    snapshot.contains(&card.stable_id()) || snapshot.contains(&format!("{card:?}"))
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
    let driver = ReplayCommandV1Driver::new("flood_watch");
    assert_eq!(
        driver
            .validate(&artifact)
            .expect_err("invalid replay-command-v1 metadata rejects")
            .kind,
        expected
    );
}

fn public_export_profile_artifact<'a>(
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

fn assert_public_export_profile_rejects(
    artifact: ProfileArtifact<'_>,
    expected: ProfileValidationErrorKind,
) {
    let driver = PublicExportV1Driver::new("flood_watch");
    assert_eq!(
        driver
            .validate(&artifact)
            .expect_err("invalid public-export-v1 metadata rejects")
            .kind,
        expected
    );
}

fn validate_public_export_profile() -> PublicExportProfileSummary {
    let mut state = setup_match(Seed(91), &seats(), &SetupOptions::default()).unwrap();
    let command = end_turn_command(&state);
    let applied = apply_command(&mut state, &command).expect("profile command applies");
    let step = public_replay_step(0, &state, &command, &applied.effects, &observer());
    let hidden_future = hidden_future_probes(&state);
    let export = export_public_replay(state.variant.id.clone(), &observer(), vec![step]);
    let imported = import_public_export(&export);
    let json = export.to_json();

    assert_eq!(export.viewer, "observer");
    assert!(json.contains("forecast"));
    assert_eq!(imported.raw_json, json);
    assert!(!json.contains("full_deck_order"));
    assert!(!json.contains("deck_order"));
    for probe in &hidden_future {
        assert!(!snapshot_contains_event(&json, &probe.canary), "{json}");
    }
    let public_export_hash = export.stable_hash();

    PublicExportProfileSummary {
        viewer: export.viewer,
        step_count: export.steps.len(),
        public_export_hash,
        hidden_future_cards_checked: hidden_future.len(),
    }
}

fn native_replay_profile_hashes() -> Vec<HashValue> {
    let mut state = setup_match(Seed(91), &seats(), &SetupOptions::default()).unwrap();
    let trace = generate_internal_full_trace(91, &state);
    let command = end_turn_command(&state);
    let applied = apply_command(&mut state, &command).expect("profile command applies");
    let tree = legal_action_tree(&state, &actor("seat_1"));
    let step = public_replay_step(0, &state, &command, &applied.effects, &observer());
    let export = export_public_replay(state.variant.id.clone(), &observer(), vec![step]);

    vec![
        trace.stable_hash(),
        state.stable_hash(),
        HashValue::from_stable_bytes(format!("{:?}", applied.effects).as_bytes()),
        action_tree_hash(&tree),
        export.stable_hash(),
    ]
}

#[derive(Debug, Eq, PartialEq)]
struct SetupEvidenceSummary {
    fixture_count: usize,
    role_order_count: usize,
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

fn validate_setup_fixtures() -> SetupEvidenceSummary {
    let fixtures = [
        load_standard_fixture().expect("standard fixture parses"),
        load_deluge_fixture().expect("deluge fixture parses"),
    ];

    for fixture in fixtures {
        let variant = ScenarioVariant::resolve(&fixture.variant).expect("fixture variant resolves");
        let state = setup_match(
            Seed(11),
            &seats(),
            &SetupOptions {
                variant: variant.clone(),
            },
        )
        .expect("fixture setup succeeds");

        assert_eq!(fixture.game_id, GAME_ID);
        assert_eq!(fixture.rules_version, RULES_VERSION_LABEL);
        assert_eq!(fixture.phase, "action");
        assert_eq!(fixture.active_seat, "seat_0");
        assert_eq!(fixture.action_budget, variant.action_budget);
        assert_eq!(fixture.draws_per_phase, variant.draws_per_phase);
        assert_eq!(fixture.levee_cap, variant.levee_cap);
        assert_eq!(fixture.max_flood_level, variant.max_flood_level);
        assert_eq!(fixture.starting_levels, variant.starting_levels);
        assert_eq!(fixture.event_composition, variant.event_composition);
        assert_eq!(fixture.event_deck_order_status, "computed_from_seed");
        assert_eq!(fixture.terminal_outcome, "none");
        assert_eq!(state.variant.id, fixture.variant);
        assert_eq!(state.seats, seats());
        assert_eq!(state.roles, variant.role_order);
        assert_eq!(state.active_seat, SeatId("seat_0".to_owned()));
        assert_eq!(
            state.phase,
            Phase::Action {
                budget_remaining: 3
            }
        );
        assert_eq!(
            state
                .districts
                .iter()
                .map(|district| district.flood_level)
                .collect::<Vec<_>>(),
            fixture.starting_levels
        );
        assert_eq!(
            state.roles,
            [FloodWatchRole::Pumpwright, FloodWatchRole::LeveeWarden]
        );
    }

    SetupEvidenceSummary {
        fixture_count: 2,
        role_order_count: FloodWatchRole::ALL.len(),
    }
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
    let bail_and_levee = setup_match(Seed(18), &seats(), &SetupOptions::default()).unwrap();

    let mut role_power = setup_match(Seed(18), &seats(), &SetupOptions::default()).unwrap();
    role_power.active_seat = SeatId("seat_1".to_owned());

    let early_end = state_after_commands(18, &[vec![ACTION_END_TURN.to_owned()]]);

    let budget_exhausted = state_after_commands(
        19,
        &[vec![
            ACTION_REINFORCE.to_owned(),
            DistrictId::Riverside.as_str().to_owned(),
        ]],
    );

    let terminal = terminal_state();

    vec![
        vector(
            "bail_and_place_levee",
            &bail_and_levee,
            "seat_0",
            3920,
            HashValue(2247660004428458771),
            HashValue(4425850002041434203),
            vec![
                vec!["bail/district_old_docks".to_owned()],
                vec!["bail/district_terraces".to_owned()],
                vec!["reinforce/district_riverside".to_owned()],
                vec!["reinforce/district_old_docks".to_owned()],
                vec!["reinforce/district_market".to_owned()],
                vec!["reinforce/district_terraces".to_owned()],
                vec!["reinforce/district_gardens".to_owned()],
                vec!["forecast".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "role_power_levee_warden",
            &role_power,
            "seat_1",
            3920,
            HashValue(4532944654053335564),
            HashValue(8946559128574054524),
            vec![
                vec!["bail/district_old_docks".to_owned()],
                vec!["bail/district_terraces".to_owned()],
                vec!["reinforce/district_riverside".to_owned()],
                vec!["reinforce/district_old_docks".to_owned()],
                vec!["reinforce/district_market".to_owned()],
                vec!["reinforce/district_terraces".to_owned()],
                vec!["reinforce/district_gardens".to_owned()],
                vec!["forecast".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "early_end_next_turn",
            &early_end,
            "seat_1",
            4375,
            HashValue(6356390137971522057),
            HashValue(13133754107875012264),
            vec![
                vec!["bail/district_old_docks".to_owned()],
                vec!["bail/district_terraces".to_owned()],
                vec!["bail/district_gardens".to_owned()],
                vec!["reinforce/district_riverside".to_owned()],
                vec!["reinforce/district_old_docks".to_owned()],
                vec!["reinforce/district_market".to_owned()],
                vec!["reinforce/district_terraces".to_owned()],
                vec!["reinforce/district_gardens".to_owned()],
                vec!["forecast".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "budget_exhausted_auto_environment",
            &budget_exhausted,
            "seat_1",
            64,
            HashValue(828296343441045014),
            HashValue(9791162161922510910),
            Vec::new(),
        ),
        vector(
            "terminal_empty_tree",
            &terminal,
            "seat_0",
            64,
            HashValue(828296343441045014),
            HashValue(9791162161922510910),
            Vec::new(),
        ),
    ]
}

fn vector(
    name: &'static str,
    state: &FloodWatchState,
    seat: &str,
    expected_bytes_len: usize,
    expected_hash: HashValue,
    expected_local_hash: HashValue,
    expected_paths: Vec<Vec<String>>,
) -> ActionTreeV1Vector {
    let tree = legal_action_tree(state, &actor(seat));
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

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn state_after_commands(seed: u64, commands: &[Vec<String>]) -> FloodWatchState {
    let mut state = setup_match(Seed(seed), &seats(), &SetupOptions::default()).unwrap();
    for segments in commands {
        let command = CommandEnvelope {
            actor: Actor {
                seat_id: state.active_seat.clone(),
            },
            action_path: ActionPath {
                segments: segments.clone(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        };
        apply_command(&mut state, &command).expect("test command applies");
    }
    state
}

fn terminal_state() -> FloodWatchState {
    let deck = vec![card(
        EventKind::StormSurge {
            district: DistrictId::OldDocks,
        },
        1,
    )];
    let mut state = FloodWatchState::new_after_setup(ScenarioVariant::standard(), seats(), deck);
    let command = end_turn_command(&state);
    apply_command(&mut state, &command).expect("terminal command applies");
    assert_eq!(state.phase, Phase::Terminal);
    state
}

fn action_paths(choices: &[engine_core::ActionChoice]) -> Vec<Vec<String>> {
    choices
        .iter()
        .map(|choice| vec![choice.segment.clone()])
        .collect()
}
