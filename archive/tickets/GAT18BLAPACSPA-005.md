# GAT18BLAPACSPA-005: public sequential bidding and team contract aggregation

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/blackglass_pact` (bidding/state/effects/visibility) + golden traces
**Deps**: GAT18BLAPACSPA-004

## Problem

Implement the `Bidding` phase: after the deal, each non-blind seat submits one immutable public bid (`bid/nil` or `bid/1`…`bid/13`) clockwise left of dealer through dealer; blind-nil declarers are skipped with a fixed `Bid::BlindNil`; the ordinary team contract is the sum of the two partners' positive numeric bids, with nil/blind-nil contributing zero and evaluated separately. No total-13/last-bidder hook (spec §3.1 bid rows, §3.2, `BP-BID-*`, candidate task `GAT18-BLAPAC-005`).

## Assumption Reassessment (2026-06-25)

1. The `Phase::Bidding { next, accepted }` stub + `Bid` enum from GAT18BLAPACSPA-003 (Appendix B.2) are implemented here; the blind-skip relies on accepted `Bid::BlindNil` set in GAT18BLAPACSPA-004.
2. Spec §3.1 pins the bid vocabulary and the explicit exclusion of Vow Tide's total-13 dealer hook; spec §3.3 defines `C` as the sum of positive numeric partner bids.
3. Cross-artifact boundary under audit: the action-tree/validator equivalence contract (every emitted leaf validates; every accepted action was emitted) and the public bid projection consumed by views/replay.
4. FOUNDATIONS §2 (Rust owns legal-action generation/validation) motivates this ticket: bid legality and contract aggregation are Rust-authored; TypeScript later only renders the Rust-derived contract field.

## Architecture Check

1. Deriving the ordinary contract from accepted bids in Rust (vs. letting the client sum partner bids) keeps the single source of truth in the game crate and is the §2-clean design the UI ticket depends on.
2. No shims; no Vow Tide hook imported.
3. `engine-core` untouched; bidding/aggregation is game-local; no `game-stdlib` change.

## Verification Layers

1. Bid order/skip and 14-leaf maximum (`nil`+1..13) -> rules/tree unit tests + `bidding-left-of-dealer-through-dealer` and `blind-nil-seat-skipped-in-bidding` traces.
2. Action-tree/validator equivalence; numeric-zero/out-of-range rejected -> equivalence test + invalid-bid diagnostic trace.
3. `C` = sum of positive numeric partner bids; nil/blind contribute zero -> contract property test + `team-contract-sums-only-positive-numeric-bids` trace.

## What to Change

### 1. Bidding phase + leaves

`bidding.rs`: emit `bid/nil`+`bid/1..13` for the active non-blind seat, skip blind declarers, accept exactly one immutable bid per seat, advance clockwise ending with dealer; stable diagnostics (`BP_BID_OUT_OF_RANGE`, `BP_BID_LOCKED`).

### 2. Contract derivation + projection

`state.rs`/`visibility.rs`: derive ordinary team contract from accepted bids; project accepted bids + Rust-derived contract publicly (stable seat/team order).

### 3. Effects + traces

`effects.rs`: `BidAccepted` (public, seat/team/bid). Add bidding golden traces (spec §7.6 #15–#24).

## Files to Touch

- `games/blackglass_pact/src/{bidding,state,visibility,effects}.rs` (modify)
- `games/blackglass_pact/tests/{rules,property}.rs` (modify)
- `games/blackglass_pact/tests/golden_traces/*.trace.json` (new — bidding scenarios)

## Out of Scope

- Trick play / scoring (GAT18BLAPACSPA-006/007).
- Cross-viewer no-leak harness (GAT18BLAPACSPA-008).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p blackglass_pact --test rules` (order, skip, immutability, invalid-bid diagnostics).
2. `cargo test -p blackglass_pact --test property` (every emitted leaf validates; contract aggregation).
3. Regression: no total-13 / last-bidder hook exists (contrast-with-Vow-Tide test).

### Invariants

1. Each non-blind seat bids exactly once; accepted bids are immutable and public.
2. The ordinary contract equals the sum of the two partners' positive numeric bids; nil/blind contribute zero.

## Test Plan

### New/Modified Tests

1. `games/blackglass_pact/tests/rules.rs` — bid order/vocabulary/skip + `BP_BID_*` diagnostics.
2. `games/blackglass_pact/tests/property.rs` — leaf/validator equivalence + contract aggregation.
3. `games/blackglass_pact/tests/golden_traces/team-contract-sums-only-positive-numeric-bids.trace.json` — contract evidence.

### Commands

1. `cargo test -p blackglass_pact --test rules --test property`
2. `cargo test -p blackglass_pact`
3. Crate-scoped tests are the boundary; trace validation runs at `replay-check` registration (GAT18BLAPACSPA-011).

## Outcome

Completed: 2026-06-25

Implemented public sequential bidding and ordinary contract aggregation:

- Added `bid/nil` and `bid/1` through `bid/13` legal leaves for the active non-blind bidder.
- Added bid action parsing, stable diagnostics for out-of-range bids and immutable accepted bids, and public `BidAccepted` effects.
- Advanced bidding clockwise from left of dealer through dealer, skipping blind-nil declarers fixed as `Bid::BlindNil`.
- Derived ordinary team contracts in Rust from positive numeric partner bids only; nil and blind nil contribute zero.
- Added stable public bidding projection rows and team contract rows for later viewer/WASM work.
- Added bidding golden trace JSON evidence for order, blind skip, invalid diagnostics, and contract aggregation.

Deviations from plan: after the final bid, the state transitions to the first `PlayingTrick` placeholder with leader/next left of dealer. Actual play legality remains deferred to GAT18BLAPACSPA-006.

Verification:

- `cargo test -p blackglass_pact --test rules --test property` passed (9 rules tests, 9 property tests).
- `cargo test -p blackglass_pact` passed (1 lib test, 9 property tests, 9 rules tests, 2 serialization tests, 1 visibility test).
- `cargo fmt --all --check` passed.
- `git diff --check` passed.
