# UNI8CR2TWOSEA-006: Poker Lite — private effect-envelope constructor

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/poker_lite/src/effects.rs`; adopts `engine-core` `EffectEnvelope::private_to`
**Deps**: 005

## Problem

Spec §3.4 / task `8C-R2-105`: Poker Lite's `private_effect` constructs a seat-private effect envelope with a local literal. R2 migrates only construction to `EffectEnvelope::private_to`, preserving private setup card delivery, bot-choice visibility, owner filtering, and effect/export hashes exactly. Shares `effects.rs` with `-005` (hence `Deps: 005`).

## Assumption Reassessment (2026-06-23)

1. `games/poker_lite/src/effects.rs::private_effect(payload, seat)` exists (confirmed ~line 98) and returns a seat-private `EffectEnvelope`; the owner `SeatId` is supplied by game code.
2. Spec §3.4/§3.8/§9: private setup card delivery and bot-choice visibility must be preserved exactly; no showdown/yield policy moves into shared code.
3. Cross-crate boundary under audit: `engine-core::EffectEnvelope::private_to` (`crates/engine-core/src/lib.rs:158`) and `VisibilityScope::PrivateToSeat` — generic seat-private constructor adopted here.
4. Deterministic + no-leak surface: the private effect hash and filtered-effects projection; confirm the private crest stays absent for observer/opponent and present only for the owner (§11 no-leak firewall), byte-identical to the `-001` baseline.

## Architecture Check

1. Adopting `EffectEnvelope::private_to` unifies seat-private construction with HCD's constructor while leaving recipient selection in game code — cleaner than a bespoke literal.
2. No backwards-compat alias; the literal is replaced outright.
3. `engine-core` stays noun-free; no `game-stdlib` change.

## Verification Layers

1. Private effect hash + owner `SeatId` unchanged -> deterministic replay-hash check (`replay-check --game poker_lite --all`).
2. Private crest absent for observer/opponent, present for owner -> no-leak visibility test (`tests/visibility.rs`, `tests/bots.rs`).
3. `EffectEnvelope::private_to` adoption -> codebase grep-proof in `effects.rs`.

## What to Change

### 1. Adopt EffectEnvelope::private_to

Replace the local seat-private envelope literal in `private_effect` with `EffectEnvelope::private_to`, passing the existing payload and owner seat unchanged.

## Files to Touch

- `games/poker_lite/src/effects.rs` (modify; serialized after `-005`)

## Out of Scope

- The public constructor (`-005`), payload formation, showdown/yield policy.
- Any golden-trace or fixture byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` green, including private-effect visibility/no-leak tests.
2. `cargo run -p replay-check -- --game poker_lite --all` — effect hashes byte-identical to baseline.

### Invariants

1. Private owner `SeatId`, setup-card delivery, and bot-choice visibility are byte-identical to baseline.
2. No hidden datum reaches observer/opponent effect surfaces.

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/visibility.rs` — assert private effect owner-only exposure unchanged (extend existing coverage).

### Commands

1. `cargo test -p poker_lite`
2. `cargo run -p replay-check -- --game poker_lite --all`

## Outcome

Completed on 2026-06-23. `games/poker_lite/src/effects.rs::private_effect`
now delegates to `EffectEnvelope::private_to(owner_seat_id, payload)` with the
existing owner `SeatId` and payload unchanged. Added focused coverage for the
constructor helper to pin `VisibilityScope::PrivateToSeat(owner)` and exact
payload preservation.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo test -p poker_lite`
3. `cargo run -p replay-check -- --game poker_lite --all`
