# GAT5COLFOUPUB-006: Column Four replay support (Rust-owned projection)

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/column_four/src/replay_support.rs`
**Deps**: 004, 005

## Problem

`column_four` must be fully replayable: given the same trace document, seed, variant, rules version, and command sequence, replay must reproduce identical public-view and effect projections at each cursor. Replay projection must be Rust-owned so the web replay viewer never reconstructs board state from effect logs (spec §13, FOUNDATIONS §2/§11 determinism).

## Assumption Reassessment (2026-06-06)

1. `games/three_marks/src/replay_support.rs` is the template: it reconstructs state from a command stream and projects the same Rust public view at each replay step. Verified the module exists and `engine-core` provides the generic `replay`/`checkpoint`/`hash` contracts (`crates/engine-core/src/replay.rs`). This ticket mirrors it for `column_four`.
2. Spec §13.1 (deterministic replay) and §13.3 (trace schema compatibility) define behavior: replay reproduces public-view/effect/terminal hashes per Trace Schema v1; exported `column_four` traces use `game_id: "column_four"`, `rules_version: "column_four-rules-v1"`, default variant. The public view (004) and effects (005) are the projection surfaces reused here.
3. Cross-artifact boundary under audit: the `engine-core` replay/checkpoint/hash contract and `docs/TRACE-SCHEMA-v1.md` (valid root fields, no unknown/behavior-looking keys, no duplicate IDs). This ticket produces Trace-Schema-v1-compatible replay behavior; it adds no mechanic noun to `engine-core`.
4. FOUNDATIONS §11 (replay, hashes, serialization order, RNG, and traces remain deterministic) and §2 (replay/hash behavior is Rust-owned) motivate this ticket. Restating: identical inputs + versions MUST yield identical projections; no wall-clock or hash-map-iteration nondeterminism may enter canonical forms. This is substrate for the golden-trace and replay-check enforcement landing in GAT5COLFOUPUB-010/013.

## Architecture Check

1. Reusing the 004 public-view projection at each replay cursor (rather than a parallel replay-only renderer) guarantees replay and live views cannot diverge — cleaner and the only determinism-safe design. Alternative (TS reconstructs from effects) is forbidden by §13/FOUNDATIONS §2.
2. No backwards-compatibility aliasing/shims — new module composing 003/004/005.
3. `engine-core` stays free of mechanic nouns (replay uses generic checkpoint/hash contracts); `game-stdlib` untouched.

## Verification Layers

1. Deterministic-replay invariant -> deterministic replay-hash check: replaying a fixed command stream twice yields byte-identical public-view/effect/terminal hashes.
2. Projection-reuse invariant -> codebase grep-proof: replay step projects through the 004 public-view function, not a separate renderer.
3. Schema-compatibility invariant -> schema validation: exported replay documents satisfy Trace Schema v1 (no unknown/behavior-looking keys, correct `game_id`/`rules_version`).
4. RNG/order invariant -> FOUNDATIONS alignment check (§11): no nondeterministic input enters canonical replay forms.

## What to Change

### 1. `games/column_four/src/replay_support.rs`

Implement command-stream reconstruction and per-cursor projection reusing the 004 public view and 005 effect log; expose replay reset/step semantics; ensure exported traces carry `game_id: "column_four"`, `rules_version: "column_four-rules-v1"`, and the default variant, with stable serialization order and deterministic hashing per `engine-core`'s contract.

## Files to Touch

- `games/column_four/src/replay_support.rs` (new)

## Out of Scope

- Golden trace fixtures and the `replay-check` tool registration (GAT5COLFOUPUB-010/013).
- WASM replay export/import wiring (GAT5COLFOUPUB-012) and the web replay viewer (014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p column_four replay` — deterministic replay-hash and projection-reuse tests pass.
2. `cargo test -p column_four` — no regression.
3. `cargo build --workspace` — engine-core replay contract still satisfied.

### Invariants

1. Replaying identical inputs+versions yields identical public-view/effect/terminal hashes.
2. Replay projection reuses the Rust public view; no separate replay renderer exists.

## Test Plan

### New/Modified Tests

1. `games/column_four/src/replay_support.rs` (unit tests) — double-replay hash equality, schema-compatible export fields, deterministic serialization order.

### Commands

1. `cargo test -p column_four replay`
2. `cargo test -p column_four`
3. `cargo clippy -p column_four --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-06

What changed:

- Added `games/column_four/src/replay_support.rs` with command-stream replay, default seats, command construction, per-step projections, stable state/effect/action-tree/view/replay hashes, effect stable strings, and a schema-shaped `ColumnFourReplayJson`.
- Reused the Rust `project_view` projection for replay view hashes and per-step public-view projection evidence.
- Exported the replay module from the crate root.
- Added tests for identical replay hash reproduction, terminal outcome/projection evidence, schema fields (`column_four`, `column_four-rules-v1`, `column_four_standard`), unknown-field rejection, and stable hash equality for repeated command streams.

Deviations from original plan:

- Golden trace fixtures and tool registration remain deferred to GAT5COLFOUPUB-010 and GAT5COLFOUPUB-013, as scoped by this ticket.

Verification results:

- Passed: `cargo test -p column_four replay`
- Passed: `cargo test -p column_four`
- Passed: `cargo clippy -p column_four --all-targets -- -D warnings`
- Passed: `cargo fmt --all --check`
- Passed: `cargo build --workspace`
