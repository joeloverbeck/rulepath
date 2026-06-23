# 8CR1PUBFIXSEA-034: Directional Flip C-08 setup-evidence profile driver

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/directional_flip` (`tests/replay.rs`); fixture bytes unchanged
**Deps**: 8CR1PUBFIXSEA-031

## Problem

Directional Flip has no `setup-evidence-v1` profile adapter around its standard setup fixture. Add a parallel `SetupEvidenceV1Driver` adapter (validator owner `fixture-check`, canonical-byte authority `none`) validating a public setup profile, then delegating to the existing setup assertions. The fixture `games/directional_flip/data/fixtures/directional_flip_standard.fixture.json` is read-only evidence; its bytes, setup hashes/state, RNG use, and visibility are unchanged. The dev-only `game-test-support` edge is already added by `-031`.

## Assumption Reassessment (2026-06-23)

1. `crates/game-test-support/src/profiles.rs` exposes `SetupEvidenceV1Driver` and the `SETUP_EVIDENCE_V1`/`PROFILE_VERSION_V1` constants; `games/directional_flip/data/fixtures/directional_flip_standard.fixture.json` exists; `games/directional_flip/tests/replay.rs` exists. The dev-only `game-test-support` dependency is added by `-031` (this ticket depends on it). Confirmed during reassessment.
2. Spec §3.7 and §5.9 (task `8C-R1-512`) classify Directional Flip `setup-evidence-v1` as `migrate`; the driver validates metadata with byte authority `none` and delegates to game setup assertions. MSC-8C-008 owns evidence-profile drivers.
3. Cross-artifact: `game-test-support` is dev-only; the adapter reads the existing setup fixture read-only. Before-baseline from `-001`.
4. §6 (evidence-heavy) and §3/§11 motivate this ticket: the driver classifies setup evidence without setting up or deciding rules; behavior stays in the game test; `fixture-check` remains the validator owner.
5. Enforcement surface = the dev-only `game-test-support` boundary (C-06), the setup-fixture bytes, and RNG use; the adapter declares byte authority `none`, changes no fixture byte or RNG vector, and leaks no hidden information.

## Architecture Check

1. A parallel typed setup-profile adapter with byte authority `none` classifies setup evidence without rewriting the fixture or claiming canonical-byte ownership.
2. No backwards-compatibility shim is introduced; the adapter is additive dev-only test code.
3. `engine-core` stays noun-free (§3); `game-test-support` stays dev-only (§4/C-06); `fixture-check` remains the validator owner.

## Verification Layers

1. `setup-evidence-v1` metadata validated (byte authority `none`, seat grammar, setup options) -> `SetupEvidenceV1Driver` assertion in `tests/replay.rs`.
2. Existing setup assertions still pass after delegation -> `cargo test -p directional_flip` + `cargo run -p fixture-check -- --game directional_flip`.
3. Fixture bytes, setup state/hashes, RNG use unchanged -> `git diff` shows no fixture change.

## What to Change

### 1. Add the setup-profile adapter test

In `games/directional_flip/tests/replay.rs`, build a `SetupEvidenceV1Driver` with validator owner `fixture-check` and byte authority `none`; validate the public setup profile; then delegate to the existing setup assertions. Read the fixture read-only.

## Files to Touch

- `games/directional_flip/tests/replay.rs` (modify; dev-dep edge created by 8CR1PUBFIXSEA-031)

## Out of Scope

- Any change to `games/directional_flip/data/fixtures/directional_flip_standard.fixture.json` bytes (read-only evidence).
- Re-adding the `game-test-support` dev-dependency (owned by `-031`).
- Any RNG, setup-state, or visibility change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p directional_flip` is green, including the new setup-profile-validation assertion.
2. `cargo run -p fixture-check -- --game directional_flip` passes; fixture bytes unchanged.
3. `git diff` shows no change to the standard setup fixture.

### Invariants

1. The setup profile declares byte authority `none`; fixture bytes, setup hashes/state, RNG use, and visibility are unchanged.
2. The driver validates metadata and delegates setup behavior to the game test.

## Test Plan

### New/Modified Tests

1. `games/directional_flip/tests/replay.rs` — `setup-evidence-v1` profile-metadata validation (byte authority `none`) wrapping the existing setup assertions.

### Commands

1. `cargo test -p directional_flip`
2. `cargo run -p fixture-check -- --game directional_flip`
3. The per-game test plus `fixture-check` are the correct boundary.
