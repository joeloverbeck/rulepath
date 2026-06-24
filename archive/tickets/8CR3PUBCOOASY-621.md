# 8CR3PUBCOOASY-621: C-08 Plain Tricks domain-evidence profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (dev-only profile adapter) — `games/plain_tricks/tests/replay.rs`
**Deps**: 8CR3PUBCOOASY-501

## Problem

C-08 adds a `domain-evidence-v1` profile driver for `plain_tricks`: a dev-only
`DomainEvidenceV1Driver` that validates domain-profile metadata/fields against
the standard fixture (deck partition, hand/tail counts, trick/round invariants
and expected outcomes) and delegates the trick algorithm to the game-owned
adapter. The driver runs no trick algorithm. Domain evidence is the center of
R3's fixture work.

## Assumption Reassessment (2026-06-24)

1. Shipped `DomainEvidenceV1Driver` at `crates/game-test-support/src/profiles.rs:112`/`:246`.
   `games/plain_tricks/data/fixtures/plain_tricks_standard.fixture.json` exists;
   the dev-dep edge is added by 501. **Plain Tricks has no `tests/rules.rs`** (it
   has `tests/{bots,property,replay,serialization,visibility}.rs`), so the
   domain driver lands in `tests/replay.rs` (the spec's "rule/replay tests"
   resolves to replay tests for this game).
2. Spec §3.9 verdict for Plain `domain-evidence-v1` is `migrate` (deck
   partition/trick-round assertions remain game-owned); §5.11 task `8C-R3-621`
   scopes the driver over deck partition, hand/tail counts, trick/round
   invariants and outcomes.
3. Cross-artifact boundary under audit: the driver validates declared domain
   fields + metadata; the game-owned adapter validates trick meaning. A single
   fixture may feed separate setup (611) and domain (621) adapters only when
   each declares its fields/visibility/owner/N-A surfaces.
4. FOUNDATIONS §2/§5/§11: the trick algorithm stays in `plain_tricks`; the
   fixture stays typed evidence; no trick rule enters the driver.
5. Enforcement surface: the driver test in `tests/replay.rs` over the existing
   fixture (read-only), byte-identical to the 001 baseline.

## Architecture Check

1. A metadata/field-validating driver delegating to the game's trick adapter
   shares profile plumbing without duplicating trick semantics; cleaner than a
   bespoke domain reader that re-implements scoring.
2. No backwards-compatibility alias — new dev-only test driver; fixture
   unchanged.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` validates profile
   shape only.

## Verification Layers

1. Metadata/field gating -> `tests/replay.rs`: valid domain metadata + declared
   fields pass; wrong profile/version/owner/visibility/field rejects.
2. Delegation correctness -> the game-owned adapter decides deck partition/
   trick/round outcomes; driver does not.
3. Fixture integrity -> `fixture-check --game plain_tricks` unchanged.

## What to Change

### 1. Add the domain-evidence-v1 driver test

In `tests/replay.rs`, construct `DomainEvidenceV1Driver`, declare the selected
domain fields (deck partition, hand/tail counts, trick/round invariants,
expected outcomes), validate metadata, and delegate meaning to the game-owned
adapter. Add wrong-metadata rejection cases.

## Files to Touch

- `games/plain_tricks/tests/replay.rs` (modify)

## Out of Scope

- Editing the fixture bytes; the setup (611)/replay-command (601)/export
  (631/641) profiles.
- Any trick algorithm inside the driver.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` (driver metadata/field + rejection tests).
2. `cargo run -p fixture-check -- --game plain_tricks` — unchanged.
3. `cargo run -p rule-coverage -- --game plain_tricks` — unchanged.

### Invariants

1. The driver validates metadata/fields then delegates; trick meaning stays
   game-owned.
2. The fixture is unchanged; no trick algorithm enters the driver.

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/replay.rs` — `DomainEvidenceV1Driver` metadata/field
   validation + rejection cases over the standard fixture (plain has no
   `tests/rules.rs`).

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p fixture-check -- --game plain_tricks`
3. A per-game test + fixture-check is the correct boundary: the driver is
   test-side and read-only over the existing fixture.

## Outcome

Completed: 2026-06-24

- Added a dev-only `DomainEvidenceV1Driver` wrapper in
  `games/plain_tricks/tests/replay.rs` for Plain Tricks
  `domain-evidence-v1` metadata. The test validates `v1` / `internal-dev`
  metadata with owner `plain_tricks`, canonical byte authority `none`, and
  fields `domain_schema_version`, `domain_input`, and `expected_domain`.
- The driver delegates to a Plain Tricks-owned domain validator after metadata
  succeeds. That validator uses existing Rust setup, visibility, command
  validation, state transition, completed-trick, golden-trace, and terminal
  outcome paths to cover the standard fixture deck partition, hand/tail counts,
  first-trick winner, round-close invariants, terminal win, and split outcome.
- Added fail-closed rejection cases for wrong profile, wrong version, invalid
  visibility, wrong owner, and unknown field. No fixture bytes, production code,
  trick algorithm, replay hash, or domain behavior changed.
- Verification:
  - `cargo test -p plain_tricks` passed, including the new
    domain-evidence-v1 wrapper.
  - `cargo run -p fixture-check -- --game plain_tricks` passed; all Plain
    Tricks fixtures passed.
  - `cargo run -p rule-coverage -- --game plain_tricks` passed.
  - `git diff --check` passed.
  - `cargo fmt --all --check` was attempted and failed on pre-existing
    formatting drift outside this ticket's owned surface; only
    `games/plain_tricks/tests/replay.rs` was formatted with `rustfmt`.
