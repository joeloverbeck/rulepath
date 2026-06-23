# UNI8CR2TWOSEA-004: Secret Draft — public effect-envelope constructor

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/secret_draft/src/effects.rs`; adopts `engine-core` `EffectEnvelope::public`
**Deps**: 001

## Problem

Spec §3.4 / task `8C-R2-103`: Secret Draft has `public_effect` only and emits **no** seat-private envelope. R2 migrates the public constructor to `EffectEnvelope::public`; the private constructor is `not-applicable` and lands as a report/register receipt, not synthetic code. `OwnCommitAccepted` and pre-reveal diagnostics stay viewer-safe by omitting the committed item — commitment/reveal policy is not moved and no private effect is invented.

## Assumption Reassessment (2026-06-23)

1. `games/secret_draft/src/effects.rs::public_effect` exists (confirmed ~line 78) and there is no `private_effect` — verified during the reassess pass.
2. Spec §3.4: public `migrate`, private `not-applicable`; §3.12/§9 forbid inventing a private effect or moving commitment/reveal policy into shared code.
3. Cross-crate boundary under audit: `engine-core::EffectEnvelope::public` (`crates/engine-core/src/lib.rs:149`) — generic constructor adopted; no mechanic noun crosses into the kernel.
4. Deterministic + no-leak surface: the public effect stable string + hash; confirm the committed item stays absent from the public payload (§11 no-leak firewall) and serialized output is byte-identical to the `-001` baseline.

## Architecture Check

1. Adopting `EffectEnvelope::public` aligns Secret's constructor with the other three games while leaving redaction/commitment policy in game code — cleaner than the local literal.
2. No backwards-compat alias; the literal is replaced outright.
3. `engine-core` stays noun-free; no `game-stdlib` change; the private N/A is a receipt, not new code.

## Verification Layers

1. Public effect stable string + hash unchanged -> deterministic replay-hash check (`replay-check --game secret_draft --all`).
2. Committed item absent pre-reveal across observer/owner/opponent -> no-leak visibility test (`tests/visibility.rs`).
3. `EffectEnvelope::public` adoption -> codebase grep-proof in `effects.rs`.

## What to Change

### 1. Adopt EffectEnvelope::public

Replace the local public-envelope literal in `public_effect` with `EffectEnvelope::public`, passing the existing redacted payload and visibility unchanged.

## Files to Touch

- `games/secret_draft/src/effects.rs` (modify)

## Out of Scope

- Inventing a private constructor (the private N/A is recorded in `-045`).
- Payload formation, commitment/reveal timing, redaction policy.
- Any golden-trace or fixture byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft` green, including effect serialization/visibility tests.
2. `cargo run -p replay-check -- --game secret_draft --all` — effect hashes byte-identical to baseline.

### Invariants

1. Serialized/stable public effect output is byte-identical to baseline.
2. The committed item never appears in a public effect payload pre-reveal.

## Test Plan

### New/Modified Tests

1. `games/secret_draft/tests/serialization.rs` — assert public effect stable string/hash unchanged (extend existing coverage if needed).

### Commands

1. `cargo test -p secret_draft`
2. `cargo run -p replay-check -- --game secret_draft --all`
