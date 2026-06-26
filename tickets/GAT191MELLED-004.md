# GAT191MELLED-004: Drive settle→transition in both hosts + host parity

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api` (`src/games/meldfall.rs`), `tools/simulate` (`src/main.rs`); tests `tests/replay.rs`, `tests/visibility.rs`
**Deps**: GAT191MELLED-002, GAT191MELLED-003

## Problem

Both hosts dead-end a non-terminal settled round. The wasm bridge emits
`round_score` and stops (`crates/wasm-api/src/games/meldfall.rs:206-225` only
finishes on `settlement.terminal == Some`), and `tools/simulate` returns at
`RoundSettled` with `capped_or_no_action` true — so
`completion_rate_percent` is 0. This ticket wires both hosts to call the shared
`advance_to_next_round` (GAT191MELLED-002) on a non-terminal settle, emit the
`NextRoundDealt` effect (GAT191MELLED-003), and continue to a real terminal, and
adds the host-parity determinism + cross-round no-leak tests.

## Assumption Reassessment (2026-06-26)

1. The wasm apply path's settle block (`meldfall.rs:206-225`): on
   `phase == RoundSettled` it calls `settle_round`, emits `RoundScore`, and on
   `settlement.terminal == Some` flips to `MatchComplete`. The `terminal == None`
   branch does nothing — the dead-end. This is the wiring site: after settle, when
   non-terminal, call `advance_to_next_round`, push `NextRoundDealt`, and produce
   the new active seat's action tree (the existing `legal_action_tree_for_seat`
   path used at meldfall.rs:200).
2. `tools/simulate/src/main.rs` (~837) has an **independent** apply loop that
   returns at `RoundSettled | MatchComplete` with
   `capped_or_no_action: state.terminal.is_none()`; it does not route through the
   wasm bridge. It must call `advance_to_next_round` on a non-terminal settle and
   continue until terminal. `completion_rate_percent` (main.rs:808) and
   `bounded_nonterminal_at_cap` (main.rs:806) are the summary fields that then
   reflect real completion.
3. Cross-artifact boundary under audit: host parity. Both hosts must call the same
   game-crate `advance_to_next_round`; the shared op is the structural parity
   guarantor (not coincidental per-host re-implementation). The contract under
   audit is "identical deterministic state/score/hash sequence for a seed across
   multiple rounds" (spec exit criterion 4).
4. FOUNDATIONS §2 + §11 restated: behavior authority stays in Rust in both hosts;
   the transition is identical because both delegate to the one op. Determinism:
   same seed ⇒ identical multi-round sequence; no wall-clock or browser RNG.
5. §11 no-leak firewall: the re-deal must not leak stock order, opponent hands, or
   private draws through state, the `NextRoundDealt` effect, DOM/logs, or replay
   export. The existing no-leak inventory (`tests/replay.rs:86-95`,
   `tests/visibility.rs`) is extended to a multi-round trace; `replay-check`'s
   declarative meldfall validator (`tools/replay-check/src/main.rs:717`) stays the
   trace guard (it does not compare cross-host hashes — that is this ticket's
   determinism test).

## Architecture Check

1. Routing both hosts through one shared `advance_to_next_round` is the only design
   that makes host parity structurally true rather than a coincidence of two
   re-implementations kept in sync by hand.
2. No backwards-compatibility shim: simulate's early-return-at-`RoundSettled` is
   replaced by a continue-until-terminal loop; the wasm dead-end branch is filled,
   not aliased.
3. `engine-core` is untouched; both hosts call the existing game-crate op; no
   `game-stdlib` change.

## Verification Layers

1. wasm apply drives settle→transition→terminal -> in-crate integration/effect test asserting `NextRoundDealt` is emitted and the new active seat gets an action tree.
2. simulate completes ordinary matches -> `simulate --games 1000` shows `completion_rate_percent > 0` and populated `wins_by_seat`.
3. Host parity / determinism -> same-seed full-match determinism test (`tests/replay.rs`) comparing `stable_internal_summary` sequences across the whole match.
4. No-leak across the transition -> multi-round no-leak test (`tests/visibility.rs`) proving stock order / opponent hands / private draws stay hidden through the re-deal and viewer export.

## What to Change

### 1. wasm apply-path wiring (`meldfall.rs`)

In the settle block, when `settlement.terminal` is `None`, call
`advance_to_next_round` in the same apply transaction, push the `NextRoundDealt`
effect, and produce the new active seat's action tree so play continues without a
player "advance" action.

### 2. simulate loop (`main.rs`)

On a non-terminal settle, call `advance_to_next_round` and continue the loop until
a terminal is reached; surface real completion and `wins_by_seat`.

### 3. Host-parity / determinism test (`tests/replay.rs`)

Run a full multi-round match from a fixed seed twice and assert identical
state/score/`stable_internal_summary` sequences across the whole match.

### 4. Cross-round no-leak test (`tests/visibility.rs`)

Assert stock order and opponent hands stay hidden across the re-deal and that the
viewer-scoped export stays deterministic and non-leaking across rounds.

## Files to Touch

- `crates/wasm-api/src/games/meldfall.rs` (modify; shares this file with
  GAT191MELLED-003, serialized via Deps)
- `tools/simulate/src/main.rs` (modify)
- `games/meldfall_ledger/tests/replay.rs` (modify)
- `games/meldfall_ledger/tests/visibility.rs` (modify)

## Out of Scope

- The transition op and effect definitions themselves — GAT191MELLED-002 / -003.
- The web `describeEffect` case and browser verification — GAT191MELLED-005.
- Authoring the `round-transition-resets-table-state` golden trace and docs/closeout — GAT191MELLED-006.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger` — the multi-round determinism and no-leak tests pass.
2. `cargo run -p simulate -- --game meldfall_ledger --games 1000` — `completion_rate_percent > 0`, `wins_by_seat` populated.
3. `cargo run -p replay-check -- --game meldfall_ledger --all` — green.

### Invariants

1. Both hosts drive a non-terminal settle through `advance_to_next_round` to a real
   terminal, producing identical deterministic state/score/hash sequences for a seed.
2. No hidden information (stock order, opponent hands, private draws) leaks across
   the transition in state, effects, DOM, logs, or replay export.

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/replay.rs` — same-seed multi-round determinism
   (host-parity) test over a full match.
2. `games/meldfall_ledger/tests/visibility.rs` — cross-round no-leak test (stock
   order / opponent hands / private draws) across the re-deal.

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo run -p simulate -- --game meldfall_ledger --games 1000`
3. `cargo run -p replay-check -- --game meldfall_ledger --all`
