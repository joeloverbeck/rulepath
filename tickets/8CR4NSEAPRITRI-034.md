# 8CR4NSEAPRITRI-034: Vow Tide C-08 setup-evidence profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/vow_tide/tests/`; fixture bytes + schedule/deal policy unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-012

## Problem

Vow's representative 3- and 7-seat schedule/deal setup metadata is not yet validated through the shipped `setup-evidence-v1` driver (MSC-8C, C-08). Add a `setup-evidence-v1` adapter over `vow_tide_3p_standard.fixture.json` and `vow_tide_7p_standard.fixture.json`, validating seat count, schedule label, and deterministic deal evidence while delegating schedule/deal rules to Vow (spec §3.9 Vow setup, §5.9). Hand schedule remains code.

## Assumption Reassessment (2026-06-24)

1. `crates/game-test-support/src/profiles.rs::SetupEvidenceV1Driver` exists; `games/vow_tide/data/fixtures/{vow_tide_3p_standard,vow_tide_7p_standard}.fixture.json` exist. Confirmed during `/reassess-spec`.
2. Spec §3.9 classifies Vow `setup-evidence-v1` as `migrate`; this ticket `Deps` `-012` so the centralized 3–7 range is in place when the profile enumerates the 3- and 7-seat setups.
3. Cross-artifact: the setup-evidence contract is owned by `game-test-support`; Vow owns `hand_schedule_for_seats`/`deal_hand`. Baseline fixture bytes come from `-001`.
4. §5/§11 motivate this ticket: setup metadata is typed evidence only — no static-data formula; deterministic-deal/schedule validation delegates to Vow Rust.
5. Enforcement surface = `SetupEvidenceV1Driver` over the 3- and 7-seat fixtures; no fixture byte changes and the hand schedule stays code.

## Architecture Check

1. A thin setup-evidence adapter delegating to the Vow setup owner is cleaner than re-deriving schedule/deal structure in the driver.
2. No backwards-compatibility shim is introduced; no fixture byte changes. Rollback removes only the adapter.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only and encodes no behavior (§4/§5).

## Verification Layers

1. Driver accepts correct setup metadata (seat count, schedule label, deterministic deal), rejects wrong profile/version/field set -> schema/serialization validation via `SetupEvidenceV1Driver`.
2. Schedule/deal validated through the game owner -> setup test (game-owned, `hand_schedule_for_seats`/`deal_hand`).
3. Fixture bytes + schedule policy unchanged -> `fixture-check --game vow_tide` byte-identical + codebase grep-proof (no static-data formula).

## What to Change

### 1. Add the `setup-evidence-v1` profile adapter

In `games/vow_tide/tests/` (setup test module), add a `SetupEvidenceV1Driver` adapter over the 3- and 7-seat standard fixtures validating seat count, schedule label, and deterministic deal metadata, delegating schedule/deal rules to Vow.

## Files to Touch

- `games/vow_tide/tests/serialization.rs` (modify; or the setup test module)

## Out of Scope

- The replay-command (`-033`) and domain (`-035`) profiles; the public/seat-private export profiles are pilot credit.
- Encoding any schedule/deal formula in fixture/profile metadata; rewriting any fixture.
- Any schedule/deal/dealer policy change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` is green, including the `setup-evidence-v1` driver + game-owned schedule/deal assertion at 3 and 7 seats.
2. `cargo run -p fixture-check -- --game vow_tide` passes with fixture bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The driver validates metadata only; hand schedule/deal stays code; fixture bytes are unchanged.
2. No procedural metadata is inserted into any fixture.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/serialization.rs` — `setup-evidence-v1` driver metadata + game-owned schedule/deal assertion for 3 and 7 seats.

### Commands

1. `cargo test -p vow_tide`
2. `cargo run -p fixture-check -- --game vow_tide`
3. The per-game fixture/setup test is the correct boundary: setup evidence delegates to the game setup owner.
