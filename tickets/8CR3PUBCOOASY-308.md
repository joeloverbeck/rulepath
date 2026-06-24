# 8CR3PUBCOOASY-308: C-03 Event Frontier variant seat-count predicate

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/event_frontier/src/setup.rs`
**Deps**: 8CR3PUBCOOASY-307

## Problem

`event_frontier` separately validates `variant.seat_count` with a bare
comparison. C-03 adopts the behavior-free `SeatCount` for this count-only
predicate as its own diff, keeping variant policy and faction/event/resource
setup game-owned. Serialized after 307 (same `setup.rs`).

## Assumption Reassessment (2026-06-24)

1. `games/event_frontier/src/setup.rs::setup_match` holds
   `if variant.seat_count != STANDARD_SEAT_COUNT` (line ~80). Shipped
   `SeatCount::new`/`.get()` at `crates/game-stdlib/src/seat.rs:9`/`:13`; Event
   Frontier already has the `game-stdlib` edge.
2. Spec §3.6 verdict for Event `variant.seat_count` is `migrate` (a separate
   count-only diff); §5.5 task `8C-R3-308` scopes exactly this predicate.
3. Cross-crate boundary under audit: structural `SeatCount` vs the game-local
   variant count check — only the structural count guard is shared; variant and
   faction/event/resource setup stay local.
4. FOUNDATIONS §4 motivates the boundary: structural adoption only.
5. Enforcement surface: variant acceptance/diagnostic and setup equality;
   faction/event/resource setup unchanged from the 001 baseline.

## Architecture Check

1. A separate count-only diff keeps the surface independently reviewable per the
   one-surface-per-diff admission rule.
2. No backwards-compatibility alias — the local comparison is replaced.
3. `engine-core` untouched; `SeatCount` is structural only — no faction noun in
   `game-stdlib`.

## Verification Layers

1. Variant accept/reject -> `cargo test -p event_frontier` setup/variant tests.
2. Byte/hash neutrality -> `replay-check --game event_frontier --all` +
   serialization tests byte-identical to baseline.
3. Diagnostic stability -> variant diagnostic unchanged from baseline.

## What to Change

### 1. Adopt `SeatCount` for the variant count predicate

In `setup_match`, replace the `variant.seat_count` comparison with a
`SeatCount`-based structural count comparison. Preserve variant policy and
faction/event/resource setup.

## Files to Touch

- `games/event_frontier/src/setup.rs` (modify; serialized after 307)

## Out of Scope

- The roster (307) predicate and the C-09 sampler (703).
- Faction identity/order (exception — recorded by 802); event/resource setup.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier`.
2. `cargo run -p replay-check -- --game event_frontier --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game event_frontier`.

### Invariants

1. Variant acceptance/diagnostic and setup equality are unchanged from baseline.
2. Variant policy and faction/event/resource setup remain game-owned.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing setup/replay/serialization suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p replay-check -- --game event_frontier --all`
3. A per-game test + replay-check is the correct boundary: only the variant
   count predicate changes.
