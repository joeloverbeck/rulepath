# 8CR3PUBCOOASY-104: C-01 Frontier Control public effect constructor

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/frontier_control/src/effects.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`frontier_control` builds its public effect envelopes with a game-local literal
constructor instead of the shipped `EffectEnvelope::public`. All graph/clash/
scoring effects are public and local; the game has no hidden effect class. C-01
adopts the canonical constructor with byte/hash/visibility neutrality; the
private N/A is recorded by the register ticket 802.

## Assumption Reassessment (2026-06-24)

1. `games/frontier_control/src/effects.rs::public_effect` (line ~74) returns
   `FrontierControlEffectEnvelope` via a local literal; shipped
   `EffectEnvelope::public` exists at `crates/engine-core/src/lib.rs:149`.
2. Spec §3.4 verdict for `frontier_control` C-01 is public `migrate`, private
   `not-applicable`; §5.3 task `8C-R3-104` scopes exactly `public_effect`.
3. Cross-crate boundary under audit: `EffectEnvelope::public` vs the
   game-local public wrapper — only envelope assembly moves; the
   `FrontierControlEffect` payload and graph/clash/scoring semantics stay local.
4. FOUNDATIONS §11 (determinism) motivates neutrality: no golden trace
   regenerates and no effect/replay/export byte changes.
5. Enforcement surfaces: effect hash, replay checkpoints, fully-public export
   bytes (`tests/replay.rs`, `tests/serialization.rs`); kept byte-identical to
   the 001 baseline.

## Architecture Check

1. Reusing the shipped constructor removes a duplicated envelope literal with no
   behavior change.
2. No backwards-compatibility alias — the local literal is replaced.
3. `engine-core` already owns `EffectEnvelope`; no mechanic noun added, no
   `game-stdlib` change.

## Verification Layers

1. Public envelope assembly correctness -> `cargo test -p frontier_control`
   effect tests.
2. Byte/hash neutrality -> `replay-check --game frontier_control --all` +
   serialization tests byte-identical to baseline.
3. Public-view equality preserved -> `tests/visibility.rs` observer=seat0=seat1
   expectations unchanged.

## What to Change

### 1. Adopt `EffectEnvelope::public`

In `games/frontier_control/src/effects.rs::public_effect`, construct via
`EffectEnvelope::public(payload)` instead of the local literal. Preserve the
exact payload, rendering, and effect order.

## Files to Touch

- `games/frontier_control/src/effects.rs` (modify)
- `games/frontier_control/tests/replay.rs` (modify; only if an assertion references the constructor path — otherwise unchanged)

## Out of Scope

- Any other game's effects; any private effect class (N/A — recorded by 802).
- Payload, ordering, or visibility changes.
- Regenerating any golden trace.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control`
2. `cargo run -p replay-check -- --game frontier_control --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game frontier_control`

### Invariants

1. Public effect bytes, effect hash, replay checkpoints, and export bytes are
   unchanged from baseline.
2. The payload type and graph/clash/scoring semantics remain game-owned.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing replay/serialization/visibility suites and the 001 baseline are the regression guard.`

### Commands

1. `cargo test -p frontier_control`
2. `cargo run -p replay-check -- --game frontier_control --all`
3. A per-game `replay-check` is the correct boundary: only `frontier_control`
   public effects change.
