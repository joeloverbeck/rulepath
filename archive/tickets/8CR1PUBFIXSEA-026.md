# 8CR1PUBFIXSEA-026: Directional Flip C-04/C-05 parallel action-tree v1 surface

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/directional_flip` (`src/replay_support.rs`, `tests/replay.rs`); legacy hash retained
**Deps**: 8CR1PUBFIXSEA-001

## Problem

Directional Flip has no parallel action-tree v1 byte/hash surface. Add `action_tree_v1_bytes` / `action_tree_v1_hash` to `games/directional_flip/src/replay_support.rs` that delegate to `engine-core::ActionTree::stable_bytes/stable_hash(ActionTreeEncodingVersion::V1)`, proving the engine v1 encoder covers segment, label, accessibility label, metadata order, tag order, preview, child structure, and freshness framing — an ADR-0009 `parallel-new` surface. The legacy `action_tree_hash`, `ReplayHashes.action_tree_hash`, and all committed trace bytes stay untouched.

## Assumption Reassessment (2026-06-23)

1. `games/directional_flip/src/replay_support.rs` defines `action_tree_hash` and `ReplayHashes.action_tree_hash` but NOT the v1 wrappers; `ActionTree::stable_bytes` + `ActionTreeEncodingVersion::V1` exist in `crates/engine-core/src/action.rs` and already encode metadata/tags/preview per the engine contract. Confirmed during reassessment.
2. Spec §3.6 and §5.7 (task `8C-R1-403`) classify this as `parallel-new` + retained legacy compatibility exception; MSC-8C-004/005 own the v1 encoding/writer. Legacy hash stays authoritative (no flip in R1).
3. Cross-artifact: the v1 encoder is an `engine-core` contract (`ActionTree`); the new wrappers are thin game-local delegations. Legacy sentinel from `-001`/spec §7.3 (`directional_flip` `opening-legal-move.trace.json` → `16457061400249558986`).
4. §11/§13 motivate this ticket: a parallel versioned hash must not change replay/hash semantics of existing traces; the legacy field stays authoritative, so no §13 ADR trigger fires.
5. Enforcement surface = the action-tree stable-byte/hash encoding; the v1 wrappers delegate to the kernel writer (no hand-rolled framing), pin segment/label/accessibility/metadata/tag/preview/child/freshness coverage in a receipt, and leave every committed trace byte-identical.

## Architecture Check

1. A thin delegation to the kernel v1 encoder is cleaner and safer than a game-local reimplementation of the stable-byte framing, especially for the metadata/tag/preview-rich action tree.
2. No backwards-compatibility shim is introduced; the legacy hash is retained as an explicit compatibility exception.
3. `engine-core` stays noun-free (§3); the v1 encoder already lives in the kernel; no `game-stdlib` change (§4).

## Verification Layers

1. Legacy `action_tree_hash` byte-identical -> pinned receipt + `replay-check --game directional_flip --all`.
2. New v1 bytes/hash cover segment/label/accessibility/metadata-order/tag-order/preview/child/freshness -> focused serialization unit test (`tests/replay.rs`).
3. v1 delegates to the engine API; legacy-vs-v1 relationship pinned -> focused assertion against the kernel encoder output.

## What to Change

### 1. Add parallel v1 wrappers

In `games/directional_flip/src/replay_support.rs`, add `action_tree_v1_bytes` / `action_tree_v1_hash` delegating to `ActionTree::stable_bytes/stable_hash(ActionTreeEncodingVersion::V1)`. Do not modify `action_tree_hash` or `ReplayHashes`.

### 2. Pin the receipt

In `games/directional_flip/tests/replay.rs`, pin the legacy sentinel, the v1 bytes/hash, and explicit coverage of segment, label, accessibility label, metadata order, tag order, preview, child structure, and freshness framing.

## Files to Touch

- `games/directional_flip/src/replay_support.rs` (modify)
- `games/directional_flip/tests/replay.rs` (modify)

## Out of Scope

- Replacing `ReplayHashes.action_tree_hash` with v1, or making v1 authoritative in existing traces.
- Any committed trace byte change.
- Hand-duplicating `StableBytesWriter` framing in the game.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p directional_flip` is green, including the v1 byte/hash receipt with metadata/tag/preview coverage.
2. `cargo run -p replay-check -- --game directional_flip --all` passes; legacy action-tree hash and all committed traces are byte-identical.
3. The v1 wrappers call `ActionTree::stable_bytes/stable_hash(V1)` (grep-proof; no hand-rolled framing).

### Invariants

1. The legacy `action_tree_hash` and `ReplayHashes.action_tree_hash` are byte-identical and remain authoritative.
2. The v1 surface is additive and delegates entirely to the kernel encoder, including metadata/tag/preview/freshness.

## Test Plan

### New/Modified Tests

1. `games/directional_flip/tests/replay.rs` — pinned legacy sentinel + v1 bytes/hash + segment/label/accessibility/metadata/tag/preview/child/freshness coverage.

### Commands

1. `cargo test -p directional_flip`
2. `cargo run -p replay-check -- --game directional_flip --all`
3. The per-game serialization test plus replay-check are the correct boundary.

## Outcome

Completed on 2026-06-23.

Added Directional Flip's parallel v1 action-tree helpers in
`games/directional_flip/src/replay_support.rs`; both wrappers delegate directly
to `ActionTree::stable_bytes/stable_hash(ActionTreeEncodingVersion::V1)`.
Legacy `action_tree_hash`, `ReplayHashes.action_tree_hash`, and committed trace
hashes remain authoritative and unchanged.

Pinned a focused receipt in `games/directional_flip/tests/replay.rs` for the
opening tree:

- legacy action-tree hash: `17097613169116532881`
- v1 action-tree hash: `15334878763169959513`
- v1 byte length: `2092`
- v1 domain/header coverage: `RPSB` + `action_tree`
- segment order: `place/r3c4` < `place/r4c3` < `place/r5c6` < `place/r6c5`
- label/accessibility, metadata order, and tag order are covered by byte-order
  assertions
- preview, child structure, and freshness framing are covered by v1 hash
  mutation-sensitivity assertions while the wrapper output is pinned equal to
  the kernel encoder output

Verification:

1. `cargo test -p directional_flip action_tree_legacy_and_v1_surfaces_are_pinned_in_parallel -- --exact`
2. `cargo test -p directional_flip`
3. `cargo run -p replay-check -- --game directional_flip --all`
4. `rg -n "stable_bytes\\(ActionTreeEncodingVersion::V1\\)|stable_hash\\(ActionTreeEncodingVersion::V1\\)|fn action_tree_v1" games/directional_flip/src/replay_support.rs`
5. `cargo fmt --all -- --check`
