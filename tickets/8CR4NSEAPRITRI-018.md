# 8CR4NSEAPRITRI-018: Vow Tide C-04/05 parallel action-tree v1 bytes/hash

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/vow_tide` (`src/replay_support.rs` new parallel-v1 function; `src/actions.rs` read-only); legacy Debug-derived hashes + view/export bytes unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

Vow's `actions::legal_action_tree` has compound bid and play branches, but replay hashing is `Debug`-text-derived via `replay_support::snapshot` with no explicit `ActionTreeEncodingVersion::V1` byte/hash surface (MSC-8C, C-04/C-05). Add a **parallel** v1 bytes/hash surface covering bidding, dealer-hook exclusion, play/follow-suit, empty/wrong-actor trees, and 3–7 seats, leaving the legacy `Debug`-derived hashes and all view/export bytes unchanged (spec §3.7 Vow, §5.6).

## Assumption Reassessment (2026-06-24)

1. `games/vow_tide/src/actions.rs::legal_action_tree` (confirmed at src/actions.rs:42) and `games/vow_tide/src/replay_support.rs::snapshot` (confirmed at line 51) exist; `ActionTree::{stable_bytes, stable_hash}` + `ActionTreeEncodingVersion::V1` exist in `engine-core`. Confirmed during `/reassess-spec`.
2. Spec §3.7 classifies the v1 bytes/hash as `migrate` (ADR-0009 `parallel-new`); the legacy `Debug`-derived hashes are an explicit `exception` (existing replay authority).
3. Cross-artifact: the v1 encoder is `engine-core`-owned and called by a Vow adapter; the pre-change legacy Debug hash values come from `-001`. Vow's view/export bytes stay unchanged.
4. §11 acceptance invariant (deterministic replay/hash) motivates this ticket: the v1 surface is additive across 3–7 seats; no legacy Debug hash or view/export byte changes, and a green v1 hash never replaces the legacy authority.
5. Enforcement surface = v1 bytes/hash vs the legacy Debug-derived hashes; the adapter calls the existing engine encoder and adds a parallel function only.

## Architecture Check

1. A game-owned v1 adapter over the existing `engine-core` encoder is cleaner than continuing to hash `Debug` text — it gives an explicit, versioned byte surface while the legacy hash stays authoritative.
2. No backwards-compatibility shim is introduced; the legacy Debug hashes remain. If the v1 API cannot express the Vow compound tree, the task stops (§8.4 trigger 1).
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4).

## Verification Layers

1. v1 bytes/hash deterministic over bid, dealer-hook exclusion, play/follow-suit, empty/wrong-actor trees, 3–7 seats -> golden / deterministic replay-hash check on the new parallel surface.
2. Legacy Debug-derived hashes + view/export bytes unchanged -> deterministic replay-hash check (`replay-check --game vow_tide --all`) against `-001` baseline.
3. Adapter calls the engine encoder -> codebase grep-proof (`ActionTreeEncodingVersion::V1` + `stable_bytes`/`stable_hash` present in the new function; `snapshot` untouched).

## What to Change

### 1. Add the parallel v1 action-tree bytes/hash

Add a clearly-named parallel-v1 function in `games/vow_tide/src/replay_support.rs` that builds v1 bytes/hash via `ActionTree::{stable_bytes, stable_hash}(ActionTreeEncodingVersion::V1)` over `actions::legal_action_tree`. Pin dealer-hook exclusion, bid ordering, card ordering, empty/wrong-actor trees, and representative 3–7 states. Leave `snapshot` and all legacy Debug hashes unchanged.

## Files to Touch

- `games/vow_tide/src/replay_support.rs` (modify)

## Out of Scope

- Replacing or removing the legacy Debug-derived hashes or any view/export byte.
- Any bid/contract/hook/last-bidder, follow-suit, trump, trick-winner/leader, or scoring logic change.
- The C-07 no-leak matrices (`-023`/`-024`) and C-08 profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` is green, including the parallel-v1 byte/hash vectors across bid/play and 3–7 seats.
2. `cargo run -p replay-check -- --game vow_tide --all` passes with the legacy Debug hashes and view/export bytes byte-identical to the `-001` baseline.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The legacy Debug-derived hashes and all view/export bytes are byte-identical to baseline; the v1 surface is additive.
2. No bid/trick/scoring logic is altered.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/` (rules/serialization) — parallel-v1 byte/hash vectors over bid, dealer-hook exclusion, play/follow-suit, empty/wrong-actor trees, 3–7 seats.

### Commands

1. `cargo test -p vow_tide`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The per-game test plus replay-check are the correct boundary: the v1 hash is a game-local surface over the kernel encoder.
