# GAT4THRMARBOA-007: Three Marks replay support, hashes, golden traces + replay/serialization tests

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `games/three_marks/src/replay_support.rs`, hash surfaces; new `tests/replay_tests.rs`, `tests/serialization_tests.rs`, `tests/golden_traces/*.trace.json`
**Deps**: GAT4THRMARBOA-004, GAT4THRMARBOA-005, GAT4THRMARBOA-006

## Problem

Three Marks must support deterministic, board-aware replay: export/import viewer-safe traces, reproduce final state/effect/action-tree/public-view/replay hashes, and project the board (plus step effects and winning-line/draw) at each replay step from Rust — not from TypeScript diffs. Golden traces must fail loudly on drift. This is the determinism evidence FOUNDATIONS §11 and `docs/TESTING-REPLAY-BENCHMARKING.md` require.

## Assumption Reassessment (2026-06-06)

1. `games/race_to_n/src/replay_support.rs` is the mirror: it exposes `replay_commands`, `replay_bot_action`, `replay_invalid`, and `ReplayHashes` (imported by `tools/replay-check/src/main.rs:9`). `games/race_to_n/tests/golden_traces/` holds `shortest-normal`, `terminal`, `not-applicable`, `invalid-stale-diagnostic`, `bot-action`, `wasm-exported` traces — `three_marks` mirrors these adapted to a board game. Verified those files exist.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §13 (board-aware replay), §15.3 (golden trace set), §15.4 (replay tests reproduce hashes/outcome/projection). State/view from GAT4THRMARBOA-005, effects from 004, bots from 006 (bot-action trace).
3. Cross-artifact boundary under audit: the trace/replay schema in `docs/TRACE-SCHEMA-v1.md` and `docs/TESTING-REPLAY-BENCHMARKING.md`, and the `tools/replay-check` consumer (which currently gates `game_id == "race_to_n"` / `rules_version == "race_to_n-rules-v1"` at `tools/replay-check/src/main.rs:269-274` — that generalization is GAT4THRMARBOA-014, but the `three_marks` traces produced here must conform to the schema replay-check enforces).
4. FOUNDATIONS §2 (Rust owns replay and hash behaviour) and §11 (replay, hashes, serialization order, RNG, traces remain deterministic) motivate this ticket: identical seed/options/commands + versions reproduce identical hashes and outcome; canonical forms use stable serialization order and the engine's deterministic RNG only.
5. Deterministic replay/hash enforcement surface (§11/§13): `ReplayHashes` and the trace `expected_*_hashes` are the surface — name them. This is a *new game's* additive replay support, not a change to replay/hash *semantics*, so no §13 ADR is triggered; confirm no wall-clock/hash-map-iteration nondeterminism enters canonical forms and that traces carry no hidden state (perfect information → `not_applicable` hidden-info row).
6. Extends the golden-trace / checkpoint / hash schema with `three_marks` traces and hash surfaces. Consumer: `tools/replay-check` (generalized in 014). The extension is additive (a second game's conformant traces); the `wasm-exported` trace is produced by the WASM export path and added in GAT4THRMARBOA-009.

## Architecture Check

1. Authoring replay projection in Rust (board + step effects + outcome) keeps TypeScript a pure presenter and makes replay byte-reproducible — cleaner and matching `race_to_n`. Alternative (TS reconstructs board from view diffs) is a §11/§12 stop condition and rejected.
2. No backwards-compatibility aliasing/shims — new module/tests.
3. `engine-core` gains no board/line nouns; replay projection's board vocabulary is local to `games/three_marks/src/replay_support.rs`; no `game-stdlib` extraction.

## Verification Layers

1. Hash-reproduction invariant -> deterministic replay-hash check (`tests/replay_tests.rs`: reproduce final state, effect, action-tree, public-view, replay hashes).
2. Outcome/terminal/projection invariant -> golden trace / replay test (replay reaches expected win/draw outcome and terminal flag; projection contains board, marks, step effects, winning line/draw).
3. Serialization round-trip invariant -> schema/serialization validation (`tests/serialization_tests.rs`: state/view/replay/action surfaces and relevant hashes survive round trips; stable order).
4. Golden-trace drift invariant -> golden trace check (each `tests/golden_traces/*.trace.json` fails loudly on hash/outcome drift; schema conforms to `docs/TRACE-SCHEMA-v1.md`).

## What to Change

### 1. `src/replay_support.rs` + hash surfaces

Mirror `race_to_n::replay_support` (`replay_commands`, `replay_bot_action`, `replay_invalid`, `ReplayHashes`): replay a command log to reproduce hashes; board-aware step projection (board at step + step effects + winning-line/draw); stable serialization/hash surfaces for state, public view, action tree, effects, and replay.

### 2. `tests/golden_traces/*.trace.json`

Shortest-normal win (five-ply), representative draw (nine-ply), terminal, occupied-cell diagnostic, stale-action diagnostic, bot-action (prefer Level 1 if stable), and not-applicable (documents hidden-info + stochastic-rule-events as not applicable). Each carries expected hashes/outcome per schema. (The `wasm-exported` trace lands in GAT4THRMARBOA-009.)

### 3. `tests/replay_tests.rs` and `tests/serialization_tests.rs`

§15.4 replay tests (reproduce each hash, outcome, terminal, board-aware projection) and the §15.2 serialization round-trip suite over state/view/replay/action surfaces.

## Files to Touch

- `games/three_marks/src/replay_support.rs` (new)
- `games/three_marks/src/lib.rs` (modify)
- `games/three_marks/tests/replay_tests.rs` (new)
- `games/three_marks/tests/serialization_tests.rs` (new)
- `games/three_marks/tests/golden_traces/shortest-normal.trace.json` (new)
- `games/three_marks/tests/golden_traces/draw.trace.json` (new)
- `games/three_marks/tests/golden_traces/terminal.trace.json` (new)
- `games/three_marks/tests/golden_traces/occupied-diagnostic.trace.json` (new)
- `games/three_marks/tests/golden_traces/stale-diagnostic.trace.json` (new)
- `games/three_marks/tests/golden_traces/bot-action.trace.json` (new)
- `games/three_marks/tests/golden_traces/not-applicable.trace.json` (new)

## Out of Scope

- The `wasm-exported` golden trace and WASM export path (GAT4THRMARBOA-009).
- Generalizing `tools/replay-check` to accept `--game three_marks` (GAT4THRMARBOA-014).
- Board-aware replay *UI* (GAT4THRMARBOA-012).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p three_marks --test replay_tests --test serialization_tests` — hash reproduction, outcome, projection, and round-trips pass.
2. `cargo test -p three_marks` — full crate suite green (golden traces load and match).
3. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Identical seed/options/commands + versions reproduce identical state/effect/action-tree/public-view/replay hashes and the same outcome/terminal state.
2. Every golden trace conforms to `docs/TRACE-SCHEMA-v1.md`, carries no hidden state, and fails loudly on drift.

## Test Plan

### New/Modified Tests

1. `games/three_marks/tests/replay_tests.rs` — hash/outcome/terminal/projection reproduction.
2. `games/three_marks/tests/serialization_tests.rs` — state/view/replay/action round-trips.
3. `games/three_marks/tests/golden_traces/*.trace.json` — drift-detecting fixtures (win/draw/terminal/diagnostic/bot/not-applicable).

### Commands

1. `cargo test -p three_marks --test replay_tests --test serialization_tests`
2. `cargo test -p three_marks && bash scripts/boundary-check.sh`
3. `replay-check --game three_marks` is exercised in 014 (tool generalization); crate-level replay/serialization tests are the correct boundary for the replay-support diff.

## Outcome

Completed: 2026-06-06

What changed:

- Added `games/three_marks/src/replay_support.rs` with deterministic replay helpers, replay hash surfaces, effect/action-tree/view/replay hashes, diagnostic hashes, default seats, command creation, bot replay, diagnostic replay, and board-aware step projections.
- Added `ThreeMarksReplayJson` stable serialization and round-trip parsing.
- Added replay tests proving deterministic hash reproduction, terminal outcome, board-aware projection, and golden trace drift detection.
- Added serialization tests for public view round-trip/unknown-field rejection, replay JSON round-trip, stable serialization order, and replay-ready command envelopes.
- Added golden Trace Schema v1 fixtures for shortest win, draw, terminal, occupied diagnostic, stale diagnostic, Level 1 bot action, and not-applicable surfaces.

Deviations from original plan:

- The `wasm-exported` trace remains deferred to GAT4THRMARBOA-009.
- `tools/replay-check --game three_marks` remains deferred to GAT4THRMARBOA-014; this ticket proves the crate-level replay/hash surfaces and conformant fixtures.

Verification results:

- `cargo fmt --all --check`
- `cargo test -p three_marks --test replay_tests --test serialization_tests`
- `cargo test -p three_marks`
- `bash scripts/boundary-check.sh`
