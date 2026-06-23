# UNI8CR2TWOSEA-015: Poker Lite — exact-two-seat structural validation + game-stdlib dependency

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/poker_lite/Cargo.toml`, `games/poker_lite/src/setup.rs`; adds normal `game-stdlib` dependency and adopts `SeatCount`
**Deps**: 001

## Problem

Spec §3.6 / task `8C-R2-303`: `setup_match` validates seat structure with a hand-written length predicate against `STANDARD_SEAT_COUNT`. R2 replaces only that predicate with structural `SeatCount` use, adding a normal `game-stdlib` dependency, and preserves diagnostics and state bytes.

## Assumption Reassessment (2026-06-23)

1. `games/poker_lite/src/setup.rs::setup_match` exists (confirmed ~line 22) and compares `seats.len()` against the game-local `STANDARD_SEAT_COUNT`; `poker_lite` does **not** currently depend on `game-stdlib` (confirmed — this ticket adds it).
2. Spec §3.6: `migrate`; diagnostics and state bytes byte-identical; `next_ring_index` is `not-applicable`.
3. Cross-crate boundary under audit: `game-stdlib::SeatCount` (`crates/game-stdlib/src/seat.rs`) — structural count helper; the `STANDARD_SEAT_COUNT` expectation stays game-local.
4. Determinism: diagnostics and state hashes must stay byte-identical to the `-001` baseline; structural validation introduces no nondeterministic input (§11).

## Architecture Check

1. Structural `SeatCount` validation plus a normal `game-stdlib` edge replaces a bespoke predicate and reuses the already-earned helper (§4 — no new `game-stdlib` surface).
2. No backwards-compat alias; the local predicate is replaced.
3. `engine-core` stays noun-free; `game-stdlib` use is the already-earned `SeatCount` (no ring policy). The new dependency is a normal edge, distinct from the C-06 dev-dependency in `-022`.

## Verification Layers

1. Accepted count 2, rejected counts with the exact diagnostic -> `cargo test -p poker_lite` (setup tests).
2. State hashes unchanged -> deterministic replay-hash check (`replay-check --game poker_lite --all`).
3. `SeatCount` adoption + normal `game-stdlib` edge -> codebase grep-proof in `setup.rs` and `Cargo.toml`.

## What to Change

### 1. Add normal game-stdlib dependency

Add `game-stdlib` under `[dependencies]` in `games/poker_lite/Cargo.toml`.

### 2. Adopt SeatCount for structural validation

Replace the hand-written length predicate in `setup_match` with a structural `SeatCount` check; keep `STANDARD_SEAT_COUNT` as the expected value and the diagnostic verbatim.

## Files to Touch

- `games/poker_lite/Cargo.toml` (modify)
- `games/poker_lite/src/setup.rs` (modify)

## Out of Scope

- The C-06 `game-test-support` dev-dependency (`-022`).
- Ring/`next_ring_index` helpers; any diagnostic or golden-trace change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` green, with accepted/rejected seat-count cases.
2. `cargo run -p replay-check -- --game poker_lite --all` — state hashes byte-identical to baseline.

### Invariants

1. The diagnostic and `STANDARD_SEAT_COUNT` expectation are byte-identical to baseline.
2. Only a normal `game-stdlib` edge is added; no dev-dependency and no behavior change.

## Test Plan

### New/Modified Tests

1. `games/poker_lite/src/setup.rs` (or `tests/`) — accepted/rejected structural assertions with the exact diagnostic.

### Commands

1. `cargo test -p poker_lite`
2. `cargo run -p replay-check -- --game poker_lite --all`
