# UNI8CR2TWOSEA-017: High Card Duel — parallel action-tree v1 bytes/hash

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/high_card_duel/src/replay_support.rs`; adds a parallel `ActionTreeEncodingVersion::V1` byte/hash surface via `StableBytesWriter`
**Deps**: 001

## Problem

Spec §3.7 / task `8C-R2-401`: HCD has `legal_action_tree` but **no** game-owned v1 byte/hash wrapper, and Unit 8C uses a debug snapshot only inside C-07. R2 adds an explicit version-pinned v1 bytes/hash surface (`parallel-new-surface`) for representative commit states. It does not relabel the C-07 snapshot as C-04 and preserves all state/effect/export hashes.

## Assumption Reassessment (2026-06-23)

1. `games/high_card_duel/src/actions.rs::legal_action_tree` exists; `games/high_card_duel/src/replay_support.rs` has `state_hash`/`effect_hash` but **no** local `action_tree_hash` (confirmed in the reassess spot-check) — the v1 surface is genuinely new here.
2. Spec §3.7/§9: `parallel-new-surface`; do not relabel the C-07 snapshot as C-04; no legal-choice/metadata/preview change; no existing golden trace changes.
3. Cross-crate boundary under audit: `engine-core::ActionTreeEncodingVersion::V1` (`crates/engine-core/src/action.rs:61`) and `StableBytesWriter` — the generic versioned byte writer; legality stays game-local.
4. Determinism: the new v1 bytes/hash are computed from the existing legal action tree with no nondeterministic input; all existing state/effect/export hashes stay byte-identical to the `-001` baseline (§11).
5. Schema extension: this is additive-only — a parallel `ActionTreeEncodingVersion::V1` adapter whose only consumers are this game's own replay/serialization tests; no existing replay, fixture, or legacy hash is reinterpreted (ADR-0009 `parallel-new-surface`).

## Architecture Check

1. A version-explicit parallel v1 surface gives HCD canonical action-tree evidence without touching the C-07 debug snapshot or any legacy byte — cleaner than overloading the snapshot.
2. No backwards-compat alias; the v1 adapter is purely additive and independently removable.
3. `engine-core` stays noun-free (it sees only the opaque tree bytes); no `game-stdlib` change.

## Verification Layers

1. v1 bytes/hash deterministic for representative commit states -> golden trace / deterministic replay-hash check (`cargo test -p high_card_duel`, `replay-check --game high_card_duel --all`).
2. All state/effect/export hashes + C-07 snapshot unchanged -> deterministic replay-hash check + no-leak visibility test.
3. `ActionTreeEncodingVersion::V1` / `StableBytesWriter` adoption -> codebase grep-proof in `replay_support.rs`.

## What to Change

### 1. Add a parallel v1 action-tree byte/hash adapter

In `replay_support.rs`, add a version-pinned `ActionTreeEncodingVersion::V1` bytes/hash adapter over the existing legal action tree for representative commit states, with `StableBytesWriter`.

### 2. Add v1 evidence tests

Add v1 byte/hash vectors to `tests/replay.rs` / `tests/serialization.rs` without altering existing assertions.

## Files to Touch

- `games/high_card_duel/src/replay_support.rs` (modify)
- `games/high_card_duel/tests/replay.rs` (modify)
- `games/high_card_duel/tests/serialization.rs` (modify)

## Out of Scope

- The C-07 debug snapshot (verified in `-024`), legality, metadata, or preview.
- Any existing golden-trace or fixture byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel` green, with the new v1 byte/hash vectors.
2. `cargo run -p replay-check -- --game high_card_duel --all` — existing hashes/traces byte-identical to baseline.

### Invariants

1. The v1 surface is additive and version-explicit; no legacy/state/effect/export byte changes.
2. Legal choices are unchanged.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/replay.rs` — v1 action-tree byte/hash vectors for representative commit states.

### Commands

1. `cargo test -p high_card_duel`
2. `cargo run -p replay-check -- --game high_card_duel --all`
