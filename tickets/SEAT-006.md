# SEAT-006: Cross-game seat-label consistency integration test

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `apps/web` integration test (and optional wasm-api assertion); no product code change
**Deps**: SEAT-001, SEAT-002, SEAT-004, SEAT-005

## Problem

The original defect — VIEWER showing "Seat 0".."Seat 3" while the play area showed
"Seat 1".."Seat 4" — existed because no test asserted that the VIEWER label set and
the play-area label set agree for a given game. This ticket adds the regression
guard the series exists to lock in: for every registered game, the labels rendered
on the VIEWER surface equal the labels rendered in the play area, and both equal the
Rust catalog `seat_labels`.

This is the "cover all involved code with integration tests" requirement for the
shared-resolver refactor (SEAT-003/004/005).

## Assumption Reassessment (2026-06-21)

1. After SEAT-001/002, the Rust catalog `seat_labels` is the single source of truth
   per game (default "Seat N"; "Player N" for Race/Directional; custom for the
   multi-seat games). After SEAT-004/005, both web surfaces consume it via the
   shared resolver. The test asserts equality across all three.
2. Confirm the web test harness (the `smoke:ui` / vitest setup under `apps/web`)
   can enumerate registered games and render both a board and the VIEWER for each;
   reuse the existing smoke harness rather than introducing a new runner.
3. Shared boundary under audit: the `{seat,label}` contract across Rust catalog →
   view payload → resolver → both render surfaces. This is the canonical end-state
   path; the test must fail if any surface re-introduces an independent formatter.
8. Adjacent: this ticket adds no product behavior; if enumerating all games is
   impractical in one test, cover at minimum one game per labeling scheme
   (default "Seat N": Briar Circuit; "Player N": Race to N; custom multi-seat:
   River Ledger) and document the coverage.

## Architecture Check

1. A single cross-game equality test is the smallest guard that would have caught
   the reported bug and prevents re-divergence — stronger than per-board snapshots
   that can drift independently.
2. No backwards-compatibility shim; this is test-only.
3. No Rust/legality change; `apps/web` test plus an optional wasm-api catalog
   assertion.

## Verification Layers

1. VIEWER label set == play-area label set per game -> integration test (renders
   both surfaces, asserts equality).
2. Both surfaces == Rust catalog `seat_labels` -> integration test asserts against
   the catalog payload (no TS-side divergence).
3. Briar Circuit specifically renders "Seat 1".."Seat 4" on both surfaces ->
   integration test (the originally-reported case).

## What to Change

### 1. Add the consistency integration test

Add an `apps/web` integration/smoke test that, for each covered game, renders the
VIEWER and the board, collects the seat labels from each, and asserts they equal
each other and the Rust catalog `seat_labels`.

### 2. (Optional) wasm-api catalog assertion

Add/confirm a wasm-api test asserting each game's catalog `seat_labels` matches the
agreed convention, so the Rust source of truth is pinned independently of the web
layer.

## Files to Touch

- `apps/web/src/seatLabelConsistency.test.ts` (new — or extend the existing smoke suite)
- `crates/wasm-api/src/tests.rs` (modify — optional catalog convention assertion)

## Out of Scope

- Any change to label-producing code (SEAT-001..005 own that).

## Acceptance Criteria

### Tests That Must Pass

1. The new consistency test passes for every covered game, including the
   originally-reported Briar Circuit case ("Seat 1".."Seat 4" on both surfaces).
2. `npm --prefix apps/web run smoke:ui`
3. `npm --prefix apps/web run build`
4. `cargo test -p wasm-api` (if the optional Rust assertion is added)

### Invariants

1. VIEWER and play-area labels can never silently diverge again — a divergence
   fails this test.
2. Both web surfaces equal the Rust catalog `seat_labels`.

## Test Plan

### New/Modified Tests

1. `apps/web/src/seatLabelConsistency.test.ts` — per-game VIEWER == play-area ==
   catalog `seat_labels`.
2. `crates/wasm-api/src/tests.rs` — optional per-game catalog convention assertion.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run build`
3. `cargo test -p wasm-api`
