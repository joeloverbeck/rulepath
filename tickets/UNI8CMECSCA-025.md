# UNI8CMECSCA-025: Vow Tide drives `public-export-v1` and `seat-private-export-v1`

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/vow_tide/tests/replay.rs`, `games/vow_tide/Cargo.toml` (`[dev-dependencies]`)
**Deps**: UNI8CMECSCA-022

## Problem

Prove the export drivers against a real game with declared seat viewers: Vow Tide drives `public-export-v1` and `seat-private-export-v1` using its existing public round-trip and all-viewers seat-private round-trip fixtures. Public export stays observer-safe; every declared seat viewer is explicitly labeled; import reconstructs a viewer-scoped observation timeline, not omniscient state. This is a profile-driver pilot — Vow's bidding/deal/trick/no-leak policy stays local; it is not a full Vow retrofit.

## Assumption Reassessment (2026-06-22)

1. `games/vow_tide/tests/golden_traces/{public-replay-export-import,seat-private-replay-export-import-all-viewers}.trace.json` exist; `games/vow_tide/tests/replay.rs` carries replay/export tests. `PublicExportV1Driver` and `SeatPrivateExportV1Driver` exist after UNI8CMECSCA-022. `game-test-support` is not yet a Vow dev-dependency.
2. Spec §4.5 + §5 8C-025 fix the boundary: real `public-export-v1` and `seat-private-export-v1` drivers including all declared seat viewers; this does not migrate Vow's setup, no-leak matrix, seat/ring helpers, modulo RNG caller, bidding, deal, or trick behavior. Both profiles are defined in `docs/EVIDENCE-FIXTURE-CONTRACT.md`.
3. Cross-artifact boundary under audit: Vow's export fixtures and the two drivers. The drivers own profile/visibility checks and round-trip sequencing; projection/redaction, public-state construction, import semantics, authorization, reveal timing, and seat-private projection stay in Vow.
4. FOUNDATIONS §11 no-leak firewall + ADR 0004 (EC-21): public export observer-safe; seat-private export explicitly viewer-labeled; import is an observation timeline, not omniscient state.
5. No-leak/visibility surface under audit (§11/EC-21): the seat-private driver invokes the pairwise round-trip for every declared seat viewer via game-supplied projection; it must not let import restore facts the viewer never observed; Vow's hidden bidding/deal/trick policy stays local.

## Architecture Check

1. Driving both export profiles from Vow's existing round-trip fixtures proves the drivers sequence export/import without owning projection/redaction/import semantics.
2. No backwards-compatibility shim — drivers adopted; fixtures read, not rewritten.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` enters only as a dev-dependency; Vow behavior stays local.

## Verification Layers

1. Public export round-trip stays observer-safe → `cargo test -p vow_tide` (public-export driver test) + `cargo run -p replay-check -- --game vow_tide --all`.
2. Seat-private export labels every declared viewer; import restores no omniscient state → seat-private driver test (EV-PROFILES, all viewers).
3. Vow bidding/deal/trick/no-leak policy unchanged → grep-proof those modules are untouched.
4. `game-test-support` dev-only edge → `cargo tree --workspace -e normal --invert game-test-support`.

## What to Change

### 1. `games/vow_tide/Cargo.toml`

Add `game-test-support` under `[dev-dependencies]`.

### 2. `games/vow_tide/tests/replay.rs`

Adopt `PublicExportV1Driver` and `SeatPrivateExportV1Driver` over the existing public and all-viewers seat-private fixtures, delegating projection/redaction/import semantics to Vow; assert observer-safety and per-viewer labeling.

## Files to Touch

- `games/vow_tide/Cargo.toml` (modify)
- `games/vow_tide/tests/replay.rs` (modify)

## Out of Scope

- Migrating Vow's setup, no-leak matrix, seat/ring helpers, modulo RNG caller, bidding, deal, or trick behavior (a full Vow C-11 retrofit, not 8C).
- Moving projection/redaction/import semantics into the drivers.
- Any normal/build dependency on `game-test-support`.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` passes both export-driver tests, covering all declared seat viewers.
2. `cargo run -p replay-check -- --game vow_tide --all` passes (export round-trips observer-safe).
3. `cargo test --workspace` passes.

### Invariants

1. Public export is observer-safe; seat-private export is explicitly viewer-labeled; import restores no omniscient state.
2. Vow bidding/deal/trick/no-leak policy stays local; `game-test-support` is dev-only.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/replay.rs` — `public-export-v1` and `seat-private-export-v1` driver round-trips (all viewers).

### Commands

1. `cargo test -p vow_tide`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The game suite plus `replay-check` are the correct boundary — export round-trips are exercised through real Vow projection.
