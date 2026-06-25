# PREGAT18FORSCA-004: ENGINE-GAME-DATA-BOUNDARY §13A forward-conformance boundary

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — governance/area-doc edit (`docs/ENGINE-GAME-DATA-BOUNDARY.md`)
**Deps**: PREGAT18FORSCA-002

## Problem

`ENGINE-GAME-DATA-BOUNDARY.md` §13 describes the behavioral `game-stdlib` promotion boundary, but there is no parallel forward mechanical-scaffolding conformance boundary. This ticket adds §13A so the boundary doc states the forward reuse-first-audit / register-new / queue-or-dispose obligation alongside (not replacing) the behavioral §13 process.

## Assumption Reassessment (2026-06-25)

1. `docs/ENGINE-GAME-DATA-BOUNDARY.md` §13 "`game-stdlib` promotion boundary" is at L295 and §14 "Future DSL policy" is at L319. Verified this session — §13A must land between them.
2. The spec (D4, plan §5.4) requires a new §13A "Forward mechanical-scaffolding conformance boundary" after §13, before §14, parallel to the behavioral §13 process.
3. Shared contract under audit: the boundary doc's section spine (§13 behavioral, §13A forward-scaffolding, §14 DSL) — the insertion must not renumber §14.
4. FOUNDATIONS §3 (`engine-core` contract kernel) and §5 (static data is not behavior) under audit: §13A keeps the scaffolding lane on the behavior-free side of the boundary; the receipt it references (`ci/scaffolding-audits.json`) is static evidence, not loaded by any game/WASM path.

## Architecture Check

1. A parallel §13A (rather than editing §13) keeps the behavioral promotion boundary and the mechanical-scaffolding lane visibly distinct — the same separation FOUNDATIONS §4 draws between the mechanic atlas and the scaffolding register.
2. No backwards-compatibility shim — §13A is additive; §14 is unchanged.
3. `engine-core` stays noun-free; §13A reaffirms that scaffolding may only cover behavior-free plumbing around allowed generic-contract vocabulary.

## Verification Layers

1. Additive section → grep-proof §13 and §14 bodies are byte-unchanged and §13A sits between them.
2. Behavior-free-lane alignment (§3/§5) → FOUNDATIONS alignment check: §13A excludes legality/scoring/reveal/turn/strategy/hidden-state semantics, matching FOUNDATIONS §4 L97–100.
3. Doc-link integrity → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. New §13A — Forward mechanical-scaffolding conformance boundary

Insert the §13A section (plan §5.4 draft) after §13 and before §14: the forward conformance expectations stated as a boundary rule, parallel to the behavioral §13 promotion boundary, excluding all behavior-bearing surfaces.

## Files to Touch

- `docs/ENGINE-GAME-DATA-BOUNDARY.md` (modify)

## Out of Scope

- Any change to §13 behavioral promotion-boundary text or §14 DSL policy.
- Introducing any runtime registry, selector, or DSL (the receipt stays static evidence — owned by PREGAT18FORSCA-017).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "13A" docs/ENGINE-GAME-DATA-BOUNDARY.md` confirms the new section between §13 and §14.
2. §13 and §14 bodies are byte-unchanged (source comparison).
3. `node scripts/check-doc-links.mjs` and `bash scripts/boundary-check.sh` pass.

### Invariants

1. §13A is additive and parallel to §13; §14 is not renumbered.
2. §13A excludes every behavior-bearing surface (legality, scoring, reveal, turn, strategy, hidden-state).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `bash scripts/boundary-check.sh`
3. `git diff -- docs/ENGINE-GAME-DATA-BOUNDARY.md` (review: only §13A added)

## Outcome

Completed: 2026-06-25

Changed `docs/ENGINE-GAME-DATA-BOUNDARY.md` by adding §13A
`Forward mechanical-scaffolding conformance boundary` between §13
`game-stdlib` promotion boundary and §14 Future DSL policy.

The new section states the forward reuse-first audit, first-use registration,
queue-or-dispose closeout, behavior-bearing exclusion list, and ADR 0009
migration boundary for any replay/hash/fixture/export/RNG/serialization or
visibility change.

Deviation: none. §13 behavioral promotion-boundary text and §14 DSL policy were
not edited.

Verification:

- `grep -n "13A" docs/ENGINE-GAME-DATA-BOUNDARY.md` confirmed the new section.
- A safely quoted `rg` check for the §13 heading, §14 heading, behavior-exclusion
  text, and ADR 0009 boundary text confirmed §13A is between §13 and §14.
- `git diff -- docs/ENGINE-GAME-DATA-BOUNDARY.md` showed only the §13A insertion.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
- `bash scripts/boundary-check.sh` passed.
- `git diff --check` passed.
