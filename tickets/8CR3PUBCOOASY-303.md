# 8CR3PUBCOOASY-303: C-03 Flood Watch variant seat-count predicate

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/flood_watch/src/setup.rs`
**Deps**: 8CR3PUBCOOASY-302

## Problem

`flood_watch` separately validates `variant.seat_count` with a bare comparison.
C-03 adopts the behavior-free `SeatCount` for this count-only predicate as its
own reviewable diff, keeping variant selection and all setup semantics
game-owned. Serialized after 302 because it edits the same `setup.rs`.

## Assumption Reassessment (2026-06-24)

1. `games/flood_watch/src/setup.rs::setup_match` holds
   `if variant.seat_count != STANDARD_SEAT_COUNT` (line ~84). Shipped
   `SeatCount::new`/`.get()` at `crates/game-stdlib/src/seat.rs:9`/`:13`; the
   `game-stdlib` edge is added by 302.
2. Spec §3.6 verdict for Flood `variant.seat_count` is `migrate` (a separate
   count-only diff); §5.5 task `8C-R3-303` scopes exactly this predicate.
3. Cross-crate boundary under audit: structural `SeatCount` vs the game-local
   variant count check — only the structural count guard is shared; variant
   policy stays local.
4. FOUNDATIONS §4 motivates the boundary: structural adoption only, no policy
   promotion.
5. Enforcement surface: variant acceptance/diagnostic and setup equality;
   byte-identical to the 001 baseline.

## Architecture Check

1. A separate count-only diff for the variant predicate keeps each selected
   surface independently reviewable and rollback-isolated, per the spec's
   one-surface-per-diff admission rule.
2. No backwards-compatibility alias — the local comparison is replaced.
3. `engine-core` untouched; `game-stdlib::SeatCount` is structural only.

## Verification Layers

1. Variant accept/reject -> `cargo test -p flood_watch` setup/variant tests.
2. Byte/hash neutrality -> `replay-check --game flood_watch --all` +
   serialization tests byte-identical to baseline.
3. Diagnostic stability -> variant diagnostic unchanged from baseline.

## What to Change

### 1. Adopt `SeatCount` for the variant count predicate

In `setup_match`, replace the `variant.seat_count` comparison with a
`SeatCount`-based structural count comparison. Preserve variant selection and
all setup semantics.

## Files to Touch

- `games/flood_watch/src/setup.rs` (modify; serialized after 302)

## Out of Scope

- The roster (302) and `role_order.len()` (304) predicates.
- Variant policy, role identity/order, or setup composition changes.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p flood_watch`.
2. `cargo run -p replay-check -- --game flood_watch --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game flood_watch`.

### Invariants

1. Variant acceptance/diagnostic and setup equality are unchanged from baseline.
2. Variant policy remains game-owned; `SeatCount` provides structure only.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing setup/replay/serialization suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p replay-check -- --game flood_watch --all`
3. A per-game test + replay-check is the correct boundary: only the variant
   count predicate changes.
