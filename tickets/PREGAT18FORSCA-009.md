# PREGAT18FORSCA-009: TESTING governance-check section + §17 CI-expectations bullet

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance/area-doc edit (`docs/TESTING-REPLAY-BENCHMARKING.md`)
**Deps**: PREGAT18FORSCA-002

## Problem

`TESTING-REPLAY-BENCHMARKING.md` §17 lists CI expectations but says nothing about the mechanical-scaffolding governance check. This ticket adds a governance-check section (its two enforcement layers, false-positive controls, no-bypass clause, ADR-0009 deferral) and a §17 CI-expectations bullet, so the testing law documents the checker that PREGAT18FORSCA-018/020 implement and wire.

## Assumption Reassessment (2026-06-25)

1. `docs/TESTING-REPLAY-BENCHMARKING.md` §17 "CI expectations" is at L474 (CI-lane bullets L476–505). No "Mechanical-scaffolding governance check" section exists yet. Verified this session.
2. The spec (D9, plan §5.9) requires a "Mechanical-scaffolding governance check" section (two enforcement layers, false-positive controls, no-bypass clause, ADR-0009 deferral) immediately before/after §17, plus a §17 CI-expectations bullet; final §-numbering normalized at reassess (the reassessed spec keeps §17 the anchor).
3. Shared contract under audit: the testing-law description of the checker's two layers (receipt/register/tracker validation + known-shape fingerprints) and the no-bypass clause that the checker (PREGAT18FORSCA-018) must honor (no env/label/comment bypass).
4. FOUNDATIONS §11 (fail-closed, blocking validation; deterministic) under audit: the section documents the check as deterministic, blocking, and non-overridable — it describes machinery; it introduces no leak or nondeterminism itself. This is doctrine describing a checker that lands later (PREGAT18FORSCA-018), so its prose references the script by path/name (not a markdown doc-link), avoiding a doc-link red window.

## Architecture Check

1. Documenting the governance check in the testing law (not only the register) co-locates it with the other CI-lane expectations, so a reader auditing CI sees it alongside boundary/doc-link/catalog checks.
2. No backwards-compatibility shim — the section and bullet are additive.
3. `engine-core`/`game-stdlib` discipline untouched; the section reaffirms fail-closed, no-bypass validation per FOUNDATIONS §11.

## Verification Layers

1. Section presence → grep-proof the governance-check section names the two enforcement layers, false-positive controls, no-bypass clause, and ADR-0009 deferral.
2. §17 bullet → grep-proof §17 lists the mechanical-scaffolding governance CI step.
3. Doc-link integrity → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Mechanical-scaffolding governance check section

Add the section (plan §5.9 draft) adjacent to §17: the two enforcement layers (receipt/register/tracker validation; known-shape fingerprints), false-positive controls, the no-bypass clause, and the ADR-0009 deferral for any byte/hash/visibility migration.

### 2. §17 CI-expectations bullet

Add the bullet naming `node scripts/check-scaffolding-governance.mjs` as a Gate 1 repo check.

## Files to Touch

- `docs/TESTING-REPLAY-BENCHMARKING.md` (modify)

## Out of Scope

- Implementing the checker (PREGAT18FORSCA-018) or wiring it into the workflow (PREGAT18FORSCA-020).
- Any change to existing CI-lane bullets beyond the additive governance entry.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -ni "scaffolding governance" docs/TESTING-REPLAY-BENCHMARKING.md` ≥ 1 (section + §17 bullet).
2. The section states the no-bypass clause and ADR-0009 deferral (grep-proof).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The check is documented as deterministic, fail-closed, blocking, and non-overridable.
2. Byte/hash/visibility migration is deferred to ADR 0009.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- docs/TESTING-REPLAY-BENCHMARKING.md` (review: governance section + §17 bullet)
3. `grep -n "no-bypass\|ADR 0009\|ADR-0009" docs/TESTING-REPLAY-BENCHMARKING.md`
