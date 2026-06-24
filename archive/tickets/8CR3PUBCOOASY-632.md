# 8CR3PUBCOOASY-632: C-08 Flood Watch public-export profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/flood_watch/tests/replay.rs`
**Deps**: 8CR3PUBCOOASY-602

## Problem

C-08 adds a `public-export-v1` profile driver for `flood_watch`: a dev-only
`PublicExportV1Driver` validating public-export profile metadata over the
existing public exporter — public forecast/resolved events only, hidden
future-deck absence — and delegating export production to the game. ADR 0004
keeps the public export free of the hidden future deck.

## Assumption Reassessment (2026-06-24)

1. Shipped `PublicExportV1Driver` at `crates/game-test-support/src/profiles.rs:100`.
   Flood's public exporter lives in `src/replay_support.rs`; `tests/replay.rs`
   exists. 602 already touched `replay_support.rs` (serialized chain); 632 `Deps`
   602.
2. Spec §3.9 verdict for Flood `public-export-v1` is `migrate` (public forecast/
   resolved events only, no future deck); §5.12 task `8C-R3-632` scopes the
   driver over the existing exporter and the `public-observer-no-leak` evidence.
3. Cross-artifact boundary under audit: the driver validates metadata and
   delegates to the existing exporter; it creates no new exporter.
4. FOUNDATIONS §11 no-leak + ADR 0004: the public export omits the hidden future
   deck; the driver must not broaden the observer view.
5. Enforcement surface: the driver test in `tests/replay.rs` over the existing
   exporter (export bytes/hash byte-identical to the 001 baseline; future deck
   absent).

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
2. No-leak -> the export carries only public forecast/resolved events; hidden
   future deck absent; export bytes/hash byte-identical to baseline.
3. Round-trip -> `public-replay-export-import` round-trips unchanged.

## What to Change

### 1. Add the public-export-v1 driver test

In `tests/replay.rs`, construct `PublicExportV1Driver`, validate the
public-export metadata, drive the existing exporter, assert public forecast/
resolved events only with no future deck, and import round-trips. Add
wrong-metadata rejection cases.

## Files to Touch

- `games/flood_watch/tests/replay.rs` (modify)

## Out of Scope

- Creating a new exporter or broadening the observer view.
- Per-seat private export (N/A — recorded by 802); other profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p flood_watch` (driver metadata + no-leak + round-trip tests).
2. `cargo run -p replay-check -- --game flood_watch --all` — byte-identical to baseline.

### Invariants

1. The public export omits the hidden future deck; export bytes/hash unchanged.
2. The driver validates metadata then delegates; no new exporter is created.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/replay.rs` — `PublicExportV1Driver` metadata +
   future-deck-no-leak + import-round-trip cases over the existing exporter.

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p replay-check -- --game flood_watch --all`
3. A per-game test + replay-check is the correct boundary: the driver is
   test-side over the existing exporter.

## Outcome

- Added a dev-only `PublicExportV1Driver` wrapper in
  `games/flood_watch/tests/replay.rs` for `public-export-v1` / `v1` with
  `public` visibility, owner `flood_watch`, canonical byte authority `none`,
  and fields `export_steps`, `import_round_trip`, and
  `hidden_absence_tokens`.
- The wrapper delegates to Flood Watch's existing public exporter and asserts
  observer export/import shape, public forecast evidence, and absence of hidden
  future-deck tokens. No exporter, fixture bytes, production code, replay
  behavior, or observer visibility behavior changed.
- Added fail-closed rejection cases for wrong profile, wrong version, invalid
  visibility, wrong owner, and unknown field.
- Verification passed:
  - `cargo test -p flood_watch`
  - `cargo run -p replay-check -- --game flood_watch --all`
  - `git diff --check`
