use engine_core::{
    ActionPath, ActionTree, CommandEnvelope, EffectEnvelope, HashValue, RulesVersion, SeatId, Seed,
    StableSerialize,
};
use race_to_n::{
    apply_action, legal_action_tree, project_view, setup_match, validate_command, RaceEffect,
    RaceRandomBot, RaceSnapshot, RaceState, SetupOptions,
};

#[derive(Debug)]
struct TraceFixture {
    id: String,
    kind: String,
    note: String,
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReplayHashes {
    state_hash: HashValue,
    effect_hash: HashValue,
    action_tree_hash: HashValue,
    view_hash: HashValue,
    diagnostic_hash: Option<HashValue>,
    outcome: Option<race_to_n::RaceSeat>,
}

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor_for_state(state: &RaceState) -> engine_core::Actor {
    engine_core::Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

fn command_for_state(state: &RaceState, segment: String) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_state(state),
        action_path: ActionPath {
            segments: vec![segment],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn replay_commands(seed: u64, commands: &[String]) -> ReplayHashes {
    let mut state = setup_match(Seed(seed), &seats(), &SetupOptions::default()).unwrap();
    let mut effects = Vec::new();

    for segment in commands {
        let command = command_for_state(&state, segment.clone());
        let action = validate_command(&state, &command).expect("trace command validates");
        effects.extend(apply_action(&mut state, action));
    }

    hashes_for_state(&state, &effects, None)
}

fn replay_bot_action(seed: u64, bot_seed: u64) -> ReplayHashes {
    let mut state = setup_match(Seed(seed), &seats(), &SetupOptions::default()).unwrap();
    let bot = RaceRandomBot::new(Seed(bot_seed));
    let action_path = bot
        .select_action(&state, state.active_seat)
        .expect("bot action selected");
    let command = CommandEnvelope {
        actor: actor_for_state(&state),
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    let action = validate_command(&state, &command).expect("bot action validates");
    let effects = apply_action(&mut state, action);

    hashes_for_state(&state, &effects, None)
}

fn replay_invalid(seed: u64, invalid: &str, stale: &str) -> ReplayHashes {
    let state = setup_match(Seed(seed), &seats(), &SetupOptions::default()).unwrap();
    let invalid_diagnostic =
        validate_command(&state, &command_for_state(&state, invalid.to_owned()))
            .expect_err("invalid command rejected");
    let stale_command = CommandEnvelope {
        actor: actor_for_state(&state),
        action_path: ActionPath {
            segments: vec![stale.to_owned()],
        },
        freshness_token: state.freshness_token.next(),
        rules_version: RulesVersion(1),
    };
    let stale_diagnostic =
        validate_command(&state, &stale_command).expect_err("stale command rejected");
    let diagnostic_hash = HashValue::from_stable_bytes(
        format!(
            "{}:{}|{}:{}",
            invalid_diagnostic.code,
            invalid_diagnostic.message,
            stale_diagnostic.code,
            stale_diagnostic.message
        )
        .as_bytes(),
    );

    hashes_for_state(&state, &[], Some(diagnostic_hash))
}

fn hashes_for_state(
    state: &RaceState,
    effects: &[EffectEnvelope<RaceEffect>],
    diagnostic_hash: Option<HashValue>,
) -> ReplayHashes {
    let actor = actor_for_state(state);
    ReplayHashes {
        state_hash: RaceSnapshot::from_state(state).stable_hash(),
        effect_hash: effect_hash(effects),
        action_tree_hash: action_tree_hash(&legal_action_tree(state, &actor)),
        view_hash: project_view(state).stable_hash(),
        diagnostic_hash,
        outcome: state.winner,
    }
}

fn effect_hash(effects: &[EffectEnvelope<RaceEffect>]) -> HashValue {
    let bytes = effects
        .iter()
        .map(effect_json)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

fn effect_json(effect: &EffectEnvelope<RaceEffect>) -> String {
    match &effect.payload {
        RaceEffect::ActionStarted { actor, amount } => {
            format!("ActionStarted:{}:{amount}", actor.as_str())
        }
        RaceEffect::CounterAdvanced {
            actor,
            from,
            to,
            amount,
        } => format!(
            "CounterAdvanced:{}:{}:{}:{amount}",
            actor.as_str(),
            from.0,
            to.0
        ),
        RaceEffect::TurnChanged { next_actor } => {
            format!("TurnChanged:{}", next_actor.as_str())
        }
        RaceEffect::GameEnded { winner } => format!("GameEnded:{}", winner.as_str()),
        RaceEffect::ActionCompleted { actor } => format!("ActionCompleted:{}", actor.as_str()),
    }
}

fn action_tree_hash(tree: &ActionTree) -> HashValue {
    let bytes = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

fn parse_fixture(input: &str) -> TraceFixture {
    let mut fixture = TraceFixture {
        id: String::new(),
        kind: String::new(),
        note: String::new(),
        seed: 0,
        commands: Vec::new(),
        bot_seed: None,
        invalid_command: None,
        stale_command: None,
        expected_state_hash: 0,
        expected_effect_hash: 0,
        expected_action_tree_hash: 0,
        expected_view_hash: 0,
        expected_diagnostic_hash: None,
    };

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line.split_once('=').expect("fixture line is key=value");
        let value = value.trim();
        match key.trim() {
            "id" => fixture.id = value.to_owned(),
            "kind" => fixture.kind = value.to_owned(),
            "note" => fixture.note = value.to_owned(),
            "seed" => fixture.seed = value.parse().expect("seed parses"),
            "commands" => {
                fixture.commands = if value.is_empty() {
                    Vec::new()
                } else {
                    value.split(',').map(str::to_owned).collect()
                };
            }
            "bot_seed" => fixture.bot_seed = Some(value.parse().expect("bot seed parses")),
            "invalid_command" => fixture.invalid_command = Some(value.to_owned()),
            "stale_command" => fixture.stale_command = Some(value.to_owned()),
            "expected_state_hash" => {
                fixture.expected_state_hash = value.parse().expect("state hash parses");
            }
            "expected_effect_hash" => {
                fixture.expected_effect_hash = value.parse().expect("effect hash parses");
            }
            "expected_action_tree_hash" => {
                fixture.expected_action_tree_hash = value.parse().expect("tree hash parses");
            }
            "expected_view_hash" => {
                fixture.expected_view_hash = value.parse().expect("view hash parses");
            }
            "expected_diagnostic_hash" => {
                fixture.expected_diagnostic_hash =
                    Some(value.parse().expect("diagnostic hash parses"));
            }
            "not_applicable" => {}
            other => panic!("unknown fixture key {other}"),
        }
    }

    fixture
}

fn assert_fixture(fixture: TraceFixture) {
    assert!(!fixture.note.is_empty(), "{} has a trace note", fixture.id);
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
fn replay_reproduces_hashes_for_same_inputs() {
    let commands = vec!["add-3".to_owned(), "add-2".to_owned(), "add-1".to_owned()];
    let left = replay_commands(99, &commands);
    let right = replay_commands(99, &commands);

    assert_eq!(left, right);
}

#[test]
fn golden_traces_match_expected_hashes() {
    for fixture in [
        include_str!("golden_traces/shortest-normal.trace"),
        include_str!("golden_traces/terminal.trace"),
        include_str!("golden_traces/bot-action.trace"),
        include_str!("golden_traces/invalid-stale-diagnostic.trace"),
    ] {
        assert_fixture(parse_fixture(fixture));
    }
}

#[test]
fn trace_set_records_not_applicable_hidden_and_stochastic_rationale() {
    let note = include_str!("golden_traces/not-applicable.trace");

    assert!(note.contains("redacted-hidden-info=not-applicable"));
    assert!(note.contains("stochastic-game-event=not-applicable"));
}
