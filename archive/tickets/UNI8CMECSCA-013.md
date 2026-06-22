# UNI8CMECSCA-013: Implement `ActionTreeEncodingVersion::V1` over `StableBytesWriter`

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/engine-core/src/action.rs`
**Deps**: UNI8CMECSCA-012

## Problem

`ActionTree` is a kernel contract, but per-game encoders are drifting and ambiguous (UNI8CMECSCA-004). This ticket adds `ActionTreeEncodingVersion::V1`, a canonical encoding over `StableBytesWriter` (UNI8CMECSCA-012) covering exactly the current action-tree contract — recursively, in existing vector order — plus version-explicit `stable_bytes(version)` / `stable_hash(version)` persistence methods. No unversioned "current" persisted hash is allowed, and no hypothetical field is invented. It migrates no game (the pilots are UNI8CMECSCA-014/015).

## Assumption Reassessment (2026-06-22)

1. The contract lives in `crates/engine-core/src/action.rs`: `ActionTree { root: ActionNode, freshness_token: FreshnessToken }`; `ActionNode { choices: Vec<ActionChoice> }`; `ActionChoice { segment, label, accessibility_label, metadata: Vec<ActionMetadata>, tags: Vec<String>, preview: ActionPreview, next: Option<Box<ActionNode>> }`; `ActionMetadata { key, value }`; `ActionPreview { Unavailable, Available }` (payload-free) — all confirmed by grep at the reassessed commit. `StableBytesWriter` exists after UNI8CMECSCA-012.
2. Spec §4.3 C-04 fixes the V1 field coverage: encoding/domain version, freshness token, ordered root + child choices, choice segment, label, accessibility label, metadata entries (key+value framed independently) in existing vector order, tags in existing vector order, the current `ActionPreview` discriminant (no payload), and explicit `next = none/some` with recursively framed child. The current contract has **no** disabled-state/reason fields — V1 must not invent them. Register entry `MSC-8C-004` homes this in `engine-core`.
3. Cross-artifact boundary under audit: the action-tree contract (`crates/engine-core/src/action.rs`, `docs/ARCHITECTURE.md` action section) and the new byte writer. V1 is a new persisted surface, not a replacement of any existing per-game hash.
4. FOUNDATIONS §11 determinism: encoding preserves vector order, frames children recursively, and includes explicit version/domain — identical trees produce identical bytes; no incidental ordering enters.
5. Deterministic replay/hash surface under audit (§11/§13): only version-explicit methods are exposed (`stable_bytes(version)`, `stable_hash(version)`); no unversioned current hash. `HashValue::from_stable_bytes` is unchanged. A future contract field requires a later encoding version or an explicit out-of-hash rule (no silent extension).

## Architecture Check

1. A kernel-owned versioned encoder over the explicit writer replaces drifting per-game encoders with one auditable surface that covers only contract fields and order — not legal-choice meaning.
2. No backwards-compatibility shim — V1 is a new named surface; the legacy per-game encoders remain until their pilots migrate (UNI8CMECSCA-014/015).
3. `engine-core` encodes only the existing `ActionTree` contract; no mechanic noun or choice semantics enters (`bash scripts/boundary-check.sh`).

## Verification Layers

1. Field coverage + vector-order preservation for empty, flat, multi-choice, metadata/tag, preview, and recursive trees → engine-core unit tests with pinned bytes.
2. Child framing + freshness inclusion + explicit version/domain present in bytes → byte-assertion tests.
3. No hypothetical field encoded → test asserting the byte layout has no disabled-state/reason slot.
4. Only version-explicit persistence exists → grep-proof: no unversioned `stable_hash()`/`stable_bytes()` on `ActionTree`.

## What to Change

### 1. `crates/engine-core/src/action.rs` — V1 encoder

Add `ActionTreeEncodingVersion::V1` and `impl ActionTree { stable_bytes(version) -> Vec<u8>; stable_hash(version) -> HashValue; }` over `StableBytesWriter`, encoding the exact contract fields above in existing vector order.

### 2. Kernel tests

Empty / flat / multi-choice / metadata+tag / preview / recursive trees with pinned v1 bytes and hashes.

## Files to Touch

- `crates/engine-core/src/action.rs` (modify)

## Out of Scope

- Migrating any game's action-tree hash (UNI8CMECSCA-014/015).
- Inventing disabled-state/reason or any field not in the current contract.
- An unversioned "current" persisted hash.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p engine-core` passes, including empty/flat/multi-choice/metadata-tag/preview/recursive coverage.
2. A test pins v1 bytes for a recursive tree and proves child framing + freshness + version/domain inclusion.
3. `bash scripts/boundary-check.sh` and `cargo build --workspace` pass.

### Invariants

1. V1 covers every current contract field and recursive child in deterministic order; no nonexistent field is encoded.
2. Only `stable_bytes(version)`/`stable_hash(version)` exist — no unversioned persisted hash.

## Test Plan

### New/Modified Tests

1. `crates/engine-core/src/action.rs` (inline `#[cfg(test)]`) — per-shape v1 byte/hash vectors (empty → recursive).

### Commands

1. `cargo test -p engine-core`
2. `bash scripts/boundary-check.sh`
3. The engine-core suite is the correct boundary — game pilots adopt v1 in UNI8CMECSCA-014/015.

## Outcome

Completed: 2026-06-22

What changed:
- Added `ActionTreeEncodingVersion::V1` and version-explicit
  `ActionTree::stable_bytes(version)` / `ActionTree::stable_hash(version)` over
  `StableBytesWriter` with domain `action_tree` and surface version `1`.
- Encoded the current action-tree contract only: freshness token, root/child
  choices in vector order, choice segment/label/accessibility label, metadata
  key/value records in vector order, tags in vector order, payload-free preview
  discriminants, and explicit `next` none/some child framing.
- Re-exported `ActionTreeEncodingVersion` from `engine-core`.
- Added engine-core V1 byte/hash coverage for empty, flat, metadata/tag,
  preview, recursive, child-framing, vector-order, and non-contract-field
  absence cases.
- Flipped `MSC-8C-004` in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` from
  `candidate` to `accepted`.

Deviations:
- The ticket listed only `crates/engine-core/src/action.rs`; updating
  `crates/engine-core/src/lib.rs` was required to expose the version enum to
  follow-on game pilots.
- No game hash, fixture, replay output, or unversioned action-tree persisted
  hash method was added.

Verification:
- `cargo fmt --all --check`
- `cargo test -p engine-core`
- `bash scripts/boundary-check.sh`
- `cargo build --workspace`
- `rg -n "pub fn stable_(bytes|hash)\\(" crates/engine-core/src/action.rs`
  returned only the two version-explicit methods:
  `stable_bytes(&self, version: ActionTreeEncodingVersion)` and
  `stable_hash(&self, version: ActionTreeEncodingVersion)`.
