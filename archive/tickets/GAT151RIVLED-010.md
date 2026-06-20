# GAT151RIVLED-010: Effects, views, and explanations

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/river_ledger` (`effects.rs`, `visibility.rs`, `ui.rs`), tests
**Deps**: GAT151RIVLED-009

## Problem

Project the one authoritative resolved result through viewer-safe public state and Rust-authored explanations. Add ordered semantic effects for stack changes, all-in transitions, uncalled returns, and pot resolution/award; project public stacks, all-in status, ordered pot tiers, eligibility, and returns while preserving private-hand/deck redaction; and author neutral live and terminal per-pot explanation rows in Rust. TypeScript renders only the projection.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/src/effects.rs`, `visibility.rs`, and `ui.rs` already emit showdown effects, viewer-scoped projections, and Rust-authored labels for the base game; this ticket extends them with stack/all-in/pot/return surfaces reading the resolved result from GAT151RIVLED-009.
2. Docs: spec §3.3(7) + §4 name the required public projection shapes (`SeatStackView`, `PotTierView`, `UncalledReturnView`, `PotAllocationView`) and the effects (`StackChanged`, `SeatBecameAllIn`, `UncalledContributionReturned`, `PotResolved`, `PotAwarded`); §3.3(7) requires uncalled excess shown as a return (not a won pot) and foldout/sole-eligible awards explained without exposing cards.
3. Cross-artifact boundary under audit: the public-view/effect-envelope contract (`docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`) — these projections are the sole source the WASM bridge (GAT151RIVLED-013) and web renderer (GAT151RIVLED-014) read; no second computation may exist.
4. (§11 no-leak firewall) Restate: public stacks, contributions, all-in status, pot amounts/tiers, and eligibility are public; private cards/deck/evaluator internals stay viewer-scoped. Confirm effects are public accounting facts only and never a covert hidden-state log; the fact that a seat is eligible is public, an unrevealed hand's contents are not.
5. (effect-schema extension) Adds new effect kinds — additive to the effect envelope; consumers are the EffectLog/animation path (GAT151RIVLED-014) and no-leak matrix (GAT151RIVLED-016). Each new kind carries only viewer-safe public/authorized-reveal data.

## Architecture Check

1. One authoritative Rust-authored explanation per public allocation/return keeps the outcome surface single-sourced; renderer diffs are diagnostics only (§7).
2. No backwards-compatibility shims; new effects extend the ordered effect stream rather than overloading existing kinds.
3. Effect/view/label vocabulary stays game-local; `engine-core`'s effect-envelope and view contracts are unchanged and noun-free (§3).

## Verification Layers

1. Effect order and content -> effect-order unit tests (stack → all-in → return → pot-resolved → pot-awarded).
2. Public projection shapes carry only authorized facts -> Rust projection snapshots + no-leak unit tests.
3. Uncalled excess shown as a return, not a won pot -> outcome unit test.
4. Every public allocation/return has exactly one Rust-authored explanation -> manual review + snapshot.

## What to Change

### 1. Ordered semantic effects + projections

Add the `StackChanged`/`SeatBecameAllIn`/`UncalledContributionReturned`/`PotResolved`/`PotAwarded` effects (viewer-safe payloads) in `effects.rs`; project `SeatStackView`/`PotTierView`/`UncalledReturnView`/`PotAllocationView` in `visibility.rs` with private-hand/deck redaction across seat, cross-seat, and observer views.

### 2. Rust-authored explanations

Author neutral live and terminal per-pot explanation rows, all-in indicators, uncalled-return text, button-order remainder text, and accessibility copy in `ui.rs`; retain authorized hand-comparison teaching detail.

## Files to Touch

- `games/river_ledger/src/effects.rs` (modify)
- `games/river_ledger/src/visibility.rs` (modify)
- `games/river_ledger/src/ui.rs` (modify)
- `games/river_ledger/tests/visibility.rs` (modify)

## Out of Scope

- Serialization/hash of the new state (GAT151RIVLED-011).
- WASM marshalling (GAT151RIVLED-013) and web rendering (GAT151RIVLED-014).
- The full pairwise no-leak matrix (GAT151RIVLED-016).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — effect-order, outcome, projection-snapshot, and no-leak unit tests.
2. `cargo run -p rule-coverage -- --game river_ledger` — `RL-OUTCOME-POT-001`, `RL-VIS-POT-001` map to the new tests.
3. `cargo test -p river_ledger visibility` — seat/cross-seat/observer projections redact private cards/deck.

### Invariants

1. No effect or projection carries a card/deck/evaluator field the viewer is not authorized to see.
2. Every public pot award and uncalled return has exactly one Rust-authored explanation.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/visibility.rs` — projection-shape, effect-order, and redaction cases.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p rule-coverage -- --game river_ledger`
3. `cargo test -p river_ledger visibility` — projection/redaction is the correct narrow boundary; cross-surface no-leak is proven in GAT151RIVLED-016.

## Outcome

Completed on 2026-06-20.

- Added public accounting effects for stack changes, all-in transitions, uncalled returns, aggregate pot resolution, and aggregate pot awards.
- Emitted stack/all-in/return effects from before/after ledger diffs, ordered before pot resolution and showdown resolution.
- Added public stack fields, pot-tier projections, and uncalled-return projections to `PublicView`; stable summaries now include those public accounting fields.
- Added Rust-authored uncalled-return explanation copy in `ui.rs`.
- Added visibility coverage proving stack/pot projections and accounting effects expose public accounting facts without unrevealed deck/future-card leakage.

Verification:

- `cargo fmt --all --check`
- `cargo test -p river_ledger`
- `cargo run -p rule-coverage -- --game river_ledger`
- `cargo test -p river_ledger visibility`
