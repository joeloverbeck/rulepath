# 8CR4NSEAPRITRI-014: River Ledger C-04/05 parallel action-tree v1 adapter

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger` (`src/replay_support.rs` new parallel-v1 function; `src/actions.rs` read-only); legacy `action_tree_hash` authority unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

River's legacy `replay_support::action_tree_hash` hashes the `ActionTree` it is handed, but there is no explicit, named `ActionTreeEncodingVersion::V1` byte/hash surface beside it (MSC-8C, C-04/C-05). Add a game-owned adapter that invokes `ActionTree::{stable_bytes, stable_hash}(ActionTreeEncodingVersion::V1)` over the already game-owned `actions::legal_action_tree`, as a **parallel-new** surface with the legacy hash left authoritative and readable (spec §3.7 River, §5.6). No betting/all-in/pot logic changes.

## Assumption Reassessment (2026-06-24)

1. `games/river_ledger/src/actions.rs::legal_action_tree` and `games/river_ledger/src/replay_support.rs::action_tree_hash` exist; `ActionTree::{stable_bytes, stable_hash}` and `ActionTreeEncodingVersion::V1` exist in `crates/engine-core/src/action.rs`. Confirmed during `/reassess-spec`.
2. Spec §3.7 classifies the action-tree v1 surface as `migrate` (ADR-0009 `parallel-new`); register MSC-8C owns the action-tree encoding contract; the legacy hash is an explicit `exception` (existing replay authority).
3. Cross-artifact: the action-tree v1 encoder is an `engine-core` contract called by a game adapter; the pre-change legacy hash vectors come from `-001`. The richer all-in/side-pot vectors are added in `-015` (this ticket is the adapter only).
4. §11 acceptance invariant (deterministic replay/hash) motivates this ticket: the new v1 bytes/hash are additive; no legacy authority field or byte changes, and a green v1 hash is never used as permission to replace the legacy hash (§7.3 unauthorized list).
5. Enforcement surface = action-tree v1 stable bytes/hash vs the legacy `action_tree_hash`; the adapter calls the existing engine encoder and adds a parallel function only, leaking nothing and changing no legacy byte.

## Architecture Check

1. A game-owned adapter over the existing `engine-core` v1 encoder is cleaner than re-deriving an encoding locally — it reuses the single owned contract and keeps the legacy hash intact for compatibility.
2. No backwards-compatibility shim is introduced; the legacy `action_tree_hash` reader/vectors remain. If the v1 API cannot express the River tree without a contract change, the task stops (§8.4 trigger 1), it does not broaden `engine-core`.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Legacy `action_tree_hash` bytes unchanged -> deterministic replay-hash check (`replay-check --game river_ledger --all`) against the `-001` sentinels.
2. New v1 bytes/hash deterministic over fold/check/call/bet/raise + all-in metadata -> golden / deterministic replay-hash check on the new parallel vectors.
3. Adapter calls the engine encoder -> codebase grep-proof (`ActionTreeEncodingVersion::V1` + `stable_bytes`/`stable_hash` present in the new function; legacy function untouched).

## What to Change

### 1. Add the parallel v1 action-tree adapter

Add a clearly-named parallel-v1 function in `games/river_ledger/src/replay_support.rs` that builds the v1 bytes/hash via `ActionTree::{stable_bytes, stable_hash}(ActionTreeEncodingVersion::V1)` over `actions::legal_action_tree`. Characterize fold/check/call/bet/raise ordering plus full/short all-in metadata first. Leave `action_tree_hash` unchanged.

## Files to Touch

- `games/river_ledger/src/replay_support.rs` (modify)

## Out of Scope

- The richer all-in/side-pot expected vectors (`-015`).
- Replacing or removing the legacy `action_tree_hash` or any other legacy state/effect/view/replay/export hash.
- Any betting, all-in, reopen, pot/side-pot, evaluator, showdown, or allocation logic change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` is green, including the new parallel-v1 byte/hash characterization.
2. `cargo run -p replay-check -- --game river_ledger --all` passes with the legacy `action_tree_hash` byte-identical to the `-001` baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The legacy action-tree hash and all other legacy authorities are byte-identical to baseline.
2. The v1 surface is additive (parallel-new); no legacy reader/vector is deleted.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/` (rules or a narrowly added action-tree module) — add parallel-v1 byte/hash assertions over fold/check/call/bet/raise + all-in metadata.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. The per-game test plus replay-check are the correct boundary: action-tree encoding is game-local over a kernel contract.
