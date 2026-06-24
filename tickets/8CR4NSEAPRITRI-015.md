# 8CR4NSEAPRITRI-015: River Ledger C-05 all-in/side-pot action-tree v1 vectors

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/river_ledger` (focused tests only); deterministic parallel-new v1 vectors, no golden rewrite
**Deps**: 8CR4NSEAPRITRI-014

## Problem

The parallel action-tree v1 adapter from `-014` must be exercised over River's richer all-in/side-pot lifecycle states, not just the base fold/check/call/bet/raise tree. Add parallel-v1 expected byte/hash vectors for the selected richer states using existing golden traces as **read-only** inputs, with no betting/all-in/pot logic change and no golden rewrite (spec §3.7 River, §5.6).

## Assumption Reassessment (2026-06-24)

1. The `-014` parallel-v1 function exists; the read-only input traces `short-small-blind-all-in`, `short-raise-all-in`, `cumulative-reopen`, `all-all-in-runout`, and `three-way-main-two-side-pots` exist under `games/river_ledger/tests/golden_traces/`. Confirmed during `/reassess-spec`.
2. Spec §3.7 classifies the richer vectors as `migrate` (ADR-0009 `parallel-new`); this ticket `Deps` `-014` for the adapter it exercises.
3. Cross-artifact: the v1 encoding contract is `engine-core`-owned; the selected richer states are River test states, not behavior moved into any harness. Baseline legacy hashes come from `-001`.
4. §11 acceptance invariant (deterministic replay/hash) motivates this ticket: the richer v1 vectors are additive and deterministic; existing golden trace bytes stay unchanged (the traces are read-only inputs).
5. Enforcement surface = parallel-v1 vectors for all-in/side-pot-relevant states; adding test vectors changes no production byte and no golden trace.

## Architecture Check

1. Exercising the existing adapter over the richest lifecycle states is cleaner than expanding the adapter — coverage lives in tests, not in new production logic.
2. No backwards-compatibility shim is introduced; no golden trace is regenerated. Rollback removes only these v1 vectors.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4); all-in/side-pot policy stays in `river_ledger`.

## Verification Layers

1. Parallel-v1 vectors deterministic over short/raise/cumulative-reopen/all-all-in-runout/multipot states -> golden / deterministic replay-hash check on the new vectors.
2. Existing golden traces unchanged -> golden trace byte check (read-only inputs; `replay-check --game river_ledger --all` byte-identical to baseline).
3. No logic change -> codebase grep-proof (only test files added/modified; no `src/` betting/pot change).

## What to Change

### 1. Add parallel-v1 expected vectors for richer states

In focused River rules/serialization tests, add parallel-v1 expected byte/hash vectors for at least short blind all-in, short raise, cumulative reopen, all-all-in runout, and three-way main + two side pots, reading the named golden traces as inputs only.

## Files to Touch

- `games/river_ledger/tests/serialization.rs` (modify; or a narrowly added action-tree-v1 test module)

## Out of Scope

- Any change to the `-014` adapter or to legacy hashes.
- Rewriting, regenerating, or byte-changing any golden trace or fixture.
- Any betting, all-in, reopen, pot/side-pot, evaluator, showdown, or allocation logic change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` is green, including the new richer-state parallel-v1 vectors.
2. `cargo run -p replay-check -- --game river_ledger --all` passes with all existing golden trace bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The richer v1 vectors are additive and deterministic; no golden trace byte changes.
2. No all-in/side-pot/allocation logic is altered.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/serialization.rs` (or a narrow new v1 module) — parallel-v1 vectors over the selected all-in/side-pot states.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. The per-game test plus replay-check are the correct boundary: these are game-local action-tree-v1 test states.
