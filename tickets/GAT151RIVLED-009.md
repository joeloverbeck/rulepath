# GAT151RIVLED-009: Per-pot allocation and settlement

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/river_ledger` (`pot.rs`, `showdown.rs`), tests
**Deps**: GAT151RIVLED-007, GAT151RIVLED-008

## Problem

Join betting termination to the layered pots: evaluate winners independently within each pot's eligible seats, split each pot by integer units, award each pot's remainder independently in the existing button order, and aggregate per-seat awards and final stacks. Side pots are never recombined for allocation. A one-eligible-seat pot is awarded without exposing other private hands. All settlement invariants from spec §3.3(6) must hold under checked arithmetic.

## Assumption Reassessment (2026-06-20)

1. Code: the layered constructor and resolved-pot/return structures landed in GAT151RIVLED-008 (`pot.rs`); `showdown.rs` currently evaluates a single pot via the seven-card evaluator and `winners_in_button_order(...)`. This ticket adds per-pot evaluation and the aggregate result.
2. Docs: spec §3.3(6) — per-pot independent evaluation/split/remainder; no recombination; sole-eligible award without reveal; aggregate awards/final stacks from ordered per-pot results + returns; the full invariant list (no contribution exceeds starting stack, no negative remaining stack, conservation before/after, per-pot `sum(shares)==amount`, every winner eligible, folded seats not eligible, `sum(final)==sum(starting)`, no singleton-contributor final pot).
3. Cross-artifact boundary under audit: the single authoritative resolved result consumed by effects/views (GAT151RIVLED-010), serialization/hash (GAT151RIVLED-011), and bots (GAT151RIVLED-012) — there must be exactly one terminal-result assembly, not competing ones.
4. (§2 behavior authority) Restate: winner selection, allocation, remainder ordering, and final-stack derivation are computed in Rust; no UI/WASM-side arithmetic re-derives them. The existing `winners_in_button_order` helper is reused per the GAT151RIVLED-002 ledger decision.
5. (§11 no-leak firewall) A pot with one eligible seat is awarded without comparing or exposing other seats' private hands; confirm settlement reveals only authorized showdown facts. This is later proven by GAT151RIVLED-016/-017.

## Architecture Check

1. Evaluating each pot against its own eligibility set, with one aggregate assembly, prevents the multiple-competing-terminal-result bug spec §9 forbids.
2. No backwards-compatibility shims; the single-pot split becomes the one-layer case of the general allocator.
3. Allocation/showdown vocabulary stays game-local; reuses the existing button-order helper rather than promoting it (per GAT151RIVLED-002).

## Verification Layers

1. Different winners across pots; ties; sole eligibility; odd units -> rule/property tests per pot.
2. Conservation `sum(final stacks)==sum(starting stacks)` and per-pot `sum(shares)==amount` -> property tests under checked arithmetic.
3. Remainder awarded independently per pot in button order -> unit tests (no single global remainder).
4. Sole-eligible award exposes no other private hand -> no-leak unit test.

## What to Change

### 1. Per-pot evaluation and allocation

In `showdown.rs`/`pot.rs`, evaluate each resolved pot's eligible seats independently, split by integer units, and award each remainder independently using `winners_in_button_order`; never recombine side pots.

### 2. Aggregate terminal result

Produce one authoritative resolved result: aggregate per-seat awards and final stacks derived from the ordered per-pot awards plus uncalled returns, asserting all §3.3(6) invariants.

## Files to Touch

- `games/river_ledger/src/pot.rs` (modify)
- `games/river_ledger/src/showdown.rs` (modify)
- `games/river_ledger/tests/rules.rs` (modify)
- `games/river_ledger/tests/property.rs` (modify)

## Out of Scope

- Semantic effects, public projections, and explanations of the result (GAT151RIVLED-010).
- Serialization/hash of the result (GAT151RIVLED-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — different winners across pots, ties across pots, sole eligibility, odd units, uncalled excess, and total conservation.
2. `cargo run -p simulate -- --game river_ledger --games 1000` — every terminal settlement conserves the initial total across seat counts.
3. `cargo run -p rule-coverage -- --game river_ledger` — `RL-POT-ALLOC-001`, `RL-POT-REMAINDER-001` map to the new tests.

### Invariants

1. Every winner is eligible for the pot it wins; folded seats can contribute but never win.
2. `sum(final stacks) == sum(starting stacks)`; side pots are never recombined before splitting.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` — per-pot winner/tie/sole-eligibility/odd-unit cases.
2. `games/river_ledger/tests/property.rs` — settlement conservation and per-pot share-sum properties.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p simulate -- --game river_ledger --games 1000`
3. `cargo run -p rule-coverage -- --game river_ledger` — allocation/remainder coverage is the correct boundary; effects/serialization are verified in later tickets.
