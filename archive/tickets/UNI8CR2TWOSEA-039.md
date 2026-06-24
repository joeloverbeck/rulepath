# UNI8CR2TWOSEA-039: Masked Claims — public-export-v1 profile driver

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/masked_claims/tests/{replay,visibility}.rs`; adopts `game-test-support` `PublicExportV1Driver` (observer-only)
**Deps**: 027, 031

## Problem

Spec §3.9 / task `8C-R2-624`: add a `public-export-v1` profile driver for Masked Claims over the observer-only `PublicReplayExport` path, asserting current export bytes/hash with claim-tile redaction. Current export bytes remain the authority (`canonical_byte_authority: none`). Reuses the `-027` C-07 viewer expectations and shares the `replay_support` chain via `-031` (hence `Deps: 027, 031`).

## Assumption Reassessment (2026-06-23)

1. `games/masked_claims/src/replay_support.rs::PublicReplayExport` exists (confirmed line ~6) as the observer-only public export path; `PublicExportV1Driver`/`PUBLIC_EXPORT_V1` exist.
2. Spec §3.9/§9 + ADR-0004: observer-only; claim tile identity redacted; current export bytes remain authoritative; no artifact rewrite.
3. Cross-crate boundary under audit: `game-test-support::profiles::PublicExportV1Driver` — validates metadata and delegates to the existing export path; the game owns the export bytes.
4. Determinism / no-leak: the driver validates the observer export with byte equality to the `-001` baseline; claimed/masked tile identity never appears in the public export before the authorized reveal (§11, ADR-0004).

## Architecture Check

1. A thin public-export driver over the existing observer export adds typed evidence without a new canonical-byte authority — ADR-0004-faithful and consistent across games.
2. No backwards-compat alias; no export rewrite.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates observer export metadata, rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p masked_claims`).
2. Public export redacts claim tile identity -> no-leak visibility test (`tests/visibility.rs`).
3. Export bytes/hash unchanged -> deterministic replay-hash check (`replay-check --game masked_claims --all`).

## What to Change

### 1. Add the public-export-v1 driver test

In `tests/replay.rs` (with no-leak assertions in `tests/visibility.rs`), invoke `PublicExportV1Driver` over the `PublicReplayExport` observer path, asserting valid metadata and wrong-profile/owner/field rejection.

## Files to Touch

- `games/masked_claims/tests/replay.rs` (modify)
- `games/masked_claims/tests/visibility.rs` (modify)

## Out of Scope

- Any export byte rewrite; a seat-private exporter (Masked seat-private is N/A, recorded in `-045`).
- The replay-command / setup-evidence profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` green, including the public-export driver test.
2. `cargo run -p replay-check -- --game masked_claims --all` — export bytes/hash byte-identical to baseline.

### Invariants

1. The public export discloses no claimed/masked tile identity; the driver claims no new canonical bytes.
2. The export remains observer-only.

## Test Plan

### New/Modified Tests

1. `games/masked_claims/tests/replay.rs` — `public-export-v1` driver metadata + rejection test; no-leak assertion in `tests/visibility.rs`.

### Commands

1. `cargo test -p masked_claims`
2. `cargo run -p replay-check -- --game masked_claims --all`

## Outcome

Implemented in `games/masked_claims/tests/replay.rs` with
`public_export_v1_profile_driver_wraps_observer_export_validator`. The test
validates `public-export-v1` metadata for `masked_claims`, delegates through
`PublicExportV1Driver::validate_with` to the existing observer export JSON
bytes, and confirms the driver makes no canonical byte claim.

The observer export path is unchanged and continues to redact claim tile
identity and seed material. The test rejects wrong profile id, wrong validator
owner, wrong visibility, and an illegal profile field.
`games/masked_claims/tests/visibility.rs` now explicitly checks observer export
seed redaction alongside hidden claimed-tile redaction.

Verification passed:

1. `cargo test -p masked_claims public_export_v1_profile_driver_wraps_observer_export_validator -- --nocapture`
2. `cargo test -p masked_claims public_and_opponent_surfaces_hide_unrevealed_tile_ids -- --nocapture`
3. `cargo test -p masked_claims`
4. `cargo run -p replay-check -- --game masked_claims --all`
