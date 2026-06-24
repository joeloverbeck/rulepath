# 8CR3PUBCOOASY-307: C-03 Event Frontier roster count

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/event_frontier/src/setup.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`event_frontier` checks its roster with a bare `seats.len()` comparison instead
of the shipped behavior-free `SeatCount`. C-03 adopts `SeatCount` for the roster
predicate, keeping the exact two-seat asymmetric policy and diagnostic
game-owned. Event Frontier already has a `game-stdlib` edge, so no Cargo change
is needed. This is the setup.rs chain head for Event Frontier (308 follows; 703
the sampler).

## Assumption Reassessment (2026-06-24)

1. `games/event_frontier/src/setup.rs::setup_match` (line ~52) holds
   `if seats.len() != STANDARD_SEAT_COUNT as usize` (line ~57);
   `STANDARD_SEAT_COUNT: u8 = 2` is in `games/event_frontier/src/ids.rs:7`.
   Event Frontier already has `game-stdlib` (`games/event_frontier/Cargo.toml:11`).
   Shipped `SeatCount::new`/`.get()` at `crates/game-stdlib/src/seat.rs:9`/`:13`.
2. Spec §3.6 verdict for Event `seats.len()` is `migrate`; §5.5 task
   `8C-R3-307` scopes the roster predicate only.
3. Cross-crate boundary under audit: `game-stdlib::SeatCount` structural
   cardinality vs the game-local exact count; the helper is structural only.
4. FOUNDATIONS §4 motivates the boundary: structural adoption only, no policy
   promotion.
5. Enforcement surface: accepted (2) / rejected count vectors, the exact
   diagnostic, and deck/state/replay/view hashes; byte-identical to the 001
   baseline.

## Architecture Check

1. Wrapping the roster length in `SeatCount` shares the non-zero structural
   guard without moving the asymmetric count policy.
2. No backwards-compatibility alias — the local length predicate is replaced.
3. `engine-core` untouched; `SeatCount` is structural only — no faction/event
   noun enters `game-stdlib`.

## Verification Layers

1. Accept/reject behavior -> `cargo test -p event_frontier` setup tests.
2. Byte/hash neutrality -> `replay-check --game event_frontier --all` +
   serialization tests byte-identical to baseline.
3. Diagnostic stability -> exact diagnostic unchanged from baseline.

## What to Change

### 1. Adopt `SeatCount` for the roster predicate

In `setup_match`, construct `SeatCount::new(seats.len())` and compare `.get()`
to `STANDARD_SEAT_COUNT`. Preserve the exact diagnostic and two-seat asymmetric
policy.

## Files to Touch

- `games/event_frontier/src/setup.rs` (modify)

## Out of Scope

- The `variant.seat_count` (308) predicate and the C-09 sampler (703).
- Faction identity/order (exception — recorded by 802); event/resource setup.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` (setup accept/reject + diagnostic).
2. `cargo run -p replay-check -- --game event_frontier --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game event_frontier`.

### Invariants

1. Accepted/rejected counts, the exact diagnostic, and deck/state hashes are
   unchanged from baseline.
2. `SeatCount` provides structure only; the exact count and policy remain
   game-owned.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing setup/replay/serialization suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p replay-check -- --game event_frontier --all`
3. A per-game test + replay-check is the correct boundary: only the Event roster
   predicate changes.
