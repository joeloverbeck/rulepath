# 8CR3PUBCOOASY-633: C-08 Frontier Control public-export profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/frontier_control/tests/replay.rs`
**Deps**: 8CR3PUBCOOASY-603

## Problem

C-08 adds a `public-export-v1` profile driver for `frontier_control`: a dev-only
`PublicExportV1Driver` validating public-export profile metadata over the
existing exporter — a fully-public observation timeline with import round-trip —
and delegating export production to the game. The game is fully public, so the
export carries no hidden facts.

## Assumption Reassessment (2026-06-24)

1. Shipped `PublicExportV1Driver` at `crates/game-test-support/src/profiles.rs:100`.
   Frontier's exporter lives in `src/replay_support.rs`; `tests/replay.rs`
   exists. 603 already touched `replay_support.rs` (serialized chain); 633 `Deps`
   603.
2. Spec §3.9 verdict for Frontier `public-export-v1` is `migrate` (fully public
   observation timeline); §5.12 task `8C-R3-633` scopes the driver over the
   existing exporter and `replay-export-import` round-trip.
3. Cross-artifact boundary under audit: the driver validates metadata and
   delegates to the existing exporter; it creates no new exporter.
4. FOUNDATIONS §11 + ADR 0004: the export is fully public (no hidden class); the
   driver must not invent a redaction it doesn't need.
5. Enforcement surface: the driver test in `tests/replay.rs` over the existing
   exporter (export bytes/hash byte-identical to the 001 baseline; round-trip
   preserved).

## Architecture Check

1. A metadata-validating driver delegating to the existing exporter shares the
   profile plumbing without duplicating export logic.
2. No backwards-compatibility alias — new dev-only test driver; exporter
   unchanged.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` validates profile
   metadata only.

## Verification Layers

1. Metadata gating -> `tests/replay.rs`: valid public-export metadata passes;
   wrong profile/version/owner/visibility/field rejects.
2. Public correctness -> the fully-public timeline is exported; export
   bytes/hash byte-identical to baseline.
3. Round-trip -> `replay-export-import` round-trips unchanged.

## What to Change

### 1. Add the public-export-v1 driver test

In `tests/replay.rs`, construct `PublicExportV1Driver`, validate the
public-export metadata, drive the existing exporter, assert the fully-public
timeline and import round-trip. Add wrong-metadata rejection cases.

## Files to Touch

- `games/frontier_control/tests/replay.rs` (modify)

## Out of Scope

- Creating a new exporter or inventing a redaction class.
- Seat-private export (N/A — recorded by 802); other profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` (driver metadata + round-trip tests).
2. `cargo run -p replay-check -- --game frontier_control --all` — byte-identical to baseline.

### Invariants

1. The fully-public export is unchanged from baseline; round-trip preserved.
2. The driver validates metadata then delegates; no new exporter is created.

## Test Plan

### New/Modified Tests

1. `games/frontier_control/tests/replay.rs` — `PublicExportV1Driver` metadata +
   import-round-trip cases over the existing exporter.

### Commands

1. `cargo test -p frontier_control`
2. `cargo run -p replay-check -- --game frontier_control --all`
3. A per-game test + replay-check is the correct boundary: the driver is
   test-side over the existing exporter.

## Outcome

- Added a dev-only `PublicExportV1Driver` wrapper in
  `games/frontier_control/tests/replay.rs` for `public-export-v1` / `v1` with
  `public` visibility, owner `frontier_control`, canonical byte authority
  `none`, and fields `export_steps`, `import_round_trip`,
  `hidden_absence_tokens`, and `not_applicable`.
- The wrapper delegates to Frontier Control's existing fully-public exporter,
  asserting export/import round-trip, stable public export hash generation, and
  the existing not-applicable hidden-information note. No exporter, fixture
  bytes, production code, replay behavior, or redaction behavior changed.
- Added fail-closed rejection cases for wrong profile, wrong version, invalid
  visibility, wrong owner, and unknown field.
- Verification passed:
  - `cargo test -p frontier_control`
  - `cargo run -p replay-check -- --game frontier_control --all`
  - `git diff --check`
