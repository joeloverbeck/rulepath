use std::{collections::BTreeMap, env, fs, path::PathBuf, process};

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let config = Config::parse(args)?;
    let report_input = fs::read_to_string(&config.input)
        .map_err(|error| format!("{}: failed to read report: {error}", config.input.display()))?;
    let thresholds_input = fs::read_to_string(&config.thresholds).map_err(|error| {
        format!(
            "{}: failed to read thresholds: {error}",
            config.thresholds.display()
        )
    })?;

    let report = Report::parse(&report_input)?;
    let thresholds = ThresholdSet::parse(&thresholds_input)?;
    if let Some(game) = &config.game {
        let registered = resolve_game(game)?;
        if report.game_id != registered.game_id {
            return Err(format!(
                "bench-report: --game `{}` does not match report game_id `{}`",
                registered.game_id, report.game_id
            ));
        }
    }
    validate_report(&report, &thresholds)?;

    println!(
        "bench-report: {} operations passed thresholds for {}",
        report.operations.len(),
        report.game_id
    );
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    input: PathBuf,
    thresholds: PathBuf,
    game: Option<String>,
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

        let mut input = None;
        let mut thresholds = None;
        let mut game = None;
        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--input" => input = Some(PathBuf::from(next_arg(&mut iter, "--input")?)),
                "--thresholds" => {
                    thresholds = Some(PathBuf::from(next_arg(&mut iter, "--thresholds")?));
                }
                "--game" => game = Some(next_arg(&mut iter, "--game")?),
                other => return Err(format!("unknown argument `{other}`")),
            }
        }

        let thresholds = match (thresholds, game.as_deref()) {
            (Some(path), _) => path,
            (None, Some(game)) => PathBuf::from(resolve_game(game)?.thresholds_path),
            (None, None) => {
                return Err("--thresholds is required unless --game is supplied".to_owned())
            }
        };

        Ok(Self {
            input: input.ok_or_else(|| "--input is required".to_owned())?,
            thresholds,
            game,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RegisteredGame {
    game_id: &'static str,
    thresholds_path: &'static str,
}

fn resolve_game(game: &str) -> Result<RegisteredGame, String> {
    match game {
        "race_to_n" => Ok(RegisteredGame {
            game_id: "race_to_n",
            thresholds_path: "games/race_to_n/benches/thresholds.json",
        }),
        "column_four" => Ok(RegisteredGame {
            game_id: "column_four",
            thresholds_path: "games/column_four/benches/thresholds.json",
        }),
        "directional_flip" => Ok(RegisteredGame {
            game_id: "directional_flip",
            thresholds_path: "games/directional_flip/benches/thresholds.json",
        }),
        _ => Err(format!("unsupported game `{game}`")),
    }
}

fn next_arg(iter: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    iter.next()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn print_help() {
    println!("bench-report 0.1.0");
    println!("usage: bench-report --input <report> --thresholds <thresholds>");
    println!(
        "       bench-report --game <race_to_n|column_four|directional_flip> --input <report>"
    );
}

#[derive(Clone, Debug)]
struct Report {
    schema_version: u64,
    game_id: String,
    rules_version: String,
    data_version: String,
    engine_version: String,
    build_profile: String,
    command: String,
    os: String,
    rust_version: String,
    hardware_environment_notes: String,
    operations: Vec<Operation>,
}

#[derive(Clone, Debug)]
struct Operation {
    operation_name: String,
    unit: String,
    current_value: f64,
}

#[derive(Clone, Debug)]
struct ThresholdSet {
    schema_version: u64,
    game_id: String,
    rules_version: String,
    data_version: String,
    engine_version: String,
    thresholds: Vec<Threshold>,
}

#[derive(Clone, Debug)]
struct Threshold {
    operation_name: String,
    unit: String,
    threshold: f64,
    rationale_class: String,
    rationale: String,
}

impl Report {
    fn parse(input: &str) -> Result<Self, String> {
        let json = extract_json_object(input).ok_or_else(|| "report: malformed JSON".to_owned())?;
        validate_json_nesting(&json).map_err(|error| format!("report: {error}"))?;
        let operations = array_objects(&json, "operations")?
            .into_iter()
            .map(|object| {
                Ok(Operation {
                    operation_name: required_string(&object, "operation_name", "report operation")?,
                    unit: required_string(&object, "unit", "report operation")?,
                    current_value: required_number(&object, "current_value", "report operation")?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        let report = Self {
            schema_version: required_integer(&json, "schema_version", "report")?,
            game_id: required_string(&json, "game_id", "report")?,
            rules_version: required_string(&json, "rules_version", "report")?,
            data_version: required_string(&json, "data_version", "report")?,
            engine_version: required_string(&json, "engine_version", "report")?,
            build_profile: required_string(&json, "build_profile", "report")?,
            command: required_string(&json, "command", "report")?,
            os: required_string(&json, "os", "report")?,
            rust_version: required_string(&json, "rust_version", "report")?,
            hardware_environment_notes: required_string(
                &json,
                "hardware_environment_notes",
                "report",
            )?,
            operations,
        };
        if report.operations.is_empty() {
            return Err("report: operations must be non-empty".to_owned());
        }
        Ok(report)
    }
}

impl ThresholdSet {
    fn parse(input: &str) -> Result<Self, String> {
        let json =
            extract_json_object(input).ok_or_else(|| "thresholds: malformed JSON".to_owned())?;
        validate_json_nesting(&json).map_err(|error| format!("thresholds: {error}"))?;
        let thresholds = array_objects(&json, "thresholds")?
            .into_iter()
            .map(|object| {
                Ok(Threshold {
                    operation_name: required_string(&object, "operation_name", "threshold")?,
                    unit: required_string(&object, "unit", "threshold")?,
                    threshold: required_number(&object, "threshold", "threshold")?,
                    rationale_class: required_string(&object, "rationale_class", "threshold")?,
                    rationale: required_string(&object, "rationale", "threshold")?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        let set = Self {
            schema_version: required_integer(&json, "schema_version", "thresholds")?,
            game_id: required_string(&json, "game_id", "thresholds")?,
            rules_version: required_string(&json, "rules_version", "thresholds")?,
            data_version: required_string(&json, "data_version", "thresholds")?,
            engine_version: required_string(&json, "engine_version", "thresholds")?,
            thresholds,
        };
        if set.thresholds.is_empty() {
            return Err("thresholds: thresholds must be non-empty".to_owned());
        }
        Ok(set)
    }
}

fn validate_report(report: &Report, thresholds: &ThresholdSet) -> Result<(), String> {
    if report.schema_version != 1 || thresholds.schema_version != 1 {
        return Err("bench-report: schema_version must be 1".to_owned());
    }
    for (field, report_value, threshold_value) in [
        ("game_id", &report.game_id, &thresholds.game_id),
        (
            "rules_version",
            &report.rules_version,
            &thresholds.rules_version,
        ),
        (
            "data_version",
            &report.data_version,
            &thresholds.data_version,
        ),
        (
            "engine_version",
            &report.engine_version,
            &thresholds.engine_version,
        ),
    ] {
        if report_value != threshold_value {
            return Err(format!(
                "bench-report: {field} mismatch: report `{report_value}`, thresholds `{threshold_value}`"
            ));
        }
    }
    for (field, value) in [
        ("build_profile", &report.build_profile),
        ("command", &report.command),
        ("os", &report.os),
        ("rust_version", &report.rust_version),
        (
            "hardware_environment_notes",
            &report.hardware_environment_notes,
        ),
    ] {
        if value.trim().is_empty() {
            return Err(format!(
                "bench-report: required metadata `{field}` is empty"
            ));
        }
    }

    let operations = report
        .operations
        .iter()
        .map(|operation| (operation.operation_name.as_str(), operation))
        .collect::<BTreeMap<_, _>>();
    let mut failures = Vec::new();
    for threshold in &thresholds.thresholds {
        let Some(operation) = operations.get(threshold.operation_name.as_str()) else {
            failures.push(format!(
                "missing operation {}\nthreshold: {:.2}\nrationale: {} ({})\nenvironment caveat: {}",
                threshold.operation_name,
                threshold.threshold,
                threshold.rationale_class,
                threshold.rationale,
                report.hardware_environment_notes
            ));
            continue;
        };
        if operation.unit != threshold.unit {
            failures.push(format!(
                "operation {} unit mismatch: report {}, threshold {}",
                threshold.operation_name, operation.unit, threshold.unit
            ));
            continue;
        }
        if operation.current_value < threshold.threshold {
            failures.push(format!(
                "operation {} below threshold\ncurrent value: {:.2}\nthreshold: {:.2}\nrationale: {} ({})\nenvironment caveat: {}",
                threshold.operation_name,
                operation.current_value,
                threshold.threshold,
                threshold.rationale_class,
                threshold.rationale,
                report.hardware_environment_notes
            ));
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!("bench-report failure\n{}", failures.join("\n\n")))
    }
}

fn extract_json_object(input: &str) -> Option<String> {
    if let Some(body) = extract_marked_json(input) {
        return Some(body);
    }
    let trimmed = input.trim();
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        Some(trimmed.to_owned())
    } else {
        None
    }
}

/// Pull the JSON body out of a `BEGIN_<GAME>_BENCHMARK_JSON` / `END_<GAME>_BENCHMARK_JSON`
/// block. The `<GAME>` token varies per game (`RACE_TO_N`, `THREE_MARKS`, ...), so the
/// markers are matched structurally rather than by a fixed game name.
fn extract_marked_json(input: &str) -> Option<String> {
    const SUFFIX: &str = "_BENCHMARK_JSON";
    let (_, begin_end) = marker_position(input, "BEGIN_", SUFFIX, 0)?;
    let (end_start, _) = marker_position(input, "END_", SUFFIX, begin_end)?;
    Some(input[begin_end..end_start].trim().to_owned())
}

/// Find a `<prefix><token><suffix>` marker at or after `from`, where `<token>` is a
/// contiguous, whitespace-free game identifier. Returns the marker's (start, end) byte
/// offsets. The whitespace check rejects an unrelated earlier `prefix` occurrence.
fn marker_position(input: &str, prefix: &str, suffix: &str, from: usize) -> Option<(usize, usize)> {
    let mut search = from;
    while let Some(rel) = input[search..].find(prefix) {
        let start = search + rel;
        let token_start = start + prefix.len();
        if let Some(srel) = input[token_start..].find(suffix) {
            let token = &input[token_start..token_start + srel];
            if !token.is_empty() && !token.contains(char::is_whitespace) {
                return Some((start, token_start + srel + suffix.len()));
            }
        }
        search = token_start;
    }
    None
}

fn validate_json_nesting(input: &str) -> Result<(), String> {
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
    if depth == 0 && !in_string {
        Ok(())
    } else {
        Err("malformed JSON nesting".to_owned())
    }
}

fn array_objects(input: &str, key: &str) -> Result<Vec<String>, String> {
    let body = array_body(input, key)?;
    let mut objects = Vec::new();
    let mut start = None;
    let mut depth = 0_u32;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, ch) in body.char_indices() {
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
            '{' => {
                if depth == 0 {
                    start = Some(offset);
                }
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let start = start.expect("object start recorded");
                    objects.push(body[start..=offset].to_owned());
                }
            }
            _ => {}
        }
    }
    Ok(objects)
}

fn array_body(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing array `{key}`"))?
        + needle.len();
    let open = input[start..]
        .find('[')
        .ok_or_else(|| format!("field `{key}` must be an array"))?
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
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(input[open + 1..open + offset].to_owned());
                }
            }
            _ => {}
        }
    }
    Err(format!("array `{key}` must close"))
}

fn required_string(input: &str, key: &str, context: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("{context}: missing field `{key}`"))?
        + needle.len();
    parse_string_at(input, start).ok_or_else(|| format!("{context}: field `{key}` must be string"))
}

fn required_integer(input: &str, key: &str, context: &str) -> Result<u64, String> {
    let value = required_number(input, key, context)?;
    if value.fract() == 0.0 {
        Ok(value as u64)
    } else {
        Err(format!("{context}: field `{key}` must be integer"))
    }
}

fn required_number(input: &str, key: &str, context: &str) -> Result<f64, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("{context}: missing field `{key}`"))?
        + needle.len();
    parse_number_at(input, start).ok_or_else(|| format!("{context}: field `{key}` must be number"))
}

fn parse_string_at(input: &str, start: usize) -> Option<String> {
    let tail = input[start..].trim_start();
    let tail = tail.strip_prefix('"')?;
    let end = tail.find('"')?;
    Some(tail[..end].to_owned())
}

fn parse_number_at(input: &str, start: usize) -> Option<f64> {
    let tail = input[start..].trim_start();
    let number = tail
        .chars()
        .take_while(|ch| ch.is_ascii_digit() || matches!(ch, '.'))
        .collect::<String>();
    if number.is_empty() {
        None
    } else {
        number.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const THRESHOLDS: &str = include_str!("../../../games/race_to_n/benches/thresholds.json");

    fn valid_report() -> String {
        let operations = [
            ("legal_actions", "trees_per_second", 1_000_001.0),
            ("apply_action", "actions_per_second", 5_000_001.0),
            ("public_view_generation", "views_per_second", 10_000_001.0),
            ("effect_filtering", "filters_per_second", 10_000_001.0),
            (
                "serialization_roundtrip",
                "roundtrips_per_second",
                200_001.0,
            ),
            ("replay_throughput", "replays_per_second", 250_001.0),
            ("random_playout", "games_per_second", 500_001.0),
            ("bot_decision", "decisions_per_second", 1_000_001.0),
        ]
        .into_iter()
        .map(|(name, unit, value)| {
            format!(
                "{{\"operation_name\":\"{name}\",\"unit\":\"{unit}\",\"current_value\":{value}}}"
            )
        })
        .collect::<Vec<_>>()
        .join(",");

        format!(
            concat!(
                "{{",
                "\"schema_version\":1,",
                "\"game_id\":\"race_to_n\",",
                "\"rules_version\":\"race_to_n-rules-v1\",",
                "\"data_version\":\"1\",",
                "\"engine_version\":\"engine-core-0.1.0\",",
                "\"build_profile\":\"bench\",",
                "\"command\":\"cargo bench -p race_to_n\",",
                "\"os\":\"linux x86_64\",",
                "\"rust_version\":\"rustc test\",",
                "\"hardware_environment_notes\":\"test environment\",",
                "\"operations\":[{}]",
                "}}"
            ),
            operations
        )
    }

    fn validate(input: &str) -> Result<(), String> {
        let report = Report::parse(input)?;
        let thresholds = ThresholdSet::parse(THRESHOLDS)?;
        validate_report(&report, &thresholds)
    }

    #[test]
    fn valid_report_passes() {
        validate(&valid_report()).unwrap();
    }

    #[test]
    fn regressed_operation_fails() {
        let input = valid_report().replace(
            "{\"operation_name\":\"random_playout\",\"unit\":\"games_per_second\",\"current_value\":500001}",
            "{\"operation_name\":\"random_playout\",\"unit\":\"games_per_second\",\"current_value\":1}",
        );
        let error = validate(&input).unwrap_err();

        let random_playout_threshold = ThresholdSet::parse(THRESHOLDS)
            .unwrap()
            .thresholds
            .into_iter()
            .find(|threshold| threshold.operation_name == "random_playout")
            .expect("random_playout threshold present")
            .threshold;

        assert!(error.contains("random_playout"));
        assert!(error.contains("current value: 1.00"));
        assert!(error.contains(&format!("threshold: {random_playout_threshold:.2}")));
        assert!(error.contains("accepted_adr"));
    }

    #[test]
    fn malformed_report_fails() {
        let error = Report::parse("{ nope").unwrap_err();

        assert!(error.contains("malformed JSON"));
    }

    #[test]
    fn missing_metadata_fails() {
        let input = valid_report().replace("\"rust_version\":\"rustc test\",", "");
        let error = validate(&input).unwrap_err();

        assert!(error.contains("missing field `rust_version`"));
    }

    #[test]
    fn missing_operation_fails() {
        let input = valid_report().replace(
            "{\"operation_name\":\"bot_decision\",\"unit\":\"decisions_per_second\",\"current_value\":1000001}",
            "",
        );
        let error = validate(&input).unwrap_err();

        assert!(error.contains("missing operation bot_decision"));
    }

    #[test]
    fn marked_harness_output_is_accepted() {
        let input = format!(
            "human summary\nBEGIN_RACE_TO_N_BENCHMARK_JSON\n{}\nEND_RACE_TO_N_BENCHMARK_JSON\n",
            valid_report()
        );

        validate(&input).unwrap();
    }

    #[test]
    fn marked_harness_output_is_accepted_for_other_games() {
        let input = format!(
            "human summary\nBEGIN_THREE_MARKS_BENCHMARK_JSON\n{}\nEND_THREE_MARKS_BENCHMARK_JSON\n",
            valid_report()
        );

        validate(&input).unwrap();
    }
}
