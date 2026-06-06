use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    process,
};

use engine_core::HashValue;
use race_to_n::replay_support::{
    replay_bot_action as race_replay_bot_action, replay_commands as race_replay_commands,
    replay_invalid as race_replay_invalid,
};
use three_marks::replay_support::{
    replay_bot_action as three_replay_bot_action, replay_commands as three_replay_commands,
    replay_diagnostic as three_replay_diagnostic, replay_stale as three_replay_stale,
};

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let config = Config::parse(args)?;
    let game = resolve_game(&config.game)?;
    if config.legacy_migration {
        return Err("legacy migration import is not implemented by replay-check".to_owned());
    }

    let trace_paths = config.trace_paths()?;
    let mut seen_ids = HashSet::new();
    let mut failures = Vec::new();
    for path in trace_paths {
        match check_trace_path(game, &path, &mut seen_ids) {
            Ok(()) => {}
            Err(error) => failures.push(error),
        }
    }

    if failures.is_empty() {
        println!("replay-check: all traces passed");
        Ok(())
    } else {
        Err(failures.join("\n\n"))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RegisteredGame {
    game_id: &'static str,
    rules_version: &'static str,
    trace_dir: &'static str,
}

fn resolve_game(game: &str) -> Result<RegisteredGame, String> {
    match game {
        "race_to_n" => Ok(RegisteredGame {
            game_id: "race_to_n",
            rules_version: "race_to_n-rules-v1",
            trace_dir: "games/race_to_n/tests/golden_traces",
        }),
        "three_marks" => Ok(RegisteredGame {
            game_id: "three_marks",
            rules_version: "three_marks-rules-v1",
            trace_dir: "games/three_marks/tests/golden_traces",
        }),
        _ => Err(format!("unsupported game `{game}`")),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ActualHashes {
    state_hash: HashValue,
    effect_hash: HashValue,
    action_tree_hash: HashValue,
    view_hash: HashValue,
    diagnostic_hash: Option<HashValue>,
    terminal: bool,
    winner: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    game: String,
    trace: Option<PathBuf>,
    directory: Option<PathBuf>,
    all: bool,
    legacy_migration: bool,
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
        let mut directory = None;
        let mut all = false;
        let mut legacy_migration = false;
        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--game" => game = Some(next_arg(&mut iter, "--game")?),
                "--trace" => trace = Some(PathBuf::from(next_arg(&mut iter, "--trace")?)),
                "--directory" => {
                    directory = Some(PathBuf::from(next_arg(&mut iter, "--directory")?))
                }
                "--all" => all = true,
                "--legacy-migration" => legacy_migration = true,
                other => return Err(format!("unknown argument `{other}`")),
            }
        }

        let selected_modes =
            usize::from(trace.is_some()) + usize::from(directory.is_some()) + usize::from(all);
        if selected_modes != 1 {
            return Err(
                "choose exactly one of --trace <path>, --directory <dir>, or --all".to_owned(),
            );
        }

        Ok(Self {
            game: game.ok_or_else(|| "--game is required".to_owned())?,
            trace,
            directory,
            all,
            legacy_migration,
        })
    }

    fn trace_paths(&self) -> Result<Vec<PathBuf>, String> {
        if let Some(trace) = &self.trace {
            return Ok(vec![trace.clone()]);
        }

        let directory = self.directory.clone().unwrap_or_else(|| {
            PathBuf::from(
                resolve_game(&self.game)
                    .expect("config game resolves")
                    .trace_dir,
            )
        });
        let mut paths = Vec::new();
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("failed to read `{}`: {error}", directory.display()))?
        {
            let path = entry
                .map_err(|error| {
                    format!("failed to read `{}` entry: {error}", directory.display())
                })?
                .path();
            if path.extension().and_then(|value| value.to_str()) == Some("json") {
                paths.push(path);
            }
        }
        paths.sort();
        if paths.is_empty() {
            return Err(format!(
                "no .json traces found in `{}`",
                directory.display()
            ));
        }
        Ok(paths)
    }
}

fn next_arg(iter: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    iter.next()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn print_help() {
    println!("replay-check 0.1.0");
    println!("usage:");
    println!("  replay-check --game <race_to_n|three_marks> --trace <path>");
    println!("  replay-check --game <race_to_n|three_marks> --directory <dir>");
    println!("  replay-check --game <race_to_n|three_marks> --all");
}

fn check_trace_path(
    game: RegisteredGame,
    path: &Path,
    seen_ids: &mut HashSet<String>,
) -> Result<(), String> {
    let input = fs::read_to_string(path)
        .map_err(|error| format!("{}: failed to read: {error}", path.display()))?;
    let trace = Trace::parse(path, &input)?;
    if !seen_ids.insert(trace.trace_id.clone()) {
        return Err(trace.failure("duplicate trace_id in checked trace set"));
    }
    trace.check(game)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Trace {
    path: PathBuf,
    trace_id: String,
    fixture_kind: String,
    game_id: String,
    schema_version: u64,
    rules_version: String,
    engine_version: String,
    data_version: String,
    migration_update_note: String,
    seed: u64,
    commands: Vec<String>,
    bot_seed: Option<u64>,
    invalid_command: Option<String>,
    stale_command: Option<String>,
    expected_state_hash: Option<u64>,
    expected_effect_hash: Option<u64>,
    expected_action_tree_hash: Option<u64>,
    expected_public_view_hash: Option<u64>,
    expected_diagnostic_hash: Option<u64>,
    expected_terminal: Option<bool>,
    expected_winner: Option<String>,
}

impl Trace {
    fn parse(path: &Path, input: &str) -> Result<Self, String> {
        validate_json_object(path, input)?;
        reject_unknown_root_fields(path, input)?;

        let schema_version = number_field(input, "schema_version")
            .map_err(|error| parse_error(path, "unknown", &error))?;
        if schema_version != 1 {
            return Err(parse_error(
                path,
                &string_field(input, "trace_id").unwrap_or_else(|_| "unknown".to_owned()),
                &format!("unsupported schema_version `{schema_version}`"),
            ));
        }

        let fixture_kind = string_field(input, "fixture_kind")
            .map_err(|error| parse_error(path, "unknown", &error))?;
        let trace_id = string_field(input, "trace_id")
            .map_err(|error| parse_error(path, "unknown", &error))?;
        let game_id =
            string_field(input, "game_id").map_err(|error| parse_error(path, &trace_id, &error))?;
        let rules_version = string_field(input, "rules_version")
            .map_err(|error| parse_error(path, &trace_id, &error))?;
        let engine_version = string_field(input, "engine_version")
            .map_err(|error| parse_error(path, &trace_id, &error))?;
        let data_version = string_field(input, "data_version")
            .map_err(|error| parse_error(path, &trace_id, &error))?;
        let migration_update_note = string_field(input, "migration_update_note")
            .map_err(|error| parse_error(path, &trace_id, &error))?;
        if migration_update_note.trim().is_empty() {
            return Err(parse_error(
                path,
                &trace_id,
                "migration_update_note must be non-empty",
            ));
        }
        if string_field(input, "note")
            .map_err(|error| parse_error(path, &trace_id, &error))?
            .trim()
            .is_empty()
        {
            return Err(parse_error(path, &trace_id, "note must be non-empty"));
        }

        let trace = Self {
            path: path.to_path_buf(),
            trace_id,
            fixture_kind,
            game_id,
            schema_version,
            rules_version,
            engine_version,
            data_version,
            migration_update_note,
            seed: number_field(input, "seed")
                .map_err(|error| parse_error(path, "unknown", &error))?,
            commands: action_paths(input),
            bot_seed: optional_number_field(input, "bot_seed")
                .map_err(|error| parse_error(path, "unknown", &error))?,
            invalid_command: command_with_expect(input, "invalid_action"),
            stale_command: command_with_expect(input, "stale_action"),
            expected_state_hash: optional_hash(input, "expected_state_hashes", "final")?,
            expected_effect_hash: optional_hash(input, "expected_effect_hashes", "final")?,
            expected_action_tree_hash: optional_hash(
                input,
                "expected_action_tree_hashes",
                "final",
            )?,
            expected_public_view_hash: optional_hash(input, "expected_public_view_hashes", "all")?,
            expected_diagnostic_hash: optional_diagnostic_hash(input),
            expected_terminal: optional_bool_in_object(
                input,
                "expected_terminal_state",
                "terminal",
            )?,
            expected_winner: optional_string_or_null_in_object(
                input,
                "expected_terminal_state",
                "winner",
            )?,
        };
        Ok(trace)
    }

    fn validate_versions_and_required_surfaces(&self, game: RegisteredGame) -> Result<(), String> {
        if self.game_id != game.game_id {
            return Err(self.failure(&format!("unsupported trace game_id `{}`", self.game_id)));
        }
        if self.rules_version != game.rules_version {
            return Err(self.failure(&format!(
                "unsupported rules_version `{}`",
                self.rules_version
            )));
        }
        if self.fixture_kind == "not_applicable" {
            return Ok(());
        }

        for (name, value) in [
            ("expected_state_hashes.final", self.expected_state_hash),
            ("expected_effect_hashes.final", self.expected_effect_hash),
            (
                "expected_action_tree_hashes.final",
                self.expected_action_tree_hash,
            ),
            (
                "expected_public_view_hashes.all",
                self.expected_public_view_hash,
            ),
        ] {
            if value.is_none() {
                return Err(self.failure(&format!("missing expected surface `{name}`")));
            }
        }
        if self.expected_terminal.is_none() {
            return Err(self.failure("missing expected terminal state"));
        }
        if self.fixture_kind == "invalid" && self.expected_diagnostic_hash.is_none() {
            return Err(self.failure("missing expected diagnostic hash"));
        }
        Ok(())
    }

    fn check(&self, game: RegisteredGame) -> Result<(), String> {
        self.validate_versions_and_required_surfaces(game)?;
        let Some(actual) = self.actual_hashes(game)? else {
            println!(
                "{} {}: not-applicable trace accepted",
                self.path.display(),
                self.trace_id
            );
            return Ok(());
        };

        self.compare_hash(
            "state",
            self.expected_state_hash,
            actual.state_hash,
            "final",
            None,
        )?;
        self.compare_hash(
            "effect",
            self.expected_effect_hash,
            actual.effect_hash,
            "final",
            None,
        )?;
        self.compare_hash(
            "action-tree",
            self.expected_action_tree_hash,
            actual.action_tree_hash,
            "final",
            None,
        )?;
        self.compare_hash(
            "public-view",
            self.expected_public_view_hash,
            actual.view_hash,
            "all",
            None,
        )?;
        if let Some(expected) = self.expected_diagnostic_hash {
            let actual_hash = actual
                .diagnostic_hash
                .ok_or_else(|| self.failure("expected diagnostic hash but replay produced none"))?;
            self.compare_hash(
                "diagnostic",
                Some(expected),
                actual_hash,
                "diagnostic",
                Some(self.commands.len().saturating_sub(1)),
            )?;
        } else if actual.diagnostic_hash.is_some() {
            return Err(self.failure("unexpected diagnostic hash produced by replay"));
        }

        if Some(actual.terminal) != self.expected_terminal {
            return Err(self.failure(&format!(
                "terminal mismatch at checkpoint final: expected {:?}, actual {}",
                self.expected_terminal, actual.terminal
            )));
        }
        if actual.winner != self.expected_winner {
            return Err(self.failure(&format!(
                "outcome mismatch at checkpoint final: expected {:?}, actual {:?}",
                self.expected_winner, actual.winner
            )));
        }

        println!("{} {}: ok", self.path.display(), self.trace_id);
        Ok(())
    }

    fn actual_hashes(&self, game: RegisteredGame) -> Result<Option<ActualHashes>, String> {
        match game.game_id {
            "race_to_n" => self.race_actual_hashes(),
            "three_marks" => self.three_actual_hashes(),
            _ => unreachable!("resolved games only"),
        }
    }

    fn race_actual_hashes(&self) -> Result<Option<ActualHashes>, String> {
        let Some(hashes) = (match self.fixture_kind.as_str() {
            "commands" | "terminal" => Some(race_replay_commands(self.seed, &self.commands)),
            "bot" => Some(race_replay_bot_action(
                self.seed,
                self.bot_seed
                    .ok_or_else(|| self.failure("bot trace missing producer bot_seed"))?,
            )),
            "invalid" | "diagnostic" => Some(race_replay_invalid(
                self.seed,
                self.invalid_command
                    .as_deref()
                    .ok_or_else(|| self.failure("invalid trace missing invalid command"))?,
                self.stale_command
                    .as_deref()
                    .ok_or_else(|| self.failure("invalid trace missing stale command"))?,
            )),
            "not_applicable" => None,
            other => return Err(self.failure(&format!("unsupported fixture_kind `{other}`"))),
        }) else {
            return Ok(None);
        };

        Ok(Some(ActualHashes {
            state_hash: hashes.state_hash,
            effect_hash: hashes.effect_hash,
            action_tree_hash: hashes.action_tree_hash,
            view_hash: hashes.view_hash,
            diagnostic_hash: hashes.diagnostic_hash,
            terminal: hashes.outcome.is_some(),
            winner: hashes.outcome.map(|winner| winner.as_str().to_owned()),
        }))
    }

    fn three_actual_hashes(&self) -> Result<Option<ActualHashes>, String> {
        let Some(hashes) = (match self.fixture_kind.as_str() {
            "commands" | "terminal" => Some(three_replay_commands(self.seed, &self.commands)),
            "bot" => Some(three_replay_bot_action(self.seed)),
            "diagnostic" => {
                if let Some(stale) = self.stale_command.as_deref() {
                    Some(three_replay_stale(self.seed, stale))
                } else {
                    let diagnostic = self
                        .commands
                        .last()
                        .cloned()
                        .ok_or_else(|| self.failure("diagnostic trace missing command"))?;
                    let setup = self.commands[..self.commands.len().saturating_sub(1)].to_vec();
                    Some(three_replay_diagnostic(self.seed, &setup, &diagnostic))
                }
            }
            "not_applicable" => None,
            other => return Err(self.failure(&format!("unsupported fixture_kind `{other}`"))),
        }) else {
            return Ok(None);
        };

        Ok(Some(ActualHashes {
            state_hash: hashes.state_hash,
            effect_hash: hashes.effect_hash,
            action_tree_hash: hashes.action_tree_hash,
            view_hash: hashes.view_hash,
            diagnostic_hash: hashes.diagnostic_hash,
            terminal: hashes.terminal,
            winner: hashes.outcome.and_then(|outcome| match outcome {
                three_marks::TerminalOutcome::Win { seat, .. } => Some(seat.as_str().to_owned()),
                three_marks::TerminalOutcome::Draw => None,
            }),
        }))
    }

    fn compare_hash(
        &self,
        surface: &str,
        expected: Option<u64>,
        actual: HashValue,
        checkpoint: &str,
        command_index: Option<usize>,
    ) -> Result<(), String> {
        let expected =
            expected.ok_or_else(|| self.failure(&format!("missing expected {surface} hash")))?;
        if actual != HashValue(expected) {
            return Err(self.failure(&format!(
                "{surface} hash drift\ncommand index: {}\ncheckpoint: {checkpoint}\nexpected: {expected}\nactual: {}",
                command_index
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| self.commands.len().saturating_sub(1).to_string()),
                actual.0
            )));
        }
        Ok(())
    }

    fn failure(&self, reason: &str) -> String {
        format!(
            "replay-check failure\ntrace path: {}\ntrace ID: {}\ngame ID: {}\nschema version: {}\nrules version: {}\nengine version: {}\ndata version: {}\nreason: {}\nreplay command: cargo run -p replay-check -- --game race_to_n --trace {}\nintentional updates require a migration/update note",
            self.path.display(),
            self.trace_id,
            self.game_id,
            self.schema_version,
            self.rules_version,
            self.engine_version,
            self.data_version,
            reason,
            self.path.display()
        )
    }
}

fn validate_json_object(path: &Path, input: &str) -> Result<(), String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err(parse_error(path, "unknown", "malformed trace JSON object"));
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
            return Err(parse_error(path, "unknown", "malformed trace JSON nesting"));
        }
    }
    if depth != 0 || in_string {
        return Err(parse_error(path, "unknown", "malformed trace JSON nesting"));
    }
    Ok(())
}

fn reject_unknown_root_fields(path: &Path, input: &str) -> Result<(), String> {
    let allowed = [
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
        "expected_replay_hashes",
        "expected_private_view_hashes",
        "expected_diagnostics",
        "expected_outcome",
        "expected_terminal_state",
        "not_applicable",
    ];
    for key in top_level_keys(input)? {
        if !allowed.contains(&key.as_str()) {
            return Err(parse_error(
                path,
                "unknown",
                &format!("unknown root field `{key}`"),
            ));
        }
    }
    Ok(())
}

fn top_level_keys(input: &str) -> Result<Vec<String>, String> {
    let body = input
        .trim()
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .ok_or_else(|| "malformed trace JSON object".to_owned())?;
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
            return Err("malformed trace JSON field".to_owned());
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

fn optional_number_field(input: &str, key: &str) -> Result<Option<u64>, String> {
    let needle = format!("\"{key}\":");
    Ok(input.find(&needle).map(|start| {
        parse_number_at(input, start + needle.len())
            .unwrap_or_else(|| panic!("field `{key}` must be a number"))
    }))
}

fn optional_hash(input: &str, section: &str, key: &str) -> Result<Option<u64>, String> {
    if !input.contains(&format!("\"{section}\":")) {
        return Ok(None);
    }
    let body = object_body(input, section)?;
    if body.contains(&format!("\"{key}\":")) {
        number_field(&body, key).map(Some)
    } else {
        Ok(None)
    }
}

fn optional_diagnostic_hash(input: &str) -> Option<u64> {
    input.find("\"expected_diagnostics\":").map(|start| {
        let tail = &input[start..];
        number_field(tail, "hash").expect("diagnostic hash must be numeric")
    })
}

fn optional_bool_in_object(input: &str, section: &str, key: &str) -> Result<Option<bool>, String> {
    let body = object_body(input, section)?;
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

fn optional_string_or_null_in_object(
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

fn object_body(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing `{key}`"))?
        + needle.len();
    let open = input[start..]
        .find('{')
        .ok_or_else(|| format!("field `{key}` must be an object"))?
        + start;
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
        match ch {
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(input[open..=open + offset].to_owned());
                }
            }
            _ => {}
        }
    }
    Err(format!("object `{key}` must close"))
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

fn parse_first_array_string(input: &str) -> String {
    parse_string_at(input, 0).expect("array must contain a string path segment")
}

fn parse_error(path: &Path, trace_id: &str, reason: &str) -> String {
    format!(
        "replay-check failure\ntrace path: {}\ntrace ID: {}\ngame ID: unknown\nschema version: unknown\nrules version: unknown\nengine version: unknown\ndata version: unknown\nreason: {reason}\nreplay command: cargo run -p replay-check -- --game race_to_n --trace {}\nintentional updates require a migration/update note",
        path.display(),
        trace_id,
        path.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str =
        include_str!("../../../games/race_to_n/tests/golden_traces/shortest-normal.trace.json");

    #[test]
    fn valid_trace_passes() {
        let trace = Trace::parse(Path::new("shortest-normal.trace.json"), VALID).unwrap();
        trace.check(resolve_game("race_to_n").unwrap()).unwrap();
    }

    #[test]
    fn corrupted_hash_fails() {
        let corrupted = VALID.replace("4954817074678372285", "4954817074678372286");
        let trace = Trace::parse(Path::new("shortest-normal.trace.json"), &corrupted).unwrap();
        let error = trace.check(resolve_game("race_to_n").unwrap()).unwrap_err();

        assert!(error.contains("state hash drift"));
        assert!(error.contains("expected: 4954817074678372286"));
        assert!(error.contains("actual: 4954817074678372285"));
    }

    #[test]
    fn malformed_trace_fails() {
        let error = Trace::parse(Path::new("bad.trace.json"), "{ nope").unwrap_err();

        assert!(error.contains("malformed trace JSON object"));
    }

    #[test]
    fn unknown_schema_version_fails() {
        let unknown = VALID.replace("\"schema_version\": 1", "\"schema_version\": 99");
        let error = Trace::parse(Path::new("shortest-normal.trace.json"), &unknown).unwrap_err();

        assert!(error.contains("unsupported schema_version `99`"));
    }

    #[test]
    fn unknown_root_field_fails() {
        let unknown = VALID.replace(
            "\"trace_id\": \"shortest-normal\"",
            "\"trace_id\": \"shortest-normal\", \"extra\": true",
        );
        let error = Trace::parse(Path::new("shortest-normal.trace.json"), &unknown).unwrap_err();

        assert!(error.contains("unknown root field `extra`"));
    }
}
