# UNI8CR2TWOSEA-019: Poker Lite — parallel action-tree v1 bytes/hash

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/poker_lite/src/replay_support.rs`; adds a parallel `ActionTreeEncodingVersion::V1` byte/hash surface alongside the retained legacy `action_tree_hash`
**Deps**: 001

## Problem

Spec §3.7 / task `8C-R2-403`: Poker Lite has a local `replay_support::action_tree_hash` string encoder. R2 retains that legacy hash as an `exception` and adds a parallel v1 bytes/hash surface with vectors across the pledge phases, comparing semantics and ordering. No pledge legality change.

## Assumption Reassessment (2026-06-23)

1. `games/poker_lite/src/replay_support.rs::action_tree_hash` exists (confirmed line ~347) as a string encoder over `engine_core::ActionTree`.
2. Spec §3.7/§9: retain the legacy hash as `exception`; add a `parallel-new-surface`; do not reinterpret legacy traces; no pledge legality change.
3. Cross-crate boundary under audit: `engine-core::ActionTreeEncodingVersion::V1` (`action.rs:61`) + `StableBytesWriter` — the generic versioned byte writer; legality stays game-local.
4. Determinism: the legacy `action_tree_hash` value stays byte-identical to the `-001` baseline; the new v1 bytes are deterministic across the pledge phases (§11).
5. Schema extension: additive-only — a parallel `ActionTreeEncodingVersion::V1` adapter whose consumers are this game's own replay tests; the legacy encoder remains authoritative through its window (ADR-0009 `parallel-new-surface`; legacy `exception`).

## Architecture Check

1. Keeping the legacy hash readable while adding a version-explicit v1 surface preserves compatibility and gives canonical evidence — cleaner than mutating the legacy encoder.
2. No backwards-compat alias; the v1 adapter is additive and independently removable, legacy untouched.
3. `engine-core` stays noun-free; no `game-stdlib` change.

## Verification Layers

1. Legacy `action_tree_hash` unchanged; v1 bytes deterministic across pledge phases -> deterministic replay-hash check (`cargo test -p poker_lite`, `replay-check --game poker_lite --all`).
2. No pledge legality change -> `cargo test -p poker_lite` (rule tests).
3. `ActionTreeEncodingVersion::V1` adoption + legacy retention -> codebase grep-proof in `replay_support.rs`.

## What to Change

### 1. Add a parallel v1 action-tree byte/hash adapter

In `replay_support.rs`, add a version-pinned v1 bytes/hash adapter over the pledge-phase trees with `StableBytesWriter`; keep `action_tree_hash` intact.

### 2. Add v1 evidence tests

Add v1 byte/hash vectors across pledge phases to `tests/replay.rs` and a legacy-hash-equality guard, without altering existing assertions.

## Files to Touch

- `games/poker_lite/src/replay_support.rs` (modify)
- `games/poker_lite/tests/replay.rs` (modify)

## Out of Scope

- Any pledge/showdown/yield legality change; any legacy-hash byte change.
- Any existing golden-trace or fixture byte change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` green, with v1 vectors and the legacy-hash-equality guard.
2. `cargo run -p replay-check -- --game poker_lite --all` — legacy hash + existing traces byte-identical to baseline.

### Invariants

1. The legacy `action_tree_hash` value is byte-identical to baseline.
2. The v1 surface is additive and version-explicit; pledge legality is unchanged.

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/replay.rs` — v1 byte/hash vectors across pledge phases + legacy-hash equality guard.

### Commands

1. `cargo test -p poker_lite`
2. `cargo run -p replay-check -- --game poker_lite --all`
