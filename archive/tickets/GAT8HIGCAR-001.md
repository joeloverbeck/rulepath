# GAT8HIGCAR-001: Robust public-observer-projection replay import routing

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (replay import routing + tests). No `engine-core`, `game-stdlib`, or `games/*` changes. No schema/trace/serialization-format change.
**Deps**: None. Relates to accepted ADR `docs/adr/0004-hidden-info-replay-export-taxonomy.md` (public observer projection export class).

## Problem

The browser "Export Current Run" button followed by "Import Replay" is a broken round-trip for `high_card_duel`. Importing a freshly exported public-observer-projection replay fails on-screen with:

```
invalid_replay
invalid replay document: unknown field `export_class`
```

Root cause is a serialization-format-fragile router, not a hidden-information or determinism defect:

1. The web export UI pretty-prints the document before placing it in the textarea: `apps/web/src/components/ReplayImportExport.tsx:20` does `setDocumentText(JSON.stringify(document, null, 2))`, producing 2-space-indented JSON with a space after every colon (`"export_class": "public_observer_projection_v1"`).
2. "Import Replay" (`ReplayImportExport.tsx:33`) passes that text verbatim to Rust `import_replay`.
3. The import router `is_high_card_public_export` (`crates/wasm-api/src/lib.rs:1383-1386`) detects the public-export class with raw compact-byte substring matching:
   ```rust
   doc.contains("\"export_class\":\"public_observer_projection_v1\"")
       && doc.contains("\"game_id\":\"high_card_duel\"")
   ```
   The pretty-printed value (space after the colon) is not a substring of the compact form, so the function returns `false`.
4. Detection fails, so the document falls through to the generic internal-trace parser `parse_replay_document` (`lib.rs:1184`), whose `reject_unknown_root_fields` allow-list (`lib.rs:3804-3834`) does not contain `export_class`/`viewer`/`steps`, and `reject_unknown_root_fields` (`lib.rs:3918-3925`) returns `unknown field \`export_class\``.

The app's own export therefore cannot be re-imported. The existing test `high_card_duel_surface_filters_hidden_information` (`lib.rs:4845`) re-imports the **compact** `exported` string directly and so never exercises the pretty-printed form the UI actually produces — the defect is invisible to current coverage.

## Assumption Reassessment (2026-06-08)

1. **Router uses compact substring matching.** Confirmed: `is_high_card_public_export` at `crates/wasm-api/src/lib.rs:1383-1386` uses `doc.contains("\"export_class\":\"public_observer_projection_v1\"")`, which embeds the value immediately after the colon with no whitespace tolerance.
2. **The structural field extractors are whitespace-tolerant.** Confirmed: `string_field` (`lib.rs:3993-4000`) finds the needle `"key":` and delegates value parsing to `parse_string_at`, and `top_level_keys` (`lib.rs:3927-3957`) / `skip_json_value` (`lib.rs:3959`) explicitly `trim_start` around structural tokens. `import_high_card_public_replay` already relies on `string_field` to read `rules_version`/`variant`/`viewer` (`lib.rs:1395-1415`), so value-based detection via `string_field` is whitespace-tolerant by construction.
3. **Cross-artifact boundary under audit:** the replay import serialization contract between the web export UI (`apps/web/src/components/ReplayImportExport.tsx`, which pretty-prints) and the Rust `import_replay` router (`crates/wasm-api/src/lib.rs`). The canonical end-state is: import routing must accept any whitespace-equivalent serialization of the document the app exports.
4. **FOUNDATIONS principle under audit (§11 "Unknown fields in hand-authored data are rejected by default" + fail-closed validation):** the fix must NOT weaken `reject_unknown_root_fields`. It must route the public-export document to its dedicated importer `import_high_card_public_replay`; it must not add `export_class`/`viewer`/`steps` to the generic internal-trace allow-list. The generic parser keeps rejecting unknown fields.
5. **Deterministic replay/hash & serialization surface (§11/§13):** this is an import-side routing change only. `PublicReplayExport::to_json` (`games/high_card_duel/src/replay_support.rs:94-112`) and all golden traces are untouched, so export bytes and replay hashes are unchanged. No hidden information is introduced into any payload (§11 no-leak firewall) — routing reads only `export_class` and `game_id`, both already public.
6. **Other games are not affected.** Confirmed: all other games export internal full traces imported through `parse_replay_document`, whose hand-rolled parser is whitespace-tolerant (item 2). Only the High-Card public-export path bypasses structural parsing with raw `contains`. No generalization of the dispatch is in scope (only one game has a public export today — FOUNDATIONS §4 "earned, not speculative"); the round-trip regression test below is the guard against the anti-pattern recurring.

No spec mismatch found: ADR-0004 requires `import_replay` to "distinguish public projection replay from full internal test/dev trace" — this ticket restores that distinction for the serialization the app actually emits.

## Architecture Check

1. **Cleaner than alternatives.** Replacing raw compact-byte `contains` with value comparison through the existing whitespace-tolerant `string_field` extractor makes detection robust to any equivalent JSON formatting (pretty-print, reordering of unrelated whitespace) while reusing the parser already trusted in `import_high_card_public_replay`. The rejected alternative — adding `export_class`/`viewer`/`steps` to `reject_unknown_root_fields`' allow-list — is incorrect: it weakens fail-closed unknown-field rejection (§11) and would let a malformed internal trace masquerade as a public export.
2. **No backwards-compatibility aliasing/shims introduced.** The compact form already matched and continues to match; this widens detection to whitespace-equivalent forms, not a parallel legacy path.
3. **`engine-core` untouched (stays free of mechanic nouns, §3); no `game-stdlib` change (§4).** All edits are in `crates/wasm-api` routing plus its test module.

## Verification Layers

1. App-emitted export round-trips through import (serialization-format robustness) -> new wasm-api unit test that imports the `JSON.stringify(_, null, 2)`-equivalent (indented, space-after-colon) form of a high_card_duel public export and asserts success.
2. Unknown-field rejection remains fail-closed for the generic internal-trace path (§11) -> existing `import_rejects_wrong_game_version_malformed_and_oversized` (`lib.rs:4900`) keeps passing; add an assertion that a non-public document with a stray `export_class`-like unknown root field is still rejected by `parse_replay_document` (no allow-list weakening).
3. No hidden-information leak via routing change (§11 no-leak firewall) -> assert the imported public timeline result still contains no `hcd:r` card identities and reports `"public_export":true`.
4. Export bytes / replay-hash determinism unchanged (§11/§13) -> grep-proof that `games/high_card_duel/src/replay_support.rs` and golden trace fixtures are not modified; existing serialization tests (`games/high_card_duel/tests/serialization.rs`) pass unchanged.

## What to Change

### 1. Make public-export detection whitespace-tolerant

In `crates/wasm-api/src/lib.rs`, reimplement `is_high_card_public_export` (lines 1383-1386) to compare extracted field values rather than match compact byte sequences. Use the existing `string_field` extractor and compare values, e.g. read `export_class` and `game_id` via `string_field` and compare to `"public_observer_projection_v1"` and `high_card_duel::GAME_ID`. Detection must return `false` (not error) when fields are absent, so non-public documents still fall through to `parse_replay_document` unchanged.

Do NOT modify `reject_unknown_root_fields` or its allow-list (`lib.rs:3804-3834`). The generic internal-trace path keeps rejecting `export_class` as an unknown field — only the routing decision changes.

### 2. Add a pretty-printed round-trip regression test

In the `wasm-api` test module (`mod tests` at `lib.rs:4462`), add a test that:
- creates and plays a `high_card_duel` match (mirror the setup in `high_card_duel_surface_filters_hidden_information`, `lib.rs:4790`),
- exports via `export_replay`,
- reformats the compact export into the indented, space-after-colon form the web UI produces (equivalent to `JSON.stringify(document, null, 2)`; reformat the exported string or construct a representative pretty-printed document),
- calls `import_replay` on the pretty-printed form and asserts it succeeds, returns `"public_export":true`, `"game_id":"high_card_duel"`, and contains no `hcd:r` identity leak.

Keep the existing compact-form import assertions (`lib.rs:4845-4851`) so both serializations are covered.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify) — `is_high_card_public_export` routing; new regression test in `mod tests`.

## Out of Scope

- **F3 — empty-step reconstruction (flagged, not fixed here).** `import_high_card_public_replay` (`lib.rs:1416-1432`) rebuilds steps by counting `"step_index":` occurrences and fills empty `public_view_summary`/`public_effects`, and the public step API returns `"view":null,"effects":[]` (`lib.rs:1459-`). After this routing fix, import succeeds but the imported timeline carries no per-step content. Whether that is accepted Gate-8 minimal scope or a fidelity gap warranting a follow-up ticket is deliberately deferred.
- Generalizing the public-export dispatch to a game-agnostic `export_class` router (no second hidden-info game with a public export exists yet — FOUNDATIONS §4).
- Any change to export serialization, `PublicReplayExport::to_json`, golden traces, or the web UI export/import components.
- Changing the web UI to send compact JSON (the Rust router is the correct, behavior-authoritative place to fix per FOUNDATIONS §2).

## Acceptance Criteria

### Tests That Must Pass

- New wasm-api regression test: a `high_card_duel` public export reformatted to the indented `JSON.stringify(_, null, 2)`-equivalent form imports successfully via `import_replay`, returns `"public_export":true` and `"game_id":"high_card_duel"`, and leaks no `hcd:r` identities.
- Existing `high_card_duel_surface_filters_hidden_information` (`lib.rs:4790`) still passes (compact-form import unchanged).
- Existing `import_rejects_wrong_game_version_malformed_and_oversized` (`lib.rs:4900`) still passes (fail-closed unknown-field rejection preserved on the generic path).
- `cargo test -p wasm-api` passes.

### Commands

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p wasm-api
cargo test --workspace
cargo run -p replay-check  -- --game high_card_duel --all
cargo run -p fixture-check -- --game high_card_duel
```

### Manual verification (optional, browser)

```bash
npm --prefix apps/web run smoke:wasm
```
Then in the app: play a `high_card_duel` match to terminal → "Export Current Run" → "Import Replay" on the unmodified textarea contents → import succeeds with no `invalid_replay` diagnostic.

## Outcome

Completed: 2026-06-08

What changed:
- `crates/wasm-api/src/lib.rs` now routes `high_card_duel` public observer replay exports by extracting `export_class` and `game_id` values with the existing whitespace-tolerant field parser instead of compact-byte substring matching.
- Added wasm-api regression coverage that imports a UI-style pretty-printed public export, asserts `"public_export":true`, `"game_id":"high_card_duel"`, and confirms no `hcd:r` hidden card identity leak in the import/reset surfaces.
- Added coverage that a generic internal replay carrying a stray `export_class` root field is still rejected, preserving fail-closed unknown-field handling.

Deviations:
- None. `reject_unknown_root_fields`, `games/high_card_duel/src/replay_support.rs`, and golden trace fixtures were not changed.

Verification:
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace` — passed.
- `cargo test -p wasm-api` — passed.
- `cargo test --workspace` — passed.
- `cargo run -p replay-check -- --game high_card_duel --all` — passed.
- `cargo run -p fixture-check -- --game high_card_duel` — passed.
