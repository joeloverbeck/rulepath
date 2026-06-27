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
    "formula",
    "score_formula",
    "tie_break_formula",
    "trick_winner_formula",
    "follow_suit_formula",
    "deal_formula",
    "rotation_formula",
    "bid_formula",
];

const PROFILE_VERSION_V1: &str = "v1";
const SETUP_EVIDENCE_V1: &str = "setup-evidence-v1";
const DOMAIN_EVIDENCE_V1: &str = "domain-evidence-v1";
const PROFILE_COMMON_FIELDS: &[&str] = &[
    "profile_id",
    "profile_version",
    "visibility_class",
    "validator_owner",
    "hash_surface_version",
    "canonical_byte_authority",
];
const PROFILE_SPECIFIC_FIELDS: &[&str] = &[
    "seat_grammar_version",
    "setup_options",
    "expected_setup",
    "domain_schema_version",
    "domain_input",
    "expected_domain",
];
const SETUP_EVIDENCE_PROFILE_FIELDS: &[&str] =
    &["seat_grammar_version", "setup_options", "expected_setup"];
const DOMAIN_EVIDENCE_PROFILE_FIELDS: &[&str] =
    &["domain_schema_version", "domain_input", "expected_domain"];

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
    "shared_outcome",
    "expected_effects",
    "expected_event",
    "expected_role",
    "expected_starting_levels",
    "expected_draw_count",
    "expected_state_change",
    "expected_reinforce_amount",
    "expected_bail_amount",
    "expected_public_deck_count",
    "expected_rise",
    "public_export_contains_ordered_deck",
    "viewer_scope",
    "district",
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
    "phase",
    "responder",
    "choices",
    "claimant_tree_empty",
    "score_delta",
    "revealed_tiles",
    "reveal_event",
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
    "hidden_information_redaction",
    "per_seat_hidden_surface",
    "internal_trace_full_deck_hash",
    "stochastic_game_events",
    "stochastic_game_rule_events",
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
    "profile_id",
    "profile_version",
    "visibility_class",
    "validator_owner",
    "hash_surface_version",
    "canonical_byte_authority",
    "seat_grammar_version",
    "setup_options",
    "expected_setup",
    "domain_schema_version",
    "domain_input",
    "expected_domain",
];

const VOW_TIDE_ALLOWED_JSON_KEYS: &[&str] = &[
    "schema_version",
    "trace_id",
    "fixture_kind",
    "purpose",
    "game_id",
    "rules_version",
    "engine_version",
    "data_version",
    "seed",
    "variant",
    "seats",
    "seat_count",
    "expected_dealer",
    "expected_first_leader",
    "expected_hand_size",
    "expected_hand_count",
    "expected_trump_public",
    "expected_hidden_stock_count",
    "expected_hook_prefix_total",
    "expected_hook_forbidden_bid",
    "expected_terminal_winners",
    "expected_competition_ranks",
    "note",
    "profile_id",
    "profile_version",
    "visibility_class",
    "validator_owner",
    "hash_surface_version",
    "canonical_byte_authority",
];

const BLACKGLASS_PACT_ALLOWED_JSON_KEYS: &[&str] = &[
    "schema_version",
    "fixture_id",
    "trace_id",
    "fixture_kind",
    "purpose",
    "note",
    "migration_update_note",
    "game_id",
    "rules_version",
    "data_version",
    "variant",
    "seed",
    "seat_count",
    "seats",
    "teams",
    "dealer",
    "hand_index",
    "team",
    "team_scores",
    "pending_blind_nil_order",
    "legal_action_paths",
    "public_pre_deal_surfaces",
    "card_identity",
    "deck_order",
    "rng_samples",
    "prior_bags",
    "new_bags",
    "bag_penalty_count",
    "next_bags",
    "scores_after_hand",
    "terminal",
    "next_hand_required",
    "expected_setup",
    "expected_team_order",
    "expected_seat_order",
    "expected_dealer",
    "expected_phase",
    "expected_hand_count",
    "profile_id",
    "profile_version",
    "visibility_class",
    "validator_owner",
    "hash_surface_version",
    "canonical_byte_authority",
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
        for path in pilot_profile_fixture_paths(game) {
            collect(
                validate_trace_path(game, Path::new(path), &mut seen_ids),
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
        "masked_claims" => Ok(RegisteredGame {
            game_id: "masked_claims",
            rules_version: "masked-claims-rules-v1",
            trace_dir: "games/masked_claims/tests/golden_traces",
            fixture_dir: "games/masked_claims/data/fixtures",
            manifest_path: "games/masked_claims/data/manifest.toml",
            variants_path: "games/masked_claims/data/variants.toml",
            variant_id: "masked_claims_standard",
        }),
        "flood_watch" => Ok(RegisteredGame {
            game_id: "flood_watch",
            rules_version: "flood-watch-rules-v1",
            trace_dir: "games/flood_watch/tests/golden_traces",
            fixture_dir: "games/flood_watch/data/fixtures",
            manifest_path: "games/flood_watch/data/manifest.toml",
            variants_path: "games/flood_watch/data/variants.toml",
            variant_id: "flood_watch_standard",
        }),
        "frontier_control" => Ok(RegisteredGame {
            game_id: "frontier_control",
            rules_version: "frontier-control-rules-v1",
            trace_dir: "games/frontier_control/tests/golden_traces",
            fixture_dir: "games/frontier_control/data/fixtures",
            manifest_path: "games/frontier_control/data/manifest.toml",
            variants_path: "games/frontier_control/data/variants.toml",
            variant_id: "frontier_control_standard",
        }),
        "event_frontier" => Ok(RegisteredGame {
            game_id: "event_frontier",
            rules_version: "event-frontier-rules-v1",
            trace_dir: "games/event_frontier/tests/golden_traces",
            fixture_dir: "games/event_frontier/data/fixtures",
            manifest_path: "games/event_frontier/data/manifest.toml",
            variants_path: "games/event_frontier/data/variants.toml",
            variant_id: "event_frontier_standard",
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
        "river_ledger" => Ok(RegisteredGame {
            game_id: "river_ledger",
            rules_version: "river-ledger-rules-v2",
            trace_dir: "games/river_ledger/tests/golden_traces",
            fixture_dir: "games/river_ledger/data/fixtures",
            manifest_path: "games/river_ledger/data/manifest.toml",
            variants_path: "games/river_ledger/data/variants.toml",
            variant_id: "river_ledger_standard",
        }),
        "briar_circuit" => Ok(RegisteredGame {
            game_id: "briar_circuit",
            rules_version: "briar-circuit-rules-v1",
            trace_dir: "games/briar_circuit/tests/golden_traces",
            fixture_dir: "games/briar_circuit/data/fixtures",
            manifest_path: "games/briar_circuit/data/manifest.toml",
            variants_path: "games/briar_circuit/data/variants.toml",
            variant_id: "briar_circuit_standard",
        }),
        "vow_tide" => Ok(RegisteredGame {
            game_id: "vow_tide",
            rules_version: "vow-tide-rules-v1",
            trace_dir: "games/vow_tide/data/fixtures",
            fixture_dir: "games/vow_tide/data/fixtures",
            manifest_path: "games/vow_tide/data/manifest.toml",
            variants_path: "games/vow_tide/data/variants.toml",
            variant_id: "vow_tide_standard",
        }),
        "blackglass_pact" => Ok(RegisteredGame {
            game_id: "blackglass_pact",
            rules_version: "blackglass-pact-rules-v1",
            trace_dir: "games/blackglass_pact/data/fixtures",
            fixture_dir: "games/blackglass_pact/data/fixtures",
            manifest_path: "games/blackglass_pact/data/manifest.toml",
            variants_path: "games/blackglass_pact/data/variants.toml",
            variant_id: "blackglass_pact_standard",
        }),
        "meldfall_ledger" => Ok(RegisteredGame {
            game_id: "meldfall_ledger",
            rules_version: "meldfall-ledger-rules-v1",
            trace_dir: "games/meldfall_ledger/tests/golden_traces",
            fixture_dir: "games/meldfall_ledger/data/fixtures",
            manifest_path: "games/meldfall_ledger/data/manifest.toml",
            variants_path: "games/meldfall_ledger/data/variants.toml",
            variant_id: "classic_500_single_deck_v1",
        }),
        "starbridge_crossing" => Ok(RegisteredGame {
            game_id: "starbridge_crossing",
            rules_version: "starbridge-crossing-rules-v1",
            trace_dir: "games/starbridge_crossing/tests/golden_traces",
            fixture_dir: "games/starbridge_crossing/data/fixtures",
            manifest_path: "games/starbridge_crossing/data/manifest.toml",
            variants_path: "games/starbridge_crossing/data/variants.toml",
            variant_id: "starbridge_crossing_classic_star_v1",
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
        "  fixture-check --game <race_to_n|three_marks|column_four|directional_flip|draughts_lite|high_card_duel|masked_claims|flood_watch|frontier_control|event_frontier|token_bazaar|secret_draft|poker_lite|plain_tricks|river_ledger|briar_circuit|vow_tide|blackglass_pact|meldfall_ledger|starbridge_crossing>"
    );
    println!("  fixture-check --game <race_to_n|three_marks|column_four|directional_flip|draughts_lite|high_card_duel|masked_claims|flood_watch|frontier_control|event_frontier|token_bazaar|secret_draft|poker_lite|plain_tricks|river_ledger|briar_circuit|vow_tide|blackglass_pact|meldfall_ledger|starbridge_crossing> --trace <path>");
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

fn pilot_profile_fixture_paths(game: RegisteredGame) -> &'static [&'static str] {
    match game.game_id {
        "river_ledger" => {
            &["games/river_ledger/data/fixtures/river_ledger_3p_standard.fixture.json"]
        }
        "briar_circuit" => &[
            "games/briar_circuit/data/fixtures/briar_circuit_moon.fixture.json",
            "games/briar_circuit/data/fixtures/briar_circuit_first_trick_exception.fixture.json",
        ],
        _ => &[],
    }
}

fn validate_fixture_profile_dispatch(
    game: RegisteredGame,
    path: &Path,
    input: &str,
) -> Result<(), String> {
    let Some(profile_id) = optional_string_field(input, "profile_id") else {
        return Ok(());
    };
    let expected = match game.game_id {
        "river_ledger" => &[SETUP_EVIDENCE_V1][..],
        "briar_circuit" => &[DOMAIN_EVIDENCE_V1][..],
        _ => &[][..],
    };
    if !expected.contains(&profile_id.as_str()) {
        return Err(format!(
            "{}: profile `{profile_id}` is not registered for fixture-check game {}",
            path.display(),
            game.game_id
        ));
    }
    validate_fixture_profile_metadata(path, input)
}

fn validate_fixture_profile_metadata(path: &Path, input: &str) -> Result<(), String> {
    let Some(profile_id) = optional_string_field(input, "profile_id") else {
        return Ok(());
    };
    let (allowed_visibility, allowed_profile_fields) = match profile_id.as_str() {
        SETUP_EVIDENCE_V1 => (
            &["public", "viewer-scoped", "seat-private", "internal-dev"][..],
            SETUP_EVIDENCE_PROFILE_FIELDS,
        ),
        DOMAIN_EVIDENCE_V1 => (
            &[
                "public",
                "viewer-scoped",
                "seat-private",
                "internal-dev",
                "private-source",
            ][..],
            DOMAIN_EVIDENCE_PROFILE_FIELDS,
        ),
        _ => {
            return Err(format!(
                "{}: unknown profile `{profile_id}`",
                path.display()
            ));
        }
    };

    let profile_version = required_string(path, input, "profile_version")?;
    if profile_version != PROFILE_VERSION_V1 {
        return Err(format!(
            "{}: profile_version must be {PROFILE_VERSION_V1}",
            path.display()
        ));
    }
    let visibility = required_string(path, input, "visibility_class")?;
    if !allowed_visibility.contains(&visibility.as_str()) {
        return Err(format!(
            "{}: visibility_class `{visibility}` is invalid for {profile_id}",
            path.display()
        ));
    }
    if required_string(path, input, "validator_owner")?
        .trim()
        .is_empty()
    {
        return Err(format!(
            "{}: validator_owner must be non-empty",
            path.display()
        ));
    }
    if required_string(path, input, "canonical_byte_authority")?
        .trim()
        .is_empty()
    {
        return Err(format!(
            "{}: canonical_byte_authority must be non-empty",
            path.display()
        ));
    }
    if optional_string_field(input, "migration_update_note")
        .or_else(|| optional_string_field(input, "migration_notes"))
        .unwrap_or_default()
        .trim()
        .is_empty()
    {
        return Err(format!(
            "{}: migration_update_note or migration_notes must be non-empty",
            path.display()
        ));
    }

    for key in top_level_keys(input).map_err(|error| format!("{}: {error}", path.display()))? {
        if PROFILE_SPECIFIC_FIELDS.contains(&key.as_str())
            && !allowed_profile_fields.contains(&key.as_str())
        {
            return Err(format!(
                "{}: field `{key}` is not valid for {profile_id}",
                path.display()
            ));
        }
        if PROFILE_COMMON_FIELDS.contains(&key.as_str()) {
            continue;
        }
    }
    Ok(())
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
        "masked_claims" => {
            let manifest = masked_claims::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = masked_claims::load_variants().map_err(|error| {
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
        "flood_watch" => {
            let manifest = flood_watch::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = flood_watch::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.standard.id,
            )
        }
        "frontier_control" => {
            let manifest = frontier_control::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = frontier_control::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.standard.id,
            )
        }
        "event_frontier" => {
            let manifest = event_frontier::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = event_frontier::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            let _cards = event_frontier::load_cards()
                .map_err(|error| format!("games/event_frontier/data/cards.toml: {error}"))?;
            (
                manifest.game_id,
                manifest.rules_version,
                manifest.data_version,
                manifest.schema_version,
                variants.standard.id,
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
        "river_ledger" => {
            let manifest = river_ledger::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = river_ledger::load_variants().map_err(|error| {
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
        "briar_circuit" => {
            let manifest = briar_circuit::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = briar_circuit::load_variants().map_err(|error| {
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
        "vow_tide" => {
            let manifest = vow_tide::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = vow_tide::load_variants().map_err(|error| {
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
        "blackglass_pact" => {
            let manifest = blackglass_pact::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = blackglass_pact::load_variants().map_err(|error| {
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
        "meldfall_ledger" => {
            let manifest = meldfall_ledger::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = meldfall_ledger::load_variants().map_err(|error| {
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
        "starbridge_crossing" => {
            let manifest = starbridge_crossing::load_manifest().map_err(|error| {
                format!("{}: manifest parse failed: {error}", game.manifest_path)
            })?;
            let variants = starbridge_crossing::load_variants().map_err(|error| {
                format!("{}: variants parse failed: {error}", game.variants_path)
            })?;
            if manifest.rules_version_label != game.rules_version {
                return Err(format!(
                    "{}: rules_version_label must be {}",
                    game.manifest_path, game.rules_version
                ));
            }
            if manifest.data_version_label != "starbridge-crossing-data-v1" {
                return Err(format!(
                    "{}: data_version_label must be starbridge-crossing-data-v1",
                    game.manifest_path
                ));
            }
            if variants.selected.rules_version_label != game.rules_version
                || variants.selected.data_version_label != "starbridge-crossing-data-v1"
            {
                return Err(format!(
                    "{}: selected variant version labels must match Starbridge v1",
                    game.variants_path
                ));
            }
            (manifest.game_id, 1, 1, 1, variants.selected.id)
        }
        _ => unreachable!("resolved games only"),
    };

    if manifest_game_id != game.game_id {
        return Err(format!(
            "{}: game_id must be {}, got `{}`",
            game.manifest_path, game.game_id, manifest_game_id
        ));
    }
    let expected_rules_version = if game.game_id == "river_ledger" { 2 } else { 1 };
    if manifest_rules_version != expected_rules_version {
        return Err(format!(
            "{}: rules_version `{}` does not map to {}",
            game.manifest_path, manifest_rules_version, game.rules_version
        ));
    }
    let expected_data_version = if game.game_id == "river_ledger" { 2 } else { 1 };
    if manifest_data_version != expected_data_version || manifest_schema_version != 1 {
        return Err(format!(
            "{}: data_version must be {} and schema_version must be 1",
            game.manifest_path, expected_data_version
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
    validate_fixture_profile_dispatch(game, path, input)?;
    if game.game_id == "river_ledger" {
        if input.contains("\"fixture_id\": \"river_ledger_3p_standard\"") {
            return validate_river_ledger_setup_evidence_fixture(game, path, input, seen_ids);
        }
        return validate_river_ledger_trace(game, path, input, seen_ids);
    }
    if game.game_id == "briar_circuit" {
        return validate_briar_circuit_fixture(game, path, input, seen_ids);
    }
    if game.game_id == "vow_tide" {
        return validate_vow_tide_fixture(game, path, input, seen_ids);
    }
    if game.game_id == "blackglass_pact" {
        return validate_blackglass_pact_fixture(game, path, input, seen_ids);
    }
    if game.game_id == "meldfall_ledger" {
        return validate_meldfall_ledger_fixture(game, path, input, seen_ids);
    }
    if game.game_id == "starbridge_crossing" {
        return validate_starbridge_crossing_fixture(game, path, input, seen_ids);
    }
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
    if game.game_id == "flood_watch" {
        for field in [
            "schema_version",
            "purpose",
            "note",
            "game_id",
            "rules_version",
            "engine_version",
            "data_version",
            "seed",
            "variant",
            "public_no_leak",
        ] {
            require_key(path, input, field)?;
        }
        if fixture_kind != "setup" {
            require_key(path, input, "commands")?;
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
        if !input.contains("\"public_no_leak\":true") {
            return Err(format!(
                "{}: flood_watch trace must assert public_no_leak",
                path.display()
            ));
        }
        return Ok(());
    }
    let required_fields = [
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
        "checkpoints",
        "expected_state_hashes",
        "expected_effect_hashes",
        "expected_action_tree_hashes",
        "expected_public_view_hashes",
        "expected_outcome",
        "expected_terminal_state",
        "not_applicable",
    ];
    for field in required_fields {
        if game.game_id == "masked_claims"
            && matches!(
                field,
                "expected_action_tree_hashes"
                    | "expected_outcome"
                    | "expected_public_view_hashes"
                    | "expected_terminal_state"
                    | "not_applicable"
            )
            && !input.contains(&format!("\"{field}\""))
        {
            continue;
        }
        require_key(path, input, field)?;
    }
    if game.game_id != "masked_claims" {
        for field in ["options", "seats"] {
            require_key(path, input, field)?;
        }
    }
    if !matches!(game.game_id, "token_bazaar" | "masked_claims") {
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
    let trace_rules_version = required_string(path, input, "rules_version")?;
    let river_ledger_placeholder_v1 =
        game.game_id == "river_ledger" && trace_rules_version == "river-ledger-rules-v1";
    if trace_rules_version != game.rules_version && !river_ledger_placeholder_v1 {
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
    let has_hidden_na = input.contains("\"hidden_information\"")
        || (game.game_id == "frontier_control"
            && input.contains("\"hidden_information_redaction\""));
    let has_stochastic_na = input.contains("\"stochastic_game_events\"")
        || (matches!(game.game_id, "frontier_control" | "event_frontier")
            && input.contains("\"stochastic_game_rule_events\""));
    if game.game_id != "masked_claims" && (!has_hidden_na || !has_stochastic_na) {
        return Err(format!(
            "{}: not_applicable must record hidden_information and stochastic_game_events rationale",
            path.display()
        ));
    }

    Ok(())
}

fn validate_starbridge_crossing_fixture(
    game: RegisteredGame,
    path: &Path,
    input: &str,
    seen_ids: &mut HashSet<String>,
) -> Result<(), String> {
    let keys = all_json_keys(input).map_err(|error| format!("{}: {error}", path.display()))?;
    for key in keys {
        if BEHAVIOR_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "{}: behavior-looking key `{key}` is not allowed",
                path.display()
            ));
        }
    }
    let trace_id = required_string(path, input, "trace_id")?;
    if trace_id.trim().is_empty() {
        return Err(format!("{}: trace_id must be non-empty", path.display()));
    }
    if !seen_ids.insert(trace_id.clone()) {
        return Err(format!(
            "{}: duplicate trace_id `{trace_id}`",
            path.display()
        ));
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
    if input.contains("\"rules_version\"")
        && required_string(path, input, "rules_version")? != game.rules_version
    {
        return Err(format!(
            "{}: rules_version must be {}",
            path.display(),
            game.rules_version
        ));
    }
    if !input.contains("\"coverage\"") {
        return Err(format!("{}: missing coverage receipt", path.display()));
    }
    if input.contains("\"public_no_leak\"") && !input.contains("\"public_no_leak\":true") {
        return Err(format!(
            "{}: public_no_leak receipts must be true",
            path.display()
        ));
    }
    Ok(())
}

fn validate_briar_circuit_fixture(
    game: RegisteredGame,
    path: &Path,
    input: &str,
    seen_ids: &mut HashSet<String>,
) -> Result<(), String> {
    let keys = all_json_keys(input).map_err(|error| format!("{}: {error}", path.display()))?;
    for key in keys {
        if BEHAVIOR_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "{}: behavior-looking key `{key}` is not allowed",
                path.display()
            ));
        }
    }
    let id = optional_string_field(input, "trace")
        .or_else(|| optional_string_field(input, "trace_id"))
        .or_else(|| optional_string_field(input, "id"))
        .unwrap_or_else(|| path.file_stem().unwrap().to_string_lossy().to_string());
    if !seen_ids.insert(id.clone()) {
        return Err(format!("{}: duplicate trace id `{id}`", path.display()));
    }
    if !input.contains("\"schema_version\":1") && !input.contains("\"schema_version\": 1") {
        return Err(format!("{}: schema_version must be 1", path.display()));
    }
    let game_id =
        optional_string_field(input, "game_id").or_else(|| optional_string_field(input, "game"));
    if game_id.as_deref() != Some(game.game_id) {
        return Err(format!(
            "{}: game_id must be {}",
            path.display(),
            game.game_id
        ));
    }
    if !input.contains(game.rules_version) {
        return Err(format!(
            "{}: rules_version must be {}",
            path.display(),
            game.rules_version
        ));
    }
    Ok(())
}

fn validate_vow_tide_fixture(
    game: RegisteredGame,
    path: &Path,
    input: &str,
    seen_ids: &mut HashSet<String>,
) -> Result<(), String> {
    let keys = all_json_keys(input).map_err(|error| format!("{}: {error}", path.display()))?;
    for key in keys {
        if BEHAVIOR_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "{}: behavior-looking key `{key}` is not allowed",
                path.display()
            ));
        }
        if !VOW_TIDE_ALLOWED_JSON_KEYS.contains(&key.as_str()) {
            return Err(format!("{}: unknown field `{key}`", path.display()));
        }
    }
    let trace_id = required_string(path, input, "trace_id")?;
    if !trace_id.starts_with("vow_tide_") {
        return Err(format!(
            "{}: vow_tide trace_id must start with vow_tide_",
            path.display()
        ));
    }
    if !seen_ids.insert(trace_id.clone()) {
        return Err(format!(
            "{}: duplicate trace_id `{trace_id}`",
            path.display()
        ));
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
    if required_number(path, input, "data_version")? != 1 {
        return Err(format!("{}: data_version must be 1", path.display()));
    }
    if required_string(path, input, "variant")? != game.variant_id {
        return Err(format!(
            "{}: variant must be {}",
            path.display(),
            game.variant_id
        ));
    }
    if required_string(path, input, "purpose")?.trim().is_empty() {
        return Err(format!("{}: purpose must be non-empty", path.display()));
    }
    let fixture_kind = required_string(path, input, "fixture_kind")?;
    if !matches!(
        fixture_kind.as_str(),
        "setup" | "bidding_hook" | "terminal_tie"
    ) {
        return Err(format!(
            "{}: unsupported vow_tide fixture_kind `{fixture_kind}`",
            path.display()
        ));
    }
    let seats = optional_string_array_field(input, "seats")
        .ok_or_else(|| format!("{}: missing string array `seats`", path.display()))?;
    if !(3..=7).contains(&seats.len()) {
        return Err(format!(
            "{}: vow_tide seats must contain 3, 4, 5, 6, or 7 seats",
            path.display()
        ));
    }
    for (index, seat) in seats.iter().enumerate() {
        if seat != &format!("seat_{index}") {
            return Err(format!(
                "{}: vow_tide seat order must be stable seat_N order",
                path.display()
            ));
        }
    }
    if required_number(path, input, "seat_count")? != seats.len() as u64 {
        return Err(format!(
            "{}: seat_count must match seats length",
            path.display()
        ));
    }
    for field in [
        "expected_dealer",
        "expected_first_leader",
        "expected_hand_size",
        "expected_hand_count",
        "expected_trump_public",
        "expected_hidden_stock_count",
    ] {
        require_key(path, input, field)?;
    }
    if fixture_kind == "bidding_hook" {
        require_key(path, input, "expected_hook_prefix_total")?;
        require_key(path, input, "expected_hook_forbidden_bid")?;
    }
    if fixture_kind == "terminal_tie" {
        require_key(path, input, "expected_terminal_winners")?;
        require_key(path, input, "expected_competition_ranks")?;
    }
    Ok(())
}

fn validate_blackglass_pact_fixture(
    game: RegisteredGame,
    path: &Path,
    input: &str,
    seen_ids: &mut HashSet<String>,
) -> Result<(), String> {
    let keys = all_json_keys(input).map_err(|error| format!("{}: {error}", path.display()))?;
    for key in keys {
        if BEHAVIOR_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "{}: behavior-looking key `{key}` is not allowed",
                path.display()
            ));
        }
        if !BLACKGLASS_PACT_ALLOWED_JSON_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "{}: unknown blackglass_pact field `{key}`",
                path.display()
            ));
        }
    }

    let id = optional_string_field(input, "fixture_id")
        .or_else(|| optional_string_field(input, "trace_id"))
        .unwrap_or_else(|| path.file_stem().unwrap().to_string_lossy().to_string());
    if !seen_ids.insert(id.clone()) {
        return Err(format!("{}: duplicate fixture id `{id}`", path.display()));
    }
    if id.trim().is_empty() {
        return Err(format!("{}: fixture id must be non-empty", path.display()));
    }
    if input.contains("\"game_id\"") && required_string(path, input, "game_id")? != game.game_id {
        return Err(format!(
            "{}: game_id must be {}",
            path.display(),
            game.game_id
        ));
    }
    if input.contains("\"rules_version\"")
        && required_string(path, input, "rules_version")? != game.rules_version
    {
        return Err(format!(
            "{}: rules_version must be {}",
            path.display(),
            game.rules_version
        ));
    }
    if input.contains("\"variant\"") && required_string(path, input, "variant")? != game.variant_id
    {
        return Err(format!(
            "{}: variant must be {}",
            path.display(),
            game.variant_id
        ));
    }
    if input.contains("\"schema_version\"") && required_number(path, input, "schema_version")? != 1
    {
        return Err(format!("{}: schema_version must be 1", path.display()));
    }
    Ok(())
}

fn validate_meldfall_ledger_fixture(
    game: RegisteredGame,
    path: &Path,
    input: &str,
    seen_ids: &mut HashSet<String>,
) -> Result<(), String> {
    let keys = all_json_keys(input).map_err(|error| format!("{}: {error}", path.display()))?;
    for key in keys {
        if BEHAVIOR_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "{}: behavior-looking key `{key}` is not allowed",
                path.display()
            ));
        }
    }
    let id = optional_string_field(input, "fixture_id")
        .or_else(|| optional_string_field(input, "trace_id"))
        .or_else(|| optional_string_field(input, "trace"))
        .unwrap_or_else(|| path.file_stem().unwrap().to_string_lossy().to_string());
    if !seen_ids.insert(id.clone()) {
        return Err(format!("{}: duplicate fixture id `{id}`", path.display()));
    }
    if id.trim().is_empty() {
        return Err(format!("{}: fixture id must be non-empty", path.display()));
    }
    let game_id =
        optional_string_field(input, "game_id").or_else(|| optional_string_field(input, "game"));
    if game_id.as_deref() != Some(game.game_id) {
        return Err(format!(
            "{}: game_id must be {}",
            path.display(),
            game.game_id
        ));
    }
    let rules_version = optional_string_field(input, "rules_version")
        .or_else(|| optional_string_field(input, "rules"));
    if let Some(rules_version) = rules_version {
        if rules_version != game.rules_version {
            return Err(format!(
                "{}: rules_version must be {}",
                path.display(),
                game.rules_version
            ));
        }
    }
    if input.contains("\"variant\"") && required_string(path, input, "variant")? != game.variant_id
    {
        return Err(format!(
            "{}: variant must be {}",
            path.display(),
            game.variant_id
        ));
    }
    if input.contains("\"schema_version\"") && required_number(path, input, "schema_version")? != 1
    {
        return Err(format!("{}: schema_version must be 1", path.display()));
    }
    if input.contains("\"seat_count\"") {
        let seat_count = required_number(path, input, "seat_count")? as usize;
        let fixture_id = id.as_str();
        let invalid_seat_diagnostic =
            fixture_id.contains("invalid-seat-count") || fixture_id.contains("invalid_seat_count");
        if !meldfall_ledger::supported_seat_count(seat_count) && !invalid_seat_diagnostic {
            return Err(format!(
                "{}: unsupported meldfall_ledger seat_count {seat_count}",
                path.display()
            ));
        }
        if let Some(seats) = optional_string_array_field(input, "seats") {
            if seats.len() != seat_count {
                return Err(format!(
                    "{}: seat_count must match seats length",
                    path.display()
                ));
            }
            for (index, seat) in seats.iter().enumerate() {
                if seat != &format!("seat_{index}") {
                    return Err(format!(
                        "{}: meldfall_ledger seat order must be stable seat_N order",
                        path.display()
                    ));
                }
            }
        }
        if input.contains("\"expected_hand_count\"") && !invalid_seat_diagnostic {
            let expected = required_number(path, input, "expected_hand_count")? as u8;
            if meldfall_ledger::hand_size_for_seats(seat_count) != Some(expected) {
                return Err(format!(
                    "{}: expected_hand_count does not match variant hand size",
                    path.display()
                ));
            }
        }
    }
    if input.contains("\"purpose\"") && required_string(path, input, "purpose")?.trim().is_empty() {
        return Err(format!("{}: purpose must be non-empty", path.display()));
    }
    if path
        .components()
        .any(|component| component.as_os_str() == "fixtures")
        && !input.contains("\"purpose\"")
    {
        return Err(format!(
            "{}: fixture files must include non-empty purpose",
            path.display()
        ));
    }
    Ok(())
}

fn validate_river_ledger_setup_evidence_fixture(
    game: RegisteredGame,
    path: &Path,
    input: &str,
    seen_ids: &mut HashSet<String>,
) -> Result<(), String> {
    let keys = all_json_keys(input).map_err(|error| format!("{}: {error}", path.display()))?;
    for key in keys {
        if BEHAVIOR_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "{}: behavior-looking key `{key}` is not allowed",
                path.display()
            ));
        }
    }
    let fixture_id = required_string(path, input, "fixture_id")?;
    if !seen_ids.insert(fixture_id.clone()) {
        return Err(format!(
            "{}: duplicate fixture_id `{fixture_id}`",
            path.display()
        ));
    }
    if fixture_id != "river_ledger_3p_standard" {
        return Err(format!(
            "{}: unsupported setup-evidence fixture `{fixture_id}`",
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
    let rules_version = required_string(path, input, "rules_version")?;
    if !matches!(
        rules_version.as_str(),
        "river-ledger-rules-v1" | "river-ledger-rules-v2"
    ) {
        return Err(format!(
            "{}: unsupported river_ledger rules_version `{rules_version}`",
            path.display()
        ));
    }
    if required_string(path, input, "variant")? != game.variant_id {
        return Err(format!(
            "{}: variant must be {}",
            path.display(),
            game.variant_id
        ));
    }
    if required_number(path, input, "seat_count")? != 3 {
        return Err(format!("{}: seat_count must be 3", path.display()));
    }
    for (field, expected) in [
        ("button", "seat_0"),
        ("small_blind", "seat_1"),
        ("big_blind", "seat_2"),
        ("initial_active", "seat_0"),
    ] {
        if required_string(path, input, field)? != expected {
            return Err(format!("{}: {field} must be {expected}", path.display()));
        }
    }
    for (field, expected) in [
        ("public_board_count", 0),
        ("private_hole_cards_per_seat", 2),
        ("reserved_community_count", 5),
        ("deck_tail_count", 41),
    ] {
        if required_number(path, input, field)? != expected {
            return Err(format!("{}: {field} must be {expected}", path.display()));
        }
    }
    if required_string(path, input, "purpose")?.trim().is_empty() {
        return Err(format!("{}: purpose must be non-empty", path.display()));
    }
    Ok(())
}

fn validate_river_ledger_trace(
    game: RegisteredGame,
    path: &Path,
    input: &str,
    seen_ids: &mut HashSet<String>,
) -> Result<(), String> {
    let keys = all_json_keys(input).map_err(|error| format!("{}: {error}", path.display()))?;
    for key in keys {
        if BEHAVIOR_KEYS.contains(&key.as_str()) {
            return Err(format!(
                "{}: behavior-looking key `{key}` is not allowed",
                path.display()
            ));
        }
    }
    let trace_id = required_string(path, input, "trace_id")?;
    if !seen_ids.insert(trace_id.clone()) {
        return Err(format!(
            "{}: duplicate trace_id `{trace_id}`",
            path.display()
        ));
    }
    if !trace_id.starts_with("river-ledger-") {
        return Err(format!(
            "{}: river_ledger trace_id must start with river-ledger-",
            path.display()
        ));
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
    let trace_rules_version = required_string(path, input, "rules_version")?;
    let river_ledger_placeholder_v1 =
        game.game_id == "river_ledger" && trace_rules_version == "river-ledger-rules-v1";
    if trace_rules_version != game.rules_version && !river_ledger_placeholder_v1 {
        return Err(format!(
            "{}: rules_version must be {}",
            path.display(),
            game.rules_version
        ));
    }
    if required_string(path, input, "purpose")?.trim().is_empty() {
        return Err(format!("{}: purpose must be non-empty", path.display()));
    }
    if let Some(seats) = optional_string_array_field(input, "seats") {
        let fixture_kind = required_string(path, input, "fixture_kind")?;
        if fixture_kind != "setup-diagnostic" && !(3..=6).contains(&seats.len()) {
            return Err(format!(
                "{}: river_ledger seats must contain 3, 4, 5, or 6 seats",
                path.display()
            ));
        }
        for (index, seat) in seats.iter().enumerate() {
            if seat != &format!("seat_{index}") {
                return Err(format!(
                    "{}: river_ledger seat order must be stable seat_N order",
                    path.display()
                ));
            }
        }
    }
    for action in command_string_fields(input, "action") {
        if river_ledger::parse_action_segment(&action).is_none() {
            return Err(format!(
                "{}: river_ledger command has invalid action `{action}`",
                path.display()
            ));
        }
    }
    for seat in command_string_fields(input, "seat") {
        if river_ledger::RiverLedgerSeat::parse(&seat).is_none() {
            return Err(format!(
                "{}: river_ledger command has invalid seat `{seat}`",
                path.display()
            ));
        }
    }
    let fixture_kind = required_string(path, input, "fixture_kind")?;
    match fixture_kind.as_str() {
        "setup" => require_key(path, input, "expected_public_setup")?,
        "setup-diagnostic" | "diagnostic-placeholder" => {
            require_key(path, input, "expected_diagnostics")?
        }
        "betting-placeholder" | "showdown-placeholder" => {
            if !input.contains("\"expected_public_result\"") && !input.contains("\"allocations\"") {
                return Err(format!(
                    "{}: missing river_ledger public result/allocation evidence",
                    path.display()
                ));
            }
        }
        "visibility-placeholder" => {
            if !input.contains("\"forbidden_public_facts\"")
                && !input.contains("\"forbidden_cross_seat_facts\"")
            {
                return Err(format!(
                    "{}: river_ledger visibility trace must name forbidden facts",
                    path.display()
                ));
            }
        }
        "replay-placeholder" => require_key(path, input, "public_export")?,
        "bot-placeholder" => require_key(path, input, "expected")?,
        "evaluator-placeholder" => {
            if !input.contains("\"category\"")
                && !input.contains("\"expected_comparison\"")
                && !input.contains("\"expected_remainder_order\"")
            {
                return Err(format!(
                    "{}: river_ledger evaluator trace must name expected evaluator evidence",
                    path.display()
                ));
            }
        }
        other => {
            return Err(format!(
                "{}: unsupported river_ledger fixture_kind `{other}`",
                path.display()
            ));
        }
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

fn optional_string_array_field(input: &str, key: &str) -> Option<Vec<String>> {
    let needle = format!("\"{key}\":");
    let start = input.find(&needle)? + needle.len();
    let tail = &input[start..];
    let open = tail.find('[')?;
    let close = tail[open..].find(']')? + open;
    Some(parse_array_strings(&tail[open + 1..close]))
}

fn optional_string_field(input: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\":");
    let start = input.find(&needle)? + needle.len();
    parse_string_at(input, start)
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

fn parse_array_strings(input: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut remaining = input;
    while let Some(start) = remaining.find('"') {
        remaining = &remaining[start + 1..];
        let Some(end) = remaining.find('"') else {
            break;
        };
        values.push(remaining[..end].to_owned());
        remaining = &remaining[end + 1..];
    }
    values
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
        index = skip_json_value(body, index);
        keys.push(key);
    }
    Ok(keys)
}

fn skip_json_value(input: &str, mut index: usize) -> usize {
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
                    return index + offset;
                }
                depth -= 1;
            }
            ',' if depth == 0 => return index + offset + 1,
            _ => {}
        }
    }
    input.len()
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
    const VALID_RIVER_SETUP: &str = include_str!(
        "../../../games/river_ledger/data/fixtures/river_ledger_3p_standard.fixture.json"
    );
    const VALID_VOW_TIDE_FIXTURE: &str = r#"{
      "schema_version": 1,
      "trace_id": "vow_tide_test_fixture",
      "fixture_kind": "setup",
      "purpose": "test fixture",
      "game_id": "vow_tide",
      "rules_version": "vow-tide-rules-v1",
      "engine_version": "engine-core-0.1.0",
      "data_version": 1,
      "seed": 1,
      "variant": "vow_tide_standard",
      "seats": ["seat_0", "seat_1", "seat_2"],
      "seat_count": 3,
      "expected_dealer": "seat_0",
      "expected_first_leader": "seat_1",
      "expected_hand_size": 10,
      "expected_hand_count": 19,
      "expected_trump_public": true,
      "expected_hidden_stock_count": 21,
      "note": "test"
    }"#;

    fn validate_one(input: &str) -> Result<(), String> {
        let mut seen = HashSet::new();
        validate_trace(
            resolve_game("race_to_n").unwrap(),
            Path::new("fixture.trace.json"),
            input,
            &mut seen,
        )
    }

    fn river_setup_profile_fixture(extra: &str) -> String {
        VALID_RIVER_SETUP.replace(
            "\"fixture_id\": \"river_ledger_3p_standard\"",
            &format!(
                "\"fixture_id\": \"river_ledger_3p_standard\", \
                 \"profile_id\": \"setup-evidence-v1\", \
                 \"profile_version\": \"v1\", \
                 \"visibility_class\": \"public\", \
                 \"validator_owner\": \"fixture-check\", \
                 \"hash_surface_version\": \"not-applicable\", \
                 \"canonical_byte_authority\": \"none\", \
                 \"migration_update_note\": \"test profile metadata\"{extra}"
            ),
        )
    }

    #[test]
    fn valid_trace_passes() {
        validate_one(VALID).unwrap();
    }

    #[test]
    fn river_setup_profile_metadata_passes() {
        let mut seen = HashSet::new();
        validate_trace(
            resolve_game("river_ledger").unwrap(),
            Path::new("river_ledger_3p_standard.fixture.json"),
            &river_setup_profile_fixture(""),
            &mut seen,
        )
        .unwrap();
    }

    #[test]
    fn fixture_unknown_profile_fails() {
        let input =
            river_setup_profile_fixture("").replace("setup-evidence-v1", "unknown-profile-v1");
        let mut seen = HashSet::new();
        let err = validate_trace(
            resolve_game("river_ledger").unwrap(),
            Path::new("river_ledger_3p_standard.fixture.json"),
            &input,
            &mut seen,
        )
        .unwrap_err();

        assert!(err.contains(
            "profile `unknown-profile-v1` is not registered for fixture-check game river_ledger"
        ));
    }

    #[test]
    fn fixture_cross_profile_field_fails() {
        let input = river_setup_profile_fixture(", \"domain_input\": {}");
        let mut seen = HashSet::new();
        let err = validate_trace(
            resolve_game("river_ledger").unwrap(),
            Path::new("river_ledger_3p_standard.fixture.json"),
            &input,
            &mut seen,
        )
        .unwrap_err();

        assert!(err.contains("field `domain_input` is not valid for setup-evidence-v1"));
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
    fn vow_tide_fixture_behavior_key_fails() {
        let input = VALID_VOW_TIDE_FIXTURE.replace(
            r#""note": "test""#,
            r#""note": "test", "score_formula": "10 + bid""#,
        );
        let mut seen = HashSet::new();
        let err = validate_vow_tide_fixture(
            resolve_game("vow_tide").unwrap(),
            Path::new("vow_tide_test.fixture.json"),
            &input,
            &mut seen,
        )
        .unwrap_err();

        assert!(err.contains("behavior-looking key `score_formula`"));
    }

    #[test]
    fn vow_tide_fixture_unknown_key_fails() {
        let input = VALID_VOW_TIDE_FIXTURE.replace(
            r#""note": "test""#,
            r#""note": "test", "unknown_field": "nope""#,
        );
        let mut seen = HashSet::new();
        let err = validate_vow_tide_fixture(
            resolve_game("vow_tide").unwrap(),
            Path::new("vow_tide_test.fixture.json"),
            &input,
            &mut seen,
        )
        .unwrap_err();

        assert!(err.contains("unknown field `unknown_field`"));
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
