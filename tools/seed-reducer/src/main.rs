use std::{env, fs, path::PathBuf, process};

use race_to_n::replay_support::replay_commands;

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let config = Config::parse(args)?;
    if config.game != "race_to_n" {
        return Err(format!("unsupported game `{}`", config.game));
    }

    let input = if let Some(path) = &config.failure_report {
        ReducerInput::from_failure_report(path)?
    } else {
        ReducerInput::from_explicit(config.seed, config.commands)?
    };

    println!("{}", reduction_report(&input));
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    game: String,
    failure_report: Option<PathBuf>,
    seed: Option<u64>,
    commands: Option<Vec<String>>,
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
        let mut failure_report = None;
        let mut seed = None;
        let mut commands = None;
        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--game" => game = Some(next_arg(&mut iter, "--game")?),
                "--failure-report" => {
                    failure_report = Some(PathBuf::from(next_arg(&mut iter, "--failure-report")?));
                }
                "--seed" => seed = Some(parse_u64(&mut iter, "--seed")?),
                "--commands" => {
                    commands = Some(parse_commands(&next_arg(&mut iter, "--commands")?))
                }
                other => return Err(format!("unknown argument `{other}`")),
            }
        }

        if failure_report.is_some() && (seed.is_some() || commands.is_some()) {
            return Err("--failure-report cannot be combined with --seed/--commands".to_owned());
        }
        if failure_report.is_none() && (seed.is_none() || commands.is_none()) {
            return Err(
                "provide either --failure-report <path> or --seed <n> --commands <stream>"
                    .to_owned(),
            );
        }

        Ok(Self {
            game: game.ok_or_else(|| "--game is required".to_owned())?,
            failure_report,
            seed,
            commands,
        })
    }
}

fn print_help() {
    println!("seed-reducer 0.1.0");
    println!("usage:");
    println!("  seed-reducer --game race_to_n --failure-report <path>");
    println!("  seed-reducer --game race_to_n --seed <n> --commands <comma-stream>");
}

fn next_arg(iter: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    iter.next()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn parse_u64(iter: &mut impl Iterator<Item = String>, flag: &str) -> Result<u64, String> {
    next_arg(iter, flag)?
        .parse()
        .map_err(|_| format!("{flag} requires an unsigned integer"))
}

fn parse_commands(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReducerInput {
    source: String,
    seed: u64,
    action_cap: u64,
    variant: String,
    game_id: String,
    rules_version: String,
    data_version: String,
    engine_version: String,
    command_stream: Vec<String>,
    failure_reason: String,
    replay_command: String,
}

impl ReducerInput {
    fn from_failure_report(path: &PathBuf) -> Result<Self, String> {
        let input = fs::read_to_string(path).map_err(|error| {
            format!("{}: failed to read failure report: {error}", path.display())
        })?;
        validate_json_object(&input).map_err(|error| format!("{}: {error}", path.display()))?;
        let game_id = required_string(&input, "game_id")?;
        let seed = required_number(&input, "seed")?;
        let variant = required_string(&input, "variant")?;
        let action_cap = number_field(&object_body(&input, "options")?, "action_cap")?;
        let replay_command = if required_string(&input, "failure_reason")? == "injected failure" {
            format!(
                "cargo run -p simulate -- --game race_to_n --games 1 --start-seed {seed} --action-cap {action_cap} --inject-failure-seed {seed}"
            )
        } else {
            required_string(&input, "replay_command")?
        };
        Ok(Self {
            source: path.display().to_string(),
            seed,
            action_cap,
            variant,
            game_id,
            rules_version: required_string(&input, "rules_version")?,
            data_version: required_string(&input, "data_version")?,
            engine_version: required_string(&input, "engine_version")?,
            command_stream: string_array(&input, "command_stream")?,
            failure_reason: required_string(&input, "failure_reason")?,
            replay_command,
        })
    }

    fn from_explicit(seed: Option<u64>, commands: Option<Vec<String>>) -> Result<Self, String> {
        let seed = seed.expect("validated by Config::parse");
        let command_stream = commands.expect("validated by Config::parse");
        Ok(Self {
            source: "explicit-arguments".to_owned(),
            seed,
            action_cap: 64,
            variant: "race_to_21".to_owned(),
            game_id: "race_to_n".to_owned(),
            rules_version: "race_to_n-rules-v1".to_owned(),
            data_version: "1".to_owned(),
            engine_version: "engine-core-0.1.0".to_owned(),
            command_stream,
            failure_reason: "explicit command stream supplied without failure predicate".to_owned(),
            replay_command: format!(
                "cargo run -p simulate -- --game race_to_n --games 1 --start-seed {seed} --action-cap 64"
            ),
        })
    }
}

fn reduction_report(input: &ReducerInput) -> String {
    let trace_status = if input.command_stream.is_empty() {
        "trace_reproducer_status=not_emitted_empty_command_stream\n".to_owned()
    } else {
        format!(
            "trace_reproducer_json={}\n",
            compact_trace_reproducer_json(input)
        )
    };
    format!(
        "seed-reducer v0 report\n\
         source={}\n\
         game_id={}\n\
         seed={}\n\
         variant={}\n\
         command_stream={}\n\
         normalized_simulate_command={}\n\
         minimization_status=unavailable_without_failure_predicate\n\
         minimization_note=v0 preserves the exact reproducer and does not claim delta-debugging.\n\
         failure_reason={}\n\
         {trace_status}",
        input.source,
        input.game_id,
        input.seed,
        input.variant,
        if input.command_stream.is_empty() {
            "[]".to_owned()
        } else {
            input.command_stream.join(",")
        },
        input.replay_command,
        input.failure_reason
    )
}

fn compact_trace_reproducer_json(input: &ReducerInput) -> String {
    let actions = input
        .command_stream
        .iter()
        .map(|command| split_actor_command(command).1.to_owned())
        .collect::<Vec<_>>();
    let hashes = replay_commands(input.seed, &actions);
    let terminal = hashes.outcome.is_some();
    let winner = hashes
        .outcome
        .map(|winner| format!("\"{}\"", winner.as_str()))
        .unwrap_or_else(|| "null".to_owned());
    let commands = input
        .command_stream
        .iter()
        .enumerate()
        .map(|(index, command)| {
            let (_, action) = split_actor_command(command);
            format!(
                "{{\"index\":{},\"actor_seat\":\"{}\",\"action_path\":[\"{}\"],\"freshness_token\":\"unknown\",\"expect\":\"applied\"}}",
                index,
                escape_json(split_actor_command(command).0),
                escape_json(action)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "{{",
            "\"schema_version\":1,",
            "\"trace_id\":\"seed-reducer-{}\",",
            "\"fixture_kind\":\"commands\",",
            "\"purpose\":\"seed_reducer_reproducer\",",
            "\"note\":\"Normalized v0 seed-reducer reproducer; minimization unavailable without predicate.\",",
            "\"migration_update_note\":\"Generated from seed-reducer v0 input with expected hashes from race_to_n replay support.\",",
            "\"game_id\":\"{}\",",
            "\"rules_version\":\"{}\",",
            "\"engine_version\":\"{}\",",
            "\"data_version\":\"{}\",",
            "\"seed\":{},",
            "\"variant\":\"{}\",",
            "\"options\":{{\"action_cap\":{}}},",
            "\"seats\":[{{\"seat_id\":\"seat-0\",\"player_id\":\"player-0\"}},{{\"seat_id\":\"seat-1\",\"player_id\":\"player-1\"}}],",
            "\"commands\":[{}],",
            "\"checkpoints\":[{{\"id\":\"final\",\"after_command_index\":{}}}],",
            "\"expected_state_hashes\":{{\"final\":{}}},",
            "\"expected_effect_hashes\":{{\"final\":{}}},",
            "\"expected_action_tree_hashes\":{{\"final\":{}}},",
            "\"expected_public_view_hashes\":{{\"all\":{}}},",
            "\"expected_private_view_hashes\":{{\"not_applicable\":\"race_to_n is perfect-information.\"}},",
            "\"expected_outcome\":{{\"terminal\":{},\"winner\":{}}},",
            "\"expected_terminal_state\":{{\"terminal\":{},\"winner\":{}}},",
            "\"not_applicable\":{{\"hidden_information\":\"race_to_n is perfect-information.\",\"stochastic_game_events\":\"race_to_n game rules use no randomness.\",\"private_view_hashes\":\"race_to_n has no private-view API.\",\"preview_hashes\":\"race_to_n has no Rust preview surface in Gate 2.\"}}",
            "}}"
        ),
        input.seed,
        input.game_id,
        input.rules_version,
        input.engine_version,
        input.data_version,
        input.seed,
        input.variant,
        input.action_cap,
        commands,
        input.command_stream.len().saturating_sub(1),
        hashes.state_hash.0,
        hashes.effect_hash.0,
        hashes.action_tree_hash.0,
        hashes.view_hash.0,
        terminal,
        &winner,
        terminal,
        &winner
    )
}

fn split_actor_command(command: &str) -> (&str, &str) {
    command.split_once(':').unwrap_or(("seat_0", command))
}

fn validate_json_object(input: &str) -> Result<(), String> {
    let trimmed = input.trim();
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        Ok(())
    } else {
        Err("malformed JSON object".to_owned())
    }
}

fn required_string(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing field `{key}`"))?
        + needle.len();
    parse_string_at(input, start).ok_or_else(|| format!("field `{key}` must be a string"))
}

fn required_number(input: &str, key: &str) -> Result<u64, String> {
    number_field(input, key)
}

fn number_field(input: &str, key: &str) -> Result<u64, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing field `{key}`"))?
        + needle.len();
    parse_number_at(input, start).ok_or_else(|| format!("field `{key}` must be a number"))
}

fn object_body(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing object `{key}`"))?
        + needle.len();
    let open = input[start..]
        .find('{')
        .ok_or_else(|| format!("field `{key}` must be object"))?
        + start;
    let close = input[open..]
        .find('}')
        .ok_or_else(|| format!("object `{key}` must close"))?
        + open;
    Ok(input[open..=close].to_owned())
}

fn string_array(input: &str, key: &str) -> Result<Vec<String>, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing array `{key}`"))?
        + needle.len();
    let open = input[start..]
        .find('[')
        .ok_or_else(|| format!("field `{key}` must be array"))?
        + start;
    let close = input[open..]
        .find(']')
        .ok_or_else(|| format!("array `{key}` must close"))?
        + open;
    let body = &input[open + 1..close];
    let mut values = Vec::new();
    let mut rest = body;
    while let Some(offset) = rest.find('"') {
        let start = offset;
        let value = parse_string_at(rest, start).expect("array string parses");
        values.push(value);
        rest = &rest[start + 1..];
        let consumed = rest.find('"').expect("closing quote exists") + 1;
        rest = &rest[consumed..];
    }
    Ok(values)
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

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    const FAILURE_REPORT: &str = r#"{
  "schema_version": 1,
  "report_kind": "simulation_failure",
  "game_id": "race_to_n",
  "rules_version": "race_to_n-rules-v1",
  "data_version": "1",
  "engine_version": "engine-core-0.1.0",
  "seed": 7,
  "options": {"variant": "race_to_21", "action_cap": 64},
  "variant": "race_to_21",
  "command_stream": [],
  "state_hash": "18176667065230576156",
  "effect_hash": "14695981039346656037",
  "view_hash": "16970779661227325598",
  "failure_reason": "injected failure",
  "replay_command": "cargo run -p simulate -- --game race_to_n --games 1 --start-seed 7 --action-cap 64"
}"#;

    #[test]
    fn failure_report_normalizes_without_claiming_minimization() {
        let path = PathBuf::from("/tmp/seed-reducer-test-report.json");
        fs::write(&path, FAILURE_REPORT).unwrap();
        let input = ReducerInput::from_failure_report(&path).unwrap();
        let report = reduction_report(&input);

        assert!(report.contains("normalized_simulate_command=cargo run -p simulate"));
        assert!(report.contains("--inject-failure-seed 7"));
        assert!(report.contains("minimization_status=unavailable_without_failure_predicate"));
        assert!(report.contains("trace_reproducer_status=not_emitted_empty_command_stream"));
    }

    #[test]
    fn explicit_command_stream_emits_trace_reproducer_json() {
        let input = ReducerInput::from_explicit(
            Some(9),
            Some(vec!["seat_0:add-1".to_owned(), "seat_1:add-2".to_owned()]),
        )
        .unwrap();
        let report = reduction_report(&input);

        assert!(report.contains("trace_reproducer_json={"));
        assert!(report.contains("\"trace_id\":\"seed-reducer-9\""));
        assert!(report.contains("\"action_path\":[\"add-1\"]"));
        assert!(report.contains("\"expected_state_hashes\":{\"final\":"));
        assert!(report.contains("minimization_status=unavailable_without_failure_predicate"));
    }
}
