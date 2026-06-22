# UNI8CMECSCA-005: Add `EffectEnvelope::public` / `private_to` constructors

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `crates/engine-core/src/lib.rs` (`EffectEnvelope<T>` inherent methods)
**Deps**: UNI8CMECSCA-002

## Problem

Games construct `EffectEnvelope<T>` by struct literal, repeating the visibility-scope choice at every effect site (C-01). This ticket adds two behavior-free constructors on the kernel type — `public(payload)` and `private_to(seat_id, payload)` — that set only `visibility` and `payload`. No game is migrated here (that is UNI8CMECSCA-006); this is the engine-core helper plus focused unit tests proving constructor/literal equality and move semantics.

## Assumption Reassessment (2026-06-22)

1. `EffectEnvelope<T>` is defined in `crates/engine-core/src/lib.rs` as a generic struct with exactly the fields `visibility: VisibilityScope` and `payload: T`; `VisibilityScope` is `Public | PrivateToSeat(SeatId)` and `SeatId(pub String)` in the same file. No existing `public`/`private_to` constructor exists — confirmed by grep at the reassessed commit.
2. Spec §4.3 C-01 fixes the signatures: `pub fn public(payload: T) -> Self` and `pub fn private_to(seat_id: SeatId, payload: T) -> Self`; `private_to` takes an already-typed `SeatId`, not `impl Into<String>`. Register entry `MSC-8C-001` (UNI8CMECSCA-002) homes this in `engine-core`.
3. Cross-artifact boundary under audit: the effect-envelope contract in `crates/engine-core/src/lib.rs` and `docs/ENGINE-GAME-DATA-BOUNDARY.md`. The constructors add no field and change no serialization.
4. FOUNDATIONS §3: `engine-core` already owns `EffectEnvelope`/`VisibilityScope`/`SeatId` (allowed kernel nouns); adding constructors introduces no mechanic noun and no game policy.
5. Visibility/no-leak surface under audit (§11): the constructors set `visibility` from the explicit argument only — they perform no filtering, reveal, projection, or serialization — so they cannot change who may observe an effect; the game payload and `VisibilityScope` still decide that.

## Architecture Check

1. Inherent constructors on the kernel type are the narrowest lawful home (the type and visibility vocabulary already live there); a `game-stdlib` wrapper would add an unnecessary layer.
2. No backwards-compatibility shim — the existing struct-literal form keeps working; the constructors are additive.
3. `engine-core` stays noun-free; `game-stdlib` is not touched (no earned-promotion question).

## Verification Layers

1. `public(p)` ≡ `EffectEnvelope { visibility: Public, payload: p }` and `private_to(s, p)` ≡ the `PrivateToSeat(s)` literal → engine-core unit tests asserting field equality.
2. Payload move semantics preserved (no clone introduced) → unit test moving a non-`Copy` payload.
3. No serialization/hash delta → `cargo test -p engine-core` plus existing `StableSerialize` tests unchanged.

## What to Change

### 1. `crates/engine-core/src/lib.rs` — constructors

Add `impl<T> EffectEnvelope<T>` with `public` and `private_to` as specified; each sets only `visibility` and `payload`. Add rustdoc stating no filtering/reveal/serialization behavior is performed.

### 2. Unit tests

Constructor-vs-literal equality for both scopes; payload move semantics; exact scope values.

## Files to Touch

- `crates/engine-core/src/lib.rs` (modify)

## Out of Scope

- Migrating any game's effect sites (UNI8CMECSCA-006).
- Any `impl Into<String>` ergonomics on `private_to` (spec forbids; takes typed `SeatId`).
- Any serialization, filtering, or reveal logic.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p engine-core` passes, including the new constructor tests.
2. A test asserts `EffectEnvelope::public(p)` and `EffectEnvelope::private_to(s, p)` field-equal their struct literals.
3. `cargo build --workspace` and `bash scripts/boundary-check.sh` pass.

### Invariants

1. The constructors set only `visibility` and `payload`; no other behavior.
2. No existing `StableSerialize`/hash output for `EffectEnvelope` changes.

## Test Plan

### New/Modified Tests

1. `crates/engine-core/src/lib.rs` (inline `#[cfg(test)]`) — constructor/literal equality + move semantics.

### Commands

1. `cargo test -p engine-core`
2. `cargo build --workspace && bash scripts/boundary-check.sh`
3. The engine-core unit suite is the correct boundary — no game consumes the constructors yet.
