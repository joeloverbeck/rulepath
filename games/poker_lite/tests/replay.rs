use engine_core::{
    ActionPath, Actor, CommandEnvelope, FreshnessToken, HashValue, RulesVersion, SeatId,
    StableSerialize, Viewer,
};
use poker_lite::{
    apply_action, legal_action_tree,
    replay_support::{
        action_tree_hash, effect_hash, export_public_replay, import_public_export, state_hash,
        trace_from_commands, view_hash, PokerLiteInternalTrace, ReplayCommand,
    },
    setup_effects, setup_match, validate_command, Phase, PokerLiteSeat, PokerLiteState,
    SetupOptions, TerminalOutcome, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID,
};

#[derive(Debug)]
struct TraceFixture {
    id: String,
    purpose: String,
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
    terminal: bool,
    winner: Option<String>,
    draw: bool,
}

#[derive(Debug)]
struct TraceCommand {
    actor_seat: PokerLiteSeat,
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
    terminal: bool,
    outcome: Option<TerminalOutcome>,
    export_json: String,
}

#[test]
fn golden_traces_match_expected_replay_hashes_diagnostics_and_no_leak_surfaces() {
    let fixtures = [
        include_str!("golden_traces/deal-private-no-leak.trace.json"),
        include_str!("golden_traces/hold-hold-center-reveal.trace.json"),
        include_str!("golden_traces/press-match-showdown-reveal.trace.json"),
        include_str!("golden_traces/lift-match-showdown.trace.json"),
        include_str!("golden_traces/yield-terminal-no-showdown.trace.json"),
        include_str!("golden_traces/pair-beats-high-card.trace.json"),
        include_str!("golden_traces/high-card-showdown.trace.json"),
        include_str!("golden_traces/tie-split.trace.json"),
        include_str!("golden_traces/no-leak-public-observer.trace.json"),
        include_str!("golden_traces/seat-private-view.trace.json"),
        include_str!("golden_traces/invalid-wrong-seat-diagnostic.trace.json"),
        include_str!("golden_traces/invalid-stale-diagnostic.trace.json"),
        include_str!("golden_traces/invalid-lift-cap-diagnostic.trace.json"),
        include_str!("golden_traces/invalid-private-card-redacted.trace.json"),
        include_str!("golden_traces/public-replay-export-import.trace.json"),
    ];

    assert_eq!(fixtures.len(), 15);
    for input in fixtures {
        let fixture = parse_trace_fixture(input);
        assert_trace_fixture(&fixture);
    }
}

#[test]
fn internal_trace_replays_to_the_same_hashes_and_terminal() {
    let trace = trace_from_commands(
        0,
        &[
            (PokerLiteSeat::Seat0, "hold"),
            (PokerLiteSeat::Seat1, "hold"),
            (PokerLiteSeat::Seat1, "hold"),
            (PokerLiteSeat::Seat0, "hold"),
        ],
    );
    let first = poker_lite::replay_support::replay_internal_full_trace(&trace);
    let second = poker_lite::replay_support::replay_internal_full_trace(&trace);

    assert_eq!(first.trace_hash, second.trace_hash);
    assert_eq!(first.state_hash, second.state_hash);
    assert_eq!(first.effect_hash, second.effect_hash);
    assert_eq!(first.view_hash, second.view_hash);
    assert_eq!(first.action_tree_hashes, second.action_tree_hashes);
    assert_eq!(first.final_state.phase, Phase::Terminal);
}

#[test]
fn public_export_import_round_trips_for_observer_and_seat_viewer() {
    let trace = trace_from_commands(
        11,
        &[
            (PokerLiteSeat::Seat0, "press"),
            (PokerLiteSeat::Seat1, "yield"),
        ],
    );
    for viewer in [
        Viewer { seat_id: None },
        Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        },
    ] {
        let export = export_public_replay(&trace, &viewer);
        let imported = import_public_export(&export);

        assert_eq!(imported.viewer, export.viewer);
        assert_eq!(imported.steps, export.steps);
        assert_eq!(
            poker_lite::replay_support::PublicReplayExport::from_json(&export.to_json())
                .expect("export parses"),
            export
        );
    }
}

#[test]
fn yield_terminal_public_export_cannot_reconstruct_folded_private_cards() {
    let trace = trace_from_commands(
        11,
        &[
            (PokerLiteSeat::Seat0, "press"),
            (PokerLiteSeat::Seat1, "yield"),
        ],
    );
    let state = setup_state(trace.seed_evidence);
    let export = export_public_replay(&trace, &Viewer { seat_id: None });
    let json = export.to_json();

    assert_no_private_cards(&json, &state);
    assert!(!json.contains("seed_evidence"));
    assert!(!json.contains("\"seed\""));
}

fn assert_trace_fixture(fixture: &TraceFixture) {
    let first = replay_fixture(fixture);
    let second = replay_fixture(fixture);

    assert_eq!(first.state_hash, second.state_hash, "{} state", fixture.id);
    assert_eq!(
        first.effect_hash, second.effect_hash,
        "{} effects",
        fixture.id
    );
    assert_eq!(
        first.action_tree_hash, second.action_tree_hash,
        "{} action tree",
        fixture.id
    );
    assert_eq!(
        first.observer_view_hash, second.observer_view_hash,
        "{} observer view",
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
    assert_eq!(first.terminal, fixture.terminal, "{} terminal", fixture.id);
    assert_eq!(
        winner(&first.outcome),
        fixture.winner.as_deref(),
        "{} winner",
        fixture.id
    );
    assert_eq!(
        matches!(first.outcome, Some(TerminalOutcome::Split { .. })),
        fixture.draw,
        "{} draw",
        fixture.id
    );

    if fixture.purpose.contains("no_leak")
        || fixture.purpose.contains("redacted")
        || fixture.purpose == "public_replay_export_import"
    {
        let state = setup_state(fixture.seed);
        assert_no_private_cards(&first.export_json, &state);
        assert!(!first.export_json.contains("seed_evidence"));
        assert!(!first.export_json.contains("\"seed\""));
    }

    if fixture.purpose == "public_replay_export_import" {
        let trace = internal_trace_from_fixture(fixture);
        let export = export_public_replay(&trace, &Viewer { seat_id: None });
        let imported = import_public_export(&export);
        assert_eq!(imported.viewer, "observer");
        assert_eq!(imported.steps, export.steps);
    }
}

fn replay_fixture(fixture: &TraceFixture) -> ReplayActual {
    let mut state = setup_state(fixture.seed);
    let mut effects = setup_effects(&state);
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

    let trace = PokerLiteInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
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
        terminal: state.phase == Phase::Terminal,
        outcome: state.terminal_outcome,
        export_json: export.to_json(),
    }
}

fn setup_state(seed: u64) -> PokerLiteState {
    setup_match(
        engine_core::Seed(seed),
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn export_viewer_for_fixture(fixture: &TraceFixture) -> Viewer {
    if fixture.purpose == "seat_private_view" {
        Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        }
    } else {
        Viewer { seat_id: None }
    }
}

fn command_envelope(state: &PokerLiteState, command: &TraceCommand) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[command.actor_seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: command.action_path.clone(),
        },
        freshness_token: FreshnessToken(command.freshness_token),
        rules_version: RulesVersion(1),
    }
}

fn combined_action_tree_hash(state: &PokerLiteState) -> HashValue {
    let parts = PokerLiteSeat::ALL
        .iter()
        .map(|seat| {
            let actor = Actor {
                seat_id: state.seats[seat.index()].clone(),
            };
            action_tree_hash(&legal_action_tree(state, &actor))
                .0
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(parts.as_bytes())
}

fn internal_trace_from_fixture(fixture: &TraceFixture) -> PokerLiteInternalTrace {
    PokerLiteInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
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

fn assert_no_private_cards(text: &str, state: &PokerLiteState) {
    for card in state.private_cards_internal() {
        assert!(!text.contains(card.as_str()), "{text}");
        assert!(!text.contains(&card.label()), "{text}");
    }
}

fn winner(outcome: &Option<TerminalOutcome>) -> Option<&'static str> {
    match outcome {
        Some(TerminalOutcome::YieldWin { winner, .. })
        | Some(TerminalOutcome::ShowdownWin { winner, .. }) => Some(winner.as_str()),
        Some(TerminalOutcome::Split { .. }) | None => None,
    }
}

fn parse_trace_fixture(input: &str) -> TraceFixture {
    assert_eq!(number_field(input, "schema_version"), 1);
    assert_eq!(string_field(input, "game_id"), GAME_ID);
    assert_eq!(string_field(input, "rules_version"), RULES_VERSION_LABEL);
    assert_eq!(string_field(input, "variant"), VARIANT_ID);
    let public_view_hashes = object_body(input, "expected_public_view_hashes");
    let diagnostic_body = input.find("\"expected_diagnostics\":").and_then(|start| {
        let rest = &input[start..];
        (!rest.starts_with("\"expected_diagnostics\": null"))
            .then(|| object_body(rest, "expected_diagnostics"))
    });
    TraceFixture {
        id: string_field(input, "trace_id"),
        purpose: string_field(input, "purpose"),
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
        terminal: bool_field(&object_body(input, "expected_terminal_state"), "terminal"),
        winner: nullable_string_field(&object_body(input, "expected_terminal_state"), "winner"),
        draw: bool_field(&object_body(input, "expected_terminal_state"), "draw"),
    }
}

fn commands(input: &str) -> Vec<TraceCommand> {
    let body = array_body(input, "commands");
    if body.trim().is_empty() {
        return Vec::new();
    }
    split_top_level(&body, ',')
        .into_iter()
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let expected_code = optional_string_field(&entry, "expected_diagnostic_code");
            TraceCommand {
                actor_seat: PokerLiteSeat::parse(&string_field(&entry, "actor_seat"))
                    .expect("trace seat is valid"),
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

fn parse_json_string(raw: &str) -> String {
    raw.trim()
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .expect("expected JSON string")
        .replace("\\n", "\n")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
}
