# 8CR1PUBFIXSEA-004: Column Four C-01 public-envelope constructor adoption

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/column_four` (`src/effects.rs`); effect bytes and visibility byte-identical
**Deps**: 8CR1PUBFIXSEA-001

## Problem

`games/column_four/src/effects.rs::public_effect` constructs the public effect envelope with a `VisibilityScope::Public` struct literal instead of the shipped kernel constructor `EffectEnvelope::public` (MSC-8C-001). Adopt the constructor with zero change to payload, effect order, or visibility — an ADR-0009 `unchanged` adoption.

## Assumption Reassessment (2026-06-23)

1. `games/column_four/src/effects.rs::public_effect` currently builds `EffectEnvelope { visibility: VisibilityScope::Public, payload }` as a literal; `EffectEnvelope::public(payload)` exists as an inherent constructor in `crates/engine-core/src/lib.rs`. Confirmed during reassessment.
2. Spec §3.3 and §5.3 classify this as `migrate` / ADR-0009 `unchanged`; register entry MSC-8C-001 owns the constructor.
3. Cross-artifact: the effect-envelope contract lives in `engine-core`; the before-baseline (effect bytes/hash) comes from `-001`.
4. §11 acceptance invariant motivates this ticket: effect bytes and public visibility MUST be byte-identical before/after (no-leak + determinism).
5. Enforcement surface = effect serialization/hash and the Column Four replay traces; the constructor swap produces identical bytes (ADR-0009 `unchanged`), leaking no hidden information and breaking no deterministic replay/hash.

## Architecture Check

1. Using the kernel inherent constructor over a hand-rolled literal removes duplicated envelope-shape knowledge from the game and routes it through the single owned API.
2. No backwards-compatibility aliasing or shim is introduced; the literal is replaced, not wrapped.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Effect bytes/hash unchanged -> deterministic replay-hash check (`replay-check --game column_four --all`).
2. Public visibility preserved -> focused public-visibility assertion in the Column Four effect/replay tests.
3. Constructor adopted -> codebase grep-proof (`EffectEnvelope::public` present, `VisibilityScope::Public` literal gone from `public_effect`).

## What to Change

### 1. Adopt `EffectEnvelope::public`

In `public_effect`, replace the `EffectEnvelope { visibility: VisibilityScope::Public, payload }` literal with `EffectEnvelope::public(payload)`. Preserve payload construction and effect ordering exactly.

## Files to Touch

- `games/column_four/src/effects.rs` (modify)

## Out of Scope

- Any private-envelope or C-07 hidden-information work.
- Other games' C-01 adoption.
- Changing effect payload content, ordering, or any hash.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p column_four` is green, including a focused before/after public-effect equality assertion.
2. `cargo run -p replay-check -- --game column_four --all` passes with effect/public-view hashes byte-identical to the `-001` baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The effect envelope's bytes and `VisibilityScope::Public` classification are unchanged.
2. No new public symbol or shim is introduced in the game crate.

## Test Plan

### New/Modified Tests

1. `games/column_four/` effect/replay test module — add (or strengthen) a focused assertion that `public_effect` yields a `Public`-visibility envelope with the baseline payload bytes.

### Commands

1. `cargo test -p column_four`
2. `cargo run -p replay-check -- --game column_four --all`
3. The per-game replay-check is the correct boundary: this surface is game-local effect serialization.

## Outcome

Completed: 2026-06-23

Implemented the C-01 Column Four public-envelope constructor adoption by
replacing the local `EffectEnvelope { visibility: VisibilityScope::Public,
payload }` literal in `games/column_four/src/effects.rs::public_effect` with
`EffectEnvelope::public(payload)`. Added a focused unit test proving public
visibility and payload preservation.

Deviations from the original plan:

- None.

Verification:

- `cargo fmt --all -- --check` passed.
- `cargo test -p column_four` passed.
- `cargo run -p replay-check -- --game column_four --all` passed.
- `bash scripts/boundary-check.sh` passed.
- `rg -n "EffectEnvelope::public|EffectEnvelope \\{|VisibilityScope::Public" games/column_four/src/effects.rs` confirmed the public helper now calls `EffectEnvelope::public`, with `VisibilityScope::Public` only in tests.
