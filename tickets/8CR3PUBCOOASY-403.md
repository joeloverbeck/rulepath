# 8CR3PUBCOOASY-403: C-04/C-05 Frontier Control action-tree v1 parallel surface

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/frontier_control/src/visibility.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`frontier_control` has an ad hoc local action-tree hash
(`visibility.rs::action_tree_hash`). C-04/C-05 add the shipped canonical
action-tree v1 bytes/hash as a **parallel new surface**, retaining the local
hash as an exception. Graph legality is unchanged; no existing hash, trace,
state, effect, view, or export byte changes; no legal choice/label/metadata/
branch order changes.

## Assumption Reassessment (2026-06-24)

1. `games/frontier_control/src/visibility.rs::action_tree_hash` exists at line
   ~164 (re-exported in `lib.rs`). Shipped `ActionTree::stable_bytes`/
   `stable_hash` + `ActionTreeEncodingVersion::V1` at
   `crates/engine-core/src/action.rs`; `StableBytesWriter` at
   `crates/engine-core/src/replay.rs:125`.
2. Spec §3.7 verdict for Frontier C-04/C-05 is `migrate` (parallel-new-surface);
   §5.6 task `8C-R3-403` scopes adding the v1 surface and retaining the local
   hash. Representative trees: muster/reinforce, move, clash branch,
   stake/dismantle, early end, terminal.
3. Cross-crate boundary under audit: `engine-core` action-tree v1 encoding vs
   the game-local legal-tree builder — the game keeps producing the same
   `ActionTree`; v1 bytes/hash are computed from it in parallel.
4. FOUNDATIONS §11 (determinism) + §13: additive, versioned surface; local hash
   and adjacent surfaces are explicit exceptions (no intentional-migration
   packet).
5. Enforcement surface: new v1 byte/hash vectors for the named trees, plus the
   unchanged local hash and state/effect/view/replay/export bytes; all existing
   surfaces byte-identical to the 001 baseline.

## Architecture Check

1. Adding v1 as a parallel surface preserves every existing consumer; lower-risk
   than a silent swap.
2. No backwards-compatibility alias — the local hash is retained as a named
   exception.
3. `engine-core` already owns the action-tree v1 contract; no mechanic noun
   added, no `game-stdlib` change.

## Verification Layers

1. v1 byte/hash determinism -> new vectors for the named trees in
   `tests/replay.rs`.
2. No adjacent drift -> `replay-check --game frontier_control --all` +
   serialization tests byte-identical to baseline.
3. No legal-tree drift -> graph legality, labels, metadata, branch order
   unchanged.

## What to Change

### 1. Add the parallel v1 action-tree surface

In `games/frontier_control/src/visibility.rs`, compute `tree.stable_bytes(V1)`
and `tree.stable_hash(V1)` for the legal action tree as a named parallel surface
alongside the retained local `action_tree_hash`. Add representative v1 vectors.

## Files to Touch

- `games/frontier_control/src/visibility.rs` (modify)
- `games/frontier_control/tests/replay.rs` (modify — add v1 vectors)

## Out of Scope

- Replacing or removing the local `action_tree_hash` (retained exception).
- Any state/effect/view/replay/export/diagnostic byte change.
- Changing graph legality, labels, metadata, or branch order.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` (new v1 vector tests + existing suites).
2. `cargo run -p replay-check -- --game frontier_control --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game frontier_control`.

### Invariants

1. The local action-tree hash and all adjacent bytes are unchanged from baseline.
2. The v1 surface is versioned, deterministic, and computed from the unchanged
   legal tree.

## Test Plan

### New/Modified Tests

1. `games/frontier_control/tests/replay.rs` — v1 byte/hash vectors for
   muster/reinforce, move, clash branch, stake/dismantle, early end, terminal.

### Commands

1. `cargo test -p frontier_control`
2. `cargo run -p replay-check -- --game frontier_control --all`
3. A per-game test + replay-check is the correct boundary: the v1 surface is
   game-local and additive; adjacency is asserted unchanged.
