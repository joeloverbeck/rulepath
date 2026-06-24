use engine_core::{
    ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, StableSerialize, Viewer,
};
use frontier_control::{
    action_tree_hash, action_tree_v1_bytes, action_tree_v1_hash, apply_command,
    export_public_replay, import_public_export, legal_action_tree, load_highlands_fixture,
    load_standard_fixture, public_replay_step, setup_match, FactionId, FrontierControlState, Phase,
    SetupOptions, SiteId, VariantMap, ACTION_END_TURN, ACTION_MARCH, GAME_ID, RULES_VERSION_LABEL,
};
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind, PublicExportV1Driver,
    ReplayCommandV1Driver, SetupEvidenceV1Driver, PROFILE_VERSION_V1, PUBLIC_EXPORT_V1,
    REPLAY_COMMAND_V1, SETUP_EVIDENCE_V1,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn end_turn_command(state: &frontier_control::FrontierControlState, seat: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: SeatId(seat.to_owned()),
        },
        action_path: ActionPath {
            segments: vec![ACTION_END_TURN.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn command(
    state: &FrontierControlState,
    faction: FactionId,
    segments: Vec<&str>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for(state, faction),
        action_path: ActionPath {
            segments: segments.into_iter().map(str::to_owned).collect(),
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PublicExportProfileSummary {
    step_count: usize,
    public_export_hash: HashValue,
    hidden_redaction_note: String,
}

#[test]
fn setup_and_replay_hashes_are_deterministic() {
    let first = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let second = setup_match(&seats(), &SetupOptions::default()).unwrap();
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn command_stream_reproduces_effects_state_and_public_export() {
    let mut first = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let mut second = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let first_command = end_turn_command(&first, "seat_1");
    let second_command = end_turn_command(&second, "seat_1");
    let first_applied = apply_command(&mut first, &first_command).unwrap();
    let second_applied = apply_command(&mut second, &second_command).unwrap();

    assert_eq!(first_applied.effects, second_applied.effects);
    assert_eq!(first.stable_hash(), second.stable_hash());

    let step = public_replay_step(
        0,
        &first,
        &first_command,
        &first_applied.effects,
        &Viewer { seat_id: None },
    );
    let export = export_public_replay(first.variant.id.clone(), vec![step]);
    assert_eq!(import_public_export(&export).raw_json, export.to_json());
}

#[test]
fn replay_command_v1_profile_driver_wraps_public_native_replay_evidence() {
    let driver = ReplayCommandV1Driver::new("frontier_control");
    let artifact = replay_command_profile_artifact(
        REPLAY_COMMAND_V1,
        PROFILE_VERSION_V1,
        Some("public"),
        "frontier_control",
        &["commands", "checkpoints", "expected_hashes"],
    );

    let report = driver
        .validate(&artifact)
        .expect("profile metadata validates");
    assert_eq!(report.profile_id, REPLAY_COMMAND_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "public");
    assert_eq!(report.validator_owner, "frontier_control");
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let hashes = driver
        .validate_with(&artifact, |_| native_replay_profile_hashes())
        .expect("profile delegates to native replay evidence");
    assert_eq!(hashes.len(), 4);

    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            "public-export-v1",
            PROFILE_VERSION_V1,
            Some("public"),
            "frontier_control",
            &["commands"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            "v2",
            Some("public"),
            "frontier_control",
            &["commands"],
        ),
        ProfileValidationErrorKind::WrongProfileVersion,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("seat-private"),
            "frontier_control",
            &["commands"],
        ),
        ProfileValidationErrorKind::InvalidVisibility,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("public"),
            "replay-check",
            &["commands"],
        ),
        ProfileValidationErrorKind::WrongValidatorOwner,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("public"),
            "frontier_control",
            &["commands", "export_steps"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn public_export_v1_profile_driver_wraps_frontier_control_public_exporter() {
    let driver = PublicExportV1Driver::new("frontier_control");
    let artifact = public_export_profile_artifact(
        PUBLIC_EXPORT_V1,
        PROFILE_VERSION_V1,
        Some("public"),
        "frontier_control",
        &[
            "export_steps",
            "import_round_trip",
            "hidden_absence_tokens",
            "not_applicable",
        ],
    );

    let report = driver
        .validate(&artifact)
        .expect("public export metadata validates");
    assert_eq!(report.profile_id, PUBLIC_EXPORT_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "public");
    assert_eq!(report.validator_owner, "frontier_control");
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let summary = driver
        .validate_with(&artifact, |_| validate_public_export_profile())
        .expect("profile delegates to Frontier Control public exporter");
    assert_eq!(summary.step_count, 1);
    assert_ne!(summary.public_export_hash.0, 0);
    assert!(summary
        .hidden_redaction_note
        .contains("perfect information"));

    assert_public_export_profile_rejects(
        public_export_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("public"),
            "frontier_control",
            &["export_steps"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_public_export_profile_rejects(
        public_export_profile_artifact(
            PUBLIC_EXPORT_V1,
            "v2",
            Some("public"),
            "frontier_control",
            &["export_steps"],
        ),
        ProfileValidationErrorKind::WrongProfileVersion,
    );
    assert_public_export_profile_rejects(
        public_export_profile_artifact(
            PUBLIC_EXPORT_V1,
            PROFILE_VERSION_V1,
            Some("internal-dev"),
            "frontier_control",
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
            "frontier_control",
            &["export_steps", "commands"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn setup_evidence_v1_profile_driver_wraps_standard_and_highlands_fixtures() {
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
    assert_eq!(summary.site_count, SiteId::ALL.len());

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
            "frontier_control",
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
    let opening = setup_match(&seats(), &SetupOptions::default()).unwrap();

    let mut clash_branch = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let first_march = command(
        &clash_branch,
        FactionId::Prospectors,
        vec![
            ACTION_MARCH,
            SiteId::BaseCamp.as_str(),
            SiteId::Ford.as_str(),
        ],
    );
    apply_command(&mut clash_branch, &first_march).expect("first march applies");

    let mut stake_available = setup_match(&seats(), &SetupOptions::default()).unwrap();
    stake_available
        .site_mut(SiteId::Ford)
        .expect("ford exists")
        .crews = 1;

    let mut dismantle_available = setup_match(&seats(), &SetupOptions::default()).unwrap();
    dismantle_available.active_faction = FactionId::Garrison;
    dismantle_available
        .site_mut(SiteId::Ford)
        .expect("ford exists")
        .stake = true;
    dismantle_available
        .site_mut(SiteId::Ford)
        .expect("ford exists")
        .guards = 1;

    let mut early_end = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let end_turn = command(&early_end, FactionId::Prospectors, vec![ACTION_END_TURN]);
    apply_command(&mut early_end, &end_turn).expect("end turn applies");

    let mut terminal = setup_match(&seats(), &SetupOptions::default()).unwrap();
    terminal.phase = Phase::Terminal;

    vec![
        vector(
            "opening_moves",
            &opening,
            FactionId::Prospectors,
            1291,
            HashValue(14934942909345403747),
            HashValue(16277890795749786444),
            vec![
                vec!["march/site_base_camp/site_ford".to_owned()],
                vec!["march/site_base_camp/site_timberline".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "move_clash_branch",
            &clash_branch,
            FactionId::Prospectors,
            3310,
            HashValue(4769522588459725601),
            HashValue(8239912348712405228),
            vec![
                vec!["march/site_base_camp/site_ford".to_owned()],
                vec!["march/site_base_camp/site_timberline".to_owned()],
                vec!["march/site_ford/site_gatehouse".to_owned()],
                vec!["march/site_ford/site_base_camp".to_owned()],
                vec!["march/site_ford/site_quarry".to_owned()],
                vec!["stake/site_ford".to_owned()],
                vec!["muster".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "stake_available",
            &stake_available,
            FactionId::Prospectors,
            2601,
            HashValue(12908324649299837008),
            HashValue(11013731039854121046),
            vec![
                vec!["march/site_base_camp/site_ford".to_owned()],
                vec!["march/site_base_camp/site_timberline".to_owned()],
                vec!["march/site_ford/site_gatehouse".to_owned()],
                vec!["march/site_ford/site_quarry".to_owned()],
                vec!["stake/site_ford".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "dismantle_available",
            &dismantle_available,
            FactionId::Garrison,
            5890,
            HashValue(4031145394212002295),
            HashValue(26708586450493490),
            vec![
                vec!["patrol/site_gatehouse/site_signal_hill".to_owned()],
                vec!["patrol/site_gatehouse/site_ford".to_owned()],
                vec!["patrol/site_gatehouse/site_quarry".to_owned()],
                vec!["patrol/site_signal_hill/site_gatehouse".to_owned()],
                vec!["patrol/site_signal_hill/site_quarry".to_owned()],
                vec!["patrol/site_signal_hill/site_goldfield".to_owned()],
                vec!["patrol/site_ford/site_gatehouse".to_owned()],
                vec!["patrol/site_ford/site_base_camp".to_owned()],
                vec!["patrol/site_ford/site_quarry".to_owned()],
                vec!["reinforce/site_gatehouse".to_owned()],
                vec!["reinforce/site_signal_hill".to_owned()],
                vec!["dismantle/site_ford".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "early_end_next_turn",
            &early_end,
            FactionId::Garrison,
            4092,
            HashValue(480402586032591446),
            HashValue(16861215057075239797),
            vec![
                vec!["patrol/site_gatehouse/site_signal_hill".to_owned()],
                vec!["patrol/site_gatehouse/site_ford".to_owned()],
                vec!["patrol/site_gatehouse/site_quarry".to_owned()],
                vec!["patrol/site_signal_hill/site_gatehouse".to_owned()],
                vec!["patrol/site_signal_hill/site_quarry".to_owned()],
                vec!["patrol/site_signal_hill/site_goldfield".to_owned()],
                vec!["reinforce/site_gatehouse".to_owned()],
                vec!["reinforce/site_signal_hill".to_owned()],
                vec!["end_turn".to_owned()],
            ],
        ),
        vector(
            "terminal_empty_tree",
            &terminal,
            FactionId::Prospectors,
            64,
            HashValue(17387353871007407771),
            HashValue(10022657772393329959),
            Vec::new(),
        ),
    ]
}

fn vector(
    name: &'static str,
    state: &FrontierControlState,
    faction: FactionId,
    expected_bytes_len: usize,
    expected_hash: HashValue,
    expected_local_hash: HashValue,
    expected_paths: Vec<Vec<String>>,
) -> ActionTreeV1Vector {
    let tree = legal_action_tree(state, &actor_for(state, faction));
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

fn actor_for(state: &FrontierControlState, faction: FactionId) -> Actor {
    Actor {
        seat_id: state
            .seats
            .iter()
            .find(|seat| state.faction_for_seat(seat) == Some(faction))
            .expect("seat exists")
            .clone(),
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
    let driver = ReplayCommandV1Driver::new("frontier_control");
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
    let driver = PublicExportV1Driver::new("frontier_control");
    assert_eq!(
        driver
            .validate(&artifact)
            .expect_err("invalid public-export-v1 metadata rejects")
            .kind,
        expected
    );
}

fn validate_public_export_profile() -> PublicExportProfileSummary {
    let mut state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let command = end_turn_command(&state, "seat_1");
    let applied = apply_command(&mut state, &command).expect("profile command applies");
    let step = public_replay_step(
        0,
        &state,
        &command,
        &applied.effects,
        &Viewer { seat_id: None },
    );
    let export = export_public_replay(state.variant.id.clone(), vec![step]);
    let imported = import_public_export(&export);
    let json = export.to_json();

    assert_eq!(imported.raw_json, json);
    assert_eq!(export.game_id, GAME_ID);
    assert_eq!(export.rules_version_label, RULES_VERSION_LABEL);
    assert_eq!(
        export.not_applicable.hidden_information_redaction,
        "not applicable: Frontier Control is perfect information and all game state is public"
    );
    assert!(json.contains("public_view_summary"));
    assert!(json.contains("not_applicable"));

    PublicExportProfileSummary {
        step_count: export.steps.len(),
        public_export_hash: export.stable_hash(),
        hidden_redaction_note: export.not_applicable.hidden_information_redaction,
    }
}

fn native_replay_profile_hashes() -> Vec<HashValue> {
    let mut state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let command = end_turn_command(&state, "seat_1");
    let applied = apply_command(&mut state, &command).expect("profile command applies");
    let tree = legal_action_tree(&state, &actor_for(&state, FactionId::Prospectors));
    let step = public_replay_step(
        0,
        &state,
        &command,
        &applied.effects,
        &Viewer { seat_id: None },
    );
    let export = export_public_replay(state.variant.id.clone(), vec![step]);

    vec![
        state.stable_hash(),
        HashValue::from_stable_bytes(format!("{:?}", applied.effects).as_bytes()),
        action_tree_hash(&tree),
        export.stable_hash(),
    ]
}

#[derive(Debug, Eq, PartialEq)]
struct SetupEvidenceSummary {
    fixture_count: usize,
    site_count: usize,
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
        load_highlands_fixture().expect("highlands fixture parses"),
    ];

    for fixture in fixtures {
        let variant = VariantMap::resolve(&fixture.variant).expect("fixture variant resolves");
        let state = setup_match(
            &seats(),
            &SetupOptions {
                variant: variant.clone(),
            },
        )
        .expect("fixture setup succeeds");

        assert_eq!(fixture.game_id, GAME_ID);
        assert_eq!(fixture.rules_version, RULES_VERSION_LABEL);
        assert_eq!(fixture.phase, "action");
        assert_eq!(fixture.active_seat, "seat_1");
        assert_eq!(fixture.action_budget, variant.action_budget);
        assert_eq!(fixture.round_count, variant.round_count);
        assert_eq!(fixture.unit_cap_per_site, variant.unit_cap_per_site);
        assert_eq!(fixture.edges, variant.edges);
        assert_eq!(fixture.fort_sites, variant.fort_sites);
        assert_eq!(fixture.base_camp, variant.base_camp);
        assert_eq!(fixture.stake_values, variant.stake_values);
        assert_eq!(fixture.start_units, variant.start_units);
        assert_eq!(fixture.terminal_outcome, variant.terminal_outcomes);
        assert_eq!(state.variant.id, fixture.variant);
        assert_eq!(state.seats, seats());
        assert_eq!(state.factions, variant.faction_order);
        assert_eq!(state.active_faction, FactionId::Prospectors);
        assert_eq!(
            state.phase,
            Phase::Action {
                budget_remaining: variant.action_budget,
            }
        );
        assert_eq!(state.adjacency.len(), SiteId::ALL.len());
        for (site, guards) in &fixture.start_units.guards {
            assert_eq!(
                state.site(*site).expect("guard site exists").guards,
                *guards
            );
        }
        for (site, crews) in &fixture.start_units.crews {
            assert_eq!(state.site(*site).expect("crew site exists").crews, *crews);
        }
    }

    SetupEvidenceSummary {
        fixture_count: 2,
        site_count: SiteId::ALL.len(),
    }
}

fn action_paths(choices: &[engine_core::ActionChoice]) -> Vec<Vec<String>> {
    choices
        .iter()
        .map(|choice| vec![choice.segment.clone()])
        .collect()
}
