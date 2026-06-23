# UNI8CR2TWOSEA-035: Masked Claims — setup-evidence-v1 profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/masked_claims/tests/serialization.rs`; adopts `game-test-support` `SetupEvidenceV1Driver`; fixture read-only
**Deps**: 023

## Problem

Spec §3.9 / task `8C-R2-614`: add a `setup-evidence-v1` profile driver for Masked Claims asserting mask ordering / status metadata, with no selectors or reaction policy in data. No fixture rewrite (`canonical_byte_authority: none`). Needs the `-023` dev-dependency.

## Assumption Reassessment (2026-06-23)

1. `game-test-support` exposes `SetupEvidenceV1Driver` (`profiles.rs:108`) and `SETUP_EVIDENCE_V1`; Masked gains the dev-dependency in `-023`; `data/fixtures/masked_claims_standard.fixture.json` exists.
2. Spec §3.9/§5/§9: `migrate`; mask ordering/status metadata only; no selectors or executable reaction policy in data; no fixture rewrite.
3. Cross-crate boundary under audit: `game-test-support::profiles::SetupEvidenceV1Driver` — validates metadata and delegates to the owning setup validator; the fixture stays typed data.
4. Determinism / no-leak: the driver reads the existing fixture read-only with byte equality to the `-001` baseline; no concealed mask identity leaks into public setup data (§5/§11).

## Architecture Check

1. A thin setup-evidence driver over the existing fixture adds typed mask-ordering metadata without encoding reaction behavior — cleaner than embedding evidence in the fixture.
2. No backwards-compat alias; the fixture is read-only.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates mask-ordering/status metadata and rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p masked_claims`).
2. Fixture bytes unchanged -> `cargo run -p fixture-check -- --game masked_claims` + codebase grep-proof.
3. No selectors/reaction policy in data -> static-data discipline check (§5) against the fixture.

## What to Change

### 1. Add the setup-evidence-v1 driver test

In `tests/serialization.rs`, invoke `SetupEvidenceV1Driver` over the existing fixture's mask-ordering/status metadata, asserting valid metadata and wrong-profile/owner/field rejection.

## Files to Touch

- `games/masked_claims/tests/serialization.rs` (modify)

## Out of Scope

- Any fixture byte rewrite; encoding reaction/reveal policy into data.
- The replay-command / public-export profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` green, including the setup-evidence driver test.
2. `cargo run -p fixture-check -- --game masked_claims` — fixture valid and byte-unchanged.

### Invariants

1. The fixture is read-only; the driver claims no new canonical bytes.
2. No selector, trigger, or reaction behavior enters fixture data.

## Test Plan

### New/Modified Tests

1. `games/masked_claims/tests/serialization.rs` — `setup-evidence-v1` driver metadata + rejection test.

### Commands

1. `cargo test -p masked_claims`
2. `cargo run -p fixture-check -- --game masked_claims`

## Outcome

Implemented in `games/masked_claims/tests/serialization.rs` with
`setup_evidence_v1_profile_driver_wraps_mask_fixture_metadata`. The test
validates `setup-evidence-v1` metadata for `masked_claims`, delegates through
`SetupEvidenceV1Driver::validate_with` to the existing read-only fixture bytes,
and confirms the driver makes no canonical byte claim.

The fixture remains mask-order/status metadata only: hand status stays
`hidden_by_setup`, reserve status stays `internal_only`, and no selector,
trigger, or reaction policy appears in fixture data. The driver rejects wrong
profile id, wrong validator owner, wrong visibility, and an illegal profile
field.

Verification passed:

1. `cargo test -p masked_claims setup_evidence_v1_profile_driver_wraps_mask_fixture_metadata -- --nocapture`
2. `cargo test -p masked_claims`
3. `cargo run -p fixture-check -- --game masked_claims`
