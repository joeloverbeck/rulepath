# 8CR4NSEAPRITRI-002: Briar Circuit C-01 public-envelope constructor adoption

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/briar_circuit` (`src/visibility.rs`); effect bytes, order, and visibility byte-identical
**Deps**: 8CR4NSEAPRITRI-001

## Problem

`games/briar_circuit/src/visibility.rs::effect_envelopes` constructs the public effect envelope with a `VisibilityScope::Public` struct literal instead of the shipped kernel constructor `EffectEnvelope::public` (MSC-8C-001). Adopt the constructor with zero change to payload, effect order, or visibility — an ADR-0009 `unchanged` adoption (spec §3.4 Briar public, §5.3).

## Assumption Reassessment (2026-06-24)

1. `games/briar_circuit/src/visibility.rs::effect_envelopes` currently builds the public arm as an `EffectEnvelope { visibility: VisibilityScope::Public, payload }` literal; `EffectEnvelope::public(payload)` exists as an inherent constructor in `crates/engine-core/src/lib.rs`. Confirmed during `/reassess-spec`.
2. Spec §3.4 classifies this as `migrate` / ADR-0009 `unchanged`; register entry MSC-8C-001 owns the constructor; the before-baseline (effect bytes/hash) comes from `-001`.
3. Cross-artifact: the effect-envelope contract lives in `engine-core`; the pass-private arm (`private_effect`) is a separate surface migrated in `-003`, not this ticket.
4. §11 acceptance invariant motivates this ticket: public effect bytes, effect ordering, and `VisibilityScope::Public` classification MUST be byte-identical before/after (no-leak + determinism).
5. Enforcement surface = effect serialization/hash and the Briar replay/visibility traces plus WASM logged JSON; the constructor swap produces identical bytes (ADR-0009 `unchanged`), leaking no hidden information and breaking no deterministic replay/hash.

## Architecture Check

1. Using the kernel inherent constructor over a hand-rolled literal removes duplicated envelope-shape knowledge from the game and routes it through the single owned API.
2. No backwards-compatibility aliasing or shim is introduced; the literal is replaced, not wrapped.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4); the trick helper is untouched.

## Verification Layers

1. Public effect bytes/hash unchanged -> deterministic replay-hash check (`replay-check --game briar_circuit --all`).
2. Public visibility + filtered viewer results preserved -> focused public-visibility assertion in the Briar visibility/effect tests.
3. Constructor adopted -> codebase grep-proof (`EffectEnvelope::public` present in `effect_envelopes`; `VisibilityScope::Public` literal gone from the public arm).

## What to Change

### 1. Adopt `EffectEnvelope::public`

In `effect_envelopes`, replace the public `EffectEnvelope { visibility: VisibilityScope::Public, payload }` literal arm with `EffectEnvelope::public(payload)`. Preserve payload construction and effect ordering exactly. The private arm is untouched (owned by `-003`).

## Files to Touch

- `games/briar_circuit/src/visibility.rs` (modify)

## Out of Scope

- The owner-private `private_effect` migration (`-003`).
- Any C-07 hidden-information matrix work or reveal-timing change.
- Changing effect payload content, ordering, or any hash; other games' C-01 adoption.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including a focused before/after public-effect equality assertion.
2. `cargo run -p replay-check -- --game briar_circuit --all` passes with effect/public-view hashes byte-identical to the `-001` baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The public effect envelope's bytes, ordering, and `VisibilityScope::Public` classification are unchanged.
2. No new public symbol or shim is introduced in the game crate.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/visibility.rs` (or the effect test module) — add/strengthen a focused assertion that `effect_envelopes` yields a `Public`-visibility envelope with the baseline payload bytes.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The per-game replay-check is the correct boundary: this surface is game-local effect serialization.

## Outcome

Completed: 2026-06-24

What changed:
- Replaced the Briar Circuit public `EffectEnvelope { visibility:
  VisibilityScope::Public, payload }` literal in
  `games/briar_circuit/src/visibility.rs::effect_envelopes` with
  `EffectEnvelope::public(payload)`.
- Strengthened the focused visibility test to assert the public arm yields
  exactly one `VisibilityScope::Public` envelope with the unchanged
  `CardPlayed` payload before checking observer filtering.

Deviations:
- None. The owner-private `private_effect` surface remains untouched for
  `8CR4NSEAPRITRI-003`.

Verification:
- `cargo fmt --all --check`
- `cargo test -p briar_circuit`
- `cargo run -p replay-check -- --game briar_circuit --all`
- `bash scripts/boundary-check.sh`
