# UNI8CR2TWOSEA-013: High Card Duel — exact-two-seat structural validation

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/high_card_duel/src/setup.rs`; adopts `game-stdlib` `SeatCount`
**Deps**: 001

## Problem

Spec §3.6 / task `8C-R2-301`: `setup_match` validates seat structure with a hand-written predicate. R2 validates nonzero structure with `SeatCount` while retaining `options.variant.seat_count` as the game-owned expected value, and preserves the exact diagnostic code/message and setup/RNG/state behavior. The existing `game-stdlib` dependency is reused; no ring helper is adopted.

## Assumption Reassessment (2026-06-23)

1. `games/high_card_duel/src/setup.rs::setup_match` exists (confirmed ~line 22) and compares `seats.len()` against `options.variant.seat_count`; `high_card_duel` already depends on `game-stdlib`.
2. Spec §3.6: `migrate`; variant expected count stays game-owned; diagnostics byte-identical; `SeatCount::next_ring_index` is `not-applicable` this wave.
3. Cross-crate boundary under audit: `game-stdlib::SeatCount` (`crates/game-stdlib/src/seat.rs`) — structural count helper; the variant expectation stays in game code.
4. Determinism: diagnostics, setup shuffle/deal, and state hashes must stay byte-identical to the `-001` baseline; structural validation introduces no nondeterministic input (§11).

## Architecture Check

1. Using the structural `SeatCount` helper for the nonzero/structure check while keeping the variant's expected count game-owned is cleaner than a bespoke length predicate and earns no new `game-stdlib` surface (§4 — `SeatCount` already shipped).
2. No backwards-compat alias; the local predicate is replaced.
3. `engine-core` stays noun-free; `game-stdlib` use is the already-earned `SeatCount` (no ring policy).

## Verification Layers

1. Accepted count 2, rejected 0/1/3 with the exact diagnostic code/message -> `cargo test -p high_card_duel` (setup tests).
2. Setup/RNG/state hashes unchanged -> deterministic replay-hash check (`replay-check --game high_card_duel --all`).
3. `SeatCount` adoption, no ring helper -> codebase grep-proof in `setup.rs`.

## What to Change

### 1. Adopt SeatCount for structural validation

Replace the hand-written length predicate with a structural `SeatCount` check; keep `options.variant.seat_count` as the expected value and the existing diagnostic verbatim.

## Files to Touch

- `games/high_card_duel/src/setup.rs` (modify)

## Out of Scope

- C-09 RNG adoption (`-042`), the variant expected-count policy, ring/`next_ring_index` helpers.
- Any diagnostic message or golden-trace change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel` green, with accepted/rejected seat-count cases.
2. `cargo run -p replay-check -- --game high_card_duel --all` — setup/state hashes byte-identical to baseline.

### Invariants

1. The diagnostic code/message and variant expected-count policy are byte-identical to baseline.
2. No ring helper is adopted.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/src/setup.rs` (or `tests/`) — accepted-2 / rejected-0/1/3 structural assertions with the exact diagnostic.

### Commands

1. `cargo test -p high_card_duel`
2. `cargo run -p replay-check -- --game high_card_duel --all`

## Outcome

Completed on 2026-06-23. `setup_match` now validates nonzero seat structure
through `SeatCount` and preserves the variant-owned expected count comparison
against `options.variant.seat_count`. The exact `invalid_seat_count`
diagnostic code/message is unchanged, and no ring helper was adopted.

Added setup tests for accepted two-seat setup and rejected 0/1/3 seat counts
with the exact diagnostic.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo test -p high_card_duel`
3. `cargo run -p replay-check -- --game high_card_duel --all`
