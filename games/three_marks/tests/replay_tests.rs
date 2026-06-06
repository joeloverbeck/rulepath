use engine_core::HashValue;
use three_marks::{
    replay_support::{
        replay_bot_action, replay_commands, replay_diagnostic, replay_stale, ReplayHashes,
    },
    CellId, TerminalOutcome, ThreeMarksSeat, WinningLine,
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
    commands: Vec<String>,
    diagnostic_command: Option<String>,
    expected_state_hash: u64,
    expected_effect_hash: u64,
    expected_action_tree_hash: u64,
    expected_view_hash: u64,
    expected_replay_hash: u64,
    expected_diagnostic_hash: Option<u64>,
}

fn parse_trace_schema_v1_fixture(input: &str) -> TraceFixture {
    assert_eq!(number_field(input, "schema_version"), 1);
    assert_eq!(string_field(input, "variant"), "three_marks_standard");
    assert!(input.contains("\"not_applicable\""));
    assert!(input.contains("\"hidden_information\""));
    assert!(input.contains("\"stochastic_game_events\""));

    TraceFixture {
        id: string_field(input, "trace_id"),
        kind: string_field(input, "fixture_kind"),
        note: string_field(input, "note"),
        migration_update_note: string_field(input, "migration_update_note"),
        game_id: string_field(input, "game_id"),
        rules_version: string_field(input, "rules_version"),
        seed: number_field(input, "seed"),
        commands: action_paths(input),
        diagnostic_command: command_with_diagnostic(input),
        expected_state_hash: final_hash(input, "expected_state_hashes"),
        expected_effect_hash: final_hash(input, "expected_effect_hashes"),
        expected_action_tree_hash: final_hash(input, "expected_action_tree_hashes"),
        expected_view_hash: public_view_hash(input),
        expected_replay_hash: final_hash(input, "expected_replay_hashes"),
        expected_diagnostic_hash: optional_diagnostic_hash(input),
    }
}

fn assert_fixture(fixture: TraceFixture) {
    assert!(!fixture.note.is_empty(), "{} has a trace note", fixture.id);
    assert!(
        !fixture.migration_update_note.is_empty(),
        "{} has a migration/update note",
        fixture.id
    );
    assert_eq!(fixture.game_id, "three_marks", "{} game id", fixture.id);
    assert_eq!(
        fixture.rules_version, "three_marks-rules-v1",
        "{} rules version",
        fixture.id
    );
    let hashes = match fixture.kind.as_str() {
        "commands" | "terminal" => replay_commands(fixture.seed, &fixture.commands),
        "bot" => replay_bot_action(fixture.seed),
        "invalid" | "diagnostic" => {
            if fixture.id.contains("stale") {
                replay_stale(
                    fixture.seed,
                    fixture
                        .diagnostic_command
                        .as_deref()
                        .expect("stale fixture has diagnostic command"),
                )
            } else {
                replay_diagnostic(
                    fixture.seed,
                    &fixture.commands[..fixture.commands.len().saturating_sub(1)],
                    fixture
                        .diagnostic_command
                        .as_deref()
                        .expect("diagnostic fixture has diagnostic command"),
                )
            }
        }
        "not_applicable" => return,
        other => panic!("unknown trace kind {other}"),
    };
    assert_hashes(&fixture, hashes);
}

fn assert_hashes(fixture: &TraceFixture, hashes: ReplayHashes) {
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
    assert_eq!(
        hashes.diagnostic_hash,
        fixture.expected_diagnostic_hash.map(HashValue),
        "{} diagnostic hash",
        fixture.id
    );
}

#[test]
fn replay_reproduces_hashes_outcome_and_board_projection_for_same_inputs() {
    let commands = vec![
        "place/r1c1".to_owned(),
        "place/r2c1".to_owned(),
        "place/r1c2".to_owned(),
        "place/r2c2".to_owned(),
        "place/r1c3".to_owned(),
    ];
    let left = replay_commands(1, &commands);
    let right = replay_commands(1, &commands);

    assert_eq!(left, right);
    assert!(left.terminal);
    assert_eq!(
        left.outcome,
        Some(TerminalOutcome::Win {
            seat: ThreeMarksSeat::Seat0,
            line: WinningLine {
                cells: [CellId::R1C1, CellId::R1C2, CellId::R1C3]
            }
        })
    );
    assert_eq!(left.projections.len(), 5);
    assert!(left.projections[4]
        .board
        .contains(&"r1c3:seat_0".to_owned()));
    assert!(left.projections[4]
        .effects
        .iter()
        .any(|effect| effect.starts_with("LineCompleted:seat_0")));
}

#[test]
fn golden_traces_match_expected_hashes() {
    for fixture in [
        include_str!("golden_traces/shortest-normal.trace.json"),
        include_str!("golden_traces/draw.trace.json"),
        include_str!("golden_traces/terminal.trace.json"),
        include_str!("golden_traces/occupied-diagnostic.trace.json"),
        include_str!("golden_traces/stale-diagnostic.trace.json"),
        include_str!("golden_traces/bot-action.trace.json"),
        include_str!("golden_traces/not-applicable.trace.json"),
    ] {
        assert_fixture(parse_trace_schema_v1_fixture(fixture));
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

fn number_field(input: &str, key: &str) -> u64 {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .unwrap_or_else(|| panic!("missing `{key}`"))
        + needle.len();
    parse_number_at(input, start).unwrap_or_else(|| panic!("field `{key}` must be a number"))
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

fn action_paths(input: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let mut remaining = input;
    while let Some(offset) = remaining.find("\"action_path\":") {
        remaining = &remaining[offset + "\"action_path\":".len()..];
        let open = remaining.find('[').expect("action_path must be an array");
        let close = remaining[open..]
            .find(']')
            .expect("action_path array must close")
            + open;
        commands.push(parse_first_array_string(&remaining[open + 1..close]));
        remaining = &remaining[close + 1..];
    }
    commands
}

fn command_with_diagnostic(input: &str) -> Option<String> {
    input.find("\"expect\": \"diagnostic\"").map(|offset| {
        let before = &input[..offset];
        let action_offset = before
            .rfind("\"action_path\":")
            .expect("diagnostic command has action_path");
        action_paths(&before[action_offset..])
            .pop()
            .expect("diagnostic command action path parses")
    })
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

fn parse_first_array_string(input: &str) -> String {
    parse_string_at(input, 0).expect("array must contain a string path segment")
}
