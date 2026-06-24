use engine_core::{
    ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, StableSerialize, Viewer,
};
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind, PublicExportV1Driver,
    ReplayCommandV1Driver, SeatPrivateExportV1Driver, PROFILE_VERSION_V1, PUBLIC_EXPORT_V1,
    REPLAY_COMMAND_V1, SEAT_PRIVATE_EXPORT_V1,
};
use secret_draft::{
    actions::{commit_segment, validate_command},
    apply_action,
    ids::DraftItemId,
    replay_support::{
        action_tree_hash, action_tree_v1_bytes, action_tree_v1_hash, effect_hash,
        export_public_replay, generate_internal_full_trace, import_public_export,
        replay_internal_full_trace, state_hash, view_hash, ReplayCommand, SecretDraftInternalTrace,
    },
    setup_match, Phase, SecretDraftSeat, SecretDraftState, SetupOptions, TerminalOutcome, GAME_ID,
    RULES_VERSION_LABEL,
};

#[derive(Debug)]
struct TraceFixture {
    id: String,
    kind: String,
    purpose: String,
    game_id: String,
    rules_version: String,
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
    expected_revealed_sequence: Vec<String>,
    terminal: bool,
    winner: Option<String>,
    draw: bool,
}

#[derive(Debug)]
struct TraceCommand {
    actor_seat: SecretDraftSeat,
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
    revealed_sequence: Vec<String>,
    terminal: bool,
    outcome: Option<TerminalOutcome>,
    export_json: String,
}

#[test]
fn golden_traces_match_expected_replay_hashes_diagnostics_and_no_leak_surfaces() {
    let fixtures = [
        include_str!("golden_traces/shortest-normal.trace.json"),
        include_str!("golden_traces/first-commit-pending.trace.json"),
        include_str!("golden_traces/simultaneous-reveal-batch.trace.json"),
        include_str!("golden_traces/contested-pick-fallback.trace.json"),
        include_str!("golden_traces/terminal-tie-break.trace.json"),
        include_str!("golden_traces/draw-after-tie-breaks.trace.json"),
        include_str!("golden_traces/already-committed-diagnostic.trace.json"),
        include_str!("golden_traces/unavailable-item-diagnostic.trace.json"),
        include_str!("golden_traces/stale-diagnostic.trace.json"),
        include_str!("golden_traces/public-observer-no-leak.trace.json"),
        include_str!("golden_traces/seat-private-no-prereveal-choice.trace.json"),
        include_str!("golden_traces/bot-action.trace.json"),
        include_str!("golden_traces/public-replay-export-import.trace.json"),
        include_str!("golden_traces/wasm-exported.trace.json"),
    ];

    assert_eq!(fixtures.len(), 14);
    for input in fixtures {
        let fixture = parse_trace_fixture(input);
        assert_trace_fixture(&fixture);
    }
}

#[test]
fn action_tree_v1_bytes_and_hashes_are_pinned_alongside_legacy_hashes() {
    let mut state = setup_state();
    let seat_0_actor = Actor {
        seat_id: state.seats[SecretDraftSeat::Seat0.index()].clone(),
    };
    let first_commit_tree = secret_draft::legal_action_tree(&state, &seat_0_actor);

    assert_eq!(first_commit_tree.root.choices.len(), 12);
    assert_eq!(
        action_tree_hash(&first_commit_tree),
        HashValue(11109919055145097380)
    );
    assert_eq!(action_tree_v1_bytes(&first_commit_tree).len(), 7507);
    assert_eq!(
        action_tree_v1_hash(&first_commit_tree),
        HashValue(4430331744477066435)
    );

    let command = CommandEnvelope {
        actor: seat_0_actor,
        action_path: ActionPath {
            segments: vec![first_commit_tree.root.choices[0].segment.clone()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    let action = validate_command(&state, &command).expect("first commit validates");
    apply_action(&mut state, action).expect("first commit applies");

    let seat_1_actor = Actor {
        seat_id: state.seats[SecretDraftSeat::Seat1.index()].clone(),
    };
    let pending_second_commit_tree = secret_draft::legal_action_tree(&state, &seat_1_actor);

    assert_eq!(pending_second_commit_tree.root.choices.len(), 12);
    assert_eq!(
        action_tree_hash(&pending_second_commit_tree),
        HashValue(8995662196078409061)
    );
    assert_eq!(
        action_tree_v1_bytes(&pending_second_commit_tree).len(),
        7507
    );
    assert_eq!(
        action_tree_v1_hash(&pending_second_commit_tree),
        HashValue(4781253235714578176)
    );
}

#[test]
fn replay_command_v1_profile_driver_wraps_internal_trace_validator() {
    let trace = generate_internal_full_trace();
    let driver = ReplayCommandV1Driver::new("secret_draft");
    let artifact = replay_command_profile_artifact(
        REPLAY_COMMAND_V1,
        "secret_draft",
        &["commands", "checkpoints", "expected_hashes"],
    );

    let report = driver
        .validate(&artifact)
        .expect("profile metadata validates");
    assert_eq!(report.profile_id, REPLAY_COMMAND_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "internal-dev");
    assert_eq!(report.validator_owner, "secret_draft");

    let replay_hash = driver
        .validate_with(&artifact, |_| {
            let replay = replay_internal_full_trace(&trace);
            replay.trace_hash
        })
        .expect("profile delegates to internal trace validator");
    assert_eq!(replay_hash, trace.stable_hash());
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let wrong_profile =
        replay_command_profile_artifact("public-export-v1", "secret_draft", &["commands"]);
    assert_eq!(
        driver
            .validate(&wrong_profile)
            .expect_err("wrong profile id rejects")
            .kind,
        ProfileValidationErrorKind::WrongProfileId
    );

    let wrong_owner = replay_command_profile_artifact(REPLAY_COMMAND_V1, "other", &["commands"]);
    assert_eq!(
        driver
            .validate(&wrong_owner)
            .expect_err("wrong owner rejects")
            .kind,
        ProfileValidationErrorKind::WrongValidatorOwner
    );

    let wrong_field = replay_command_profile_artifact(
        REPLAY_COMMAND_V1,
        "secret_draft",
        &["commands", "export_steps"],
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
fn public_export_v1_profile_driver_wraps_observer_export_validator() {
    let trace = SecretDraftInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: secret_draft::VARIANT_ID.to_owned(),
        seed_evidence: 77,
        commands: vec![ReplayCommand {
            actor: SecretDraftSeat::Seat0.as_str().to_owned(),
            path: vec![commit_segment(DraftItemId::Ember4)],
        }],
    };
    let public_export = export_public_replay(&trace, &Viewer { seat_id: None });
    let driver = PublicExportV1Driver::new("secret_draft");
    let artifact = public_export_profile_artifact(
        PUBLIC_EXPORT_V1,
        Some("public"),
        "secret_draft",
        &["export_steps", "import_round_trip", "hidden_absence_tokens"],
    );

    let report = driver
        .validate(&artifact)
        .expect("profile metadata validates");
    assert_eq!(report.profile_id, PUBLIC_EXPORT_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "public");
    assert_eq!(report.validator_owner, "secret_draft");

    let export_hash = driver
        .validate_with(&artifact, |_| public_export.stable_hash())
        .expect("profile delegates to observer export validator");
    assert_eq!(export_hash, HashValue(5995340232186846963));
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let export_json = public_export.to_json();
    assert_eq!(public_export.viewer, "observer");
    assert!(export_json.contains("commit_redacted"));
    assert!(!export_json.contains("ember_4"));
    assert!(!export_json.contains("commit/ember_4"));
    assert!(!export_json.contains("seed"));
    assert!(!export_json.contains("77"));

    let wrong_profile = public_export_profile_artifact(
        "replay-command-v1",
        Some("public"),
        "secret_draft",
        &["export_steps"],
    );
    assert_eq!(
        driver
            .validate(&wrong_profile)
            .expect_err("wrong profile id rejects")
            .kind,
        ProfileValidationErrorKind::WrongProfileId
    );

    let wrong_owner = public_export_profile_artifact(
        PUBLIC_EXPORT_V1,
        Some("public"),
        "other",
        &["export_steps"],
    );
    assert_eq!(
        driver
            .validate(&wrong_owner)
            .expect_err("wrong owner rejects")
            .kind,
        ProfileValidationErrorKind::WrongValidatorOwner
    );

    let wrong_visibility = public_export_profile_artifact(
        PUBLIC_EXPORT_V1,
        Some("seat-private"),
        "secret_draft",
        &["export_steps"],
    );
    assert_eq!(
        driver
            .validate(&wrong_visibility)
            .expect_err("wrong visibility rejects")
            .kind,
        ProfileValidationErrorKind::InvalidVisibility
    );

    let wrong_field = public_export_profile_artifact(
        PUBLIC_EXPORT_V1,
        Some("public"),
        "secret_draft",
        &["export_steps", "commands"],
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
fn seat_private_export_v1_profile_driver_wraps_viewer_scoped_exports() {
    let trace = SecretDraftInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: secret_draft::VARIANT_ID.to_owned(),
        seed_evidence: 77,
        commands: vec![ReplayCommand {
            actor: SecretDraftSeat::Seat0.as_str().to_owned(),
            path: vec![commit_segment(DraftItemId::Ember4)],
        }],
    };
    let driver = SeatPrivateExportV1Driver::new("secret_draft");
    let artifact = seat_private_export_profile_artifact(
        SEAT_PRIVATE_EXPORT_V1,
        Some("seat-private"),
        "secret_draft",
        &[
            "viewer_seat",
            "viewer_seat_version",
            "export_steps",
            "pairwise_no_leak",
        ],
    );

    let report = driver
        .validate(&artifact)
        .expect("profile metadata validates");
    assert_eq!(report.profile_id, SEAT_PRIVATE_EXPORT_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "seat-private");
    assert_eq!(report.validator_owner, "secret_draft");

    for seat in ["seat_0", "seat_1"] {
        let export = export_public_replay(
            &trace,
            &Viewer {
                seat_id: Some(SeatId(seat.to_owned())),
            },
        );
        let export_hash = driver
            .validate_with(&artifact, |_| export.stable_hash())
            .expect("profile delegates to viewer-scoped export validator");
        assert_eq!(export_hash, export.stable_hash());
        assert_eq!(export.viewer, seat);

        let export_json = export.to_json();
        assert!(export_json.contains("commit_redacted"));
        assert!(!export_json.contains("ember_4"));
        assert!(!export_json.contains("commit/ember_4"));
        assert!(!export_json.contains("seed"));
        assert!(!export_json.contains("77"));
    }
    assert_eq!(artifact.metadata.canonical_byte_authority, "none");
    assert!(!artifact.canonical_byte_claim);

    let wrong_profile = seat_private_export_profile_artifact(
        "public-export-v1",
        Some("seat-private"),
        "secret_draft",
        &["export_steps"],
    );
    assert_eq!(
        driver
            .validate(&wrong_profile)
            .expect_err("wrong profile id rejects")
            .kind,
        ProfileValidationErrorKind::WrongProfileId
    );

    let wrong_owner = seat_private_export_profile_artifact(
        SEAT_PRIVATE_EXPORT_V1,
        Some("seat-private"),
        "other",
        &["export_steps"],
    );
    assert_eq!(
        driver
            .validate(&wrong_owner)
            .expect_err("wrong owner rejects")
            .kind,
        ProfileValidationErrorKind::WrongValidatorOwner
    );

    let wrong_visibility = seat_private_export_profile_artifact(
        SEAT_PRIVATE_EXPORT_V1,
        Some("public"),
        "secret_draft",
        &["export_steps"],
    );
    assert_eq!(
        driver
            .validate(&wrong_visibility)
            .expect_err("wrong visibility rejects")
            .kind,
        ProfileValidationErrorKind::InvalidVisibility
    );

    let wrong_field = seat_private_export_profile_artifact(
        SEAT_PRIVATE_EXPORT_V1,
        Some("seat-private"),
        "secret_draft",
        &["export_steps", "commands"],
    );
    assert_eq!(
        driver
            .validate(&wrong_field)
            .expect_err("wrong field rejects")
            .kind,
        ProfileValidationErrorKind::UnknownField
    );
}

fn replay_command_profile_artifact<'a>(
    profile_id: &'a str,
    validator_owner: &'a str,
    fields: &'a [&'a str],
) -> ProfileArtifact<'a> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("internal-dev"),
            validator_owner,
            canonical_byte_authority: "none",
            migration_update_note: Some("profile migration reviewed"),
        },
        fields,
        canonical_byte_claim: false,
    }
}

fn public_export_profile_artifact<'a>(
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

fn seat_private_export_profile_artifact<'a>(
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

fn assert_trace_fixture(fixture: &TraceFixture) {
    assert_eq!(fixture.game_id, GAME_ID, "{} game id", fixture.id);
    assert_eq!(
        fixture.rules_version, RULES_VERSION_LABEL,
        "{} rules version",
        fixture.id
    );
    assert!(fixture.seed > 0, "{} seed evidence", fixture.id);
    assert!(!fixture.kind.is_empty(), "{} kind", fixture.id);

    let first = replay_fixture(fixture);
    let second = replay_fixture(fixture);
    assert_eq!(
        first.state_hash, second.state_hash,
        "{} deterministic state",
        fixture.id
    );
    assert_eq!(
        first.effect_hash, second.effect_hash,
        "{} deterministic effects",
        fixture.id
    );
    assert_eq!(
        first.action_tree_hash, second.action_tree_hash,
        "{} deterministic action tree",
        fixture.id
    );
    assert_eq!(
        first.observer_view_hash, second.observer_view_hash,
        "{} deterministic observer view",
        fixture.id
    );

    assert_eq!(
        first.state_hash,
        HashValue(fixture.expected_state_hash),
        "{} state",
        fixture.id
    );
    assert_eq!(
        first.effect_hash,
        HashValue(fixture.expected_effect_hash),
        "{} effects",
        fixture.id
    );
    assert_eq!(
        first.action_tree_hash,
        HashValue(fixture.expected_action_tree_hash),
        "{} action tree",
        fixture.id
    );
    assert_eq!(
        first.observer_view_hash,
        HashValue(fixture.expected_observer_view_hash),
        "{} observer view",
        fixture.id
    );
    assert_eq!(
        first.seat_0_view_hash,
        HashValue(fixture.expected_seat_0_view_hash),
        "{} seat 0 view",
        fixture.id
    );
    assert_eq!(
        first.seat_1_view_hash,
        HashValue(fixture.expected_seat_1_view_hash),
        "{} seat 1 view",
        fixture.id
    );
    assert_eq!(
        first.replay_hash,
        HashValue(fixture.expected_replay_hash),
        "{} replay",
        fixture.id
    );
    assert_eq!(
        first.public_export_hash,
        HashValue(fixture.expected_public_export_hash),
        "{} public export",
        fixture.id
    );
    assert_eq!(
        first.diagnostic_hash,
        fixture.expected_diagnostic_hash.map(HashValue),
        "{} diagnostic hash",
        fixture.id
    );
    assert_eq!(
        first.diagnostic_code.as_deref(),
        fixture.expected_diagnostic_code.as_deref(),
        "{} diagnostic code",
        fixture.id
    );
    assert_eq!(
        first.revealed_sequence, fixture.expected_revealed_sequence,
        "{} reveal sequence",
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
        matches!(first.outcome, Some(TerminalOutcome::Draw)),
        fixture.draw,
        "{} draw",
        fixture.id
    );

    if fixture.purpose.contains("no_leak")
        || fixture.purpose == "first_commit_pending"
        || fixture.purpose == "public_replay_export_import"
    {
        let hidden = hidden_item(fixture);
        assert!(
            !first.export_json.contains(hidden.as_str()),
            "{} export leak",
            fixture.id
        );
        assert!(
            !first.export_json.contains(hidden.label()),
            "{} export label leak",
            fixture.id
        );
    }

    if fixture.purpose == "public_replay_export_import" {
        let trace = internal_trace_from_fixture(fixture);
        let export = export_public_replay(&trace, &Viewer { seat_id: None });
        let imported = import_public_export(&export);
        assert_eq!(imported.viewer, "observer");
        assert_eq!(imported.steps, export.steps);
    }

    if fixture.purpose == "contested_pick_fallback" {
        assert_eq!(first.revealed_sequence.len(), 1);
        assert!(first.revealed_sequence[0].contains("ember_4:ember_4"));
        assert!(first.revealed_sequence[0].ends_with(":ember_4:ember_1"));
    }
}

fn replay_fixture(fixture: &TraceFixture) -> ReplayActual {
    let mut state = setup_state();
    let mut effects = Vec::new();
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

    let trace = SecretDraftInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: secret_draft::VARIANT_ID.to_owned(),
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
        revealed_sequence: revealed_sequence(&state),
        terminal: state.phase == Phase::Terminal,
        outcome: state.terminal_outcome,
        export_json: export.to_json(),
    }
}

fn setup_state() -> SecretDraftState {
    setup_match(
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn export_viewer_for_fixture(fixture: &TraceFixture) -> Viewer {
    if fixture.purpose == "seat_private_no_prereveal_choice" {
        Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        }
    } else {
        Viewer { seat_id: None }
    }
}

fn command_envelope(state: &SecretDraftState, command: &TraceCommand) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[command.actor_seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: command.action_path.clone(),
        },
        freshness_token: engine_core::FreshnessToken(command.freshness_token),
        rules_version: RulesVersion(1),
    }
}

fn combined_action_tree_hash(state: &SecretDraftState) -> HashValue {
    let parts = SecretDraftSeat::ALL
        .iter()
        .map(|seat| {
            let actor = Actor {
                seat_id: state.seats[seat.index()].clone(),
            };
            action_tree_hash(&secret_draft::legal_action_tree(state, &actor))
                .0
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(parts.as_bytes())
}

fn internal_trace_from_fixture(fixture: &TraceFixture) -> SecretDraftInternalTrace {
    SecretDraftInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: secret_draft::VARIANT_ID.to_owned(),
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

fn revealed_sequence(state: &SecretDraftState) -> Vec<String> {
    state
        .revealed_history
        .iter()
        .map(|round| {
            format!(
                "{}:{}:{}:{}:{}",
                round.round_number,
                round.seat_0_choice.as_str(),
                round.seat_1_choice.as_str(),
                round.seat_0_award.as_str(),
                round.seat_1_award.as_str()
            )
        })
        .collect()
}

fn winner(outcome: &Option<TerminalOutcome>) -> Option<&'static str> {
    match outcome {
        Some(TerminalOutcome::Win { seat }) => Some(seat.as_str()),
        _ => None,
    }
}

fn hidden_item(fixture: &TraceFixture) -> DraftItemId {
    let first = fixture.commands.first().expect("no-leak trace has command");
    parse_item_path(first.action_path.first().expect("command path has segment"))
}

fn parse_trace_fixture(input: &str) -> TraceFixture {
    assert_eq!(number_field(input, "schema_version"), 1);
    let public_view_hashes = object_body(input, "expected_public_view_hashes");
    let diagnostic_body = input.find("\"expected_diagnostics\":").and_then(|start| {
        let rest = &input[start..];
        (!rest.starts_with("\"expected_diagnostics\": null"))
            .then(|| object_body(rest, "expected_diagnostics"))
    });
    TraceFixture {
        id: string_field(input, "trace_id"),
        kind: string_field(input, "fixture_kind"),
        purpose: string_field(input, "purpose"),
        game_id: string_field(input, "game_id"),
        rules_version: string_field(input, "rules_version"),
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
        expected_revealed_sequence: string_array_field(input, "expected_revealed_sequence"),
        terminal: bool_field(&object_body(input, "expected_terminal_state"), "terminal"),
        winner: nullable_string_field(&object_body(input, "expected_terminal_state"), "winner"),
        draw: bool_field(&object_body(input, "expected_terminal_state"), "draw"),
    }
}

fn commands(input: &str) -> Vec<TraceCommand> {
    let body = array_body(input, "commands");
    split_top_level(&body, ',')
        .into_iter()
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let expected_code = optional_string_field(&entry, "expected_diagnostic_code");
            TraceCommand {
                actor_seat: parse_seat(&string_field(&entry, "actor_seat")),
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

fn parse_seat(value: &str) -> SecretDraftSeat {
    SecretDraftSeat::parse(value).expect("trace seat is valid")
}

fn parse_item_path(segment: &str) -> DraftItemId {
    DraftItemId::parse(segment.strip_prefix("commit/").expect("commit segment"))
        .expect("trace item is valid")
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

fn parse_json_string(input: &str) -> String {
    let body = input
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or_else(|| panic!("expected JSON string, got `{input}`"));
    let mut output = String::new();
    let mut chars = body.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            let escaped = chars.next().expect("escape has value");
            match escaped {
                'n' => output.push('\n'),
                other => output.push(other),
            }
        } else {
            output.push(ch);
        }
    }
    output
}
