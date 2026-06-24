# PREGAT18FORSCA-020: wire scaffolding-governance check into Gate 1

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (CI) — `.github/workflows/gate-1-game-smoke.yml` (modify)
**Deps**: PREGAT18FORSCA-019

## Problem

The governance checker exists but is not run in CI, so the obligation is not enforced. This ticket adds one `repo-checks` step running `node scripts/check-scaffolding-governance.mjs`, wired at the position that makes Gate 1 go green in one PR (the checker, receipt, and tests already landed via the chain 017←018←019).

## Assumption Reassessment (2026-06-25)

1. `.github/workflows/gate-1-game-smoke.yml` has a `repo-checks` job. After the `Engine boundary` step (`bash scripts/boundary-check.sh`, L170-171) come four smoke/e2e steps (WASM smoke, race_to_n UI smoke, Static preview smoke, Cross-cutting browser e2e), then `Docs link check` (`node scripts/check-doc-links.mjs`, ~L191) and `Catalog/docs drift check`. Verified this session.
2. Per reassess finding M1, the new step is inserted **immediately before the `Docs link check` step**, grouping it with the other Node `scripts/check-*.mjs` repo/doc checks — not literally adjacent to `Engine boundary` (the two are ~20 lines / 4 steps apart). The chain `017←018←019` guarantees the checker, receipt, and tests exist before this wiring, so no multi-PR red window opens.
3. The spec (D21, plan §6.1, §6.10) requires: one `repo-checks` step (`node scripts/check-scaffolding-governance.mjs`); Gate 0 and Gate 2 unchanged.
4. FOUNDATIONS §11 (CI enforcement is fail-closed/blocking) under audit: the step is a normal blocking CI step with no bypass; it makes the governance obligation enforced rather than advisory.

## Architecture Check

1. Placing the step with the other Node repo/doc checks (immediately before `Docs link check`) groups it with its conceptual siblings (`boundary-check.sh`, `check-doc-links.mjs`, `check-catalog-docs.mjs`) and avoids interleaving a repo-wide check among the browser smoke/e2e steps.
2. No backwards-compatibility shim — one additive step; no conditional/bypass.
3. `engine-core` untouched; Gate 0 (hygiene) and Gate 2 (benchmarks) workflows are not edited.

## Verification Layers

1. Step presence/position → grep-proof the new step sits immediately before `Docs link check` in `repo-checks`.
2. Green run → the step runs `node scripts/check-scaffolding-governance.mjs` and passes against the committed repo.
3. Scope containment → Gate 0 / Gate 2 workflow files are unchanged (grep-proof).

## What to Change

### 1. Gate 1 repo-checks step

Insert, immediately before the `Docs link check` step in the `repo-checks` job:

```yaml
- name: Mechanical scaffolding governance
  run: node scripts/check-scaffolding-governance.mjs
```

## Files to Touch

- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- Editing `.github/workflows/gate-0-hygiene.yml` or `.github/workflows/gate-2-benchmarks.yml`.
- Any env/branch-label/comment bypass for the step.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "Mechanical scaffolding governance" .github/workflows/gate-1-game-smoke.yml` confirms the step immediately before `Docs link check`.
2. `node scripts/check-scaffolding-governance.mjs` exits 0 (green) against the committed repo.
3. Gate 0 and Gate 2 workflow files are unchanged (`git diff --name-only` shows only the Gate 1 file).

### Invariants

1. The governance check is a blocking Gate 1 step with no bypass.
2. Gate 0 and Gate 2 are untouched.

## Test Plan

### New/Modified Tests

1. `None — CI-wiring only; the checker and its test suite land in PREGAT18FORSCA-018/019. Verification is the green step run.`

### Commands

1. `node scripts/check-scaffolding-governance.mjs`
2. `node --test scripts/check-scaffolding-governance.test.mjs` (the suite the new step's checker is proven by)
3. `git diff -- .github/workflows/gate-1-game-smoke.yml` (review: one additive step before `Docs link check`)
