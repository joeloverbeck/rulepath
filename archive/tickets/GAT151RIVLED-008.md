# GAT151RIVLED-008: Contribution-layer constructor

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger` (`pot.rs`), tests
**Deps**: GAT151RIVLED-004

## Problem

Replace the single-pot-only helper with a pure game-local contribution-layer constructor: from per-seat total contributions and fold status, build ascending contribution-cap layers, include folded money in amounts, exclude folded seats from eligibility, extract a singleton unmatched top layer as an uncalled return, coalesce adjacent layers with identical eligibility, and assign stable deterministic pot ids — all under checked integer arithmetic. This ticket contains no hand evaluation. It is a parallel branch off the stack types (GAT151RIVLED-004) and does not depend on betting closure.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/src/pot.rs` has `allocate_single_pot(...)`, `PotAllocation { pot_total, winners, shares, remainder, remainder_order }`, and `winners_in_button_order(winners, button, seat_count)`. There is no layered constructor; this ticket adds one alongside (the per-pot allocator that *uses* it lands in GAT151RIVLED-009).
2. Docs: spec §3.3(5) gives the exact pure algorithm (sorted distinct caps → contributor/eligible sets per level → singleton return vs. segment → coalesce identical-eligibility segments → stable ids → `assert sum(pots)+sum(returns)==sum(c)`). Spec §9 forbids one-pot-per-folded-cap without coalescing.
3. Cross-artifact boundary under audit: the resolved-pot/uncalled-return structures produced here are consumed by `showdown.rs` (GAT151RIVLED-009), `effects.rs`/`visibility.rs` (GAT151RIVLED-010), and the serialization/hash surface (GAT151RIVLED-011); shape them deterministically now.
4. (§11 deterministic ordering) The constructor must be order-independent of map/container iteration: pots, contributor lists, and eligible lists are in canonical seat order and ascending-cap order. Confirm no `HashMap` iteration leaks into output ordering — this determinism is what GAT151RIVLED-011 hashes and GAT151RIVLED-017 captures.

## Architecture Check

1. A pure function from contributions+statuses to ordered pots+returns is independently testable (permutation/property) and keeps allocation policy separable from hand evaluation.
2. No backwards-compatibility shims; `allocate_single_pot` is superseded by the layered path, not aliased (the single-pot case becomes one layer).
3. Pot/eligibility nouns stay game-local in `pot.rs`; `engine-core` is untouched and no `game-stdlib` promotion occurs (per GAT151RIVLED-002).

## Verification Layers

1. Layer construction over contribution permutations -> pure unit + property tests (same multiset → same ordered pots).
2. Folded money retained in amounts; folded seats never eligible -> unit tests.
3. Singleton top layer returned, not made a pot; identical-eligibility layers coalesced -> unit tests (no artificial remainder boundary).
4. Conservation `sum(pots)+sum(returns)==sum(contributions)` -> checked-arithmetic assertion + property test.

## What to Change

### 1. Pure layered constructor

Implement the spec §3.3(5) algorithm in `pot.rs`: sorted distinct positive caps, per-level contributor and eligible sets, singleton-contributor uncalled returns, identical-eligibility coalescing, stable ascending-cap pot ids, checked conservation assertion.

### 2. Resolved-pot / return structures

Add the ordered resolved-pot and uncalled-return data structures (canonical seat order, ascending-cap order) that downstream tickets consume.

## Files to Touch

- `games/river_ledger/src/pot.rs` (modify)
- `games/river_ledger/tests/property.rs` (modify)

## Out of Scope

- Per-pot winner evaluation, split, and remainder allocation (GAT151RIVLED-009).
- Effects/projections of pots (GAT151RIVLED-010) and hashing/serialization (GAT151RIVLED-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — pure layer construction, permutation/property invariance, folded-money retention, singleton return, identical-eligibility coalescing, conservation.
2. `cargo run -p rule-coverage -- --game river_ledger` — `RL-POT-LAYER-001`, `RL-POT-ELIG-001`, `RL-POT-RETURN-001` map to the new tests.
3. `cargo test -p river_ledger pot::` — the targeted pure-module tests pass in isolation.

### Invariants

1. No final pot has only one contributor (such a top layer is returned).
2. Output ordering is independent of map/container iteration order.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/property.rs` — contribution-permutation invariance and conservation properties.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p rule-coverage -- --game river_ledger`
3. `cargo test -p river_ledger pot::` — the pure constructor is the correct narrow verification boundary; no betting/showdown is exercised here.

## Outcome

Completed on 2026-06-20.

- Added pure contribution-layer structures for ordered pots and uncalled returns in `games/river_ledger/src/pot.rs`.
- Implemented sorted-cap layer construction with canonical contributor/eligible lists, folded-money retention, folded-seat eligibility exclusion, singleton top-layer returns, identical-eligibility coalescing, stable pot ids, and checked conservation assertions.
- Kept the existing single-pot allocator intact for downstream migration in GAT151RIVLED-009.
- Added pure module tests for main/side-pot construction, folded-money eligibility, singleton returns, and coalescing.
- Added generated-profile property coverage for conservation and canonical ordering.

Verification:

- `cargo fmt --all --check`
- `cargo test -p river_ledger`
- `cargo run -p rule-coverage -- --game river_ledger`
- `cargo test -p river_ledger pot::`
