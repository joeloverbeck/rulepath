# 8CR4NSEAPRITRI-032: Briar Circuit C-08 seat-private-export profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/briar_circuit/tests/replay.rs`; export bytes/class unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-022

## Problem

Briar's seat-private viewer-timeline export for all four seats is not yet validated through the shipped `seat-private-export-v1` driver (MSC-8C, C-08). Add a `seat-private-export-v1` adapter over `export_viewer_timeline` with `ViewerExportClass::SeatPrivate` for all four seats, pinning owner/non-owner hand and pass-data boundaries with no encoding flip (spec §3.9 Briar seat-private-export, §5.9).

## Assumption Reassessment (2026-06-24)

1. `crates/game-test-support/src/profiles.rs::SeatPrivateExportV1Driver` exists; `games/briar_circuit/src/replay_support.rs::{export_viewer_timeline, import_viewer_timeline}` and `ViewerExportClass::SeatPrivate` (game-local) exist. Confirmed during `/reassess-spec`.
2. Spec §3.9 classifies Briar `seat-private-export-v1` as `migrate`; this ticket `Deps` `-022` (viewer geometry precedes export profile closeout, §11.3).
3. Cross-artifact: the seat-private export evidence contract is owned by `game-test-support`; Briar's export encoding stays authority. Baseline export bytes come from `-001`.
4. §11 no-leak firewall motivates this ticket: each seat's private export carries only that seat's hand/pass data; non-owner exports never carry it.
5. Enforcement surface = `SeatPrivateExportV1Driver` over all four seats; owner/non-owner boundaries pinned; no encoding flip.

## Architecture Check

1. A thin seat-private export adapter is cleaner than per-seat ad-hoc asserts — it routes evidence through the owned driver while Briar owns encoding.
2. No backwards-compatibility shim is introduced; no export byte changes. Rollback removes only the adapter.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only (§4/§5).

## Verification Layers

1. All four seats validated, wrong metadata rejected -> schema/serialization validation via `SeatPrivateExportV1Driver`.
2. Owner-only hand/pass data + non-owner absence -> no-leak visibility test over each seat export.
3. Export encoding unchanged -> golden trace byte check (`replay-check --game briar_circuit --all` byte-identical to baseline).

## What to Change

### 1. Add the `seat-private-export-v1` profile adapter

In `games/briar_circuit/tests/replay.rs`, add a `SeatPrivateExportV1Driver` adapter over `export_viewer_timeline` with `ViewerExportClass::SeatPrivate` for all four seats, pinning owner/non-owner hand and pass-data boundaries, delegating export meaning to Briar.

## Files to Touch

- `games/briar_circuit/tests/replay.rs` (modify)

## Out of Scope

- The replay-command (`-029`), setup (`-030`), and public export (`-031`) profiles; the domain profile is pilot credit.
- Any export byte/class flip.
- Any reveal/scoring policy change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including the `seat-private-export-v1` per-seat driver coverage for all four seats.
2. `cargo run -p replay-check -- --game briar_circuit --all` passes with export bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Each seat's private export carries only its hand/pass data; non-owner exports never carry it; encoding unchanged.
2. The driver validates metadata only and delegates to Briar.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/replay.rs` — `seat-private-export-v1` per-seat driver coverage for the four seats.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The per-game replay test is the correct boundary: the profile is a dev-only evidence adapter over the existing seat-private export.
