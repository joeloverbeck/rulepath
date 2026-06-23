# UNI8CR2TWOSEA-020: Masked Claims — parallel action-tree v1 bytes/hash (claim + response trees)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/masked_claims/src/replay_support.rs`; adds a parallel `ActionTreeEncodingVersion::V1` byte/hash surface over the compound claim and flat response trees
**Deps**: 001

## Problem

Spec §3.7 / task `8C-R2-404`: Masked Claims has a compound claim tree and a response tree but no game-owned canonical tree hash. R2 adds a `parallel-new-surface` v1 bytes/hash covering both the compound claim and flat response shapes, without changing legality or pending-responder policy.

## Assumption Reassessment (2026-06-23)

1. `games/masked_claims/src/actions.rs::legal_action_tree` exists (confirmed line ~72) and produces the compound claim / response trees; `replay_support.rs` exists with no game-owned canonical tree hash.
2. Spec §3.7/§9: `parallel-new-surface` over both claim and response shapes; response authorization stays game-local; no legality or pending-responder policy change.
3. Cross-crate boundary under audit: `engine-core::ActionTreeEncodingVersion::V1` (`action.rs:61`) + `StableBytesWriter` — the generic versioned byte writer; legality/reaction stay game-local.
4. Determinism: the new v1 bytes are deterministic over the compound claim and pending-response trees with no nondeterministic input; all existing state/effect/export hashes stay byte-identical to the `-001` baseline (§11).
5. Schema extension: additive-only — a parallel `ActionTreeEncodingVersion::V1` adapter whose consumers are this game's own replay tests; no existing trace is reinterpreted (ADR-0009 `parallel-new-surface`).

## Architecture Check

1. One version-explicit v1 adapter spanning both claim and response shapes gives canonical action-tree evidence without touching legality or reaction policy — cleaner than two ad-hoc encoders.
2. No backwards-compat alias; the v1 adapter is additive and independently removable.
3. `engine-core` stays noun-free; no `game-stdlib` change.

## Verification Layers

1. v1 bytes/hash deterministic for compound claim + pending-response trees -> deterministic replay-hash check (`cargo test -p masked_claims`, `replay-check --game masked_claims --all`).
2. No legality / pending-responder change -> `cargo test -p masked_claims` (rule tests).
3. `ActionTreeEncodingVersion::V1` adoption -> codebase grep-proof in `replay_support.rs`.

## What to Change

### 1. Add a parallel v1 action-tree byte/hash adapter

In `replay_support.rs`, add a version-pinned v1 bytes/hash adapter over the compound claim and flat response trees with `StableBytesWriter`.

### 2. Add v1 evidence tests

Add v1 byte/hash vectors for both shapes to `tests/replay.rs`, without altering existing assertions.

## Files to Touch

- `games/masked_claims/src/replay_support.rs` (modify)
- `games/masked_claims/tests/replay.rs` (modify)

## Out of Scope

- Any legality, reaction-window, or pending-responder policy change.
- Any existing golden-trace or fixture byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` green, with the new v1 byte/hash vectors for both tree shapes.
2. `cargo run -p replay-check -- --game masked_claims --all` — existing hashes/traces byte-identical to baseline.

### Invariants

1. The v1 surface is additive and version-explicit; no legality or pending-responder change.
2. No existing state/effect/export byte changes.

## Test Plan

### New/Modified Tests

1. `games/masked_claims/tests/replay.rs` — v1 byte/hash vectors for the compound claim and flat response trees.

### Commands

1. `cargo test -p masked_claims`
2. `cargo run -p replay-check -- --game masked_claims --all`
