# UNI8CR2TWOSEA-002: High Card Duel — public effect-envelope constructor

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/high_card_duel/src/effects.rs`; adopts `engine-core` `EffectEnvelope::public`
**Deps**: 001

## Problem

Spec §3.4 / task `8C-R2-101`: High Card Duel's `public_effect` constructs a public effect envelope with a local literal. R2 migrates only the envelope *construction* to `EffectEnvelope::public`, preserving payloads, `VisibilityScope`, stable effect strings/hashes, ordering, owner IDs, and observer/opponent filtering. Effect ordering, reveal state, recipient choice, and filtering remain game-owned.

## Assumption Reassessment (2026-06-23)

1. `games/high_card_duel/src/effects.rs::public_effect` exists (confirmed ~line 133 in the reassess pass) and returns a public `EffectEnvelope<HighCardDuelEffect>`; payload formation stays in game code.
2. Spec §3.4 assigns HCD public constructor `migrate`; §3.7/§9 forbid relabeling the C-07 debug snapshot as C-04 and forbid changing effect stable strings.
3. Cross-crate boundary under audit: `engine-core::EffectEnvelope::public` (confirmed `crates/engine-core/src/lib.rs:149`) — the generic constructor this game adopts; no mechanic noun crosses into the kernel.
4. Deterministic surface: the public effect stable string + effect hash. Confirm the migrated constructor yields byte-identical serialized/stable effect output for observer and both seats (no `private_to` recipient introduced), preserving §11 determinism with no new no-leak exposure (public effects are already observer-visible).

## Architecture Check

1. Delegating construction to the shared `EffectEnvelope::public` removes a hand-rolled literal while keeping payload/visibility decisions local — cleaner and consistent with the other three games' constructors.
2. No backwards-compat alias; the local literal constructor is replaced outright.
3. `engine-core` stays noun-free (it only sees the opaque `HighCardDuelEffect` payload); no `game-stdlib` change.

## Verification Layers

1. Public effect stable string + hash unchanged -> golden trace / deterministic replay-hash check (`replay-check --game high_card_duel --all`).
2. Observer/seat0/seat1 filtered effects byte-identical -> no-leak visibility test (`tests/visibility.rs`) + `cargo test -p high_card_duel`.
3. `EffectEnvelope::public` adoption -> codebase grep-proof in `effects.rs`.

## What to Change

### 1. Adopt EffectEnvelope::public

Replace the local public-envelope literal in `public_effect` with `EffectEnvelope::public`, passing the existing payload and visibility unchanged.

## Files to Touch

- `games/high_card_duel/src/effects.rs` (modify)

## Out of Scope

- The private constructor (`-003`), payload formation, effect ordering, reveal/recipient policy.
- Any golden-trace or fixture byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel` green, including effect serialization/visibility tests.
2. `cargo run -p replay-check -- --game high_card_duel --all` — effect hashes byte-identical to the `-001` baseline.

### Invariants

1. Serialized/stable public effect output is byte-identical to baseline.
2. No `private_to` recipient or `VisibilityScope` change is introduced by this diff.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/serialization.rs` — assert public effect stable string/hash unchanged (extend existing coverage if not already asserted).

### Commands

1. `cargo test -p high_card_duel`
2. `cargo run -p replay-check -- --game high_card_duel --all`
