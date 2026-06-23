# UNI8CR2TWOSEA-003: High Card Duel — private effect-envelope constructor

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/high_card_duel/src/effects.rs`; adopts `engine-core` `EffectEnvelope::private_to`
**Deps**: 002

## Problem

Spec §3.4 / task `8C-R2-102`: HCD's `private_effect` constructs a seat-private effect envelope with a local literal. R2 migrates only construction to `EffectEnvelope::private_to`, preserving the private owner `SeatId`, filtered effects, private diagnostics/deal/commit payloads, effect hash, and observer/opponent filtering. Recipient choice and reveal policy stay game-owned. Shares `effects.rs` with `-002` (hence `Deps: 002`).

## Assumption Reassessment (2026-06-23)

1. `games/high_card_duel/src/effects.rs::private_effect(payload, seat)` exists (confirmed ~line 140) and returns a seat-private `EffectEnvelope`; the owner `SeatId` is supplied by game code.
2. Spec §3.4/§3.8/§9: private owner IDs and filtering must be preserved; the HCD C-07 pilot receipt (`MSC-8C-007`) is verified not rebuilt; no reveal-timing change.
3. Cross-crate boundary under audit: `engine-core::EffectEnvelope::private_to` (confirmed `crates/engine-core/src/lib.rs:158`) and `VisibilityScope::PrivateToSeat` — the generic seat-private constructor adopted here.
4. Deterministic + no-leak surface: the private effect hash and the filtered-effects projection. Confirm the migrated constructor keeps the unrevealed private card absent from observer/opponent surfaces and present only for the owner (§11 no-leak firewall), byte-identical to the `-001` baseline.

## Architecture Check

1. Adopting `EffectEnvelope::private_to` unifies seat-private construction with Poker's constructor while leaving recipient selection in game code — cleaner than a bespoke literal.
2. No backwards-compat alias; the local literal is replaced outright.
3. `engine-core` stays noun-free; no `game-stdlib` change.

## Verification Layers

1. Private effect hash + owner `SeatId` unchanged -> deterministic replay-hash check (`replay-check --game high_card_duel --all`).
2. Unrevealed private card absent for observer/opponent, present for owner -> no-leak visibility test (`tests/visibility.rs`, `tests/bots.rs`).
3. `EffectEnvelope::private_to` adoption -> codebase grep-proof in `effects.rs`.

## What to Change

### 1. Adopt EffectEnvelope::private_to

Replace the local seat-private envelope literal in `private_effect` with `EffectEnvelope::private_to`, passing the existing payload and owner seat unchanged.

## Files to Touch

- `games/high_card_duel/src/effects.rs` (modify; serialized after `-002`)

## Out of Scope

- The public constructor (`-002`), payload formation, reveal timing, recipient choice.
- Rebuilding or re-asserting the C-07 pilot matrix (that is `-024`).
- Any golden-trace or fixture byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel` green, including private-effect visibility/no-leak tests.
2. `cargo run -p replay-check -- --game high_card_duel --all` — effect hashes byte-identical to baseline.

### Invariants

1. Private owner `SeatId` and the filtered-effects projection are byte-identical to baseline.
2. No hidden datum reaches observer/opponent effect surfaces.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/visibility.rs` — assert private effect owner-only exposure unchanged (extend existing coverage).

### Commands

1. `cargo test -p high_card_duel`
2. `cargo run -p replay-check -- --game high_card_duel --all`
