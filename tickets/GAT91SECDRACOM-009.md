# GAT91SECDRACOM-009: secret_draft rule/property/serialization/visibility tests + standard fixture

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/secret_draft/tests/{rules,property,serialization,visibility}.rs` and the standard fixture; no `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-005, GAT91SECDRACOM-006, GAT91SECDRACOM-007

## Problem

The official-game contract requires the full native test surface beyond the per-module unit tests: integration rule tests, property tests over many seeds/sequences, serialization/unknown-field-rejection tests, and cross-surface no-leak/visibility tests, all anchored on a committed standard fixture. This is the evidence that the rules, scoring, redaction, and serialization hold under breadth, not just example cases.

## Assumption Reassessment (2026-06-08)

1. `games/token_bazaar/tests/` carries `rules.rs, property.rs, replay.rs, serialization.rs, visibility.rs, bots.rs` and a `data/fixtures/` standard fixture (verified). This ticket authors `rules.rs, property.rs, serialization.rs, visibility.rs` (bot tests live in GAT91SECDRACOM-008; replay + golden traces in GAT91SECDRACOM-010); the standard fixture shell from GAT91SECDRACOM-002 is finalized/asserted here.
2. The implemented surfaces from GAT91SECDRACOM-005/006/007 are the system under test. Spec §"Acceptance evidence → Native rules, replay, visibility, and bot evidence" enumerates required coverage: setup, legal choices for both uncommitted seats, already-committed/unavailable-item/stale-token diagnostics, conflict fallback, terminal cap, scoring, tie-breaks (rules); pool-removal count, no-duplicate-awards, terminal within six rounds, score stability, visibility invariants, no panics (property); stable summaries + unknown-field rejection for manifest/variants/fixtures/export/internal trace (serialization); no-leak searches across public/seat views, action trees after commit, previews, diagnostics, effect payloads, public effect text, command summaries, export/import timelines, bot explanations, candidate rankings (visibility).
3. Cross-artifact boundary under audit: the test suite spans state/rules/effects/visibility/replay surfaces. Each invariant maps to a distinct proof; no-leak coverage here is the cross-surface negative test the earlier per-module tickets deferred to this suite.
4. §11 acceptance invariants are the motivating principles: restate before trusting spec — fail-closed serialization (unknown fields rejected by default), deterministic stable summaries, viewer-safe views, no hidden-info leak across all named surfaces. Property tests assert these hold over many seeds without panics.
5. Failing-test protocol applies: never weaken or delete a test to get green; if a test fails, validate intent, locate SUT-vs-test, fix the code (spec Forbidden changes).

## Architecture Check

1. A breadth suite anchored on one committed fixture (counts re-derived from the fixture at test start, not hardcoded) is cleaner and more robust than scattering breadth into per-module unit tests — it gives `rule-coverage` (GAT91SECDRACOM-012) a stable set to map obligations against.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` stays noun-free; tests are game-local. No `game-stdlib` helper.

## Verification Layers

1. Rule coverage -> `tests/rules.rs` integration tests (setup/diagnostics/conflict/terminal/scoring/tie-breaks).
2. Breadth/no-panic -> `tests/property.rs` over many seeds (pool-removal count, no duplicate awards, terminal ≤6 rounds, score stability, visibility invariants).
3. Fail-closed serialization -> `tests/serialization.rs` (stable summaries + unknown-field rejection for manifest/variants/fixtures/export/internal trace).
4. Cross-surface no-leak -> `tests/visibility.rs` searches all spec-named surfaces for unrevealed committed item IDs.

## What to Change

### 1. `tests/rules.rs`

Integration tests for setup, both-seat legal choices, already-committed/unavailable-item/stale-token diagnostics, conflict fallback, terminal cap, scoring components, tie-break ladder.

### 2. `tests/property.rs`

Property tests over many deterministic seeds/legal sequences asserting pool-removal count, no duplicate awards, terminal within six rounds, score stability, visibility invariants, and no panics.

### 3. `tests/serialization.rs`

Stable-summary + unknown-field-rejection tests for manifest, variants, fixtures, viewer-scoped export, and internal trace helpers.

### 4. `tests/visibility.rs`

Cross-surface no-leak tests: search public/seat views, post-commit action trees, previews, diagnostics, effect payloads/text, command summaries, export/import timelines, bot explanations, and candidate rankings for any unrevealed committed item ID.

### 5. Standard fixture

Finalize/assert `games/secret_draft/data/fixtures/secret_draft_standard.fixture.json` (initial state; counts re-derived in tests).

## Files to Touch

- `games/secret_draft/tests/rules.rs` (new)
- `games/secret_draft/tests/property.rs` (new)
- `games/secret_draft/tests/serialization.rs` (new)
- `games/secret_draft/tests/visibility.rs` (new)
- `games/secret_draft/data/fixtures/secret_draft_standard.fixture.json` (modify)

## Out of Scope

- Bot tests (GAT91SECDRACOM-008) and replay/golden-trace tests (GAT91SECDRACOM-010).
- Benchmarks (GAT91SECDRACOM-011).
- Any production-logic change — if a test exposes a bug, fix the SUT per the failing-test protocol (that fix is a consequence of this ticket, not a separate weakening).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft` — all four new integration test files pass.
2. Property tests run many seeds with no panic and all invariants hold.
3. An unknown field in manifest/variants/fixture/export is rejected (fail-closed).

### Invariants

1. No unrevealed committed item ID appears on any surface searched by `tests/visibility.rs` (§11 no-leak).
2. Serialization is deterministic and fail-closed; summaries are stable (§11).

## Test Plan

### New/Modified Tests

1. `games/secret_draft/tests/{rules,property,serialization,visibility}.rs` — per §What to Change.

### Commands

1. `cargo test -p secret_draft`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. `cargo test -p secret_draft` is the correct boundary; cross-game pipeline checks (`simulate`, `replay-check`, `rule-coverage`) land after registration in GAT91SECDRACOM-012.
