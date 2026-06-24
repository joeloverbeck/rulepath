# 8CR3PUBCOOASY-631: C-08 Plain Tricks public-export profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/plain_tricks/tests/replay.rs`
**Deps**: 8CR3PUBCOOASY-601

## Problem

C-08 adds a `public-export-v1` profile driver for `plain_tricks`: a dev-only
`PublicExportV1Driver` validating public-export profile metadata over the
existing viewer-scoped public exporter (`export_public_replay`) — public
observer timeline, hidden hand/tail absence, import round-trip — and delegating
export production to the game. ADR 0004 keeps public observer export distinct
from internal-full and seat-private exports.

## Assumption Reassessment (2026-06-24)

1. Shipped `PublicExportV1Driver` at `crates/game-test-support/src/profiles.rs:100`/`:146`.
   Plain's viewer-scoped exporter `export_public_replay` is at
   `games/plain_tricks/src/replay_support.rs:315`; `tests/replay.rs` exists.
   601 already touched `replay_support.rs` (serialized chain); 631 `Deps` 601.
2. Spec §3.9 verdict for Plain `public-export-v1` is `migrate` (public observer
   timeline, no private hand/tail); §5.12 task `8C-R3-631` scopes the driver
   over the existing exporter, import round-trip, and current export bytes/hash.
3. Cross-artifact boundary under audit: the driver validates metadata and
   delegates to the existing exporter; it creates no new exporter and adds no
   omniscient state.
4. FOUNDATIONS §11 no-leak + ADR 0004: the public export omits hidden hand/tail;
   the driver must not broaden the observer view.
5. Enforcement surface: the driver test in `tests/replay.rs` over the existing
   exporter (export bytes/hash byte-identical to the 001 baseline; hidden
   hand/tail absent; import round-trip preserved).

## Architecture Check

1. A metadata-validating driver delegating to the existing exporter shares the
   profile plumbing without duplicating export logic; cleaner than a new
   exporter.
2. No backwards-compatibility alias — new dev-only test driver; exporter
   unchanged.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` validates profile
   metadata only.

## Verification Layers

1. Metadata gating -> `tests/replay.rs`: valid public-export metadata passes;
   wrong profile/version/owner/visibility/field rejects.
2. No-leak -> the observer timeline omits hidden hand/tail; export bytes/hash
   byte-identical to the 001 baseline.
3. Round-trip -> `public-replay-export-import` round-trips unchanged.

## What to Change

### 1. Add the public-export-v1 driver test

In `tests/replay.rs`, construct `PublicExportV1Driver`, validate the
public-export metadata, drive the existing `export_public_replay`, assert the
observer timeline omits hidden hand/tail and the import round-trips. Add
wrong-metadata rejection cases.

## Files to Touch

- `games/plain_tricks/tests/replay.rs` (modify)

## Out of Scope

- Creating a new exporter or broadening the observer view.
- The seat-private export profile (641); other profiles.
- Reconstructing omniscient state.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` (driver metadata + no-leak + round-trip tests).
2. `cargo run -p replay-check -- --game plain_tricks --all` — byte-identical to baseline.

### Invariants

1. The public export omits hidden hand/tail; export bytes/hash unchanged from
   baseline.
2. The driver validates metadata then delegates; no new exporter is created.

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/replay.rs` — `PublicExportV1Driver` metadata +
   observer-no-leak + import-round-trip cases over the existing exporter.

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all`
3. A per-game test + replay-check is the correct boundary: the driver is
   test-side over the existing exporter.
