//! Characterization snapshot of the browser-facing `wasm-api` public surface.
//!
//! This is a behavior-locking safety net for refactors of the (very large)
//! `wasm-api` crate: it drives every registered game through the full public
//! API lifecycle (catalog, match creation, views, action trees, effects, and
//! replay export/import/step/reset) and compares the exact JSON outputs against
//! a committed snapshot.
//!
//! The API is fully deterministic (no time/RNG in its output) and match/replay
//! ids are sequential per thread, so capturing in a fixed order yields a stable
//! snapshot. Outputs are single-line JSON, so the snapshot is a simple
//! tab-separated `key<TAB>value` file with one entry per line.
//!
//! To regenerate the snapshot after an *intended* behavior change, run:
//!
//! ```sh
//! UPDATE_API_SNAPSHOT=1 cargo test -p wasm-api --test api_surface
//! ```
//!
//! and review the diff. Any unintended change is a refactor regression.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

/// Fixed order matching `RegisteredGame`; the order pins the sequential
/// match/replay ids embedded in captured outputs, so it must not change
/// without regenerating the snapshot.
const GAMES: &[&str] = &[
    "race_to_n",
    "three_marks",
    "column_four",
    "directional_flip",
    "draughts_lite",
    "high_card_duel",
    "masked_claims",
    "flood_watch",
    "frontier_control",
    "event_frontier",
    "token_bazaar",
    "secret_draft",
    "poker_lite",
    "plain_tricks",
    "river_ledger",
];

const SEED: u64 = 7;

const DEFAULT_SEAT_COUNT: usize = 2;

/// Per-game seat-count overrides for games whose minimum differs from the
/// platform default of two.
fn seat_count_for(game: &str) -> usize {
    match game {
        "river_ledger" => 3,
        _ => DEFAULT_SEAT_COUNT,
    }
}

/// Encode a `Result<String, String>` into a single deterministic line. Real
/// outputs are JSON (`{` / `[` / `"`-prefixed); errors are tagged so the two
/// are never confused.
fn enc(result: Result<String, String>) -> String {
    match result {
        Ok(value) => value,
        Err(err) => format!("ERR:{err}"),
    }
}

/// Extract a top-level `"field":"value"` string from a JSON object string. The
/// API emits flat, un-nested ids, so a substring scan is sufficient and avoids
/// pulling in a JSON parser dependency.
fn extract_field(json: &str, field: &str) -> Option<String> {
    let needle = format!("\"{field}\":\"");
    let start = json.find(&needle)? + needle.len();
    let rest = &json[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}

/// Drive the entire public API and return `(key, value)` capture pairs in a
/// deterministic order. All match/replay-id-allocating calls happen here in a
/// single thread so the embedded ids are reproducible.
fn capture() -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut push = |key: String, value: String| out.push((key, value));

    push(
        "_global/placeholder_version".to_owned(),
        wasm_api::placeholder_version().to_owned(),
    );
    push("_global/list_games".to_owned(), enc(wasm_api::list_games()));
    push(
        "_global/feature_report".to_owned(),
        enc(wasm_api::feature_report()),
    );

    for game in GAMES {
        let key = |suffix: &str| format!("{game}/{suffix}");
        let seat_count = seat_count_for(game);

        let new_match = wasm_api::new_match_with_seat_count(game, SEED, seat_count);
        let match_id = new_match
            .as_ref()
            .ok()
            .and_then(|json| extract_field(json, "match_id"));
        push(key("new_match"), enc(new_match));

        let Some(match_id) = match_id else {
            continue;
        };

        push(
            key("view/observer"),
            enc(wasm_api::get_view(&match_id, None)),
        );
        for seat in 0..seat_count {
            let seat_id = format!("seat_{seat}");
            push(
                key(&format!("view/{seat_id}")),
                enc(wasm_api::get_view(&match_id, Some(&seat_id))),
            );
            push(
                key(&format!("action_tree/{seat_id}")),
                enc(wasm_api::get_action_tree(&match_id, &seat_id)),
            );
        }
        push(
            key("action_tree_for_viewer/seat_0_observer"),
            enc(wasm_api::get_action_tree_for_viewer(
                &match_id, "seat_0", None,
            )),
        );

        push(
            key("effects/since_0_observer"),
            enc(wasm_api::get_effects(&match_id, 0, None)),
        );

        let export = wasm_api::export_replay(&match_id);
        let export_doc = export.as_ref().ok().cloned();
        push(key("export_replay"), enc(export));

        if let Some(doc) = export_doc {
            let import = wasm_api::import_replay(&doc);
            let replay_id = import
                .as_ref()
                .ok()
                .and_then(|json| extract_field(json, "replay_id"));
            push(key("import_replay"), enc(import));

            if let Some(replay_id) = replay_id {
                push(
                    key("replay_step/0"),
                    enc(wasm_api::replay_step(&replay_id, 0)),
                );
                push(key("replay_reset"), enc(wasm_api::replay_reset(&replay_id)));
            }
        }
    }

    out
}

fn snapshot_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("snapshots")
        .join("api_surface.tsv")
}

fn serialize(entries: &BTreeMap<String, String>) -> String {
    let mut buf = String::new();
    for (key, value) in entries {
        buf.push_str(key);
        buf.push('\t');
        buf.push_str(value);
        buf.push('\n');
    }
    buf
}

fn parse(contents: &str) -> BTreeMap<String, String> {
    contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (key, value) = line
                .split_once('\t')
                .expect("snapshot line missing tab separator");
            (key.to_owned(), value.to_owned())
        })
        .collect()
}

#[test]
fn public_api_surface_matches_snapshot() {
    let captured: BTreeMap<String, String> = capture().into_iter().collect();

    // Sanity: every game contributed a view, i.e. dispatch reached all arms.
    for game in GAMES {
        assert!(
            captured.contains_key(&format!("{game}/view/observer")),
            "missing observer view for {game}; new_match likely failed"
        );
    }

    let path = snapshot_path();

    if std::env::var("UPDATE_API_SNAPSHOT").is_ok() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create snapshot dir");
        }
        fs::write(&path, serialize(&captured)).expect("write snapshot");
        return;
    }

    let expected_contents = fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "missing snapshot at {}: {err}\n\
             generate it with: UPDATE_API_SNAPSHOT=1 cargo test -p wasm-api --test api_surface",
            path.display()
        )
    });
    let expected = parse(&expected_contents);

    let mut mismatches: Vec<String> = Vec::new();
    for (key, value) in &captured {
        match expected.get(key) {
            None => mismatches.push(format!("+ unexpected key: {key}")),
            Some(prev) if prev != value => mismatches.push(format!(
                "~ changed: {key}\n    was: {prev}\n    now: {value}"
            )),
            Some(_) => {}
        }
    }
    for key in expected.keys() {
        if !captured.contains_key(key) {
            mismatches.push(format!("- missing key: {key}"));
        }
    }

    assert!(
        mismatches.is_empty(),
        "public API surface drifted from snapshot ({} change(s)):\n{}\n\n\
         If intended, regenerate with: UPDATE_API_SNAPSHOT=1 cargo test -p wasm-api --test api_surface",
        mismatches.len(),
        mismatches.join("\n"),
    );
}
