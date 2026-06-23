# UNI8CR2TWOSEA-018: Secret Draft — parallel action-tree v1 bytes/hash

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/secret_draft/src/replay_support.rs`; adds a parallel `ActionTreeEncodingVersion::V1` byte/hash surface alongside the retained legacy `action_tree_hash`
**Deps**: 001

## Problem

Spec §3.7 / task `8C-R2-402`: Secret Draft has a local `replay_support::action_tree_hash` string encoder. R2 retains that legacy hash as an `exception` and adds a parallel v1 bytes/hash surface for the first-commit and pending-second-commit trees, comparing semantics and ordering. No reveal-policy change.

## Assumption Reassessment (2026-06-23)

1. `games/secret_draft/src/replay_support.rs::action_tree_hash` exists (confirmed line ~366) as a string encoder over `engine_core::ActionTree`.
2. Spec §3.7/§9: retain the legacy hash as `exception` (readable through its compatibility window); add a `parallel-new-surface`; do not reinterpret legacy traces; no reveal-policy change.
3. Cross-crate boundary under audit: `engine-core::ActionTreeEncodingVersion::V1` (`action.rs:61`) + `StableBytesWriter` — the generic versioned byte writer; legality and reveal stay game-local.
4. Determinism: the legacy `action_tree_hash` value stays byte-identical to the `-001` baseline; the new v1 bytes are deterministic over first-commit and pending-second-commit trees (§11).
5. Schema extension: additive-only — a parallel `ActionTreeEncodingVersion::V1` adapter whose consumers are this game's own replay tests; the legacy encoder remains authoritative through its window (ADR-0009 `parallel-new-surface`; legacy `exception`).

## Architecture Check

1. Keeping the legacy hash readable while adding a version-explicit v1 surface preserves compatibility and gives canonical evidence — cleaner than mutating the legacy encoder.
2. No backwards-compat alias; the v1 adapter is additive and independently removable, legacy untouched.
3. `engine-core` stays noun-free; no `game-stdlib` change.

## Verification Layers

1. Legacy `action_tree_hash` unchanged; v1 bytes deterministic for first-commit + pending-second-commit -> deterministic replay-hash check (`cargo test -p secret_draft`, `replay-check --game secret_draft --all`).
2. No reveal-policy / legal-choice change -> no-leak visibility test (`tests/visibility.rs`).
3. `ActionTreeEncodingVersion::V1` adoption + legacy retention -> codebase grep-proof in `replay_support.rs`.

## What to Change

### 1. Add a parallel v1 action-tree byte/hash adapter

In `replay_support.rs`, add a version-pinned v1 bytes/hash adapter over the first-commit and pending-second-commit trees with `StableBytesWriter`; keep `action_tree_hash` intact.

### 2. Add v1 evidence tests

Add v1 byte/hash vectors to `tests/replay.rs` and a legacy-hash-equality guard, without altering existing assertions.

## Files to Touch

- `games/secret_draft/src/replay_support.rs` (modify)
- `games/secret_draft/tests/replay.rs` (modify)

## Out of Scope

- Any reveal/commitment policy change; any legacy-hash byte change.
- Any existing golden-trace or fixture byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft` green, with v1 vectors and the legacy-hash-equality guard.
2. `cargo run -p replay-check -- --game secret_draft --all` — legacy hash + existing traces byte-identical to baseline.

### Invariants

1. The legacy `action_tree_hash` value is byte-identical to baseline.
2. The v1 surface is additive and version-explicit; legal choices and reveal timing are unchanged.

## Test Plan

### New/Modified Tests

1. `games/secret_draft/tests/replay.rs` — v1 byte/hash vectors for first-commit and pending-second-commit trees + legacy-hash equality guard.

### Commands

1. `cargo test -p secret_draft`
2. `cargo run -p replay-check -- --game secret_draft --all`
