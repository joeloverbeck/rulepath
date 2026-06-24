# 8CR3PUBCOOASY-612: C-08 Flood Watch setup-evidence profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/flood_watch/tests/replay.rs`
**Deps**: 8CR3PUBCOOASY-502

## Problem

C-08 adds a `setup-evidence-v1` profile driver for `flood_watch`: a dev-only
`SetupEvidenceV1Driver` validating setup-profile metadata/fields against the
standard and deluge fixtures (seats/role-order cardinality/scenario/start-state
evidence) and delegating semantic checks to the game-owned setup adapter.
Default visibility is `public` unless test-only deck facts require
`internal-dev`. No role behavior in the driver; fixtures read-only.

## Assumption Reassessment (2026-06-24)

1. Shipped `SetupEvidenceV1Driver` at `crates/game-test-support/src/profiles.rs:108`.
   `games/flood_watch/data/fixtures/flood_watch_standard.fixture.json` and
   `flood_watch_deluge.fixture.json` exist; the dev-dep edge is added by 502.
   `fixture-check` already dispatches `flood_watch` (confirmed).
2. Spec §3.9 verdict for Flood `setup-evidence-v1` is `migrate` (standard +
   deluge; default `public` unless deck facts require `internal-dev`); §5.10
   task `8C-R3-612` scopes the driver over seats/role-order/scenario/start-state.
3. Cross-artifact boundary under audit: the driver validates declared fields +
   metadata; the game-owned adapter validates meaning; a single fixture may feed
   separate setup/domain adapters only with declared fields/visibility/owner.
4. FOUNDATIONS §5/§11: fixtures stay typed evidence; deck facts (if asserted)
   stay `internal-dev` (no leak).
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
2. Delegation correctness -> the game-owned setup adapter validates role-order/
   scenario/start-state meaning; driver does not.
3. Fixture integrity -> `fixture-check --game flood_watch` unchanged.

## What to Change

### 1. Add the setup-evidence-v1 driver test

In `tests/replay.rs`, construct `SetupEvidenceV1Driver` for the standard and
deluge fixtures, declare the selected setup fields (visibility per the spec),
validate metadata, and delegate meaning to the game-owned setup adapter. Add
wrong-metadata rejection cases.

## Files to Touch

- `games/flood_watch/tests/replay.rs` (modify)

## Out of Scope

- Editing fixture bytes; the replay-command/domain/export profiles.
- Any role/scenario interpretation inside the driver.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p flood_watch` (driver metadata/field + rejection tests).
2. `cargo run -p fixture-check -- --game flood_watch` — unchanged.

### Invariants

1. The driver validates metadata/fields then delegates; setup meaning stays
   game-owned.
2. Deck facts (if asserted) remain `internal-dev`; fixtures unchanged.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/replay.rs` — `SetupEvidenceV1Driver` metadata/field
   validation + rejection cases over standard + deluge fixtures.

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p fixture-check -- --game flood_watch`
3. A per-game test + fixture-check is the correct boundary: the driver is
   test-side and read-only over existing fixtures.
