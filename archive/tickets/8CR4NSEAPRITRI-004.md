# 8CR4NSEAPRITRI-004: Vow Tide C-01 public-envelope constructor adoption (WASM)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `crates/wasm-api` (`src/games/vow.rs`); effect count/order/payload and logged JSON byte-identical
**Deps**: 8CR4NSEAPRITRI-001

## Problem

`crates/wasm-api/src/games/vow.rs::vow_apply_command` maps Vow public payloads through a hand-built `EffectEnvelope { visibility: VisibilityScope::Public, payload }` literal instead of the shipped kernel constructor `EffectEnvelope::public` (MSC-8C-001). Adopt the constructor with zero change to effect count, order, payload, or logged JSON — an ADR-0009 `unchanged` adoption (spec §3.4 Vow public, §5.3). No private Vow effect class exists or is invented (spec §3.4 Vow private = N/A).

## Assumption Reassessment (2026-06-24)

1. `crates/wasm-api/src/games/vow.rs::vow_apply_command` currently constructs the public envelope as an `EffectEnvelope { visibility: VisibilityScope::Public, payload }` literal; `EffectEnvelope::public(payload)` exists in `crates/engine-core/src/lib.rs`. Confirmed during `/reassess-spec`.
2. Spec §3.4 classifies Vow public C-01 as `migrate` / ADR-0009 `unchanged` and Vow private C-01 as `not-applicable` — Vow's effects are public game events and `visibility::filter_effects_for_viewer` returns public payloads; register entry MSC-8C-001 owns the constructor.
3. Cross-artifact: the migrated surface lives in the WASM bridge crate, not the game crate; the before-baseline (effect count/order/payload, logged JSON, replay-step output) comes from `-001`.
4. §11 acceptance invariant motivates this ticket: Vow effect ordering and rendered/logged bytes MUST be byte-identical before/after.
5. Enforcement surface = WASM effect serialization + logged JSON and the `wasm-api` Vow tests; the constructor swap produces identical bytes (ADR-0009 `unchanged`), leaking no hidden information and breaking no replay-step output.

## Architecture Check

1. Routing public construction through the kernel constructor removes a duplicated envelope literal from the WASM bridge and matches the constructor used by other games.
2. No backwards-compatibility aliasing or shim is introduced; the literal map is replaced, not wrapped. No private Vow effect class is added.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4); TypeScript decides no legality and repairs no envelope (§2).

## Verification Layers

1. Effect count/order/payload + logged JSON unchanged -> deterministic replay-hash check (`replay-check --game vow_tide --all`) plus the `wasm-api` Vow effect test.
2. Public visibility preserved (no private effect introduced) -> schema/serialization validation against the effect-envelope contract.
3. Constructor adopted -> codebase grep-proof (`EffectEnvelope::public` present in `vow_apply_command`; `VisibilityScope::Public` literal gone from it).

## What to Change

### 1. Adopt `EffectEnvelope::public`

In `vow_apply_command`, replace the literal public-envelope map with `EffectEnvelope::public(payload)`. Preserve payload construction, effect count, and ordering exactly. Do not introduce a private effect variant.

## Files to Touch

- `crates/wasm-api/src/games/vow.rs` (modify)

## Out of Scope

- Inventing any Vow private effect class merely because Vow has private views (spec §3.4 N/A; trigger is a future game-owned seat-private Vow effect).
- Vow C-02 seat work (`-008`…`-010`) and any view/export byte change.
- Changing effect payload content, ordering, or any hash; other games' C-01 adoption.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` is green, including a focused before/after Vow public-effect equality assertion.
2. `cargo run -p replay-check -- --game vow_tide --all` passes with effect/replay-step bytes byte-identical to the `-001` baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Vow effect count, order, payload, and logged JSON are unchanged; no private effect class exists.
2. No new public symbol or shim is introduced in the WASM bridge.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api` Vow effect/command test module — add/strengthen a focused assertion that `vow_apply_command` yields `Public`-visibility envelopes with baseline payload bytes and ordering.

### Commands

1. `cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The `wasm-api` test plus per-game replay-check are the correct boundary: this surface is the WASM bridge's effect mapping for Vow.

## Outcome

Completed: 2026-06-24

What changed:
- Replaced the Vow Tide WASM bridge public envelope literal in
  `crates/wasm-api/src/games/vow.rs::vow_apply_command` with
  `EffectEnvelope::public`.
- Added a focused Vow bridge test that applies a valid opening bid, confirms
  every returned envelope is `VisibilityScope::Public`, and checks the logged
  JSON still reports public `bid_accepted` effects.

Deviations:
- None to implementation scope. No private Vow effect class was introduced.

Verification:
- `cargo fmt --all --check`
- `cargo test -p wasm-api`
- `cargo run -p replay-check -- --game vow_tide --all`
- `bash scripts/boundary-check.sh`
