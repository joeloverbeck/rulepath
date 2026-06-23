# UNI8CR2TWOSEA-041: Poker Lite — seat-private-export-v1 profile driver

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/poker_lite/tests/{replay,visibility}.rs`; adopts `game-test-support` `SeatPrivateExportV1Driver` over `seat_0`/`seat_1` viewers
**Deps**: 038

## Problem

Spec §3.9 / task `8C-R2-632`: add a `seat-private-export-v1` profile driver for Poker Lite, invoking `export_public_replay` with `Viewer(seat_0)` and `Viewer(seat_1)`. The own private crest is present for its owner and the opponent's crest absent; showdown/yield phase rules are unchanged. Current export bytes remain the authority (`canonical_byte_authority: none`); no new exporter is created. Shares the export/test surface with `-038` (hence `Deps: 038`).

## Assumption Reassessment (2026-06-23)

1. `games/poker_lite/src/replay_support.rs::export_public_replay(trace, viewer)` accepts a `Viewer` (confirmed line ~287); `SeatPrivateExportV1Driver` (`profiles.rs:104`) and `SEAT_PRIVATE_EXPORT_V1` exist.
2. Spec §3.9/§12.2 + ADR-0004: own-hand-only access preserved; showdown/yield policy unchanged; do not introduce a new exporter to force applicability.
3. Cross-crate boundary under audit: `game-test-support::profiles::SeatPrivateExportV1Driver` — validates metadata and delegates to the existing viewer-scoped exporter; the viewer label is explicit (ADR-0004).
4. Determinism / no-leak: the driver validates each seat's export with byte equality to the `-001` baseline; the owner sees only its own crest, the opponent's stays absent, and no omniscient state is reconstructed (§11, ADR-0004).

## Architecture Check

1. A thin seat-private-export driver over the existing viewer-scoped exporter adds typed per-seat evidence without a new exporter or canonical-byte authority — ADR-0004-faithful.
2. No backwards-compat alias; no export rewrite; no new exporter invented.
3. `engine-core` untouched; the driver is dev-only `game-test-support`.

## Verification Layers

1. Driver validates `seat_0`/`seat_1` exports, rejects wrong id/owner/visibility/fields -> profile driver test (`cargo test -p poker_lite`).
2. Own crest present, opponent crest absent; showdown/yield preserved -> no-leak visibility test (`tests/visibility.rs`).
3. Export bytes/hash unchanged -> deterministic replay-hash check (`replay-check --game poker_lite --all`).

## What to Change

### 1. Add the seat-private-export-v1 driver test

In `tests/replay.rs` (with no-leak assertions in `tests/visibility.rs`), invoke `SeatPrivateExportV1Driver` over `export_public_replay` for `Viewer(seat_0)` and `Viewer(seat_1)`, asserting own-hand-only access, explicit viewer labelling, and wrong-profile/owner/field rejection.

## Files to Touch

- `games/poker_lite/tests/replay.rs` (modify)
- `games/poker_lite/tests/visibility.rs` (modify)

## Out of Scope

- Creating a new seat-private exporter; any export byte rewrite; any showdown/yield policy change.
- The public-export profile (`-038`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` green, including the seat-private-export driver test for both seats.
2. `cargo run -p replay-check -- --game poker_lite --all` — export bytes/hash byte-identical to baseline.

### Invariants

1. Each owner sees only its own crest; the opponent's crest stays absent; no omniscient state is reconstructed.
2. The driver claims no new canonical bytes; showdown/yield policy is unchanged.

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/replay.rs` — `seat-private-export-v1` driver test over `seat_0`/`seat_1`; no-leak assertion in `tests/visibility.rs`.

### Commands

1. `cargo test -p poker_lite`
2. `cargo run -p replay-check -- --game poker_lite --all`
