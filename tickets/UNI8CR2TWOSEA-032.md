# UNI8CR2TWOSEA-032: High Card Duel — setup-evidence-v1 profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/high_card_duel/tests/serialization.rs`; adopts `game-test-support` `SetupEvidenceV1Driver`; fixture read-only
**Deps**: 001

## Problem

Spec §3.9 / task `8C-R2-611`: add a `setup-evidence-v1` profile driver for High Card Duel asserting the fixture's public setup metadata, while the exact private-deal assertions remain internal-dev. No fixture rewrite (`canonical_byte_authority: none`).

## Assumption Reassessment (2026-06-23)

1. `game-test-support` exposes `SetupEvidenceV1Driver` (`crates/game-test-support/src/profiles.rs:108`) and `SETUP_EVIDENCE_V1` = `"setup-evidence-v1"`; HCD already carries the dev-dependency; `data/fixtures/high_card_duel_standard.fixture.json` exists.
2. Spec §3.9/§9: `migrate`; fixture metadata only; private-deal assertions remain internal-dev; no fixture rewrite to add metadata.
3. Cross-crate boundary under audit: `game-test-support::profiles::SetupEvidenceV1Driver` — validates metadata and delegates to the owning setup validator; the fixture stays typed data.
4. Determinism / no-leak: the driver reads the existing fixture read-only with byte equality to the `-001` baseline; the private deal stays an internal-dev assertion, never a public fixture field (§11).

## Architecture Check

1. A thin setup-evidence driver over the existing fixture adds typed public metadata without rewriting data — cleaner than embedding evidence in the fixture.
2. No backwards-compat alias; the fixture is read-only.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates public setup metadata and rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p high_card_duel`).
2. Fixture bytes unchanged -> codebase grep-proof / `cargo run -p fixture-check -- --game high_card_duel`.
3. Private deal stays internal-dev -> codebase grep-proof (no private deal field in the public fixture).

## What to Change

### 1. Add the setup-evidence-v1 driver test

In `tests/serialization.rs`, invoke `SetupEvidenceV1Driver` over the existing fixture's public setup metadata, asserting valid metadata and wrong-profile/owner/field rejection; keep private-deal assertions internal-dev.

## Files to Touch

- `games/high_card_duel/tests/serialization.rs` (modify)

## Out of Scope

- Any fixture byte rewrite; promoting private-deal evidence to a public field.
- The replay-command / public-export profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel` green, including the setup-evidence driver test.
2. `cargo run -p fixture-check -- --game high_card_duel` — fixture valid and byte-unchanged.

### Invariants

1. The fixture is read-only; the driver claims no new canonical bytes.
2. Private-deal evidence never appears as a public fixture field.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/serialization.rs` — `setup-evidence-v1` driver metadata + rejection test.

### Commands

1. `cargo test -p high_card_duel`
2. `cargo run -p fixture-check -- --game high_card_duel`
