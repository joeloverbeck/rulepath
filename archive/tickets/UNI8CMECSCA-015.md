# UNI8CMECSCA-015: Draughts Lite compound-tree action-encoding pilot

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/draughts_lite/src/replay_support.rs`
**Deps**: UNI8CMECSCA-014

## Problem

Race's flat tree (UNI8CMECSCA-014) cannot exercise recursion or delimiter-collision inside nested choices. This ticket pilots `ActionTreeEncodingVersion::V1` against Draughts Lite's compound multi-jump action tree, proving nested-path framing and ambiguous-string resistance. Existing multi-jump behavior and path legality are unchanged; the legacy hash/checkpoints either still pass or receive one explicit per-surface ADR-0009 migration; the recursive v1 vector is pinned. No movement/capture/promotion semantics enter shared code.

## Assumption Reassessment (2026-06-22)

1. `games/draughts_lite/src/replay_support.rs` carries the compound action-tree byte/hash logic and produces `tests/golden_traces/multi-jump.trace.json`; `ActionTree::stable_hash(V1)` exists after UNI8CMECSCA-013 and the parallel-surface pattern is established by UNI8CMECSCA-014. The UNI8CMECSCA-003 packet pins Draughts' legacy compound-tree bytes/hash.
2. Spec §5 8C-015 review boundary: existing multi-jump behavior/path legality unchanged; legacy hash/checkpoints still pass or receive one explicit per-surface migration; recursive v1 vector pinned; no movement/capture semantics in shared code. The ADR-0009 migration protocol (UNI8CMECSCA-014 AR item 2) applies identically.
3. Cross-artifact boundary under audit: Draughts' `replay_support.rs` and `multi-jump.trace.json`, plus the kernel v1 encoder. Default classification is parallel-new-surface.
4. FOUNDATIONS §11 (EC-09/EC-10/EC-11): recursive coverage and vector order are preserved; the legacy compound hash is unchanged unless a named migration is committed; the delimiter-collision case the flat tree could not reach is exercised here.
5. Deterministic replay/hash surface under audit (§11/§13): the recursive v1 byte vector is pinned; movement/capture/promotion stay in `games/draughts_lite` (no shared-code leakage); `HashValue::from_stable_bytes` unchanged; governed by accepted ADR 0009 (no new §13 ADR).

## Architecture Check

1. Piloting the recursive case proves v1 framing handles nested boundaries that the flat tree cannot, completing the action-encoding evidence (flat + compound).
2. No backwards-compatibility shim — legacy and v1 coexist as named surfaces.
3. `engine-core` untouched; no capture/movement/promotion semantics enters shared code (`bash scripts/boundary-check.sh`).

## Verification Layers

1. Existing multi-jump behavior and path legality unchanged → `cargo test -p draughts_lite`.
2. Legacy compound trace readable and passing → `cargo run -p replay-check -- --game draughts_lite --all`.
3. Recursive v1 byte vector pinned + delimiter-collision negative (EV-TREE-COMPOUND) → comparison test.
4. If a checkpoint migrates: before/after + note + version + validators + rollback committed → migration-receipt asserts.

## What to Change

### 1. `games/draughts_lite/src/replay_support.rs`

Add the v1 hash as a parallel named surface for the compound tree; pin the recursive v1 vector and exercise a nested delimiter-collision case. Default parallel-new-surface unless a packet names a migration.

### 2. Tests

Legacy-vs-v1 compound-tree bytes/hash and the nested ambiguity negative.

## Files to Touch

- `games/draughts_lite/src/replay_support.rs` (modify)

## Out of Scope

- Migrating any other game's action tree.
- Moving movement/capture/promotion logic into shared code.
- Blanket golden regeneration; silent legacy-hash change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` passes, including the recursive v1 vector and nested-collision negative.
2. `cargo run -p replay-check -- --game draughts_lite --all` passes (legacy trace unchanged).
3. `cargo test --workspace` passes.

### Invariants

1. Multi-jump path legality and the legacy compound hash are unchanged unless a named per-surface migration is committed.
2. No capture/movement/promotion semantics appears in shared code.

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/src/replay_support.rs` (inline `#[cfg(test)]`) or `tests/` — legacy-vs-v1 compound byte/hash + nested-collision negative (EV-TREE-COMPOUND).

### Commands

1. `cargo run -p replay-check -- --game draughts_lite --all`
2. `cargo test -p draughts_lite`
3. `replay-check` plus the comparison test are the correct boundary — the recursive case must not perturb the committed compound trace.

## Outcome

Completed: 2026-06-22

What changed:
- Added Draughts Lite helper functions for named legacy bytes and the parallel
  V1 action-tree surface: `action_tree_legacy_bytes`,
  `action_tree_v1_bytes`, and `action_tree_v1_hash`.
- Added a compound multi-jump comparison test that computes legacy and V1 from
  the same recursive legal action tree, pins the legacy hash, pins the V1 bytes
  through the `StableBytesWriter` field contract, and pins the V1 hash
  `390128801164796593`.
- Extended the existing nested-boundary ambiguity characterization to prove the
  legacy collision no longer collides under V1 bytes or V1 hash.

Deviations:
- The ticket listed only `games/draughts_lite/src/replay_support.rs`; the
  comparison and nested-collision assertions were added in
  `games/draughts_lite/tests/replay.rs`, where the existing compound-tree
  characterization tests live.
- No golden trace, fixture hash, legacy hash value, path legality, movement,
  capture, or promotion behavior was changed.

Verification:
- `cargo fmt --all --check`
- `cargo run -p replay-check -- --game draughts_lite --all`
- `cargo test -p draughts_lite`
- `git diff --quiet -- games/draughts_lite/tests/golden_traces`
- `cargo test --workspace`
