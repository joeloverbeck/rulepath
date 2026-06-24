# 8CR3PUBCOOASY-101: C-01 Plain Tricks public effect constructor

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/plain_tricks/src/effects.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`plain_tricks` builds its public effect envelopes with a game-local literal
constructor instead of the shipped behavior-free `EffectEnvelope::public`. C-01
adopts the canonical constructor so envelope assembly is shared plumbing, while
every game-owned decision (payload formation, effect variants, ordering,
visibility, recipient, reveal timing, filtering) stays local. The migration
must be byte/hash/visibility identical to the baseline.

## Assumption Reassessment (2026-06-24)

1. `games/plain_tricks/src/effects.rs::public_effect` (line ~62) currently
   returns `EffectEnvelope<PlainTricksEffect>` via a local literal; the shipped
   `EffectEnvelope::public` exists at `crates/engine-core/src/lib.rs:149`.
2. Spec §3.4 verdict for `plain_tricks` C-01 public is `migrate`; §5.3 task
   `8C-R3-101` scopes exactly `public_effect`. The seat-private constructor is a
   separate diff (102).
3. Cross-crate boundary under audit: `engine-core`'s `EffectEnvelope` public
   constructor vs the game-local wrapper — only envelope assembly moves; the
   payload type `PlainTricksEffect` and all effect semantics stay in the game.
4. FOUNDATIONS §11 (determinism, viewer-safety) motivates the neutrality
   requirement: a constructor swap must regenerate no golden trace and change no
   effect/replay/export byte.
5. Enforcement surfaces: effect hash + replay checkpoints + public export bytes
   (`tests/replay.rs`, `tests/serialization.rs`); the migration must leave them
   byte-identical to 001's baseline and leak no hidden datum.

## Architecture Check

1. Reusing the shipped constructor removes duplicated envelope-assembly literals
   without introducing any behavior; cleaner than maintaining a parallel local
   constructor.
2. No backwards-compatibility alias — the local literal constructor is replaced,
   not shimmed alongside.
3. `engine-core` gains no mechanic noun (it already owns `EffectEnvelope`);
   no `game-stdlib` change.

## Verification Layers

1. Public envelope assembly correctness -> `cargo test -p plain_tricks` effect
   tests.
2. Byte/hash neutrality -> `replay-check --game plain_tricks --all` +
   serialization tests stay byte-identical to 001 baseline.
3. No-leak preserved -> `tests/visibility.rs` public observer expectations
   unchanged.

## What to Change

### 1. Adopt `EffectEnvelope::public`

In `games/plain_tricks/src/effects.rs::public_effect`, construct the envelope
via `EffectEnvelope::public(payload)` instead of the local literal. Preserve the
exact payload, debug/stable rendering, and effect order.

## Files to Touch

- `games/plain_tricks/src/effects.rs` (modify)
- `games/plain_tricks/tests/replay.rs` (modify; only if an assertion references the constructor path — otherwise unchanged)

## Out of Scope

- The seat-private constructor (`private_effect` / `hand_dealt_effect`) — that is
  ticket 102.
- Any payload, ordering, visibility, or reveal-timing change.
- Regenerating any golden trace.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all` — byte-identical to 001 baseline.
3. `cargo run -p fixture-check -- --game plain_tricks`

### Invariants

1. Public effect bytes, effect hash, replay checkpoints, and public export bytes
   are unchanged from baseline.
2. The payload type and all effect semantics remain game-owned.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing replay/serialization/visibility suites and the 001 characterization baseline are the regression guard.`

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all`
3. A per-game `replay-check` is the correct boundary: only `plain_tricks`
   public effects change, so workspace-wide replay is unnecessary here.

## Outcome

Completed: 2026-06-24

Changed `games/plain_tricks/src/effects.rs::public_effect` to construct public
effect envelopes via `EffectEnvelope::public(payload)`. The change is
constructor-only; effect payloads, ordering, private effects, recipient
selection, filtering, reveal policy, replay/export bytes, and tests were
otherwise untouched.

Deviations: none.

Verification:

- `cargo test -p plain_tricks` passed.
- `cargo run -p replay-check -- --game plain_tricks --all` passed.
- `cargo run -p fixture-check -- --game plain_tricks` passed.
- No golden trace, fixture, export, or test file changed.
