# 8CR4NSEAPRITRI-033: Vow Tide C-08 replay-command profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/vow_tide/tests/replay.rs`; trace bytes/hash unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

Vow's internal bid/play command traces (representative 3- and 7-seat cases) are not yet validated through the shipped `replay-command-v1` driver (MSC-8C, C-08). Add a virtual `replay-command-v1` adapter around the existing bid/play traces, validating metadata only and leaving trace bytes/hash authoritative (spec §3.9 Vow replay-command, §5.9).

## Assumption Reassessment (2026-06-24)

1. `crates/game-test-support/src/profiles.rs::ReplayCommandV1Driver` exists; Vow's representative 3- and 7-seat bid/play golden traces exist under `games/vow_tide/tests/golden_traces/`. Confirmed during `/reassess-spec`.
2. Spec §3.9 classifies Vow `replay-command-v1` as `migrate`; the driver validates metadata then delegates to the game.
3. Cross-artifact: the evidence-fixture profile contract is owned by `game-test-support`; Vow's trace bytes/hash stay authority. Baseline trace digests come from `-001`.
4. §11/Evidence-Fixture-Contract motivates this ticket: the driver accepts correct metadata, rejects wrong profile/version/field set, and leaves trace bytes unchanged.
5. Enforcement surface = `ReplayCommandV1Driver` metadata validation over representative 3- and 7-seat bid/play traces; no trace rewrite.

## Architecture Check

1. A thin virtual profile adapter is cleaner than inline trace-structure asserts — it routes evidence through the owned driver while Vow owns replay meaning.
2. No backwards-compatibility shim is introduced; no trace byte changes. Rollback removes only the profile adapter.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only (§4/§5).

## Verification Layers

1. Driver accepts correct metadata, rejects wrong profile/version/field set -> schema/serialization validation in Vow replay tests.
2. Physical trace bytes/hash unchanged -> golden trace byte check (`replay-check --game vow_tide --all` byte-identical to baseline).
3. Behavior delegated to the game -> codebase grep-proof (driver validates metadata only).

## What to Change

### 1. Add the `replay-command-v1` profile adapter

In `games/vow_tide/tests/replay.rs`, add a `ReplayCommandV1Driver` adapter validating profile metadata over representative 3- and 7-seat bid/play command traces, delegating replay behavior to Vow.

## Files to Touch

- `games/vow_tide/tests/replay.rs` (modify)

## Out of Scope

- The setup (`-034`) and domain (`-035`) profiles; the public/seat-private export profiles are pilot credit.
- Rewriting any trace to insert profile keys; changing any trace byte.
- Any bid/trick/scoring policy.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` is green, including the `replay-command-v1` driver accept/reject metadata test across 3- and 7-seat traces.
2. `cargo run -p replay-check -- --game vow_tide --all` passes with trace bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The driver validates metadata only and delegates to Vow; physical trace bytes are unchanged.
2. No procedural metadata is inserted into any trace.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/replay.rs` — `replay-command-v1` driver metadata accept/reject over 3- and 7-seat bid/play traces.

### Commands

1. `cargo test -p vow_tide`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The per-game replay test is the correct boundary: the profile is a dev-only evidence adapter over existing traces.

## Outcome

Completed: 2026-06-24

What changed:
- Added a virtual `replay-command-v1` profile artifact and driver tests in `games/vow_tide/tests/replay.rs`.
- Covered Vow's existing L0/L1 bid-play command fixtures plus representative 3-seat and 7-seat golden evidence without rewriting trace bytes.
- Added reject coverage for wrong profile id, wrong profile version, and unknown profile fields.

Deviations:
- None.

Verification:
- `cargo test -p vow_tide replay_command_v1_driver -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p vow_tide`
- `cargo run -p replay-check -- --game vow_tide --all`
- `bash scripts/boundary-check.sh`
