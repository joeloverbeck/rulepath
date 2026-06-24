# 8CR4NSEAPRITRI-003: Briar Circuit C-01 owner-private-envelope constructor adoption

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/briar_circuit` (`src/visibility.rs`); owner-private bytes, scope, and reveal timing byte-identical
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-002

## Problem

`games/briar_circuit/src/visibility.rs::private_effect` constructs an owner-private effect envelope by hand-building a `PrivateToSeat` visibility scope for pass selection/exchange instead of the shipped kernel constructor `EffectEnvelope::private_to` (MSC-8C-001). Adopt the constructor with zero change to owner seat, payload/card order, non-owner absence, or reveal timing — an ADR-0009 `unchanged` adoption (spec §3.4 Briar private, §5.3).

## Assumption Reassessment (2026-06-24)

1. `games/briar_circuit/src/visibility.rs::private_effect` currently builds the owner-private envelope with a `PrivateToSeat` literal; `EffectEnvelope::private_to(seat, payload)` exists as an inherent constructor in `crates/engine-core/src/lib.rs`. Confirmed during `/reassess-spec`.
2. Spec §3.4 classifies this as `migrate` / ADR-0009 `unchanged`; register entry MSC-8C-001 owns the constructor; the before-baseline comes from `-001`.
3. Cross-artifact: this ticket `Deps` `-002` to serialize edits on the shared production file `games/briar_circuit/src/visibility.rs`; pass authorization/reveal logic stays local and is not touched.
4. §11 no-leak firewall motivates this ticket: the owner-private pass cards MUST remain visible only to the owning seat and absent for observer and non-owner seats, byte-identical before/after.
5. Enforcement surface = effect visibility scoping + `filter_effects_for_viewer` results and the pass-selection/exchange tests; the constructor swap produces identical bytes (ADR-0009 `unchanged`), leaking no hidden information and changing no reveal timing.

## Architecture Check

1. Routing owner-private construction through the kernel constructor removes a second hand-rolled visibility-scope literal from the game, matching the public arm migrated in `-002`.
2. No backwards-compatibility aliasing or shim is introduced; the literal is replaced, not wrapped.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4); pass-target/reveal policy remains game-local.

## Verification Layers

1. Owner-private bytes/scope unchanged -> deterministic replay-hash check (`replay-check --game briar_circuit --all`).
2. Owner-only visibility (non-owner + observer absence) preserved -> no-leak visibility test over pass selection/exchange in the Briar visibility tests.
3. Constructor adopted -> codebase grep-proof (`EffectEnvelope::private_to` present in `private_effect`; `PrivateToSeat` literal gone from it).

## What to Change

### 1. Adopt `EffectEnvelope::private_to`

In `private_effect`, replace the hand-built `PrivateToSeat` envelope literal with `EffectEnvelope::private_to(owner_seat, payload)`. Preserve owner seat resolution, payload/card ordering, and pass authorization/reveal timing exactly.

## Files to Touch

- `games/briar_circuit/src/visibility.rs` (modify; serialized after `-002`)

## Out of Scope

- The public `effect_envelopes` migration (`-002`).
- Any change to pass-target selection, atomic-exchange, or reveal-timing policy (game-local).
- Changing payload content, owner mapping, or any hash; other games' C-01 adoption.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including a focused owner-only/non-owner-absent pass-card assertion.
2. `cargo run -p replay-check -- --game briar_circuit --all` passes with private-effect/view hashes byte-identical to the `-001` baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The owner-private envelope's bytes and `PrivateToSeat` scope are unchanged; non-owner and observer never see the pass card.
2. No new public symbol or shim is introduced in the game crate.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/visibility.rs` — add/strengthen a focused assertion that `private_effect` yields a seat-private envelope owned by the passing seat, absent for all other viewers.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The per-game replay-check is the correct boundary: this surface is game-local effect serialization and visibility scoping.
