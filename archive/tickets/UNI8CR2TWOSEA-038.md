# UNI8CR2TWOSEA-038: Poker Lite — public-export-v1 profile driver

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/poker_lite/tests/{replay,visibility}.rs`; adopts `game-test-support` `PublicExportV1Driver` (observer)
**Deps**: 026, 030

## Problem

Spec §3.9 / task `8C-R2-623`: add a `public-export-v1` profile driver for Poker Lite over an observer invocation of `export_public_replay`, asserting current export bytes/hash with showdown/yield policy preserved. Current export bytes remain the authority (`canonical_byte_authority: none`). Reuses the `-026` C-07 viewer expectations and shares the `replay_support` chain via `-030` (hence `Deps: 026, 030`).

## Assumption Reassessment (2026-06-23)

1. `games/poker_lite/src/replay_support.rs::export_public_replay(trace, viewer)` exists (confirmed line ~287) returning a `PublicReplayExport`; `PublicExportV1Driver`/`PUBLIC_EXPORT_V1` exist.
2. Spec §3.9/§9 + ADR-0004: observer invocation; showdown/yield policy preserved; current export bytes remain authoritative; no artifact rewrite.
3. Cross-crate boundary under audit: `game-test-support::profiles::PublicExportV1Driver` — validates metadata and delegates to the existing exporter; the game owns the export bytes.
4. Determinism / no-leak: the driver validates the observer export with byte equality to the `-001` baseline; no private crest appears in the public export before the authorized showdown (§11, ADR-0004).

## Architecture Check

1. A thin public-export driver over the existing observer export adds typed evidence without a new canonical-byte authority — ADR-0004-faithful and consistent across games.
2. No backwards-compat alias; no export rewrite.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates observer export metadata, rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p poker_lite`).
2. Public export omits private crest pre-showdown; yield non-reveal preserved -> no-leak visibility test (`tests/visibility.rs`).
3. Export bytes/hash unchanged -> deterministic replay-hash check (`replay-check --game poker_lite --all`).

## What to Change

### 1. Add the public-export-v1 driver test

In `tests/replay.rs` (with no-leak assertions in `tests/visibility.rs`), invoke `PublicExportV1Driver` over `export_public_replay` with an observer viewer, asserting valid metadata and wrong-profile/owner/field rejection.

## Files to Touch

- `games/poker_lite/tests/replay.rs` (modify)
- `games/poker_lite/tests/visibility.rs` (modify)

## Out of Scope

- Any export byte rewrite; the seat-private export profile (`-041`).
- The replay-command / setup-evidence profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` green, including the public-export driver test.
2. `cargo run -p replay-check -- --game poker_lite --all` — export bytes/hash byte-identical to baseline.

### Invariants

1. The public export discloses no private crest before showdown; the driver claims no new canonical bytes.
2. Showdown/yield policy is unchanged.

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/replay.rs` — `public-export-v1` driver metadata + rejection test; no-leak assertion in `tests/visibility.rs`.

### Commands

1. `cargo test -p poker_lite`
2. `cargo run -p replay-check -- --game poker_lite --all`

## Outcome

Implemented in `games/poker_lite/tests/replay.rs` with
`public_export_v1_profile_driver_wraps_observer_export_validator`. The test
validates `public-export-v1` metadata for `poker_lite`, delegates through
`PublicExportV1Driver::validate_with` to the existing yield-terminal observer
export hash, and confirms the driver makes no canonical byte claim.

The export path is unchanged and preserves the no-showdown yield policy: public
export omits private crests and seed material. The test rejects wrong profile
id, wrong validator owner, wrong visibility, and an illegal profile field.
`games/poker_lite/tests/visibility.rs` now explicitly checks the yield public
export for loser-crest and seed redaction.

Verification passed:

1. `cargo test -p poker_lite public_export_v1_profile_driver_wraps_observer_export_validator -- --nocapture`
2. `cargo test -p poker_lite showdown_view_reveals_both_private_crests_and_yield_does_not -- --nocapture`
3. `cargo test -p poker_lite`
4. `cargo run -p replay-check -- --game poker_lite --all`
