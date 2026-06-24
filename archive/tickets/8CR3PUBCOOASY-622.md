# 8CR3PUBCOOASY-622: C-08 Flood Watch domain-evidence profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (dev-only profile adapter) — `games/flood_watch/tests/{rules,replay}.rs`
**Deps**: 8CR3PUBCOOASY-502

## Problem

C-08 adds a `domain-evidence-v1` profile driver for `flood_watch`: a dev-only
`DomainEvidenceV1Driver` validating domain-profile metadata/fields against the
standard and deluge fixtures (levee absorption, flood rise/inundation, forecast/
event pressure, role/start-budget and terminal evidence) and delegating all
calculations to the game-owned adapter. The driver runs no game calculation.

## Assumption Reassessment (2026-06-24)

1. Shipped `DomainEvidenceV1Driver` at `crates/game-test-support/src/profiles.rs:112`.
   Flood `tests/rules.rs` and `tests/replay.rs` exist;
   `flood_watch_standard.fixture.json` and `flood_watch_deluge.fixture.json`
   exist; the dev-dep edge is added by 502.
2. Spec §3.9 verdict for Flood `domain-evidence-v1` is `migrate` (levee
   absorption, inundation, forecast/event pressure, role and budget evidence);
   §5.11 task `8C-R3-622` scopes the driver over those domain facts.
3. Cross-artifact boundary under audit: the driver validates declared domain
   fields + metadata; the game-owned adapter validates meaning. The setup (612)
   and domain (622) adapters may share a fixture only with declared fields.
4. FOUNDATIONS §2/§5/§11: all flood/levee/budget calculations stay in
   `flood_watch`; the fixture stays typed evidence; no calculation enters the
   driver.
5. Enforcement surface: the driver test in `tests/rules.rs`/`tests/replay.rs`
   over the existing fixtures (read-only), byte-identical to the 001 baseline.

## Architecture Check

1. A metadata/field-validating driver delegating to the game's domain adapter
   shares profile plumbing without duplicating flood/budget semantics.
2. No backwards-compatibility alias — new dev-only test driver; fixtures
   unchanged.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` validates profile
   shape only.

## Verification Layers

1. Metadata/field gating -> domain metadata + declared fields pass; wrong
   profile/version/owner/visibility/field rejects.
2. Delegation correctness -> the game-owned adapter decides levee/inundation/
   forecast/budget outcomes; driver does not.
3. Fixture integrity -> `fixture-check --game flood_watch` unchanged.

## What to Change

### 1. Add the domain-evidence-v1 driver test

In `tests/rules.rs` (with replay-tie assertions in `tests/replay.rs` as needed),
construct `DomainEvidenceV1Driver`, declare the selected domain fields (levee
absorption, inundation, forecast/event pressure, role/start-budget, terminal),
validate metadata, and delegate meaning to the game-owned adapter. Add
wrong-metadata rejection cases.

## Files to Touch

- `games/flood_watch/tests/rules.rs` (modify)
- `games/flood_watch/tests/replay.rs` (modify; replay-tie assertions only, if needed)

## Out of Scope

- Editing fixture bytes; the setup (612)/replay-command (602)/export (632)
  profiles.
- Any flood/levee/budget calculation inside the driver.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p flood_watch` (driver metadata/field + rejection tests).
2. `cargo run -p fixture-check -- --game flood_watch` — unchanged.
3. `cargo run -p rule-coverage -- --game flood_watch` — unchanged.

### Invariants

1. The driver validates metadata/fields then delegates; all calculations stay
   game-owned.
2. Fixtures are unchanged; no calculation enters the driver.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/rules.rs` — `DomainEvidenceV1Driver` metadata/field
   validation + rejection cases over standard + deluge fixtures.

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p fixture-check -- --game flood_watch`
3. A per-game test + fixture-check is the correct boundary: the driver is
   test-side and read-only over existing fixtures.

## Outcome

Completed: 2026-06-24

- Added a dev-only `DomainEvidenceV1Driver` wrapper in
  `games/flood_watch/tests/rules.rs` for Flood Watch `domain-evidence-v1`
  metadata. The test validates `v1` / `internal-dev` metadata with owner
  `flood_watch`, canonical byte authority `none`, and fields
  `domain_schema_version`, `domain_input`, and `expected_domain`.
- The driver delegates to a Flood Watch-owned domain validator after metadata
  succeeds. That validator uses existing Rust fixture loading, setup, command
  validation, state transition, effect, forecast, role-power, levee,
  inundation, and terminal-outcome paths over the standard and deluge fixtures.
- Added fail-closed rejection cases for wrong profile, wrong version, invalid
  visibility, wrong owner, and unknown field. No fixture bytes, production code,
  flood/levee/budget calculation, replay hash, or domain behavior changed.
- Verification:
  - `cargo test -p flood_watch` passed, including the new
    domain-evidence-v1 wrapper.
  - `cargo run -p fixture-check -- --game flood_watch` passed; all Flood Watch
    fixtures passed.
  - `cargo run -p rule-coverage -- --game flood_watch` passed.
  - `git diff --check` passed.
