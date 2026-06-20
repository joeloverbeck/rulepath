# GAT151RIVLED-007: All-in actor rotation and runout

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger` (`betting.rs`, `rules.rs`), tests
**Deps**: GAT151RIVLED-006

## Problem

Folded and all-in seats must be excluded from actor selection; a street closes when every action-capable non-folded seat has responded with nothing further owed. When two or more non-folded seats remain but none can act (all all-in), the board runs out deterministically straight to showdown. When one non-all-in seat remains owing nothing alongside all-in seats, unmatched excess is returned before runout — no meaningless check loop. Sole-live foldout settles without revealing hidden cards.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/src/betting.rs` selects actors and closes streets for live seats; `rules.rs` advances the board and handles foldout. Neither excludes a typed all-in seat from rotation today (the `AllIn` status landed in GAT151RIVLED-004), and there is no all-all-in auto-runout path.
2. Docs: spec §3.3(4) — exclude folded/all-in from actor selection; close a street when no response is owed; deal the remaining board deterministically and advance to showdown when ≥2 non-folded seats remain and none can act; return unmatched excess before runout when one non-all-in seat owes nothing; preserve no-reveal foldout when only one non-folded seat remains.
3. Cross-artifact boundary under audit: actor-rotation state ↔ reopen/response state (GAT151RIVLED-006) ↔ the uncalled-return extraction consumed by the side-pot constructor (GAT151RIVLED-008) and settlement (GAT151RIVLED-009).
4. (§11 no-leak firewall) Auto-runout and foldout must not reveal cards the viewer is not entitled to: confirm the runout deals board cards deterministically without exposing folded/unrevealed hole cards, and foldout settles with no showdown reveal. This is the behavior the no-leak matrix (GAT151RIVLED-016) and traces (GAT151RIVLED-017) later prove.

## Architecture Check

1. Centralizing "who can act" on the typed status (skip `Folded`/`AllIn`) avoids no-op action loops and keeps street closure deterministic.
2. No backwards-compatibility shims; foldout behavior is preserved, extended only for the unmatched-excess-return case.
3. Rotation/runout logic stays game-local; `engine-core` is untouched (§3).

## Verification Layers

1. Folded/all-in seats never appear as the active actor -> 3–6-seat actor-order tests.
2. All-all-in triggers deterministic board runout to showdown -> runout golden-trace fixtures (authored fully in GAT151RIVLED-017).
3. One-live-owing-nothing returns excess before runout, no check loop -> rule tests.
4. Sole-live foldout settles with no hidden-card reveal -> no-leak unit test.

## What to Change

### 1. All-in-aware actor rotation and closure

Exclude folded/all-in seats from actor selection; close a street when every action-capable non-folded seat has responded with nothing owed; a zero-stack all-in seat never receives another turn.

### 2. Automatic runout and excess return

When ≥2 non-folded seats remain and none can act, deal the remaining board deterministically and advance to showdown; when one non-all-in seat owes nothing, return unmatched excess first; preserve sole-live no-reveal foldout.

## Files to Touch

- `games/river_ledger/src/betting.rs` (modify)
- `games/river_ledger/src/rules.rs` (modify)
- `games/river_ledger/tests/rules.rs` (modify)

## Out of Scope

- Side-pot construction from contributions (GAT151RIVLED-008).
- Per-pot evaluation and settlement (GAT151RIVLED-009).
- Semantic effects/projections for runout (GAT151RIVLED-010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — 3–6-seat actor-order, all-all-in runout, one-live excess-return, sole-live foldout, and no infinite action loops.
2. `cargo run -p simulate -- --game river_ledger --games 1000` — every hand terminates deterministically across seat counts and stack profiles.
3. `cargo run -p replay-check -- --game river_ledger --all` — existing replays still pass (new behavior is reachable only with stacks; full trace reconciliation is deferred to GAT151RIVLED-017).

### Invariants

1. No all-in seat remains in the set of actors to respond or receives a no-op turn.
2. Automatic runout and foldout reveal no card the viewer is not otherwise entitled to see.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` — actor-rotation, auto-runout, excess-return, and foldout cases.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p simulate -- --game river_ledger --games 1000`
3. `cargo run -p replay-check -- --game river_ledger --all` — narrower than golden-trace authoring (GAT151RIVLED-017); it confirms existing replays are not broken by rotation changes.
