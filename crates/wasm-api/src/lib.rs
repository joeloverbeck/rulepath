//! Browser-facing Rulepath API surface.

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    slice, str,
};

use engine_core::{
    ActionChoice, ActionPath, ActionTree, Actor, CommandEnvelope, EffectCursor, EffectEnvelope,
    EffectLog, RulesVersion, SeatId, Seed, Viewer, VisibilityScope,
};
use race_to_n::{
    apply_action as race_apply_action, legal_action_tree, project_view,
    replay_support::replay_commands, setup_match, RaceEffect, RaceRandomBot, RaceSeat, RaceState,
    SetupOptions,
};

const API_VERSION: &str = "rulepath-wasm-api/0.1.0";
const GAME_RACE_TO_N: &str = "race_to_n";
const GAME_RACE_TO_N_DISPLAY_NAME: &str = "Race to 21";
const RULES_VERSION: u32 = 1;
const SCHEMA_VERSION: u32 = 1;
const SUPPORTED_OPERATIONS: &[&str] = &[
    "feature_report",
    "list_games",
    "new_match",
    "get_view",
    "get_action_tree",
    "apply_action",
    "run_bot_turn",
    "get_effects",
    "export_replay",
    "import_replay",
    "replay_step",
    "replay_reset",
];
const FEATURE_FLAGS: &[&str] = &["catalog", "match_store", "legal_action_tree", "effects"];
const TRACE_RULES_VERSION: &str = "race_to_n-rules-v1";
const ENGINE_VERSION: &str = "engine-core-0.1.0";
const DATA_VERSION: &str = "1";
const VARIANT_RACE_TO_21: &str = "race_to_21";
const MAX_REPLAY_IMPORT_BYTES: usize = 128 * 1024;

thread_local! {
    static MATCHES: RefCell<BTreeMap<String, MatchRecord>> = const { RefCell::new(BTreeMap::new()) };
    static REPLAYS: RefCell<BTreeMap<String, ReplayRecord>> = const { RefCell::new(BTreeMap::new()) };
    static NEXT_MATCH_ID: Cell<u64> = const { Cell::new(1) };
    static NEXT_REPLAY_ID: Cell<u64> = const { Cell::new(1) };
    static LAST_OUTPUT: RefCell<String> = const { RefCell::new(String::new()) };
}

#[derive(Clone, Debug)]
struct MatchRecord {
    game_id: String,
    seed: u64,
    state: RaceState,
    effects: EffectLog<RaceEffect>,
    commands: Vec<AppliedCommand>,
}

#[derive(Clone, Debug)]
struct ReplayRecord {
    game_id: String,
    seed: u64,
    commands: Vec<AppliedCommand>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AppliedCommand {
    actor_seat: RaceSeat,
    action_path: Vec<String>,
    freshness_token: u64,
}

impl AppliedCommand {
    fn single_segment(&self) -> Result<String, String> {
        if self.action_path.len() != 1 {
            return Err(diagnostic_string(
                "unsupported_replay_action_path",
                "race_to_n replay export supports one-segment action paths",
            ));
        }
        Ok(self.action_path[0].clone())
    }
}

#[derive(Clone, Debug)]
struct ParsedReplayDocument {
    schema_version: u64,
    game_id: String,
    rules_version: String,
    seed: u64,
    commands: Vec<AppliedCommand>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RegisteredGame {
    RaceToN,
}

pub fn placeholder_version() -> &'static str {
    API_VERSION
}

pub fn list_games() -> Result<String, String> {
    let games = [RegisteredGame::RaceToN]
        .iter()
        .map(|game| match game {
            RegisteredGame::RaceToN => format!(
                "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"rules_version\":{},\"schema_version\":{}}}",
                escape_json(GAME_RACE_TO_N),
                escape_json(GAME_RACE_TO_N_DISPLAY_NAME),
                RULES_VERSION,
                SCHEMA_VERSION
            ),
        })
        .collect::<Vec<_>>()
        .join(",");
    Ok(format!("[{games}]"))
}

pub fn feature_report() -> Result<String, String> {
    Ok(format!(
        "{{\"api_version\":\"{}\",\"operations\":{},\"features\":{}}}",
        escape_json(API_VERSION),
        string_array_json(SUPPORTED_OPERATIONS),
        string_array_json(FEATURE_FLAGS)
    ))
}

pub fn new_match(game_id: &str, seed: u64) -> Result<String, String> {
    match resolve_game(game_id)? {
        RegisteredGame::RaceToN => {
            let seats = seats();
            let state = setup_match(Seed(seed), &seats, &SetupOptions::default())
                .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord {
                        game_id: game_id.to_owned(),
                        seed,
                        state,
                        effects: EffectLog::new(),
                        commands: Vec::new(),
                    },
                );
            });
            Ok(format!(
                "{{\"match_id\":\"{}\",\"game_id\":\"{}\"}}",
                escape_json(&match_id),
                escape_json(game_id)
            ))
        }
    }
}

pub fn get_view(match_id: &str, _viewer_seat: Option<&str>) -> Result<String, String> {
    with_match(match_id, |record| {
        resolve_game(&record.game_id)?;
        Ok(project_view(&record.state).to_json())
    })
}

pub fn get_action_tree(match_id: &str, actor_seat: &str) -> Result<String, String> {
    with_match(match_id, |record| {
        resolve_game(&record.game_id)?;
        let actor = actor_for_seat(&record.state, parse_seat(actor_seat)?)?;
        Ok(action_tree_json(&legal_action_tree(&record.state, &actor)))
    })
}

pub fn apply_action(
    match_id: &str,
    actor_seat: &str,
    action_path: &str,
    freshness_token: u64,
) -> Result<String, String> {
    with_match_mut(match_id, |record| {
        resolve_game(&record.game_id)?;
        let seat = parse_seat(actor_seat)?;
        let command = CommandEnvelope {
            actor: actor_for_seat(&record.state, seat)?,
            action_path: parse_action_path(action_path),
            freshness_token: engine_core::FreshnessToken(freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action =
            race_to_n::validate_command(&record.state, &command).map_err(diagnostic_json)?;
        let effects = race_apply_action(&mut record.state, action);
        let effect_json = effects_json(&effects);
        for effect in effects {
            record.effects.push(effect);
        }
        record.commands.push(AppliedCommand {
            actor_seat: seat,
            action_path: command.action_path.segments,
            freshness_token,
        });
        Ok(format!(
            "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
            effect_json,
            project_view(&record.state).to_json()
        ))
    })
}

pub fn run_bot_turn(match_id: &str, actor_seat: &str, bot_seed: u64) -> Result<String, String> {
    with_match_mut(match_id, |record| {
        resolve_game(&record.game_id)?;
        let seat = parse_seat(actor_seat)?;
        let bot = RaceRandomBot::new(Seed(bot_seed));
        let action_path = bot
            .select_action(&record.state, seat)
            .map_err(diagnostic_json)?;
        let command = CommandEnvelope {
            actor: actor_for_seat(&record.state, seat)?,
            action_path,
            freshness_token: record.state.freshness_token,
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action =
            race_to_n::validate_command(&record.state, &command).map_err(diagnostic_json)?;
        let effects = race_apply_action(&mut record.state, action);
        let effect_json = effects_json(&effects);
        for effect in effects {
            record.effects.push(effect);
        }
        record.commands.push(AppliedCommand {
            actor_seat: seat,
            action_path: command.action_path.segments,
            freshness_token: command.freshness_token.0,
        });
        Ok(format!(
            "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
            effect_json,
            project_view(&record.state).to_json()
        ))
    })
}

pub fn get_effects(
    match_id: &str,
    since_cursor: u64,
    viewer_seat: Option<&str>,
) -> Result<String, String> {
    with_match(match_id, |record| {
        resolve_game(&record.game_id)?;
        let viewer = viewer_for_seat(&record.state, viewer_seat)?;
        let effects = record
            .effects
            .since(EffectCursor(since_cursor), &viewer)
            .into_iter()
            .map(|logged| {
                format!(
                    "{{\"cursor\":{},\"effect\":{}}}",
                    logged.cursor.0,
                    effect_json(&logged.envelope)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        Ok(format!("[{effects}]"))
    })
}

pub fn export_replay(match_id: &str) -> Result<String, String> {
    with_match(match_id, |record| {
        resolve_game(&record.game_id)?;
        replay_document_json(&format!("export-{match_id}"), record.seed, &record.commands)
    })
}

pub fn import_replay(doc: &str) -> Result<String, String> {
    if doc.len() > MAX_REPLAY_IMPORT_BYTES {
        return Err(diagnostic_string(
            "replay_too_large",
            "replay document exceeds import size limit",
        ));
    }
    let parsed = parse_replay_document(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid replay document: {message}"),
        )
    })?;
    if parsed.game_id != GAME_RACE_TO_N {
        return Err(diagnostic_string(
            "unsupported_replay_game",
            &format!("unsupported replay game id: {}", parsed.game_id),
        ));
    }
    if parsed.schema_version != SCHEMA_VERSION as u64 {
        return Err(diagnostic_string(
            "unsupported_replay_schema",
            &format!(
                "unsupported replay schema version: {}",
                parsed.schema_version
            ),
        ));
    }
    if parsed.rules_version != TRACE_RULES_VERSION {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {}", parsed.rules_version),
        ));
    }

    let (state, effects) = replay_to_cursor(parsed.seed, &parsed.commands, parsed.commands.len())?;
    let replay_id = next_replay_id(&parsed.game_id);
    let command_count = parsed.commands.len();
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: parsed.game_id,
                seed: parsed.seed,
                commands: parsed.commands,
            },
        );
    });

    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"command_count\":{},\"final_view\":{},\"effect_count\":{}}}",
        escape_json(&replay_id),
        escape_json(GAME_RACE_TO_N),
        command_count,
        project_view(&state).to_json(),
        effects.len()
    ))
}

pub fn replay_step(replay_id: &str, cursor: usize) -> Result<String, String> {
    with_replay(replay_id, |record| {
        resolve_game(&record.game_id)?;
        let bounded_cursor = cursor.min(record.commands.len());
        let (state, effects) = replay_to_cursor(record.seed, &record.commands, bounded_cursor)?;
        Ok(replay_step_json(
            replay_id,
            bounded_cursor,
            record.commands.len(),
            &state,
            &effects,
        ))
    })
}

pub fn replay_reset(replay_id: &str) -> Result<String, String> {
    replay_step(replay_id, 0)
}

fn resolve_game(game_id: &str) -> Result<RegisteredGame, String> {
    match game_id {
        GAME_RACE_TO_N => Ok(RegisteredGame::RaceToN),
        _ => Err(format!(
            "{{\"code\":\"unknown_game\",\"message\":\"unsupported game id: {}\"}}",
            escape_json(game_id)
        )),
    }
}

fn next_replay_id(game_id: &str) -> String {
    NEXT_REPLAY_ID.with(|next| {
        let id = next.get();
        next.set(id.saturating_add(1));
        format!("{game_id}-replay-{id}")
    })
}

fn next_match_id(game_id: &str) -> String {
    NEXT_MATCH_ID.with(|next| {
        let id = next.get();
        next.set(id.saturating_add(1));
        format!("{game_id}-{id}")
    })
}

fn with_match<T>(
    match_id: &str,
    read: impl FnOnce(&MatchRecord) -> Result<T, String>,
) -> Result<T, String> {
    MATCHES.with(|matches| {
        let matches = matches.borrow();
        let record = matches
            .get(match_id)
            .ok_or_else(|| missing_match_json(match_id))?;
        read(record)
    })
}

fn with_match_mut<T>(
    match_id: &str,
    update: impl FnOnce(&mut MatchRecord) -> Result<T, String>,
) -> Result<T, String> {
    MATCHES.with(|matches| {
        let mut matches = matches.borrow_mut();
        let record = matches
            .get_mut(match_id)
            .ok_or_else(|| missing_match_json(match_id))?;
        update(record)
    })
}

fn with_replay<T>(
    replay_id: &str,
    read: impl FnOnce(&ReplayRecord) -> Result<T, String>,
) -> Result<T, String> {
    REPLAYS.with(|replays| {
        let replays = replays.borrow();
        let record = replays
            .get(replay_id)
            .ok_or_else(|| missing_replay_json(replay_id))?;
        read(record)
    })
}

fn missing_match_json(match_id: &str) -> String {
    format!(
        "{{\"code\":\"unknown_match\",\"message\":\"unknown match id: {}\"}}",
        escape_json(match_id)
    )
}

fn missing_replay_json(replay_id: &str) -> String {
    diagnostic_string("unknown_replay", &format!("unknown replay id: {replay_id}"))
}

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn parse_seat(value: &str) -> Result<RaceSeat, String> {
    RaceSeat::parse(value).ok_or_else(|| {
        format!(
            "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
            escape_json(value)
        )
    })
}

fn parse_replay_seat(value: &str) -> Result<RaceSeat, String> {
    match value {
        "seat-0" => Ok(RaceSeat::Seat0),
        "seat-1" => Ok(RaceSeat::Seat1),
        _ => parse_seat(value),
    }
}

fn trace_seat(seat: RaceSeat) -> &'static str {
    match seat {
        RaceSeat::Seat0 => "seat-0",
        RaceSeat::Seat1 => "seat-1",
    }
}

fn actor_for_seat(state: &RaceState, seat: RaceSeat) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

fn viewer_for_seat(state: &RaceState, seat: Option<&str>) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

fn parse_action_path(action_path: &str) -> ActionPath {
    ActionPath {
        segments: action_path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .map(str::to_owned)
            .collect(),
    }
}

fn replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(RaceState, Vec<EffectEnvelope<RaceEffect>>), String> {
    let seats = seats();
    let mut state =
        setup_match(Seed(seed), &seats, &SetupOptions::default()).map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = command.actor_seat;
        let envelope = CommandEnvelope {
            actor: actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = race_to_n::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(race_apply_action(&mut state, action));
    }
    Ok((state, all_effects))
}

fn action_tree_json(tree: &ActionTree) -> String {
    let choices = tree
        .root
        .choices
        .iter()
        .map(action_choice_json)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"freshness_token\":{},\"choices\":[{}]}}",
        tree.freshness_token.0, choices
    )
}

fn action_choice_json(choice: &ActionChoice) -> String {
    let metadata = choice
        .metadata
        .iter()
        .map(|entry| {
            format!(
                "{{\"key\":\"{}\",\"value\":\"{}\"}}",
                escape_json(&entry.key),
                escape_json(&entry.value)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let tags = choice
        .tags
        .iter()
        .map(|tag| format!("\"{}\"", escape_json(tag)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"segment\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\",\"metadata\":[{}],\"tags\":[{}]}}",
        escape_json(&choice.segment),
        escape_json(&choice.label),
        escape_json(&choice.accessibility_label),
        metadata,
        tags
    )
}

fn effects_json(effects: &[EffectEnvelope<RaceEffect>]) -> String {
    let body = effects
        .iter()
        .map(effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let command_segments = commands
        .iter()
        .map(|command| command.single_segment())
        .collect::<Result<Vec<_>, _>>()?;
    let hashes = replay_commands(seed, &command_segments);
    let commands_json = commands
        .iter()
        .enumerate()
        .map(|(index, command)| command_record_json(index, command))
        .collect::<Vec<_>>()
        .join(",");
    let checkpoints = if commands.is_empty() {
        "[{\"id\":\"final\",\"after_command_index\":0}]".to_owned()
    } else {
        format!(
            "[{{\"id\":\"final\",\"after_command_index\":{}}}]",
            commands.len().saturating_sub(1)
        )
    };
    let terminal = hashes.outcome.is_some();
    let winner = hashes.outcome.map_or_else(
        || "null".to_owned(),
        |winner| format!("\"{}\"", winner.as_str()),
    );

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported_replay\",\"note\":\"Replay exported by the Rulepath WASM API from the Rust command log.\",\"migration_update_note\":\"Generated by Gate 3 WASM replay export; expected hashes are computed by Rust replay support.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat-0\",\"player_id\":\"player-0\"}},{{\"seat_id\":\"seat-1\",\"player_id\":\"player-1\"}}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_private_view_hashes\":{{\"not_applicable\":\"race_to_n is perfect-information and has no private-view API.\"}},\"expected_outcome\":{{\"terminal\":{},\"winner\":{}}},\"expected_terminal_state\":{{\"terminal\":{},\"winner\":{}}},\"not_applicable\":{{\"hidden_information\":\"race_to_n is perfect-information and has no hidden state to redact.\",\"stochastic_game_events\":\"race_to_n game rules use no randomness; bot RNG is not replayed from exported documents because resolved commands are recorded.\",\"private_view_hashes\":\"race_to_n has no private-view API.\",\"preview_hashes\":\"race_to_n has no Rust preview surface in Gate 3.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_RACE_TO_N),
        escape_json(TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_RACE_TO_21),
        commands_json,
        checkpoints,
        hashes.state_hash.0,
        hashes.effect_hash.0,
        hashes.action_tree_hash.0,
        hashes.view_hash.0,
        terminal,
        winner,
        terminal,
        winner
    ))
}

fn command_record_json(index: usize, command: &AppliedCommand) -> String {
    let action_path = command
        .action_path
        .iter()
        .map(|segment| format!("\"{}\"", escape_json(segment)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"index\":{},\"actor_seat\":\"{}\",\"action_path\":[{}],\"freshness_token\":\"{}\",\"expect\":\"applied\"}}",
        index,
        trace_seat(command.actor_seat),
        action_path,
        command.freshness_token
    )
}

fn replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &RaceState,
    effects: &[EffectEnvelope<RaceEffect>],
) -> String {
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        project_view(state).to_json(),
        effects_json(effects)
    )
}

fn effect_json(effect: &EffectEnvelope<RaceEffect>) -> String {
    let visibility = match &effect.visibility {
        VisibilityScope::Public => "\"public\"".to_owned(),
        VisibilityScope::PrivateToSeat(seat) => {
            format!("{{\"private_to_seat\":\"{}\"}}", escape_json(&seat.0))
        }
    };
    let payload = match &effect.payload {
        RaceEffect::ActionStarted { actor, amount } => format!(
            "{{\"type\":\"action_started\",\"actor\":\"{}\",\"amount\":{}}}",
            actor.as_str(),
            amount
        ),
        RaceEffect::CounterAdvanced {
            actor,
            from,
            to,
            amount,
        } => format!(
            "{{\"type\":\"counter_advanced\",\"actor\":\"{}\",\"from\":{},\"to\":{},\"amount\":{}}}",
            actor.as_str(),
            from.0,
            to.0,
            amount
        ),
        RaceEffect::TurnChanged { next_actor } => format!(
            "{{\"type\":\"turn_changed\",\"next_actor\":\"{}\"}}",
            next_actor.as_str()
        ),
        RaceEffect::GameEnded { winner } => {
            format!("{{\"type\":\"game_ended\",\"winner\":\"{}\"}}", winner.as_str())
        }
        RaceEffect::ActionCompleted { actor } => format!(
            "{{\"type\":\"action_completed\",\"actor\":\"{}\"}}",
            actor.as_str()
        ),
    };
    format!("{{\"visibility\":{},\"payload\":{}}}", visibility, payload)
}

fn diagnostic_json(diagnostic: engine_core::Diagnostic) -> String {
    format!(
        "{{\"code\":\"{}\",\"message\":\"{}\"}}",
        escape_json(&diagnostic.code),
        escape_json(&diagnostic.message)
    )
}

fn diagnostic_string(code: &str, message: &str) -> String {
    format!(
        "{{\"code\":\"{}\",\"message\":\"{}\"}}",
        escape_json(code),
        escape_json(message)
    )
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

fn string_array_json(values: &[&str]) -> String {
    let body = values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn parse_replay_document(input: &str) -> Result<ParsedReplayDocument, String> {
    validate_json_object(input)?;
    reject_unknown_root_fields(
        input,
        &[
            "schema_version",
            "trace_id",
            "fixture_kind",
            "purpose",
            "note",
            "migration_update_note",
            "game_id",
            "rules_version",
            "engine_version",
            "data_version",
            "seed",
            "variant",
            "options",
            "seats",
            "commands",
            "checkpoints",
            "expected_state_hashes",
            "expected_effect_hashes",
            "expected_action_tree_hashes",
            "expected_public_view_hashes",
            "expected_private_view_hashes",
            "expected_diagnostics",
            "expected_outcome",
            "expected_terminal_state",
            "not_applicable",
        ],
    )?;

    let commands = array_items(input, "commands")?
        .into_iter()
        .map(|command| parse_replay_command(&command))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ParsedReplayDocument {
        schema_version: number_field(input, "schema_version")?,
        game_id: string_field(input, "game_id")?,
        rules_version: string_field(input, "rules_version")?,
        seed: number_field(input, "seed")?,
        commands,
    })
}

fn parse_replay_command(input: &str) -> Result<AppliedCommand, String> {
    validate_json_object(input)?;
    reject_unknown_root_fields(
        input,
        &[
            "index",
            "actor_seat",
            "action_path",
            "freshness_token",
            "expect",
            "expected_diagnostic_code",
            "producer",
        ],
    )?;
    let expect = string_field(input, "expect")?;
    if expect != "applied" {
        return Err(format!("unsupported command expectation `{expect}`"));
    }
    let actor_seat = parse_replay_seat(&string_field(input, "actor_seat")?)?;
    let freshness_token = string_field(input, "freshness_token")?
        .parse::<u64>()
        .map_err(|_| "freshness_token must be a u64 string".to_owned())?;
    let action_path = string_array_field(input, "action_path")?;
    if action_path.len() != 1 {
        return Err("race_to_n replay commands must have one action path segment".to_owned());
    }
    Ok(AppliedCommand {
        actor_seat,
        action_path,
        freshness_token,
    })
}

fn validate_json_object(input: &str) -> Result<(), String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err("malformed JSON object".to_owned());
    }
    let mut depth = 0_i32;
    let mut in_string = false;
    let mut escaped = false;
    for ch in trimmed.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' | '[' => depth += 1,
            '}' | ']' => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return Err("malformed JSON nesting".to_owned());
        }
    }
    if depth != 0 || in_string {
        return Err("malformed JSON nesting".to_owned());
    }
    Ok(())
}

fn reject_unknown_root_fields(input: &str, allowed: &[&str]) -> Result<(), String> {
    for key in top_level_keys(input)? {
        if !allowed.contains(&key.as_str()) {
            return Err(format!("unknown field `{key}`"));
        }
    }
    Ok(())
}

fn top_level_keys(input: &str) -> Result<Vec<String>, String> {
    let body = input
        .trim()
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .ok_or_else(|| "malformed JSON object".to_owned())?;
    let mut keys = Vec::new();
    let mut index = 0;
    while index < body.len() {
        let rest = body[index..].trim_start();
        if rest.is_empty() {
            break;
        }
        let skipped = body[index..].len() - rest.len();
        index += skipped;
        if body[index..].starts_with(',') {
            index += 1;
            continue;
        }
        let (key, next) = parse_json_string_at(body, index)?;
        index = next;
        let after_key = body[index..].trim_start();
        if !after_key.starts_with(':') {
            return Err("malformed JSON field".to_owned());
        }
        index += body[index..].len() - after_key.len() + 1;
        index = skip_json_value(body, index)?;
        keys.push(key);
    }
    Ok(keys)
}

fn skip_json_value(input: &str, mut index: usize) -> Result<usize, String> {
    while input[index..].starts_with(char::is_whitespace) {
        index += input[index..].chars().next().unwrap().len_utf8();
    }
    let mut in_string = false;
    let mut escaped = false;
    let mut depth = 0_i32;
    for (offset, ch) in input[index..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' | '[' => depth += 1,
            '}' | ']' => {
                if depth == 0 {
                    return Ok(index + offset);
                }
                depth -= 1;
            }
            ',' if depth == 0 => return Ok(index + offset + 1),
            _ => {}
        }
    }
    Ok(input.len())
}

fn string_field(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    parse_string_at(input, start).ok_or_else(|| format!("field `{key}` must be a string"))
}

fn number_field(input: &str, key: &str) -> Result<u64, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    parse_number_at(input, start).ok_or_else(|| format!("field `{key}` must be a number"))
}

fn string_array_field(input: &str, key: &str) -> Result<Vec<String>, String> {
    array_items(input, key)?
        .into_iter()
        .map(|item| parse_json_string(item.trim()))
        .collect()
}

fn array_items(input: &str, key: &str) -> Result<Vec<String>, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    let open = input[start..]
        .find('[')
        .ok_or_else(|| format!("field `{key}` must be an array"))?
        + start;
    let close = matching_bracket(input, open, '[', ']')?;
    let body = &input[open + 1..close];
    if body.trim().is_empty() {
        return Ok(Vec::new());
    }
    split_top_level(body, ',')
}

fn matching_bracket(
    input: &str,
    open: usize,
    open_ch: char,
    close_ch: char,
) -> Result<usize, String> {
    let mut depth = 0_u32;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, ch) in input[open..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        if ch == '"' {
            in_string = true;
        } else if ch == open_ch {
            depth += 1;
        } else if ch == close_ch {
            depth = depth
                .checked_sub(1)
                .ok_or_else(|| "unbalanced JSON".to_owned())?;
            if depth == 0 {
                return Ok(open + offset);
            }
        }
    }
    Err("unbalanced JSON".to_owned())
}

fn split_top_level(input: &str, delimiter: char) -> Result<Vec<String>, String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut nested = 0usize;
    for (index, ch) in input.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            '{' | '[' if !in_string => nested += 1,
            '}' | ']' if !in_string => {
                nested = nested
                    .checked_sub(1)
                    .ok_or_else(|| "unbalanced JSON".to_owned())?;
            }
            ch if ch == delimiter && !in_string && nested == 0 => {
                parts.push(input[start..index].to_owned());
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    if in_string || nested != 0 {
        return Err("unbalanced JSON".to_owned());
    }
    parts.push(input[start..].to_owned());
    Ok(parts)
}

fn parse_string_at(input: &str, start: usize) -> Option<String> {
    let tail = input[start..].trim_start();
    parse_json_string_prefix(tail).map(|(value, _)| value).ok()
}

fn parse_json_string_at(input: &str, start: usize) -> Result<(String, usize), String> {
    let (value, consumed) = parse_json_string_prefix(&input[start..])?;
    Ok((value, start + consumed))
}

fn parse_json_string(input: &str) -> Result<String, String> {
    let body = input
        .trim()
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| "expected JSON string".to_owned())?;
    let mut output = String::new();
    let mut escaped = false;
    for ch in body.chars() {
        if escaped {
            output.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else {
            output.push(ch);
        }
    }
    if escaped {
        return Err("unterminated escape".to_owned());
    }
    Ok(output)
}

fn parse_json_string_prefix(input: &str) -> Result<(String, usize), String> {
    let tail = input
        .strip_prefix('"')
        .ok_or_else(|| "expected JSON string".to_owned())?;
    let mut output = String::new();
    let mut escaped = false;
    for (index, ch) in tail.char_indices() {
        if escaped {
            output.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            return Ok((output, index + 2));
        } else {
            output.push(ch);
        }
    }
    Err("unterminated JSON string".to_owned())
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

#[no_mangle]
pub extern "C" fn rulepath_placeholder_version_ptr() -> *const u8 {
    API_VERSION.as_ptr()
}

#[no_mangle]
pub extern "C" fn rulepath_placeholder_version_len() -> usize {
    API_VERSION.len()
}

#[no_mangle]
pub extern "C" fn rulepath_feature_report() -> i32 {
    write_result(feature_report())
}

#[no_mangle]
pub extern "C" fn rulepath_list_games() -> i32 {
    write_result(list_games())
}

#[no_mangle]
pub extern "C" fn rulepath_alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}

#[no_mangle]
/// # Safety
///
/// `ptr` must have been returned by `rulepath_alloc` with the same `len`, and it
/// must not be used after this call.
pub unsafe extern "C" fn rulepath_dealloc(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        drop(unsafe { Vec::from_raw_parts(ptr, 0, len) });
    }
}

#[no_mangle]
pub extern "C" fn rulepath_last_output_ptr() -> *const u8 {
    LAST_OUTPUT.with(|output| output.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn rulepath_last_output_len() -> usize {
    LAST_OUTPUT.with(|output| output.borrow().len())
}

#[no_mangle]
/// # Safety
///
/// `game_ptr..game_ptr + game_len` must be a valid UTF-8 buffer for the duration
/// of the call.
pub unsafe extern "C" fn rulepath_new_match(
    game_ptr: *const u8,
    game_len: usize,
    seed: u64,
) -> i32 {
    let game_id = unsafe { read_string(game_ptr, game_len) };
    write_result(game_id.and_then(|game_id| new_match(&game_id, seed)))
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` must be a valid UTF-8 buffer for the
/// duration of the call.
pub unsafe extern "C" fn rulepath_get_view(match_ptr: *const u8, match_len: usize) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    write_result(match_id.and_then(|match_id| get_view(&match_id, None)))
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` and `seat_ptr..seat_ptr + seat_len` must
/// be valid UTF-8 buffers for the duration of the call.
pub unsafe extern "C" fn rulepath_get_action_tree(
    match_ptr: *const u8,
    match_len: usize,
    seat_ptr: *const u8,
    seat_len: usize,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let seat = unsafe { read_string(seat_ptr, seat_len) };
    write_result(
        match_id.and_then(|match_id| seat.and_then(|seat| get_action_tree(&match_id, &seat))),
    )
}

#[no_mangle]
/// # Safety
///
/// `match_ptr`, `seat_ptr`, and `path_ptr` with their lengths must be valid
/// UTF-8 buffers for the duration of the call.
pub unsafe extern "C" fn rulepath_apply_action(
    match_ptr: *const u8,
    match_len: usize,
    seat_ptr: *const u8,
    seat_len: usize,
    path_ptr: *const u8,
    path_len: usize,
    freshness_token: u64,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let seat = unsafe { read_string(seat_ptr, seat_len) };
    let path = unsafe { read_string(path_ptr, path_len) };
    write_result(match_id.and_then(|match_id| {
        seat.and_then(|seat| {
            path.and_then(|path| apply_action(&match_id, &seat, &path, freshness_token))
        })
    }))
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` and `seat_ptr..seat_ptr + seat_len` must
/// be valid UTF-8 buffers for the duration of the call.
pub unsafe extern "C" fn rulepath_run_bot_turn(
    match_ptr: *const u8,
    match_len: usize,
    seat_ptr: *const u8,
    seat_len: usize,
    bot_seed: u64,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let seat = unsafe { read_string(seat_ptr, seat_len) };
    write_result(
        match_id
            .and_then(|match_id| seat.and_then(|seat| run_bot_turn(&match_id, &seat, bot_seed))),
    )
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` must be valid UTF-8. If `viewer_len` is
/// nonzero, `viewer_ptr..viewer_ptr + viewer_len` must also be valid UTF-8.
pub unsafe extern "C" fn rulepath_get_effects(
    match_ptr: *const u8,
    match_len: usize,
    since_cursor: u64,
    viewer_ptr: *const u8,
    viewer_len: usize,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let viewer = if viewer_len == 0 {
        Ok(None)
    } else {
        unsafe { read_string(viewer_ptr, viewer_len) }.map(Some)
    };
    write_result(match_id.and_then(|match_id| {
        viewer.and_then(|viewer| get_effects(&match_id, since_cursor, viewer.as_deref()))
    }))
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` must be a valid UTF-8 buffer for the
/// duration of the call.
pub unsafe extern "C" fn rulepath_export_replay(match_ptr: *const u8, match_len: usize) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    write_result(match_id.and_then(|match_id| export_replay(&match_id)))
}

#[no_mangle]
/// # Safety
///
/// `doc_ptr..doc_ptr + doc_len` must be a valid UTF-8 buffer for the duration of
/// the call.
pub unsafe extern "C" fn rulepath_import_replay(doc_ptr: *const u8, doc_len: usize) -> i32 {
    let doc = unsafe { read_string(doc_ptr, doc_len) };
    write_result(doc.and_then(|doc| import_replay(&doc)))
}

#[no_mangle]
/// # Safety
///
/// `replay_ptr..replay_ptr + replay_len` must be a valid UTF-8 buffer for the
/// duration of the call.
pub unsafe extern "C" fn rulepath_replay_step(
    replay_ptr: *const u8,
    replay_len: usize,
    cursor: usize,
) -> i32 {
    let replay_id = unsafe { read_string(replay_ptr, replay_len) };
    write_result(replay_id.and_then(|replay_id| replay_step(&replay_id, cursor)))
}

#[no_mangle]
/// # Safety
///
/// `replay_ptr..replay_ptr + replay_len` must be a valid UTF-8 buffer for the
/// duration of the call.
pub unsafe extern "C" fn rulepath_replay_reset(replay_ptr: *const u8, replay_len: usize) -> i32 {
    let replay_id = unsafe { read_string(replay_ptr, replay_len) };
    write_result(replay_id.and_then(|replay_id| replay_reset(&replay_id)))
}

unsafe fn read_string(ptr: *const u8, len: usize) -> Result<String, String> {
    if ptr.is_null() && len > 0 {
        return Err(
            "{\"code\":\"invalid_pointer\",\"message\":\"input pointer is null\"}".to_owned(),
        );
    }
    let bytes = unsafe { slice::from_raw_parts(ptr, len) };
    str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| "{\"code\":\"invalid_utf8\",\"message\":\"input is not utf-8\"}".to_owned())
}

fn write_result(result: Result<String, String>) -> i32 {
    match result {
        Ok(output) => {
            write_output(output);
            0
        }
        Err(error) => {
            write_output(error);
            1
        }
    }
}

fn write_output(output: String) {
    LAST_OUTPUT.with(|last| {
        *last.borrow_mut() = output;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_version_is_stable() {
        assert_eq!(placeholder_version(), "rulepath-wasm-api/0.1.0");
    }

    #[test]
    fn list_games_reports_race_to_n() {
        let games = list_games().expect("games listed");
        assert_eq!(
            games,
            "[{\"game_id\":\"race_to_n\",\"display_name\":\"Race to 21\",\"rules_version\":1,\"schema_version\":1}]"
        );
    }

    #[test]
    fn feature_report_lists_ops() {
        let report = feature_report().expect("feature report returned");
        assert!(report.contains("\"api_version\":\"rulepath-wasm-api/0.1.0\""));
        for operation in SUPPORTED_OPERATIONS {
            assert!(
                report.contains(&format!("\"{operation}\"")),
                "missing operation {operation} in {report}"
            );
        }
        assert!(report.contains(
            "\"features\":[\"catalog\",\"match_store\",\"legal_action_tree\",\"effects\"]"
        ));
    }

    #[test]
    fn new_ops_use_status_output_convention() {
        assert_eq!(rulepath_feature_report(), 0);
        assert!(last_output_string().contains("\"api_version\":\"rulepath-wasm-api/0.1.0\""));

        assert_eq!(rulepath_list_games(), 0);
        assert!(last_output_string().contains("\"game_id\":\"race_to_n\""));
    }

    #[test]
    fn surface_drives_minimal_turn_loop() {
        let created = new_match("race_to_n", 11).expect("match created");
        let match_id = extract_match_id(&created);

        let view = get_view(&match_id, None).expect("view returned");
        assert!(view.contains("\"counter\":0"));
        assert!(!view.contains("seat-0"));

        let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
        assert!(tree.contains("\"segment\":\"add-1\""));
        assert!(tree.contains("\"freshness_token\":0"));

        let applied = apply_action(&match_id, "seat_0", "add-1", 0).expect("human action applies");
        assert!(applied.contains("\"counter\":1"));
        assert!(applied.contains("\"type\":\"counter_advanced\""));

        let effects = get_effects(&match_id, 0, None).expect("effects returned");
        assert!(effects.contains("\"cursor\":1"));
        assert!(effects.contains("\"visibility\":\"public\""));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        assert!(bot.contains("\"ok\":true"));
        assert!(bot.contains("\"active_seat\":\"seat_0\""));
    }

    #[test]
    fn stale_action_returns_diagnostic_without_mutation() {
        let created = new_match("race_to_n", 12).expect("match created");
        let match_id = extract_match_id(&created);

        apply_action(&match_id, "seat_0", "add-1", 0).expect("first action applies");
        let stale =
            apply_action(&match_id, "seat_1", "add-1", 0).expect_err("stale token rejected");
        assert!(stale.contains("\"code\":\"stale_action\""));

        let view = get_view(&match_id, None).expect("view returned");
        assert!(view.contains("\"counter\":1"));
        assert!(view.contains("\"freshness_token\":1"));
    }

    #[test]
    fn replay_round_trip_reproduces_hashes() {
        let created = new_match("race_to_n", 21).expect("match created");
        let match_id = extract_match_id(&created);
        apply_action(&match_id, "seat_0", "add-1", 0).expect("first action applies");
        apply_action(&match_id, "seat_1", "add-2", 1).expect("second action applies");

        let exported = export_replay(&match_id).expect("replay exported");
        let expected = replay_commands(21, &["add-1".to_owned(), "add-2".to_owned()]);
        assert!(exported.contains(&format!(
            "\"expected_state_hashes\":{{\"final\":{}}}",
            expected.state_hash.0
        )));
        assert!(exported.contains(&format!(
            "\"expected_effect_hashes\":{{\"final\":{}}}",
            expected.effect_hash.0
        )));

        let imported = import_replay(&exported).expect("replay imported");
        let replay_id = extract_replay_id(&imported);
        let stepped = replay_step(&replay_id, 2).expect("replay stepped");
        assert!(stepped.contains("\"cursor\":2"));
        assert!(stepped.contains("\"counter\":3"));
        assert!(stepped.contains("\"done\":true"));
    }

    #[test]
    fn import_rejects_wrong_game_version_malformed_and_oversized() {
        let created = new_match("race_to_n", 22).expect("match created");
        let match_id = extract_match_id(&created);
        apply_action(&match_id, "seat_0", "add-1", 0).expect("action applies");
        let exported = export_replay(&match_id).expect("replay exported");
        let matches_before = match_count();
        let replays_before = replay_count();

        let wrong_game = exported.replace("\"game_id\":\"race_to_n\"", "\"game_id\":\"wrong\"");
        assert!(import_replay(&wrong_game)
            .expect_err("wrong game rejected")
            .contains("\"code\":\"unsupported_replay_game\""));

        let wrong_rules = exported.replace(
            "\"rules_version\":\"race_to_n-rules-v1\"",
            "\"rules_version\":\"race_to_n-rules-v99\"",
        );
        assert!(import_replay(&wrong_rules)
            .expect_err("wrong rules rejected")
            .contains("\"code\":\"unsupported_replay_rules\""));

        assert!(import_replay("{ nope")
            .expect_err("malformed replay rejected")
            .contains("\"code\":\"invalid_replay\""));

        let oversized = "x".repeat(MAX_REPLAY_IMPORT_BYTES + 1);
        assert!(import_replay(&oversized)
            .expect_err("oversized replay rejected")
            .contains("\"code\":\"replay_too_large\""));

        assert_eq!(match_count(), matches_before);
        assert_eq!(replay_count(), replays_before);
    }

    #[test]
    fn replay_step_and_reset_match_rust_replay() {
        let created = new_match("race_to_n", 23).expect("match created");
        let match_id = extract_match_id(&created);
        apply_action(&match_id, "seat_0", "add-3", 0).expect("action applies");
        let live_view = get_view(&match_id, None).expect("live view returned");
        let exported = export_replay(&match_id).expect("replay exported");
        let imported = import_replay(&exported).expect("replay imported");
        let replay_id = extract_replay_id(&imported);

        let step = replay_step(&replay_id, 1).expect("replay step returned");
        assert!(step.contains("\"cursor\":1"));
        assert!(step.contains("\"counter\":3"));
        assert!(step.contains("\"type\":\"counter_advanced\""));
        assert!(step.contains(&format!("\"view\":{live_view}")));

        let reset = replay_reset(&replay_id).expect("replay reset returned");
        assert!(reset.contains("\"cursor\":0"));
        assert!(reset.contains("\"counter\":0"));
        assert!(reset.contains("\"effects\":[]"));
    }

    #[test]
    fn replay_export_omits_internal_state_surfaces() {
        let created = new_match("race_to_n", 24).expect("match created");
        let match_id = extract_match_id(&created);
        apply_action(&match_id, "seat_0", "add-1", 0).expect("action applies");
        let exported = export_replay(&match_id).expect("replay exported");

        assert!(exported.contains("\"commands\":["));
        assert!(!exported.contains("initial_snapshot"));
        assert!(!exported.contains("legal_additions"));
        assert!(!exported.contains("\"state\":"));
        assert!(!exported.contains("\"effects\":["));
    }

    fn extract_match_id(created: &str) -> String {
        created
            .split("\"match_id\":\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .expect("match id is present")
            .to_owned()
    }

    fn extract_replay_id(created: &str) -> String {
        created
            .split("\"replay_id\":\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .expect("replay id is present")
            .to_owned()
    }

    fn match_count() -> usize {
        MATCHES.with(|matches| matches.borrow().len())
    }

    fn replay_count() -> usize {
        REPLAYS.with(|replays| replays.borrow().len())
    }

    fn last_output_string() -> String {
        LAST_OUTPUT.with(|last| last.borrow().clone())
    }
}
