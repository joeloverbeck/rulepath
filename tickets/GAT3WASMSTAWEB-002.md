# GAT3WASMSTAWEB-002: Add feature/version report + list_games WASM ops

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api` gains a feature/version report op and a `list_games` op (new public fns + `#[no_mangle]` extern wrappers). No `engine-core`/`game-stdlib`/`games/*` change; no mechanic noun enters the kernel.
**Deps**: None

## Problem

The browser shell needs Rust-supplied game metadata and a diagnostics-grade
feature/version report (spec §9.4 "Feature/version report", "List games"; §7.2
game picker "Use game metadata supplied by Rust/WASM where practical, not
hardcoded React behavior authority"). Today `crates/wasm-api` exposes only
`new_match`, `get_view`, `get_action_tree`, `apply_action`, `run_bot_turn`,
`get_effects` plus a `placeholder_version` string. The picker (GAT3WASMSTAWEB-005)
would otherwise hardcode `race_to_n` in React, violating §7.2 / FOUNDATIONS §2.

## Assumption Reassessment (2026-06-06)

1. Current Rust surface (verified `crates/wasm-api/src/lib.rs`): six `pub fn`
   operations (lines 44–167) over a `thread_local` `MATCHES` store; the only
   version surface is `placeholder_version() -> "rulepath-wasm-api/0.1.0"`
   (line 40) and the `rulepath_placeholder_version_ptr/len` externs (lines 369–377).
   `RegisteredGame::RaceToN` + `resolve_game` (lines 35–39, 169–177) already model
   the single registered game; `GAME_RACE_TO_N`/`RULES_VERSION` consts exist
   (lines 19–20). No `list_games`, no feature report.
2. Spec §9.4 marks both ops "Required for Gate 3 minimum"; §9.5 mandates the
   stable normalized response contract (status/data/diagnostic, no panic text);
   §9.7 requires documenting the op groups (owned by docs ticket -015).
3. Cross-artifact boundary under audit: the `crates/wasm-api` JSON response
   contract consumed by the TypeScript client (`apps/web/src/wasm/client.ts`,
   created by -001) and the node smoke scripts. New responses MUST follow the
   existing hand-rolled JSON shape (`escape_json`, object/array literals) and the
   `write_result` status convention (0 = ok, 1 = error) used by every extern.
4. FOUNDATIONS §2 (behavior authority): game catalog identity and version/rules
   metadata are Rust-owned facts. Restated: TypeScript MUST render `list_games`
   output, never synthesize the catalog. The feature report lists operation names
   for diagnostics only — it grants no behavior to TS.
5. Schema/contract extension: this adds two **new** response shapes to the
   wasm-api surface (a `list_games` array of game metadata: `game_id`,
   `display_name`, `rules_version`, `schema_version`; a feature report: `api_version`,
   supported operation names/flags). Consumers: the TS client (-001), the picker
   (-005), the dev panel (-010), the WASM/API smoke (-012). The change is
   additive-only — existing ops and their JSON are untouched, so no consumer
   breaks; new consumers opt in.

## Architecture Check

1. Reusing the existing `RegisteredGame`/`resolve_game` registry to source
   `list_games` (rather than a parallel hardcoded list) keeps one game-registry
   authority in Rust and matches the existing op style. The feature report reads
   the op set the crate actually exports — single source of truth for diagnostics.
2. No backwards-compatibility shims: `placeholder_version` is superseded by the
   feature/version report in the same crate; the new report becomes the
   version-of-record. Keep `placeholder_version` only if still needed by an
   existing test, otherwise fold it into the report (no parallel version paths).
3. `engine-core` stays free of mechanic nouns — `list_games` returns opaque
   game-identity metadata strings, not mechanic vocabulary; all new code is in
   `crates/wasm-api`, never the kernel. `game-stdlib` untouched.

## Verification Layers

1. `list_games` returns Rust-owned `race_to_n` metadata → schema/serialization
   validation: a Rust unit test asserts the JSON contains `"game_id":"race_to_n"`
   and the rules/schema version fields.
2. Feature report lists the real op set → codebase grep-proof + unit test: the
   reported operation names match the crate's exported `pub fn`/extern set.
3. Normalized response contract upheld → schema validation: new ops route through
   `write_result`; success → status 0 typed JSON, failure → status 1 typed
   diagnostic; no panic/backtrace text (spec §9.5).
4. Catalog is Rust-authoritative → FOUNDATIONS §2 alignment check: no game
   identity literal is introduced in `apps/web` by this ticket.

## What to Change

### 1. `crates/wasm-api/src/lib.rs` — `list_games`

Add `pub fn list_games() -> Result<String, String>` returning a JSON array of the
registered games (today: one). Each entry carries `game_id` (`GAME_RACE_TO_N`),
`display_name` (e.g. `"Race to 21"`), `rules_version` (`RULES_VERSION`), and a
`schema_version`. Add a `#[no_mangle] pub extern "C" fn rulepath_list_games() ->
i32` wrapping it via `write_result`/`write_output`.

### 2. `crates/wasm-api/src/lib.rs` — feature/version report

Add `pub fn feature_report() -> Result<String, String>` returning `api_version`
and the supported operation names (and any feature flags) for diagnostics. Add
`#[no_mangle] pub extern "C" fn rulepath_feature_report() -> i32`. Fold the
placeholder version string into the report's `api_version` (remove the now-redundant
placeholder path if no test depends on it; otherwise keep one source).

### 3. Rust unit tests

Add `#[cfg(test)]` cases asserting `list_games` and `feature_report` JSON shape
and that they route through the status convention.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify) — add `list_games` + `feature_report` ops, externs, and unit tests

## Out of Scope

- The TypeScript client wrappers for these ops — added by their consumers (`list_games` in GAT3WASMSTAWEB-005, feature report in GAT3WASMSTAWEB-010).
- Replay export/import/step ops — GAT3WASMSTAWEB-003.
- Multi-game catalog content (still one game) — out of gate per spec §5.
- WASM/API smoke coverage of the new ops — GAT3WASMSTAWEB-012.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — new `list_games`/`feature_report` unit tests pass alongside existing surface tests.
2. `cargo build -p wasm-api --target wasm32-unknown-unknown --release` — new externs compile to the WASM target.
3. `cargo clippy -p wasm-api --all-targets -- -D warnings` — no lint regressions.

### Invariants

1. The game catalog and version/rules metadata are produced by Rust; `apps/web` introduces no game-identity literal for them.
2. New ops normalize to the §9.5 response contract (status 0/1, typed JSON, no panic/backtrace text).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` (`#[cfg(test)] mod tests`) — `list_games_reports_race_to_n` and `feature_report_lists_ops` rationale: pin the new JSON shapes the TS client/picker/smoke depend on.

### Commands

1. `cargo test -p wasm-api`
2. `cargo build -p wasm-api --target wasm32-unknown-unknown --release`
3. `cargo clippy -p wasm-api --all-targets -- -D warnings`
