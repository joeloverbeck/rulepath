# 8CR3PUBCOOASY-301: C-03 Plain Tricks roster count

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/plain_tricks/src/setup.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`plain_tricks` checks its roster with a bare `seats.len()` comparison instead of
the shipped behavior-free `SeatCount` structural helper. C-03 constructs a
`SeatCount` and compares `.get()` to `STANDARD_SEAT_COUNT`, keeping the exact
expected count, diagnostic, deal/leader rotation, and setup state game-owned.
Plain has no `variant.seat_count` predicate, so that row is N/A (recorded by the
register ticket 802) — this ticket must not add a new acceptance/rejection rule.

## Assumption Reassessment (2026-06-24)

1. `games/plain_tricks/src/setup.rs::setup_match` (line ~29) holds the predicate
   `if seats.len() != STANDARD_SEAT_COUNT as usize` (line ~34);
   `STANDARD_SEAT_COUNT: u8 = 2` is in `games/plain_tricks/src/ids.rs:4`. The
   shipped `SeatCount::new` / `.get()` are at `crates/game-stdlib/src/seat.rs:9`
   and `:13`. Plain already has a `game-stdlib` edge
   (`games/plain_tricks/Cargo.toml:11`), so no Cargo change is needed.
2. Spec §3.6 verdict for Plain `seats.len()` is `migrate`; the Plain
   `variant.seat_count` row is `not-applicable` (current setup does not enforce
   it); §5.5 task `8C-R3-301` scopes the roster predicate only.
3. Cross-crate boundary under audit: `game-stdlib::SeatCount` (structural
   cardinality only) vs the game-local exact count — the helper proves non-zero
   structure and provides a typed count; the exact expected count, diagnostic,
   and policy stay in the game.
4. FOUNDATIONS §4 motivates the boundary: `SeatCount` is a structural primitive,
   not a count policy — adopting it must pull no game count, role, or rotation
   into `game-stdlib`.
5. Enforcement surface: the accepted (2) / rejected (0/1/3 where callable) count
   vectors and the exact diagnostic code/message, plus setup/deal/RNG/state/
   effect/replay/view hashes; all stay byte-identical to the 001 baseline.

## Architecture Check

1. Wrapping the roster length in `SeatCount` shares the non-zero structural
   guard without moving the exact-count policy; cleaner than a bare `len()`
   comparison that re-implements the structural invariant inline.
2. No backwards-compatibility alias — the local length predicate is replaced.
3. `engine-core` untouched; `game-stdlib::SeatCount` is structural only — no
   mechanic noun, no count policy, enters the shared crate.

## Verification Layers

1. Accept/reject behavior -> `cargo test -p plain_tricks` setup tests (count 2
   accepts; callable rejects produce the exact diagnostic).
2. Byte/hash neutrality -> `replay-check --game plain_tricks --all` +
   serialization tests byte-identical to baseline.
3. Diagnostic stability -> exact diagnostic code/message unchanged from baseline.

## What to Change

### 1. Adopt `SeatCount` for the roster predicate

In `setup_match`, construct `SeatCount::new(seats.len())` and compare `.get()`
to `STANDARD_SEAT_COUNT`. Preserve the existing exact diagnostic, two-seat
policy, deal/leader rotation, and setup state.

## Files to Touch

- `games/plain_tricks/src/setup.rs` (modify)

## Out of Scope

- Adding a `variant.seat_count` acceptance/rejection rule (N/A — recorded by 802).
- `SeatCountRange`, ring/rotation helpers, or any role/leader policy change.
- The C-09 sampler migration (ticket 701).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` (setup accept/reject + diagnostic tests).
2. `cargo run -p replay-check -- --game plain_tricks --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game plain_tricks`.

### Invariants

1. Accepted/rejected counts, the exact diagnostic, and all setup/deal/state
   hashes are unchanged from baseline.
2. `SeatCount` provides structure only; the exact count and policy remain
   game-owned.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing setup/replay/serialization suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all`
3. A per-game test + replay-check is the correct boundary: only the Plain roster
   predicate changes; the diagnostic and hashes are asserted unchanged.

## Outcome

Completed: 2026-06-24

Changed `games/plain_tricks/src/setup.rs::setup_match` to wrap the roster
length with `SeatCount::new(seats.len()).map(SeatCount::get)` before comparing
to the game-owned `STANDARD_SEAT_COUNT`. The change is predicate-only; exact
two-seat policy, diagnostics, variant policy, deal/leader rotation, RNG
sampling, replay/export bytes, and fixtures were otherwise untouched.

Deviations: none.

Verification:

- `cargo test -p plain_tricks` passed.
- `cargo run -p replay-check -- --game plain_tricks --all` passed.
- `cargo run -p fixture-check -- --game plain_tricks` passed.
- No golden trace, fixture, export, or setup policy file changed.
