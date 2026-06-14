# GAT15RIVLEDTEX-011: Property test suite

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/river_ledger/tests/property.rs`
**Deps**: GAT15RIVLEDTEX-007

## Problem

River Ledger needs property tests over random legal action sequences to prove the deterministic-setup, accounting-conservation, and evaluator-ordering invariants hold beyond the hand-picked rule tests, completing the official-game test taxonomy in `docs/TESTING-REPLAY-BENCHMARKING.md`.

## Assumption Reassessment (2026-06-14)

1. `games/poker_lite/tests/property.rs` is the precedent for property coverage; this ticket consumes the rules/betting/showdown behavior from 005/007 and the evaluator from 006.
2. `specs/...-base.md` §7.2 (property tests class) fixes the required properties: deterministic setup, action-sequence invariants, contribution conservation, no negative ledgers, matched-or-folded at street close, evaluator total ordering, split sums to pot, stable serialization.
3. Cross-artifact boundary under audit: property tests drive the public `setup`/`apply`/evaluator/showdown surfaces only (no new production logic); they assert the invariants those modules guarantee.
4. FOUNDATIONS §11 acceptance invariants motivate this ticket: contribution conservation, non-negative ledgers, deterministic setup, and evaluator total ordering are restated as randomized properties rather than trusted from the rule tests alone.

## Architecture Check

1. Randomized property coverage catches invariant violations the fixed rule tests miss, the standard official-game test layer.
2. No backwards-compatibility aliasing/shims — test-only ticket.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4) — tests only.

## Verification Layers

1. Contribution conservation + no negative ledgers over random legal sequences -> `cargo test -p river_ledger --test property`.
2. Evaluator comparison is a deterministic total order -> property ordering test.
3. Deterministic setup + stable serialization under random seeds -> property determinism test.

## What to Change

### 1. `games/river_ledger/tests/property.rs`

Property tests over random legal action sequences for: deterministic setup; contribution conservation (`pot == Σ contributions`); no negative ledgers; all live seats matched-or-folded at street close; evaluator total ordering; split allocation sums to pot; stable serialization.

## Files to Touch

- `games/river_ledger/tests/property.rs` (new)

## Out of Scope

- Rule/golden/visibility/replay tests (owned by 004–010).
- Any production-logic change (this ticket only exercises existing surfaces).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger --test property` — all properties hold over the randomized space.
2. `cargo test -p river_ledger` passes overall.

### Invariants

1. Accounting conservation and non-negative ledgers hold for every random legal sequence (§11).
2. Evaluator ordering is a deterministic total order (§11).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/property.rs` (new) — randomized invariant coverage.

### Commands

1. `cargo test -p river_ledger --test property`
2. `cargo test -p river_ledger`
3. A property-test-scoped command is the correct boundary; fixed-path behavior is covered by the rule/golden tests in 004–010.
