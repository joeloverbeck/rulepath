# GAT151RIVLED-006: Reopen rights and cap state

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger` (`betting.rs`, `rules.rs`), tests
**Deps**: GAT151RIVLED-005

## Problem

River Ledger uses an explicit full-unit reopening rule: a full fixed bet/raise reopens raising and consumes a raise-cap slot, while a single incomplete all-in increase below the street unit does not reopen for a seat that already acted. Cumulative incomplete increases reopen only when they reach a full street unit since the seat's last completed action. This per-seat response/reopen accounting must be explicit deterministic Rust state, never inferred in TypeScript.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/src/betting.rs` tracks the existing raise cap and actor rotation but has no per-seat reopen/cumulative-increase state; `rules.rs` enforces the cap. The action metadata producer (`raise_right_open`, `raises_remaining`) landed in GAT151RIVLED-005 and is read here.
2. Docs: spec §3.3(3) — full raise increases the wager by the street unit and consumes one cap slot; single incomplete all-in increase below the unit does not reopen for an already-acted seat; a not-yet-acted seat keeps its ordinary raise right; cumulative incomplete increases reopen only at ≥ one full street unit since last completed action; at cap, neither full raise nor short raise-all-in is legal.
3. Cross-artifact boundary under audit: the betting-state ↔ action-metadata contract — reopen state feeds `raise_right_open`/`raises_remaining` emitted by `actions.rs` (GAT151RIVLED-005) and the actor-rotation in GAT151RIVLED-007.
4. (§2 behavior authority) Restate: per-seat raise-right/reopen state is explicit deterministic Rust state and MUST NOT be inferred in TypeScript. Confirm the implementation exposes it only as projected fields, with no derivation path left to the client.

## Architecture Check

1. Explicit per-seat reopen accounting separates "must respond" from "may raise" cleanly, versus re-deriving reopening from contribution deltas at every preview (fragile and easy to leak into TS).
2. No backwards-compatibility shims; the existing raise cap is preserved, not reimplemented.
3. Reopen/cap nouns stay game-local; `engine-core` is untouched (§3).

## Verification Layers

1. One short all-in increase does not reopen for an already-acted seat -> state-machine unit test.
2. Cumulative short increases reaching a full unit do reopen -> property/state test.
3. Full raises consume a cap slot; at cap no raise (full or short-all-in) is legal -> rule tests.
4. Response obligation persists independent of raise right -> focused unit tests.

## What to Change

### 1. Per-seat reopen + cumulative-increase state

Add explicit per-seat response and reopen accounting in `betting.rs`: track cumulative incomplete increase faced since each seat's last completed action; reopen a seat's raise right only at ≥ one full street unit.

### 2. Cap interaction

Keep the existing raise cap; a full raise consumes a slot; once the cap is reached, neither a full raise nor a short raise-all-in is legal — a seat may call, call all-in, check, or fold as applicable.

## Files to Touch

- `games/river_ledger/src/betting.rs` (modify)
- `games/river_ledger/src/rules.rs` (modify)
- `games/river_ledger/tests/rules.rs` (modify)

## Out of Scope

- Actor rotation, street closure, and automatic runout (GAT151RIVLED-007).
- Side-pot construction and allocation (GAT151RIVLED-008, -009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — one short increase does not reopen; cumulative full-unit pressure does; full raises consume the cap; cap prevents any further raise.
2. `cargo run -p simulate -- --game river_ledger --games 1000` — no illegal reopens across 3–6 seats.
3. `cargo run -p rule-coverage -- --game river_ledger` — `RL-ALLIN-REOPEN-001` maps to the new tests.

### Invariants

1. Per-seat raise-right/reopen state is explicit Rust state; nothing requires client-side derivation.
2. A seat owing additional contribution must still respond (fold/call/raise-if-open), independent of whether its raise right reopened.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` — reopen state-machine and cumulative-increase property cases.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p simulate -- --game river_ledger --games 1000`
3. `cargo run -p rule-coverage -- --game river_ledger` — coverage of the reopen family is the correct verification boundary.
