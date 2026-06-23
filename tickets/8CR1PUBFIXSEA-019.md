# 8CR1PUBFIXSEA-019: Draughts Lite C-03 exact seat-count validation

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/draughts_lite` (`src/setup.rs`); diagnostic bytes and setup state byte-identical
**Deps**: 8CR1PUBFIXSEA-001

## Problem

`games/draughts_lite/src/setup.rs::setup_match` validates seat count with a hand-written predicate (`seats.len() != options.variant.seat_count as usize`) instead of the shared helper `SeatCountRange::inclusive(...).validate(...)` (MSC-8C-003, `game-stdlib::seat`). Adopt the helper while preserving the `invalid_seat_count` diagnostic code, message, accepted count, ordering, and resulting setup state exactly. The game keeps its typed `other()` mapping; no ring/index arithmetic is introduced.

## Assumption Reassessment (2026-06-23)

1. `setup_match` currently uses `if seats.len() != options.variant.seat_count as usize { … invalid_seat_count … }`; `SeatCountRange::inclusive` + `.validate` exist in `crates/game-stdlib/src/seat.rs`. `draughts_lite` already depends on `game-stdlib`. Confirmed during reassessment.
2. Spec §3.5 and §5.6 classify this as `migrate` / ADR-0009 `unchanged`; MSC-8C-003 owns seat-count validation. Replacing the typed `other()` mapping with generic ring arithmetic is forbidden churn (spec §9).
3. Cross-artifact: the helper is `game-stdlib::seat`; the game owns its diagnostic mapping and setup state. Before-baseline (diagnostic bytes) from `-001`.
4. §4 (`game-stdlib` is earned) and §11 motivate this ticket: the helper is an already-shipped promotion; the game adopts it for count/range validity only, deciding no setup policy in shared code.
5. Enforcement surface = the `invalid_seat_count` diagnostic bytes and setup-state hashes; the helper decides only count/range validity, leaving diagnostic mapping, ordering, and state game-local — no hash drift, no hidden-information leak.

## Architecture Check

1. Adopting the shared count validator removes a hand-rolled predicate while keeping the game's diagnostic/state ownership intact — cleaner than duplicating count logic per game.
2. No backwards-compatibility shim is introduced; the predicate is replaced in place.
3. `engine-core` stays noun-free (§3); `game-stdlib::seat` is an earned, already-promoted helper (§4) — adoption only, no new promotion.

## Verification Layers

1. Valid exact-two setup unchanged -> unit test (setup equality) + replay-hash check.
2. Wrong-count diagnostic equality (code + message + accepted count) -> focused diagnostic-bytes assertion.
3. No ordering/state/hash drift -> `replay-check --game draughts_lite --all`.

## What to Change

### 1. Adopt the count validator

Replace the hand-written `seats.len() != …` predicate with `SeatCountRange::inclusive(...).validate(seats.len())`, mapping its result to the existing `invalid_seat_count` diagnostic. Preserve the diagnostic bytes, accepted count, and setup state exactly.

## Files to Touch

- `games/draughts_lite/src/setup.rs` (modify)

## Out of Scope

- Replacing the typed `other()` mapping with generic `usize` ring arithmetic.
- Other games' C-03 adoption.
- Any change to the `invalid_seat_count` code/message or setup state.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` is green, including a wrong-count diagnostic-equality assertion.
2. `cargo run -p replay-check -- --game draughts_lite --all` passes with setup-state and replay hashes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The `invalid_seat_count` diagnostic bytes, accepted count, ordering, and setup state are byte-identical.
2. No generic index/ring arithmetic replaces the typed `other()` mapping.

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/` setup test module — valid-count setup equality + wrong-count diagnostic-bytes assertion across the helper adoption.

### Commands

1. `cargo test -p draughts_lite`
2. `cargo run -p replay-check -- --game draughts_lite --all`
3. The per-game setup tests plus replay-check are the correct boundary: setup/diagnostic ownership is game-local.
