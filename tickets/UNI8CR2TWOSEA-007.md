# UNI8CR2TWOSEA-007: Masked Claims — public effect-envelope constructor

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/masked_claims/src/effects.rs`; adopts `engine-core` `EffectEnvelope::public`
**Deps**: 001

## Problem

Spec §3.4 / task `8C-R2-106`: Masked Claims has `public_effect` only and emits **no** seat-private envelope. R2 migrates the public constructor to `EffectEnvelope::public`, preserving claim/reaction public effects and redacted payloads. The private constructor is `not-applicable` (report/register receipt); claim-path redaction and reveal timing stay in game-owned payload/projection code.

## Assumption Reassessment (2026-06-23)

1. `games/masked_claims/src/effects.rs::public_effect` exists (confirmed ~line 85) and there is no `private_effect` — verified during the reassess pass.
2. Spec §3.4: public `migrate`, private `not-applicable`; §3.12/§9 forbid moving claim redaction / reaction / challenge-reveal policy into shared code or inventing a private effect.
3. Cross-crate boundary under audit: `engine-core::EffectEnvelope::public` (`crates/engine-core/src/lib.rs:149`) — generic constructor adopted; no mechanic noun crosses into the kernel.
4. Deterministic + no-leak surface: the public effect stable string + hash; confirm claimed/masked tile identities stay redacted in public effects (§11 no-leak firewall) and serialized output is byte-identical to the `-001` baseline.

## Architecture Check

1. Adopting `EffectEnvelope::public` aligns Masked's constructor with the other three games while leaving redaction/reaction policy in game code — cleaner than the local literal.
2. No backwards-compat alias; the literal is replaced outright.
3. `engine-core` stays noun-free; no `game-stdlib` change; the private N/A is a receipt, not new code.

## Verification Layers

1. Public effect stable string + hash unchanged -> deterministic replay-hash check (`replay-check --game masked_claims --all`).
2. Claimed/masked tile identity redacted in public effects across viewers -> no-leak visibility test (`tests/visibility.rs`).
3. `EffectEnvelope::public` adoption -> codebase grep-proof in `effects.rs`.

## What to Change

### 1. Adopt EffectEnvelope::public

Replace the local public-envelope literal in `public_effect` with `EffectEnvelope::public`, passing the existing redacted payload and visibility unchanged.

## Files to Touch

- `games/masked_claims/src/effects.rs` (modify)

## Out of Scope

- Inventing a private constructor (the private N/A is recorded in `-045`).
- Payload formation, claim redaction, reaction-window/challenge-reveal timing.
- Any golden-trace or fixture byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` green, including effect serialization/visibility tests.
2. `cargo run -p replay-check -- --game masked_claims --all` — effect hashes byte-identical to baseline.

### Invariants

1. Serialized/stable public effect output is byte-identical to baseline.
2. Claimed/masked tile identities never appear unredacted in a public effect payload.

## Test Plan

### New/Modified Tests

1. `games/masked_claims/tests/serialization.rs` — assert public effect stable string/hash unchanged (extend existing coverage if needed).

### Commands

1. `cargo test -p masked_claims`
2. `cargo run -p replay-check -- --game masked_claims --all`
