# 8CR3PUBCOOASY-611: C-08 Plain Tricks setup-evidence profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/plain_tricks/tests/replay.rs`
**Deps**: 8CR3PUBCOOASY-501

## Problem

C-08 adds a `setup-evidence-v1` profile driver for `plain_tricks`: a dev-only
`SetupEvidenceV1Driver` that validates setup-profile metadata/fields against the
standard fixture (seats/options/variant/deck-partition/deal expectations) and
delegates semantic checks to the game-owned setup adapter. Private deal/tail
facts stay `internal-dev`. The fixture is read-only.

## Assumption Reassessment (2026-06-24)

1. Shipped `SetupEvidenceV1Driver` at `crates/game-test-support/src/profiles.rs:108`/`:211`.
   `games/plain_tricks/data/fixtures/plain_tricks_standard.fixture.json` exists;
   the dev-dep edge is added by 501. `fixture-check` already dispatches
   `plain_tricks` (confirmed).
2. Spec §3.9 verdict for Plain `setup-evidence-v1` is `migrate` (standard
   fixture; `internal-dev` where deal/tail facts are asserted); §5.10 task
   `8C-R3-611` scopes the driver over seats/options/variant/deck-partition/deal
   expectations with private facts test-only.
3. Cross-artifact boundary under audit: the driver validates declared fields +
   profile metadata; the game-owned adapter validates meaning. A single fixture
   may feed separate setup/domain adapters only when each declares its fields,
   visibility, validator owner, and N/A surfaces.
4. FOUNDATIONS §5/§11: the fixture stays typed evidence (no selector/trigger/
   formula); deal/tail facts remain `internal-dev` (no leak).
5. Enforcement surface: the driver test in `tests/replay.rs` over the existing
   fixture (read-only), byte-identical to the 001 baseline.

## Architecture Check

1. A metadata/field-validating driver delegating semantics to the game adapter
   shares the profile plumbing without duplicating setup meaning; cleaner than a
   bespoke fixture reader.
2. No backwards-compatibility alias — new dev-only test driver; fixture
   unchanged.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` validates profile
   metadata/shape only.

## Verification Layers

1. Metadata/field gating -> `tests/replay.rs`: valid setup metadata + declared
   fields pass; wrong profile/version/owner/visibility/field rejects.
2. Delegation correctness -> the game-owned setup adapter validates deck
   partition/deal meaning; driver does not.
3. Fixture integrity -> `fixture-check --game plain_tricks` unchanged; fixture
   bytes read-only.

## What to Change

### 1. Add the setup-evidence-v1 driver test

In `tests/replay.rs`, construct `SetupEvidenceV1Driver`, declare the selected
setup fields with `internal-dev` for deal/tail facts, validate metadata, and
delegate meaning to the game-owned setup adapter. Add wrong-metadata rejection
cases.

## Files to Touch

- `games/plain_tricks/tests/replay.rs` (modify)

## Out of Scope

- Editing the fixture bytes; the replay-command/domain/export profiles.
- Any deck-partition/deal interpretation inside the driver.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` (driver metadata/field + rejection tests).
2. `cargo run -p fixture-check -- --game plain_tricks` — unchanged.

### Invariants

1. The driver validates metadata/fields then delegates; deck/deal meaning stays
   game-owned.
2. Deal/tail facts remain `internal-dev`; the fixture is unchanged.

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/replay.rs` — `SetupEvidenceV1Driver` metadata/field
   validation + rejection cases over the standard fixture.

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p fixture-check -- --game plain_tricks`
3. A per-game test + fixture-check is the correct boundary: the driver is
   test-side and read-only over the existing fixture.
