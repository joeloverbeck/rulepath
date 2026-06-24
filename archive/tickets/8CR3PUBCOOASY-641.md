# 8CR3PUBCOOASY-641: C-08 Plain Tricks seat-private export profile driver

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (dev-only profile adapter) — `games/plain_tricks/tests/replay.rs`
**Deps**: 8CR3PUBCOOASY-631

## Problem

`plain_tricks` is the only R3 game with a real seat-private export. C-08 adds a
`seat-private-export-v1` profile driver: a dev-only `SeatPrivateExportV1Driver`
run for both labelled viewers (`Viewer(seat_0)` and `Viewer(seat_1)`) over the
existing viewer-scoped exporter — each viewer sees its own hand at each step,
opponent/tail absent, with pairwise no-leak and import round-trip (if currently
supported). Flood/Frontier/Event have no per-seat private timeline (three N/As
recorded by 802).

## Assumption Reassessment (2026-06-24)

1. Shipped `SeatPrivateExportV1Driver` at `crates/game-test-support/src/profiles.rs:104`/`:176`.
   Plain's viewer-scoped exporter is `export_public_replay` /
   the viewer-scoped path in `src/replay_support.rs`; `tests/replay.rs` exists.
   631 already drove the exporter (serialized chain); 641 `Deps` 631.
2. Spec §3.9 verdict for Plain `seat-private-export-v1` is `migrate` (run for
   both labelled viewers with pairwise no-leak); the other three games are
   `not-applicable` (recorded by 802). §5.13 task `8C-R3-641` scopes the driver.
3. Cross-artifact boundary under audit: the driver validates metadata and runs
   the existing exporter per labelled viewer; it creates no new exporter and
   reconstructs no omniscient state.
4. FOUNDATIONS §11 no-leak firewall + ADR 0004: each viewer's seat-private
   export shows only that viewer's hand; the opponent's hand and the hidden tail
   are absent; the driver must not broaden a viewer's scope.
5. Enforcement surface: the driver test in `tests/replay.rs` (each labelled
   viewer's export shows own hand, opponent/tail absent; pairwise no-leak; export
   bytes/hash byte-identical to the 001 baseline; round-trip if supported).

## Architecture Check

1. A metadata-validating driver running the existing exporter per labelled viewer
   shares the profile plumbing without duplicating viewer-scoped export logic.
2. No backwards-compatibility alias — new dev-only test driver; exporter and
   policy unchanged.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` validates profile
   metadata only.

## Verification Layers

1. Metadata gating -> `tests/replay.rs`: valid seat-private metadata (labelled
   viewer) passes; wrong profile/version/owner/visibility/field rejects.
2. Pairwise no-leak -> each viewer's export shows only its own hand; opponent
   hand and hidden tail absent; export bytes/hash byte-identical to baseline.
3. Round-trip -> import round-trips unchanged where currently supported.

## What to Change

### 1. Add the seat-private-export-v1 driver test

In `tests/replay.rs`, construct `SeatPrivateExportV1Driver`, run it for
`Viewer(seat_0)` and `Viewer(seat_1)` over the existing viewer-scoped exporter,
assert own-hand presence + opponent/tail absence + pairwise no-leak, and import
round-trip if supported. Add wrong-metadata rejection cases.

## Files to Touch

- `games/plain_tricks/tests/replay.rs` (modify)

## Out of Scope

- Creating a new exporter or broadening any viewer's scope.
- Any seat-private export for Flood/Frontier/Event (N/A — recorded by 802).
- Reconstructing omniscient state.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` (both-viewer driver + pairwise no-leak tests).
2. `cargo run -p replay-check -- --game plain_tricks --all` — byte-identical to baseline.

### Invariants

1. Each viewer's seat-private export shows only its own hand; opponent/tail
   absent; export bytes/hash unchanged from baseline.
2. The driver validates metadata then delegates; no new exporter is created.

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/replay.rs` — `SeatPrivateExportV1Driver` for both
   labelled viewers with pairwise no-leak + round-trip cases.

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all`
3. A per-game test + replay-check is the correct boundary: the driver is
   test-side over the existing viewer-scoped exporter.

## Outcome

- Added a dev-only `SeatPrivateExportV1Driver` wrapper in
  `games/plain_tricks/tests/replay.rs` for `seat-private-export-v1` / `v1`
  with `seat-private` visibility, owner `plain_tricks`, canonical byte
  authority `none`, and fields `viewer_seat`, `viewer_seat_version`,
  `export_steps`, and `pairwise_no_leak`.
- The wrapper delegates to Plain Tricks' existing viewer-scoped exporter for
  both labelled viewers, asserting each viewer sees its own hand, the opponent
  hand and tail are absent, and import round-trip is preserved. No exporter,
  fixture bytes, production code, or viewer visibility behavior changed.
- Added fail-closed rejection cases for wrong profile, wrong version, invalid
  visibility, wrong owner, and unknown field.
- Verification passed:
  - `cargo test -p plain_tricks`
  - `cargo run -p replay-check -- --game plain_tricks --all`
  - `git diff --check`
