# 8CR3PUBCOOASY-634: C-08 Event Frontier public-export profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/event_frontier/tests/replay.rs`
**Deps**: 8CR3PUBCOOASY-604

## Problem

C-08 adds a `public-export-v1` profile driver for `event_frontier`: a dev-only
`PublicExportV1Driver` validating public-export profile metadata over the
existing exporter — current/next/resolved history only, deeper-deck absence —
and delegating export production to the game. ADR 0004 keeps the public export
free of the hidden deeper deck.

## Assumption Reassessment (2026-06-24)

1. Shipped `PublicExportV1Driver` at `crates/game-test-support/src/profiles.rs:100`.
   Event's exporter lives in `src/replay_support.rs`; `tests/replay.rs` exists.
   604 already touched `replay_support.rs` (serialized chain); 634 `Deps` 604.
2. Spec §3.9 verdict for Event `public-export-v1` is `migrate` (current/next/
   history only, no deeper deck); §5.12 task `8C-R3-634` scopes the driver over
   the existing exporter and the `replay-export-import-no-deck-leak` evidence.
3. Cross-artifact boundary under audit: the driver validates metadata and
   delegates to the existing exporter; it creates no new exporter.
4. FOUNDATIONS §11 no-leak + ADR 0004: the public export omits the hidden deeper
   deck; the driver must not broaden the observer view.
5. Enforcement surface: the driver test in `tests/replay.rs` over the existing
   exporter (export bytes/hash byte-identical to the 001 baseline; deeper deck
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
2. No-leak -> the export carries only current/next/resolved history; deeper deck
   absent; export bytes/hash byte-identical to baseline.
3. Round-trip -> `replay-export-import-no-deck-leak` round-trips unchanged.

## What to Change

### 1. Add the public-export-v1 driver test

In `tests/replay.rs`, construct `PublicExportV1Driver`, validate the
public-export metadata, drive the existing exporter, assert current/next/
resolved history only with no deeper deck, and import round-trips. Add
wrong-metadata rejection cases.

## Files to Touch

- `games/event_frontier/tests/replay.rs` (modify)

## Out of Scope

- Creating a new exporter or broadening the observer view.
- Per-seat private export (N/A — recorded by 802); other profiles.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` (driver metadata + no-leak + round-trip tests).
2. `cargo run -p replay-check -- --game event_frontier --all` — byte-identical to baseline.

### Invariants

1. The public export omits the hidden deeper deck; export bytes/hash unchanged.
2. The driver validates metadata then delegates; no new exporter is created.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/replay.rs` — `PublicExportV1Driver` metadata +
   deeper-deck-no-leak + import-round-trip cases over the existing exporter.

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p replay-check -- --game event_frontier --all`
3. A per-game test + replay-check is the correct boundary: the driver is
   test-side over the existing exporter.

## Outcome

- Added a dev-only `PublicExportV1Driver` wrapper in
  `games/event_frontier/tests/replay.rs` for `public-export-v1` / `v1` with
  `public` visibility, owner `event_frontier`, canonical byte authority
  `none`, and fields `export_steps`, `import_round_trip`, and
  `hidden_absence_tokens`.
- The wrapper delegates to Event Frontier's existing public exporter, asserting
  observer export/import round-trip, hidden surface/redaction metadata, and
  absence of hidden deeper-deck cards from the export/import surfaces. No
  exporter, fixture bytes, production code, replay behavior, or observer
  visibility behavior changed.
- Added fail-closed rejection cases for wrong profile, wrong version, invalid
  visibility, wrong owner, and unknown field.
- Verification passed:
  - `cargo test -p event_frontier`
  - `cargo run -p replay-check -- --game event_frontier --all`
  - `git diff --check`
