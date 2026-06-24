# 8CR3PUBCOOASY-614: C-08 Event Frontier setup-evidence profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/event_frontier/tests/replay.rs`
**Deps**: 8CR3PUBCOOASY-504

## Problem

C-08 adds a `setup-evidence-v1` profile driver for `event_frontier`: a dev-only
`SetupEvidenceV1Driver` validating setup-profile metadata/fields against the
standard, hard-winter, and land-rush fixtures (seats/factions/epoch/deck/start-
resources/sites evidence) and delegating semantic checks to the game-owned setup
adapter. Hidden deck-order facts stay `internal-dev`; fixtures read-only.

## Assumption Reassessment (2026-06-24)

1. Shipped `SetupEvidenceV1Driver` at `crates/game-test-support/src/profiles.rs:108`.
   `games/event_frontier/data/fixtures/event_frontier_standard.fixture.json`,
   `event_frontier_hard_winter.fixture.json`, and
   `event_frontier_land_rush.fixture.json` exist; the dev-dep edge is added by
   504. `fixture-check` already dispatches `event_frontier` (confirmed).
2. Spec §3.9 verdict for Event `setup-evidence-v1` is `migrate` (standard +
   hard-winter + land-rush; `internal-dev` where hidden deck order is asserted);
   §5.10 task `8C-R3-614` scopes the driver over seats/factions/epoch/deck/
   start-resources/sites.
3. Cross-artifact boundary under audit: the driver validates declared fields +
   metadata; the game-owned adapter validates meaning.
4. FOUNDATIONS §5/§11: fixtures stay typed evidence; hidden deck facts stay
   `internal-dev` (no leak).
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
   epoch/deck/resources meaning; driver does not.
3. Fixture integrity -> `fixture-check --game event_frontier` unchanged; hidden
   deck facts kept `internal-dev`.

## What to Change

### 1. Add the setup-evidence-v1 driver test

In `tests/replay.rs`, construct `SetupEvidenceV1Driver` for the standard,
hard-winter, and land-rush fixtures, declare the selected setup fields with
`internal-dev` for hidden deck-order facts, validate metadata, and delegate
meaning to the game-owned setup adapter. Add wrong-metadata rejection cases.

## Files to Touch

- `games/event_frontier/tests/replay.rs` (modify)

## Out of Scope

- Editing fixture bytes; the replay-command/domain/export profiles.
- Any epoch/deck/resource interpretation inside the driver.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` (driver metadata/field + rejection tests).
2. `cargo run -p fixture-check -- --game event_frontier` — unchanged.

### Invariants

1. The driver validates metadata/fields then delegates; setup meaning stays
   game-owned.
2. Hidden deck-order facts remain `internal-dev`; fixtures unchanged.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/replay.rs` — `SetupEvidenceV1Driver`
   metadata/field validation + rejection cases over standard + hard-winter +
   land-rush fixtures.

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p fixture-check -- --game event_frontier`
3. A per-game test + fixture-check is the correct boundary: the driver is
   test-side and read-only over existing fixtures.
