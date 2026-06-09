use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use directional_flip::replay_support::{
    replay_commands as directional_replay_commands,
    replay_from_state as directional_replay_from_state,
};
use race_to_n::replay_support::{
    replay_bot_action, replay_commands, replay_invalid, ReplayHashes as RaceReplayHashes,
};

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let config = Config::parse(args)?;
    if config.game != "race_to_n" && config.game != "directional_flip" {
        return Err(format!("unsupported game `{}`", config.game));
    }
    let input = fs::read_to_string(&config.trace)
        .map_err(|error| format!("{}: failed to read: {error}", config.trace.display()))?;
    let trace = Trace::parse(&config.trace, &input)?;
    if trace.game_id != config.game {
        return Err(format!(
            "--game `{}` does not match trace game_id `{}`",
            config.game, trace.game_id
        ));
    }
    println!("{}", trace.render()?);
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    game: String,
    trace: PathBuf,
}

impl Config {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        if args
            .iter()
            .any(|arg| matches!(arg.as_str(), "--help" | "-h"))
        {
            print_help();
            process::exit(0);
        }

        let mut game = None;
        let mut trace = None;
        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--game" => game = Some(next_arg(&mut iter, "--game")?),
                "--trace" => trace = Some(PathBuf::from(next_arg(&mut iter, "--trace")?)),
                other => return Err(format!("unknown argument `{other}`")),
            }
        }

        Ok(Self {
            game: game.ok_or_else(|| "--game is required".to_owned())?,
            trace: trace.ok_or_else(|| "--trace is required".to_owned())?,
        })
    }
}

fn next_arg(iter: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    iter.next()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn print_help() {
    println!("trace-viewer 0.1.0");
    println!("usage:");
    println!("  trace-viewer --game <race_to_n|directional_flip> --trace <path>");
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Trace {
    path: PathBuf,
    schema_version: u64,
    trace_id: String,
    fixture_kind: String,
    purpose: String,
    note: String,
    migration_update_note: String,
    game_id: String,
    rules_version: String,
    engine_version: String,
    data_version: String,
    seed: u64,
    variant: String,
    seats: Vec<SeatSummary>,
    commands: Vec<CommandSummary>,
    checkpoints: Vec<CheckpointSummary>,
    expected_state_hashes: Vec<(String, String)>,
    expected_effect_hashes: Vec<(String, String)>,
    expected_action_tree_hashes: Vec<(String, String)>,
    expected_public_view_hashes: Vec<(String, String)>,
    expected_private_view_hashes: Vec<(String, String)>,
    expected_diagnostics: Vec<DiagnosticSummary>,
    expected_terminal: Option<bool>,
    expected_winner: Option<String>,
    not_applicable: Vec<(String, String)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SeatSummary {
    seat_id: String,
    player_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CommandSummary {
    index: u64,
    actor_seat: String,
    action_path: Vec<String>,
    freshness_token: String,
    expect: String,
    expected_diagnostic_code: Option<String>,
    bot_policy: Option<String>,
    bot_policy_version: Option<String>,
    bot_seed: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CheckpointSummary {
    id: String,
    after_command_index: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DiagnosticSummary {
    command_index: u64,
    code: String,
    hash: u64,
}

impl Trace {
    fn parse(path: &Path, input: &str) -> Result<Self, String> {
        validate_json_object(path, input)?;
        let schema_version =
            number_field(input, "schema_version").map_err(|error| parse_error(path, &error))?;
        if schema_version != 1 {
            return Err(parse_error(
                path,
                &format!("unsupported schema_version `{schema_version}`"),
            ));
        }
        let trace = Self {
            path: path.to_path_buf(),
            schema_version,
            trace_id: required_string(path, input, "trace_id")?,
            fixture_kind: required_string(path, input, "fixture_kind")?,
            purpose: required_string(path, input, "purpose")?,
            note: required_string(path, input, "note")?,
            migration_update_note: required_string(path, input, "migration_update_note")?,
            game_id: required_string(path, input, "game_id")?,
            rules_version: required_string(path, input, "rules_version")?,
            engine_version: required_string(path, input, "engine_version")?,
            data_version: required_string(path, input, "data_version")?,
            seed: number_field(input, "seed").map_err(|error| parse_error(path, &error))?,
            variant: required_string(path, input, "variant")?,
            seats: parse_seats(path, input)?,
            commands: parse_commands(path, input)?,
            checkpoints: parse_checkpoints(path, input)?,
            expected_state_hashes: value_pairs(path, input, "expected_state_hashes")?,
            expected_effect_hashes: value_pairs(path, input, "expected_effect_hashes")?,
            expected_action_tree_hashes: value_pairs(path, input, "expected_action_tree_hashes")?,
            expected_public_view_hashes: value_pairs(path, input, "expected_public_view_hashes")?,
            expected_private_view_hashes: string_pairs(
                path,
                input,
                "expected_private_view_hashes",
            )?,
            expected_diagnostics: parse_diagnostics(path, input)?,
            expected_terminal: bool_in_object(path, input, "expected_terminal_state", "terminal")?,
            expected_winner: string_or_null_in_object(input, "expected_terminal_state", "winner")?,
            not_applicable: string_pairs_from_body(
                path,
                "not_applicable",
                &last_object_body(input, "not_applicable")?,
            )?,
        };
        trace.validate()?;
        Ok(trace)
    }

    fn validate(&self) -> Result<(), String> {
        if self.game_id != "race_to_n" && self.game_id != "directional_flip" {
            return Err(self.failure(&format!("unsupported trace game_id `{}`", self.game_id)));
        }
        let expected_rules = match self.game_id.as_str() {
            "race_to_n" => "race_to_n-rules-v1",
            "directional_flip" => "directional_flip-rules-v1",
            _ => unreachable!("validated game"),
        };
        if self.rules_version != expected_rules {
            return Err(self.failure(&format!(
                "unsupported rules_version `{}`",
                self.rules_version
            )));
        }
        for (name, value) in [
            ("purpose", &self.purpose),
            ("note", &self.note),
            ("migration_update_note", &self.migration_update_note),
        ] {
            if value.trim().is_empty() {
                return Err(self.failure(&format!("{name} must be non-empty")));
            }
        }
        if self.fixture_kind != "not_applicable" {
            for (name, values) in [
                ("expected_state_hashes", &self.expected_state_hashes),
                ("expected_effect_hashes", &self.expected_effect_hashes),
                (
                    "expected_action_tree_hashes",
                    &self.expected_action_tree_hashes,
                ),
                (
                    "expected_public_view_hashes",
                    &self.expected_public_view_hashes,
                ),
            ] {
                if values.is_empty() {
                    return Err(self.failure(&format!("{name} must be non-empty")));
                }
            }
        }
        Ok(())
    }

    fn render(&self) -> Result<String, String> {
        let mut lines = Vec::new();
        lines.push(format!("Trace: {}", self.trace_id));
        lines.push(format!("Path: {}", self.path.display()));
        lines.push(String::new());
        lines.push("Metadata".to_owned());
        lines.push(format!("  schema_version: {}", self.schema_version));
        lines.push(format!("  game_id: {}", self.game_id));
        lines.push(format!("  rules_version: {}", self.rules_version));
        lines.push(format!("  engine_version: {}", self.engine_version));
        lines.push(format!("  data_version: {}", self.data_version));
        lines.push(format!("  seed: {}", self.seed));
        lines.push(format!("  variant: {}", self.variant));
        lines.push(String::new());
        lines.push("Fixture".to_owned());
        lines.push(format!("  kind: {}", self.fixture_kind));
        lines.push(format!("  purpose: {}", self.purpose));
        lines.push(format!("  note: {}", self.note));
        lines.push(format!(
            "  migration_update_note: {}",
            self.migration_update_note
        ));
        lines.push(String::new());
        lines.push("Seats".to_owned());
        for seat in &self.seats {
            lines.push(format!("  {} -> {}", seat.seat_id, seat.player_id));
        }
        lines.push(String::new());
        lines.push("Commands".to_owned());
        if self.commands.is_empty() {
            lines.push("  none".to_owned());
        }
        for command in &self.commands {
            let path = command.action_path.join("/");
            let mut line = format!(
                "  #{} actor={} path={} freshness={} expect={}",
                command.index, command.actor_seat, path, command.freshness_token, command.expect
            );
            if let Some(code) = &command.expected_diagnostic_code {
                line.push_str(&format!(" diagnostic={code}"));
            }
            if let Some(seed) = command.bot_seed {
                line.push_str(&format!(
                    " producer={}:{} seed={seed}",
                    command.bot_policy.as_deref().unwrap_or("unknown"),
                    command.bot_policy_version.as_deref().unwrap_or("unknown")
                ));
            }
            lines.push(line);
        }
        lines.push(String::new());
        lines.push("Checkpoints".to_owned());
        for checkpoint in &self.checkpoints {
            lines.push(format!(
                "  {} after_command_index={}",
                checkpoint.id,
                checkpoint
                    .after_command_index
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "unspecified".to_owned())
            ));
        }
        lines.push(String::new());
        lines.push("Expected Hashes".to_owned());
        push_number_pairs(&mut lines, "state", &self.expected_state_hashes);
        push_number_pairs(&mut lines, "effect", &self.expected_effect_hashes);
        push_number_pairs(&mut lines, "action_tree", &self.expected_action_tree_hashes);
        push_number_pairs(&mut lines, "public_view", &self.expected_public_view_hashes);
        push_string_pairs(
            &mut lines,
            "private_view",
            &self.expected_private_view_hashes,
        );
        lines.push(String::new());
        lines.push("Diagnostics".to_owned());
        if self.expected_diagnostics.is_empty() {
            lines.push("  none".to_owned());
        }
        for diagnostic in &self.expected_diagnostics {
            lines.push(format!(
                "  command_index={} code={} hash={}",
                diagnostic.command_index, diagnostic.code, diagnostic.hash
            ));
        }
        lines.push(String::new());
        lines.push("Not Applicable".to_owned());
        push_string_pairs(&mut lines, "reason", &self.not_applicable);
        lines.push(String::new());
        lines.push("Expected Outcome".to_owned());
        lines.push(format!(
            "  terminal: {}",
            self.expected_terminal
                .map(|value| value.to_string())
                .unwrap_or_else(|| "unknown".to_owned())
        ));
        lines.push(format!(
            "  winner: {}",
            self.expected_winner.as_deref().unwrap_or("none")
        ));
        lines.push(String::new());
        lines.push("Actual Replay Annotation".to_owned());
        match self.actual_hashes()? {
            Some(actual) => push_actual_hashes(&mut lines, &actual),
            None => lines.push("  not-applicable trace; no replay annotation".to_owned()),
        }
        Ok(lines.join("\n"))
    }

    fn actual_hashes(&self) -> Result<Option<ActualReplay>, String> {
        let command_segments = self
            .commands
            .iter()
            .filter_map(|command| command.action_path.first().cloned())
            .collect::<Vec<_>>();
        if self.game_id == "directional_flip" {
            return match self.fixture_kind.as_str() {
                "commands" | "terminal" | "bot" | "diagnostic" => {
                    Ok(Some(ActualReplay::from_directional(
                        self.directional_replay_hashes(if self.fixture_kind == "diagnostic" {
                            &command_segments[..command_segments.len().saturating_sub(1)]
                        } else {
                            &command_segments
                        }),
                    )))
                }
                "not_applicable" => Ok(None),
                other => Err(self.failure(&format!("unsupported fixture_kind `{other}`"))),
            };
        }
        match self.fixture_kind.as_str() {
            "commands" | "terminal" => Ok(Some(ActualReplay::from_race(replay_commands(
                self.seed,
                &command_segments,
            )))),
            "bot" => Ok(Some(ActualReplay::from_race(replay_bot_action(
                self.seed,
                self.commands
                    .iter()
                    .find_map(|command| command.bot_seed)
                    .ok_or_else(|| self.failure("bot trace missing producer bot_seed"))?,
            )))),
            "invalid" | "diagnostic" => Ok(Some(ActualReplay::from_race(replay_invalid(
                self.seed,
                self.command_for_diagnostic("invalid_action")?.as_str(),
                self.command_for_diagnostic("stale_action")?.as_str(),
            )))),
            "not_applicable" => Ok(None),
            other => Err(self.failure(&format!("unsupported fixture_kind `{other}`"))),
        }
    }

    fn directional_replay_hashes(&self, commands: &[String]) -> directional_flip::ReplayHashes {
        match self.trace_id.as_str() {
            "directional-flip-multi-direction-flip" => {
                directional_replay_custom(self.seed, multi_direction_state(), commands)
            }
            "directional-flip-corner-capture" => {
                directional_replay_custom(self.seed, corner_state(), commands)
            }
            "directional-flip-forced-pass"
            | "directional-flip-double-pass-terminal"
            | "directional-flip-draw" => {
                directional_replay_custom(self.seed, no_move_state(), commands)
            }
            "directional-flip-full-board-terminal" => {
                directional_replay_custom(self.seed, full_board_terminal_state(), commands)
            }
            _ => directional_replay_commands(self.seed, commands),
        }
    }

    fn command_for_diagnostic(&self, code: &str) -> Result<String, String> {
        self.commands
            .iter()
            .find(|command| command.expected_diagnostic_code.as_deref() == Some(code))
            .and_then(|command| command.action_path.first().cloned())
            .ok_or_else(|| self.failure(&format!("diagnostic trace missing {code} command")))
    }

    fn failure(&self, reason: &str) -> String {
        format!(
            "trace-viewer failure\ntrace path: {}\ntrace ID: {}\ngame ID: {}\nschema version: {}\nrules version: {}\nreason: {reason}",
            self.path.display(),
            self.trace_id,
            self.game_id,
            self.schema_version,
            self.rules_version
        )
    }
}

fn push_number_pairs(lines: &mut Vec<String>, label: &str, pairs: &[(String, String)]) {
    if pairs.is_empty() {
        lines.push(format!("  {label}: none"));
        return;
    }
    for (key, value) in pairs {
        lines.push(format!("  {label}.{key}: {value}"));
    }
}

fn push_string_pairs(lines: &mut Vec<String>, label: &str, pairs: &[(String, String)]) {
    if pairs.is_empty() {
        lines.push(format!("  {label}: none"));
        return;
    }
    for (key, value) in pairs {
        lines.push(format!("  {label}.{key}: {value}"));
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ActualReplay {
    state_hash: u64,
    effect_hash: u64,
    action_tree_hash: u64,
    view_hash: u64,
    diagnostic_hash: Option<u64>,
    winner: Option<String>,
}

impl ActualReplay {
    fn from_race(actual: RaceReplayHashes) -> Self {
        Self {
            state_hash: actual.state_hash.0,
            effect_hash: actual.effect_hash.0,
            action_tree_hash: actual.action_tree_hash.0,
            view_hash: actual.view_hash.0,
            diagnostic_hash: actual.diagnostic_hash.map(|hash| hash.0),
            winner: actual.outcome.map(|winner| winner.as_str().to_owned()),
        }
    }

    fn from_directional(actual: directional_flip::ReplayHashes) -> Self {
        Self {
            state_hash: actual.state_hash.0,
            effect_hash: actual.effect_hash.0,
            action_tree_hash: actual.action_tree_hash.0,
            view_hash: actual.view_hash.0,
            diagnostic_hash: None,
            winner: actual.outcome.and_then(|outcome| match outcome {
                directional_flip::TerminalOutcome::Win { seat } => Some(seat.as_str().to_owned()),
                directional_flip::TerminalOutcome::Draw => None,
            }),
        }
    }
}

fn push_actual_hashes(lines: &mut Vec<String>, actual: &ActualReplay) {
    lines.push(format!("  state.final: {}", actual.state_hash));
    lines.push(format!("  effect.final: {}", actual.effect_hash));
    lines.push(format!("  action_tree.final: {}", actual.action_tree_hash));
    lines.push(format!("  public_view.all: {}", actual.view_hash));
    if let Some(hash) = actual.diagnostic_hash {
        lines.push(format!("  diagnostic: {hash}"));
    }
    lines.push(format!(
        "  outcome.winner: {}",
        actual.winner.as_deref().unwrap_or("none")
    ));
}

fn directional_replay_custom(
    seed: u64,
    mut state: directional_flip::DirectionalFlipState,
    commands: &[String],
) -> directional_flip::ReplayHashes {
    let initial_snapshot =
        directional_flip::DirectionalFlipSnapshot::from_state(&state).stable_summary();
    directional_replay_from_state(seed, initial_snapshot, commands, &mut state)
}

fn directional_cell(
    row: directional_flip::RowId,
    column: directional_flip::ColumnId,
) -> directional_flip::CellId {
    directional_flip::CellId::new(row, column)
}

fn directional_occupy(
    state: &mut directional_flip::DirectionalFlipState,
    cell: directional_flip::CellId,
    seat: directional_flip::DirectionalFlipSeat,
) {
    state.set_occupancy(cell, directional_flip::CellOccupancy::Occupied(seat));
}

fn directional_empty(
    active: directional_flip::DirectionalFlipSeat,
) -> directional_flip::DirectionalFlipState {
    let seats = vec![
        engine_core::SeatId("seat-0".to_owned()),
        engine_core::SeatId("seat-1".to_owned()),
    ];
    let mut state = directional_flip::setup_match(
        engine_core::Seed(1),
        &seats,
        &directional_flip::SetupOptions::default(),
    )
    .expect("directional fixture setup succeeds");
    state.cells = directional_flip::DirectionalFlipState::empty_cells();
    state.active_seat = active;
    state.ply_count = 0;
    state.consecutive_forced_passes = 0;
    state.terminal_outcome = None;
    state
}

fn no_move_state() -> directional_flip::DirectionalFlipState {
    let mut state = directional_empty(directional_flip::DirectionalFlipSeat::Seat0);
    directional_occupy(
        &mut state,
        directional_cell(directional_flip::RowId::R1, directional_flip::ColumnId::C1),
        directional_flip::DirectionalFlipSeat::Seat0,
    );
    directional_occupy(
        &mut state,
        directional_cell(directional_flip::RowId::R8, directional_flip::ColumnId::C8),
        directional_flip::DirectionalFlipSeat::Seat1,
    );
    state
}

fn corner_state() -> directional_flip::DirectionalFlipState {
    let mut state = directional_empty(directional_flip::DirectionalFlipSeat::Seat0);
    directional_occupy(
        &mut state,
        directional_cell(directional_flip::RowId::R1, directional_flip::ColumnId::C2),
        directional_flip::DirectionalFlipSeat::Seat1,
    );
    directional_occupy(
        &mut state,
        directional_cell(directional_flip::RowId::R1, directional_flip::ColumnId::C3),
        directional_flip::DirectionalFlipSeat::Seat0,
    );
    state
}

fn full_board_terminal_state() -> directional_flip::DirectionalFlipState {
    let mut state = directional_empty(directional_flip::DirectionalFlipSeat::Seat0);
    for cell in directional_flip::CellId::ALL {
        directional_occupy(
            &mut state,
            cell,
            directional_flip::DirectionalFlipSeat::Seat0,
        );
    }
    state.set_occupancy(
        directional_cell(directional_flip::RowId::R1, directional_flip::ColumnId::C1),
        directional_flip::CellOccupancy::Empty,
    );
    directional_occupy(
        &mut state,
        directional_cell(directional_flip::RowId::R1, directional_flip::ColumnId::C2),
        directional_flip::DirectionalFlipSeat::Seat1,
    );
    state
}

fn multi_direction_state() -> directional_flip::DirectionalFlipState {
    let mut state = directional_empty(directional_flip::DirectionalFlipSeat::Seat0);
    for cell in [
        directional_cell(directional_flip::RowId::R3, directional_flip::ColumnId::C4),
        directional_cell(directional_flip::RowId::R3, directional_flip::ColumnId::C5),
        directional_cell(directional_flip::RowId::R4, directional_flip::ColumnId::C5),
        directional_cell(directional_flip::RowId::R5, directional_flip::ColumnId::C5),
        directional_cell(directional_flip::RowId::R5, directional_flip::ColumnId::C4),
        directional_cell(directional_flip::RowId::R5, directional_flip::ColumnId::C3),
        directional_cell(directional_flip::RowId::R4, directional_flip::ColumnId::C3),
        directional_cell(directional_flip::RowId::R3, directional_flip::ColumnId::C3),
    ] {
        directional_occupy(
            &mut state,
            cell,
            directional_flip::DirectionalFlipSeat::Seat1,
        );
    }
    for cell in [
        directional_cell(directional_flip::RowId::R2, directional_flip::ColumnId::C4),
        directional_cell(directional_flip::RowId::R2, directional_flip::ColumnId::C6),
        directional_cell(directional_flip::RowId::R4, directional_flip::ColumnId::C6),
        directional_cell(directional_flip::RowId::R6, directional_flip::ColumnId::C6),
        directional_cell(directional_flip::RowId::R6, directional_flip::ColumnId::C4),
        directional_cell(directional_flip::RowId::R6, directional_flip::ColumnId::C2),
        directional_cell(directional_flip::RowId::R4, directional_flip::ColumnId::C2),
        directional_cell(directional_flip::RowId::R2, directional_flip::ColumnId::C2),
    ] {
        directional_occupy(
            &mut state,
            cell,
            directional_flip::DirectionalFlipSeat::Seat0,
        );
    }
    state
}

fn parse_seats(path: &Path, input: &str) -> Result<Vec<SeatSummary>, String> {
    array_objects(path, input, "seats")?
        .into_iter()
        .map(|object| {
            Ok(SeatSummary {
                seat_id: required_string(path, &object, "seat_id")?,
                player_id: required_string(path, &object, "player_id")?,
            })
        })
        .collect()
}

fn parse_commands(path: &Path, input: &str) -> Result<Vec<CommandSummary>, String> {
    if !input.contains("\"commands\":") {
        return Ok(Vec::new());
    }
    array_objects(path, input, "commands")?
        .into_iter()
        .map(|object| {
            let producer = object_body(&object, "producer").ok();
            Ok(CommandSummary {
                index: number_field(&object, "index").map_err(|error| parse_error(path, &error))?,
                actor_seat: required_string(path, &object, "actor_seat")?,
                action_path: string_array_field(path, &object, "action_path")?,
                freshness_token: required_string(path, &object, "freshness_token")?,
                expect: required_string(path, &object, "expect")?,
                expected_diagnostic_code: optional_string_field(
                    &object,
                    "expected_diagnostic_code",
                ),
                bot_policy: producer
                    .as_deref()
                    .and_then(|value| optional_string_field(value, "bot_policy")),
                bot_policy_version: producer
                    .as_deref()
                    .and_then(|value| optional_string_field(value, "bot_policy_version")),
                bot_seed: producer
                    .as_deref()
                    .and_then(|value| number_field(value, "bot_seed").ok()),
            })
        })
        .collect()
}

fn parse_checkpoints(path: &Path, input: &str) -> Result<Vec<CheckpointSummary>, String> {
    array_objects(path, input, "checkpoints")?
        .into_iter()
        .map(|object| {
            Ok(CheckpointSummary {
                id: required_string(path, &object, "id")?,
                after_command_index: number_field(&object, "after_command_index").ok(),
            })
        })
        .collect()
}

fn parse_diagnostics(path: &Path, input: &str) -> Result<Vec<DiagnosticSummary>, String> {
    if !input.contains("\"expected_diagnostics\":") {
        return Ok(Vec::new());
    }
    array_objects(path, input, "expected_diagnostics")?
        .into_iter()
        .map(|object| {
            Ok(DiagnosticSummary {
                command_index: number_field(&object, "command_index")
                    .map_err(|error| parse_error(path, &error))?,
                code: required_string(path, &object, "code")?,
                hash: number_field(&object, "hash").map_err(|error| parse_error(path, &error))?,
            })
        })
        .collect()
}

fn value_pairs(path: &Path, input: &str, key: &str) -> Result<Vec<(String, String)>, String> {
    let body = object_body(input, key).map_err(|error| parse_error(path, &error))?;
    object_pairs(&body)?
        .into_iter()
        .map(|(name, value)| {
            if let Some(number) = parse_number_at(&value, 0) {
                return Ok((name, number.to_string()));
            }
            let string = parse_string_at(&value, 0).ok_or_else(|| {
                parse_error(path, &format!("{key}.{name} must be a number or string"))
            })?;
            Ok((name, string))
        })
        .collect()
}

fn string_pairs(path: &Path, input: &str, key: &str) -> Result<Vec<(String, String)>, String> {
    if !input.contains(&format!("\"{key}\":")) {
        return Ok(Vec::new());
    }
    let body = object_body(input, key).map_err(|error| parse_error(path, &error))?;
    string_pairs_from_body(path, key, &body)
}

fn string_pairs_from_body(
    path: &Path,
    key: &str,
    body: &str,
) -> Result<Vec<(String, String)>, String> {
    object_pairs(body)?
        .into_iter()
        .map(|(name, value)| {
            let string = parse_string_at(&value, 0)
                .ok_or_else(|| parse_error(path, &format!("{key}.{name} must be a string")))?;
            Ok((name, string))
        })
        .collect()
}

fn required_string(path: &Path, input: &str, key: &str) -> Result<String, String> {
    let value = string_field(input, key).map_err(|error| parse_error(path, &error))?;
    if value.trim().is_empty() {
        return Err(parse_error(path, &format!("{key} must be non-empty")));
    }
    Ok(value)
}

fn validate_json_object(path: &Path, input: &str) -> Result<(), String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err(parse_error(path, "malformed trace JSON object"));
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
            return Err(parse_error(path, "malformed trace JSON nesting"));
        }
    }
    if depth != 0 || in_string {
        return Err(parse_error(path, "malformed trace JSON nesting"));
    }
    Ok(())
}

fn string_field(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    parse_string_at(input, start).ok_or_else(|| format!("field `{key}` must be a string"))
}

fn optional_string_field(input: &str, key: &str) -> Option<String> {
    string_field(input, key).ok()
}

fn number_field(input: &str, key: &str) -> Result<u64, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    parse_number_at(input, start).ok_or_else(|| format!("field `{key}` must be a number"))
}

fn bool_in_object(
    path: &Path,
    input: &str,
    section: &str,
    key: &str,
) -> Result<Option<bool>, String> {
    let body = object_body(input, section).map_err(|error| parse_error(path, &error))?;
    let needle = format!("\"{key}\":");
    let Some(start) = body.find(&needle) else {
        return Ok(None);
    };
    let tail = body[start + needle.len()..].trim_start();
    if tail.starts_with("true") {
        Ok(Some(true))
    } else if tail.starts_with("false") {
        Ok(Some(false))
    } else {
        Ok(None)
    }
}

fn string_or_null_in_object(
    input: &str,
    section: &str,
    key: &str,
) -> Result<Option<String>, String> {
    let body = object_body(input, section)?;
    let needle = format!("\"{key}\":");
    let Some(start) = body.find(&needle) else {
        return Ok(None);
    };
    let tail_start = start + needle.len();
    if body[tail_start..].trim_start().starts_with("null") {
        return Ok(None);
    }
    Ok(parse_string_at(&body, tail_start))
}

fn string_array_field(path: &Path, input: &str, key: &str) -> Result<Vec<String>, String> {
    let body = array_body(input, key).map_err(|error| parse_error(path, &error))?;
    let mut values = Vec::new();
    let mut index = 0;
    while index < body.len() {
        let rest = body[index..].trim_start();
        if rest.is_empty() {
            break;
        }
        index += body[index..].len() - rest.len();
        if body[index..].starts_with(',') {
            index += 1;
            continue;
        }
        let (value, next) = parse_json_string_at(&body, index)?;
        values.push(value);
        index = next;
    }
    Ok(values)
}

fn array_objects(path: &Path, input: &str, key: &str) -> Result<Vec<String>, String> {
    let body = array_body(input, key).map_err(|error| parse_error(path, &error))?;
    let mut objects = Vec::new();
    let mut index = 0;
    while index < body.len() {
        let rest = body[index..].trim_start();
        if rest.is_empty() {
            break;
        }
        index += body[index..].len() - rest.len();
        if body[index..].starts_with(',') {
            index += 1;
            continue;
        }
        if !body[index..].starts_with('{') {
            return Err(parse_error(path, &format!("{key} entries must be objects")));
        }
        let end = skip_json_value(&body, index)?;
        objects.push(body[index..end].trim_end_matches(',').trim().to_owned());
        index = end;
    }
    Ok(objects)
}

fn object_pairs(input: &str) -> Result<Vec<(String, String)>, String> {
    let body = input
        .trim()
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .ok_or_else(|| "malformed trace JSON object".to_owned())?;
    let mut pairs = Vec::new();
    let mut index = 0;
    while index < body.len() {
        let rest = body[index..].trim_start();
        if rest.is_empty() {
            break;
        }
        index += body[index..].len() - rest.len();
        if body[index..].starts_with(',') {
            index += 1;
            continue;
        }
        let (key, next) = parse_json_string_at(body, index)?;
        index = next;
        let after_key = body[index..].trim_start();
        if !after_key.starts_with(':') {
            return Err("malformed trace JSON field".to_owned());
        }
        index += body[index..].len() - after_key.len() + 1;
        let value_start = index;
        index = skip_json_value(body, index)?;
        pairs.push((
            key,
            body[value_start..index]
                .trim_end_matches(',')
                .trim()
                .to_owned(),
        ));
    }
    Ok(pairs)
}

fn object_body(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    object_body_at(input, key, start)
}

fn last_object_body(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .rfind(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    object_body_at(input, key, start)
}

fn object_body_at(input: &str, key: &str, start: usize) -> Result<String, String> {
    let tail = input[start..].trim_start();
    if !tail.starts_with('{') {
        return Err(format!("field `{key}` must be an object"));
    }
    let open = input.len() - tail.len();
    let end = skip_json_value(input, open)?;
    Ok(input[open..end].trim_end_matches(',').trim().to_owned())
}

fn array_body(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    let tail = input[start..].trim_start();
    if !tail.starts_with('[') {
        return Err(format!("field `{key}` must be an array"));
    }
    let open = input.len() - tail.len();
    let end = skip_json_value(input, open)?;
    Ok(input[open + 1..end - 1].to_owned())
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
                depth -= 1;
                if depth == 0 {
                    return Ok(index + offset + ch.len_utf8());
                }
                if depth < 0 {
                    return Err("malformed trace JSON nesting".to_owned());
                }
            }
            ',' if depth == 0 => return Ok(index + offset + 1),
            _ => {}
        }
    }
    Ok(input.len())
}

fn parse_string_at(input: &str, start: usize) -> Option<String> {
    let tail = input[start..].trim_start();
    let tail = tail.strip_prefix('"')?;
    let end = tail.find('"')?;
    Some(tail[..end].to_owned())
}

fn parse_json_string_at(input: &str, start: usize) -> Result<(String, usize), String> {
    let tail = input[start..]
        .strip_prefix('"')
        .ok_or_else(|| "expected JSON string".to_owned())?;
    let end = tail
        .find('"')
        .ok_or_else(|| "unterminated JSON string".to_owned())?;
    Ok((tail[..end].to_owned(), start + end + 2))
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

fn parse_error(path: &Path, reason: &str) -> String {
    format!(
        "trace-viewer failure\ntrace path: {}\nreason: {reason}",
        path.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str =
        include_str!("../../../games/race_to_n/tests/golden_traces/shortest-normal.trace.json");

    #[test]
    fn valid_trace_summary_contains_required_sections() {
        let trace = Trace::parse(Path::new("shortest-normal.trace.json"), VALID).unwrap();
        let summary = trace.render().unwrap();

        for expected in [
            "Metadata",
            "Fixture",
            "migration_update_note:",
            "Commands",
            "#0 actor=seat-0 path=add-1",
            "Checkpoints",
            "Expected Hashes",
            "state.final: 10275940640358619244",
            "Diagnostics",
            "Not Applicable",
            "Expected Outcome",
            "Actual Replay Annotation",
            "public_view.all: 3920897186672353542",
        ] {
            assert!(summary.contains(expected), "missing `{expected}`");
        }
    }

    #[test]
    fn malformed_trace_fails() {
        let error = Trace::parse(Path::new("bad.trace.json"), "{ nope").unwrap_err();

        assert!(error.contains("malformed trace JSON object"));
    }
}
