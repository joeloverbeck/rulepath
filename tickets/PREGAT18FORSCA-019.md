# PREGAT18FORSCA-019: check-scaffolding-governance test suite + synthetic fixtures

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes (tooling/audit tests) — `scripts/check-scaffolding-governance.test.mjs` + `scripts/testdata/scaffolding-governance/**` (new)
**Deps**: PREGAT18FORSCA-018

## Problem

The governance checker must be proven to pass on valid receipts and fail closed on every violation class, with negatives for legitimate local/test repetition (no false positives). This ticket adds the Node test suite and minimal synthetic repository fixtures exercising each case.

## Assumption Reassessment (2026-06-25)

1. `scripts/check-scaffolding-governance.mjs` (PREGAT18FORSCA-018) is the unit under test. Node `v24.16.0` provides the built-in `node --test` runner and `node:test` module; there is no existing `scripts/*.test.mjs` precedent, so this establishes that convention (no new dependency required). Verified this session.
2. The checker reads `ci/scaffolding-audits.json` + `ci/games.json` + register markdown + `games/` dirs; the fixtures supply minimal synthetic stand-ins under `scripts/testdata/scaffolding-governance/<case>/` so cases run without touching the real repo.
3. The spec (D20, plan §6.5–§6.7, §6.10) requires cases: passing receipts; missing-game; stale/missing paths; unknown IDs; unqueued prior site; invalid exception; forbidden legacy claim; and a false-positive case (legitimate behavior-bearing local code outside the fingerprint rule) that must **pass**.
4. FOUNDATIONS §11 (fail-closed validation) and AGENT-DISCIPLINE §4 (failing-test protocol) under audit: no fixture may be weakened to make the series pass; a false negative (a real violation passing) or false positive (legitimate code failing) is a checker bug to fix on the checker side, not the fixture side.

## Architecture Check

1. Synthetic per-case fixture directories keep each test hermetic and deterministic, independent of the evolving real repo — the standard pattern for a fail-closed validator.
2. No backwards-compatibility shim — fixtures are minimal and purpose-built; no fixture encodes game behavior.
3. `engine-core` untouched; fixtures are static test data read only by the checker under `node --test`.

## Verification Layers

1. Fail-closed coverage (§11) → each violation-class fixture asserts a non-zero checker exit / thrown validation error.
2. True-negative coverage → the passing-receipt and false-positive fixtures assert a zero exit (no over-flagging).
3. Determinism → repeated `node --test` runs yield identical pass/fail with no wall-clock/network input.

## What to Change

### 1. Synthetic fixtures

Add `scripts/testdata/scaffolding-governance/<case>/` directories: passing, missing-game, stale/missing-paths, unknown-id, unqueued-prior-site, invalid-exception, forbidden-legacy-claim, and false-positive (legitimate local behavior-bearing code).

### 2. Node test suite

Add `scripts/check-scaffolding-governance.test.mjs` (`node:test`) asserting the checker's exit/verdict per fixture, including the must-pass false-positive case.

## Files to Touch

- `scripts/check-scaffolding-governance.test.mjs` (new)
- `scripts/testdata/scaffolding-governance/**` (new)

## Out of Scope

- Modifying the checker logic (PREGAT18FORSCA-018) — except a checker-side fix if a fixture exposes a true false-positive/false-negative bug (then update the checker, never weaken the fixture).
- Wiring into Gate 1 (PREGAT18FORSCA-020).

## Acceptance Criteria

### Tests That Must Pass

1. `node --test scripts/check-scaffolding-governance.test.mjs` passes (all cases).
2. Each violation-class fixture asserts a checker failure; the passing + false-positive fixtures assert success.
3. No fixture is weakened to close the series (failing-test protocol upheld).

### Invariants

1. Every violation class fails closed; legitimate local/test repetition passes.
2. Fixtures encode no game behavior.

## Test Plan

### New/Modified Tests

1. `scripts/check-scaffolding-governance.test.mjs` — node:test suite asserting checker exit/verdict per fixture.
2. `scripts/testdata/scaffolding-governance/**` — minimal synthetic fixtures for each pass/fail/false-positive case.

### Commands

1. `node --test scripts/check-scaffolding-governance.test.mjs`
2. `node scripts/check-scaffolding-governance.mjs` (still green against the real repo)
