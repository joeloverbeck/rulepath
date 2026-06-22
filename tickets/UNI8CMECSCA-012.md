# UNI8CMECSCA-012: Implement `StableBytesWriter` v1 in `engine-core`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/engine-core/src/replay.rs` (or a noun-free sibling re-exported there)
**Deps**: UNI8CMECSCA-003, UNI8CMECSCA-004

## Problem

Rulepath has no single explicit canonical byte writer: stable bytes are produced ad hoc, and the local action-tree encoders are demonstrably ambiguous (UNI8CMECSCA-004). This ticket adds `StableBytesWriter` v1 — a small explicit, versioned writer with domain separation, field/type tags, length framing, and deterministic sequence rules — plus golden byte vectors for every primitive and framing operation (C-05). It defines *bytes*; it does not redefine the hash algorithm (`HashValue::from_stable_bytes` is unchanged), and it migrates no game.

## Assumption Reassessment (2026-06-22)

1. `crates/engine-core/src/replay.rs` defines `HashValue::from_stable_bytes` and the `StableSerialize` trait (`fn stable_bytes(&self) -> Vec<u8>`, default `stable_hash`); no `StableBytesWriter` exists today (confirmed by grep at the reassessed commit). The writer is additive and re-exported from `replay`.
2. Spec §4.3 C-05 fixes the v1 properties and framing: `magic = "RPSB"`, `writer_version: u16 LE = 1`, `domain_length: u32 LE`, `domain` raw bytes, `surface_version: u32 LE`, record fields in ascending tag order, `field = tag:u32 | type:u8 | payload_length:u32 | payload`, `sequence = count:u32 | repeated(len:u32 | bytes)`, `option = discriminant:u8 | optional framed value`, `bool = one byte`. No unordered-map/float/reflection/derive helper in v1; raw UTF-8 with no hidden normalization. Register entry `MSC-8C-005` homes this in `engine-core`. Exact constants are one-line-correctable before the first fixture adopts v1 (spec A-04).
3. Cross-artifact boundary under audit: the kernel hash/serialization contract (`crates/engine-core/src/replay.rs`, `docs/ARCHITECTURE.md` replay/determinism sections). The writer adds a new byte surface; existing `StableSerialize`/hash behavior is untouched.
4. FOUNDATIONS §11 determinism: framing is deterministic (fixed endianness, length-delimited, caller-supplied stable order); duplicate/non-increasing record field tags are rejected; no incidental map iteration order can enter the bytes.
5. Deterministic replay/hash surface under audit (§11/§13): `HashValue::from_stable_bytes` remains the hash function (A-05) — the writer defines input bytes only and triggers no §13 ADR (no hash-algorithm change). No hidden information enters byte framing (it carries only the values the caller frames).

## Architecture Check

1. An explicit small domain-specific writer with published framing and golden vectors is the spec's reasoned choice over a general serializer (Protobuf/Borsh/CBOR), which the §7.5 prior art shows are not canonical long-lived hash surfaces by default.
2. No backwards-compatibility shim — the writer is new; nothing is aliased and no existing byte output changes.
3. `engine-core` gains only replay/hash byte infrastructure (allowed kernel surface); identifiers stay noun-free (`bash scripts/boundary-check.sh`).

## Verification Layers

1. Golden byte vectors for every primitive/framing op (int endianness, string/bytes length, nested record, sequence, option, bool, enum discriminant) → engine-core unit tests with escaped/hex expected bytes.
2. Field-tag ordering errors: duplicate/non-increasing tags reject → negative tests.
3. Delimiter-collision resistance (a delimiter inside a string cannot forge a boundary) → negative test mirroring the UNI8CMECSCA-004 ambiguity classes.
4. Cross-run determinism (same input → same bytes) → repeated-encode equality test.
5. Existing `StableSerialize`/hash output unchanged → `cargo test -p engine-core`.

## What to Change

### 1. `StableBytesWriter` v1

Implement the writer with the §4.3 framing and the required properties: explicit domain + surface version, integer endianness, field/type tags, length-delimited strings/bytes/nested records/sequence elements, explicit option/enum discriminants, caller-supplied stable order, duplicate/non-increasing tag rejection, raw UTF-8. No unordered-map/float/reflection/derive helper.

### 2. Byte-contract tests

Golden vectors per operation; ordering and collision negatives; determinism check.

## Files to Touch

- `crates/engine-core/src/replay.rs` (modify — add the writer or a re-exported sibling)

## Out of Scope

- Encoding the action tree (UNI8CMECSCA-013) or migrating any game fixture/hash.
- Any change to `HashValue::from_stable_bytes` or existing `StableSerialize` output.
- Any unordered-map/float/reflection/derive escape hatch in v1.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p engine-core` passes, including the golden vectors and ordering/collision negatives.
2. Encoding the same value twice yields identical bytes; duplicate/non-increasing field tags error.
3. `bash scripts/boundary-check.sh` and `cargo build --workspace` pass.

### Invariants

1. v1 framing matches the published constants; no derive/reflection/unordered-map/float path exists.
2. `HashValue::from_stable_bytes` and existing `StableSerialize` outputs are unchanged.

## Test Plan

### New/Modified Tests

1. `crates/engine-core/src/replay.rs` (inline `#[cfg(test)]`) — per-operation golden byte vectors (hex/escaped).
2. `crates/engine-core/src/replay.rs` (inline `#[cfg(test)]`) — tag-ordering + delimiter-collision negatives + determinism.

### Commands

1. `cargo test -p engine-core`
2. `bash scripts/boundary-check.sh`
3. The engine-core suite is the correct boundary — no game adopts v1 bytes until the Wave-2 pilots.
