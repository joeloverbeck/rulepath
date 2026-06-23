# 8CR1PUBFIXSEA-033: Draughts Lite C-08 setup-evidence profile driver

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/draughts_lite` (`tests/replay.rs`); fixture bytes unchanged
**Deps**: 8CR1PUBFIXSEA-028

## Problem

Draughts Lite has no `setup-evidence-v1` profile adapter around its standard setup fixture. Add a parallel `SetupEvidenceV1Driver` adapter (validator owner `fixture-check`, canonical-byte authority `none`, canonical byte claim `false`) that validates canonical seat grammar, setup options, and expected setup metadata, then delegates to the existing setup assertions. The fixture `games/draughts_lite/data/fixtures/draughts_lite_standard.fixture.json` is read-only evidence; its bytes are unchanged. The dev-only `game-test-support` edge is already added by `-028`.

## Assumption Reassessment (2026-06-23)

1. `crates/game-test-support/src/profiles.rs` exposes `SetupEvidenceV1Driver` and the `SETUP_EVIDENCE_V1`/`PROFILE_VERSION_V1` constants; `games/draughts_lite/data/fixtures/draughts_lite_standard.fixture.json` exists with a `setup` fixture kind; `games/draughts_lite/tests/replay.rs` exists. The dev-only `game-test-support` dependency is added by `-028` (this ticket depends on it). Confirmed during reassessment.
2. Spec §3.7 and §5.9 (task `8C-R1-511`) classify Draughts `setup-evidence-v1` as `migrate`; the driver validates metadata with byte authority `none` and delegates to game setup assertions. MSC-8C-008 owns evidence-profile drivers.
3. Cross-artifact: `game-test-support` is dev-only; the adapter reads the existing setup fixture (shared evidence contract) read-only. Before-baseline (fixture bytes) from `-001`.
4. §6 (evidence-heavy) and §3/§11 motivate this ticket: the driver classifies setup evidence without setting up or deciding rules; behavior stays in the game test; `fixture-check` remains the validator owner.
5. Enforcement surface = the dev-only `game-test-support` boundary (C-06) and the setup-fixture bytes; the adapter declares byte authority `none`, changes no fixture byte, and leaks no hidden information.

## Architecture Check

1. A parallel typed setup-profile adapter with byte authority `none` classifies setup evidence without rewriting the fixture or claiming canonical-byte ownership.
2. No backwards-compatibility shim is introduced; the adapter is additive dev-only test code.
3. `engine-core` stays noun-free (§3); `game-test-support` stays dev-only (§4/C-06); `fixture-check` remains the validator owner.

## Verification Layers

1. `setup-evidence-v1` metadata validated (byte authority `none`, canonical byte claim `false`, seat grammar, setup options) -> `SetupEvidenceV1Driver` assertion in `tests/replay.rs`.
2. Existing setup assertions still pass after delegation -> `cargo test -p draughts_lite` + `cargo run -p fixture-check -- --game draughts_lite`.
3. Fixture bytes unchanged -> `git diff` shows no fixture change.

## What to Change

### 1. Add the setup-profile adapter test

In `games/draughts_lite/tests/replay.rs` (or the reassessed fixture-test owner), build a `SetupEvidenceV1Driver` with validator owner `fixture-check`, canonical-byte authority `none`, canonical byte claim `false`, canonical seat grammar, setup options, and expected setup; validate; then delegate to the existing setup assertions. Read the fixture read-only.

## Files to Touch

- `games/draughts_lite/tests/replay.rs` (modify; dev-dep edge created by 8CR1PUBFIXSEA-028)

## Out of Scope

- Any change to `games/draughts_lite/data/fixtures/draughts_lite_standard.fixture.json` bytes (read-only evidence).
- Re-adding the `game-test-support` dev-dependency (owned by `-028`).
- Moving setup behavior into `game-test-support` or `fixture-check`.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` is green, including the new setup-profile-validation assertion.
2. `cargo run -p fixture-check -- --game draughts_lite` passes; fixture bytes unchanged.
3. `git diff` shows no change to the standard setup fixture.

### Invariants

1. The setup profile declares byte authority `none`; the fixture bytes are unchanged.
2. The driver validates metadata and delegates setup behavior to the game test.

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/tests/replay.rs` — `setup-evidence-v1` profile-metadata validation (byte authority `none`) wrapping the existing setup assertions.

### Commands

1. `cargo test -p draughts_lite`
2. `cargo run -p fixture-check -- --game draughts_lite`
3. The per-game test plus `fixture-check` are the correct boundary: setup classification is game-local, fixture validation is `fixture-check`'s.

## Outcome

Completed on 2026-06-23.

Added `setup_evidence_v1_driver_validates_standard_setup_fixture` in
`games/draughts_lite/tests/replay.rs`. The test builds a typed
`ProfileArtifact` with `setup-evidence-v1` / `v1`, public visibility,
`fixture-check` validator ownership, `canonical_byte_authority: "none"`, and
`canonical_byte_claim: false`, then validates with
`SetupEvidenceV1Driver::new("fixture-check")` before delegating to the
existing standard setup fixture metadata assertions. The fixture remains
profile-free and byte-unchanged.

Verification:

1. `cargo test -p draughts_lite setup_evidence_v1_driver_validates_standard_setup_fixture -- --exact`
2. `cargo test -p draughts_lite`
3. `cargo run -p fixture-check -- --game draughts_lite`
4. `cargo fmt --all -- --check`
5. `git diff --name-only -- games/draughts_lite/data/fixtures/draughts_lite_standard.fixture.json`
