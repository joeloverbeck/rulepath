# 8CR3PUBCOOASY-105: C-01 Event Frontier public effect constructor

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/event_frontier/src/effects.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`event_frontier` builds its public effect envelopes with a game-local literal
constructor instead of the shipped `EffectEnvelope::public`. Current/next reveal
and hidden-tail policy stay local; the game has no seat-private effect class.
C-01 adopts the canonical constructor with byte/hash/visibility neutrality; the
private N/A is recorded by the register ticket 802.

## Assumption Reassessment (2026-06-24)

1. `games/event_frontier/src/effects.rs::public_effect` (line ~103) returns
   `EventFrontierEffectEnvelope` via a local literal; shipped
   `EffectEnvelope::public` exists at `crates/engine-core/src/lib.rs:149`.
2. Spec §3.4 verdict for `event_frontier` C-01 is public `migrate`, private
   `not-applicable`; §5.3 task `8C-R3-105` scopes exactly `public_effect`.
3. Cross-crate boundary under audit: `EffectEnvelope::public` vs the
   game-local public wrapper — only envelope assembly moves; the
   `EventFrontierEffect` payload and event/edict semantics stay local.
4. FOUNDATIONS §11 (determinism, viewer-safety) motivates neutrality: no golden
   trace regenerates and no effect/replay/export byte changes; the hidden deeper
   deck must not leak.
5. Enforcement surfaces: effect hash, replay checkpoints, public export bytes
   (no deeper-deck leak) in `tests/replay.rs`, `tests/serialization.rs`; kept
   byte-identical to the 001 baseline.

## Architecture Check

1. Reusing the shipped constructor removes a duplicated envelope literal with no
   behavior change.
2. No backwards-compatibility alias — the local literal is replaced.
3. `engine-core` already owns `EffectEnvelope`; no mechanic noun added, no
   `game-stdlib` change.

## Verification Layers

1. Public envelope assembly correctness -> `cargo test -p event_frontier`
   effect tests.
2. Byte/hash neutrality -> `replay-check --game event_frontier --all` +
   serialization tests byte-identical to baseline.
3. No-leak preserved -> `tests/visibility.rs` hidden deeper-deck expectations
   unchanged.

## What to Change

### 1. Adopt `EffectEnvelope::public`

In `games/event_frontier/src/effects.rs::public_effect`, construct via
`EffectEnvelope::public(payload)` instead of the local literal. Preserve the
exact payload, rendering, and effect order.

## Files to Touch

- `games/event_frontier/src/effects.rs` (modify)
- `games/event_frontier/tests/replay.rs` (modify; only if an assertion references the constructor path — otherwise unchanged)

## Out of Scope

- Any other game's effects; any private effect class (N/A — recorded by 802).
- Payload, ordering, current/next reveal, or hidden-tail policy changes.
- Regenerating any golden trace.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier`
2. `cargo run -p replay-check -- --game event_frontier --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game event_frontier`

### Invariants

1. Public effect bytes, effect hash, replay checkpoints, and public export bytes
   are unchanged from baseline.
2. The payload type and event/edict semantics remain game-owned.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing replay/serialization/visibility suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p replay-check -- --game event_frontier --all`
3. A per-game `replay-check` is the correct boundary: only `event_frontier`
   public effects change.

## Outcome

Completed: 2026-06-24

Changed `games/event_frontier/src/effects.rs::public_effect` to construct
public effect envelopes via `EffectEnvelope::public(payload)`. The change is
constructor-only; current/next card reveal payloads, hidden-tail redaction,
event/edict payloads, public visibility policy, effect order, replay/export
bytes, and tests were otherwise untouched.

Deviations: none.

Verification:

- `cargo test -p event_frontier` passed.
- `cargo run -p replay-check -- --game event_frontier --all` passed.
- `cargo run -p fixture-check -- --game event_frontier` passed.
- No golden trace, fixture, export, or test file changed.
