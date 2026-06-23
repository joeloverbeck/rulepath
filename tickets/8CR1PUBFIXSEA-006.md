# 8CR1PUBFIXSEA-006: Token Bazaar C-01 public-envelope constructor adoption

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/token_bazaar` (`src/effects.rs`); effect bytes, visibility, and public-export effect bytes byte-identical
**Deps**: 8CR1PUBFIXSEA-001

## Problem

`games/token_bazaar/src/effects.rs::public_effect` constructs the public effect envelope with a `VisibilityScope::Public` struct literal instead of the shipped kernel constructor `EffectEnvelope::public` (MSC-8C-001). Adopt the constructor with zero change to payload, effect order, visibility, or the public-export effect bytes — an ADR-0009 `unchanged` adoption.

## Assumption Reassessment (2026-06-23)

1. `games/token_bazaar/src/effects.rs::public_effect` currently builds `EffectEnvelope { visibility: VisibilityScope::Public, payload }` as a literal; `EffectEnvelope::public(payload)` exists as an inherent constructor in `crates/engine-core/src/lib.rs`. Confirmed during reassessment.
2. Spec §3.3 and §5.3 classify this as `migrate` / ADR-0009 `unchanged`; register entry MSC-8C-001 owns the constructor.
3. Cross-artifact: the effect-envelope contract lives in `engine-core`; Token Bazaar additionally exposes a `PublicReplayExport` surface (`src/replay_support.rs`) whose effect bytes must stay identical; before-baseline comes from `-001`.
4. §11 acceptance invariant motivates this ticket: effect bytes, public visibility, AND public-export effect bytes MUST be byte-identical before/after (no-leak + determinism).
5. Enforcement surface = effect serialization/hash, the Token Bazaar replay traces, and the public-export bytes; the constructor swap produces identical bytes (ADR-0009 `unchanged`), leaking no hidden information and breaking no deterministic replay/hash.

## Architecture Check

1. Using the kernel inherent constructor over a hand-rolled literal removes duplicated envelope-shape knowledge from the game and routes it through the single owned API.
2. No backwards-compatibility aliasing or shim is introduced; the literal is replaced, not wrapped.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Effect bytes/hash unchanged -> deterministic replay-hash check (`replay-check --game token_bazaar --all`).
2. Public visibility + public-export effect bytes preserved -> no-leak visibility test + the existing public-export round-trip assertion.
3. Constructor adopted -> codebase grep-proof (`EffectEnvelope::public` present, `VisibilityScope::Public` literal gone from `public_effect`).

## What to Change

### 1. Adopt `EffectEnvelope::public`

In `public_effect`, replace the `EffectEnvelope { visibility: VisibilityScope::Public, payload }` literal with `EffectEnvelope::public(payload)`. Preserve payload construction and effect ordering exactly.

## Files to Touch

- `games/token_bazaar/src/effects.rs` (modify)

## Out of Scope

- Any private-envelope or C-07 hidden-information work.
- Other games' C-01 adoption.
- Any change to `PublicReplayExport` format or export bytes (owned by the C-02 output and public-export-profile tickets).
- Changing effect payload content, ordering, or any hash.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` is green, including a focused before/after public-effect equality assertion.
2. `cargo run -p replay-check -- --game token_bazaar --all` passes with effect/public-view and public-export hashes byte-identical to the `-001` baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The effect envelope's bytes, `VisibilityScope::Public` classification, and public-export effect bytes are unchanged.
2. No new public symbol or shim is introduced in the game crate.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/` effect/replay test module — add (or strengthen) a focused assertion that `public_effect` yields a `Public`-visibility envelope with the baseline payload bytes; the existing public-export round-trip stays byte-identical.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo run -p replay-check -- --game token_bazaar --all`
3. The per-game replay-check plus the existing export round-trip is the correct boundary: this surface is game-local effect serialization and its public-export projection.
