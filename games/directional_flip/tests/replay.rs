use directional_flip::replay_support::{
    action_tree_hash, action_tree_v1_bytes, action_tree_v1_hash,
};
use directional_flip::{
    apply_action, legal_action_tree, replay_commands, replay_from_state, setup_match,
    validate_command, CellId, CellOccupancy, ColumnId, DirectionalFlipLevel2Bot,
    DirectionalFlipReplayJson, DirectionalFlipSeat, DirectionalFlipSnapshot, DirectionalFlipState,
    Manifest, RowId, SetupOptions, TerminalOutcome, VariantCatalog,
};
use engine_core::{
    ActionChoice, ActionNode, ActionPath, ActionPreview, ActionTreeEncodingVersion, Actor,
    CommandEnvelope, FreshnessToken, HashValue, RulesVersion, SeatId, Seed, StableSerialize,
};
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
    draw: bool,
    winner: Option<String>,
}

#[derive(Debug)]
struct TraceCommand {
    actor_seat: String,
    segment: String,
    freshness_token: u64,
    expect: String,
    expected_diagnostic_code: Option<String>,
}

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

#[test]
fn df_replay_001_replay_export_import_step_reset_hashes_are_deterministic() {
    let commands = first_legal_segments(12, 4);
    let left = replay_commands(12, &commands);
    let right = replay_commands(12, &commands);

    assert_eq!(left, right);
    assert_eq!(left.projections.len(), commands.len());
    assert_eq!(
        left.projections.last().unwrap().public_view_hash,
        left.view_hash
    );
    assert_ne!(left.replay_hash.0, 0);
}

#[test]
fn golden_traces_match_expected_replay_hashes_diagnostics_and_bot_choices() {
    let fixtures = [
        include_str!("golden_traces/opening-legal-move.trace.json"),
        include_str!("golden_traces/multi-direction-flip.trace.json"),
        include_str!("golden_traces/corner-capture.trace.json"),
        include_str!("golden_traces/forced-pass.trace.json"),
        include_str!("golden_traces/double-pass-terminal.trace.json"),
        include_str!("golden_traces/full-board-terminal.trace.json"),
        include_str!("golden_traces/draw.trace.json"),
        include_str!("golden_traces/invalid-occupied-cell.trace.json"),
        include_str!("golden_traces/invalid-non-flipping-placement.trace.json"),
        include_str!("golden_traces/stale-diagnostic.trace.json"),
        include_str!("golden_traces/non-active-seat-diagnostic.trace.json"),
        include_str!("golden_traces/bot-action.trace.json"),
        include_str!("golden_traces/wasm-exported.trace.json"),
        include_str!("golden_traces/preview-flip-set.trace.json"),
    ];

    assert_eq!(fixtures.len(), 14);
    for fixture in fixtures {
        assert_no_behavior_keys(fixture);
        assert_fixture(parse_trace_schema_v1_fixture(fixture));
    }
}

#[test]
fn replay_command_v1_driver_replays_opening_legal_move_fixture() {
    let fixture_json = include_str!("golden_traces/opening-legal-move.trace.json");
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
        .expect("replay-command-v1 driver accepts opening-legal-move profile");
}

fn replay_command_profile_artifact(fixture: &TraceFixture) -> ProfileArtifact<'_> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: REPLAY_COMMAND_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("internal-dev"),
            validator_owner: "replay-check",
            canonical_byte_authority: "directional_flip::replay_support",
            migration_update_note: Some(&fixture.migration_update_note),
        },
        fields: REPLAY_COMMAND_PROFILE_FIELDS,
        canonical_byte_claim: true,
    }
}

fn first_legal_segments(seed: u64, count: usize) -> Vec<String> {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let mut state = setup_match(Seed(seed), &seats, &SetupOptions::default()).unwrap();
    let mut commands = Vec::new();
    for _ in 0..count {
        let tree = legal_action_tree(
            &state,
            &Actor {
                seat_id: state.seats[state.active_seat.index()].clone(),
            },
        );
        let segment = tree.root.choices.first().unwrap().segment.clone();
        let command = CommandEnvelope {
            actor: Actor {
                seat_id: state.seats[state.active_seat.index()].clone(),
            },
            action_path: ActionPath {
                segments: vec![segment.clone()],
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        };
        let action = validate_command(&state, &command).unwrap();
        apply_action(&mut state, action);
        commands.push(segment);
    }
    commands
}

#[test]
fn df_ser_001_replay_json_rejects_unknown_fields_and_round_trips() {
    let replay = DirectionalFlipReplayJson {
        schema_version: 1,
        game_id: "directional_flip".to_owned(),
        rules_version: "directional_flip-rules-v1".to_owned(),
        variant: "directional_flip_standard".to_owned(),
        seed: 7,
        initial_snapshot: "snapshot".to_owned(),
        command_segments: vec!["place/r3c4".to_owned()],
    };

    let json = replay.to_json();
    assert_eq!(DirectionalFlipReplayJson::from_json(&json).unwrap(), replay);
    assert!(DirectionalFlipReplayJson::from_json("{\"debug\":\"nope\"}").is_err());
}

#[test]
fn action_tree_legacy_and_v1_surfaces_are_pinned_in_parallel() {
    let state = setup_match(Seed(9), &seats(), &SetupOptions::default()).unwrap();
    let actor = Actor {
        seat_id: SeatId("seat-0".to_owned()),
    };
    let tree = legal_action_tree(&state, &actor);
    let v1_bytes = action_tree_v1_bytes(&tree);

    assert_eq!(action_tree_hash(&tree), HashValue(17097613169116532881));
    assert_eq!(v1_bytes, tree.stable_bytes(ActionTreeEncodingVersion::V1));
    assert_eq!(
        action_tree_v1_hash(&tree),
        tree.stable_hash(ActionTreeEncodingVersion::V1)
    );
    assert_eq!(action_tree_v1_hash(&tree), HashValue(15334878763169959513));
    assert_eq!(v1_bytes.len(), 2092);
    assert!(v1_bytes.starts_with(b"RPSB"));
    assert!(v1_bytes
        .windows(b"action_tree".len())
        .any(|window| window == b"action_tree"));
    assert!(byte_offset(&v1_bytes, b"place/r3c4") < byte_offset(&v1_bytes, b"place/r4c3"));
    assert!(byte_offset(&v1_bytes, b"place/r4c3") < byte_offset(&v1_bytes, b"place/r5c6"));
    assert!(byte_offset(&v1_bytes, b"place/r5c6") < byte_offset(&v1_bytes, b"place/r6c5"));
    assert!(
        byte_offset(&v1_bytes, b"Place at r3c4")
            < byte_offset(&v1_bytes, b"Place at r3c4, flipping 1 disc")
    );
    assert!(
        byte_offset(&v1_bytes, b"action_kind") < byte_offset(&v1_bytes, b"cell_id")
            && byte_offset(&v1_bytes, b"cell_id") < byte_offset(&v1_bytes, b"row")
            && byte_offset(&v1_bytes, b"row") < byte_offset(&v1_bytes, b"column")
            && byte_offset(&v1_bytes, b"column") < byte_offset(&v1_bytes, b"preview_id")
            && byte_offset(&v1_bytes, b"preview_id")
                < byte_offset(&v1_bytes, b"ordered_flip_cells")
            && byte_offset(&v1_bytes, b"ordered_flip_cells")
                < byte_offset(&v1_bytes, b"direction_groups")
            && byte_offset(&v1_bytes, b"direction_groups") < byte_offset(&v1_bytes, b"explanation")
    );
    let flat_tag = byte_offset(&v1_bytes, b"flat");
    let placement_tag = byte_offset_after(&v1_bytes, b"placement", flat_tag);
    let cell_tag = byte_offset_after(&v1_bytes, b"cell", placement_tag);
    let preview_tag = byte_offset_after(&v1_bytes, b"preview", cell_tag);
    assert!(flat_tag < placement_tag && placement_tag < cell_tag && cell_tag < preview_tag);
    assert!(tree
        .root
        .choices
        .iter()
        .all(|choice| choice.preview == ActionPreview::Available && choice.next.is_none()));
    let mut freshness_changed = tree.clone();
    freshness_changed.freshness_token = FreshnessToken(tree.freshness_token.0 + 1);
    assert_ne!(
        action_tree_v1_hash(&tree),
        action_tree_v1_hash(&freshness_changed)
    );
    let mut preview_changed = tree.clone();
    preview_changed.root.choices[0].preview = ActionPreview::Unavailable;
    assert_ne!(
        action_tree_v1_hash(&tree),
        action_tree_v1_hash(&preview_changed)
    );
    let mut child_changed = tree.clone();
    child_changed.root.choices[0].next = Some(Box::new(ActionNode {
        choices: vec![ActionChoice::leaf(
            "confirm",
            "Confirm",
            "Confirm placement",
        )],
    }));
    assert_ne!(
        action_tree_v1_hash(&tree),
        action_tree_v1_hash(&child_changed)
    );
    assert_ne!(action_tree_hash(&tree), action_tree_v1_hash(&tree));
}

fn byte_offset(haystack: &[u8], needle: &[u8]) -> usize {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
        .expect("needle appears in v1 bytes")
}

fn byte_offset_after(haystack: &[u8], needle: &[u8], offset: usize) -> usize {
    offset
        + haystack[offset..]
            .windows(needle.len())
            .position(|window| window == needle)
            .expect("needle appears in v1 bytes after offset")
}

#[test]
fn df_ser_001_static_data_rejects_unknown_and_behavior_looking_fields() {
    assert!(Manifest::parse("game_id = \"directional_flip\"\nextra = \"nope\"\n").is_err());
    assert!(VariantCatalog::parse(
        "variant_id = \"directional_flip_standard\"\nif = \"place/r1c1\"\n"
    )
    .is_err());
}

#[test]
fn df_replay_001_snapshot_stable_serialization_is_repeatable() {
    let hashes = replay_commands(3, &["place/r3c4".to_owned()]);
    assert_eq!(hashes.state_hash, hashes.state_hash);

    let snapshot_text = DirectionalFlipSnapshot {
        schema_version: 1,
        rules_version: 1,
        rules_version_label: "directional_flip-rules-v1".to_owned(),
        variant: directional_flip::Variant::directional_flip_standard(),
        cells: directional_flip::DirectionalFlipState::empty_cells(),
        active_seat: directional_flip::DirectionalFlipSeat::Seat0,
        seats: [
            engine_core::SeatId("seat-0".to_owned()),
            engine_core::SeatId("seat-1".to_owned()),
        ],
        ply_count: 0,
        consecutive_forced_passes: 0,
        terminal_outcome: None,
        terminal_reason: None,
        freshness_token: engine_core::FreshnessToken(0),
    }
    .stable_summary();
    assert_eq!(
        DirectionalFlipReplayJson {
            schema_version: 1,
            game_id: "directional_flip".to_owned(),
            rules_version: "directional_flip-rules-v1".to_owned(),
            variant: "directional_flip_standard".to_owned(),
            seed: 3,
            initial_snapshot: snapshot_text,
            command_segments: vec!["place/r3c4".to_owned()],
        }
        .stable_hash(),
        DirectionalFlipReplayJson {
            schema_version: 1,
            game_id: "directional_flip".to_owned(),
            rules_version: "directional_flip-rules-v1".to_owned(),
            variant: "directional_flip_standard".to_owned(),
            seed: 3,
            initial_snapshot: DirectionalFlipSnapshot {
                schema_version: 1,
                rules_version: 1,
                rules_version_label: "directional_flip-rules-v1".to_owned(),
                variant: directional_flip::Variant::directional_flip_standard(),
                cells: directional_flip::DirectionalFlipState::empty_cells(),
                active_seat: directional_flip::DirectionalFlipSeat::Seat0,
                seats: [
                    engine_core::SeatId("seat-0".to_owned()),
                    engine_core::SeatId("seat-1".to_owned()),
                ],
                ply_count: 0,
                consecutive_forced_passes: 0,
                terminal_outcome: None,
                terminal_reason: None,
                freshness_token: engine_core::FreshnessToken(0),
            }
            .stable_summary(),
            command_segments: vec!["place/r3c4".to_owned()],
        }
        .stable_hash()
    );
}

fn parse_trace_schema_v1_fixture(input: &str) -> TraceFixture {
    assert_eq!(number_field(input, "schema_version"), 1);
    assert_eq!(string_field(input, "variant"), "directional_flip_standard");
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
        draw: bool_field(&object_body(input, "expected_terminal_state"), "draw"),
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
    assert_eq!(
        fixture.game_id, "directional_flip",
        "{} game id",
        fixture.id
    );
    assert_eq!(
        fixture.rules_version, "directional_flip-rules-v1",
        "{} rules version",
        fixture.id
    );

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
        .map(|command| command.segment.clone())
        .collect::<Vec<_>>();
    let hashes = hashes_for_fixture(&fixture.id, fixture.seed, &applied);

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
    assert_eq!(hashes.outcome == Some(TerminalOutcome::Draw), fixture.draw);
    assert_eq!(winner(&hashes.outcome), fixture.winner.as_deref());
}

fn hashes_for_fixture(id: &str, seed: u64, commands: &[String]) -> directional_flip::ReplayHashes {
    match id {
        "directional-flip-multi-direction-flip" => {
            replay_custom(seed, multi_direction_state(), commands)
        }
        "directional-flip-corner-capture" => replay_custom(seed, corner_state(), commands),
        "directional-flip-forced-pass"
        | "directional-flip-double-pass-terminal"
        | "directional-flip-draw" => replay_custom(seed, no_move_state(), commands),
        "directional-flip-full-board-terminal" => {
            replay_custom(seed, full_board_terminal_state(), commands)
        }
        _ => replay_commands(seed, commands),
    }
}

fn replay_custom(
    seed: u64,
    mut state: DirectionalFlipState,
    commands: &[String],
) -> directional_flip::ReplayHashes {
    let initial_snapshot = DirectionalFlipSnapshot::from_state(&state).stable_summary();
    replay_from_state(seed, initial_snapshot, commands, &mut state)
}

fn assert_bot_fixture_command(fixture: &TraceFixture) {
    let state = setup_match(Seed(fixture.seed), &seats(), &SetupOptions::default())
        .expect("bot fixture setup succeeds");
    let decision = DirectionalFlipLevel2Bot::new(Seed(fixture.seed))
        .select_decision(&state, state.active_seat)
        .expect("bot fixture decision succeeds");
    let expected = fixture
        .commands
        .iter()
        .find(|command| command.expect == "applied")
        .expect("bot fixture has applied command");
    assert_eq!(
        decision.action_path.segments,
        vec![expected.segment.clone()],
        "{} bot command",
        fixture.id
    );
}

fn assert_diagnostic_fixture(fixture: &TraceFixture) {
    let mut state = setup_match(Seed(fixture.seed), &seats(), &SetupOptions::default())
        .expect("diagnostic fixture setup succeeds");

    for command in fixture
        .commands
        .iter()
        .filter(|command| command.expect == "applied")
    {
        let envelope = command_envelope(command);
        let action = validate_command(&state, &envelope).expect("prelude command validates");
        apply_action(&mut state, action);
    }

    let diagnostic_command = fixture
        .commands
        .iter()
        .find(|command| command.expect == "diagnostic")
        .expect("diagnostic fixture has a diagnostic command");
    let envelope = command_envelope(diagnostic_command);
    let diagnostic = validate_command(&state, &envelope).expect_err("diagnostic command rejects");
    let expected_code = fixture
        .expected_diagnostic_code
        .as_deref()
        .expect("diagnostic fixture has expected diagnostic code");
    assert_eq!(
        diagnostic.code, expected_code,
        "{} diagnostic code",
        fixture.id
    );
    assert_eq!(
        diagnostic_command.expected_diagnostic_code.as_deref(),
        Some(expected_code),
        "{} command diagnostic code",
        fixture.id
    );
    assert_eq!(
        HashValue::from_stable_bytes(
            format!("{}:{}", diagnostic.code, diagnostic.message).as_bytes()
        ),
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
            segments: vec![command.segment.clone()],
        },
        freshness_token: FreshnessToken(command.freshness_token),
        rules_version: RulesVersion(1),
    }
}

fn winner(outcome: &Option<TerminalOutcome>) -> Option<&'static str> {
    match outcome {
        Some(TerminalOutcome::Win { seat }) => Some(seat.as_str()),
        _ => None,
    }
}

fn seat_id(seat: &str) -> SeatId {
    match seat {
        "seat_0" => SeatId("seat-0".to_owned()),
        "seat_1" => SeatId("seat-1".to_owned()),
        other => panic!("unknown fixture seat {other}"),
    }
}

fn assert_no_behavior_keys(input: &str) {
    for key in [
        "when",
        "if",
        "then",
        "else",
        "selector",
        "condition",
        "trigger",
        "script",
        "loop",
        "foreach",
        "priority_expression",
        "ai_condition",
        "effect_script",
        "rule",
        "requires",
        "valid_if",
        "on_play",
        "on_reveal",
    ] {
        assert!(
            !input.contains(&format!("\"{key}\"")),
            "forbidden key {key}"
        );
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
            segment: action_path(body),
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

fn action_path(input: &str) -> String {
    let start = input
        .find("\"action_path\":")
        .expect("command has action path")
        + "\"action_path\":".len();
    let open = input[start..].find('[').expect("action path opens") + start;
    parse_string_at(input, open + 1).expect("action path has one segment")
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
                    return input[open..=open + offset].to_owned();
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

fn c(row: RowId, column: ColumnId) -> CellId {
    CellId::new(row, column)
}

fn occupy(state: &mut DirectionalFlipState, cell: CellId, seat: DirectionalFlipSeat) {
    state.set_occupancy(cell, CellOccupancy::Occupied(seat));
}

fn empty(active: DirectionalFlipSeat) -> DirectionalFlipState {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.cells = DirectionalFlipState::empty_cells();
    state.active_seat = active;
    state.ply_count = 0;
    state.consecutive_forced_passes = 0;
    state.terminal_outcome = None;
    state.terminal_reason = None;
    state
}

fn no_move_state() -> DirectionalFlipState {
    let mut state = empty(DirectionalFlipSeat::Seat0);
    occupy(
        &mut state,
        c(RowId::R1, ColumnId::C1),
        DirectionalFlipSeat::Seat0,
    );
    occupy(
        &mut state,
        c(RowId::R8, ColumnId::C8),
        DirectionalFlipSeat::Seat1,
    );
    state
}

fn corner_state() -> DirectionalFlipState {
    let mut state = empty(DirectionalFlipSeat::Seat0);
    occupy(
        &mut state,
        c(RowId::R1, ColumnId::C2),
        DirectionalFlipSeat::Seat1,
    );
    occupy(
        &mut state,
        c(RowId::R1, ColumnId::C3),
        DirectionalFlipSeat::Seat0,
    );
    state
}

fn full_board_terminal_state() -> DirectionalFlipState {
    let mut state = empty(DirectionalFlipSeat::Seat0);
    for cell in CellId::ALL {
        occupy(&mut state, cell, DirectionalFlipSeat::Seat0);
    }
    state.set_occupancy(c(RowId::R1, ColumnId::C1), CellOccupancy::Empty);
    occupy(
        &mut state,
        c(RowId::R1, ColumnId::C2),
        DirectionalFlipSeat::Seat1,
    );
    state
}

fn multi_direction_state() -> DirectionalFlipState {
    let mut state = empty(DirectionalFlipSeat::Seat0);
    for cell in [
        c(RowId::R3, ColumnId::C4),
        c(RowId::R3, ColumnId::C5),
        c(RowId::R4, ColumnId::C5),
        c(RowId::R5, ColumnId::C5),
        c(RowId::R5, ColumnId::C4),
        c(RowId::R5, ColumnId::C3),
        c(RowId::R4, ColumnId::C3),
        c(RowId::R3, ColumnId::C3),
    ] {
        occupy(&mut state, cell, DirectionalFlipSeat::Seat1);
    }
    for cell in [
        c(RowId::R2, ColumnId::C4),
        c(RowId::R2, ColumnId::C6),
        c(RowId::R4, ColumnId::C6),
        c(RowId::R6, ColumnId::C6),
        c(RowId::R6, ColumnId::C4),
        c(RowId::R6, ColumnId::C2),
        c(RowId::R4, ColumnId::C2),
        c(RowId::R2, ColumnId::C2),
    ] {
        occupy(&mut state, cell, DirectionalFlipSeat::Seat0);
    }
    state
}
