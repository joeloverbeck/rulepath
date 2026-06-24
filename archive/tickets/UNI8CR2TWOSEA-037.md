# UNI8CR2TWOSEA-037: Secret Draft — public-export-v1 profile driver

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/secret_draft/tests/{replay,visibility}.rs`; adopts `game-test-support` `PublicExportV1Driver` (observer)
**Deps**: 025, 029

## Problem

Spec §3.9 / task `8C-R2-622`: add a `public-export-v1` profile driver for Secret Draft over an observer invocation of `export_public_replay`, asserting current export bytes/hash with pre-reveal path/seed redaction. Current export bytes remain the authority (`canonical_byte_authority: none`). Reuses the `-025` C-07 viewer expectations and shares the `replay_support` chain via `-029` (hence `Deps: 025, 029`).

## Assumption Reassessment (2026-06-23)

1. `games/secret_draft/src/replay_support.rs::export_public_replay(trace, viewer)` exists (confirmed line ~308) returning a `PublicReplayExport`; `PublicExportV1Driver`/`PUBLIC_EXPORT_V1` exist.
2. Spec §3.9/§9 + ADR-0004: observer invocation; pre-reveal path/seed redaction preserved; current export bytes remain authoritative; no artifact rewrite.
3. Cross-crate boundary under audit: `game-test-support::profiles::PublicExportV1Driver` — validates metadata and delegates to the existing exporter; the game owns the export bytes.
4. Determinism / no-leak: the driver validates the observer export with byte equality to the `-001` baseline; the committed item and seed never appear in the public export pre-reveal (§11, ADR-0004).

## Architecture Check

1. A thin public-export driver over the existing observer export adds typed evidence without a new canonical-byte authority — ADR-0004-faithful and consistent across games.
2. No backwards-compat alias; no export rewrite.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates observer export metadata, rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p secret_draft`).
2. Public export redacts committed item + seed pre-reveal -> no-leak visibility test (`tests/visibility.rs`).
3. Export bytes/hash unchanged -> deterministic replay-hash check (`replay-check --game secret_draft --all`).

## What to Change

### 1. Add the public-export-v1 driver test

In `tests/replay.rs` (with no-leak assertions in `tests/visibility.rs`), invoke `PublicExportV1Driver` over `export_public_replay` with an observer viewer, asserting valid metadata and wrong-profile/owner/field rejection.

## Files to Touch

- `games/secret_draft/tests/replay.rs` (modify)
- `games/secret_draft/tests/visibility.rs` (modify)

## Out of Scope

- Any export byte rewrite; the seat-private export profile (`-040`).
- The replay-command / setup-evidence profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft` green, including the public-export driver test.
2. `cargo run -p replay-check -- --game secret_draft --all` — export bytes/hash byte-identical to baseline.

### Invariants

1. The public export discloses no committed item or seed pre-reveal; the driver claims no new canonical bytes.
2. The observer export path is unchanged.

## Test Plan

### New/Modified Tests

1. `games/secret_draft/tests/replay.rs` — `public-export-v1` driver metadata + rejection test; no-leak assertion in `tests/visibility.rs`.

### Commands

1. `cargo test -p secret_draft`
2. `cargo run -p replay-check -- --game secret_draft --all`

## Outcome

Implemented in `games/secret_draft/tests/replay.rs` with
`public_export_v1_profile_driver_wraps_observer_export_validator`. The test
validates `public-export-v1` metadata for `secret_draft`, delegates through
`PublicExportV1Driver::validate_with` to the existing observer pre-reveal
export hash, and confirms the driver makes no canonical byte claim.

The observer export path is unchanged and continues to redact the committed
item path and seed material. The test rejects wrong profile id, wrong validator
owner, wrong visibility, and an illegal profile field.
`games/secret_draft/tests/visibility.rs` now explicitly asserts pre-reveal
public export seed redaction alongside committed-item redaction.

Verification passed:

1. `cargo test -p secret_draft public_export_v1_profile_driver_wraps_observer_export_validator -- --nocapture`
2. `cargo test -p secret_draft pending_effects_diagnostics_and_public_export_redact_committed_item -- --nocapture`
3. `cargo test -p secret_draft`
4. `cargo run -p replay-check -- --game secret_draft --all`
