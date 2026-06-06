# GAT5COLFOUPUB-009: Column Four Rust test suite

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/column_four/tests/` (integration tests); unit tests within `games/column_four/src/*`
**Deps**: 004, 006, 008

## Problem

`docs/OFFICIAL-GAME-CONTRACT.md` requires every official game to carry unit, named-rule, property/invariant, serialization, visibility, replay, and bot-legality tests. The per-module tests in 003–008 prove local invariants; this ticket adds the cross-cutting and property/simulation coverage that proves the game holds together end-to-end (spec §18 Rust coverage).

## Assumption Reassessment (2026-06-06)

1. `games/three_marks` carries module unit tests plus a `tests/` integration directory and property-style coverage (verified `games/three_marks/tests/` exists alongside `tests/golden_traces/`). This ticket mirrors that test surface for `column_four` against the modules built in 003 (rules), 004 (visibility), 005 (effects), 006 (replay), 008 (bots).
2. Spec §18 (Rust and workspace checks) enumerates required tests: setup, legal-column, full-column, gravity/landing, turn-alternation, horizontal/vertical/both-diagonal wins, draw, terminal no-actions, stale/invalid commands, visibility/public-view, serialization/stable-hash, replay, bot, and property/simulation. These map to the surfaces from 003–008.
3. Cross-artifact boundary under audit: the test suite exercises the `engine-core` action/effect/view/replay contracts as consumed by `column_four`; it asserts behavior, it does not change any contract.
4. FOUNDATIONS §6 (official games are evidence-heavy) motivates this ticket: a game is not done without unit/rule/property/simulation/replay/serialization/visibility/bot tests.
5. Deterministic replay/hash & serialization and the no-leak visibility firewall (§11) are the enforcement surfaces under audit: serialization tests assert stable order and stable hashing; visibility tests assert no hidden/internal field leaks; replay tests assert byte-identical projections — confirming the 004/006 substrate does not introduce a leak or nondeterminism path.

## Architecture Check

1. A dedicated cross-cutting test ticket keeps each implementation ticket (003–008) reviewable while still proving the official-game evidence bar in one auditable place — cleaner than inflating every implementation ticket with full-matrix tests.
2. No backwards-compatibility aliasing/shims — tests only.
3. No production code changes; `engine-core` and `game-stdlib` untouched.

## Verification Layers

1. Rule-matrix invariant -> named-rule tests: setup, legal/full column, gravity, turn alternation, H/V/both-diagonal win, draw, terminal no-actions, stale/invalid commands.
2. Serialization invariant -> schema/serialization validation: stable serialization order and stable hashing for state/view/effects.
3. Visibility invariant -> no-leak visibility test: public view exposes no hidden/internal field (perfect-information marker present).
4. Replay invariant -> deterministic replay-hash check: double-replay equality of view/effect/terminal hashes.
5. Bot invariant -> bot legality check: bots act only via the legal action tree; terminal states yield no illegal action.
6. Property/simulation invariant -> simulation/CLI run: randomized games never reach an illegal state or non-terminating loop.

## What to Change

### 1. `games/column_four/tests/`

Add integration tests covering the spec §18 matrix that span multiple modules (full game playthroughs to win/draw, serialization round-trips, replay-hash equality, visibility no-leak assertions, bot-legality across many seeds, and a property/invariant test that random legal play always terminates in win or draw).

### 2. `games/column_four/src/*` unit tests

Backfill any §18 unit cases not already covered by 003–008 (e.g. turn-alternation across a long game, stable-hash assertions).

## Files to Touch

- `games/column_four/tests/rules.rs` (new)
- `games/column_four/tests/replay.rs` (new)
- `games/column_four/tests/visibility.rs` (new)
- `games/column_four/tests/bots.rs` (new)
- `games/column_four/tests/property.rs` (new)

## Out of Scope

- Golden trace fixtures (GAT5COLFOUPUB-010) and the simulation-tool registration (GAT5COLFOUPUB-013) — this ticket may invoke in-crate simulation helpers but does not wire `tools/simulate`.
- Benchmarks (GAT5COLFOUPUB-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p column_four` — full unit + integration matrix passes.
2. `cargo test --workspace` — no regression to other crates.
3. `cargo clippy -p column_four --all-targets -- -D warnings` — tests lint-clean.

### Invariants

1. Every spec §18 Rust-coverage rule has at least one named test.
2. Serialization order/hash, replay projection, and public-view no-leak are each asserted by a distinct test.

## Test Plan

### New/Modified Tests

1. `games/column_four/tests/rules.rs` — rule matrix (legality, gravity, win 4-dir, draw, terminal).
2. `games/column_four/tests/replay.rs` — deterministic replay-hash equality.
3. `games/column_four/tests/visibility.rs` — public-view no-leak + stable serialization.
4. `games/column_four/tests/bots.rs` — bot legality/determinism across seeds.
5. `games/column_four/tests/property.rs` — random legal play always terminates in win/draw.

### Commands

1. `cargo test -p column_four`
2. `cargo test --workspace`
3. `cargo clippy -p column_four --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-06

What changed:

- Added `games/column_four/tests/rules.rs` for cross-module rule coverage: legal columns, full-column rejection, gravity, turn alternation, stale/wrong/invalid/unknown/terminal diagnostics, horizontal/vertical/rising/falling wins, draw, terminal no-actions, and win-over-draw precedence.
- Added `games/column_four/tests/replay.rs` for deterministic replay hash equality, projection hash reuse, replay JSON stable serialization, and unknown-field rejection.
- Added `games/column_four/tests/visibility.rs` for public-view no-leak checks, explicit perfect-information private status, stable serialization, stable cell order, terminal no-actions, and public piece token metadata.
- Added `games/column_four/tests/bots.rs` for Level 0 and Level 2 legal action validation across seeds, deterministic Level 2 explanations, no score/candidate/debug leakage, and terminal no-action behavior.
- Added `games/column_four/tests/property.rs` for many-seed random legal playout termination within the 42-ply bound.

Deviations from original plan:

- None. Golden trace fixtures, simulation tool registration, and benchmark coverage remain deferred to their dedicated tickets.

Verification results:

- Passed: `cargo test -p column_four`
- Passed: `cargo test --workspace`
- Passed: `cargo clippy -p column_four --all-targets -- -D warnings`
- Passed: `cargo fmt --all --check`
