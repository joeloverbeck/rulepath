# UNI8CR2TWOSEA-033: Secret Draft — setup-evidence-v1 profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/secret_draft/tests/serialization.rs`; adopts `game-test-support` `SetupEvidenceV1Driver`; fixture read-only
**Deps**: 021

## Problem

Spec §3.9 / task `8C-R2-612`: add a `setup-evidence-v1` profile driver for Secret Draft asserting the fixture's public setup parameters with commitments empty — no reveal behavior in data. No fixture rewrite (`canonical_byte_authority: none`). Needs the `-021` dev-dependency.

## Assumption Reassessment (2026-06-23)

1. `game-test-support` exposes `SetupEvidenceV1Driver` (`profiles.rs:108`) and `SETUP_EVIDENCE_V1`; Secret gains the dev-dependency in `-021`; `data/fixtures/secret_draft_standard.fixture.json` exists.
2. Spec §3.9/§5/§9: `migrate`; fixture contains public setup parameters / empty commitments; no selectors, triggers, or reveal policy in data; no fixture rewrite.
3. Cross-crate boundary under audit: `game-test-support::profiles::SetupEvidenceV1Driver` — validates metadata and delegates to the owning setup validator; the fixture stays typed data.
4. Determinism / no-leak: the driver reads the existing fixture read-only with byte equality to the `-001` baseline; commitments remain empty and no committed item appears in setup data (§5/§11).

## Architecture Check

1. A thin setup-evidence driver over the existing fixture adds typed public metadata without rewriting data or encoding reveal behavior — cleaner than embedding evidence in the fixture.
2. No backwards-compat alias; the fixture is read-only.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates public setup parameters and rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p secret_draft`).
2. Fixture bytes unchanged, commitments empty -> `cargo run -p fixture-check -- --game secret_draft` + codebase grep-proof.
3. No reveal behavior / selectors in data -> static-data discipline check (§5) against the fixture.

## What to Change

### 1. Add the setup-evidence-v1 driver test

In `tests/serialization.rs`, invoke `SetupEvidenceV1Driver` over the existing fixture's public setup parameters (commitments empty), asserting valid metadata and wrong-profile/owner/field rejection.

## Files to Touch

- `games/secret_draft/tests/serialization.rs` (modify)

## Out of Scope

- Any fixture byte rewrite; encoding commitment/reveal behavior into data.
- The replay-command / public-export / seat-private-export profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft` green, including the setup-evidence driver test.
2. `cargo run -p fixture-check -- --game secret_draft` — fixture valid and byte-unchanged.

### Invariants

1. The fixture is read-only with empty commitments; the driver claims no new canonical bytes.
2. No selector, trigger, or reveal behavior enters fixture data.

## Test Plan

### New/Modified Tests

1. `games/secret_draft/tests/serialization.rs` — `setup-evidence-v1` driver metadata + rejection test.

### Commands

1. `cargo test -p secret_draft`
2. `cargo run -p fixture-check -- --game secret_draft`
