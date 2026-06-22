# UNI8CMECSCA-014: Race flat-tree action-encoding pilot (parallel new surface)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/race_to_n/src/replay_support.rs`
**Deps**: UNI8CMECSCA-013, UNI8CMECSCA-003

## Problem

Prove `ActionTreeEncodingVersion::V1` (UNI8CMECSCA-013) against Race to N's flat action tree by adding the v1 hash as a **parallel named surface** first: compute both the legacy local action-tree hash and the new v1 hash from the same tree and pin both. The legacy trace stays readable and passing. A fixture/checkpoint migrates only if its evidence packet names the new hash-surface version and compatibility behavior — following the ADR-0009 migration protocol, never a silent flip.

## Assumption Reassessment (2026-06-22)

1. `games/race_to_n/src/replay_support.rs` carries Race's local action-tree byte/hash logic and produces the `shortest-normal.trace.json` golden (hyphen seats); `ActionTree::stable_hash(V1)` exists after UNI8CMECSCA-013. The UNI8CMECSCA-003 packet pins Race's legacy flat-tree bytes/hash.
2. Spec §5 8C-014 + the "ADR-0009 migration protocol" fix the order: name the legacy surface, commit characterization first, add v1 and compute old/new in parallel, classify (unchanged / parallel-new-surface / intentional-migration), keep legacy read-only through the C-11 window, run validators, merge with a rollback point. A "regenerate all expected hashes" approach fails review.
3. Cross-artifact boundary under audit: Race's `replay_support.rs` and `tests/golden_traces/shortest-normal.trace.json`, plus the kernel v1 encoder. Default classification is **parallel-new-surface** (legacy authoritative for old artifacts; v1 added for new/opted-in evidence).
4. FOUNDATIONS §11 (EC-10/EC-11): existing hashes are unchanged unless a per-surface migration note, version, compatibility window, validators, and rollback are present; legacy traces are not retroactively reinterpreted by v1.
5. Deterministic replay/hash surface under audit (§11/§13): v1 bytes/hash are pinned; if a single fixture migrates, before/after bytes, old/new hash, update note, validator result, and rollback point are committed together. `HashValue::from_stable_bytes` unchanged; no §13 ADR (governed by accepted ADR 0009).

## Architecture Check

1. Parallel-surface adoption proves v1 against a real flat tree without disturbing the committed legacy hash — the staged, named migration the §7.5 Git SHA-256 prior art recommends.
2. No backwards-compatibility shim — legacy and v1 coexist as named surfaces; neither aliases the other.
3. `engine-core` untouched (encoder already landed); no choice semantics moves into shared code.

## Verification Layers

1. Legacy Race trace remains readable and passing → `cargo run -p replay-check -- --game race_to_n --all`.
2. New v1 bytes/hash pinned for the flat tree → `cargo test -p race_to_n` (parallel-surface test).
3. Legacy vs v1 computed from the same tree (EV-TREE-FLAT) → comparison test.
4. If a fixture migrates: before/after + note + version + validators + rollback committed → migration-receipt test/asserts.

## What to Change

### 1. `games/race_to_n/src/replay_support.rs`

Add the v1 hash as a parallel named surface alongside the legacy hash; expose both for the flat tree. Default to **parallel-new-surface** (no golden edit) unless a packet names a migration.

### 2. Tests

Pin legacy and v1 bytes/hash for the same flat tree; assert legacy trace unchanged.

## Files to Touch

- `games/race_to_n/src/replay_support.rs` (modify)

## Out of Scope

- Migrating Draughts (UNI8CMECSCA-015) or any non-flat tree.
- Any blanket golden regeneration; flipping the legacy hash without a named migration packet.
- Changing seat output (UNI8CMECSCA-009 governs).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game race_to_n --all` passes (legacy trace unchanged).
2. `cargo test -p race_to_n` passes, pinning both legacy and v1 flat-tree bytes/hash.
3. `cargo test --workspace` passes.

### Invariants

1. The legacy Race action-tree hash is unchanged unless a named per-surface migration (note + version + validators + rollback) is committed.
2. v1 is a distinct named surface; no golden is bulk-regenerated.

## Test Plan

### New/Modified Tests

1. `games/race_to_n/src/replay_support.rs` (inline `#[cfg(test)]`) or `tests/` — legacy-vs-v1 flat-tree byte/hash comparison (EV-TREE-FLAT).

### Commands

1. `cargo run -p replay-check -- --game race_to_n --all`
2. `cargo test -p race_to_n`
3. `replay-check` plus the comparison test are the correct boundary — the parallel surface must not alter the committed legacy trace.
