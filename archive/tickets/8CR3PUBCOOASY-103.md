# 8CR3PUBCOOASY-103: C-01 Flood Watch public effect constructor

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/flood_watch/src/effects.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`flood_watch` builds its public effect envelopes with a game-local literal
constructor instead of the shipped `EffectEnvelope::public`. C-01 adopts the
canonical constructor; forecast/event/levee visibility decisions stay local.
`flood_watch` has no seat-private effect class, so only the public constructor
migrates (the private N/A is recorded by the register ticket 802). The change
must be byte/hash/visibility neutral.

## Assumption Reassessment (2026-06-24)

1. `games/flood_watch/src/effects.rs::public_effect` (line ~58) returns
   `FloodWatchEffectEnvelope` via a local literal; shipped
   `EffectEnvelope::public` exists at `crates/engine-core/src/lib.rs:149`.
2. Spec §3.4 verdict for `flood_watch` C-01 is public `migrate`, private
   `not-applicable`; §5.3 task `8C-R3-103` scopes exactly `public_effect`.
3. Cross-crate boundary under audit: `EffectEnvelope::public` vs the
   game-local public wrapper — only envelope assembly moves; the
   `FloodWatchEffect` payload and forecast/event semantics stay local.
4. FOUNDATIONS §11 (determinism, viewer-safety) motivates the neutrality
   requirement: no golden trace regenerates and no effect/replay/export byte
   changes.
5. Enforcement surfaces: effect hash, replay checkpoints, public export bytes
   (`tests/replay.rs`, `tests/serialization.rs`); kept byte-identical to the 001
   baseline with no hidden future-deck datum leaking.

## Architecture Check

1. Reusing the shipped constructor removes a duplicated envelope literal with no
   behavior change; cleaner than a parallel local constructor.
2. No backwards-compatibility alias — the local literal is replaced.
3. `engine-core` already owns `EffectEnvelope`; no mechanic noun added, no
   `game-stdlib` change.

## Verification Layers

1. Public envelope assembly correctness -> `cargo test -p flood_watch` effect
   tests.
2. Byte/hash neutrality -> `replay-check --game flood_watch --all` +
   serialization tests byte-identical to baseline.
3. No-leak preserved -> `tests/visibility.rs` public-observer / hidden-future
   expectations unchanged.

## What to Change

### 1. Adopt `EffectEnvelope::public`

In `games/flood_watch/src/effects.rs::public_effect`, construct via
`EffectEnvelope::public(payload)` instead of the local literal. Preserve the
exact payload, rendering, and effect order.

## Files to Touch

- `games/flood_watch/src/effects.rs` (modify)
- `games/flood_watch/tests/replay.rs` (modify; only if an assertion references the constructor path — otherwise unchanged)

## Out of Scope

- Any other game's effects; any private effect class (N/A — recorded by 802).
- Payload, ordering, forecast/event visibility, or reveal-timing changes.
- Regenerating any golden trace.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p flood_watch`
2. `cargo run -p replay-check -- --game flood_watch --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game flood_watch`

### Invariants

1. Public effect bytes, effect hash, replay checkpoints, and public export bytes
   are unchanged from baseline.
2. The payload type and forecast/event semantics remain game-owned.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing replay/serialization/visibility suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p replay-check -- --game flood_watch --all`
3. A per-game `replay-check` is the correct boundary: only `flood_watch` public
   effects change.

## Outcome

Completed: 2026-06-24

Changed `games/flood_watch/src/effects.rs::public_effect` to construct public
effect envelopes via `EffectEnvelope::public(payload)`. The change is
constructor-only; forecast/event/levee payloads, visibility policy, hidden
future-deck redaction, effect order, replay/export bytes, and tests were
otherwise untouched.

Deviations: none.

Verification:

- `cargo test -p flood_watch` passed.
- `cargo run -p replay-check -- --game flood_watch --all` passed.
- `cargo run -p fixture-check -- --game flood_watch` passed.
- No golden trace, fixture, export, or test file changed.
