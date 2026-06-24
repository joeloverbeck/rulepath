# 8CR3PUBCOOASY-306: C-03 Frontier Control variant seat-count predicate

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/frontier_control/src/setup.rs`
**Deps**: 8CR3PUBCOOASY-305

## Problem

`frontier_control` separately validates `variant.seat_count` with a bare
comparison. C-03 adopts the behavior-free `SeatCount` for this count-only
predicate as its own diff, keeping variant policy and the faction/graph setup
game-owned. Serialized after 305 (same `setup.rs`, and depends on the
`game-stdlib` edge 305 adds).

## Assumption Reassessment (2026-06-24)

1. `games/frontier_control/src/setup.rs::setup_match` holds
   `if variant.seat_count != STANDARD_SEAT_COUNT` (line ~46). Shipped
   `SeatCount::new`/`.get()` at `crates/game-stdlib/src/seat.rs:9`/`:13`; the
   `game-stdlib` edge is added by 305.
2. Spec §3.6 verdict for Frontier `variant.seat_count` is `migrate` (a separate
   count-only diff); §5.5 task `8C-R3-306` scopes exactly this predicate.
3. Cross-crate boundary under audit: structural `SeatCount` vs the game-local
   variant count check — only the structural count guard is shared; variant and
   faction/graph setup stay local.
4. FOUNDATIONS §4 motivates the boundary: structural adoption only.
5. Enforcement surface: variant acceptance/diagnostic and setup equality;
   faction and graph setup unchanged from the 001 baseline.

## Architecture Check

1. A separate count-only diff keeps the surface independently reviewable per the
   one-surface-per-diff admission rule.
2. No backwards-compatibility alias — the local comparison is replaced.
3. `engine-core` untouched; `SeatCount` is structural only — no faction noun in
   `game-stdlib`.

## Verification Layers

1. Variant accept/reject -> `cargo test -p frontier_control` setup/variant tests.
2. Byte/hash neutrality -> `replay-check --game frontier_control --all` +
   serialization tests byte-identical to baseline.
3. Diagnostic stability -> variant diagnostic unchanged from baseline.

## What to Change

### 1. Adopt `SeatCount` for the variant count predicate

In `setup_match`, replace the `variant.seat_count` comparison with a
`SeatCount`-based structural count comparison. Preserve variant policy and
faction/graph setup.

## Files to Touch

- `games/frontier_control/src/setup.rs` (modify; serialized after 305)

## Out of Scope

- The roster (305) predicate.
- Faction identity/order (exception — recorded by 802), graph, or scoring.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control`.
2. `cargo run -p replay-check -- --game frontier_control --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game frontier_control`.

### Invariants

1. Variant acceptance/diagnostic and setup equality are unchanged from baseline.
2. Variant policy and faction/graph setup remain game-owned.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing setup/replay/serialization suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p frontier_control`
2. `cargo run -p replay-check -- --game frontier_control --all`
3. A per-game test + replay-check is the correct boundary: only the variant
   count predicate changes.

## Outcome

Completed: 2026-06-24

Changed `games/frontier_control/src/setup.rs::validate_variant` to wrap
`variant.seat_count` with `SeatCount::new(variant.seat_count as usize).map(SeatCount::get)`
before comparing to the game-owned `STANDARD_SEAT_COUNT`. The change is
predicate-only; roster validation, faction identity/order, graph setup,
diagnostics, replay/export bytes, and fixtures were otherwise untouched.

Deviations: none.

Verification:

- `cargo test -p frontier_control` passed.
- `cargo run -p replay-check -- --game frontier_control --all` passed.
- `cargo run -p fixture-check -- --game frontier_control` passed.
- No golden trace, fixture, export, or variant policy file changed.
