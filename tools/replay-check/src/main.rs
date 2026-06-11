use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    process,
};

use column_four::replay_support::replay_commands as column_replay_commands;
use directional_flip::replay_support::{
    replay_commands as directional_replay_commands,
    replay_from_state as directional_replay_from_state,
};
use draughts_lite::replay_support::{
    diagnostic_hash as draughts_diagnostic_hash, replay_commands as draughts_replay_commands,
    replay_from_state as draughts_replay_from_state,
};
use engine_core::{
    ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, Seed, StableSerialize,
    Viewer,
};
use race_to_n::replay_support::{
    replay_bot_action as race_replay_bot_action, replay_commands as race_replay_commands,
    replay_invalid as race_replay_invalid,
};
use three_marks::replay_support::{
    replay_bot_action as three_replay_bot_action, replay_commands as three_replay_commands,
    replay_diagnostic as three_replay_diagnostic, replay_stale as three_replay_stale,
};
use token_bazaar::{ContractId, ResourceCounts};

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
        "column_four" => Ok(RegisteredGame {
            game_id: "column_four",
            rules_version: "column_four-rules-v1",
            trace_dir: "games/column_four/tests/golden_traces",
        }),
        "directional_flip" => Ok(RegisteredGame {
            game_id: "directional_flip",
            rules_version: "directional_flip-rules-v1",
            trace_dir: "games/directional_flip/tests/golden_traces",
        }),
        "draughts_lite" => Ok(RegisteredGame {
            game_id: "draughts_lite",
            rules_version: "draughts_lite-rules-v1",
            trace_dir: "games/draughts_lite/tests/golden_traces",
        }),
        "high_card_duel" => Ok(RegisteredGame {
            game_id: "high_card_duel",
            rules_version: "high-card-duel-rules-v1",
            trace_dir: "games/high_card_duel/tests/golden_traces",
        }),
        "masked_claims" => Ok(RegisteredGame {
            game_id: "masked_claims",
            rules_version: "masked-claims-rules-v1",
            trace_dir: "games/masked_claims/tests/golden_traces",
        }),
        "token_bazaar" => Ok(RegisteredGame {
            game_id: "token_bazaar",
            rules_version: "token-bazaar-rules-v1",
            trace_dir: "games/token_bazaar/tests/golden_traces",
        }),
        "secret_draft" => Ok(RegisteredGame {
            game_id: "secret_draft",
            rules_version: "secret-draft-rules-v1",
            trace_dir: "games/secret_draft/tests/golden_traces",
        }),
        "poker_lite" => Ok(RegisteredGame {
            game_id: "poker_lite",
            rules_version: "poker-lite-rules-v1",
            trace_dir: "games/poker_lite/tests/golden_traces",
        }),
        "plain_tricks" => Ok(RegisteredGame {
            game_id: "plain_tricks",
            rules_version: "plain-tricks-rules-v1",
            trace_dir: "games/plain_tricks/tests/golden_traces",
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
        if selected_modes > 1 {
            return Err(
                "choose exactly one of --trace <path>, --directory <dir>, or --all".to_owned(),
            );
        }

        Ok(Self {
            game: game.ok_or_else(|| "--game is required".to_owned())?,
            trace,
            directory,
            all: all || selected_modes == 0,
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
    println!(
        "  replay-check --game <race_to_n|three_marks|column_four|directional_flip|draughts_lite|high_card_duel|masked_claims|token_bazaar|secret_draft|poker_lite|plain_tricks> --trace <path>"
    );
    println!("  replay-check --game <race_to_n|three_marks|column_four|directional_flip|draughts_lite|high_card_duel|masked_claims|token_bazaar|secret_draft|poker_lite|plain_tricks> --directory <dir>");
    println!("  replay-check --game <race_to_n|three_marks|column_four|directional_flip|draughts_lite|high_card_duel|masked_claims|token_bazaar|secret_draft|poker_lite|plain_tricks> --all");
}

fn check_trace_path(
    game: RegisteredGame,
    path: &Path,
    seen_ids: &mut HashSet<String>,
) -> Result<(), String> {
    let input = fs::read_to_string(path)
        .map_err(|error| format!("{}: failed to read: {error}", path.display()))?;
    if is_public_export_fixture(&input) {
        validate_public_export_fixture(game, path, &input)?;
        println!("{}: public export fixture accepted", path.display());
        return Ok(());
    }
    let trace = Trace::parse(path, &input)?;
    if !seen_ids.insert(trace.trace_id.clone()) {
        return Err(trace.failure("duplicate trace_id in checked trace set"));
    }
    trace.check(game)
}

fn is_public_export_fixture(input: &str) -> bool {
    input.contains("\"export_class\":")
}

fn validate_public_export_fixture(
    game: RegisteredGame,
    path: &Path,
    input: &str,
) -> Result<(), String> {
    validate_json_object(path, input)?;
    let schema_version = number_field(input, "schema_version")
        .map_err(|error| parse_error(path, "public-export", &error))?;
    if schema_version != 1 {
        return Err(parse_error(
            path,
            "public-export",
            &format!("unsupported schema_version `{schema_version}`"),
        ));
    }
    let export_class = string_field(input, "export_class")
        .map_err(|error| parse_error(path, "public-export", &error))?;
    if export_class.trim().is_empty() {
        return Err(parse_error(
            path,
            "public-export",
            "export_class must be non-empty",
        ));
    }
    let game_id = string_field(input, "game_id")
        .map_err(|error| parse_error(path, "public-export", &error))?;
    if game_id != game.game_id {
        return Err(parse_error(
            path,
            "public-export",
            &format!("unsupported export game_id `{game_id}`"),
        ));
    }
    let rules_version = string_field(input, "rules_version")
        .map_err(|error| parse_error(path, "public-export", &error))?;
    if rules_version != game.rules_version {
        return Err(parse_error(
            path,
            "public-export",
            &format!("unsupported export rules_version `{rules_version}`"),
        ));
    }
    if !input.contains("\"steps\":") {
        return Err(parse_error(
            path,
            "public-export",
            "public export fixture must contain steps",
        ));
    }
    Ok(())
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
    commands: Vec<Vec<String>>,
    command_actor_seats: Vec<String>,
    bot_seed: Option<u64>,
    invalid_command: Option<Vec<String>>,
    stale_command: Option<Vec<String>>,
    setup_patch: Option<String>,
    expected_state_hash: Option<u64>,
    expected_effect_hash: Option<u64>,
    expected_action_tree_hash: Option<u64>,
    expected_public_view_hash: Option<u64>,
    expected_diagnostic_hash: Option<u64>,
    expected_terminal: Option<bool>,
    expected_winner: Option<String>,
    diagnostic_actor_seat: Option<String>,
    diagnostic_freshness_token: Option<u64>,
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
        let is_masked_claims = game_id == "masked_claims";
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
            command_actor_seats: command_string_fields(input, "actor_seat"),
            bot_seed: optional_number_field(input, "bot_seed")
                .map_err(|error| parse_error(path, "unknown", &error))?,
            invalid_command: command_with_expect(input, "invalid_action"),
            stale_command: command_with_expect(input, "stale_action"),
            setup_patch: optional_string_field(input, "setup_patch")
                .map_err(|error| parse_error(path, "unknown", &error))?,
            expected_state_hash: optional_hash(input, "expected_state_hashes", "final")?,
            expected_effect_hash: optional_hash(input, "expected_effect_hashes", "final")?,
            expected_action_tree_hash: optional_hash(
                input,
                "expected_action_tree_hashes",
                "final",
            )?,
            expected_public_view_hash: optional_hash(input, "expected_public_view_hashes", "all")?
                .or(optional_hash(
                    input,
                    "expected_public_view_hashes",
                    "observer",
                )?),
            expected_diagnostic_hash: optional_diagnostic_hash(input),
            expected_terminal: if is_masked_claims && !input.contains("\"expected_terminal_state\"")
            {
                None
            } else {
                optional_bool_in_object(input, "expected_terminal_state", "terminal")?
            },
            expected_winner: if is_masked_claims && !input.contains("\"expected_terminal_state\"") {
                None
            } else {
                optional_string_or_null_in_object(input, "expected_terminal_state", "winner")?
            },
            diagnostic_actor_seat: last_command_string_field(input, "actor_seat"),
            diagnostic_freshness_token: last_command_string_field(input, "freshness_token")
                .and_then(|value| value.parse().ok()),
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
            if self.game_id == "masked_claims"
                && matches!(
                    name,
                    "expected_action_tree_hashes.final" | "expected_public_view_hashes.all"
                )
                && value.is_none()
            {
                continue;
            }
            if value.is_none() {
                return Err(self.failure(&format!("missing expected surface `{name}`")));
            }
        }
        if self.expected_terminal.is_none() && self.game_id != "masked_claims" {
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
            "column_four" => self.column_actual_hashes(),
            "directional_flip" => self.directional_actual_hashes(),
            "draughts_lite" => self.draughts_actual_hashes(),
            "high_card_duel" => self.high_card_duel_actual_hashes(),
            "masked_claims" => Ok(None),
            "token_bazaar" => self.token_bazaar_actual_hashes(),
            "secret_draft" => self.secret_draft_actual_hashes(),
            "poker_lite" => self.poker_lite_actual_hashes(),
            "plain_tricks" => self.plain_tricks_actual_hashes(),
            _ => unreachable!("resolved games only"),
        }
    }

    fn race_actual_hashes(&self) -> Result<Option<ActualHashes>, String> {
        let Some(hashes) = (match self.fixture_kind.as_str() {
            "commands" | "terminal" => Some(race_replay_commands(
                self.seed,
                &single_segments(&self.commands)?,
            )),
            "bot" => Some(race_replay_bot_action(
                self.seed,
                self.bot_seed
                    .ok_or_else(|| self.failure("bot trace missing producer bot_seed"))?,
            )),
            "invalid" | "diagnostic" => Some(race_replay_invalid(
                self.seed,
                &single_segment(
                    self.invalid_command
                        .as_ref()
                        .ok_or_else(|| self.failure("invalid trace missing invalid command"))?,
                )?,
                &single_segment(
                    self.stale_command
                        .as_ref()
                        .ok_or_else(|| self.failure("invalid trace missing stale command"))?,
                )?,
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
            "commands" | "terminal" => Some(three_replay_commands(
                self.seed,
                &single_segments(&self.commands)?,
            )),
            "bot" => Some(three_replay_bot_action(self.seed)),
            "diagnostic" => {
                if let Some(stale) = self.stale_command.as_ref() {
                    Some(three_replay_stale(self.seed, &single_segment(stale)?))
                } else {
                    let diagnostic = self
                        .commands
                        .last()
                        .cloned()
                        .ok_or_else(|| self.failure("diagnostic trace missing command"))?;
                    let setup =
                        single_segments(&self.commands[..self.commands.len().saturating_sub(1)])?;
                    let diagnostic = single_segment(&diagnostic)?;
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

    fn column_actual_hashes(&self) -> Result<Option<ActualHashes>, String> {
        let (commands, diagnostic_hash) = if self.fixture_kind == "diagnostic" {
            let diagnostic = self
                .commands
                .last()
                .cloned()
                .ok_or_else(|| self.failure("diagnostic trace missing command"))?;
            let setup = self.commands[..self.commands.len().saturating_sub(1)].to_vec();
            (
                setup.clone(),
                Some(self.column_diagnostic_hash(&setup, &diagnostic)?),
            )
        } else {
            (self.commands.clone(), None)
        };
        let command_segments = single_segments(&commands)?;

        let Some(hashes) = (match self.fixture_kind.as_str() {
            "commands" | "terminal" | "bot" | "diagnostic" => {
                Some(column_replay_commands(self.seed, &command_segments))
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
            diagnostic_hash,
            terminal: hashes.terminal,
            winner: hashes.outcome.and_then(|outcome| match outcome {
                column_four::TerminalOutcome::Win { seat, .. } => Some(seat.as_str().to_owned()),
                column_four::TerminalOutcome::Draw => None,
            }),
        }))
    }

    fn column_diagnostic_hash(
        &self,
        setup_commands: &[Vec<String>],
        diagnostic_command: &[String],
    ) -> Result<HashValue, String> {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let mut state = column_four::setup_match(
            Seed(self.seed),
            &seats,
            &column_four::SetupOptions::default(),
        )
        .map_err(diagnostic_string)?;
        for path in setup_commands {
            let command = column_command_for_state(&state, single_segment(path)?);
            let action =
                column_four::validate_command(&state, &command).map_err(diagnostic_string)?;
            column_four::apply_action(&mut state, action);
        }
        let diagnostic_segment = single_segment(diagnostic_command)?;
        let mut command = column_command_for_state(&state, diagnostic_segment);
        if self.stale_command.as_deref() == Some(diagnostic_command) {
            command.freshness_token = state.freshness_token.next();
        }
        let diagnostic = column_four::validate_command(&state, &command)
            .expect_err("diagnostic trace command must reject");
        Ok(HashValue::from_stable_bytes(
            format!("{}:{}", diagnostic.code, diagnostic.message).as_bytes(),
        ))
    }

    fn directional_actual_hashes(&self) -> Result<Option<ActualHashes>, String> {
        let (commands, diagnostic_hash) = if self.fixture_kind == "diagnostic" {
            let diagnostic = self
                .commands
                .last()
                .cloned()
                .ok_or_else(|| self.failure("diagnostic trace missing command"))?;
            let setup = self.commands[..self.commands.len().saturating_sub(1)].to_vec();
            (
                setup.clone(),
                Some(self.directional_diagnostic_hash(&setup, &diagnostic)?),
            )
        } else {
            (self.commands.clone(), None)
        };

        let Some(hashes) = (match self.fixture_kind.as_str() {
            "commands" | "terminal" | "bot" | "diagnostic" => {
                Some(self.directional_replay_hashes(&single_segments(&commands)?))
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
            diagnostic_hash,
            terminal: hashes.terminal,
            winner: hashes.outcome.and_then(|outcome| match outcome {
                directional_flip::TerminalOutcome::Win { seat } => Some(seat.as_str().to_owned()),
                directional_flip::TerminalOutcome::Draw => None,
            }),
        }))
    }

    fn directional_diagnostic_hash(
        &self,
        setup_commands: &[Vec<String>],
        diagnostic_command: &[String],
    ) -> Result<HashValue, String> {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let mut state = directional_flip::setup_match(
            Seed(self.seed),
            &seats,
            &directional_flip::SetupOptions::default(),
        )
        .map_err(diagnostic_string)?;
        for path in setup_commands {
            let command = directional_command_for_state(&state, single_segment(path)?);
            let action =
                directional_flip::validate_command(&state, &command).map_err(diagnostic_string)?;
            directional_flip::apply_action(&mut state, action);
        }
        let actor_seat = self
            .diagnostic_actor_seat
            .as_deref()
            .and_then(directional_flip::DirectionalFlipSeat::parse)
            .unwrap_or(state.active_seat);
        let diagnostic_segment = single_segment(diagnostic_command)?;
        let mut command = directional_command_for_seat(&state, actor_seat, &diagnostic_segment);
        if let Some(freshness_token) = self.diagnostic_freshness_token {
            command.freshness_token = engine_core::FreshnessToken(freshness_token);
        }
        let diagnostic = directional_flip::validate_command(&state, &command)
            .expect_err("diagnostic trace command must reject");
        Ok(HashValue::from_stable_bytes(
            format!("{}:{}", diagnostic.code, diagnostic.message).as_bytes(),
        ))
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

    fn draughts_actual_hashes(&self) -> Result<Option<ActualHashes>, String> {
        let (commands, diagnostic_hash) = if self.fixture_kind == "diagnostic" {
            let diagnostic = self
                .commands
                .last()
                .cloned()
                .ok_or_else(|| self.failure("diagnostic trace missing command"))?;
            let setup = self.commands[..self.commands.len().saturating_sub(1)].to_vec();
            (
                setup.clone(),
                Some(self.draughts_diagnostic_hash(&setup, &diagnostic)?),
            )
        } else {
            (self.commands.clone(), None)
        };

        let Some(hashes) = (match self.fixture_kind.as_str() {
            "commands" | "terminal" | "bot" | "diagnostic" => {
                Some(self.draughts_replay_hashes(&commands))
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
            diagnostic_hash,
            terminal: hashes.terminal,
            winner: hashes.outcome.map(|outcome| match outcome {
                draughts_lite::TerminalOutcome::Win { seat } => seat.as_str().to_owned(),
            }),
        }))
    }

    fn draughts_diagnostic_hash(
        &self,
        setup_commands: &[Vec<String>],
        diagnostic_command: &[String],
    ) -> Result<HashValue, String> {
        let mut state = draughts_initial_state(&self.trace_id, self.seed);
        for path in setup_commands {
            let command = draughts_command_for_state(&state, state.active_seat, path.clone());
            let action =
                draughts_lite::validate_command(&state, &command).map_err(diagnostic_string)?;
            draughts_lite::apply_action(&mut state, action);
        }
        let actor_seat = self
            .diagnostic_actor_seat
            .as_deref()
            .and_then(parse_draughts_trace_seat)
            .unwrap_or(state.active_seat);
        let mut command =
            draughts_command_for_state(&state, actor_seat, diagnostic_command.to_vec());
        if let Some(freshness_token) = self.diagnostic_freshness_token {
            command.freshness_token = engine_core::FreshnessToken(freshness_token);
        }
        let diagnostic = draughts_lite::validate_command(&state, &command)
            .expect_err("diagnostic trace command must reject");
        Ok(draughts_diagnostic_hash(&[diagnostic]))
    }

    fn draughts_replay_hashes(&self, commands: &[Vec<String>]) -> draughts_lite::ReplayHashes {
        match self.trace_id.as_str() {
            "draughts-lite-mandatory-capture-suppresses-quiet"
            | "draughts-lite-quiet-while-capture-diagnostic"
            | "draughts-lite-single-capture"
            | "draughts-lite-multi-jump"
            | "draughts-lite-illegal-continuation-diagnostic"
            | "draughts-lite-forced-continuation-branch"
            | "draughts-lite-promotion-quiet"
            | "draughts-lite-promotion-during-capture-stop"
            | "draughts-lite-path-after-promotion-stop-diagnostic"
            | "draughts-lite-terminal-no-pieces"
            | "draughts-lite-terminal-no-legal-moves" => {
                let mut state = draughts_initial_state(&self.trace_id, self.seed);
                let initial_snapshot =
                    draughts_lite::DraughtsLiteSnapshot::from_state(&state).stable_summary();
                draughts_replay_from_state(self.seed, initial_snapshot, commands, &mut state)
            }
            _ => draughts_replay_commands(self.seed, commands),
        }
    }

    fn high_card_duel_actual_hashes(&self) -> Result<Option<ActualHashes>, String> {
        let (commands, diagnostic_hash) = if self.fixture_kind == "diagnostic" {
            let diagnostic = self
                .commands
                .last()
                .cloned()
                .ok_or_else(|| self.failure("diagnostic trace missing command"))?;
            let setup = self.commands[..self.commands.len().saturating_sub(1)].to_vec();
            (
                setup.clone(),
                Some(self.high_card_duel_diagnostic_hash(&setup, &diagnostic)?),
            )
        } else {
            (self.commands.clone(), None)
        };

        let Some(actual) = (match self.fixture_kind.as_str() {
            "commands" | "terminal" | "bot" | "diagnostic" => {
                Some(self.high_card_duel_replay_hashes(&commands)?)
            }
            "not_applicable" => None,
            other => return Err(self.failure(&format!("unsupported fixture_kind `{other}`"))),
        }) else {
            return Ok(None);
        };

        Ok(Some(ActualHashes {
            state_hash: actual.state_hash,
            effect_hash: actual.effect_hash,
            action_tree_hash: actual.action_tree_hash,
            view_hash: actual.view_hash,
            diagnostic_hash,
            terminal: actual.terminal,
            winner: actual.outcome.and_then(|outcome| match outcome {
                high_card_duel::TerminalOutcome::Win { seat } => Some(seat.as_str().to_owned()),
                high_card_duel::TerminalOutcome::Draw => None,
            }),
        }))
    }

    fn high_card_duel_diagnostic_hash(
        &self,
        setup_commands: &[Vec<String>],
        diagnostic_command: &[String],
    ) -> Result<HashValue, String> {
        let seats = high_card_duel::default_seats();
        let mut state = high_card_duel::setup_match(
            Seed(self.seed),
            &seats,
            &high_card_duel::SetupOptions::default(),
        )
        .map_err(diagnostic_string)?;
        for path in setup_commands {
            let command = high_card_duel_command_for_state(&state, path.clone())?;
            let action =
                high_card_duel::validate_command(&state, &command).map_err(diagnostic_string)?;
            high_card_duel::apply_action(&mut state, action);
        }
        let actor_seat = self
            .diagnostic_actor_seat
            .as_deref()
            .and_then(parse_high_card_duel_trace_seat)
            .or_else(|| high_card_duel::active_commit_seat(&state))
            .ok_or_else(|| self.failure("diagnostic trace has no active seat"))?;
        let mut command =
            high_card_duel_command_for_seat(&state, actor_seat, diagnostic_command.to_vec());
        if let Some(freshness_token) = self.diagnostic_freshness_token {
            command.freshness_token = engine_core::FreshnessToken(freshness_token);
        }
        let diagnostic = high_card_duel::validate_command(&state, &command)
            .expect_err("diagnostic trace command must reject");
        Ok(HashValue::from_stable_bytes(
            format!("{}:{}", diagnostic.code, diagnostic.message).as_bytes(),
        ))
    }

    fn high_card_duel_replay_hashes(
        &self,
        commands: &[Vec<String>],
    ) -> Result<HighCardDuelReplayHashes, String> {
        let seats = high_card_duel::default_seats();
        let mut state = high_card_duel::setup_match(
            Seed(self.seed),
            &seats,
            &high_card_duel::SetupOptions::default(),
        )
        .map_err(diagnostic_string)?;
        let mut effects = Vec::new();

        for path in commands {
            let command = high_card_duel_command_for_state(&state, path.clone())?;
            let action =
                high_card_duel::validate_command(&state, &command).map_err(diagnostic_string)?;
            effects.extend(high_card_duel::apply_action(&mut state, action));
        }

        Ok(HighCardDuelReplayHashes {
            state_hash: high_card_duel::state_hash(&state),
            effect_hash: high_card_duel::effect_hash(&effects),
            action_tree_hash: high_card_duel_action_tree_hash(&state),
            view_hash: high_card_duel::project_view(&state, &Viewer { seat_id: None })
                .stable_hash(),
            terminal: state.terminal_outcome.is_some(),
            outcome: state.terminal_outcome,
        })
    }

    fn token_bazaar_actual_hashes(&self) -> Result<Option<ActualHashes>, String> {
        let (commands, diagnostic_hash) = if self.fixture_kind == "diagnostic" {
            let diagnostic = self
                .commands
                .last()
                .cloned()
                .ok_or_else(|| self.failure("diagnostic trace missing command"))?;
            let setup = self.commands[..self.commands.len().saturating_sub(1)].to_vec();
            (
                setup.clone(),
                Some(self.token_bazaar_diagnostic_hash(&setup, &diagnostic)?),
            )
        } else {
            (self.commands.clone(), None)
        };

        let Some(actual) = (match self.fixture_kind.as_str() {
            "commands" | "terminal" | "bot" | "wasm" | "diagnostic" => {
                Some(self.token_bazaar_replay_hashes(&commands)?)
            }
            "not_applicable" => None,
            other => return Err(self.failure(&format!("unsupported fixture_kind `{other}`"))),
        }) else {
            return Ok(None);
        };

        Ok(Some(ActualHashes {
            state_hash: actual.state_hash,
            effect_hash: actual.effect_hash,
            action_tree_hash: actual.action_tree_hash,
            view_hash: actual.view_hash,
            diagnostic_hash,
            terminal: actual.terminal,
            winner: actual.outcome.and_then(|outcome| match outcome {
                token_bazaar::TerminalOutcome::Win { seat } => Some(seat.as_str().to_owned()),
                token_bazaar::TerminalOutcome::Draw => None,
            }),
        }))
    }

    fn token_bazaar_diagnostic_hash(
        &self,
        setup_commands: &[Vec<String>],
        diagnostic_command: &[String],
    ) -> Result<HashValue, String> {
        let mut state = self.token_bazaar_initial_state()?;
        for path in setup_commands {
            let command = token_bazaar::command_for_state(&state, path.clone());
            let action =
                token_bazaar::validate_command(&state, &command).map_err(diagnostic_string)?;
            token_bazaar::apply_action(&mut state, action);
        }
        let actor_seat = self
            .diagnostic_actor_seat
            .as_deref()
            .and_then(parse_token_bazaar_trace_seat)
            .unwrap_or(state.active_seat);
        let mut command =
            token_bazaar_command_for_seat(&state, actor_seat, diagnostic_command.to_vec());
        if let Some(freshness_token) = self.diagnostic_freshness_token {
            command.freshness_token = engine_core::FreshnessToken(freshness_token);
        }
        let diagnostic = token_bazaar::validate_command(&state, &command)
            .expect_err("diagnostic trace command must reject");
        Ok(HashValue::from_stable_bytes(
            format!("diagnostic:{}", diagnostic.code).as_bytes(),
        ))
    }

    fn token_bazaar_replay_hashes(
        &self,
        commands: &[Vec<String>],
    ) -> Result<TokenBazaarReplayHashes, String> {
        if self.setup_patch.is_none() {
            let replay = token_bazaar::replay_commands(self.seed, commands);
            return Ok(TokenBazaarReplayHashes {
                state_hash: replay.final_state_hash,
                effect_hash: replay.effect_hash,
                action_tree_hash: replay.action_tree_hash,
                view_hash: replay.public_view_hash,
                terminal: replay.terminal,
                outcome: replay.terminal_outcome,
            });
        }

        let mut state = self.token_bazaar_initial_state()?;
        let mut effects = Vec::new();
        for path in commands {
            let command = token_bazaar::command_for_state(&state, path.clone());
            let action =
                token_bazaar::validate_command(&state, &command).map_err(diagnostic_string)?;
            effects.extend(token_bazaar::apply_action(&mut state, action));
        }
        Ok(TokenBazaarReplayHashes {
            state_hash: token_bazaar::state_hash(&state),
            effect_hash: token_bazaar::effect_hash(&effects),
            action_tree_hash: token_bazaar_action_tree_hash(&state),
            view_hash: token_bazaar::project_view(&state, &Viewer { seat_id: None }).stable_hash(),
            terminal: state.terminal_outcome.is_some(),
            outcome: state.terminal_outcome,
        })
    }

    fn token_bazaar_initial_state(&self) -> Result<token_bazaar::TokenBazaarState, String> {
        let mut state = token_bazaar::setup_match(
            Seed(self.seed),
            &token_bazaar::default_seats(),
            &token_bazaar::SetupOptions::default(),
        )
        .map_err(diagnostic_string)?;
        match self.setup_patch.as_deref() {
            None => Ok(state),
            Some("near_market_exhaustion") => {
                state.slots = [Some(ContractId::CrownRoute), None, None];
                state.queue.clear();
                state.inventories[0] = ResourceCounts::new(2, 0, 2);
                state.supply = ResourceCounts::new(12, 14, 12);
                Ok(state)
            }
            Some("empty_slot_non_terminal") => {
                state.slots = [None, Some(ContractId::CrownRoute), None];
                state.queue.clear();
                state.inventories[0] = ResourceCounts::new(2, 0, 2);
                state.supply = ResourceCounts::new(12, 14, 12);
                Ok(state)
            }
            Some("terminal_score_win") => {
                state.scores = [5, 3];
                state.terminal_outcome = Some(token_bazaar::determine_terminal_outcome(&state));
                state.terminal_trigger = Some(token_bazaar::TerminalTrigger::TurnCap);
                Ok(state)
            }
            Some("terminal_fulfilled_tiebreak_win") => {
                state.scores = [3, 3];
                state.fulfilled = [
                    vec![ContractId::BalancedWares, ContractId::AmberGuild],
                    vec![ContractId::IronGuild],
                ];
                state.terminal_outcome = Some(token_bazaar::determine_terminal_outcome(&state));
                state.terminal_trigger = Some(token_bazaar::TerminalTrigger::TurnCap);
                Ok(state)
            }
            Some("terminal_inventory_tiebreak_win") => {
                state.scores = [3, 3];
                state.fulfilled = [vec![ContractId::BalancedWares], vec![ContractId::IronGuild]];
                state.inventories = [ResourceCounts::new(1, 1, 1), ResourceCounts::new(2, 1, 1)];
                state.terminal_outcome = Some(token_bazaar::determine_terminal_outcome(&state));
                state.terminal_trigger = Some(token_bazaar::TerminalTrigger::MarketExhaustion);
                Ok(state)
            }
            Some("terminal_all_tied_draw") => {
                state.scores = [3, 3];
                state.fulfilled = [vec![ContractId::BalancedWares], vec![ContractId::IronGuild]];
                state.inventories = [ResourceCounts::new(1, 1, 1), ResourceCounts::new(1, 1, 1)];
                state.terminal_outcome = Some(token_bazaar::determine_terminal_outcome(&state));
                state.terminal_trigger = Some(token_bazaar::TerminalTrigger::TurnCap);
                Ok(state)
            }
            Some(other) => Err(self.failure(&format!("unknown setup_patch `{other}`"))),
        }
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

    fn secret_draft_actual_hashes(&self) -> Result<Option<ActualHashes>, String> {
        if !matches!(
            self.fixture_kind.as_str(),
            "commands" | "terminal" | "diagnostic" | "bot" | "no_leak" | "export" | "wasm"
        ) {
            return Ok(None);
        }

        let mut state = secret_draft::setup_match(
            &secret_draft::replay_support::default_seats(),
            &secret_draft::SetupOptions::default(),
        )
        .map_err(diagnostic_string)?;
        let mut effects = Vec::new();
        let mut diagnostic_hash = None;
        let has_diagnostic = self.expected_diagnostic_hash.is_some();
        let applied_count = if has_diagnostic {
            self.commands.len().saturating_sub(1)
        } else {
            self.commands.len()
        };

        for index in 0..applied_count {
            let seat = self.secret_draft_actor(index)?;
            let command = secret_draft_command_for_seat(&state, seat, self.commands[index].clone());
            let action = secret_draft::actions::validate_command(&state, &command)
                .map_err(diagnostic_string)?;
            effects
                .extend(secret_draft::apply_action(&mut state, action).map_err(diagnostic_string)?);
        }

        if has_diagnostic {
            let index = self.commands.len().saturating_sub(1);
            let seat = self.secret_draft_actor(index)?;
            let mut command =
                secret_draft_command_for_seat(&state, seat, self.commands[index].clone());
            if let Some(freshness) = self.diagnostic_freshness_token {
                command.freshness_token = engine_core::FreshnessToken(freshness);
            }
            let diagnostic = secret_draft::actions::validate_command(&state, &command)
                .expect_err("diagnostic trace rejects");
            diagnostic_hash = Some(HashValue::from_stable_bytes(
                format!("{diagnostic:?}").as_bytes(),
            ));
        }

        Ok(Some(ActualHashes {
            state_hash: secret_draft::state_hash(&state),
            effect_hash: secret_draft::replay_support::effect_hash(&effects),
            action_tree_hash: secret_draft_action_tree_hash(&state),
            view_hash: secret_draft::replay_support::view_hash(&state, &Viewer { seat_id: None }),
            diagnostic_hash,
            terminal: state.phase == secret_draft::Phase::Terminal,
            winner: match state.terminal_outcome {
                Some(secret_draft::TerminalOutcome::Win { seat }) => Some(seat.as_str().to_owned()),
                Some(secret_draft::TerminalOutcome::Draw) | None => None,
            },
        }))
    }

    fn secret_draft_actor(&self, index: usize) -> Result<secret_draft::SecretDraftSeat, String> {
        self.command_actor_seats
            .get(index)
            .and_then(|seat| secret_draft::SecretDraftSeat::parse(seat))
            .ok_or_else(|| self.failure("secret_draft trace command has invalid actor_seat"))
    }

    fn poker_lite_actual_hashes(&self) -> Result<Option<ActualHashes>, String> {
        if self.fixture_kind == "bot_action" {
            return Ok(None);
        }
        if !matches!(
            self.fixture_kind.as_str(),
            "commands" | "terminal" | "diagnostic" | "no_leak" | "export"
        ) {
            return Ok(None);
        }

        let mut state = poker_lite::setup_match(
            engine_core::Seed(self.seed),
            &poker_lite::replay_support::default_seats(),
            &poker_lite::SetupOptions::default(),
        )
        .map_err(diagnostic_string)?;
        let mut effects = poker_lite::setup_effects(&state);
        let mut diagnostic_hash = None;
        let has_diagnostic = self.expected_diagnostic_hash.is_some();
        let applied_count = if has_diagnostic {
            self.commands.len().saturating_sub(1)
        } else {
            self.commands.len()
        };

        for index in 0..applied_count {
            let seat = self.poker_lite_actor(index)?;
            let command = poker_lite_command_for_seat(&state, seat, self.commands[index].clone());
            let action =
                poker_lite::validate_command(&state, &command).map_err(diagnostic_string)?;
            effects
                .extend(poker_lite::apply_action(&mut state, action).map_err(diagnostic_string)?);
        }

        if has_diagnostic {
            let index = self.commands.len().saturating_sub(1);
            let seat = self.poker_lite_actor(index)?;
            let mut command =
                poker_lite_command_for_seat(&state, seat, self.commands[index].clone());
            if let Some(freshness) = self.diagnostic_freshness_token {
                command.freshness_token = engine_core::FreshnessToken(freshness);
            }
            let diagnostic = poker_lite::validate_command(&state, &command)
                .expect_err("diagnostic trace rejects");
            diagnostic_hash = Some(HashValue::from_stable_bytes(
                format!("{diagnostic:?}").as_bytes(),
            ));
        }

        Ok(Some(ActualHashes {
            state_hash: poker_lite::replay_support::state_hash(&state),
            effect_hash: poker_lite::replay_support::effect_hash(&effects),
            action_tree_hash: poker_lite_action_tree_hash(&state),
            view_hash: poker_lite::replay_support::view_hash(&state, &Viewer { seat_id: None }),
            diagnostic_hash,
            terminal: state.phase == poker_lite::Phase::Terminal,
            winner: match state.terminal_outcome {
                Some(poker_lite::TerminalOutcome::YieldWin { winner, .. })
                | Some(poker_lite::TerminalOutcome::ShowdownWin { winner, .. }) => {
                    Some(winner.as_str().to_owned())
                }
                Some(poker_lite::TerminalOutcome::Split { .. }) | None => None,
            },
        }))
    }

    fn poker_lite_actor(&self, index: usize) -> Result<poker_lite::PokerLiteSeat, String> {
        self.command_actor_seats
            .get(index)
            .and_then(|seat| poker_lite::PokerLiteSeat::parse(seat))
            .ok_or_else(|| self.failure("poker_lite trace command has invalid actor_seat"))
    }

    fn plain_tricks_actual_hashes(&self) -> Result<Option<ActualHashes>, String> {
        if self.fixture_kind == "bot_action" {
            return Ok(None);
        }
        if !matches!(
            self.fixture_kind.as_str(),
            "commands" | "terminal" | "diagnostic" | "no_leak" | "export"
        ) {
            return Ok(None);
        }

        let mut state = plain_tricks::setup_match(
            engine_core::Seed(self.seed),
            &plain_tricks::replay_support::default_seats(),
            &plain_tricks::SetupOptions::default(),
        )
        .map_err(diagnostic_string)?;
        let mut effects = plain_tricks::setup_effects(&state);
        let mut diagnostic_hash = None;
        let has_diagnostic = self.expected_diagnostic_hash.is_some();
        let applied_count = if has_diagnostic {
            self.commands.len().saturating_sub(1)
        } else {
            self.commands.len()
        };

        for index in 0..applied_count {
            let seat = self.plain_tricks_actor(index)?;
            let command = plain_tricks_command_for_seat(&state, seat, self.commands[index].clone());
            let action =
                plain_tricks::validate_command(&state, &command).map_err(diagnostic_string)?;
            effects
                .extend(plain_tricks::apply_action(&mut state, action).map_err(diagnostic_string)?);
        }

        if has_diagnostic {
            let index = self.commands.len().saturating_sub(1);
            let seat = self.plain_tricks_actor(index)?;
            let mut command =
                plain_tricks_command_for_seat(&state, seat, self.commands[index].clone());
            if let Some(freshness) = self.diagnostic_freshness_token {
                command.freshness_token = engine_core::FreshnessToken(freshness);
            }
            let diagnostic = plain_tricks::validate_command(&state, &command)
                .expect_err("diagnostic trace rejects");
            diagnostic_hash = Some(HashValue::from_stable_bytes(
                format!("{diagnostic:?}").as_bytes(),
            ));
        }

        Ok(Some(ActualHashes {
            state_hash: plain_tricks::replay_support::state_hash(&state),
            effect_hash: plain_tricks::replay_support::effect_hash(&effects),
            action_tree_hash: plain_tricks_action_tree_hash(&state),
            view_hash: plain_tricks::replay_support::view_hash(&state, &Viewer { seat_id: None }),
            diagnostic_hash,
            terminal: state.phase == plain_tricks::Phase::Terminal,
            winner: match state.terminal_outcome {
                Some(plain_tricks::TerminalOutcome::TrickWin { winner, .. }) => {
                    Some(winner.as_str().to_owned())
                }
                Some(plain_tricks::TerminalOutcome::Split { .. }) | None => None,
            },
        }))
    }

    fn plain_tricks_actor(&self, index: usize) -> Result<plain_tricks::PlainTricksSeat, String> {
        self.command_actor_seats
            .get(index)
            .and_then(|seat| plain_tricks::PlainTricksSeat::parse(seat))
            .ok_or_else(|| self.failure("plain_tricks trace command has invalid actor_seat"))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HighCardDuelReplayHashes {
    state_hash: HashValue,
    effect_hash: HashValue,
    action_tree_hash: HashValue,
    view_hash: HashValue,
    terminal: bool,
    outcome: Option<high_card_duel::TerminalOutcome>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TokenBazaarReplayHashes {
    state_hash: HashValue,
    effect_hash: HashValue,
    action_tree_hash: HashValue,
    view_hash: HashValue,
    terminal: bool,
    outcome: Option<token_bazaar::TerminalOutcome>,
}

fn parse_token_bazaar_trace_seat(value: &str) -> Option<token_bazaar::TokenBazaarSeat> {
    match value {
        "seat_0" => Some(token_bazaar::TokenBazaarSeat::Seat0),
        "seat_1" => Some(token_bazaar::TokenBazaarSeat::Seat1),
        _ => None,
    }
}

fn token_bazaar_command_for_seat(
    state: &token_bazaar::TokenBazaarState,
    actor_seat: token_bazaar::TokenBazaarSeat,
    action_path: Vec<String>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: action_path,
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn token_bazaar_action_tree_hash(state: &token_bazaar::TokenBazaarState) -> HashValue {
    let actor = if state.terminal_outcome.is_some() {
        Actor {
            seat_id: SeatId("terminal".to_owned()),
        }
    } else {
        Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        }
    };
    token_bazaar::action_tree_hash(&token_bazaar::legal_action_tree(state, &actor))
}

fn secret_draft_command_for_seat(
    state: &secret_draft::SecretDraftState,
    actor_seat: secret_draft::SecretDraftSeat,
    path: Vec<String>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        },
        action_path: ActionPath { segments: path },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn secret_draft_action_tree_hash(state: &secret_draft::SecretDraftState) -> HashValue {
    let parts = secret_draft::SecretDraftSeat::ALL
        .iter()
        .map(|seat| {
            let actor = Actor {
                seat_id: state.seats[seat.index()].clone(),
            };
            secret_draft::replay_support::action_tree_hash(&secret_draft::legal_action_tree(
                state, &actor,
            ))
            .0
            .to_string()
        })
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(parts.as_bytes())
}

fn poker_lite_command_for_seat(
    state: &poker_lite::PokerLiteState,
    actor_seat: poker_lite::PokerLiteSeat,
    path: Vec<String>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        },
        action_path: ActionPath { segments: path },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn poker_lite_action_tree_hash(state: &poker_lite::PokerLiteState) -> HashValue {
    let parts = poker_lite::PokerLiteSeat::ALL
        .iter()
        .map(|seat| {
            let actor = Actor {
                seat_id: state.seats[seat.index()].clone(),
            };
            poker_lite::replay_support::action_tree_hash(&poker_lite::legal_action_tree(
                state, &actor,
            ))
            .0
            .to_string()
        })
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(parts.as_bytes())
}

fn plain_tricks_command_for_seat(
    state: &plain_tricks::PlainTricksState,
    actor_seat: plain_tricks::PlainTricksSeat,
    path: Vec<String>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        },
        action_path: ActionPath { segments: path },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn plain_tricks_action_tree_hash(state: &plain_tricks::PlainTricksState) -> HashValue {
    let parts = plain_tricks::PlainTricksSeat::ALL
        .iter()
        .map(|seat| {
            let actor = Actor {
                seat_id: state.seats[seat.index()].clone(),
            };
            plain_tricks::replay_support::action_tree_hash(&plain_tricks::legal_action_tree(
                state, &actor,
            ))
            .0
            .to_string()
        })
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(parts.as_bytes())
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
        "expected_public_export_hashes",
        "expected_private_view_hashes",
        "expected_revealed_sequence",
        "expected_diagnostic_hashes",
        "expected_diagnostics",
        "expected_outcome",
        "expected_terminal_state",
        "expected_resolution",
        "expected_window",
        "expected_bot_rationale",
        "bot_rationales",
        "observer_surface",
        "redacted_command_summaries",
        "public_export_contains_tile_ids",
        "accepted_revealed_tiles",
        "public_no_leak",
        "tiebreak",
        "not_applicable",
        "producer",
        "bot_policy_id",
        "bot_level",
        "bot_seed",
        "public_input_summary",
        "expected_bot_action",
        "expected_public_explanation",
        "expected_private_explanation",
        "actor_seat",
        "setup_patch",
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

fn optional_string_field(input: &str, key: &str) -> Result<Option<String>, String> {
    let needle = format!("\"{key}\":");
    Ok(input.find(&needle).map(|start| {
        parse_string_at(input, start + needle.len())
            .unwrap_or_else(|| panic!("field `{key}` must be a string"))
    }))
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
    if input.contains("\"expected_diagnostic_hashes\":") {
        let tail = &input[input.find("\"expected_diagnostic_hashes\":").unwrap()
            + "\"expected_diagnostic_hashes\":".len()..];
        if tail.trim_start().starts_with("null") {
            return None;
        }
        return optional_hash(input, "expected_diagnostic_hashes", "final")
            .expect("diagnostic hash object parses");
    }
    input.find("\"expected_diagnostics\":").and_then(|start| {
        let tail = &input[start..];
        number_field(tail, "hash").ok()
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

fn action_paths(input: &str) -> Vec<Vec<String>> {
    let mut commands = Vec::new();
    let mut remaining = input;
    while let Some(offset) = remaining.find("\"action_path\":") {
        remaining = &remaining[offset + "\"action_path\":".len()..];
        let open = remaining.find('[').expect("action_path must be an array");
        let close = remaining[open..]
            .find(']')
            .expect("action_path array must close")
            + open;
        commands.push(parse_array_strings(&remaining[open + 1..close]));
        remaining = &remaining[close + 1..];
    }
    commands
}

fn command_string_fields(input: &str, key: &str) -> Vec<String> {
    let command_end = input.find("\"checkpoints\":").unwrap_or(input.len());
    let mut values = Vec::new();
    let mut remaining = &input[..command_end];
    let needle = format!("\"{key}\":");
    while let Some(offset) = remaining.find(&needle) {
        remaining = &remaining[offset + needle.len()..];
        if let Some(value) = parse_string_at(remaining, 0) {
            values.push(value);
        }
    }
    values
}

fn command_with_expect(input: &str, expected_code: &str) -> Option<Vec<String>> {
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

fn last_command_string_field(input: &str, key: &str) -> Option<String> {
    let command_end = input.find("\"checkpoints\":").unwrap_or(input.len());
    let before_checkpoints = &input[..command_end];
    let needle = format!("\"{key}\":");
    let start = before_checkpoints.rfind(&needle)? + needle.len();
    parse_string_at(before_checkpoints, start)
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

fn parse_array_strings(input: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut index = 0;
    while index < input.len() {
        let rest = input[index..].trim_start();
        if rest.is_empty() {
            break;
        }
        let skipped = input[index..].len() - rest.len();
        index += skipped;
        if input[index..].starts_with(',') {
            index += 1;
            continue;
        }
        let value = parse_string_at(input, index).expect("array item must be a string");
        let (_, next) = parse_json_string_at(input, index).expect("array item string parses");
        values.push(value);
        index = next;
    }
    values
}

fn single_segments(commands: &[Vec<String>]) -> Result<Vec<String>, String> {
    commands.iter().map(|path| single_segment(path)).collect()
}

fn single_segment(path: &[String]) -> Result<String, String> {
    if path.len() == 1 {
        Ok(path[0].clone())
    } else {
        Err(format!(
            "expected one-segment command path, got [{}]",
            path.join(",")
        ))
    }
}

fn column_command_for_state(
    state: &column_four::ColumnFourState,
    segment: String,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: vec![segment],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn directional_command_for_state(
    state: &directional_flip::DirectionalFlipState,
    segment: String,
) -> CommandEnvelope {
    directional_command_for_seat(state, state.active_seat, &segment)
}

fn directional_command_for_seat(
    state: &directional_flip::DirectionalFlipState,
    seat: directional_flip::DirectionalFlipSeat,
    segment: &str,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
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

fn draughts_command_for_state(
    state: &draughts_lite::DraughtsLiteState,
    seat: draughts_lite::DraughtsLiteSeat,
    action_path: Vec<String>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: action_path,
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn parse_draughts_trace_seat(value: &str) -> Option<draughts_lite::DraughtsLiteSeat> {
    match value {
        "seat-0" => Some(draughts_lite::DraughtsLiteSeat::Seat0),
        "seat-1" => Some(draughts_lite::DraughtsLiteSeat::Seat1),
        _ => draughts_lite::DraughtsLiteSeat::parse(value),
    }
}

fn high_card_duel_command_for_state(
    state: &high_card_duel::HighCardDuelState,
    action_path: Vec<String>,
) -> Result<CommandEnvelope, String> {
    let actor_seat = high_card_duel::active_commit_seat(state)
        .ok_or_else(|| "non-terminal High Card Duel trace command has no active seat".to_owned())?;
    Ok(high_card_duel_command_for_seat(
        state,
        actor_seat,
        action_path,
    ))
}

fn high_card_duel_command_for_seat(
    state: &high_card_duel::HighCardDuelState,
    seat: high_card_duel::HighCardDuelSeat,
    action_path: Vec<String>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: action_path,
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn parse_high_card_duel_trace_seat(value: &str) -> Option<high_card_duel::HighCardDuelSeat> {
    match value {
        "seat-0" => Some(high_card_duel::HighCardDuelSeat::Seat0),
        "seat-1" => Some(high_card_duel::HighCardDuelSeat::Seat1),
        _ => high_card_duel::HighCardDuelSeat::parse(value),
    }
}

fn high_card_duel_action_tree_hash(state: &high_card_duel::HighCardDuelState) -> HashValue {
    let actor = Actor {
        seat_id: high_card_duel::active_commit_seat(state)
            .map(|seat| state.seats[seat.index()].clone())
            .unwrap_or_else(|| SeatId("seat-0".to_owned())),
    };
    let tree = high_card_duel::legal_action_tree(state, &actor);
    let bytes = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

fn draughts_initial_state(trace_id: &str, seed: u64) -> draughts_lite::DraughtsLiteState {
    match trace_id {
        "draughts-lite-mandatory-capture-suppresses-quiet"
        | "draughts-lite-quiet-while-capture-diagnostic"
        | "draughts-lite-single-capture" => draughts_empty_state(
            draughts_lite::DraughtsLiteSeat::Seat0,
            vec![
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat0, 1, 3, 2),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 1, 4, 3),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 2, 8, 7),
            ],
        ),
        "draughts-lite-multi-jump" | "draughts-lite-illegal-continuation-diagnostic" => {
            draughts_empty_state(
                draughts_lite::DraughtsLiteSeat::Seat0,
                vec![
                    draughts_man(draughts_lite::DraughtsLiteSeat::Seat0, 1, 3, 2),
                    draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 1, 4, 3),
                    draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 2, 6, 5),
                    draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 3, 8, 7),
                ],
            )
        }
        "draughts-lite-forced-continuation-branch" => draughts_empty_state(
            draughts_lite::DraughtsLiteSeat::Seat0,
            vec![
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat0, 1, 3, 2),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 1, 4, 3),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 2, 6, 3),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 3, 6, 5),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 4, 8, 7),
            ],
        ),
        "draughts-lite-promotion-quiet" => draughts_empty_state(
            draughts_lite::DraughtsLiteSeat::Seat0,
            vec![
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat0, 1, 7, 2),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 1, 6, 7),
            ],
        ),
        "draughts-lite-promotion-during-capture-stop"
        | "draughts-lite-path-after-promotion-stop-diagnostic" => draughts_empty_state(
            draughts_lite::DraughtsLiteSeat::Seat0,
            vec![
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat0, 1, 6, 3),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 1, 7, 4),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 2, 7, 6),
            ],
        ),
        "draughts-lite-terminal-no-pieces" => draughts_empty_state(
            draughts_lite::DraughtsLiteSeat::Seat0,
            vec![
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat0, 1, 3, 2),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 1, 4, 3),
            ],
        ),
        "draughts-lite-terminal-no-legal-moves" => draughts_empty_state(
            draughts_lite::DraughtsLiteSeat::Seat0,
            vec![
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat0, 1, 1, 2),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat0, 2, 3, 4),
                draughts_man(draughts_lite::DraughtsLiteSeat::Seat1, 1, 2, 1),
            ],
        ),
        _ => draughts_lite::setup_match(
            Seed(seed),
            &[SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())],
            &draughts_lite::SetupOptions::default(),
        )
        .expect("draughts setup succeeds"),
    }
}

fn draughts_coord(row: u8, col: u8) -> game_stdlib::board_space::Coord {
    game_stdlib::board_space::Coord::checked(row, col).unwrap()
}

fn draughts_piece_id(
    owner: draughts_lite::DraughtsLiteSeat,
    ordinal: u8,
) -> draughts_lite::PieceId {
    draughts_lite::PieceId::new(owner, ordinal).unwrap()
}

fn draughts_man(
    owner: draughts_lite::DraughtsLiteSeat,
    ordinal: u8,
    row: u8,
    col: u8,
) -> draughts_lite::Piece {
    draughts_lite::Piece {
        id: draughts_piece_id(owner, ordinal),
        owner,
        kind: draughts_lite::PieceKind::Man,
        cell: draughts_coord(row, col),
    }
}

fn draughts_empty_state(
    active_seat: draughts_lite::DraughtsLiteSeat,
    mut pieces: Vec<draughts_lite::Piece>,
) -> draughts_lite::DraughtsLiteState {
    let board = draughts_lite::ids::board_dimensions();
    pieces.sort_by_key(|piece| piece.id);
    let mut cells = draughts_lite::DraughtsLiteState::empty_cells();
    for piece in &pieces {
        cells[piece.cell.row_col_index(board).unwrap()] =
            draughts_lite::CellOccupancy::Occupied(piece.id);
    }

    draughts_lite::DraughtsLiteState {
        variant: draughts_lite::Variant::draughts_lite_standard(),
        board,
        cells,
        pieces,
        active_seat,
        seats: [SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())],
        ply_count: 0,
        command_count: 0,
        terminal_outcome: None,
        terminal_reason: None,
        freshness_token: engine_core::FreshnessToken(0),
    }
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
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let mut state =
        directional_flip::setup_match(Seed(1), &seats, &directional_flip::SetupOptions::default())
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

fn diagnostic_string(diagnostic: engine_core::Diagnostic) -> String {
    format!("{}: {}", diagnostic.code, diagnostic.message)
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
        let corrupted = VALID.replace("10275940640358619244", "10275940640358619245");
        let trace = Trace::parse(Path::new("shortest-normal.trace.json"), &corrupted).unwrap();
        let error = trace.check(resolve_game("race_to_n").unwrap()).unwrap_err();

        assert!(error.contains("state hash drift"));
        assert!(error.contains("expected: 10275940640358619245"));
        assert!(error.contains("actual: 10275940640358619244"));
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
