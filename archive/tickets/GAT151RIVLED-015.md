# GAT151RIVLED-015: Integrated rule/property/serialization suite

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/river_ledger/tests/{rules,property,serialization}.rs`
**Deps**: GAT151RIVLED-012

## Problem

Consolidate the full Rust test matrix for the v2 behavior so the whole game proves out under `cargo test --workspace` without weakened tests: conservation, partition/coalescing, eligibility, actor rotation, setup validation, deterministic ordering, and every retained Gate 15 regression. This is the Rust-level integrated suite; WASM/web verification lives in GAT151RIVLED-013/-014, and golden traces in -017.

## Assumption Reassessment (2026-06-20)

1. Code: the behavior landed across GAT151RIVLED-003–012; `games/river_ledger/tests/{rules,property,serialization}.rs` hold per-ticket cases. This ticket fills matrix gaps and asserts the §7.3 property set as one coherent suite, retaining all valid Gate 15 tests.
2. Docs: spec §7.3 enumerates the required test classes and the property assertions (`0 <= contribution <= starting_stack`, `remaining+contribution==starting` pre-settlement, `sum(pots)+sum(returns)==sum(contributions)`, `sum(shares for pot)==pot.amount`, winner-set ⊆ eligible-set non-empty, folded never eligible, no singleton-contributor final layer, `sum(final)==sum(starting)`, canonical-input determinism).
3. Cross-artifact boundary under audit: the union of `state`/`betting`/`pot`/`showdown`/`replay_support` behavior — this suite is the cross-module property harness over all of them.
4. (§11 deterministic + no-weakening) Restate: property generators cover bounded 3–6-seat contribution vectors filtered/classified by validity; no Gate 15 test is deleted or weakened to pass (AGENT-DISCIPLINE failing-test protocol). Confirm same-canonical-input determinism across pots/shares/effects/serialization/hash.

## Architecture Check

1. One integrated property suite catches cross-module interactions (rotation × pots × settlement) that per-ticket unit tests miss.
2. No backwards-compatibility shims; no test is weakened — a genuine failure routes to the system under test.
3. No production code changes here; `engine-core` boundary untouched.

## Verification Layers

1. Conservation/partition/eligibility/ordering properties -> property tests over bounded validated vectors.
2. Actor rotation + setup validation across mixes of live/folded/all-in -> rule tests.
3. Canonical-input determinism (pots/shares/effects/serialization) -> deterministic-order serialization tests.
4. Gate 15 regressions intact -> full `cargo test -p river_ledger` with the prior suite retained.

## What to Change

### 1. Property matrix completion

Extend `tests/property.rs` to assert the full §7.3 property set over generated, validity-classified 3–6-seat contribution vectors.

### 2. Rule + serialization integration

Extend `tests/rules.rs` and `tests/serialization.rs` to cover actor rotation, setup validation, and deterministic ordering across modules, retaining all Gate 15 regression cases.

## Files to Touch

- `games/river_ledger/tests/rules.rs` (modify)
- `games/river_ledger/tests/property.rs` (modify)
- `games/river_ledger/tests/serialization.rs` (modify)

## Out of Scope

- Pairwise N-seat no-leak matrix (GAT151RIVLED-016) and golden traces (GAT151RIVLED-017).
- WASM/web tests (GAT151RIVLED-013, -014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` — green with no weakened or deleted tests.
2. `cargo run -p rule-coverage -- --game river_ledger` — every new rule row points to an exact test (coverage finalized once GAT151RIVLED-019 lands `RULE-COVERAGE.md`).
3. `cargo run -p simulate -- --game river_ledger --games 1000` — integrated behavior is stable across seat/stack profiles.

### Invariants

1. No valid Gate 15 test is removed or weakened.
2. Same canonical input yields the same ordered pots, shares, effects, serialization, and hash.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/property.rs` — full §7.3 property set.
2. `games/river_ledger/tests/rules.rs`, `tests/serialization.rs` — rotation/setup/ordering integration cases.

### Commands

1. `cargo test --workspace`
2. `cargo run -p rule-coverage -- --game river_ledger`
3. `cargo run -p simulate -- --game river_ledger --games 1000` — the workspace suite is the correct integrated boundary; no-leak and traces are separate tickets.

## Outcome

Completed 2026-06-20.

- Extended the River Ledger property matrix with generated 3–6 seat contribution profiles that assert stack bounds, contribution conservation, ordered/coalesced pot layers, non-empty eligibility, folded-seat exclusion, canonical contributor/eligible ordering, and singleton top-layer returns.
- Added a mixed short-stack rule case proving folded and all-in seats stay non-actionable while live seats rotate deterministically across street advance.
- Added an all-in side-pot serialization determinism case comparing repeated canonical input state summaries, view summaries, state-summary hashes, view hashes, ordered tier tokens, and final returned stack/contribution state.
- Retained all existing Gate 15 tests; no tests were deleted or weakened.

Verification:

1. `cargo test -p river_ledger`
2. `cargo test --workspace`
3. `cargo run -p rule-coverage -- --game river_ledger`
4. `cargo run -p simulate -- --game river_ledger --games 1000`
