# GAT10POKLITBET-008: Property tests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/poker_lite/tests/property.rs` (test-only; no production logic). No kernel change.
**Deps**: GAT10POKLITBET-007

## Problem

Per-example unit tests prove specific cases; property tests prove the invariants hold across the whole reachable state space. `poker_lite` needs property coverage for deterministic replay, the action-cap terminal bound, accounting invariants, legal-action-tree safety, and the no-leak invariant over many random legal playouts.

## Assumption Reassessment (2026-06-08)

1. The property-test pattern matches `games/secret_draft/tests/property.rs` (proptest-style or seeded-loop invariants over random legal command streams). This ticket consumes the action tree (GAT10POKLITBET-004), rules engine (005), effects (006), and projection (007) without adding production logic.
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §7 "Property tests") fixes the invariant set: deterministic replay from seed + command stream; action-cap terminal bound; contributions never negative; shared pool equals contribution sum; legal action tree never offers an illegal lift after cap; no hidden id in public-facing projections before reveal.
3. Cross-artifact boundary under audit: this test exercises the full crate-local pipeline (`actions`→`rules`→`effects`→`visibility`). It is the property-level proof surface that the per-example tests in 004/005/007 complement; it does not own any production contract.
4. FOUNDATIONS §11 (determinism; no-leak; bounded termination) motivates this ticket. Restated: identical inputs+version → identical output across the state space, and no random playout leaks a hidden id.
5. Determinism + no-leak invariant surface under audit (§11): the replay-determinism property here is the broad complement to the GAT10POKLITBET-009 golden-trace hash check; the no-hidden-id property is the broad complement to the GAT10POKLITBET-007 string-search tests. Confirm the action-cap (spec uses 16 in §7 acceptance evidence) guarantees termination so the property loop cannot diverge.

## Architecture Check

1. Property tests over random legal streams catch invariant violations that hand-picked examples miss (e.g. an accounting overrun reachable only via a specific lift/match order), at low cost since the game is tiny and bounded. Matches the sibling property-test approach.
2. No backwards-compatibility aliasing/shims — new test file.
3. `engine-core` untouched (§3); no `game-stdlib` promotion (§4); test-only.

## Verification Layers

1. Deterministic replay (random seed+stream replays identically) -> `cargo test -p poker_lite --test property` determinism property.
2. Bounded termination (every playout terminates within the action cap) -> action-cap property.
3. Accounting invariants (contributions ≥ 0; pool == Σ contributions; no overrun) -> accounting property.
4. Legal-tree safety + no-leak (no illegal lift after cap; no hidden id in pre-reveal public projection) -> legality + no-leak properties.

## What to Change

### 1. `games/poker_lite/tests/property.rs`

Implement the six invariants from §7 as properties over random legal command streams seeded deterministically: replay determinism, action-cap terminal bound, `contributions[i] >= 0`, `shared_pool == sum(contributions)`, legal tree never offers a post-cap lift, and no hidden card id/rank in the observer projection before the rule-defined reveal point.

## Files to Touch

- `games/poker_lite/tests/property.rs` (new)

## Out of Scope

- Golden traces and replay-hash checkpoints (GAT10POKLITBET-009).
- Per-example rule/visibility unit tests (already in GAT10POKLITBET-005/007).
- Bot-specific properties (GAT10POKLITBET-010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite --test property` — all six invariants hold across the seeded random playouts.
2. `cargo test -p poker_lite` passes overall.

### Invariants

1. Identical `(seed, command stream, version)` replays to an identical terminal across the state space (§11 determinism).
2. No public-facing projection exposes a hidden card id before reveal in any sampled playout (§11 no-leak).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/property.rs` — replay determinism, action-cap bound, accounting, legal-tree safety, no-leak.

### Commands

1. `cargo test -p poker_lite --test property`
2. `cargo test -p poker_lite`
3. Narrower boundary: property tests are crate-local; the workspace-wide run is exercised by the capstone (GAT10POKLITBET-018), not here.
