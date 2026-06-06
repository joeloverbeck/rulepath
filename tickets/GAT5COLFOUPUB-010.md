# GAT5COLFOUPUB-010: Column Four golden traces & fixtures

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/column_four/tests/golden_traces/*.trace.json`, `games/column_four/data/fixtures/*`
**Deps**: 006, 008

## Problem

`column_four` needs golden traces and fixtures as deterministic regression evidence covering each win direction, draw, every diagnostic, a bot action, terminal replay, and a WASM-exported trace (spec §13.2). The draw trace is long and easy to get wrong because accidental winning lines are common (spec §21 draw-trace risk), so it must be built deliberately and validated through Rust replay.

## Assumption Reassessment (2026-06-06)

1. `games/three_marks/tests/golden_traces/` holds the trace set (`shortest-normal`, `draw`, `bot-action`, `terminal`, `wasm-exported`, `stale-diagnostic`, `occupied-diagnostic`, `not-applicable`) and `games/three_marks/data/fixtures/` holds fixtures alongside `data/manifest.toml`/`data/variants.toml` (verified). `column_four` mirrors this; `manifest.toml`/`variants.toml` were created in GAT5COLFOUPUB-002, so this ticket adds the `fixtures/` contents and the trace set.
2. Spec §13.2 lists the required traces: `shortest-normal-win`, `vertical-win`, `horizontal-win`, `diagonal-win`, `draw`, `stale-diagnostic`, `invalid-column-diagnostic`, `full-column-diagnostic`, `bot-action`, `terminal-replay`, `wasm-exported`. Replay projection (006) and the bot (008) produce the trace inputs; Trace Schema v1 (`docs/TRACE-SCHEMA-v1.md`) governs the format.
3. Cross-artifact boundary under audit: `docs/TRACE-SCHEMA-v1.md` (valid root fields, no unknown/behavior-looking keys, no duplicate IDs) and the replay/hash contract. Traces are fixtures and regression evidence, never rule authorities.
4. FOUNDATIONS §11 (replay/traces remain deterministic) and §6 (golden traces are required evidence) motivate this ticket.
5. Deterministic replay/hash is the enforcement surface: each trace must round-trip through GAT5COLFOUPUB-006 replay with stable hashes, and the draw trace must be Rust-validated to contain no accidental winning line — confirming no nondeterminism path before `replay-check` registration (GAT5COLFOUPUB-013) enforces it in CI.

## Architecture Check

1. Authoring traces from the Rust replay engine (rather than hand-editing JSON) guarantees schema-valid, deterministic, rule-consistent fixtures — cleaner and the only way to keep the draw trace honest. Alternative (hand-built JSON) risks accidental-line and hash drift.
2. No backwards-compatibility aliasing/shims — new fixture files.
3. No production code; `engine-core`/`game-stdlib` untouched. Traces carry no behavior-looking keys (§5/§11).

## Verification Layers

1. Trace-coverage invariant -> codebase grep-proof: every spec §13.2 trace file exists with the documented coverage.
2. Schema-validity invariant -> schema/serialization validation: each trace conforms to Trace Schema v1 (no unknown/behavior-looking keys, no duplicate IDs).
3. Replay-consistency invariant -> deterministic replay-hash check: each trace replays through GAT5COLFOUPUB-006 to its expected terminal outcome with stable hashes.
4. Draw-correctness invariant -> deterministic replay-hash check: the draw trace fills the board with no winning line (Rust-validated, not asserted by hand).

## What to Change

### 1. `games/column_four/tests/golden_traces/`

Author the trace set: `shortest-normal-win.trace.json`, `vertical-win.trace.json`, `horizontal-win.trace.json`, `diagonal-win.trace.json`, `draw.trace.json`, `stale-diagnostic.trace.json`, `invalid-column-diagnostic.trace.json`, `full-column-diagnostic.trace.json`, `bot-action.trace.json`, `terminal-replay.trace.json`, `wasm-exported.trace.json` (filenames may follow repo convention if RULE-COVERAGE makes the mapping unmistakable).

### 2. `games/column_four/data/fixtures/`

Add fixture metadata mirroring `three_marks` (typed fixture entries referencing the variant; no behavior fields), consistent with `data/manifest.toml`/`data/variants.toml` from GAT5COLFOUPUB-002.

## Files to Touch

- `games/column_four/tests/golden_traces/shortest-normal-win.trace.json` (new)
- `games/column_four/tests/golden_traces/vertical-win.trace.json` (new)
- `games/column_four/tests/golden_traces/horizontal-win.trace.json` (new)
- `games/column_four/tests/golden_traces/diagonal-win.trace.json` (new)
- `games/column_four/tests/golden_traces/draw.trace.json` (new)
- `games/column_four/tests/golden_traces/stale-diagnostic.trace.json` (new)
- `games/column_four/tests/golden_traces/invalid-column-diagnostic.trace.json` (new)
- `games/column_four/tests/golden_traces/full-column-diagnostic.trace.json` (new)
- `games/column_four/tests/golden_traces/bot-action.trace.json` (new)
- `games/column_four/tests/golden_traces/terminal-replay.trace.json` (new)
- `games/column_four/tests/golden_traces/wasm-exported.trace.json` (new)
- `games/column_four/data/fixtures/` (new)

## Out of Scope

- `tools/replay-check` / `tools/fixture-check` registration (GAT5COLFOUPUB-013) — this ticket produces the data those tools will validate.
- RULE-COVERAGE.md trace-catalog rows (GAT5COLFOUPUB-013).

## Acceptance Criteria

### Tests That Must Pass

1. `ls games/column_four/tests/golden_traces/*.trace.json | wc -l` — the full trace set is present (≥11 or the documented count).
2. `cargo test -p column_four replay` — every trace replays to its expected terminal outcome with stable hashes.
3. Draw-trace check: the `draw` trace fills 42 cells with no winning line (validated via the Rust replay/visibility path).

### Invariants

1. Every spec §13.2 coverage category has a trace; each is Trace-Schema-v1-valid with no behavior-looking keys.
2. The draw trace is genuinely terminal-by-draw (no accidental line), proven by Rust replay.

## Test Plan

### New/Modified Tests

1. `games/column_four/tests/golden_traces/*.trace.json` — the trace set itself, each replay-validated.
2. `games/column_four/data/fixtures/*` — fixture metadata validated against the variant.

### Commands

1. `cargo test -p column_four replay`
2. `ls games/column_four/tests/golden_traces/*.trace.json`
3. `cargo run -p replay-check -- --game column_four --all` — note: passes only after GAT5COLFOUPUB-013 registers the game; until then the in-crate replay test is the verification boundary.
