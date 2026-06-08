use engine_core::{
    ActionPath, Actor, CommandEnvelope, FreshnessToken, HashValue, RulesVersion, SeatId, Seed,
    StableSerialize, Viewer,
};
use high_card_duel::{
    active_commit_seat, apply_action, effect_hash, export_public_observer_replay,
    generate_internal_full_trace, import_public_export, legal_action_tree, project_view,
    replay_internal_full_trace, setup_match, state_hash, validate_command,
    HighCardDuelInternalTrace, HighCardDuelRandomBot, HighCardDuelSeat, ReplayCommandPath,
    SetupOptions, TerminalOutcome, GAME_ID, RANDOM_POLICY_ID, RULES_VERSION_LABEL, VARIANT_ID,
};

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
    expected_private_view_hash: Option<u64>,
    expected_diagnostic_code: Option<String>,
    expected_diagnostic_hash: Option<u64>,
    terminal: bool,
    winner: Option<String>,
    draw: bool,
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
fn replaying_internal_full_trace_reproduces_revealed_sequence() {
    let trace = generate_internal_full_trace(9);

    let left = replay_internal_full_trace(&trace);
    let right = replay_internal_full_trace(&trace);

    assert_eq!(left.revealed_sequence, right.revealed_sequence);
    assert_eq!(left.state_hash, right.state_hash);
    assert_eq!(left.effect_hash, right.effect_hash);
    assert_eq!(left.terminal_outcome, right.terminal_outcome);
    assert!(trace
        .command_paths
        .iter()
        .flatten()
        .any(|segment| segment.contains("hcd:r")));
}

#[test]
fn replay_standard_fixture_metadata_is_present_and_public_safe() {
    let fixture = include_str!("../data/fixtures/high_card_duel_standard.fixture.json");

    assert_eq!(
        string_field(fixture, "fixture_id"),
        "high_card_duel_standard_gate8"
    );
    assert_eq!(string_field(fixture, "game_id"), GAME_ID);
    assert_eq!(string_field(fixture, "variant"), VARIANT_ID);
    assert_eq!(string_field(fixture, "rules_version"), RULES_VERSION_LABEL);
    assert_eq!(number_field(fixture, "trace_schema_version"), 1);
    assert!(fixture.contains("\"fixture_kinds\""));
    assert!(!fixture.contains("debug"));
}

#[test]
fn golden_traces_match_expected_replay_hashes_diagnostics_bot_and_no_leak_surfaces() {
    let fixtures = [
        include_str!("golden_traces/shortest-normal.trace.json"),
        include_str!("golden_traces/tie-round.trace.json"),
        include_str!("golden_traces/invalid-wrong-seat-diagnostic.trace.json"),
        include_str!("golden_traces/invalid-private-card-redacted.trace.json"),
        include_str!("golden_traces/stale-diagnostic.trace.json"),
        include_str!("golden_traces/bot-action.trace.json"),
        include_str!("golden_traces/hidden-info-public-observer.trace.json"),
        include_str!("golden_traces/seat-private-view.trace.json"),
        include_str!("golden_traces/public-replay-export-import.trace.json"),
        include_str!("golden_traces/terminal.trace.json"),
    ];

    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert_trace_fixture(parse_trace_schema_v1_fixture(fixture), fixture);
    }
}

fn parse_trace_schema_v1_fixture(input: &str) -> TraceFixture {
    assert_eq!(number_field(input, "schema_version"), 1);
    assert_eq!(string_field(input, "variant"), VARIANT_ID);
    assert!(input.contains("\"not_applicable\""));
    assert!(input.contains("\"hidden_information\""));
    assert!(input.contains("\"stochastic_game_events\""));

    let expected_diagnostic_code = input.find("\"expected_diagnostics\":").map(|start| {
        let diagnostics = &input[start..];
        string_field(diagnostics, "code")
    });

    let private_hashes = object_body(input, "expected_private_view_hashes");
    let expected_private_view_hash = private_hashes
        .contains("\"seat_0\":")
        .then(|| number_field(&private_hashes, "seat_0"));

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
        expected_private_view_hash,
        expected_diagnostic_code,
        expected_diagnostic_hash: optional_diagnostic_hash(input),
        terminal: bool_field(&object_body(input, "expected_terminal_state"), "terminal"),
        winner: nullable_string_field(&object_body(input, "expected_terminal_state"), "winner"),
        draw: bool_field(&object_body(input, "expected_terminal_state"), "draw"),
    }
}

fn assert_trace_fixture(fixture: TraceFixture, input: &str) {
    assert!(!fixture.note.is_empty(), "{} has a trace note", fixture.id);
    assert!(
        !fixture.migration_update_note.is_empty(),
        "{} has a migration/update note",
        fixture.id
    );
    assert_eq!(fixture.game_id, GAME_ID, "{} game id", fixture.id);
    assert_eq!(
        fixture.rules_version, RULES_VERSION_LABEL,
        "{} rules version",
        fixture.id
    );
    assert!(!fixture.note.contains("debug"));

    if fixture.kind == "bot" {
        assert_bot_fixture_command(&fixture);
    }
    if matches!(fixture.kind.as_str(), "invalid" | "diagnostic") {
        assert_diagnostic_fixture(&fixture);
    }
    if matches!(
        fixture.purpose.as_str(),
        "hidden_info_public_observer" | "public_replay_export_import"
    ) {
        assert!(
            !input.contains("hcd:r"),
            "{} trace is public-safe",
            fixture.id
        );
        assert_public_export_has_no_hidden_identity(fixture.seed);
    }

    let hashes = replay_applied_commands(&fixture);
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
        "{} public view hash",
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
    assert_eq!(
        matches!(hashes.outcome, Some(TerminalOutcome::Draw)),
        fixture.draw,
        "{} draw",
        fixture.id
    );

    if let Some(expected) = fixture.expected_private_view_hash {
        assert_eq!(
            hashes.private_view_hash,
            HashValue(expected),
            "{} private view hash",
            fixture.id
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
    let decision = HighCardDuelRandomBot::new(Seed(fixture.seed))
        .select_decision(&state, HighCardDuelSeat::Seat0)
        .expect("bot fixture decision succeeds");
    let expected = fixture
        .commands
        .iter()
        .find(|command| command.expect == "applied")
        .expect("bot fixture has applied command");

    assert_eq!(decision.policy_id, RANDOM_POLICY_ID);
    assert_eq!(
        decision.action_path.segments, expected.action_path,
        "{} bot command",
        fixture.id
    );
}

fn assert_diagnostic_fixture(fixture: &TraceFixture) {
    let mut state = setup_match(
        Seed(fixture.seed),
        &default_seats(),
        &SetupOptions::default(),
    )
    .expect("diagnostic fixture setup succeeds");
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

#[derive(Clone, Debug)]
struct ReplayHashes {
    state_hash: HashValue,
    effect_hash: HashValue,
    action_tree_hash: HashValue,
    view_hash: HashValue,
    private_view_hash: HashValue,
    replay_hash: HashValue,
    terminal: bool,
    outcome: Option<TerminalOutcome>,
}

fn replay_applied_commands(fixture: &TraceFixture) -> ReplayHashes {
    let applied_commands = fixture
        .commands
        .iter()
        .filter(|command| command.expect == "applied")
        .collect::<Vec<_>>();
    let mut state = setup_match(
        Seed(fixture.seed),
        &default_seats(),
        &SetupOptions::default(),
    )
    .expect("trace setup succeeds");
    let mut effects = Vec::new();

    for command in &applied_commands {
        let envelope = command_envelope(command);
        let action = validate_command(&state, &envelope).expect("trace command validates");
        effects.extend(apply_action(&mut state, action));
    }

    let command_paths = applied_commands
        .iter()
        .map(|command| command.action_path.clone())
        .collect::<Vec<ReplayCommandPath>>();
    let trace = HighCardDuelInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        seed: fixture.seed,
        command_paths,
    };

    ReplayHashes {
        state_hash: state_hash(&state),
        effect_hash: effect_hash(&effects),
        action_tree_hash: action_tree_hash(&state),
        view_hash: project_view(&state, &Viewer { seat_id: None }).stable_hash(),
        private_view_hash: project_view(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat-0".to_owned())),
            },
        )
        .stable_hash(),
        replay_hash: trace.stable_hash(),
        terminal: state.terminal_outcome.is_some(),
        outcome: state.terminal_outcome,
    }
}

fn action_tree_hash(state: &high_card_duel::HighCardDuelState) -> HashValue {
    let actor = Actor {
        seat_id: active_commit_seat(state)
            .map(|seat| state.seats[seat.index()].clone())
            .unwrap_or_else(|| SeatId("seat-0".to_owned())),
    };
    let tree = legal_action_tree(state, &actor);
    let bytes = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

fn diagnostic_hash(diagnostics: &[engine_core::Diagnostic]) -> HashValue {
    let bytes = diagnostics
        .iter()
        .map(|diagnostic| format!("{}:{}", diagnostic.code, diagnostic.message))
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

fn assert_public_export_has_no_hidden_identity(seed: u64) {
    let trace = generate_internal_full_trace(seed);
    let replay = replay_internal_full_trace(&trace);
    let export = export_public_observer_replay(&trace);
    let export_json = export.to_json();

    assert!(!export_json.contains("\"seed\""));
    assert!(!export_json.contains("commit/hcd:r"));
    for hidden_card in replay.final_state.deck.iter() {
        assert!(!export_json.contains(&hidden_card.stable_id()));
    }
    for hand in &replay.final_state.hands {
        for hidden_card in hand {
            assert!(!export_json.contains(&hidden_card.stable_id()));
        }
    }
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
        Some(TerminalOutcome::Draw) | None => None,
    }
}

fn seat_id(seat: &str) -> SeatId {
    match seat {
        "seat_0" | "seat-0" => SeatId("seat-0".to_owned()),
        "seat_1" | "seat-1" => SeatId("seat-1".to_owned()),
        other => panic!("unknown fixture seat {other}"),
    }
}

fn default_seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
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
    if input[start..].trim_start().starts_with("null") {
        return None;
    }
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
                    return input[open + 1..open + offset].to_owned();
                }
            }
            _ => {}
        }
    }
    panic!("object `{key}` closes");
}

fn array_body(input: &str, key: &str) -> String {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .unwrap_or_else(|| panic!("missing `{key}`"))
        + needle.len();
    array_body_from(&input[start..], "")
}

fn array_body_from(input: &str, key: &str) -> String {
    let start = if key.is_empty() {
        0
    } else {
        input
            .find(&format!("\"{key}\":"))
            .unwrap_or_else(|| panic!("missing `{key}`"))
            + key.len()
            + 3
    };
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
    panic!("array `{key}` closes");
}

fn parse_string_at(input: &str, start: usize) -> Option<String> {
    let start = input[start..].find('"')? + start + 1;
    let mut value = String::new();
    let mut escaped = false;
    for ch in input[start..].chars() {
        if escaped {
            value.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            return Some(value);
        } else {
            value.push(ch);
        }
    }
    None
}

fn parse_number_at(input: &str, start: usize) -> Option<u64> {
    let tail = input[start..].trim_start();
    let digits = tail
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    (!digits.is_empty()).then(|| digits.parse().ok()).flatten()
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

#[test]
fn public_replay_export_has_no_unrevealed_internal_card_identities() {
    let trace = generate_internal_full_trace(12);
    let replay = replay_internal_full_trace(&trace);
    let export = export_public_observer_replay(&trace);
    let export_json = export.to_json();

    assert!(!export_json.contains("\"seed\""));
    assert!(!export_json.contains("commit/hcd:r"));
    for hidden_card in replay.final_state.deck.iter() {
        assert!(!export_json.contains(&hidden_card.stable_id()));
    }
    for hand in &replay.final_state.hands {
        for hidden_card in hand {
            assert!(!export_json.contains(&hidden_card.stable_id()));
        }
    }
    assert!(export_json.contains("commit_redacted"));
}

#[test]
fn import_public_export_produces_public_timeline_without_hidden_reconstruction() {
    let trace = generate_internal_full_trace(15);
    let export = export_public_observer_replay(&trace);
    let timeline = import_public_export(&export);

    assert_eq!(timeline.viewer, "observer");
    assert_eq!(timeline.steps, export.steps);
    assert_eq!(timeline.steps.len(), trace.command_paths.len() + 1);
    assert!(timeline
        .steps
        .iter()
        .all(|step| !step.redacted_command_summary.contains("hcd:r")));
    assert!(timeline.steps.iter().any(|step| step.terminal));
}
