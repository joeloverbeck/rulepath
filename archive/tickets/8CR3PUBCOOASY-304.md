# 8CR3PUBCOOASY-304: C-03 Flood Watch role-order cardinality predicate

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/flood_watch/src/setup.rs`
**Deps**: 8CR3PUBCOOASY-303

## Problem

`flood_watch` validates the cooperative `role_order` cardinality with a bare
`role_order.len()` comparison. C-03 adopts the behavior-free `SeatCount` for
this count-only predicate as its own diff, keeping role identities, ordering,
assignment, powers, and cooperative policy game-owned. Serialized after 303
(same `setup.rs`).

## Assumption Reassessment (2026-06-24)

1. `games/flood_watch/src/setup.rs::setup_match` holds
   `if variant.role_order.len() != STANDARD_SEAT_COUNT as usize` (line ~90).
   Shipped `SeatCount::new`/`.get()` at `crates/game-stdlib/src/seat.rs:9`/`:13`;
   the `game-stdlib` edge is added by 302.
2. Spec §3.6 verdict for Flood `variant.role_order.len()` is `migrate` (a
   separate count-only diff); §5.5 task `8C-R3-304` scopes exactly this
   predicate. Role identity/order remains an evidence receipt, not code.
3. Cross-crate boundary under audit: structural `SeatCount` cardinality vs the
   game-local role-order length check — only the count guard is shared; role
   identity/order/powers stay local.
4. FOUNDATIONS §4 motivates the boundary: structural adoption only; the
   Multi-Seat contract distinguishes role identity from generic seat structure,
   so no role semantics enter `game-stdlib`.
5. Enforcement surface: role-order cardinality acceptance/diagnostic; role
   identities/order/powers and all setup hashes byte-identical to baseline.

## Architecture Check

1. A separate count-only diff for the role-order cardinality keeps the surface
   independently reviewable; cleaner than folding it into another predicate.
2. No backwards-compatibility alias — the local comparison is replaced.
3. `engine-core` untouched; `SeatCount` proves cardinality only — role identity
   and order stay game-owned (no role noun in `game-stdlib`).

## Verification Layers

1. Cardinality accept/reject -> `cargo test -p flood_watch` setup/role tests.
2. Byte/hash neutrality -> `replay-check --game flood_watch --all` +
   serialization tests byte-identical to baseline.
3. Role policy preserved -> role identities/order/powers unchanged (manual +
   rules tests).

## What to Change

### 1. Adopt `SeatCount` for role-order cardinality

In `setup_match`, replace the `variant.role_order.len()` comparison with a
`SeatCount`-based structural cardinality comparison. Preserve role identities,
ordering, assignment, powers, and cooperative policy.

## Files to Touch

- `games/flood_watch/src/setup.rs` (modify; serialized after 303)

## Out of Scope

- The roster (302) and `variant.seat_count` (303) predicates.
- Role identity/order/power/assignment changes (evidence-only — recorded by 802).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p flood_watch`.
2. `cargo run -p replay-check -- --game flood_watch --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game flood_watch`.

### Invariants

1. Role-order cardinality acceptance/diagnostic and all setup hashes are
   unchanged from baseline.
2. Role identity/order/powers remain game-owned; `SeatCount` provides
   cardinality only.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing setup/rules/replay suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p replay-check -- --game flood_watch --all`
3. A per-game test + replay-check is the correct boundary: only the role-order
   cardinality predicate changes.

## Outcome

Completed: 2026-06-24

Changed `games/flood_watch/src/setup.rs::validate_variant` to wrap
`variant.role_order.len()` with `SeatCount::new(variant.role_order.len()).map(SeatCount::get)`
before comparing to the game-owned `STANDARD_SEAT_COUNT`. The change is
predicate-only; role identities, ordering, assignment, powers, variant policy,
event-deck setup, diagnostics, replay/export bytes, and fixtures were otherwise
untouched.

Deviations: none.

Verification:

- `cargo test -p flood_watch` passed.
- `cargo run -p replay-check -- --game flood_watch --all` passed.
- `cargo run -p fixture-check -- --game flood_watch` passed.
- No golden trace, fixture, export, role policy, or variant policy file changed.
