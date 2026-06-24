# 8CR3PUBCOOASY-302: C-03 Flood Watch roster count + game-stdlib edge

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/flood_watch/Cargo.toml`, `games/flood_watch/src/setup.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`flood_watch` checks its roster with a bare `seats.len()` comparison and has no
`game-stdlib` dependency yet. C-03 adds a normal `game-stdlib` edge and adopts
the behavior-free `SeatCount` for the roster predicate, keeping the exact
two-seat cooperative policy and diagnostic game-owned. This is the setup.rs
chain head for Flood Watch (303/304 follow; 502 adds the dev-dep, 702 the
sampler).

## Assumption Reassessment (2026-06-24)

1. `games/flood_watch/src/setup.rs::setup_match` (line ~24) holds
   `if seats.len() != STANDARD_SEAT_COUNT as usize` (line ~29);
   `STANDARD_SEAT_COUNT: u8 = 2` is in `games/flood_watch/src/ids.rs:6`.
   `games/flood_watch/Cargo.toml` has **no** `game-stdlib` edge yet (confirmed).
   Shipped `SeatCount::new`/`.get()` are at `crates/game-stdlib/src/seat.rs:9`/`:13`.
2. Spec §3.6 verdict for Flood `seats.len()` is `migrate`; §3.6 notes Flood may
   require a new normal `game-stdlib` dependency; §5.5 task `8C-R3-302` scopes
   the Cargo edge + roster predicate.
3. Cross-crate boundary under audit: `game-stdlib::SeatCount` structural
   cardinality vs the game-local exact count; the new `game-stdlib` edge must
   stay noun-free and pull no behavior into the shared crate.
4. FOUNDATIONS §4 motivates the boundary: adopting `SeatCount` is structural
   adoption of a shipped primitive, not a promotion — the third-use ledger stays
   `_None_`.
5. Enforcement surface: accepted (2) / rejected count vectors, the exact
   diagnostic, and setup/deck/state/replay hashes; byte-identical to the 001
   baseline.

## Architecture Check

1. Adding the `game-stdlib` edge + `SeatCount` shares the structural guard
   without moving the cooperative count policy; cleaner than a bare `len()`
   check.
2. No backwards-compatibility alias — the local length predicate is replaced.
3. `engine-core` untouched; the new `game-stdlib` edge is normal-only and pulls
   in structural `SeatCount` only — no mechanic noun or policy enters the crate.

## Verification Layers

1. Accept/reject behavior -> `cargo test -p flood_watch` setup tests.
2. Byte/hash neutrality -> `replay-check --game flood_watch --all` +
   serialization tests byte-identical to baseline.
3. Boundary cleanliness -> `bash scripts/boundary-check.sh` (engine-core stays
   noun-free; the new edge is `game-stdlib`, not `engine-core`).

## What to Change

### 1. Add the `game-stdlib` dependency

In `games/flood_watch/Cargo.toml`, add `game-stdlib = { path = "../../crates/game-stdlib" }`
under `[dependencies]`.

### 2. Adopt `SeatCount` for the roster predicate

In `setup_match`, construct `SeatCount::new(seats.len())` and compare `.get()`
to `STANDARD_SEAT_COUNT`. Preserve the exact diagnostic and two-seat policy.

## Files to Touch

- `games/flood_watch/Cargo.toml` (modify)
- `games/flood_watch/src/setup.rs` (modify)

## Out of Scope

- The `variant.seat_count` (303) and `role_order.len()` (304) predicates.
- The `game-test-support` dev-dependency (502) and the C-09 sampler (702).
- `SeatCountRange`, ring/rotation, role identity/order changes.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p flood_watch` (setup accept/reject + diagnostic).
2. `cargo run -p replay-check -- --game flood_watch --all` — byte-identical to baseline.
3. `bash scripts/boundary-check.sh`.

### Invariants

1. Accepted/rejected counts, the exact diagnostic, and setup/deck/state hashes
   are unchanged from baseline.
2. The new dependency is `game-stdlib` (structural only); `engine-core` stays
   noun-free.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing setup/replay/serialization suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p replay-check -- --game flood_watch --all`
3. `bash scripts/boundary-check.sh` is included because this ticket adds a new
   crate dependency edge; it confirms the boundary stays clean.
