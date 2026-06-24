# 8CR3PUBCOOASY-623: C-08 Frontier Control domain-evidence profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (dev-only profile adapter) — `games/frontier_control/tests/{rules,replay}.rs`
**Deps**: 8CR3PUBCOOASY-503

## Problem

C-08 adds a `domain-evidence-v1` profile driver for `frontier_control`: a
dev-only `DomainEvidenceV1Driver` validating domain-profile metadata/fields
against the standard and highlands fixtures (site/edge inputs, adjacency
examples, clash/start composition, supply/connectivity, round/final scoring
evidence) and delegating all graph/scoring logic to the game-owned adapter. No
graph or scoring helper is created.

## Assumption Reassessment (2026-06-24)

1. Shipped `DomainEvidenceV1Driver` at `crates/game-test-support/src/profiles.rs:112`.
   Frontier `tests/rules.rs` and `tests/replay.rs` exist;
   `frontier_control_standard.fixture.json` and
   `frontier_control_highlands.fixture.json` exist; the dev-dep edge is added by
   503.
2. Spec §3.9 verdict for Frontier `domain-evidence-v1` is `migrate` (graph/
   topology inputs, adjacency, clash, connectivity, round scoring); §5.11 task
   `8C-R3-623` scopes the driver over those domain facts.
3. Cross-artifact boundary under audit: the driver validates declared domain
   fields + metadata; the game-owned adapter validates graph/scoring meaning.
4. FOUNDATIONS §2/§4/§5/§11: graph/topology/connectivity/scoring stay in
   `frontier_control`; no graph or scoring helper is promoted; the fixture stays
   typed evidence.
5. Enforcement surface: the driver test in `tests/rules.rs`/`tests/replay.rs`
   over the existing fixtures (read-only), byte-identical to the 001 baseline.

## Architecture Check

1. A metadata/field-validating driver delegating to the game's domain adapter
   shares profile plumbing without duplicating graph/scoring semantics.
2. No backwards-compatibility alias — new dev-only test driver; fixtures
   unchanged.
3. `engine-core`/`game-stdlib` untouched; no graph/scoring helper created;
   `game-test-support` validates profile shape only.

## Verification Layers

1. Metadata/field gating -> domain metadata + declared fields pass; wrong
   profile/version/owner/visibility/field rejects.
2. Delegation correctness -> the game-owned adapter decides adjacency/clash/
   connectivity/scoring outcomes; driver does not.
3. Fixture integrity -> `fixture-check --game frontier_control` unchanged.

## What to Change

### 1. Add the domain-evidence-v1 driver test

In `tests/rules.rs` (with replay-tie assertions in `tests/replay.rs` as needed),
construct `DomainEvidenceV1Driver`, declare the selected domain fields (site/edge
inputs, adjacency, clash/start composition, supply/connectivity, round/final
scoring), validate metadata, and delegate meaning to the game-owned adapter. Add
wrong-metadata rejection cases.

## Files to Touch

- `games/frontier_control/tests/rules.rs` (modify)
- `games/frontier_control/tests/replay.rs` (modify; replay-tie assertions only, if needed)

## Out of Scope

- Editing fixture bytes; the setup (613)/replay-command (603)/export (633)
  profiles.
- Any graph/scoring calculation inside the driver; creating a graph/scoring
  helper.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` (driver metadata/field + rejection tests).
2. `cargo run -p fixture-check -- --game frontier_control` — unchanged.
3. `cargo run -p rule-coverage -- --game frontier_control` — unchanged.

### Invariants

1. The driver validates metadata/fields then delegates; graph/scoring stays
   game-owned.
2. Fixtures are unchanged; no graph/scoring helper is created.

## Test Plan

### New/Modified Tests

1. `games/frontier_control/tests/rules.rs` — `DomainEvidenceV1Driver`
   metadata/field validation + rejection cases over standard + highlands
   fixtures.

### Commands

1. `cargo test -p frontier_control`
2. `cargo run -p fixture-check -- --game frontier_control`
3. A per-game test + fixture-check is the correct boundary: the driver is
   test-side and read-only over existing fixtures.
