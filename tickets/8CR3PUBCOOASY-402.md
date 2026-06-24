# 8CR3PUBCOOASY-402: C-04/C-05 Flood Watch action-tree v1 parallel surface

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/flood_watch/src/visibility.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`flood_watch` has an ad hoc local action-tree hash
(`visibility.rs::action_tree_hash`). C-04/C-05 add the shipped canonical
action-tree v1 bytes/hash as a **parallel new surface**, retaining the local
debug-derived hash as an exception. No existing hash, trace, state, effect,
view, or export byte changes; no legal choice/label/metadata/branch order
changes.

## Assumption Reassessment (2026-06-24)

1. `games/flood_watch/src/visibility.rs::action_tree_hash` exists at line ~193
   (re-exported in `lib.rs`). Shipped `ActionTree::stable_bytes`/`stable_hash` +
   `ActionTreeEncodingVersion::V1` at `crates/engine-core/src/action.rs`;
   `StableBytesWriter` at `crates/engine-core/src/replay.rs:125`.
2. Spec §3.7 verdict for Flood C-04/C-05 is `migrate` (parallel-new-surface);
   §5.6 task `8C-R3-402` scopes adding the v1 surface and retaining the local
   debug hash. Representative trees: bail, place levee, role power, early end,
   budget exhausted/automatic environment, terminal.
3. Cross-crate boundary under audit: `engine-core` action-tree v1 encoding vs
   the game-local legal-tree builder — the game keeps producing the same
   `ActionTree`; v1 bytes/hash are computed from it in parallel.
4. FOUNDATIONS §11 (determinism) + §13: the v1 surface is additive and
   versioned; the existing local hash and adjacent surfaces are explicit
   exceptions (no intentional-migration packet).
5. Enforcement surface: the new v1 byte/hash vectors for the named trees, plus
   the unchanged local hash, state/effect/view/replay/export bytes; all existing
   surfaces byte-identical to the 001 baseline.

## Architecture Check

1. Adding v1 as a parallel surface preserves every existing consumer while
   introducing the canonical versioned encoding; lower-risk than a silent swap.
2. No backwards-compatibility alias — the local hash is retained as a named
   exception.
3. `engine-core` already owns the action-tree v1 contract; no mechanic noun
   added, no `game-stdlib` change.

## Verification Layers

1. v1 byte/hash determinism -> new vectors for the named trees in
   `tests/replay.rs` (stable across runs).
2. No adjacent drift -> `replay-check --game flood_watch --all` + serialization
   tests byte-identical to baseline.
3. No legal-tree drift -> legal choices/labels/metadata/branch order unchanged.

## What to Change

### 1. Add the parallel v1 action-tree surface

In `games/flood_watch/src/visibility.rs`, compute `tree.stable_bytes(V1)` and
`tree.stable_hash(V1)` for the legal action tree as a named parallel surface
alongside the retained local `action_tree_hash`. Add representative v1 vectors
for the named trees.

## Files to Touch

- `games/flood_watch/src/visibility.rs` (modify)
- `games/flood_watch/tests/replay.rs` (modify — add v1 vectors)

## Out of Scope

- Replacing or removing the local `action_tree_hash` (retained exception).
- Any state/effect/view/replay/export/diagnostic byte change.
- Changing legal choices, labels, metadata, or branch order.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p flood_watch` (new v1 vector tests + existing suites).
2. `cargo run -p replay-check -- --game flood_watch --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game flood_watch`.

### Invariants

1. The local action-tree hash and all adjacent bytes are unchanged from baseline.
2. The v1 surface is versioned, deterministic, and computed from the unchanged
   legal tree.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/replay.rs` — v1 byte/hash vectors for bail, place
   levee, role power, early end, budget exhausted/automatic environment, terminal.

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p replay-check -- --game flood_watch --all`
3. A per-game test + replay-check is the correct boundary: the v1 surface is
   game-local and additive; adjacency is asserted unchanged.
