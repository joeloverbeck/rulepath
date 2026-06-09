use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    process,
};

const BEHAVIOR_KEYS: &[&str] = &[
    "when",
    "if",
    "then",
    "else",
    "selector",
    "condition",
    "trigger",
    "script",
    "loop",
    "foreach",
    "priority_expression",
    "ai_condition",
    "effect_script",
    "rule",
    "requires",
    "valid_if",
    "on_play",
    "on_reveal",
];

const ALLOWED_JSON_KEYS: &[&str] = &[
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
    "expected_public_export_hashes",
    "expected_revealed_sequence",
    "expected_diagnostic_hashes",
    "expected_diagnostics",
    "expected_outcome",
    "expected_terminal_state",
    "not_applicable",
    "seat_id",
    "player_id",
    "index",
    "actor_seat",
    "action_path",
    "freshness_token",
    "expect",
    "expected_diagnostic_code",
    "producer",
    "kind",
    "level",
    "bot_policy",
    "bot_policy_id",
    "bot_policy_version",
    "bot_seed",
    "bot_level",
    "policy_id",
    "public_input_summary",
    "expected_bot_action",
    "expected_public_explanation",
    "expected_private_explanation",
    "opponent_private_card",
    "opponent_hand",
    "hidden_center",
    "deck_tail",
    "tail",
    "sampling",
    "wasm_exported_trace",
    "id",
    "after_command_index",
    "final",
    "all",
    "observer",
    "seat_0",
    "seat_1",
    "command_index",
    "code",
    "hash",
    "terminal",
    "winner",
    "draw",
    "kind",
    "hidden_information",
    "stochastic_game_events",
    "private_view_hashes",
    "preview_hashes",
    "action_cap",
    "setup_patch",
    "export_class",
    "viewer",
    "steps",
    "step_index",
    "public_view_summary",
    "public_effects",
    "redacted_command_summary",
];

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let config = Config::parse(args)?;
    let game = resolve_game(&config.game)?;

    let mut failures = Vec::new();
    let mut seen_ids = HashSet::new();

    if let Some(trace) = &config.trace {
        collect(
            validate_trace_path(game, trace, &mut seen_ids),
            &mut failures,
        );
    } else {
        collect(validate_static_data(game), &mut failures);
        collect(reject_yaml_paths(game), &mut failures);
        for path in trace_paths(game)? {
            collect(
                validate_trace_path(game, &path, &mut seen_ids),
                &mut failures,
            );
        }
    }

    if failures.is_empty() {
        println!("fixture-check: all fixtures passed");
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
    fixture_dir: &'static str,
    manifest_path: &'static str,
    variants_path: &'static str,
    variant_id: &'static str,
}

fn resolve_game(game: &str) -> Result<RegisteredGame, String> {
    match game {
        "race_to_n" => Ok(RegisteredGame {
            game_id: "race_to_n",
            rules_version: "race_to_n-rules-v1",
            trace_dir: "games/race_to_n/tests/golden_traces",
            fixture_dir: "games/race_to_n/data/fixtures",
            manifest_path: "games/race_to_n/data/manifest.toml",
            variants_path: "games/race_to_n/data/variants.toml",
            variant_id: "race_to_21",
        }),
        "three_marks" => Ok(RegisteredGame {
            game_id: "three_marks",
            rules_version: "three_marks-rules-v1",
            trace_dir: "games/three_marks/tests/golden_traces",
            fixture_dir: "games/three_marks/data/fixtures",
            manifest_path: "games/three_marks/data/manifest.toml",
            variants_path: "games/three_marks/data/variants.toml",
            variant_id: "three_marks_standard",
        }),
        "column_four" => Ok(RegisteredGame {
            game_id: "column_four",
            rules_version: "column_four-rules-v1",
            trace_dir: "games/column_four/tests/golden_traces",
            fixture_dir: "games/column_four/data/fixtures",
            manifest_path: "games/column_four/data/manifest.toml",
            variants_path: "games/column_four/data/variants.toml",
            variant_id: "column_four_standard",
        }),
        "directional_flip" => Ok(RegisteredGame {
            game_id: "directional_flip",
            rules_version: "directional_flip-rules-v1",
            trace_dir: "games/directional_flip/tests/golden_traces",
            fixture_dir: "games/directional_flip/data/fixtures",
            manifest_path: "games/directional_flip/data/manifest.toml",
            variants_path: "games/directional_flip/data/variants.toml",
            variant_id: "directional_flip_standard",
        }),
        "draughts_lite" => Ok(RegisteredGame {
            game_id: "draughts_lite",
            rules_version: "draughts_lite-rules-v1",
            trace_dir: "games/draughts_lite/tests/golden_traces",
            fixture_dir: "games/draughts_lite/data/fixtures",
            manifest_path: "games/draughts_lite/data/manifest.toml",
            variants_path: "games/draughts_lite/data/variants.toml",
            variant_id: "draughts_lite_standard",
        }),
        "high_card_duel" => Ok(RegisteredGame {
            game_id: "high_card_duel",
            rules_version: "high-card-duel-rules-v1",
            trace_dir: "games/high_card_duel/tests/golden_traces",
            fixture_dir: "games/high_card_duel/data/fixtures",
            manifest_path: "games/high_card_duel/data/manifest.toml",
            variants_path: "games/high_card_duel/data/variants.toml",
            variant_id: "high_card_duel_standard",
        }),
        "token_bazaar" => Ok(RegisteredGame {
            game_id: "token_bazaar",
            rules_version: "token-bazaar-rules-v1",
            trace_dir: "games/token_bazaar/tests/golden_traces",
            fixture_dir: "games/token_bazaar/data/fixtures",
            manifest_path: "games/token_bazaar/data/manifest.toml",
            variants_path: "games/token_bazaar/data/variants.toml",
            variant_id: "token_bazaar_standard",
        }),
        "secret_draft" => Ok(RegisteredGame {
            game_id: "secret_draft",
            rules_version: "secret-draft-rules-v1",
            trace_dir: "games/secret_draft/tests/golden_traces",
            fixture_dir: "games/secret_draft/data/fixtures",
            manifest_path: "games/secret_draft/data/manifest.toml",
            variants_path: "games/secret_draft/data/variants.toml",
            variant_id: "secret_draft_standard",
        }),
        "poker_lite" => Ok(RegisteredGame {
            game_id: "poker_lite",
            rules_version: "poker-lite-rules-v1",
            trace_dir: "games/poker_lite/tests/golden_traces",
            fixture_dir: "games/poker_lite/data/fixtures",
            manifest_path: "games/poker_lite/data/manifest.toml",
            variants_path: "games/poker_lite/data/variants.toml",
            variant_id: "poker_lite_standard",
        }),
        "plain_tricks" => Ok(RegisteredGame {
            game_id: "plain_tricks",
            rules_version: "plain-tricks-rules-v1",
            trace_dir: "games/plain_tricks/tests/golden_traces",
            fixture_dir: "games/plain_tricks/data/fixtures",
            manifest_path: "games/plain_tricks/data/manifest.toml",
            variants_path: "games/plain_tricks/data/variants.toml",
            variant_id: "plain_tricks_standard",
        }),
        _ => Err(format!("unsupported game `{game}`")),
    }
}

fn collect(result: Result<(), String>, failures: &mut Vec<String>) {
    if let Err(error) = result {
        failures.push(error);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    game: String,
    trace: Option<PathBuf>,
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
            trace,
        })
    }
}

fn next_arg(iter: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    iter.next()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn print_help() {
    println!("fixture-check 0.1.0");
    println!("usage:");
    println!(
        "  fixture-check --game <race_to_n|three_marks|column_four|directional_flip|draughts_lite|high_card_duel|token_bazaar|secret_draft|poker_lite|plain_tricks>"
    );
    println!("  fixture-check --game <race_to_n|three_marks|column_four|directional_flip|draughts_lite|high_card_duel|token_bazaar|secret_draft|poker_lite|plain_tricks> --trace <path>");
}

fn trace_paths(game: RegisteredGame) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    for entry in
        fs::read_dir(game.trace_dir).map_err(|error| format!("{}: {error}", game.trace_dir))?
    {
        let path = entry
            .map_err(|error| format!("{}: {error}", game.trace_dir))?
            .path();
        if path.extension().and_then(|value| value.to_str()) == Some("json") {
            paths.push(path);
        }
    }
    paths.sort();
    if paths.is_empty() {
        return Err(format!("{}: no .json traces found", game.trace_dir));
    }
    Ok(paths)
}

fn validate_static_data(game: RegisteredGame) -> Result<(), String> {
    let (
        manifest_game_id,
        manifest_rules_version,
        manifest_data_version,
        manifest_schema_version,
        selected_variant,
    ) = match game.game_id {
        "race_to_n" => {
            let manifest = race_to_n::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = race_to_n::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.selected.id,
            )
        }
        "three_marks" => {
            let manifest = three_marks::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = three_marks::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.selected.id,
            )
        }
        "column_four" => {
            let manifest = column_four::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = column_four::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.selected.id,
            )
        }
        "directional_flip" => {
            let manifest = directional_flip::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = directional_flip::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.selected.id,
            )
        }
        "draughts_lite" => {
            let manifest = draughts_lite::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = draughts_lite::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.selected.id,
            )
        }
        "high_card_duel" => {
            let manifest = high_card_duel::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = high_card_duel::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.selected.id,
            )
        }
        "token_bazaar" => {
            let manifest = token_bazaar::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = token_bazaar::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.selected.id,
            )
        }
        "secret_draft" => {
            let manifest = secret_draft::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = secret_draft::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.selected.id,
            )
        }
        "poker_lite" => {
            let manifest = poker_lite::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = poker_lite::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.selected.id,
            )
        }
        "plain_tricks" => {
            let manifest = plain_tricks::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = plain_tricks::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.selected.id,
            )
        }
        _ => unreachable!("resolved games only"),
    };

    if manifest_game_id != game.game_id {
        return Err(format!(
            "{}: game_id must be {}, got `{}`",
            game.manifest_path, game.game_id, manifest_game_id
        ));
    }
    if manifest_rules_version != 1 {
        return Err(format!(
            "{}: rules_version `{}` does not map to {}",
            game.manifest_path, manifest_rules_version, game.rules_version
        ));
    }
    if manifest_data_version != 1 || manifest_schema_version != 1 {
        return Err(format!(
            "{}: data_version and schema_version must be 1",
            game.manifest_path
        ));
    }
    if selected_variant != game.variant_id {
        return Err(format!(
            "{}: selected variant must be {}, got `{}`",
            game.variants_path, game.variant_id, selected_variant
        ));
    }
    Ok(())
}

fn reject_yaml_paths(game: RegisteredGame) -> Result<(), String> {
    for root in [game.trace_dir, game.fixture_dir, "reports"] {
        let root_path = Path::new(root);
        if !root_path.exists() {
            continue;
        }
        for path in walk_files(root_path)? {
            if matches!(
                path.extension().and_then(|value| value.to_str()),
                Some("yaml" | "yml")
            ) {
                return Err(format!(
                    "{}: YAML is not allowed in fixture/trace/report paths",
                    path.display()
                ));
            }
        }
    }
    Ok(())
}

fn walk_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(root).map_err(|error| format!("{}: {error}", root.display()))? {
        let path = entry
            .map_err(|error| format!("{}: {error}", root.display()))?
            .path();
        if path.is_dir() {
            files.extend(walk_files(&path)?);
        } else {
            files.push(path);
        }
    }
    Ok(files)
}

fn validate_trace_path(
    game: RegisteredGame,
    path: &Path,
    seen_ids: &mut HashSet<String>,
) -> Result<(), String> {
    let input = fs::read_to_string(path)
        .map_err(|error| format!("{}: failed to read: {error}", path.display()))?;
    validate_trace(game, path, &input, seen_ids)
}

fn validate_trace(
    game: RegisteredGame,
    path: &Path,
    input: &str,
    seen_ids: &mut HashSet<String>,
) -> Result<(), String> {
    validate_json_object(path, input)?;
    if input.contains("\"export_class\":") {
        return validate_public_export_fixture(game, path, input);
    }
    let keys = all_json_keys(input).map_err(|error| format!("{}: {error}", path.display()))?;
    for key in keys {
        if BEHAVIOR_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "{}: behavior-looking key `{key}` is not allowed",
                path.display()
            ));
        }
        if !ALLOWED_JSON_KEYS.contains(&key.as_str()) {
            return Err(format!("{}: unknown field `{key}`", path.display()));
        }
    }

    let trace_id = required_string(path, input, "trace_id")?;
    if !seen_ids.insert(trace_id.clone()) {
        return Err(format!(
            "{}: duplicate trace_id `{trace_id}`",
            path.display()
        ));
    }

    let fixture_kind = required_string(path, input, "fixture_kind")?;
    for field in [
        "schema_version",
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
        "checkpoints",
        "expected_state_hashes",
        "expected_effect_hashes",
        "expected_action_tree_hashes",
        "expected_public_view_hashes",
        "expected_outcome",
        "expected_terminal_state",
        "not_applicable",
    ] {
        require_key(path, input, field)?;
    }
    if game.game_id != "token_bazaar" {
        require_key(path, input, "expected_private_view_hashes")?;
    }
    if fixture_kind != "not_applicable" {
        require_key(path, input, "commands")?;
    }
    if matches!(fixture_kind.as_str(), "invalid" | "diagnostic") {
        require_key(path, input, "expected_diagnostics")?;
    }

    if required_number(path, input, "schema_version")? != 1 {
        return Err(format!("{}: schema_version must be 1", path.display()));
    }
    if required_string(path, input, "game_id")? != game.game_id {
        return Err(format!(
            "{}: game_id must be {}",
            path.display(),
            game.game_id
        ));
    }
    if required_string(path, input, "rules_version")? != game.rules_version {
        return Err(format!(
            "{}: rules_version must be {}",
            path.display(),
            game.rules_version
        ));
    }
    if required_string(path, input, "data_version")? != "1" {
        return Err(format!("{}: data_version must be 1", path.display()));
    }
    if required_string(path, input, "note")?.trim().is_empty() {
        return Err(format!("{}: note must be non-empty", path.display()));
    }
    if required_string(path, input, "migration_update_note")?
        .trim()
        .is_empty()
    {
        return Err(format!(
            "{}: migration_update_note must be non-empty",
            path.display()
        ));
    }
    if !input.contains("\"hidden_information\"") || !input.contains("\"stochastic_game_events\"") {
        return Err(format!(
            "{}: not_applicable must record hidden_information and stochastic_game_events rationale",
            path.display()
        ));
    }

    Ok(())
}

fn validate_public_export_fixture(
    game: RegisteredGame,
    path: &Path,
    input: &str,
) -> Result<(), String> {
    let keys = all_json_keys(input).map_err(|error| format!("{}: {error}", path.display()))?;
    for key in keys {
        if BEHAVIOR_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "{}: behavior-looking key `{key}` is not allowed",
                path.display()
            ));
        }
        if !ALLOWED_JSON_KEYS.contains(&key.as_str()) {
            return Err(format!("{}: unknown field `{key}`", path.display()));
        }
    }
    for field in [
        "schema_version",
        "export_class",
        "game_id",
        "rules_version",
        "variant",
        "steps",
    ] {
        require_key(path, input, field)?;
    }
    if required_number(path, input, "schema_version")? != 1 {
        return Err(format!("{}: schema_version must be 1", path.display()));
    }
    if required_string(path, input, "export_class")?
        .trim()
        .is_empty()
    {
        return Err(format!(
            "{}: export_class must be non-empty",
            path.display()
        ));
    }
    if required_string(path, input, "game_id")? != game.game_id {
        return Err(format!(
            "{}: game_id must be {}",
            path.display(),
            game.game_id
        ));
    }
    if required_string(path, input, "rules_version")? != game.rules_version {
        return Err(format!(
            "{}: rules_version must be {}",
            path.display(),
            game.rules_version
        ));
    }
    if required_string(path, input, "variant")? != game.variant_id {
        return Err(format!(
            "{}: variant must be {}",
            path.display(),
            game.variant_id
        ));
    }
    Ok(())
}

fn require_key(path: &Path, input: &str, key: &str) -> Result<(), String> {
    if input.contains(&format!("\"{key}\":")) {
        Ok(())
    } else {
        Err(format!("{}: missing field `{key}`", path.display()))
    }
}

fn required_string(path: &Path, input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("{}: missing field `{key}`", path.display()))?
        + needle.len();
    parse_string_at(input, start)
        .ok_or_else(|| format!("{}: field `{key}` must be a string", path.display()))
}

fn required_number(path: &Path, input: &str, key: &str) -> Result<u64, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("{}: missing field `{key}`", path.display()))?
        + needle.len();
    parse_number_at(input, start)
        .ok_or_else(|| format!("{}: field `{key}` must be a number", path.display()))
}

fn validate_json_object(path: &Path, input: &str) -> Result<(), String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err(format!("{}: malformed trace JSON object", path.display()));
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
            return Err(format!("{}: malformed trace JSON nesting", path.display()));
        }
    }
    if depth != 0 || in_string {
        return Err(format!("{}: malformed trace JSON nesting", path.display()));
    }
    Ok(())
}

fn all_json_keys(input: &str) -> Result<Vec<String>, String> {
    let mut keys = Vec::new();
    let mut expect_key = false;
    let mut stack = Vec::new();
    let mut index = 0;
    while index < input.len() {
        let Some(ch) = input[index..].chars().next() else {
            break;
        };
        match ch {
            '{' => {
                stack.push('{');
                expect_key = true;
                index += 1;
            }
            '[' => {
                stack.push('[');
                expect_key = false;
                index += 1;
            }
            '}' | ']' => {
                stack.pop();
                expect_key = false;
                index += 1;
            }
            ',' => {
                expect_key = stack.last() == Some(&'{');
                index += 1;
            }
            '"' => {
                let (value, next) = parse_json_string_at(input, index)?;
                let after = input[next..].trim_start();
                if expect_key && after.starts_with(':') {
                    keys.push(value);
                    expect_key = false;
                }
                index = next;
            }
            _ => index += ch.len_utf8(),
        }
    }
    Ok(keys)
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

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str =
        include_str!("../../../games/race_to_n/tests/golden_traces/shortest-normal.trace.json");

    fn validate_one(input: &str) -> Result<(), String> {
        let mut seen = HashSet::new();
        validate_trace(
            resolve_game("race_to_n").unwrap(),
            Path::new("fixture.trace.json"),
            input,
            &mut seen,
        )
    }

    #[test]
    fn valid_trace_passes() {
        validate_one(VALID).unwrap();
    }

    #[test]
    fn unknown_field_fails() {
        let input = VALID.replace(
            "\"trace_id\": \"shortest-normal\"",
            "\"trace_id\": \"shortest-normal\", \"extra\": true",
        );
        let err = validate_one(&input).unwrap_err();

        assert!(err.contains("unknown field `extra`"));
    }

    #[test]
    fn behavior_key_fails() {
        let input = VALID.replace("\"options\": {}", "\"options\": { \"trigger\": \"bad\" }");
        let err = validate_one(&input).unwrap_err();

        assert!(err.contains("behavior-looking key `trigger`"));
    }

    #[test]
    fn duplicate_id_fails() {
        let mut seen = HashSet::new();
        let game = resolve_game("race_to_n").unwrap();
        validate_trace(game, Path::new("first.trace.json"), VALID, &mut seen).unwrap();
        let err =
            validate_trace(game, Path::new("second.trace.json"), VALID, &mut seen).unwrap_err();

        assert!(err.contains("duplicate trace_id `shortest-normal`"));
    }

    #[test]
    fn empty_note_fails() {
        let input = VALID.replace(
            "\"note\": \"One non-terminal human action proves the smallest normal command/effect/view hash path.\"",
            "\"note\": \"\"",
        );
        let err = validate_one(&input).unwrap_err();

        assert!(err.contains("note must be non-empty"));
    }

    #[test]
    fn missing_migration_note_fails() {
        let input = VALID.replace(
            "\"migration_update_note\":",
            "\"migration_update_note_missing\":",
        );
        let err = validate_one(&input).unwrap_err();

        assert!(err.contains("unknown field `migration_update_note_missing`"));
    }

    #[test]
    fn malformed_trace_fails() {
        let err = validate_one("{ nope").unwrap_err();

        assert!(err.contains("malformed trace JSON object"));
    }
}
