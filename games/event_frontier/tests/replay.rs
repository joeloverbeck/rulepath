use engine_core::{
    ActionChoice, ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, Seed,
    StableSerialize, Viewer,
};
use game_test_support::no_leak::{
    assert_pairwise_no_leak, ExposureExpectation, LeakProbe,
};
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ProfileValidationErrorKind, ReplayCommandV1Driver,
    PROFILE_VERSION_V1, REPLAY_COMMAND_V1,
};
use event_frontier::{
    action_tree_hash, action_tree_v1_bytes, action_tree_v1_hash, apply_command,
    export_public_replay, generate_internal_full_trace, import_public_export,
    import_public_export_json, legal_action_tree, project_view, public_replay_step,
    resolve_reckoning, setup_match, ActiveEdict, CardId, CardPhase, EventFrontierState, FactionId,
    FirstChoice, SetupOptions, SiteId, ACTION_OPERATION, ACTION_PASS, TRACE_HIDDEN_SURFACE,
    TRACE_STOCHASTIC_SURFACE,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn observer() -> Viewer {
    Viewer { seat_id: None }
}

fn seat_viewer(seat: &str) -> Viewer {
    Viewer {
        seat_id: Some(SeatId(seat.to_owned())),
    }
}

fn pass_command(seat: &str, state: &event_frontier::EventFrontierState) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![ACTION_PASS.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn debug_hash<T: std::fmt::Debug>(value: &T) -> HashValue {
    HashValue::from_stable_bytes(format!("{value:?}").as_bytes())
}

fn command_segments(
    seat: &str,
    state: &EventFrontierState,
    segments: Vec<&str>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: segments.into_iter().map(str::to_owned).collect(),
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn scenario_options() -> [SetupOptions; 3] {
    [
        SetupOptions::standard(),
        SetupOptions::hard_winter(),
        SetupOptions::land_rush(),
    ]
}

#[test]
fn deterministic_setup_reproduces_deck_order_and_state_hash() {
    let seats = seats();

    for options in scenario_options() {
        let first = setup_match(Seed(99), &seats, &options).expect("first setup");
        let second = setup_match(Seed(99), &seats, &options).expect("second setup");

        assert_eq!(first.deck, second.deck);
        assert_eq!(first.stable_hash(), second.stable_hash());
        assert_eq!(first.stable_summary(), second.stable_summary());
    }
}

#[test]
fn reckoning_is_never_first_in_any_seeded_epoch() {
    let seats = seats();

    for options in scenario_options() {
        for seed in 0..150 {
            let state = setup_match(Seed(seed), &seats, &options).expect("setup");
            let mut deck = Vec::new();
            deck.extend(state.deck.current);
            deck.extend(state.deck.next_public);
            deck.extend(state.deck.undrawn);

            for epoch_start in [0, 7, 14] {
                assert!(!is_reckoning(deck[epoch_start]));
            }
        }
    }
}

fn is_reckoning(card: CardId) -> bool {
    matches!(
        card,
        CardId::ReckoningOne | CardId::ReckoningTwo | CardId::ReckoningThree
    )
}

#[test]
fn reckoning_breakdown_scores_and_terminal_reproduce_for_same_state() {
    let seats = seats();
    let mut first = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("setup");
    let mut second = first.clone();
    for state in [&mut first, &mut second] {
        state.deck.current = Some(CardId::ReckoningOne);
        state.card_phase = CardPhase::Reckoning;
    }

    let first_result = resolve_reckoning(&mut first).expect("first reckoning");
    let second_result = resolve_reckoning(&mut second).expect("second reckoning");

    assert_eq!(first.scores, second.scores);
    assert_eq!(first.terminal_outcome, second.terminal_outcome);
    assert_eq!(
        format!("{:?}", first_result.effects),
        format!("{:?}", second_result.effects)
    );
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn public_replay_export_import_reproduces_public_hashes_without_hidden_order() {
    let seats = seats();
    let mut state = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("setup");
    let hidden = state.deck.undrawn[0].as_str().to_owned();
    let command = pass_command("seat_1", &state);

    let applied = apply_command(&mut state, &command).expect("pass command applies");
    let step = public_replay_step(0, &state, &command, &applied.effects, &observer());
    let export = export_public_replay(state.variant.id.clone(), &observer(), vec![step]);

    let imported_from_struct = import_public_export(&export);
    let imported_from_json = import_public_export_json(&export.to_json()).expect("public import");

    assert_eq!(
        imported_from_struct.stable_hash(),
        imported_from_json.stable_hash()
    );
    assert_eq!(imported_from_struct.raw_json, export.to_json());
    assert_eq!(imported_from_json.raw_json, export.to_json());
    assert!(!export.stable_summary().contains(&hidden));
    assert!(!export.to_json().contains(&hidden));
    assert!(!imported_from_json.stable_summary().contains(&hidden));
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
            Self::Observer => observer(),
            Self::Seat0 => seat_viewer("seat_0"),
            Self::Seat1 => seat_viewer("seat_1"),
        }
    }
}

#[test]
fn public_replay_exports_do_not_leak_hidden_deeper_deck_cards() {
    let seats = seats();
    let mut state = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("setup");
    let command = pass_command("seat_1", &state);
    let applied = apply_command(&mut state, &command).expect("pass command applies");
    let probes = state
        .deck
        .undrawn
        .iter()
        .filter(|card| Some(**card) != state.deck.current)
        .filter(|card| Some(**card) != state.deck.next_public)
        .enumerate()
        .map(|(index, card)| LeakProbe {
            source_seat: index,
            canary_id: card.as_str(),
            canary: *card,
        })
        .collect::<Vec<_>>();
    let canary = ["R3", "EVENT", "NOLEAK", "CANARY"].join("_");

    assert_pairwise_no_leak(
        [ExportViewer::Observer, ExportViewer::Seat0, ExportViewer::Seat1],
        ["public_export_json"],
        probes,
        |viewer_case, _surface| {
            let viewer = viewer_case.viewer();
            let step = public_replay_step(0, &state, &command, &applied.effects, &viewer);
            let export = export_public_replay(state.variant.id.clone(), &viewer, vec![step]);
            format!("{}{}", export.to_json(), canary)
        },
        |_source, _viewer, _surface, _canary_id| ExposureExpectation::MustBeAbsent,
        |text, card| text.contains(card.as_str()),
    )
    .expect("Event Frontier public export no-leak matrix has no failures");
}

#[test]
fn replaying_same_seed_scenario_and_command_stream_reproduces_public_hashes() {
    let seats = seats();
    let mut first = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("first setup");
    let mut second = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("second setup");
    let first_command = pass_command("seat_1", &first);
    let second_command = pass_command("seat_1", &second);

    let first_applied = apply_command(&mut first, &first_command).expect("first command applies");
    let second_applied =
        apply_command(&mut second, &second_command).expect("second command applies");

    assert_eq!(first.stable_hash(), second.stable_hash());
    assert_eq!(
        debug_hash(&first_applied.effects),
        debug_hash(&second_applied.effects)
    );
    assert_eq!(
        debug_hash(&legal_action_tree(&first, &actor("seat_0"))),
        debug_hash(&legal_action_tree(&second, &actor("seat_0")))
    );
    assert_eq!(
        project_view(&first, &observer()).stable_hash(),
        project_view(&second, &observer()).stable_hash()
    );
}

#[test]
fn replay_command_v1_profile_driver_wraps_internal_native_replay_evidence() {
    let driver = ReplayCommandV1Driver::new("event_frontier");
    let artifact = replay_command_profile_artifact(
        REPLAY_COMMAND_V1,
        PROFILE_VERSION_V1,
        Some("internal-dev"),
        "event_frontier",
        &["commands", "checkpoints", "expected_hashes"],
    );

    let report = driver
        .validate(&artifact)
        .expect("profile metadata validates");
    assert_eq!(report.profile_id, REPLAY_COMMAND_V1);
    assert_eq!(report.profile_version, PROFILE_VERSION_V1);
    assert_eq!(report.visibility_class, "internal-dev");
    assert_eq!(report.validator_owner, "event_frontier");
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
            "event_frontier",
            &["commands"],
        ),
        ProfileValidationErrorKind::WrongProfileId,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            "v2",
            Some("internal-dev"),
            "event_frontier",
            &["commands"],
        ),
        ProfileValidationErrorKind::WrongProfileVersion,
    );
    assert_replay_profile_rejects(
        replay_command_profile_artifact(
            REPLAY_COMMAND_V1,
            PROFILE_VERSION_V1,
            Some("seat-private"),
            "event_frontier",
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
            "event_frontier",
            &["commands", "export_steps"],
        ),
        ProfileValidationErrorKind::UnknownField,
    );
}

#[test]
fn internal_trace_marks_hidden_and_stochastic_surfaces() {
    let seats = seats();
    let state = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("setup");
    let hidden = state.deck.undrawn[0].as_str();

    let trace = generate_internal_full_trace(1, &state);

    assert_eq!(trace.hidden_surface, TRACE_HIDDEN_SURFACE);
    assert_eq!(trace.stochastic_surface, TRACE_STOCHASTIC_SURFACE);
    assert_eq!(trace.per_seat_hidden_surface, "not_applicable");
    assert!(trace.full_deck_order.iter().any(|card| card == hidden));
}

#[test]
fn action_tree_v1_parallel_vectors_cover_representative_trees_without_hidden_deck_order() {
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
        assert!(
            !vector.contains_hidden_undrawn_card,
            "{} hidden deck order",
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
    contains_hidden_undrawn_card: bool,
    expected_bytes_len: usize,
    expected_hash: HashValue,
    expected_local_hash: HashValue,
    expected_paths: Vec<Vec<String>>,
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
    let driver = ReplayCommandV1Driver::new("event_frontier");
    assert_eq!(
        driver
            .validate(&artifact)
            .expect_err("invalid replay-command-v1 metadata rejects")
            .kind,
        expected
    );
}

fn native_replay_profile_hashes() -> Vec<HashValue> {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
    let trace = generate_internal_full_trace(1, &state);
    let command = pass_command("seat_1", &state);
    let applied = apply_command(&mut state, &command).expect("profile command applies");
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let step = public_replay_step(0, &state, &command, &applied.effects, &observer());
    let export = export_public_replay(state.variant.id.clone(), &observer(), vec![step]);

    vec![
        trace.stable_hash(),
        state.stable_hash(),
        debug_hash(&applied.effects),
        action_tree_hash(&tree),
        export.stable_hash(),
    ]
}

fn action_tree_v1_vectors() -> Vec<ActionTreeV1Vector> {
    let full_operation =
        setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup full operation");

    let mut limited_operation =
        setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup limited operation");
    let full_op = command_segments(
        "seat_1",
        &limited_operation,
        vec![
            ACTION_OPERATION,
            "cache",
            SiteId::Landing.as_str(),
        ],
    );
    apply_command(&mut limited_operation, &full_op).expect("full op");

    let mut event_choice =
        setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup event choice");
    event_choice.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };

    let mut pass_choice =
        setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup pass choice");
    pass_choice.card_phase = CardPhase::AwaitingSecondChoice {
        first_faction: FactionId::Charter,
        second_faction: FactionId::Freeholders,
        first_choice: FirstChoice::Event,
    };

    let mut edict_blocked = setup_match(Seed(1), &seats(), &SetupOptions::hard_winter())
        .expect("setup edict blocked");
    edict_blocked.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Freeholders,
    };
    edict_blocked
        .site_mut(SiteId::Landing)
        .expect("landing exists")
        .agents = 1;
    edict_blocked.active_edicts = vec![ActiveEdict {
        kind: event_frontier::cards::EdictKind::SurveyBan,
        card: CardId::SurveyBan,
        activation_index: 0,
        expires_at_reckoning: 1,
    }];

    let mut reckoning =
        setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup reckoning");
    reckoning.card_phase = CardPhase::Reckoning;

    let mut terminal =
        setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup terminal");
    terminal.card_phase = CardPhase::Terminal;

    vec![
        vector(
            "full_operation_multi_site",
            &full_operation,
            "seat_1",
            5724,
            HashValue(12263323764607805373),
            HashValue(12025048674674442718),
            path_strings(&[
                &["event"],
                &["operation", "trek", "operation/trek/site_landing>site_crossing"],
                &["operation", "trek", "operation/trek/site_landing>site_high_meadow"],
                &["operation", "trek", "operation/trek/site_high_meadow>site_old_mill"],
                &[
                    "operation",
                    "trek",
                    "operation/trek/site_landing>site_crossing,site_high_meadow>site_old_mill",
                ],
                &[
                    "operation",
                    "trek",
                    "operation/trek/site_landing>site_high_meadow,site_high_meadow>site_old_mill",
                ],
                &["operation", "cache", "operation/cache/site_landing"],
                &["operation", "cache", "operation/cache/site_high_meadow"],
                &["operation", "cache", "operation/cache/site_landing,site_high_meadow"],
                &["operation", "rally", "operation/rally/site_high_meadow"],
                &["pass"],
            ]),
        ),
        vector(
            "limited_operation_second_choice",
            &limited_operation,
            "seat_0",
            2688,
            HashValue(5287035841278219952),
            HashValue(1262519681689202196),
            path_strings(&[
                &["event"],
                &[
                    "limited_operation",
                    "survey",
                    "limited_operation/survey/site_charterhouse",
                ],
                &[
                    "limited_operation",
                    "survey",
                    "limited_operation/survey/site_crossing",
                ],
                &[
                    "limited_operation",
                    "survey",
                    "limited_operation/survey/site_granite_pass",
                ],
                &["pass"],
            ]),
        ),
        vector(
            "event_choice_charter",
            &event_choice,
            "seat_0",
            3776,
            HashValue(6239437208328345357),
            HashValue(12651858397689234283),
            path_strings(&[
                &["event"],
                &["operation", "survey", "operation/survey/site_charterhouse"],
                &["operation", "survey", "operation/survey/site_crossing"],
                &["operation", "survey", "operation/survey/site_granite_pass"],
                &[
                    "operation",
                    "survey",
                    "operation/survey/site_charterhouse,site_crossing",
                ],
                &[
                    "operation",
                    "survey",
                    "operation/survey/site_charterhouse,site_granite_pass",
                ],
                &[
                    "operation",
                    "survey",
                    "operation/survey/site_crossing,site_granite_pass",
                ],
                &["pass"],
            ]),
        ),
        vector(
            "pass_after_event",
            &pass_choice,
            "seat_1",
            5418,
            HashValue(18107612798635470515),
            HashValue(9761797406534023113),
            path_strings(&[
                &["operation", "trek", "operation/trek/site_landing>site_crossing"],
                &["operation", "trek", "operation/trek/site_landing>site_high_meadow"],
                &["operation", "trek", "operation/trek/site_high_meadow>site_old_mill"],
                &[
                    "operation",
                    "trek",
                    "operation/trek/site_landing>site_crossing,site_high_meadow>site_old_mill",
                ],
                &[
                    "operation",
                    "trek",
                    "operation/trek/site_landing>site_high_meadow,site_high_meadow>site_old_mill",
                ],
                &["operation", "cache", "operation/cache/site_landing"],
                &["operation", "cache", "operation/cache/site_high_meadow"],
                &["operation", "cache", "operation/cache/site_landing,site_high_meadow"],
                &["operation", "rally", "operation/rally/site_high_meadow"],
                &["pass"],
            ]),
        ),
        vector(
            "edict_blocked_survey",
            &edict_blocked,
            "seat_1",
            2924,
            HashValue(17452768486966187756),
            HashValue(16133410113579192678),
            path_strings(&[
                &["event"],
                &["operation", "trek", "operation/trek/site_landing>site_crossing"],
                &["operation", "trek", "operation/trek/site_landing>site_high_meadow"],
                &["operation", "cache", "operation/cache/site_landing"],
                &["pass"],
            ]),
        ),
        vector(
            "reckoning_empty_tree",
            &reckoning,
            "seat_0",
            64,
            HashValue(17387353871007407771),
            HashValue(10022657772393329959),
            Vec::new(),
        ),
        vector(
            "terminal_empty_tree",
            &terminal,
            "seat_0",
            64,
            HashValue(17387353871007407771),
            HashValue(10022657772393329959),
            Vec::new(),
        ),
    ]
}

fn vector(
    name: &'static str,
    state: &EventFrontierState,
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
        contains_hidden_undrawn_card: state
            .deck
            .undrawn
            .iter()
            .any(|card| bytes.windows(card.as_str().len()).any(|w| w == card.as_str().as_bytes())),
        bytes,
        expected_bytes_len,
        expected_hash,
        expected_local_hash,
        expected_paths,
    }
}

fn action_paths(choices: &[ActionChoice]) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    for choice in choices {
        collect_action_paths(choice, Vec::new(), &mut paths);
    }
    paths
}

fn path_strings(paths: &[&[&str]]) -> Vec<Vec<String>> {
    paths
        .iter()
        .map(|path| path.iter().map(|segment| (*segment).to_owned()).collect())
        .collect()
}

fn collect_action_paths(
    choice: &ActionChoice,
    mut prefix: Vec<String>,
    paths: &mut Vec<Vec<String>>,
) {
    prefix.push(choice.segment.clone());
    if let Some(next) = &choice.next {
        for child in &next.choices {
            collect_action_paths(child, prefix.clone(), paths);
        }
    } else {
        paths.push(prefix);
    }
}
