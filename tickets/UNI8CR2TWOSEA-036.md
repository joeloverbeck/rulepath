# UNI8CR2TWOSEA-036: High Card Duel — public-export-v1 profile driver

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/high_card_duel/tests/{replay,visibility}.rs`; adopts `game-test-support` `PublicExportV1Driver` (observer-only)
**Deps**: 024, 028

## Problem

Spec §3.9 / task `8C-R2-621`: add a `public-export-v1` profile driver for High Card Duel over the observer-only `export_public_observer_replay`, asserting the current export bytes/hash with no private fact disclosed. Current export bytes remain the authority (`canonical_byte_authority: none`). Reuses the `-024` C-07 viewer expectations and shares the `replay_support` chain via `-028` (hence `Deps: 024, 028`).

## Assumption Reassessment (2026-06-23)

1. `games/high_card_duel/src/replay_support.rs::export_public_observer_replay` exists (confirmed line ~215) returning a `PublicReplayExport`; `PublicExportV1Driver` (`profiles.rs:100`) and `PUBLIC_EXPORT_V1` = `"public-export-v1"` exist.
2. Spec §3.9/§9 + ADR-0004: observer-only; public exports omit all disallowed private facts; current export bytes remain authoritative; no artifact rewrite.
3. Cross-crate boundary under audit: `game-test-support::profiles::PublicExportV1Driver` — validates metadata and delegates to the existing export/validator; the game owns the export bytes.
4. Determinism / no-leak: the driver validates the observer export with byte equality to the `-001` baseline; the unrevealed private card never appears in the public export (§11 no-leak firewall, ADR-0004).

## Architecture Check

1. A thin public-export driver over the existing observer export adds typed evidence without a new canonical-byte authority — consistent across games and ADR-0004-faithful.
2. No backwards-compat alias; no export rewrite.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates observer export metadata, rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p high_card_duel`).
2. Public export omits private facts -> no-leak visibility test (`tests/visibility.rs`).
3. Export bytes/hash unchanged -> deterministic replay-hash check (`replay-check --game high_card_duel --all`).

## What to Change

### 1. Add the public-export-v1 driver test

In `tests/replay.rs` (with the no-leak assertions in `tests/visibility.rs`), invoke `PublicExportV1Driver` over `export_public_observer_replay`, asserting valid metadata and wrong-profile/owner/field rejection.

## Files to Touch

- `games/high_card_duel/tests/replay.rs` (modify)
- `games/high_card_duel/tests/visibility.rs` (modify)

## Out of Scope

- Any export byte rewrite; a seat-private exporter (HCD seat-private is N/A, recorded in `-045`).
- The replay-command / setup-evidence profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel` green, including the public-export driver test.
2. `cargo run -p replay-check -- --game high_card_duel --all` — export bytes/hash byte-identical to baseline.

### Invariants

1. The public export discloses no private fact; the driver claims no new canonical bytes.
2. The export remains observer-only.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/replay.rs` — `public-export-v1` driver metadata + rejection test; no-leak assertion in `tests/visibility.rs`.

### Commands

1. `cargo test -p high_card_duel`
2. `cargo run -p replay-check -- --game high_card_duel --all`
