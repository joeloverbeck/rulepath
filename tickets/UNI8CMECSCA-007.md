# UNI8CMECSCA-007: Strict canonical `SeatId` parse/format in `engine-core`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/engine-core/src/lib.rs` (`SeatId` methods, `CanonicalSeatIdError`)
**Deps**: UNI8CMECSCA-002

## Problem

Canonical seat identity (`seat_<zero-based>`) is kernel vocabulary but currently has no single strict parser/formatter — games hand-roll `format!("seat_{}", …)` and bespoke `parse` functions, and legacy hyphen spellings circulate at the transport edge. This ticket adds the one strict canonical constructor/parser/index-extractor in `engine-core`, with typed errors and a documented grammar version (C-02, kernel side). It deliberately does **not** add a permissive global `FromStr`: legacy/symbolic aliases are handled only at the WASM import boundary (UNI8CMECSCA-008).

## Assumption Reassessment (2026-06-22)

1. `SeatId(pub String)` is defined in `crates/engine-core/src/lib.rs` with no `FromStr`/`Display`/`parse_canonical`/`from_zero_based_index`/`canonical_zero_based_index` impls today (confirmed by grep at the reassessed commit). The game layer already trends underscore-canonical (e.g. `games/race_to_n/src/ids.rs` formats `seat_<n>`), so the kernel grammar matches existing game output.
2. Spec §4.3 C-02 fixes the surface: `from_zero_based_index(u32) -> Self`, `parse_canonical(&str) -> Result<Self, CanonicalSeatIdError>`, `canonical_zero_based_index(&self) -> Result<u32, CanonicalSeatIdError>`. Grammar is exactly `seat_<unsigned-zero-based-decimal>`; the strict parser rejects whitespace, signs, empty suffix, non-digits, overflow, and non-canonical leading zeros (except `seat_0`); formatting always emits canonical form. Register entry `MSC-8C-002` homes this in `engine-core`.
3. Cross-artifact boundary under audit: the `seat id` kernel vocabulary (`docs/ENGINE-GAME-DATA-BOUNDARY.md` §3, `docs/WASM-CLIENT-BOUNDARY.md`). This ticket owns the canonical Rust authority only; the transport alias adapter is UNI8CMECSCA-008.
4. FOUNDATIONS §2: Rust becomes the sole canonical parse/format authority for seat IDs; no legality or normalization moves to TypeScript. §11 `FromStr` round-trip caution (§7.5) motivates one documented strict grammar rather than global permissiveness.
5. No-leak / determinism surface under audit (§11): canonical formatting is deterministic and total; parsing is fail-closed (errors are typed, never a silent fallback). No hidden information is involved — seat IDs are public identity.

## Architecture Check

1. A strict canonical constructor/parser in the kernel, with aliases isolated at import, prevents legacy/role labels from silently becoming the global `SeatId` meaning — the explicit boundary the spec requires.
2. No backwards-compatibility shim — no permissive global `FromStr`; legacy parsing is the adapter's job, not the kernel's.
3. `engine-core` gains only `seat id` ergonomics (an allowed kernel noun); no mechanic noun, no role/team semantics.

## Verification Layers

1. Round-trip: `from_zero_based_index(n)` → `canonical_zero_based_index` returns `n`; `parse_canonical("seat_n")` round-trips → engine-core unit tests.
2. Strict rejection: whitespace, signs, empty suffix, non-digits, overflow (`u32`), leading zeros (except `seat_0`), non-ASCII digits each error with `CanonicalSeatIdError` → exhaustive rejection table test.
3. Formatting always emits underscore canonical → format test.
4. Kernel stays noun-free → `bash scripts/boundary-check.sh`.

## What to Change

### 1. `crates/engine-core/src/lib.rs` — canonical `SeatId` API

Add `from_zero_based_index`, `parse_canonical`, `canonical_zero_based_index`, and the `CanonicalSeatIdError` enum (typed structural variants). Add rustdoc defining the grammar and its version. Do not add a blanket `FromStr` for all `SeatId` values.

### 2. Tests

Exhaustive round-trip and rejection tables covering `seat_0`, boundary indices, `u32` overflow, signs, whitespace, empty suffix, leading zeros, and non-ASCII digits.

## Files to Touch

- `crates/engine-core/src/lib.rs` (modify)

## Out of Scope

- The WASM import/alias adapter and legacy hyphen handling (UNI8CMECSCA-008).
- Migrating any game's call sites (UNI8CMECSCA-009).
- A permissive global `FromStr` or symbolic-alias acceptance in the kernel.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p engine-core` passes, including the round-trip and rejection tables.
2. A rejection test proves each of: whitespace, sign, empty suffix, non-digit, `u32` overflow, leading-zero (non-`seat_0`), non-ASCII digit → `CanonicalSeatIdError`.
3. `bash scripts/boundary-check.sh` passes (no forbidden noun in new identifiers).

### Invariants

1. No permissive `FromStr` accepts non-canonical spellings as `SeatId`.
2. Formatting always emits `seat_<n>` underscore canonical.

## Test Plan

### New/Modified Tests

1. `crates/engine-core/src/lib.rs` (inline `#[cfg(test)]`) — canonical round-trip table.
2. `crates/engine-core/src/lib.rs` (inline `#[cfg(test)]`) — strict-rejection table.

### Commands

1. `cargo test -p engine-core`
2. `bash scripts/boundary-check.sh`
3. The engine-core suite is the correct boundary — no game adopts the API until UNI8CMECSCA-009.
