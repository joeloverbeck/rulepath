# PREGAT18REUDOC-016: AGENT-DISCIPLINE scaffold-refactor protocol + archival closeout receipt

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — docs-only (`docs/AGENT-DISCIPLINE.md`, `docs/archival-workflow.md`)
**Deps**: 004

## Problem

The scaffolding lane needs a bounded scaffold-refactor protocol in `AGENT-DISCIPLINE.md` (so a future scaffolding extraction stays bounded and reviewable), and `archival-workflow.md` lacks a closeout-receipt requirement (so completed units record their evidence on archival). Both align the process docs with the post-ADR-0008 doctrine.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/AGENT-DISCIPLINE.md` and `docs/archival-workflow.md` exist; spec D11 (report A-15 scaffold-refactor protocol; A-17 archival closeout receipt). The scaffold-refactor protocol references the scaffolding lane that ADR 0008 (ticket 004) establishes; hence `Deps: 004` + acceptance precondition for that portion.
2. Verified against spec D11: add the bounded scaffolding/hash-refactor protocol + an archival closeout-receipt requirement.
3. Cross-artifact boundary under audit: the scaffold-refactor protocol authored here is referenced by `AGENT-TASK.md` (ticket 021, which `Deps: 016`).
4. FOUNDATIONS §13 + the agent-discipline contract motivate this: restating — a bounded scaffold-refactor protocol keeps refactors bounded/reviewable (no "generalize the engine" scope creep, §12), and the closeout receipt makes a completed unit record its acceptance evidence on archival.

## Architecture Check

1. A bounded scaffold-refactor protocol + an archival closeout receipt are cleaner than ad-hoc refactor scope and undocumented closeouts; they extend the existing agent-discipline contract minimally.
2. No backwards-compatibility shims.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; docs-only.

## Verification Layers

1. `AGENT-DISCIPLINE.md` carries the bounded scaffold-refactor protocol -> codebase grep-proof.
2. `archival-workflow.md` carries a closeout-receipt requirement -> grep.
3. ADR 0008 `Accepted` precondition (scaffold-refactor portion) -> grep (`^Status: Accepted` on `docs/adr/0008-*.md`).
4. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. AGENT-DISCIPLINE scaffold-refactor protocol

Add a bounded scaffolding/hash-refactor protocol (bounded scope, forbidden changes, evidence) consistent with ADR 0008's lane.

### 2. archival-workflow closeout receipt

Add a closeout-receipt requirement so completed units record their acceptance evidence on archival.

## Files to Touch

- `docs/AGENT-DISCIPLINE.md` (modify)
- `docs/archival-workflow.md` (modify)

## Out of Scope

- The `AGENT-TASK.md` reference-not-copy edit (ticket 021, which consumes this protocol).
- The other D11 docs (013/014/015).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "scaffold.refactor|scaffolding refactor" docs/AGENT-DISCIPLINE.md` returns the protocol.
2. `grep -niE "closeout receipt" docs/archival-workflow.md` returns the requirement.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The scaffold-refactor protocol keeps refactors bounded (no unbounded "generalize" scope, §12).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (protocol grep, closeout-receipt grep, link check) named in Assumption Reassessment.`

### Commands

1. `grep -niE "scaffold.refactor|closeout receipt" docs/AGENT-DISCIPLINE.md docs/archival-workflow.md`
2. `node scripts/check-doc-links.mjs`
3. The protocol + receipt grep is the correct boundary; docs-only with no code surface.

## Outcome

Completed: 2026-06-22

Added `docs/AGENT-DISCIPLINE.md` §8A, a bounded scaffold-refactor protocol for
shared-scaffolding and hash-sensitive refactors. The protocol requires accepted
lane authority, exact adopter/hash/visibility/migration inventory,
characterization tests, one-reference-game migration, trace/hash/no-leak
comparison, register exceptions for non-migration, and explicit authorization
for any intentional trace/hash/schema migration. It bans broad "update all
goldens" or green-tests-only acceptance.

Updated `docs/archival-workflow.md` to require a closeout receipt inside
`## Outcome` for archived specs, reports, ticket series, agent tasks,
design/plan docs, and triage docs that change authority, evidence surfaces, or
closeout state. The receipt records commits, recommendation dispositions,
affected authority/ADR state, open mechanic/scaffolding debt, checks run, live
index links, and historical-commit provenance caveats.

Verification:

- `grep -niE "scaffold.refactor|scaffolding refactor" docs/AGENT-DISCIPLINE.md`
  returned the new protocol.
- `grep -niE "closeout receipt" docs/archival-workflow.md` returned the
  archival requirement.
- `grep -n "^Status: Accepted" docs/adr/0008-mechanical-scaffolding-governance.md`
  returned `Status: Accepted`.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).

Deviation: none; this is process-doc-only and does not implement scaffolding
extraction.
