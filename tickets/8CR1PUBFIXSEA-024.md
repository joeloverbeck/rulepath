# 8CR1PUBFIXSEA-024: Three Marks C-04/C-05 parallel action-tree v1 surface

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/three_marks` (`src/replay_support.rs`, `tests/serialization_tests.rs`); legacy hash retained
**Deps**: 8CR1PUBFIXSEA-001

## Problem

Three Marks has no parallel action-tree v1 byte/hash surface (Race and Draughts already ship theirs). Add `action_tree_v1_bytes` / `action_tree_v1_hash` to `games/three_marks/src/replay_support.rs` that delegate to `engine-core::ActionTree::stable_bytes/stable_hash(ActionTreeEncodingVersion::V1)`, with a focused pinned receipt — an ADR-0009 `parallel-new` surface. The legacy `action_tree_hash`, `ReplayHashes.action_tree_hash`, and all committed trace bytes stay untouched. No RPSB framing is reimplemented in the game.

## Assumption Reassessment (2026-06-23)

1. `games/three_marks/src/replay_support.rs` defines `action_tree_hash` and `ReplayHashes.action_tree_hash` but NOT `action_tree_v1_bytes`/`action_tree_v1_hash`; `ActionTree::stable_bytes` + `ActionTreeEncodingVersion::V1` exist in `crates/engine-core/src/action.rs`. Confirmed during reassessment.
2. Spec §3.6 and §5.7 (task `8C-R1-401`) classify this as `parallel-new` + retained legacy compatibility exception; MSC-8C-004/005 own the v1 encoding/writer. The legacy hash remains authoritative for existing trace fields (no flip in R1).
3. Cross-artifact: the v1 encoder is an `engine-core` contract (`ActionTree`); the new wrappers are thin game-local delegations. Legacy sentinel from `-001`/spec §7.3 (`three_marks` `shortest-normal.trace.json` → `14695981039346656037`).
4. §11/§13 motivate this ticket: adding a parallel versioned hash must not change replay/hash semantics of existing traces; the legacy field stays authoritative, so no §13 ADR trigger fires.
5. Enforcement surface = the action-tree stable-byte/hash encoding; the v1 wrappers delegate to the kernel writer (no hand-rolled framing), pin order/framing in a receipt, and leave every committed trace byte-identical — no determinism break, no leak.

## Architecture Check

1. A thin delegation to the kernel v1 encoder is cleaner and safer than a game-local reimplementation of the stable-byte framing (which the spec forbids).
2. No backwards-compatibility shim is introduced; the legacy hash is retained as an explicit compatibility exception, not aliased.
3. `engine-core` stays noun-free (§3); the v1 encoder already lives in the kernel; no `game-stdlib` change (§4).

## Verification Layers

1. Legacy `action_tree_hash` byte-identical -> pinned receipt + `replay-check --game three_marks --all`.
2. New v1 bytes/hash pinned and delegate to the engine API -> focused serialization unit test (`tests/serialization_tests.rs`).
3. v1 order/framing and legacy-vs-v1 inequality where expected -> focused assertion against the kernel encoder output.

## What to Change

### 1. Add parallel v1 wrappers

In `games/three_marks/src/replay_support.rs`, add `action_tree_v1_bytes` / `action_tree_v1_hash` delegating to `ActionTree::stable_bytes/stable_hash(ActionTreeEncodingVersion::V1)`. Do not modify `action_tree_hash` or `ReplayHashes`.

### 2. Pin the receipt

In `games/three_marks/tests/serialization_tests.rs`, pin the representative legacy hash sentinel, the new v1 bytes/hash, ordering, and the expected legacy-vs-v1 relationship.

## Files to Touch

- `games/three_marks/src/replay_support.rs` (modify)
- `games/three_marks/tests/serialization_tests.rs` (modify)

## Out of Scope

- Replacing `ReplayHashes.action_tree_hash` with v1, or making v1 authoritative in existing traces.
- Any committed trace byte change.
- Hand-duplicating `StableBytesWriter` framing in the game.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p three_marks` is green, including the new v1 byte/hash pinned receipt.
2. `cargo run -p replay-check -- --game three_marks --all` passes; legacy action-tree hash and all committed traces are byte-identical.
3. The v1 wrappers call `ActionTree::stable_bytes/stable_hash(V1)` (grep-proof; no hand-rolled framing).

### Invariants

1. The legacy `action_tree_hash` and `ReplayHashes.action_tree_hash` are byte-identical and remain authoritative for existing traces.
2. The v1 surface is additive and delegates entirely to the kernel encoder.

## Test Plan

### New/Modified Tests

1. `games/three_marks/tests/serialization_tests.rs` — pinned legacy sentinel + new v1 bytes/hash + order/framing assertions.

### Commands

1. `cargo test -p three_marks`
2. `cargo run -p replay-check -- --game three_marks --all`
3. The per-game serialization test plus replay-check are the correct boundary: this is a game-local parallel hash surface delegating to a kernel encoder.
