# 8CR3PUBCOOASY-624: C-08 Event Frontier domain-evidence profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (dev-only profile adapter) — `games/event_frontier/tests/{rules,replay}.rs`
**Deps**: 8CR3PUBCOOASY-504

## Problem

C-08 adds a `domain-evidence-v1` profile driver for `event_frontier`: a dev-only
`DomainEvidenceV1Driver` validating domain-profile metadata/fields against the
standard, hard-winter, and land-rush fixtures (site/trail inputs, event/edict
cases, eligibility, operation funding/pass/Reckoning income, caps and scoring
evidence) and delegating all logic to the game-owned adapter. No event/budget/
scoring DSL or helper is created.

## Assumption Reassessment (2026-06-24)

1. Shipped `DomainEvidenceV1Driver` at `crates/game-test-support/src/profiles.rs:112`.
   Event `tests/rules.rs` and `tests/replay.rs` exist; the standard, hard-winter,
   and land-rush fixtures exist; the dev-dep edge is added by 504.
2. Spec §3.9 verdict for Event `domain-evidence-v1` is `migrate` (graph/event/
   edict/funding/resource/Reckoning evidence); §5.11 task `8C-R3-624` scopes the
   driver over those domain facts.
3. Cross-artifact boundary under audit: the driver validates declared domain
   fields + metadata; the game-owned adapter validates event/resource meaning.
4. FOUNDATIONS §2/§5/§11: event/edict resolution, eligibility, funding, caps and
   scoring stay in `event_frontier`; no DSL/helper is created; fixtures stay
   typed evidence.
5. Enforcement surface: the driver test in `tests/rules.rs`/`tests/replay.rs`
   over the existing fixtures (read-only), byte-identical to the 001 baseline.

## Architecture Check

1. A metadata/field-validating driver delegating to the game's domain adapter
   shares profile plumbing without duplicating event/resource semantics.
2. No backwards-compatibility alias — new dev-only test driver; fixtures
   unchanged.
3. `engine-core`/`game-stdlib` untouched; no event/budget/scoring DSL or helper
   created; `game-test-support` validates profile shape only.

## Verification Layers

1. Metadata/field gating -> domain metadata + declared fields pass; wrong
   profile/version/owner/visibility/field rejects.
2. Delegation correctness -> the game-owned adapter decides event/edict/
   eligibility/funding/Reckoning outcomes; driver does not.
3. Fixture integrity -> `fixture-check --game event_frontier` unchanged.

## What to Change

### 1. Add the domain-evidence-v1 driver test

In `tests/rules.rs` (with replay-tie assertions in `tests/replay.rs` as needed),
construct `DomainEvidenceV1Driver`, declare the selected domain fields (site/
trail inputs, event/edict cases, eligibility, funding/pass/Reckoning income,
caps and scoring), validate metadata, and delegate meaning to the game-owned
adapter. Add wrong-metadata rejection cases.

## Files to Touch

- `games/event_frontier/tests/rules.rs` (modify)
- `games/event_frontier/tests/replay.rs` (modify; replay-tie assertions only, if needed)

## Out of Scope

- Editing fixture bytes; the setup (614)/replay-command (604)/export (634)
  profiles.
- Any event/budget/scoring calculation, DSL, or helper inside the driver.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` (driver metadata/field + rejection tests).
2. `cargo run -p fixture-check -- --game event_frontier` — unchanged.
3. `cargo run -p rule-coverage -- --game event_frontier` — unchanged.

### Invariants

1. The driver validates metadata/fields then delegates; event/resource/scoring
   stays game-owned.
2. Fixtures are unchanged; no event/budget/scoring DSL or helper is created.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/rules.rs` — `DomainEvidenceV1Driver`
   metadata/field validation + rejection cases over standard + hard-winter +
   land-rush fixtures.

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p fixture-check -- --game event_frontier`
3. A per-game test + fixture-check is the correct boundary: the driver is
   test-side and read-only over existing fixtures.

## Outcome

- Added a dev-only `DomainEvidenceV1Driver` wrapper in
  `games/event_frontier/tests/rules.rs` for `domain-evidence-v1` / `v1` with
  `internal-dev` visibility, owner `event_frontier`, canonical byte authority
  `none`, and fields `domain_schema_version`, `domain_input`, and
  `expected_domain`.
- The wrapper delegates to Event Frontier-owned Rust validation over the
  standard, hard-winter, and land-rush fixtures plus event/edict resolution,
  eligibility/pass flow, operation funding/bounds, Reckoning income, and
  terminal scoring. No fixture bytes, production code, event/resource/scoring
  DSL, or shared domain helper changed.
- Added fail-closed rejection cases for wrong profile, wrong version, invalid
  visibility, wrong owner, and unknown field.
- Verification passed:
  - `cargo test -p event_frontier`
  - `cargo run -p fixture-check -- --game event_frontier`
  - `cargo run -p rule-coverage -- --game event_frontier`
  - `git diff --check`
