# GAT4THRMARBOA-009: WASM/API multi-game registry + Three Marks operation-group smoke

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs`, `crates/wasm-api/Cargo.toml`; `apps/web/scripts/smoke-load-wasm.mjs`; new `games/three_marks/tests/golden_traces/wasm-exported.trace.json`
**Deps**: GAT4THRMARBOA-004, GAT4THRMARBOA-005, GAT4THRMARBOA-006, GAT4THRMARBOA-007

## Problem

The Gate 3 WASM shell is hardcoded to `race_to_n` (`RegisteredGame` has only `RaceToN`; `resolve_game` matches `GAME_RACE_TO_N`; replay export/import gate on `GAME_RACE_TO_N`). Gate 4 must extend the static catalog/registry so the existing operation groups (catalog, new match, get view, action tree, apply, run bot, effects, replay export/import/reset/step) serve both `race_to_n` and `three_marks`, with viewer-safe game-specific JSON and no plugin/dynamic loading — and prove it with operation-group smoke coverage.

## Assumption Reassessment (2026-06-06)

1. `crates/wasm-api/src/lib.rs` currently hardcodes one game: `enum RegisteredGame { RaceToN }` (line 98), `resolve_game` matches only `GAME_RACE_TO_N` (line 356), `new_match`/apply/bot dispatch on `RegisteredGame::RaceToN`, and replay import rejects `parsed.game_id != GAME_RACE_TO_N` (line 289) with a hardcoded export format string (line 598). `crates/wasm-api/Cargo.toml` depends on `race_to_n` only. Verified all sites. This ticket generalizes them to a game-resolution layer adding `ThreeMarks`.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §11 (operation groups stay conceptually stable for two games), §11.2 (static minimal registry, no plugin architecture, typed game-specific JSON with discriminated rendering), §15.6 (WASM/API smoke). The Three Marks crate surfaces (view 005, actions/apply 003, effects 004, bots 006, replay 007) are the producers.
3. Cross-crate boundary under audit: `crates/wasm-api` depends on `three_marks` (new Cargo dep, mirroring the `race_to_n` dep) and on the generic `engine-core` operation-group contract; `docs/WASM-CLIENT-BOUNDARY.md` governs that the browser receives viewer-safe typed JSON and infers no rules.
4. FOUNDATIONS §2 (TypeScript does not decide legality; Rust owns behaviour through the boundary) and §11 (browser payloads are viewer-safe) motivate this ticket: the registry routes per game but the browser still receives only Rust-projected legal actions/views/effects/replay.
5. No-leak + determinism enforcement surfaces (§11): the WASM payloads (view, effects, replay export) are the firewall and the deterministic replay surface — name them. Three Marks is perfect-information; the exported `wasm-exported` trace must carry the `not_applicable` hidden-info/private-view rows (mirroring `race_to_n`'s exported trace) and reproduce hashes via `replay-check`.
6. Extends the registry/catalog + replay-export schema, generalizing the hardcoded `race_to_n` export path to be game-parameterized. Consumers: `apps/web` (client + components, GAT4THRMARBOA-010+) and `tools/replay-check` (the exported trace must validate). The change is additive for `race_to_n` (its behaviour and exported format must be preserved — regression-checked) and new for `three_marks`.

## Architecture Check

1. A small static `resolve_game` dispatch over a closed `RegisteredGame` enum is the minimal multi-game mechanism and avoids a plugin/dynamic-loading system (forbidden, spec §4/§11.2). Game-specific view/effect payloads ride as discriminated JSON the TS layer type-guards — cleaner than a lossy common shape.
2. No backwards-compatibility aliasing/shims — `race_to_n` paths are generalized in place, not aliased; its observable ABI/export format is preserved (regression-tested), not shimmed.
3. `engine-core` gains no mechanic nouns; `wasm-api` carries viewer-safe game-specific JSON but no rule logic (validation stays `three_marks::validate_command`, mirroring the existing `race_to_n::validate_command` call at line 191).

## Verification Layers

1. Catalog/registry invariant -> simulation/smoke (list-games returns both `race_to_n` and `three_marks`; `resolve_game("three_marks")` succeeds).
2. Operation-group invariant -> WASM smoke (`crates/wasm-api` `#[cfg(test)]`: start match, get view with board/active-seat/legal-targets/variant, action tree with nine targets, apply mutates + emits effects, run-bot applies a legal action, get-effects, replay export/import + reset/step board-aware projection).
3. Viewer-safe payload invariant -> no-leak visibility test (Three Marks WASM view/effect/replay payloads carry no hidden/debug data; perfect-info `not_applicable` rows present in export).
4. Race-to-N non-regression invariant -> simulation/smoke (existing `race_to_n` operation-group tests and exported-trace format still pass).
5. Exported-trace determinism invariant -> deterministic replay-hash check (the new `wasm-exported.trace.json` reproduces hashes; conforms to `docs/TRACE-SCHEMA-v1.md`).

## What to Change

### 1. `crates/wasm-api/src/lib.rs` — game-resolution layer

Add `RegisteredGame::ThreeMarks`; extend `resolve_game`, `list_games` (include `three_marks` public name/rules/schema), `new_match`, get-view, get-action-tree, apply-action, run-bot, get-effects, and replay export/import/reset/step to dispatch per game. Generalize the hardcoded `GAME_RACE_TO_N` replay-export/import path (lines 81, 289, 328, 598) to a game-parameterized form, preserving `race_to_n`'s exact existing output.

### 2. `crates/wasm-api/Cargo.toml`

Add `three_marks = { path = "../../games/three_marks" }` (mirror the `race_to_n` dependency).

### 3. `apps/web/scripts/smoke-load-wasm.mjs` (extend)

Assert the catalog lists `three_marks` and a `three_marks_standard` match can start through the loaded WASM module.

### 4. `games/three_marks/tests/golden_traces/wasm-exported.trace.json`

The trace produced by the generalized WASM export path for `three_marks`, with `not_applicable` hidden-info/private-view rows and Rust-computed expected hashes.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `crates/wasm-api/Cargo.toml` (modify)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify)
- `games/three_marks/tests/golden_traces/wasm-exported.trace.json` (new)

## Out of Scope

- Web shell catalog/setup UI (GAT4THRMARBOA-010) and the board renderer (011).
- Generalizing the native `tools/replay-check` game gate (GAT4THRMARBOA-014) — this ticket only produces a conformant exported trace.
- Any rule logic in `wasm-api` (forbidden) — validation stays in the game crate.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — multi-game operation-group smoke (both games) passes.
2. `npm --prefix apps/web run smoke:wasm` — WASM loads and a `three_marks` match starts; catalog lists both games.
3. `cargo test --workspace` — `race_to_n` WASM behaviour is non-regressed; `bash scripts/boundary-check.sh` clean.

### Invariants

1. The browser receives only Rust-projected legal actions/views/effects/replay for both games; no rule logic lives in `wasm-api`; no plugin/dynamic loading is introduced.
2. `race_to_n`'s existing operation-group behaviour and exported-trace format are byte-preserved; the `three_marks` exported trace reproduces hashes and carries `not_applicable` perfect-info rows.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` `#[cfg(test)]` — Three Marks operation-group smoke + Race-to-N regression.
2. `apps/web/scripts/smoke-load-wasm.mjs` — catalog + start-match assertion for `three_marks`.
3. `games/three_marks/tests/golden_traces/wasm-exported.trace.json` — deterministic exported-trace fixture.

### Commands

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run smoke:wasm && cargo test --workspace`
3. Browser-rendered board interaction is verified in 011/013; ABI/operation-group smoke is the correct boundary for the registry diff.

## Outcome

Completed: 2026-06-06

Changes:
- Added `three_marks` to the WASM API dependency set, static game registry, catalog output, match store, action/view/effect dispatch, bot turn path, and replay import/export/reset/step operations.
- Preserved Race to N behavior while adding game-specific wrappers for Three Marks state/effects and Rust-validated `place/<cell>` action paths.
- Added the release WASM smoke coverage for `three_marks` catalog/start/action/bot/effects/replay flow.
- Added `games/three_marks/tests/golden_traces/wasm-exported.trace.json` with Rust-computed state/effect/action-tree/public-view/replay hashes and perfect-information `not_applicable` rows.

Deviations:
- The Three Marks catalog row includes a `variants` array so the standard variant is discoverable by the next web setup ticket; the Race to N row is unchanged.
- Native `replay-check --game three_marks` support remains in GAT4THRMARBOA-014 as scoped; this ticket locks the exported trace fixture and WASM import/export path.

Verification:
- `cargo test -p wasm-api`
- `npm --prefix apps/web run smoke:wasm`
- `cargo fmt --all --check`
- `bash scripts/boundary-check.sh`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
