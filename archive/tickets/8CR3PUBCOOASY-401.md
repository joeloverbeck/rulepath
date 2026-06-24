# 8CR3PUBCOOASY-401: C-04/C-05 Plain Tricks action-tree v1 parallel surface

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/plain_tricks/src/replay_support.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`plain_tricks` has an ad hoc local action-tree hash
(`replay_support.rs::action_tree_hash`). C-04/C-05 add the shipped canonical
action-tree v1 bytes/hash (`ActionTree::stable_bytes(V1)` /
`stable_hash(V1)` via `StableBytesWriter`) as a **parallel new surface**,
retaining the local hash as an exception. No existing hash, trace, state,
effect, view, or export byte changes; no legal choice/label/metadata/branch
order changes.

## Assumption Reassessment (2026-06-24)

1. `games/plain_tricks/src/replay_support.rs::action_tree_hash` exists (local
   surface). Shipped `ActionTree::stable_bytes`/`stable_hash` +
   `ActionTreeEncodingVersion::V1` are at `crates/engine-core/src/action.rs:37`/
   `:45`/`:61`; `StableBytesWriter` at `crates/engine-core/src/replay.rs:125`.
2. Spec §3.7 verdict for Plain C-04/C-05 is `migrate` (parallel-new-surface);
   §5.6 task `8C-R3-401` scopes adding v1 bytes/hash and retaining the local
   hash. Representative trees: opening trick, forced follow-suit, void/free
   discard, final play, terminal empty tree.
3. Cross-crate boundary under audit: `engine-core` action-tree v1 encoding vs
   the game-local legal-tree builder — the game keeps producing the same
   `ActionTree`; v1 bytes/hash are computed *from* it in parallel.
4. FOUNDATIONS §11 (determinism) + §13 (replay/hash semantics): the v1 surface
   is additive and versioned; the existing local hash and every adjacent
   byte/hash surface are explicit exceptions, so this is not a hash-semantics
   migration of an existing surface (no ADR-0009 intentional-migration packet).
5. Enforcement surface: the new v1 byte/hash vectors for the named trees, plus
   the unchanged local action-tree hash, state/effect/view/replay/export bytes;
   all existing surfaces byte-identical to the 001 baseline.

## Architecture Check

1. Adding v1 as a parallel surface (not replacing the local hash) preserves
   every existing consumer while introducing the canonical versioned encoding;
   cleaner and lower-risk than a silent hash swap.
2. No backwards-compatibility alias — the local hash is *retained as a named
   exception*, not aliased; the v1 surface is genuinely new.
3. `engine-core` already owns the action-tree v1 contract; no mechanic noun
   added, no `game-stdlib` change.

## Verification Layers

1. v1 byte/hash determinism -> new vectors for the five representative trees in
   `tests/replay.rs` (stable across runs; hash = `from_stable_bytes(bytes)`).
2. No adjacent drift -> `replay-check --game plain_tricks --all` +
   serialization tests byte-identical to baseline (local hash, state, effect,
   view, export unchanged).
3. No legal-tree drift -> legal choices/labels/metadata/branch order unchanged
   (assertion in the v1 vector tests).

## What to Change

### 1. Add the parallel v1 action-tree surface

In `games/plain_tricks/src/replay_support.rs`, compute `tree.stable_bytes(V1)`
and `tree.stable_hash(V1)` for the legal action tree as a named parallel surface
alongside the retained local `action_tree_hash`. Add representative v1 vectors
for the five named trees.

## Files to Touch

- `games/plain_tricks/src/replay_support.rs` (modify)
- `games/plain_tricks/tests/replay.rs` (modify — add v1 vectors)

## Out of Scope

- Replacing or removing the local `action_tree_hash` (retained exception).
- Any state/effect/view/replay/export/diagnostic byte change.
- Changing legal choices, labels, accessibility labels, metadata, or branch order.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` (new v1 vector tests + existing suites).
2. `cargo run -p replay-check -- --game plain_tricks --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game plain_tricks`.

### Invariants

1. The local action-tree hash and all adjacent bytes are unchanged from baseline.
2. The v1 surface is versioned, deterministic, and computed from the unchanged
   legal tree.

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/replay.rs` — v1 byte/hash vectors for opening
   trick, forced follow-suit, void/free discard, final play, terminal empty tree.

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all`
3. A per-game test + replay-check is the correct boundary: the v1 surface is
   game-local and additive; adjacency is asserted unchanged.

## Outcome

Completed: 2026-06-24

Added parallel `ActionTreeEncodingVersion::V1` helpers in
`games/plain_tricks/src/replay_support.rs` and representative v1 vectors in
`games/plain_tricks/tests/replay.rs` for opening trick, forced follow-suit,
void/free discard, final play, and terminal empty tree. The existing local
`action_tree_hash` is retained and asserted unchanged. The change is additive;
legal choices, labels, metadata, branch order, state/effect/view hashes,
replay/export bytes, fixtures, and golden traces were otherwise untouched.

Deviations: the test pins v1 byte length plus v1 hash derived from the actual
bytes rather than embedding the full byte hex for each vector; this keeps the
large opening vector maintainable while still detecting byte-surface drift.

Verification:

- `cargo test -p plain_tricks` passed.
- `cargo run -p replay-check -- --game plain_tricks --all` passed.
- `cargo run -p fixture-check -- --game plain_tricks` passed.
- No golden trace, fixture, export, state/effect/view hash, or local
  action-tree hash surface changed.
