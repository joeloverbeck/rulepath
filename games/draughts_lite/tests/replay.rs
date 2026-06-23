use draughts_lite::replay_support::{
    action_tree_hash, action_tree_legacy_bytes, action_tree_v1_bytes, action_tree_v1_hash,
    diagnostic_hash, replay_from_state, DraughtsLiteReplayJson,
};
use draughts_lite::{
    apply_action, legal_action_tree, setup_match, validate_command, CellOccupancy,
    DraughtsLiteSeat, DraughtsLiteSnapshot, DraughtsLiteState, Piece, PieceId, PieceKind,
    SetupOptions, TerminalOutcome, Variant,
};
use engine_core::{
    ActionChoice, ActionMetadata, ActionNode, ActionPath, ActionTree, Actor, CommandEnvelope,
    FreshnessToken, HashValue, RulesVersion, SeatId, Seed, StableBytesRecordWriter,
    StableBytesTypeTag, StableBytesWriter, StableSerialize,
};
use game_stdlib::board_space::Coord;
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ReplayCommandV1Driver, PROFILE_VERSION_V1, REPLAY_COMMAND_V1,
};

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

#[derive(Debug)]
struct TraceFixture {
    id: String,
    kind: String,
    purpose: String,
    note: String,
    migration_update_note: String,
    game_id: String,
    rules_version: String,
    seed: u64,
    commands: Vec<TraceCommand>,
    expected_state_hash: u64,
    expected_effect_hash: u64,
    expected_action_tree_hash: u64,
    expected_view_hash: u64,
    expected_replay_hash: u64,
    expected_diagnostic_code: Option<String>,
    expected_diagnostic_hash: Option<u64>,
    terminal: bool,
    winner: Option<String>,
}

#[derive(Debug)]
struct TraceCommand {
    actor_seat: String,
    action_path: Vec<String>,
    freshness_token: u64,
    expect: String,
    expected_diagnostic_code: Option<String>,
}

#[test]
fn replay_json_stable_serialization_preserves_multi_segment_paths() {
    let replay = DraughtsLiteReplayJson {
        schema_version: 1,
        game_id: "draughts_lite".to_owned(),
        rules_version: "draughts_lite-rules-v1".to_owned(),
        variant: "draughts_lite_standard".to_owned(),
        seed: 4,
        initial_snapshot: "snapshot".to_owned(),
        command_paths: vec![vec![
            "from/r3c2".to_owned(),
            "jump/r5c4".to_owned(),
            "jump/r7c6".to_owned(),
        ]],
    };

    let json = replay.to_json();

    assert_eq!(json.as_bytes(), replay.stable_bytes());
    assert!(json.contains("\"command_paths\":[[\"from/r3c2\",\"jump/r5c4\",\"jump/r7c6\"]]"));
    assert!(!json.contains("debug"));
    assert!(!json.contains("hidden"));
}

#[test]
fn replay_standard_fixture_metadata_is_present_and_public_safe() {
    let fixture = include_str!("../data/fixtures/draughts_lite_standard.fixture.json");

    assert_eq!(
        string_field(fixture, "fixture_id"),
        "draughts_lite_standard_gate7"
    );
    assert_eq!(string_field(fixture, "game_id"), "draughts_lite");
    assert_eq!(string_field(fixture, "variant"), "draughts_lite_standard");
    assert_eq!(
        string_field(fixture, "rules_version"),
        "draughts_lite-rules-v1"
    );
    assert_eq!(number_field(fixture, "trace_schema_version"), 1);
    assert!(fixture.contains("\"fixture_kinds\""));
    assert!(!fixture.contains("private"));
    assert!(!fixture.contains("debug"));
}

#[test]
fn golden_traces_match_expected_replay_hashes_diagnostics_and_bot_choice() {
    for fixture in [
        include_str!("golden_traces/shortest-quiet.trace.json"),
        include_str!("golden_traces/mandatory-capture-suppresses-quiet.trace.json"),
        include_str!("golden_traces/single-capture.trace.json"),
        include_str!("golden_traces/multi-jump.trace.json"),
        include_str!("golden_traces/forced-continuation-branch.trace.json"),
        include_str!("golden_traces/promotion-quiet.trace.json"),
        include_str!("golden_traces/promotion-during-capture-stop.trace.json"),
        include_str!("golden_traces/terminal-no-pieces.trace.json"),
        include_str!("golden_traces/terminal-no-legal-moves.trace.json"),
        include_str!("golden_traces/stale-diagnostic.trace.json"),
        include_str!("golden_traces/non-active-seat-diagnostic.trace.json"),
        include_str!("golden_traces/occupied-destination-diagnostic.trace.json"),
        include_str!("golden_traces/non-playable-cell-diagnostic.trace.json"),
        include_str!("golden_traces/quiet-while-capture-diagnostic.trace.json"),
        include_str!("golden_traces/illegal-continuation-diagnostic.trace.json"),
        include_str!("golden_traces/path-after-promotion-stop-diagnostic.trace.json"),
        include_str!("golden_traces/bot-action.trace.json"),
        include_str!("golden_traces/wasm-exported.trace.json"),
    ] {
        assert_fixture(parse_trace_schema_v1_fixture(fixture));
    }
}

#[test]
fn characterization_compound_action_tree_legacy_hash_is_pinned() {
    let state = initial_state("multi_jump", 7);
    let actor = Actor {
        seat_id: SeatId("seat-0".to_owned()),
    };
    let tree = legal_action_tree(&state, &actor);

    assert_eq!(tree.root.choices.len(), 1);
    assert_eq!(tree.root.choices[0].segment, "from/r3c2");
    assert!(tree.root.choices[0].next.is_some());
    assert_eq!(action_tree_hash(&tree), HashValue(7788678278305142813));
}

#[test]
fn compound_action_tree_legacy_and_v1_surfaces_are_pinned_in_parallel() {
    let state = initial_state("multi_jump", 7);
    let actor = Actor {
        seat_id: SeatId("seat-0".to_owned()),
    };
    let tree = legal_action_tree(&state, &actor);
    let v1_bytes = action_tree_v1_bytes(&tree);

    assert_eq!(action_tree_hash(&tree), HashValue(7788678278305142813));
    assert_eq!(
        HashValue::from_stable_bytes(action_tree_legacy_bytes(&tree).as_bytes()),
        action_tree_hash(&tree)
    );
    assert_eq!(v1_bytes, expected_action_tree_v1_bytes(&tree));
    assert_eq!(
        action_tree_v1_hash(&tree),
        HashValue(390_128_801_164_796_593)
    );
    assert_ne!(action_tree_hash(&tree), action_tree_v1_hash(&tree));
}

#[test]
fn characterization_compound_action_tree_nested_boundaries_are_ambiguous() {
    let mut nested_parent = ActionChoice::leaf("from/r3c2", "From", "From");
    nested_parent.next = Some(Box::new(ActionNode {
        choices: vec![ActionChoice::leaf("jump/r5c4", "Jump", "Jump")],
    }));
    let nested_tree = ActionTree::flat(FreshnessToken(0), vec![nested_parent]);

    let flat_delimiter_segment = ActionTree::flat(
        FreshnessToken(0),
        vec![ActionChoice::leaf(
            "from/r3c2|From|From|unavailable|||jump/r5c4",
            "Jump",
            "Jump",
        )],
    );

    assert_ne!(nested_tree, flat_delimiter_segment);
    assert_eq!(
        action_tree_hash(&nested_tree),
        action_tree_hash(&flat_delimiter_segment)
    );
    assert_ne!(
        action_tree_v1_bytes(&nested_tree),
        action_tree_v1_bytes(&flat_delimiter_segment)
    );
    assert_ne!(
        action_tree_v1_hash(&nested_tree),
        action_tree_v1_hash(&flat_delimiter_segment)
    );
}

#[test]
fn characterization_compound_action_tree_metadata_and_tags_are_unframed() {
    let mut compact_entries = ActionChoice::leaf("from/r3c2", "From", "From");
    compact_entries.metadata = vec![ActionMetadata {
        key: "captured".to_owned(),
        value: "seat_1-p01,landing=r5c4".to_owned(),
    }];
    compact_entries.tags = vec!["capture,forced".to_owned()];

    let mut split_entries = ActionChoice::leaf("from/r3c2", "From", "From");
    split_entries.metadata = vec![
        ActionMetadata {
            key: "captured".to_owned(),
            value: "seat_1-p01".to_owned(),
        },
        ActionMetadata {
            key: "landing".to_owned(),
            value: "r5c4".to_owned(),
        },
    ];
    split_entries.tags = vec!["capture".to_owned(), "forced".to_owned()];

    let compact_tree = ActionTree::flat(FreshnessToken(0), vec![compact_entries]);
    let split_tree = ActionTree::flat(FreshnessToken(0), vec![split_entries]);

    assert_ne!(compact_tree, split_tree);
    assert_eq!(
        action_tree_hash(&compact_tree),
        action_tree_hash(&split_tree)
    );
}

#[test]
fn characterization_legacy_trace_without_profile_metadata_is_still_accepted() {
    let fixture = include_str!("golden_traces/multi-jump.trace.json");

    assert!(!fixture.contains("\"profile_id\""));
    assert!(!fixture.contains("\"profile_version\""));
    assert!(!fixture.contains("\"canonical_byte_authority\""));

    let parsed = parse_trace_schema_v1_fixture(fixture);
    assert_eq!(parsed.id, "draughts-lite-multi-jump");
    assert_eq!(parsed.kind, "commands");
    assert_eq!(
        parsed.commands[0].action_path,
        ["from/r3c2", "jump/r5c4", "jump/r7c6"]
    );
}

#[test]
fn replay_command_v1_driver_replays_shortest_quiet_fixture() {
    let fixture_json = include_str!("golden_traces/shortest-quiet.trace.json");
    assert!(!fixture_json.contains("\"profile_id\""));
    assert!(!fixture_json.contains("\"profile_version\""));
    assert!(!fixture_json.contains("\"canonical_byte_authority\""));
    let fixture = parse_trace_schema_v1_fixture(fixture_json);
    let driver = ReplayCommandV1Driver::new("replay-check");
    let profile = replay_command_profile_artifact(&fixture);

    driver
        .validate_with(&profile, |_| {
            assert_fixture(parse_trace_schema_v1_fixture(fixture_json))
        })
        .expect("replay-command-v1 driver accepts shortest-quiet profile");
}

fn replay_command_profile_artifact(fixture: &TraceFixture) -> ProfileArtifact<'_> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: REPLAY_COMMAND_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("internal-dev"),
            validator_owner: "replay-check",
            canonical_byte_authority: "draughts_lite::replay_support",
            migration_update_note: Some(&fixture.migration_update_note),
        },
        fields: REPLAY_COMMAND_PROFILE_FIELDS,
        canonical_byte_claim: true,
    }
}

fn parse_trace_schema_v1_fixture(input: &str) -> TraceFixture {
    assert_eq!(number_field(input, "schema_version"), 1);
    assert_eq!(string_field(input, "variant"), "draughts_lite_standard");
    assert!(input.contains("\"not_applicable\""));
    assert!(input.contains("\"hidden_information\""));
    assert!(input.contains("\"stochastic_game_events\""));

    let expected_diagnostic_code = input.find("\"expected_diagnostics\":").map(|start| {
        let diagnostics = &input[start..];
        string_field(diagnostics, "code")
    });

    TraceFixture {
        id: string_field(input, "trace_id"),
        kind: string_field(input, "fixture_kind"),
        purpose: string_field(input, "purpose"),
        note: string_field(input, "note"),
        migration_update_note: string_field(input, "migration_update_note"),
        game_id: string_field(input, "game_id"),
        rules_version: string_field(input, "rules_version"),
        seed: number_field(input, "seed"),
        commands: commands(input),
        expected_state_hash: final_hash(input, "expected_state_hashes"),
        expected_effect_hash: final_hash(input, "expected_effect_hashes"),
        expected_action_tree_hash: final_hash(input, "expected_action_tree_hashes"),
        expected_view_hash: public_view_hash(input),
        expected_replay_hash: final_hash(input, "expected_replay_hashes"),
        expected_diagnostic_code,
        expected_diagnostic_hash: optional_diagnostic_hash(input),
        terminal: bool_field(&object_body(input, "expected_terminal_state"), "terminal"),
        winner: nullable_string_field(&object_body(input, "expected_terminal_state"), "winner"),
    }
}

fn assert_fixture(fixture: TraceFixture) {
    assert!(!fixture.note.is_empty(), "{} has a trace note", fixture.id);
    assert!(
        !fixture.migration_update_note.is_empty(),
        "{} has a migration/update note",
        fixture.id
    );
    assert_eq!(fixture.game_id, "draughts_lite", "{} game id", fixture.id);
    assert_eq!(
        fixture.rules_version, "draughts_lite-rules-v1",
        "{} rules version",
        fixture.id
    );
    assert!(!fixture.note.contains("debug"));
    assert!(!fixture.note.contains("hidden"));

    if fixture.kind == "bot" {
        assert_bot_fixture_command(&fixture);
    }
    if fixture.kind == "diagnostic" {
        assert_diagnostic_fixture(&fixture);
    }

    let applied = fixture
        .commands
        .iter()
        .filter(|command| command.expect == "applied")
        .map(|command| command.action_path.clone())
        .collect::<Vec<_>>();
    let mut state = initial_state(&fixture.purpose, fixture.seed);
    let initial_snapshot = DraughtsLiteSnapshot::from_state(&state).stable_summary();
    let hashes = replay_from_state(fixture.seed, initial_snapshot, &applied, &mut state);

    assert_eq!(
        hashes.state_hash,
        HashValue(fixture.expected_state_hash),
        "{} state hash",
        fixture.id
    );
    assert_eq!(
        hashes.effect_hash,
        HashValue(fixture.expected_effect_hash),
        "{} effect hash",
        fixture.id
    );
    assert_eq!(
        hashes.action_tree_hash,
        HashValue(fixture.expected_action_tree_hash),
        "{} action-tree hash",
        fixture.id
    );
    assert_eq!(
        hashes.view_hash,
        HashValue(fixture.expected_view_hash),
        "{} view hash",
        fixture.id
    );
    assert_eq!(
        hashes.replay_hash,
        HashValue(fixture.expected_replay_hash),
        "{} replay hash",
        fixture.id
    );
    assert_eq!(hashes.terminal, fixture.terminal, "{} terminal", fixture.id);
    assert_eq!(winner(&hashes.outcome), fixture.winner.as_deref());

    if fixture.id == "draughts-lite-multi-jump" {
        assert_eq!(
            fixture.commands[0].action_path,
            ["from/r3c2", "jump/r5c4", "jump/r7c6"]
        );
    }
}

fn assert_bot_fixture_command(fixture: &TraceFixture) {
    let state = setup_match(
        Seed(fixture.seed),
        &default_seats(),
        &SetupOptions::default(),
    )
    .expect("bot fixture setup succeeds");
    let decision = draughts_lite::DraughtsLiteLevel1Bot::new(Seed(fixture.seed))
        .select_decision(&state, state.active_seat)
        .expect("bot fixture decision succeeds");
    let expected = fixture
        .commands
        .iter()
        .find(|command| command.expect == "applied")
        .expect("bot fixture has applied command");
    assert_eq!(
        decision.action_path.segments, expected.action_path,
        "{} bot command",
        fixture.id
    );
}

fn assert_diagnostic_fixture(fixture: &TraceFixture) {
    let mut state = initial_state(&fixture.purpose, fixture.seed);
    let mut diagnostics = Vec::new();

    for command in &fixture.commands {
        let envelope = command_envelope(command);
        match command.expect.as_str() {
            "applied" => {
                let action = validate_command(&state, &envelope).expect("prelude validates");
                apply_action(&mut state, action);
            }
            "diagnostic" => {
                let diagnostic =
                    validate_command(&state, &envelope).expect_err("diagnostic command rejects");
                let expected_code = command
                    .expected_diagnostic_code
                    .as_deref()
                    .expect("command diagnostic code");
                assert_eq!(
                    diagnostic.code, expected_code,
                    "{} command code",
                    fixture.id
                );
                diagnostics.push(diagnostic);
            }
            other => panic!("unexpected command expectation {other}"),
        }
    }

    let expected_code = fixture
        .expected_diagnostic_code
        .as_deref()
        .expect("diagnostic fixture has expected diagnostic code");
    assert_eq!(
        diagnostics.last().unwrap().code,
        expected_code,
        "{} diagnostic code",
        fixture.id
    );
    assert_eq!(
        diagnostic_hash(&diagnostics),
        HashValue(fixture.expected_diagnostic_hash.expect("diagnostic hash")),
        "{} diagnostic hash",
        fixture.id
    );
}

fn command_envelope(command: &TraceCommand) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: seat_id(&command.actor_seat),
        },
        action_path: ActionPath {
            segments: command.action_path.clone(),
        },
        freshness_token: FreshnessToken(command.freshness_token),
        rules_version: RulesVersion(1),
    }
}

fn winner(outcome: &Option<TerminalOutcome>) -> Option<&'static str> {
    match outcome {
        Some(TerminalOutcome::Win { seat }) => Some(seat.as_str()),
        None => None,
    }
}

fn seat_id(seat: &str) -> SeatId {
    match seat {
        "seat_0" | "seat-0" => SeatId("seat-0".to_owned()),
        "seat_1" | "seat-1" => SeatId("seat-1".to_owned()),
        other => panic!("unknown fixture seat {other}"),
    }
}

fn initial_state(purpose: &str, seed: u64) -> DraughtsLiteState {
    match purpose {
        "mandatory_capture_suppresses_quiet"
        | "quiet_while_capture_diagnostic"
        | "single_capture" => empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 2, 8, 7),
            ],
        ),
        "multi_jump" | "illegal_continuation_diagnostic" => empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 2, 6, 5),
                man(DraughtsLiteSeat::Seat1, 3, 8, 7),
            ],
        ),
        "forced_continuation_branch" => empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 2, 6, 3),
                man(DraughtsLiteSeat::Seat1, 3, 6, 5),
                man(DraughtsLiteSeat::Seat1, 4, 8, 7),
            ],
        ),
        "promotion_quiet" => empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 7, 2),
                man(DraughtsLiteSeat::Seat1, 1, 6, 7),
            ],
        ),
        "promotion_during_capture_stop" | "path_after_promotion_stop_diagnostic" => empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 6, 3),
                man(DraughtsLiteSeat::Seat1, 1, 7, 4),
                man(DraughtsLiteSeat::Seat1, 2, 7, 6),
            ],
        ),
        "terminal_no_pieces" => empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
            ],
        ),
        "terminal_no_legal_moves" => empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 1, 2),
                man(DraughtsLiteSeat::Seat0, 2, 3, 4),
                man(DraughtsLiteSeat::Seat1, 1, 2, 1),
            ],
        ),
        _ => setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap(),
    }
}

fn default_seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn coord(row: u8, col: u8) -> Coord {
    Coord::checked(row, col).unwrap()
}

fn piece_id(owner: DraughtsLiteSeat, ordinal: u8) -> PieceId {
    PieceId::new(owner, ordinal).unwrap()
}

fn man(owner: DraughtsLiteSeat, ordinal: u8, row: u8, col: u8) -> Piece {
    Piece {
        id: piece_id(owner, ordinal),
        owner,
        kind: PieceKind::Man,
        cell: coord(row, col),
    }
}

fn empty_state(active_seat: DraughtsLiteSeat, mut pieces: Vec<Piece>) -> DraughtsLiteState {
    let board = draughts_lite::ids::board_dimensions();
    pieces.sort_by_key(|piece| piece.id);
    let mut cells = DraughtsLiteState::empty_cells();
    for piece in &pieces {
        cells[piece.cell.row_col_index(board).unwrap()] = CellOccupancy::Occupied(piece.id);
    }

    DraughtsLiteState {
        variant: Variant::draughts_lite_standard(),
        board,
        cells,
        pieces,
        active_seat,
        seats: [default_seats()[0].clone(), default_seats()[1].clone()],
        ply_count: 0,
        command_count: 0,
        terminal_outcome: None,
        terminal_reason: None,
        freshness_token: FreshnessToken(0),
    }
}

fn expected_action_tree_v1_bytes(tree: &ActionTree) -> Vec<u8> {
    let mut writer = StableBytesWriter::new(b"action_tree", 1).expect("writer");
    writer
        .write_u64_field(1, tree.freshness_token.0)
        .expect("freshness");
    writer
        .write_record_field(2, |record| encode_expected_node_v1(record, &tree.root))
        .expect("root");
    writer.into_bytes()
}

fn encode_expected_node_v1(
    record: &mut StableBytesRecordWriter,
    node: &ActionNode,
) -> Result<(), engine_core::StableBytesWriterError> {
    let choices = node
        .choices
        .iter()
        .map(expected_choice_v1_record)
        .collect::<Result<Vec<_>, _>>()?;
    record.write_sequence_field(1, choices)
}

fn expected_choice_v1_record(
    choice: &ActionChoice,
) -> Result<Vec<u8>, engine_core::StableBytesWriterError> {
    let mut record = StableBytesRecordWriter::new();
    record.write_string_field(1, &choice.segment)?;
    record.write_string_field(2, &choice.label)?;
    record.write_string_field(3, &choice.accessibility_label)?;
    record.write_sequence_field(4, expected_metadata_v1_records(&choice.metadata)?)?;
    record.write_sequence_field(5, choice.tags.iter().map(String::as_bytes))?;
    record.write_enum_field(6, expected_preview_discriminant(choice.preview))?;
    if let Some(next) = &choice.next {
        let mut child = StableBytesRecordWriter::new();
        encode_expected_node_v1(&mut child, next)?;
        record.write_some_field(7, StableBytesTypeTag::Record, &child.into_bytes())?;
    } else {
        record.write_none_field(7)?;
    }
    Ok(record.into_bytes())
}

fn expected_metadata_v1_records(
    metadata: &[ActionMetadata],
) -> Result<Vec<Vec<u8>>, engine_core::StableBytesWriterError> {
    metadata
        .iter()
        .map(|entry| {
            let mut record = StableBytesRecordWriter::new();
            record.write_string_field(1, &entry.key)?;
            record.write_string_field(2, &entry.value)?;
            Ok(record.into_bytes())
        })
        .collect()
}

const fn expected_preview_discriminant(preview: engine_core::ActionPreview) -> u32 {
    match preview {
        engine_core::ActionPreview::Unavailable => 0,
        engine_core::ActionPreview::Available => 1,
    }
}

fn string_field(input: &str, key: &str) -> String {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .unwrap_or_else(|| panic!("missing `{key}`"))
        + needle.len();
    parse_string_at(input, start).unwrap_or_else(|| panic!("field `{key}` must be a string"))
}

fn nullable_string_field(input: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .unwrap_or_else(|| panic!("missing `{key}`"))
        + needle.len();
    parse_string_at(input, start)
}

fn number_field(input: &str, key: &str) -> u64 {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .unwrap_or_else(|| panic!("missing `{key}`"))
        + needle.len();
    parse_number_at(input, start).unwrap_or_else(|| panic!("field `{key}` must be a number"))
}

fn bool_field(input: &str, key: &str) -> bool {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .unwrap_or_else(|| panic!("missing `{key}`"))
        + needle.len();
    parse_bool_at(input, start).unwrap_or_else(|| panic!("field `{key}` must be a bool"))
}

fn final_hash(input: &str, section: &str) -> u64 {
    let section_body = object_body(input, section);
    number_field(&section_body, "final")
}

fn public_view_hash(input: &str) -> u64 {
    let section_body = object_body(input, "expected_public_view_hashes");
    number_field(&section_body, "all")
}

fn optional_diagnostic_hash(input: &str) -> Option<u64> {
    input.find("\"expected_diagnostics\":").map(|start| {
        let tail = &input[start..];
        number_field(tail, "hash")
    })
}

fn commands(input: &str) -> Vec<TraceCommand> {
    let mut commands = Vec::new();
    let mut remaining = array_body(input, "commands");
    while let Some(offset) = remaining.find("\"index\":") {
        remaining = remaining[offset..].to_owned();
        let close = remaining.find('}').expect("command object closes");
        let body = &remaining[..=close];
        commands.push(TraceCommand {
            actor_seat: string_field(body, "actor_seat"),
            action_path: action_path(body),
            freshness_token: string_field(body, "freshness_token")
                .parse()
                .expect("freshness token is numeric"),
            expect: string_field(body, "expect"),
            expected_diagnostic_code: body
                .contains("\"expected_diagnostic_code\":")
                .then(|| string_field(body, "expected_diagnostic_code")),
        });
        remaining = remaining[close + 1..].to_owned();
    }
    commands
}

fn action_path(input: &str) -> Vec<String> {
    array_body_from(input, "action_path")
        .split(',')
        .filter_map(|value| parse_string_at(value, 0))
        .collect()
}

fn object_body(input: &str, key: &str) -> String {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .unwrap_or_else(|| panic!("missing `{key}`"))
        + needle.len();
    let open = input[start..]
        .find('{')
        .unwrap_or_else(|| panic!("field `{key}` must be an object"))
        + start;
    let mut depth = 0_u32;
    for (offset, ch) in input[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return input[open..=open + offset].to_owned();
                }
            }
            _ => {}
        }
    }
    panic!("object `{key}` must close");
}

fn array_body(input: &str, key: &str) -> String {
    array_body_from(input, key)
}

fn array_body_from(input: &str, key: &str) -> String {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .unwrap_or_else(|| panic!("missing `{key}`"))
        + needle.len();
    let open = input[start..]
        .find('[')
        .unwrap_or_else(|| panic!("field `{key}` must be an array"))
        + start;
    let mut depth = 0_u32;
    for (offset, ch) in input[open..].char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return input[open + 1..open + offset].to_owned();
                }
            }
            _ => {}
        }
    }
    panic!("array `{key}` must close");
}

fn parse_string_at(input: &str, start: usize) -> Option<String> {
    let tail = input[start..].trim_start();
    let tail = tail.strip_prefix('"')?;
    let end = tail.find('"')?;
    Some(tail[..end].to_owned())
}

fn parse_number_at(input: &str, start: usize) -> Option<u64> {
    let tail = input[start..].trim_start();
    let digits = tail
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

fn parse_bool_at(input: &str, start: usize) -> Option<bool> {
    let tail = input[start..].trim_start();
    if tail.starts_with("true") {
        Some(true)
    } else if tail.starts_with("false") {
        Some(false)
    } else {
        None
    }
}
