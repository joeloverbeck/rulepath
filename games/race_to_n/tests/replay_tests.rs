use engine_core::HashValue;
use game_test_support::profiles::{
    ProfileArtifact, ProfileMetadata, ReplayCommandV1Driver, PROFILE_VERSION_V1, REPLAY_COMMAND_V1,
};
use race_to_n::replay_support::{replay_bot_action, replay_commands, replay_invalid};

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
    commands: Vec<String>,
    bot_seed: Option<u64>,
    invalid_command: Option<String>,
    stale_command: Option<String>,
    expected_state_hash: u64,
    expected_effect_hash: u64,
    expected_action_tree_hash: u64,
    expected_view_hash: u64,
    expected_diagnostic_hash: Option<u64>,
}

fn parse_trace_schema_v1_fixture(input: &str) -> TraceFixture {
    assert_eq!(number_field(input, "schema_version"), 1);
    assert_eq!(string_field(input, "variant"), "race_to_21");
    assert!(input.contains("\"not_applicable\""));
    assert!(input.contains("\"hidden_information\""));
    assert!(input.contains("\"stochastic_game_events\""));

    let kind = string_field(input, "fixture_kind");
    let commands = action_paths(input);
    TraceFixture {
        id: string_field(input, "trace_id"),
        kind,
        note: string_field(input, "note"),
        migration_update_note: string_field(input, "migration_update_note"),
        game_id: string_field(input, "game_id"),
        rules_version: string_field(input, "rules_version"),
        seed: number_field(input, "seed"),
        bot_seed: optional_number_field(input, "bot_seed"),
        invalid_command: command_with_expect(input, "invalid_action"),
        stale_command: command_with_expect(input, "stale_action"),
        expected_state_hash: final_hash(input, "expected_state_hashes"),
        expected_effect_hash: final_hash(input, "expected_effect_hashes"),
        expected_action_tree_hash: final_hash(input, "expected_action_tree_hashes"),
        expected_view_hash: public_view_hash(input),
        expected_diagnostic_hash: optional_diagnostic_hash(input),
        commands,
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

fn optional_number_field(input: &str, key: &str) -> Option<u64> {
    let needle = format!("\"{key}\":");
    input.find(&needle).map(|start| {
        parse_number_at(input, start + needle.len())
            .unwrap_or_else(|| panic!("field `{key}` must be a number"))
    })
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

fn command_with_expect(input: &str, expected_code: &str) -> Option<String> {
    input
        .find(&format!(
            "\"expected_diagnostic_code\": \"{expected_code}\""
        ))
        .map(|code_offset| {
            let before = &input[..code_offset];
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

fn replay_command_profile_artifact(fixture: &TraceFixture) -> ProfileArtifact<'_> {
    ProfileArtifact {
        metadata: ProfileMetadata {
            profile_id: REPLAY_COMMAND_V1,
            profile_version: PROFILE_VERSION_V1,
            visibility_class: Some("internal-dev"),
            validator_owner: "replay-check",
            canonical_byte_authority: "race_to_n::replay_support",
            migration_update_note: Some(&fixture.migration_update_note),
        },
        fields: REPLAY_COMMAND_PROFILE_FIELDS,
        canonical_byte_claim: true,
    }
}

fn assert_fixture(fixture: &TraceFixture) {
    assert!(!fixture.note.is_empty(), "{} has a trace note", fixture.id);
    assert!(
        !fixture.migration_update_note.is_empty(),
        "{} has a migration/update note",
        fixture.id
    );
    assert_eq!(fixture.game_id, "race_to_n", "{} game id", fixture.id);
    assert_eq!(
        fixture.rules_version, "race_to_n-rules-v1",
        "{} rules version",
        fixture.id
    );
    let hashes = match fixture.kind.as_str() {
        "commands" => replay_commands(fixture.seed, &fixture.commands),
        "bot" => replay_bot_action(
            fixture.seed,
            fixture.bot_seed.expect("bot fixture has bot_seed"),
        ),
        "invalid" => replay_invalid(
            fixture.seed,
            fixture
                .invalid_command
                .as_deref()
                .expect("invalid fixture has invalid command"),
            fixture
                .stale_command
                .as_deref()
                .expect("invalid fixture has stale command"),
        ),
        other => panic!("unknown trace kind {other}"),
    };
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
        hashes.diagnostic_hash,
        fixture.expected_diagnostic_hash.map(HashValue),
        "{} diagnostic hash",
        fixture.id
    );
}

#[test]
fn replay_command_v1_driver_replays_shortest_normal_fixture() {
    let fixture_json = include_str!("golden_traces/shortest-normal.trace.json");
    assert!(!fixture_json.contains("\"profile_id\""));
    assert!(!fixture_json.contains("\"profile_version\""));
    let fixture = parse_trace_schema_v1_fixture(fixture_json);
    let driver = ReplayCommandV1Driver::new("replay-check");
    let profile = replay_command_profile_artifact(&fixture);

    driver
        .validate_with(&profile, |_| assert_fixture(&fixture))
        .expect("replay-command-v1 driver accepts shortest-normal profile");
}

#[test]
fn replay_reproduces_hashes_for_same_inputs() {
    let commands = vec!["add-3".to_owned(), "add-2".to_owned(), "add-1".to_owned()];
    let left = replay_commands(99, &commands);
    let right = replay_commands(99, &commands);

    assert_eq!(left, right);
}

#[test]
fn golden_traces_match_expected_hashes() {
    for fixture in [
        include_str!("golden_traces/shortest-normal.trace.json"),
        include_str!("golden_traces/terminal.trace.json"),
        include_str!("golden_traces/bot-action.trace.json"),
        include_str!("golden_traces/invalid-stale-diagnostic.trace.json"),
    ] {
        assert_fixture(&parse_trace_schema_v1_fixture(fixture));
    }
}

#[test]
fn trace_set_records_not_applicable_hidden_and_stochastic_rationale() {
    let note = include_str!("golden_traces/not-applicable.trace.json");

    assert!(note.contains("\"fixture_kind\": \"not_applicable\""));
    assert!(note.contains("\"hidden_information\""));
    assert!(note.contains("\"stochastic_game_events\""));
    assert!(note.contains("perfect-information"));
}
