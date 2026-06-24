# 8CR3PUBCOOASY-613: C-08 Frontier Control setup-evidence profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/frontier_control/tests/replay.rs`
**Deps**: 8CR3PUBCOOASY-503

## Problem

C-08 adds a `setup-evidence-v1` profile driver for `frontier_control`: a
dev-only `SetupEvidenceV1Driver` validating setup-profile metadata/fields
against the standard and highlands fixtures (seats/factions/start-sites/units/
graph-shape evidence) and delegating semantic checks to the game-owned setup
adapter. Default visibility is `public` (fully-public game). No movement/scoring
policy in the driver; fixtures read-only.

## Assumption Reassessment (2026-06-24)

1. Shipped `SetupEvidenceV1Driver` at `crates/game-test-support/src/profiles.rs:108`.
   `games/frontier_control/data/fixtures/frontier_control_standard.fixture.json`
   and `frontier_control_highlands.fixture.json` exist; the dev-dep edge is
   added by 503. `fixture-check` already dispatches `frontier_control`
   (confirmed).
2. Spec §3.9 verdict for Frontier `setup-evidence-v1` is `migrate` (standard +
   highlands; default `public`); §5.10 task `8C-R3-613` scopes the driver over
   seats/factions/start-sites/units/graph-shape evidence.
3. Cross-artifact boundary under audit: the driver validates declared fields +
   metadata; the game-owned adapter validates meaning.
4. FOUNDATIONS §5/§11: fixtures stay typed evidence; fully-public so `public`
   visibility is sound (no hidden setup fact).
5. Enforcement surface: the driver test in `tests/replay.rs` over the existing
   fixtures (read-only), byte-identical to the 001 baseline.

## Architecture Check

1. A metadata/field-validating driver delegating semantics to the game adapter
   shares profile plumbing without duplicating setup meaning.
2. No backwards-compatibility alias — new dev-only test driver; fixtures
   unchanged.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` validates profile
   shape only.

## Verification Layers

1. Metadata/field gating -> `tests/replay.rs`: valid setup metadata + declared
   fields pass; wrong profile/version/owner/visibility/field rejects.
2. Delegation correctness -> the game-owned setup adapter validates factions/
   sites/graph-shape meaning; driver does not.
3. Fixture integrity -> `fixture-check --game frontier_control` unchanged.

## What to Change

### 1. Add the setup-evidence-v1 driver test

In `tests/replay.rs`, construct `SetupEvidenceV1Driver` for the standard and
highlands fixtures, declare the selected setup fields with `public` visibility,
validate metadata, and delegate meaning to the game-owned setup adapter. Add
wrong-metadata rejection cases.

## Files to Touch

- `games/frontier_control/tests/replay.rs` (modify)

## Out of Scope

- Editing fixture bytes; the replay-command/domain/export profiles.
- Any movement/scoring interpretation inside the driver.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` (driver metadata/field + rejection tests).
2. `cargo run -p fixture-check -- --game frontier_control` — unchanged.

### Invariants

1. The driver validates metadata/fields then delegates; setup meaning stays
   game-owned.
2. `public` visibility is sound (no hidden setup fact); fixtures unchanged.

## Test Plan

### New/Modified Tests

1. `games/frontier_control/tests/replay.rs` — `SetupEvidenceV1Driver`
   metadata/field validation + rejection cases over standard + highlands
   fixtures.

### Commands

1. `cargo test -p frontier_control`
2. `cargo run -p fixture-check -- --game frontier_control`
3. A per-game test + fixture-check is the correct boundary: the driver is
   test-side and read-only over existing fixtures.
