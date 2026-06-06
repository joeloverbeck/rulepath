use column_four::replay_support::{replay_commands, ColumnFourReplayJson};
use column_four::{
    apply_action, setup_match, validate_command, ColumnFourSeat, ColumnId, RowId, SetupOptions,
    TerminalOutcome, WinningLine,
};
use engine_core::{
    ActionPath, Actor, CommandEnvelope, FreshnessToken, HashValue, RulesVersion, SeatId, Seed,
    StableSerialize,
};

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

fn cell(row: RowId, column: ColumnId) -> column_four::CellId {
    column_four::CellId::new(row, column)
}

#[test]
fn replay_hashes_are_identical_for_same_input_stream() {
    let commands = vec![
        "drop/c1".to_owned(),
        "drop/c2".to_owned(),
        "drop/c1".to_owned(),
        "drop/c2".to_owned(),
        "drop/c1".to_owned(),
        "drop/c2".to_owned(),
        "drop/c1".to_owned(),
    ];

    let left = replay_commands(99, &commands);
    let right = replay_commands(99, &commands);

    assert_eq!(left, right);
    assert_eq!(
        left.outcome,
        Some(TerminalOutcome::Win {
            seat: ColumnFourSeat::Seat0,
            line: WinningLine {
                cells: [
                    cell(RowId::R1, ColumnId::C1),
                    cell(RowId::R2, ColumnId::C1),
                    cell(RowId::R3, ColumnId::C1),
                    cell(RowId::R4, ColumnId::C1),
                ]
            }
        })
    );
    assert_eq!(
        left.projections.last().unwrap().public_view_hash,
        left.view_hash
    );
    assert!(left
        .projections
        .last()
        .unwrap()
        .effects
        .iter()
        .any(|effect| effect.starts_with("WinDetected:seat_0")));
}

#[test]
fn replay_json_stable_serialization_rejects_unknown_fields() {
    let replay = ColumnFourReplayJson {
        schema_version: 1,
        game_id: "column_four".to_owned(),
        rules_version: "column_four-rules-v1".to_owned(),
        variant: "column_four_standard".to_owned(),
        seed: 3,
        initial_snapshot: "snapshot".to_owned(),
        command_segments: vec!["drop/c4".to_owned(), "drop/c3".to_owned()],
    };
    let json = replay.to_json();

    assert_eq!(json.as_bytes(), replay.stable_bytes());
    assert_eq!(ColumnFourReplayJson::from_json(&json).unwrap(), replay);
    assert!(ColumnFourReplayJson::from_json("{\"debug\":\"nope\"}").is_err());
}

#[test]
fn golden_traces_match_expected_replay_hashes_and_diagnostics() {
    for fixture in [
        include_str!("golden_traces/shortest-normal-win.trace.json"),
        include_str!("golden_traces/vertical-win.trace.json"),
        include_str!("golden_traces/horizontal-win.trace.json"),
        include_str!("golden_traces/diagonal-win.trace.json"),
        include_str!("golden_traces/draw.trace.json"),
        include_str!("golden_traces/stale-diagnostic.trace.json"),
        include_str!("golden_traces/invalid-column-diagnostic.trace.json"),
        include_str!("golden_traces/full-column-diagnostic.trace.json"),
        include_str!("golden_traces/bot-action.trace.json"),
        include_str!("golden_traces/terminal-replay.trace.json"),
        include_str!("golden_traces/wasm-exported.trace.json"),
    ] {
        assert_fixture(parse_trace_schema_v1_fixture(fixture));
    }
}

fn parse_trace_schema_v1_fixture(input: &str) -> TraceFixture {
    assert_eq!(number_field(input, "schema_version"), 1);
    assert_eq!(string_field(input, "variant"), "column_four_standard");
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
    assert_eq!(fixture.game_id, "column_four", "{} game id", fixture.id);
    assert_eq!(
        fixture.rules_version, "column_four-rules-v1",
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
    let hashes = replay_commands(fixture.seed, &applied);

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

    if fixture.id == "column-four-draw" {
        assert_eq!(applied.len(), 42, "draw trace fills every board cell");
        assert_eq!(hashes.outcome, Some(TerminalOutcome::Draw));
        let final_projection = hashes
            .projections
            .last()
            .expect("draw trace has final projection");
        assert_eq!(
            final_projection
                .board
                .iter()
                .filter(|cell| !cell.ends_with(":empty"))
                .count(),
            42,
            "draw public projection has 42 occupied cells"
        );
        assert!(!final_projection
            .effects
            .iter()
            .any(|effect| effect.starts_with("WinDetected:")));
    }
}

fn assert_bot_fixture_command(fixture: &TraceFixture) {
    let state = setup_match(
        Seed(fixture.seed),
        &[SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("bot fixture setup succeeds");
    let decision = column_four::ColumnFourLevel2Bot::new(Seed(fixture.seed))
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
    let mut state = setup_match(
        Seed(fixture.seed),
        &[SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())],
        &SetupOptions::default(),
    )
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
        Some(TerminalOutcome::Win { seat, .. }) => Some(seat.as_str()),
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
