# UNI8CMECSCA-021: Pilot the no-leak harness in River Ledger (N-seat 3–6)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/tests/visibility.rs`, `games/river_ledger/Cargo.toml` (`[dev-dependencies]`)
**Deps**: UNI8CMECSCA-020

## Problem

Prove the no-leak harness across an N-seat hidden-information game: River Ledger at every supported seat count 3–6 and every source-seat × viewer pair. Complete matrices must pass for the public observer plus all seat viewers across public/private effects, action/preview/diagnostic, view/export, and showdown transitions, plus any bot explanation/candidate surface. Betting, all-in/reopen, pot, evaluator, showdown, and allocation rules stay game-owned.

## Assumption Reassessment (2026-06-22)

1. `games/river_ledger/tests/visibility.rs` exists with N-seat private-hand visibility assertions; `tests/golden_traces/{public-replay-export-import,seat-private-view}.trace.json` exist. `game-test-support::no_leak` exists and the two-seat pattern is proven by UNI8CMECSCA-020. River's deal feeds seat-private hands (the UNI8CMECSCA-017 RNG adoption must already be byte-stable).
2. Spec §4.5 + §5 8C-021 fix the boundary: every supported count 3–6 and every source-seat × viewer pair; public/private effects, action/preview/diagnostic, view/export, showdown transitions, bot explanation/candidates leak-safe; betting/pot/showdown stay game-owned. The UNI8CMECSCA-003 packet pins River's seat-private artifacts and canary scopes.
3. Cross-artifact boundary under audit: River's visibility suite and the generic harness. `game-test-support` is added only under `[dev-dependencies]`.
4. FOUNDATIONS §11 no-leak firewall + ADR 0004 (EC-17): no private fact leaks through view, action tree, preview, diagnostic, effect, export, replay, bot explanation/candidates, logs, or test IDs — for any of counts 3/4/5/6 and any source seat.
5. No-leak/visibility surface under audit (§11/EC-17/EC-18): the pilot enumerates the full source × viewer × surface product per count via game-supplied closures; showdown reveal timing and pot/allocation stay game-owned; canaries are test-generated and absent from committed artifacts.

## Architecture Check

1. Exercising the full N-seat matrix proves the harness scales beyond two seats without absorbing River's betting/showdown policy.
2. No backwards-compatibility shim — existing River visibility assertions are retained or strengthened.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` enters only as a dev-dependency.

## Verification Layers

1. Complete matrices pass for observer + all seats at counts 3/4/5/6 across every applicable surface → `cargo test -p river_ledger` (visibility suite, EV-NOLEAK-N).
2. Showdown transitions remain leak-safe; betting/pot/showdown logic unchanged → grep-proof those modules are untouched + `cargo run -p replay-check -- --game river_ledger --all`.
3. No private canary in committed artifacts → grep-proof on `games/river_ledger/tests/golden_traces/*`.
4. `game-test-support` is a dev-only edge → `cargo tree --workspace -e normal --invert game-test-support`.

## What to Change

### 1. `games/river_ledger/Cargo.toml`

Add `game-test-support` under `[dev-dependencies]`.

### 2. `games/river_ledger/tests/visibility.rs`

Adopt `assert_pairwise_no_leak` for the full 3–6 source × viewer × surface matrices with game-supplied closures and test-generated canaries; retain/strengthen existing assertions.

## Files to Touch

- `games/river_ledger/Cargo.toml` (modify)
- `games/river_ledger/tests/visibility.rs` (modify)

## Out of Scope

- Moving betting/all-in/pot/evaluator/showdown/allocation policy into the harness.
- Any normal/build dependency on `game-test-support`.
- Re-characterizing River RNG/deal (UNI8CMECSCA-017 owns that).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` passes the full pairwise matrix for observer + all seats at counts 3, 4, 5, 6.
2. `cargo run -p replay-check -- --game river_ledger --all` passes (showdown transitions leak-safe).
3. Existing River visibility assertions remain present (or strengthened).

### Invariants

1. No private fact leaks through any surface for any count 3–6 or source seat.
2. Betting/pot/showdown/allocation stay in `games/river_ledger`; `game-test-support` is dev-only.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/visibility.rs` — full 3–6 pairwise no-leak matrices (EV-NOLEAK-N) over the retained game-specific assertions.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. The visibility suite plus `replay-check` are the correct boundary — the per-count matrices and showdown exports are exercised together.
