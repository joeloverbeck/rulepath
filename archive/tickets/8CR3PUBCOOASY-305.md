# 8CR3PUBCOOASY-305: C-03 Frontier Control roster count + game-stdlib edge

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/frontier_control/Cargo.toml`, `games/frontier_control/src/setup.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`frontier_control` checks its roster with a bare `seats.len()` comparison and
has no `game-stdlib` dependency yet. C-03 adds a normal `game-stdlib` edge and
adopts the behavior-free `SeatCount` for the roster predicate, keeping the exact
two-seat asymmetric policy and diagnostic game-owned. This is the setup.rs chain
head for Frontier Control (306 follows; 503 adds the dev-dep).

## Assumption Reassessment (2026-06-24)

1. `games/frontier_control/src/setup.rs::setup_match` (line ~26) holds
   `if seats.len() != STANDARD_SEAT_COUNT as usize` (line ~30);
   `STANDARD_SEAT_COUNT: u8 = 2` is in `games/frontier_control/src/ids.rs:6`.
   `games/frontier_control/Cargo.toml` has **no** `game-stdlib` edge yet
   (confirmed). Shipped `SeatCount::new`/`.get()` at
   `crates/game-stdlib/src/seat.rs:9`/`:13`.
2. Spec §3.6 verdict for Frontier `seats.len()` is `migrate`; §3.6 notes
   Frontier may require a new normal `game-stdlib` dependency; §5.5 task
   `8C-R3-305` scopes the Cargo edge + roster predicate.
3. Cross-crate boundary under audit: `game-stdlib::SeatCount` structural
   cardinality vs the game-local exact count; the new edge must stay noun-free.
4. FOUNDATIONS §4 motivates the boundary: structural adoption only, no policy
   promotion; the third-use ledger stays `_None_`.
5. Enforcement surface: accepted (2) / rejected count vectors, the exact
   diagnostic, and setup/state/replay hashes; byte-identical to the 001 baseline.

## Architecture Check

1. Adding the `game-stdlib` edge + `SeatCount` shares the structural guard
   without moving the asymmetric count policy; cleaner than a bare `len()` check.
2. No backwards-compatibility alias — the local length predicate is replaced.
3. `engine-core` untouched; the new `game-stdlib` edge is normal-only and pulls
   in structural `SeatCount` only — no faction or graph noun enters the crate.

## Verification Layers

1. Accept/reject behavior -> `cargo test -p frontier_control` setup tests.
2. Byte/hash neutrality -> `replay-check --game frontier_control --all` +
   serialization tests byte-identical to baseline.
3. Boundary cleanliness -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. Add the `game-stdlib` dependency

In `games/frontier_control/Cargo.toml`, add
`game-stdlib = { path = "../../crates/game-stdlib" }` under `[dependencies]`.

### 2. Adopt `SeatCount` for the roster predicate

In `setup_match`, construct `SeatCount::new(seats.len())` and compare `.get()`
to `STANDARD_SEAT_COUNT`. Preserve the exact diagnostic and two-seat asymmetric
policy.

## Files to Touch

- `games/frontier_control/Cargo.toml` (modify)
- `games/frontier_control/src/setup.rs` (modify)

## Out of Scope

- The `variant.seat_count` (306) predicate.
- The `game-test-support` dev-dependency (503).
- Faction sequence/identity/order (exception — recorded by 802); graph/scoring.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` (setup accept/reject + diagnostic).
2. `cargo run -p replay-check -- --game frontier_control --all` — byte-identical to baseline.
3. `bash scripts/boundary-check.sh`.

### Invariants

1. Accepted/rejected counts, the exact diagnostic, and setup/state hashes are
   unchanged from baseline.
2. The new dependency is `game-stdlib` (structural only); `engine-core` stays
   noun-free.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing setup/replay/serialization suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p frontier_control`
2. `cargo run -p replay-check -- --game frontier_control --all`
3. `bash scripts/boundary-check.sh` is included because this ticket adds a new
   crate dependency edge.

## Outcome

Completed: 2026-06-24

Added a normal `game-stdlib` dependency to
`games/frontier_control/Cargo.toml` and changed
`games/frontier_control/src/setup.rs::setup_match` to wrap the roster length
with `SeatCount::new(seats.len()).map(SeatCount::get)` before comparing to the
game-owned `STANDARD_SEAT_COUNT`. The change is predicate/dependency-only;
variant seat count, faction identity/order, graph setup, diagnostics,
asymmetric two-seat policy, replay/export bytes, and fixtures were otherwise
untouched.

Deviations: none.

Verification:

- `cargo test -p frontier_control` passed.
- `cargo run -p replay-check -- --game frontier_control --all` passed.
- `bash scripts/boundary-check.sh` passed.
- No golden trace, fixture, export, or variant policy file changed.
