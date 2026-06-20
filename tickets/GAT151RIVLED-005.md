# GAT151RIVLED-005: Stack-capped legal actions

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/river_ledger` (`actions.rs`, `rules.rs`), tests
**Deps**: GAT151RIVLED-004

## Problem

Legal actions must become stack-aware while preserving the five action families. All-in is a consequence of `Call`, `Bet`, or `Raise` — never a new arbitrary-sizing family. Action metadata must expose the exact amount, stack-after, all-in/full-raise flags, raise-cap consumption, and whether the actor's raise right is open, so every downstream surface (previews, bots, UI, WASM) reads authoritative Rust values. Street-closure changes are out of scope here.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/src/actions.rs` emits the `Fold`/`Check`/`Call`/`Bet`/`Raise` action paths; `rules.rs` applies them against the contribution ledger. Neither is stack-aware today, and there is no all-in/full-raise metadata on the action tree.
2. Docs: spec §3.3(2) enumerates the stack-to-call relationships — `amount_owed > stack_remaining` → `Fold` + all-in `Call`; `== stack_remaining` → full call leaves all-in; cover-call-but-not-`call+street_unit` → short raise-all-in legal only when raising is otherwise legal and the cap is open; no-wager-faced with `0 < stack < street_unit` → short opening bet-all-in; full bet/raise consuming exact stack → all-in full bet/raise. Required metadata: `amount_owed`, `adds_to_pot`, `stack_before`, `stack_after`, `is_all_in`, `is_full_raise`, `raise_right_open`, `raises_remaining`.
3. Cross-artifact boundary under audit: the action-tree / action-path schema (`docs/ARCHITECTURE.md`) — the new metadata fields are additive and read by previews, bots, WASM, and UI; this ticket is their producer.
4. (§2 behavior authority) The legal-action set and all-in detection are computed in Rust; TypeScript must never derive all-in, sizing, or cap state. Restate: all legality and sizing stay in `games/river_ledger`; the new flags are projected, not inferred.
5. (schema extension) The action-tree gains `is_all_in`/`is_full_raise`/`raise_right_open`/`raises_remaining` fields — additive-only; consumers (preview rendering in GAT151RIVLED-010/014, bots in -012, WASM in -013) gain new arms, none break on the additive fields.

## Architecture Check

1. Modeling all-in as a qualification of the existing families keeps the action tree honest (no sixth arbitrary-sizing family that would invite TS sizing) and matches the fixed-limit structure.
2. No backwards-compatibility shims; stack-capping replaces the unbounded amount computation in place.
3. Action/all-in vocabulary stays game-local; `engine-core`'s action-path contract is unchanged and noun-free (§3).

## Verification Layers

1. Legal set per stack-to-call relationship -> action-tree rule tests (short call, exact all-in call, short opening bet, short raise, full all-in raise, cap closure).
2. Metadata correctness -> preview/metadata unit tests asserting `is_all_in`/`is_full_raise`/`stack_after`/`raises_remaining`.
3. Malformed/stale/wrong-seat input -> deterministic diagnostic tests (§11 fail-closed).
4. Schema conformance -> action-tree validation against `docs/ARCHITECTURE.md` (additive fields, stable order).

## What to Change

### 1. Stack-capped action amounts

Make `Call`/`Bet`/`Raise` amounts stack-aware in `actions.rs`/`rules.rs` while preserving the action-path segments; a call may be less than the amount owed when the seat is all-in.

### 2. Authoritative action metadata

Emit `amount_owed`, `adds_to_pot`, `stack_before`, `stack_after`, `is_all_in`, `is_full_raise`, `raise_right_open`, `raises_remaining`, and accessible presentation hints on each legal action.

## Files to Touch

- `games/river_ledger/src/actions.rs` (modify)
- `games/river_ledger/src/rules.rs` (modify)
- `games/river_ledger/tests/rules.rs` (modify)

## Out of Scope

- Per-seat reopen-rights / cumulative-increase accounting and street closure (GAT151RIVLED-006, -007).
- Side-pot construction and settlement (GAT151RIVLED-008, -009).
- WASM/UI projection of the new metadata (GAT151RIVLED-013, -014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — rule/action-tree/preview tests for short call, exact all-in call, short opening bet, short raise, full all-in raise, cap closure, and malformed/stale/wrong-seat diagnostics.
2. `cargo run -p rule-coverage -- --game river_ledger` — new `RL-ALLIN-CALL/BET/CAP` rows map to these tests (coverage may be partial until GAT151RIVLED-019 reconciles docs).
3. `cargo run -p simulate -- --game river_ledger --games 1000` — only legal actions are produced across 3–6 seats.

### Invariants

1. Exactly five action families exist; all-in is always a qualified `Call`/`Bet`/`Raise`, never a new family.
2. No action lets a seat contribute more than its remaining stack.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` — the full stack-to-call legal-action matrix and metadata assertions.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p simulate -- --game river_ledger --games 1000`
3. `cargo run -p rule-coverage -- --game river_ledger` — coverage rows are the correct boundary for new rule families, even while `RULE-COVERAGE.md` reconciliation is deferred to GAT151RIVLED-019.
