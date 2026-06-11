use std::{collections::BTreeMap, env, fs, process};

fn main() {
    if let Err(error) = run(env::args().skip(1)) {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run(args: impl IntoIterator<Item = String>) -> Result<(), String> {
    let config = Config::parse(args)?;
    let game = resolve_game(&config.game)?;
    check_docs(
        &fs::read_to_string(game.rules_path)
            .map_err(|error| format!("{}: {error}", game.rules_path))?,
        &fs::read_to_string(game.coverage_path)
            .map_err(|error| format!("{}: {error}", game.coverage_path))?,
        &read_benchmarks(game)?,
    )?;
    println!("rule-coverage: {} coverage matrix passed", game.game_id);
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RegisteredGame {
    game_id: &'static str,
    rules_path: &'static str,
    coverage_path: &'static str,
    benchmarks_path: &'static str,
    benchmarks_required: bool,
}

fn resolve_game(game: &str) -> Result<RegisteredGame, String> {
    match game {
        "race_to_n" => Ok(RegisteredGame {
            game_id: "race_to_n",
            rules_path: "games/race_to_n/docs/RULES.md",
            coverage_path: "games/race_to_n/docs/RULE-COVERAGE.md",
            benchmarks_path: "games/race_to_n/docs/BENCHMARKS.md",
            benchmarks_required: true,
        }),
        "three_marks" => Ok(RegisteredGame {
            game_id: "three_marks",
            rules_path: "games/three_marks/docs/RULES.md",
            coverage_path: "games/three_marks/docs/RULE-COVERAGE.md",
            benchmarks_path: "games/three_marks/docs/BENCHMARKS.md",
            benchmarks_required: true,
        }),
        "column_four" => Ok(RegisteredGame {
            game_id: "column_four",
            rules_path: "games/column_four/docs/RULES.md",
            coverage_path: "games/column_four/docs/RULE-COVERAGE.md",
            benchmarks_path: "games/column_four/docs/BENCHMARKS.md",
            benchmarks_required: true,
        }),
        "directional_flip" => Ok(RegisteredGame {
            game_id: "directional_flip",
            rules_path: "games/directional_flip/docs/RULES.md",
            coverage_path: "games/directional_flip/docs/RULE-COVERAGE.md",
            benchmarks_path: "games/directional_flip/docs/BENCHMARKS.md",
            benchmarks_required: true,
        }),
        "draughts_lite" => Ok(RegisteredGame {
            game_id: "draughts_lite",
            rules_path: "games/draughts_lite/docs/RULES.md",
            coverage_path: "games/draughts_lite/docs/RULE-COVERAGE.md",
            benchmarks_path: "games/draughts_lite/docs/BENCHMARKS.md",
            benchmarks_required: true,
        }),
        "high_card_duel" => Ok(RegisteredGame {
            game_id: "high_card_duel",
            rules_path: "games/high_card_duel/docs/RULES.md",
            coverage_path: "games/high_card_duel/docs/RULE-COVERAGE.md",
            benchmarks_path: "games/high_card_duel/docs/BENCHMARKS.md",
            benchmarks_required: true,
        }),
        "masked_claims" => Ok(RegisteredGame {
            game_id: "masked_claims",
            rules_path: "games/masked_claims/docs/RULES.md",
            coverage_path: "games/masked_claims/docs/RULE-COVERAGE.md",
            benchmarks_path: "games/masked_claims/docs/BENCHMARKS.md",
            benchmarks_required: true,
        }),
        "token_bazaar" => Ok(RegisteredGame {
            game_id: "token_bazaar",
            rules_path: "games/token_bazaar/docs/RULES.md",
            coverage_path: "games/token_bazaar/docs/RULE-COVERAGE.md",
            benchmarks_path: "games/token_bazaar/docs/BENCHMARKS.md",
            benchmarks_required: true,
        }),
        "secret_draft" => Ok(RegisteredGame {
            game_id: "secret_draft",
            rules_path: "games/secret_draft/docs/RULES.md",
            coverage_path: "games/secret_draft/docs/RULE-COVERAGE.md",
            benchmarks_path: "games/secret_draft/docs/BENCHMARKS.md",
            benchmarks_required: true,
        }),
        "poker_lite" => Ok(RegisteredGame {
            game_id: "poker_lite",
            rules_path: "games/poker_lite/docs/RULES.md",
            coverage_path: "games/poker_lite/docs/RULE-COVERAGE.md",
            benchmarks_path: "games/poker_lite/docs/BENCHMARKS.md",
            benchmarks_required: true,
        }),
        "plain_tricks" => Ok(RegisteredGame {
            game_id: "plain_tricks",
            rules_path: "games/plain_tricks/docs/RULES.md",
            coverage_path: "games/plain_tricks/docs/RULE-COVERAGE.md",
            benchmarks_path: "games/plain_tricks/docs/BENCHMARKS.md",
            benchmarks_required: false,
        }),
        _ => Err(format!("unsupported game `{game}`")),
    }
}

fn read_benchmarks(game: RegisteredGame) -> Result<String, String> {
    match fs::read_to_string(game.benchmarks_path) {
        Ok(contents) => Ok(contents),
        Err(error) if !game.benchmarks_required && error.kind() == std::io::ErrorKind::NotFound => {
            Ok(String::new())
        }
        Err(error) => Err(format!("{}: {error}", game.benchmarks_path)),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    game: String,
}

impl Config {
    fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut game = None;
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => {
                    println!("rule-coverage 0.1.0");
                    println!(
                        "usage: rule-coverage --game <race_to_n|three_marks|column_four|directional_flip|draughts_lite|high_card_duel|masked_claims|token_bazaar|secret_draft|poker_lite|plain_tricks>"
                    );
                    process::exit(0);
                }
                "--game" => {
                    game = Some(
                        args.next()
                            .ok_or_else(|| "--game requires a value".to_owned())?,
                    )
                }
                other => return Err(format!("unknown argument `{other}`")),
            }
        }
        Ok(Self {
            game: game.ok_or_else(|| "--game is required".to_owned())?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CoverageRow {
    rule_id: String,
    evidence: String,
    status: String,
    notes: String,
}

fn check_docs(rules: &str, coverage: &str, benchmarks: &str) -> Result<(), String> {
    let rules_ids = extract_rule_ids(rules);
    let rows = coverage_rows(coverage)?;
    let mut failures = Vec::new();
    let mut by_id: BTreeMap<String, Vec<&CoverageRow>> = BTreeMap::new();
    for row in &rows {
        by_id.entry(row.rule_id.clone()).or_default().push(row);
    }

    for rule_id in &rules_ids {
        match by_id.get(rule_id.as_str()) {
            None => failures.push(format!("missing coverage row for {rule_id}")),
            Some(rows) if rows.len() > 1 => {
                failures.push(format!("{rule_id} has {} coverage rows", rows.len()));
            }
            Some(_) => {}
        }
    }
    for row in &rows {
        if !rules_ids.contains(&row.rule_id) {
            failures.push(format!("unknown rule ID in coverage row: {}", row.rule_id));
        }
        if row.evidence.trim().is_empty() {
            failures.push(format!("{} has blank evidence", row.rule_id));
        }
        if matches!(
            row.status.as_str(),
            "not-applicable" | "unsupported" | "intentionally-deferred"
        ) && row.notes.trim().is_empty()
        {
            failures.push(format!(
                "{} status `{}` lacks rationale",
                row.rule_id, row.status
            ));
        }
        if !matches!(
            row.status.as_str(),
            "covered"
                | "covered-by-trace"
                | "not-applicable"
                | "unsupported"
                | "intentionally-deferred"
        ) {
            failures.push(format!(
                "{} has unknown status `{}`",
                row.rule_id, row.status
            ));
        }
    }

    for line in coverage
        .lines()
        .filter(|line| line.contains("Stage-1 perf budget"))
    {
        if line.contains("intentionally-deferred") && benchmarks.contains("ADR 0001") {
            failures.push(
                "Stage-1 perf budget row is intentionally-deferred, but BENCHMARKS records ADR 0001 resolution"
                    .to_owned(),
            );
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!("rule-coverage failure\n{}", failures.join("\n")))
    }
}

fn extract_rule_ids(input: &str) -> Vec<String> {
    let mut ids = Vec::new();
    for token in input.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '-')) {
        if is_rule_id(token) && !ids.iter().any(|existing| existing == token) {
            ids.push(token.to_owned());
        }
    }
    ids.sort();
    ids
}

fn is_rule_id(value: &str) -> bool {
    let parts = value.split('-').collect::<Vec<_>>();
    parts.len() == 3
        && matches!(
            parts[0],
            "R" | "TM" | "CF" | "DF" | "DL" | "HCD" | "TB" | "SD" | "CL" | "PT" | "MC"
        )
        && !parts[1].is_empty()
        && parts[1].chars().all(|ch| ch.is_ascii_uppercase())
        && parts[2].len() == 3
        && parts[2].chars().all(|ch| ch.is_ascii_digit())
}

fn coverage_rows(input: &str) -> Result<Vec<CoverageRow>, String> {
    let mut in_matrix = false;
    let mut rows = Vec::new();
    for line in input.lines() {
        if line.starts_with("## Rule Coverage Matrix") {
            in_matrix = true;
            continue;
        }
        if in_matrix && line.starts_with("## ") {
            break;
        }
        if !in_matrix || !line.trim_start().starts_with('|') || !line.contains('`') {
            continue;
        }
        let cells = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().to_owned())
            .collect::<Vec<_>>();
        if cells.len() < 6 || cells[0] == "Rule ID" || cells[0].starts_with("---") {
            continue;
        }
        let Some(rule_id) = first_code(&cells[0]) else {
            continue;
        };
        rows.push(CoverageRow {
            rule_id,
            evidence: strip_markdown(&cells[3]),
            status: strip_markdown(&cells[4]),
            notes: strip_markdown(&cells[5]),
        });
    }
    Ok(rows)
}

fn first_code(input: &str) -> Option<String> {
    let start = input.find('`')? + 1;
    let end = input[start..].find('`')? + start;
    Some(input[start..end].to_owned())
}

fn strip_markdown(input: &str) -> String {
    input.replace('`', "").trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    const RULES: &str = "| `R-ONE-001` | one |\n| `R-TWO-002` | two |\n";
    const COVERAGE: &str = r#"
## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `R-ONE-001` | One. | code | test | covered | ok |
| `R-TWO-002` | Two. | code | rationale | unsupported | not in variant |
"#;

    #[test]
    fn valid_fixture_passes() {
        check_docs(RULES, COVERAGE, "no ADR").unwrap();
    }

    #[test]
    fn draughts_lite_rule_prefix_is_valid() {
        assert!(is_rule_id("DL-SCOPE-001"));
        assert!(is_rule_id("HCD-SETUP-001"));
        assert!(is_rule_id("TB-SETUP-001"));
        assert!(is_rule_id("CL-SETUP-001"));
        assert!(!is_rule_id("XX-SCOPE-001"));
    }

    #[test]
    fn deleted_row_fails() {
        let coverage = COVERAGE.replace(
            "| `R-TWO-002` | Two. | code | rationale | unsupported | not in variant |\n",
            "",
        );
        let error = check_docs(RULES, &coverage, "no ADR").unwrap_err();

        assert!(error.contains("missing coverage row for R-TWO-002"));
    }

    #[test]
    fn unknown_id_fails() {
        let coverage = COVERAGE.replace("R-TWO-002", "R-BOGUS-999");
        let error = check_docs(RULES, &coverage, "no ADR").unwrap_err();

        assert!(error.contains("unknown rule ID"));
    }

    #[test]
    fn blank_evidence_fails() {
        let coverage = COVERAGE.replace(
            "| `R-ONE-001` | One. | code | test | covered | ok |",
            "| `R-ONE-001` | One. | code |  | covered | ok |",
        );
        let error = check_docs(RULES, &coverage, "no ADR").unwrap_err();

        assert!(error.contains("R-ONE-001 has blank evidence"));
    }

    #[test]
    fn missing_rationale_fails() {
        let coverage = COVERAGE.replace(
            "| `R-TWO-002` | Two. | code | rationale | unsupported | not in variant |",
            "| `R-TWO-002` | Two. | code | rationale | unsupported |  |",
        );
        let error = check_docs(RULES, &coverage, "no ADR").unwrap_err();

        assert!(error.contains("lacks rationale"));
    }
}
