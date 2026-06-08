use engine_core::{
    ActionPath, Actor, CommandEnvelope, FreshnessToken, HashValue, RulesVersion, Seed,
    StableSerialize, Viewer,
};
use token_bazaar::{
    action_tree_hash, apply_action, default_seats, effect_hash, export_public_replay,
    import_public_export, legal_action_tree, project_view, setup_match, state_hash,
    validate_command, ContractId, ResourceCounts, TerminalOutcome, TokenBazaarLevel1Bot,
    TokenBazaarSeat, TokenBazaarState, GAME_ID, LEVEL1_POLICY_ID, RULES_VERSION_LABEL, VARIANT_ID,
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
    setup_patch: Option<String>,
    commands: Vec<TraceCommand>,
    expected_state_hash: u64,
    expected_effect_hash: u64,
    expected_action_tree_hash: u64,
    expected_public_view_hash: u64,
    expected_replay_hash: u64,
    expected_public_export_hash: u64,
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
fn golden_traces_replay_hashes_diagnostics_exports_and_no_leak_surfaces() {
    let fixtures = [
        include_str!("golden_traces/shortest-normal.trace.json"),
        include_str!("golden_traces/terminal-turn-cap.trace.json"),
        include_str!("golden_traces/contract-fulfill-refill.trace.json"),
        include_str!("golden_traces/market-exhaustion.trace.json"),
        include_str!("golden_traces/exchange.trace.json"),
        include_str!("golden_traces/supply-exhaustion-diagnostic.trace.json"),
        include_str!("golden_traces/insufficient-resources-diagnostic.trace.json"),
        include_str!("golden_traces/empty-slot-diagnostic.trace.json"),
        include_str!("golden_traces/stale-diagnostic.trace.json"),
        include_str!("golden_traces/non-active-seat-diagnostic.trace.json"),
        include_str!("golden_traces/bot-action.trace.json"),
        include_str!("golden_traces/wasm-exported.trace.json"),
    ];

    assert_eq!(fixtures.len(), 12);
    for input in fixtures {
        assert_public_safe_trace_surface(input);
        assert_trace_fixture(parse_trace_fixture(input));
    }
}

fn parse_trace_fixture(input: &str) -> TraceFixture {
    assert_eq!(number_field(input, "schema_version"), 1);
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
        setup_patch: optional_string_field(input, "setup_patch"),
        commands: commands(input),
        expected_state_hash: final_hash(input, "expected_state_hashes"),
        expected_effect_hash: final_hash(input, "expected_effect_hashes"),
        expected_action_tree_hash: final_hash(input, "expected_action_tree_hashes"),
        expected_public_view_hash: public_view_hash(input),
        expected_replay_hash: final_hash(input, "expected_replay_hashes"),
        expected_public_export_hash: final_hash(input, "expected_public_export_hashes"),
        expected_diagnostic_code,
        expected_diagnostic_hash: optional_diagnostic_hash(input),
        terminal: bool_field(&object_body(input, "expected_terminal_state"), "terminal"),
        winner: nullable_string_field(&object_body(input, "expected_terminal_state"), "winner"),
        draw: bool_field(&object_body(input, "expected_terminal_state"), "draw"),
    }
}

fn assert_trace_fixture(fixture: TraceFixture) {
    assert!(!fixture.note.is_empty(), "{} note", fixture.id);
    assert!(
        !fixture.migration_update_note.is_empty(),
        "{} migration note",
        fixture.id
    );
    assert_eq!(fixture.game_id, GAME_ID, "{} game id", fixture.id);
    assert_eq!(
        fixture.rules_version, RULES_VERSION_LABEL,
        "{} rules version",
        fixture.id
    );

    if fixture.kind == "bot" {
        assert_bot_fixture_command(&fixture);
    }

    let replay = replay_fixture(&fixture);
    assert_eq!(
        replay.state_hash,
        HashValue(fixture.expected_state_hash),
        "{} state hash",
        fixture.id
    );
    assert_eq!(
        replay.effect_hash,
        HashValue(fixture.expected_effect_hash),
        "{} effect hash",
        fixture.id
    );
    assert_eq!(
        replay.action_tree_hash,
        HashValue(fixture.expected_action_tree_hash),
        "{} action-tree hash",
        fixture.id
    );
    assert_eq!(
        replay.public_view_hash,
        HashValue(fixture.expected_public_view_hash),
        "{} public view hash",
        fixture.id
    );
    assert_eq!(
        replay.replay_hash,
        HashValue(fixture.expected_replay_hash),
        "{} replay hash",
        fixture.id
    );
    assert_eq!(
        replay.public_export_hash,
        HashValue(fixture.expected_public_export_hash),
        "{} public export hash",
        fixture.id
    );
    assert_eq!(replay.terminal, fixture.terminal, "{} terminal", fixture.id);
    assert_eq!(winner(&replay.outcome), fixture.winner.as_deref());
    assert_eq!(
        matches!(replay.outcome, Some(TerminalOutcome::Draw)),
        fixture.draw,
        "{} draw",
        fixture.id
    );

    if let Some(expected_code) = fixture.expected_diagnostic_code.as_deref() {
        assert_eq!(
            replay.diagnostic_code.as_deref(),
            Some(expected_code),
            "{} diagnostic code",
            fixture.id
        );
        assert_eq!(
            replay.diagnostic_hash,
            Some(HashValue(
                fixture
                    .expected_diagnostic_hash
                    .expect("diagnostic fixture has diagnostic hash"),
            )),
            "{} diagnostic hash",
            fixture.id
        );
    } else {
        assert!(
            replay.diagnostic_code.is_none(),
            "{} no diagnostic",
            fixture.id
        );
        assert!(
            fixture.expected_diagnostic_hash.is_none(),
            "{} no diagnostic hash",
            fixture.id
        );
    }

    if fixture.purpose == "public_export_round_trip" {
        let applied_paths = applied_paths(&fixture);
        let export = export_public_replay(fixture.seed, &applied_paths);
        let timeline = import_public_export(&export);
        assert_eq!(timeline.game_id, GAME_ID, "{} export game id", fixture.id);
        assert_eq!(
            timeline.variant, VARIANT_ID,
            "{} export variant",
            fixture.id
        );
        assert_eq!(
            timeline.steps.len(),
            applied_paths.len() + 1,
            "{} export step count",
            fixture.id
        );
    }
}

#[derive(Debug)]
struct ReplayHashes {
    state_hash: HashValue,
    effect_hash: HashValue,
    action_tree_hash: HashValue,
    public_view_hash: HashValue,
    replay_hash: HashValue,
    public_export_hash: HashValue,
    diagnostic_code: Option<String>,
    diagnostic_hash: Option<HashValue>,
    terminal: bool,
    outcome: Option<TerminalOutcome>,
}

fn replay_fixture(fixture: &TraceFixture) -> ReplayHashes {
    let mut state = setup_state(fixture);
    let mut effects = Vec::new();
    let mut diagnostic_code = None;
    let mut diagnostic_hash = None;

    for command in &fixture.commands {
        let envelope = command_envelope(&state, command);
        match command.expect.as_str() {
            "applied" => {
                let action =
                    validate_command(&state, &envelope).expect("fixture command validates");
                effects.extend(apply_action(&mut state, action));
            }
            "diagnostic" => {
                let before = state.clone();
                let diagnostic = validate_command(&state, &envelope)
                    .expect_err("fixture diagnostic command rejects");
                let expected_code = command
                    .expected_diagnostic_code
                    .as_deref()
                    .expect("diagnostic command has code");
                assert_eq!(
                    diagnostic.code, expected_code,
                    "{} command diagnostic code",
                    fixture.id
                );
                assert_eq!(
                    state, before,
                    "{} diagnostic did not mutate state",
                    fixture.id
                );
                diagnostic_hash = Some(HashValue::from_stable_bytes(
                    format!("diagnostic:{}", diagnostic.code).as_bytes(),
                ));
                diagnostic_code = Some(diagnostic.code);
            }
            other => panic!("unexpected command expectation {other}"),
        }
    }

    let actor = actor_for_hash(&state);
    let applied_paths = applied_paths(fixture);
    let replay_hash = replay_hash(fixture, &applied_paths);
    ReplayHashes {
        state_hash: state_hash(&state),
        effect_hash: effect_hash(&effects),
        action_tree_hash: action_tree_hash(&legal_action_tree(&state, &actor)),
        public_view_hash: project_view(&state, &Viewer { seat_id: None }).stable_hash(),
        replay_hash,
        public_export_hash: replay_hash,
        diagnostic_code,
        diagnostic_hash,
        terminal: state.terminal_outcome.is_some(),
        outcome: state.terminal_outcome,
    }
}

fn setup_state(fixture: &TraceFixture) -> TokenBazaarState {
    let mut state =
        setup_match(Seed(fixture.seed), &default_seats(), &Default::default()).expect("setup");
    match fixture.setup_patch.as_deref() {
        None => state,
        Some("near_market_exhaustion") => {
            state.slots = [Some(ContractId::CrownRoute), None, None];
            state.queue.clear();
            state.inventories[0] = ResourceCounts::new(2, 0, 2);
            state.supply = ResourceCounts::new(12, 14, 12);
            state
        }
        Some("empty_slot_non_terminal") => {
            state.slots = [None, Some(ContractId::CrownRoute), None];
            state.queue.clear();
            state.inventories[0] = ResourceCounts::new(2, 0, 2);
            state.supply = ResourceCounts::new(12, 14, 12);
            state
        }
        Some(other) => panic!("unknown setup patch {other}"),
    }
}

fn command_envelope(state: &TokenBazaarState, command: &TraceCommand) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[seat_from_fixture(&command.actor_seat).index()].clone(),
        },
        action_path: ActionPath {
            segments: command.action_path.clone(),
        },
        freshness_token: FreshnessToken(command.freshness_token),
        rules_version: RulesVersion(1),
    }
}

fn assert_bot_fixture_command(fixture: &TraceFixture) {
    let state =
        setup_match(Seed(fixture.seed), &default_seats(), &Default::default()).expect("setup");
    let decision = TokenBazaarLevel1Bot::new(Seed(fixture.seed))
        .select_decision(&state, TokenBazaarSeat::Seat0)
        .expect("bot decision");
    let expected = fixture
        .commands
        .iter()
        .find(|command| command.expect == "applied")
        .expect("bot fixture has command");

    assert_eq!(decision.policy_id, LEVEL1_POLICY_ID);
    assert_eq!(
        decision.action_path.segments, expected.action_path,
        "{} bot command",
        fixture.id
    );
}

fn actor_for_hash(state: &TokenBazaarState) -> Actor {
    if state.terminal_outcome.is_some() {
        Actor {
            seat_id: engine_core::SeatId("terminal".to_owned()),
        }
    } else {
        Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        }
    }
}

fn replay_hash(fixture: &TraceFixture, applied_paths: &[Vec<String>]) -> HashValue {
    if fixture.setup_patch.is_none() {
        export_public_replay(fixture.seed, applied_paths).stable_hash()
    } else {
        HashValue::from_stable_bytes(
            format!(
                "patched:{}:{}",
                trace_suffix(&fixture.id),
                command_summary(applied_paths)
            )
            .as_bytes(),
        )
    }
}

fn applied_paths(fixture: &TraceFixture) -> Vec<Vec<String>> {
    fixture
        .commands
        .iter()
        .filter(|command| command.expect == "applied")
        .map(|command| command.action_path.clone())
        .collect()
}

fn command_summary(commands: &[Vec<String>]) -> String {
    commands
        .iter()
        .map(|path| path.join("/"))
        .collect::<Vec<_>>()
        .join("|")
}

fn trace_suffix(trace_id: &str) -> &str {
    trace_id
        .strip_prefix("token-bazaar-")
        .expect("token bazaar trace id prefix")
}

fn winner(outcome: &Option<TerminalOutcome>) -> Option<&'static str> {
    match outcome {
        Some(TerminalOutcome::Win { seat }) => Some(seat.as_str()),
        _ => None,
    }
}

fn seat_from_fixture(value: &str) -> TokenBazaarSeat {
    match value {
        "seat_0" => TokenBazaarSeat::Seat0,
        "seat_1" => TokenBazaarSeat::Seat1,
        other => panic!("unknown fixture seat {other}"),
    }
}

fn assert_public_safe_trace_surface(input: &str) {
    for forbidden_field in [
        "\"debug\"",
        "\"candidate\"",
        "\"internal\"",
        "\"bot_debug\"",
        "\"bot_candidate\"",
    ] {
        assert!(
            !input.contains(forbidden_field),
            "trace contains forbidden field {forbidden_field}",
        );
    }
}

fn commands(input: &str) -> Vec<TraceCommand> {
    let body = array_body(input, "commands");
    let mut commands = Vec::new();
    let mut rest = body.as_str();
    while let Some(start) = rest.find('{') {
        let after_start = &rest[start + 1..];
        let end = after_start.find('}').expect("command object end");
        let object = &after_start[..end];
        commands.push(TraceCommand {
            actor_seat: string_field(object, "actor_seat"),
            action_path: string_array_field(object, "action_path"),
            freshness_token: string_field(object, "freshness_token")
                .parse()
                .expect("freshness number"),
            expect: string_field(object, "expect"),
            expected_diagnostic_code: optional_string_field(object, "expected_diagnostic_code"),
        });
        rest = &after_start[end + 1..];
    }
    commands
}

fn final_hash(input: &str, field: &str) -> u64 {
    number_field(&object_body(input, field), "final")
}

fn public_view_hash(input: &str) -> u64 {
    let body = object_body(input, "expected_public_view_hashes");
    if body.contains("\"all\":") {
        number_field(&body, "all")
    } else {
        number_field(&body, "observer")
    }
}

fn optional_diagnostic_hash(input: &str) -> Option<u64> {
    let body = object_body(input, "expected_diagnostic_hashes");
    if body.trim() == "null" {
        None
    } else {
        Some(number_field(&body, "final"))
    }
}

fn object_body(input: &str, field: &str) -> String {
    let marker = format!("\"{field}\":");
    let start = input.find(&marker).expect("object field exists") + marker.len();
    let after = input[start..].trim_start();
    if after.starts_with("null") {
        return "null".to_owned();
    }
    let open = after.find('{').expect("object open");
    let mut depth = 0_i32;
    for (index, ch) in after[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return after[open + 1..open + index].to_owned();
                }
            }
            _ => {}
        }
    }
    panic!("object body not closed for {field}");
}

fn array_body(input: &str, field: &str) -> String {
    let marker = format!("\"{field}\":");
    let start = input.find(&marker).expect("array field exists") + marker.len();
    let after = input[start..].trim_start();
    let open = after.find('[').expect("array open");
    let mut depth = 0_i32;
    for (index, ch) in after[open..].char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return after[open + 1..open + index].to_owned();
                }
            }
            _ => {}
        }
    }
    panic!("array body not closed for {field}");
}

fn string_field(input: &str, field: &str) -> String {
    optional_string_field(input, field).unwrap_or_else(|| panic!("missing string field {field}"))
}

fn optional_string_field(input: &str, field: &str) -> Option<String> {
    let marker = format!("\"{field}\":");
    let start = input.find(&marker)? + marker.len();
    let after = input[start..].trim_start();
    let quoted = after.strip_prefix('"')?;
    let end = quoted.find('"').expect("string close");
    Some(quoted[..end].to_owned())
}

fn nullable_string_field(input: &str, field: &str) -> Option<String> {
    let marker = format!("\"{field}\":");
    let start = input.find(&marker).expect("nullable field exists") + marker.len();
    let after = input[start..].trim_start();
    if after.starts_with("null") {
        None
    } else {
        Some(string_field(input, field))
    }
}

fn number_field(input: &str, field: &str) -> u64 {
    let marker = format!("\"{field}\":");
    let start = input.find(&marker).expect("number field exists") + marker.len();
    let after = input[start..].trim_start();
    let digits = after
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    digits.parse().expect("number parses")
}

fn bool_field(input: &str, field: &str) -> bool {
    let marker = format!("\"{field}\":");
    let start = input.find(&marker).expect("bool field exists") + marker.len();
    let after = input[start..].trim_start();
    if after.starts_with("true") {
        true
    } else if after.starts_with("false") {
        false
    } else {
        panic!("bool field {field} did not parse")
    }
}

fn string_array_field(input: &str, field: &str) -> Vec<String> {
    let body = array_body(input, field);
    body.split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .strip_prefix('"')
                .and_then(|value| value.strip_suffix('"'))
                .expect("string array entry")
                .to_owned()
        })
        .collect()
}
