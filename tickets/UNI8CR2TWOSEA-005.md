# UNI8CR2TWOSEA-005: Poker Lite — public effect-envelope constructor

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/poker_lite/src/effects.rs`; adopts `engine-core` `EffectEnvelope::public`
**Deps**: 001

## Problem

Spec §3.4 / task `8C-R2-104`: Poker Lite's `public_effect` constructs a public effect envelope with a local literal. R2 migrates only construction to `EffectEnvelope::public`, preserving public effects and reveal-safe stable strings/hashes. Reveal timing, pot, and showdown policy remain game-owned.

## Assumption Reassessment (2026-06-23)

1. `games/poker_lite/src/effects.rs::public_effect` exists (confirmed ~line 91) and returns a public `EffectEnvelope<PokerLiteEffect>`; payload formation stays in game code.
2. Spec §3.4 assigns Poker public constructor `migrate`; §3.12/§9 forbid moving showdown/yield/pot policy into shared code.
3. Cross-crate boundary under audit: `engine-core::EffectEnvelope::public` (`crates/engine-core/src/lib.rs:149`) — generic constructor adopted; no mechanic noun crosses into the kernel.
4. Deterministic surface: the public effect stable string + hash; confirm byte-identical serialized output for observer and both seats and that no private crest reaches a public effect (§11 determinism + no-leak), matching the `-001` baseline.

## Architecture Check

1. Delegating construction to `EffectEnvelope::public` removes a hand-rolled literal while keeping payload/visibility decisions local — consistent with the other games.
2. No backwards-compat alias; the literal is replaced outright.
3. `engine-core` stays noun-free; no `game-stdlib` change.

## Verification Layers

1. Public effect stable string + hash unchanged -> deterministic replay-hash check (`replay-check --game poker_lite --all`).
2. Observer/seat0/seat1 filtered effects byte-identical; no private crest in public effects -> no-leak visibility test (`tests/visibility.rs`).
3. `EffectEnvelope::public` adoption -> codebase grep-proof in `effects.rs`.

## What to Change

### 1. Adopt EffectEnvelope::public

Replace the local public-envelope literal in `public_effect` with `EffectEnvelope::public`, passing the existing payload and visibility unchanged.

## Files to Touch

- `games/poker_lite/src/effects.rs` (modify)

## Out of Scope

- The private constructor (`-006`), payload formation, showdown/yield/pot policy.
- Any golden-trace or fixture byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` green, including effect serialization/visibility tests.
2. `cargo run -p replay-check -- --game poker_lite --all` — effect hashes byte-identical to baseline.

### Invariants

1. Serialized/stable public effect output is byte-identical to baseline.
2. No `private_to` recipient or `VisibilityScope` change is introduced by this diff.

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/serialization.rs` — assert public effect stable string/hash unchanged (extend existing coverage if needed).

### Commands

1. `cargo test -p poker_lite`
2. `cargo run -p replay-check -- --game poker_lite --all`
