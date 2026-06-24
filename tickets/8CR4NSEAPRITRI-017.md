# 8CR4NSEAPRITRI-017: Briar Circuit C-05 parallel action-tree v1 bytes/hash

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/briar_circuit` (`src/replay_support.rs` new parallel-v1 fields/function); legacy preview hashes + browser JSON unchanged
**Deps**: 8CR4NSEAPRITRI-016

## Problem

With the game-owned typed action-tree adapter in place (`-016`), add a **parallel** `ActionTreeEncodingVersion::V1` bytes/hash surface for observer and seat viewers, beside the existing preview `Debug` hashes in `replay_support::{action_hash, replay_hash_snapshot}` (MSC-8C, C-05). The existing preview hashes and browser JSON remain authoritative; the new v1 fields are additive (spec §3.7 Briar, §5.6).

## Assumption Reassessment (2026-06-24)

1. `games/briar_circuit/src/replay_support.rs::{action_hash, replay_hash_snapshot}` exist (confirmed at lines 189 / 149 during `/reassess-spec`); the typed `legal_action_tree` adapter from `-016` is the source for v1 bytes; `ActionTree::{stable_bytes, stable_hash}` + `ActionTreeEncodingVersion::V1` exist in `engine-core`.
2. Spec §3.7 classifies the v1 bytes/hash as `migrate` (ADR-0009 `parallel-new`); this ticket `Deps` `-016` (parity must be proven before the hash surface is added).
3. Cross-artifact: the v1 encoder is `engine-core`-owned; the existing preview `Debug` hashes are an explicit `exception` (existing replay authority). Baseline preview hashes come from `-001`.
4. §11 acceptance invariant (deterministic replay/hash + no-leak) motivates this ticket: the v1 surface is additive over observer + each seat viewer, leaks no hidden pass/hand data, and changes no existing preview hash or browser byte.
5. Enforcement surface = v1 bytes/hash vs the legacy preview `Debug` hashes; adding parallel fields/functions changes no existing snapshot schema in place where compatibility would break, and a green v1 hash never replaces the legacy authority.

## Architecture Check

1. Adding the v1 surface over the typed adapter is cleaner than hashing the WASM JSON — it hashes the Rust-owned tree through the single owned encoder while preview hashes stay intact.
2. No backwards-compatibility shim is introduced; the existing snapshot schema is not changed in place if compatibility would break (new fields/functions only). If the v1 API cannot express the Briar tree, the task stops (§8.4 trigger 1).
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4).

## Verification Layers

1. v1 bytes/hash deterministic over pass select/unselect/confirm + legal play, for observer and seat viewers -> golden / deterministic replay-hash check on the new parallel surface.
2. Existing preview `Debug` hashes + browser JSON unchanged -> deterministic replay-hash check (`replay-check --game briar_circuit --all`) against `-001` baseline.
3. No hidden pass/hand leak in v1 bytes -> no-leak visibility test over observer/non-owner viewers of the v1 surface.

## What to Change

### 1. Add the parallel v1 bytes/hash surface

Add new parallel-v1 fields/function in `games/briar_circuit/src/replay_support.rs` that build v1 bytes/hash via `ActionTree::{stable_bytes, stable_hash}(ActionTreeEncodingVersion::V1)` over the `-016` adapter, for observer and each seat viewer. Pin pass select/unselect/confirm and legal play paths. Leave `action_hash`/`replay_hash_snapshot` unchanged.

## Files to Touch

- `games/briar_circuit/src/replay_support.rs` (modify)

## Out of Scope

- Replacing the legacy preview `Debug` hashes or the browser JSON.
- Changing the existing snapshot schema in place where compatibility would break.
- Any pass/trick/moon/scoring policy change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including the new parallel-v1 byte/hash + per-viewer no-leak assertions.
2. `cargo run -p replay-check -- --game briar_circuit --all` passes with preview `Debug` hashes byte-identical to the `-001` baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The existing preview hashes and browser JSON are byte-identical to baseline; the v1 surface is additive.
2. No hidden pass/hand datum appears in the v1 bytes for observer or non-owner viewers.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/` (replay/serialization) — parallel-v1 byte/hash vectors + per-viewer no-leak assertions for pass and play paths.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The per-game test plus replay-check are the correct boundary: the v1 hash is a game-local surface over the kernel encoder.
