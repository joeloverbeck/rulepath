# GAT151RIVLED-004: Stack ledger and forced posts

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger` (`state.rs`, `setup.rs`), tests
**Deps**: GAT151RIVLED-003

## Problem

With configurable stacks in place, the state model must track each seat's starting and remaining stack, distinguish a typed all-in status, hold checked conservation invariants, and cap blind posting by the posting seat's remaining stack (a short blind leaves that seat all-in). This is the accounting backbone every later betting and pot ticket reads; it adds no legal-action changes.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/src/state.rs` defines `SeatStatus { Live, Folded, ShowdownEligible }` and `SeatLedger { seat, status, street_contribution: u16, total_contribution: u16 }` plus `ContributionLedger { seats, pot_total }`. There is no `remaining_stack`, starting-stack, or all-in field today; blinds post from hardcoded constants.
2. Docs: spec §3.3(1) requires per-seat starting/remaining stack, street/total contribution, a typed all-in status distinct from live/folded/showdown, checked conservation before/during/after settlement, and blind posts capped by remaining stack.
3. Cross-artifact boundary under audit: the `SeatLedger` / `ContributionLedger` shape consumed downstream by `actions.rs`, `betting.rs`, `rules.rs`, `pot.rs`, `showdown.rs`, `visibility.rs` — this ticket extends it additively before those tickets read the new fields.
4. (§11 deterministic substrate) Conservation is the enforcement surface: `sum(remaining stacks) + sum(total contributions) == sum(starting stacks)` must hold under checked arithmetic at every step. Confirm the new fields introduce no nondeterminism and no negative/overflow path; this substrate feeds the settlement invariants in GAT151RIVLED-009 and the hash state in GAT151RIVLED-011.

## Architecture Check

1. Extending the existing `SeatLedger` with stack/all-in fields keeps one authoritative per-seat accounting record rather than a parallel stack table that could desynchronize.
2. No backwards-compatibility shims; the all-in status is a new typed variant, not an overloaded `Live`.
3. Stack/all-in nouns stay game-local in `games/river_ledger`; `engine-core` is untouched (§3).

## Verification Layers

1. Conservation holds before/during/after posting -> property tests over bounded contribution vectors.
2. Short small/big blind leaves the posting seat all-in for its exact stack -> unit tests.
3. Exact-exhaustion and no-underflow/overflow -> checked-arithmetic unit tests.
4. Deterministic state summary -> serialization spot-check (stable field order) consumed later by GAT151RIVLED-011.

## What to Change

### 1. Stack-aware seat state

Add starting and remaining stack to the per-seat ledger; add a typed `AllIn` status distinct from `Live`/`Folded`/`ShowdownEligible`; preserve existing contribution semantics and button/blind order.

### 2. Capped forced posts + conservation

Cap each blind post by the posting seat's remaining stack, creating a deterministic short-blind all-in when the stack is below the blind; assert the checked conservation invariant after posting.

## Files to Touch

- `games/river_ledger/src/state.rs` (modify)
- `games/river_ledger/src/setup.rs` (modify)
- `games/river_ledger/tests/property.rs` (modify)
- `games/river_ledger/tests/rules.rs` (modify)

## Out of Scope

- Stack-capped legal actions and all-in metadata (GAT151RIVLED-005).
- Reopen-rights / cap-state accounting (GAT151RIVLED-006).
- Side-pot construction (GAT151RIVLED-008).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — conservation, short small/big blind, exact blind exhaustion, no underflow/overflow.
2. `cargo run -p simulate -- --game river_ledger --games 1000` — no panics across 3–6 seats with asymmetric stacks.
3. `cargo run -p fixture-check -- --game river_ledger` — fixtures still load against the extended state.

### Invariants

1. `sum(remaining stacks) + sum(total contributions) == sum(starting stacks)` holds at every step under checked arithmetic.
2. No remaining stack is negative; an all-in seat's remaining stack is exactly zero.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/property.rs` — conservation property over bounded stack/contribution vectors.
2. `games/river_ledger/tests/rules.rs` — short/exact/full blind posting and all-in-on-post cases.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p simulate -- --game river_ledger --games 1000`
3. `cargo run -p fixture-check -- --game river_ledger` — narrower than the web/WASM gates, which this ticket does not touch.

## Outcome

Completed: 2026-06-20

What changed:

- Added the typed `SeatStatus::AllIn` state and public UI label.
- Changed forced-post setup to cap small/big blind contributions by the posting seat's starting stack, leaving exact-exhausted or short-posting seats with `remaining_stack = 0` and `AllIn` status.
- Derived setup `pot_total` and preflop `current_to_call` from the actual capped forced posts.
- Kept all stack/all-in behavior game-local in `games/river_ledger`; no `engine-core` or `game-stdlib` change.
- Added setup-level stack conservation assertions and rule tests for short small blind, short big blind, exact blind exhaustion, and no-underflow behavior.

Deviations:

- Full action-level stack decrement/capping remains out of scope for this ticket and is still owned by GAT151RIVLED-005. Conservation assertions added here are setup/forced-post focused so they do not preempt the next legal-action ticket.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p river_ledger` passed.
- `cargo run -p simulate -- --game river_ledger --games 1000` passed (`games_run=1000`).
- `cargo run -p fixture-check -- --game river_ledger` passed (`fixture-check: all fixtures passed`).
