# UNI8CR2TWOSEA-034: Poker Lite — setup-evidence-v1 profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/poker_lite/tests/serialization.rs`; adopts `game-test-support` `SetupEvidenceV1Driver`; fixture read-only
**Deps**: 022

## Problem

Spec §3.9 / task `8C-R2-613`: add a `setup-evidence-v1` profile driver for Poker Lite asserting setup/deck declaration metadata, with the private dealt cards remaining internal test evidence (not an exported private hand). No fixture rewrite (`canonical_byte_authority: none`). Needs the `-022` dev-dependency.

## Assumption Reassessment (2026-06-23)

1. `game-test-support` exposes `SetupEvidenceV1Driver` (`profiles.rs:108`) and `SETUP_EVIDENCE_V1`; Poker gains the dev-dependency in `-022`; `data/fixtures/poker_lite_standard.fixture.json` exists.
2. Spec §3.9/§9: `migrate`; fixture metadata / deck declaration only; private dealt cards stay internal test evidence; no fixture rewrite.
3. Cross-crate boundary under audit: `game-test-support::profiles::SetupEvidenceV1Driver` — validates metadata and delegates to the owning setup validator; the fixture stays typed data.
4. Determinism / no-leak: the driver reads the existing fixture read-only with byte equality to the `-001` baseline; private dealt cards never appear as a public fixture field (§11).

## Architecture Check

1. A thin setup-evidence driver over the existing fixture adds typed deck/setup metadata without exporting a private hand — cleaner than embedding evidence in the fixture.
2. No backwards-compat alias; the fixture is read-only.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates deck/setup metadata and rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p poker_lite`).
2. Fixture bytes unchanged -> `cargo run -p fixture-check -- --game poker_lite` + codebase grep-proof.
3. Private dealt cards stay internal-dev -> codebase grep-proof (no private hand field in the public fixture).

## What to Change

### 1. Add the setup-evidence-v1 driver test

In `tests/serialization.rs`, invoke `SetupEvidenceV1Driver` over the existing fixture's setup/deck metadata, asserting valid metadata and wrong-profile/owner/field rejection; keep dealt-card assertions internal-dev.

## Files to Touch

- `games/poker_lite/tests/serialization.rs` (modify)

## Out of Scope

- Any fixture byte rewrite; exporting a private hand as fixture data.
- The replay-command / public-export / seat-private-export profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` green, including the setup-evidence driver test.
2. `cargo run -p fixture-check -- --game poker_lite` — fixture valid and byte-unchanged.

### Invariants

1. The fixture is read-only; the driver claims no new canonical bytes.
2. Private dealt cards never appear as a public fixture field.

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/serialization.rs` — `setup-evidence-v1` driver metadata + rejection test.

### Commands

1. `cargo test -p poker_lite`
2. `cargo run -p fixture-check -- --game poker_lite`
