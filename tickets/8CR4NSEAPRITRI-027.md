# 8CR4NSEAPRITRI-027: River Ledger C-08 seat-private-export profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/river_ledger/tests/`; export encoding authority unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-020

## Problem

River's seat-private export for every viewer at counts 3–6 (including a multipot terminal path) is not yet validated through the shipped `seat-private-export-v1` driver (MSC-8C, C-08). Add a `seat-private-export-v1` adapter over every declared viewer, pinning private-to-owner and non-owner absence with no export encoding flip (spec §3.9 River seat-private-export, §5.9).

## Assumption Reassessment (2026-06-24)

1. `crates/game-test-support/src/profiles.rs::SeatPrivateExportV1Driver` exists; River's seat-private export path and the multipot terminal export exist. Confirmed during `/reassess-spec`.
2. Spec §3.9 classifies River `seat-private-export-v1` as `migrate`; this ticket `Deps` `-020` (viewer geometry precedes export profile closeout, §11.3).
3. Cross-artifact: the seat-private export evidence contract is owned by `game-test-support`; River's export encoding stays authority. Baseline export bytes come from `-001`.
4. §11 no-leak firewall motivates this ticket: each seat's private export carries only that seat's authorized data; non-owner exports never carry it, including the multipot terminal path.
5. Enforcement surface = `SeatPrivateExportV1Driver` over every viewer for counts 3–6; private-to-owner and non-owner absence are pinned; no encoding flip.

## Architecture Check

1. A thin seat-private export profile adapter is cleaner than per-viewer ad-hoc asserts — it routes evidence through the owned driver while River owns export encoding.
2. No backwards-compatibility shim is introduced; no export byte changes. Rollback removes only the driver adapter.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only (§4/§5).

## Verification Layers

1. Every viewer 3–6 export validated, wrong metadata rejected -> schema/serialization validation via `SeatPrivateExportV1Driver`.
2. Private-to-owner + non-owner absence incl. multipot terminal path -> no-leak visibility test over each seat export.
3. Export encoding unchanged -> golden trace byte check (`replay-check --game river_ledger --all` byte-identical to baseline).

## What to Change

### 1. Add the `seat-private-export-v1` profile adapter

In `games/river_ledger/tests/replay.rs` (and `tests/visibility.rs` as needed), add a `SeatPrivateExportV1Driver` adapter over every declared viewer for counts 3–6 including a multipot terminal state, pinning private-to-owner data and non-owner absence, delegating export meaning to River.

## Files to Touch

- `games/river_ledger/tests/replay.rs` (modify)

## Out of Scope

- The replay-command (`-025`), public export (`-026`), and domain (`-028`) profiles.
- Any export encoding flip or byte rewrite.
- Any reveal/allocation policy change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` is green, including the `seat-private-export-v1` per-viewer driver coverage for 3–6.
2. `cargo run -p replay-check -- --game river_ledger --all` passes with export bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Each seat's private export carries only its authorized data; non-owner exports never carry it; encoding is unchanged.
2. The driver validates metadata only and delegates to River.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/replay.rs` — `seat-private-export-v1` per-viewer driver coverage for counts 3–6 incl. multipot terminal path.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. The per-game replay/visibility test is the correct boundary: the profile is a dev-only evidence adapter over the existing seat-private export.
