# 8CR1PUBFIXSEA-027: Token Bazaar C-04/C-05 parallel action-tree v1 surface

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/token_bazaar` (`src/replay_support.rs`, `tests/serialization.rs`); legacy and public-export hashes retained
**Deps**: 8CR1PUBFIXSEA-001

## Problem

Token Bazaar has no parallel action-tree v1 byte/hash surface. Add `action_tree_v1_bytes` / `action_tree_v1_hash` to `games/token_bazaar/src/replay_support.rs` that delegate to `engine-core::ActionTree::stable_bytes/stable_hash(ActionTreeEncodingVersion::V1)`, proving action-segment and metadata ordering in v1 — an ADR-0009 `parallel-new` surface. The legacy `action_tree_hash`, `ReplayHashes.action_tree_hash`, committed trace bytes, and public-export hashes stay untouched.

## Assumption Reassessment (2026-06-23)

1. `games/token_bazaar/src/replay_support.rs` defines `action_tree_hash` and `ReplayHashes.action_tree_hash` (and the `PublicReplayExport` surface) but NOT the v1 wrappers; `ActionTree::stable_bytes` + `ActionTreeEncodingVersion::V1` exist in `crates/engine-core/src/action.rs`. Confirmed during reassessment.
2. Spec §3.6 and §5.7 (task `8C-R1-404`) classify this as `parallel-new` + retained legacy compatibility exception; MSC-8C-004/005 own the v1 encoding/writer. Legacy and public-export hashes stay authoritative (no flip in R1).
3. Cross-artifact: the v1 encoder is an `engine-core` contract (`ActionTree`); the new wrappers are thin game-local delegations alongside the existing `PublicReplayExport`. Legacy sentinel from `-001`/spec §7.3 (`token_bazaar` `shortest-normal.trace.json` → `6002416109879922099`).
4. §11/§13 motivate this ticket: a parallel versioned hash must not change replay/hash semantics of existing traces or public-export bytes; the legacy fields stay authoritative, so no §13 ADR trigger fires.
5. Enforcement surface = the action-tree stable-byte/hash encoding and the public-export hash authority; the v1 wrappers delegate to the kernel writer (no hand-rolled framing), pin action-segment/metadata order in a receipt, and leave every committed trace and export byte-identical.

## Architecture Check

1. A thin delegation to the kernel v1 encoder is cleaner and safer than a game-local reimplementation of the stable-byte framing.
2. No backwards-compatibility shim is introduced; the legacy hash is retained as an explicit compatibility exception.
3. `engine-core` stays noun-free (§3); the v1 encoder already lives in the kernel; no `game-stdlib` change (§4).

## Verification Layers

1. Legacy `action_tree_hash` and public-export hashes byte-identical -> pinned receipt + `replay-check --game token_bazaar --all`.
2. New v1 bytes/hash cover action-segment and metadata ordering -> focused serialization unit test (`tests/serialization.rs`).
3. v1 delegates to the engine API; legacy-vs-v1 relationship pinned -> focused assertion against the kernel encoder output.

## What to Change

### 1. Add parallel v1 wrappers

In `games/token_bazaar/src/replay_support.rs`, add `action_tree_v1_bytes` / `action_tree_v1_hash` delegating to `ActionTree::stable_bytes/stable_hash(ActionTreeEncodingVersion::V1)`. Do not modify `action_tree_hash`, `ReplayHashes`, or `PublicReplayExport`.

### 2. Pin the receipt

In `games/token_bazaar/tests/serialization.rs`, pin the legacy sentinel, the v1 bytes/hash, and the action-segment + metadata ordering, while preserving legacy and public-export hashes.

## Files to Touch

- `games/token_bazaar/src/replay_support.rs` (modify)
- `games/token_bazaar/tests/serialization.rs` (modify)

## Out of Scope

- Replacing `ReplayHashes.action_tree_hash` with v1, or making v1 authoritative in existing traces.
- Any committed trace or public-export byte change.
- Hand-duplicating `StableBytesWriter` framing in the game.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` is green, including the v1 byte/hash receipt with action-segment/metadata-order coverage.
2. `cargo run -p replay-check -- --game token_bazaar --all` passes; legacy action-tree hash, committed traces, and public-export hashes are byte-identical.
3. The v1 wrappers call `ActionTree::stable_bytes/stable_hash(V1)` (grep-proof; no hand-rolled framing).

### Invariants

1. The legacy `action_tree_hash`, `ReplayHashes.action_tree_hash`, and public-export hashes are byte-identical and remain authoritative.
2. The v1 surface is additive and delegates entirely to the kernel encoder.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/tests/serialization.rs` — pinned legacy sentinel + v1 bytes/hash + action-segment/metadata-order assertions.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo run -p replay-check -- --game token_bazaar --all`
3. The per-game serialization test plus replay-check are the correct boundary.
