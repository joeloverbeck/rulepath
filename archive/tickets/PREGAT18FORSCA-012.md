# PREGAT18FORSCA-012: templates/README authority list + lifecycle paragraph + index rows

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: None — governance/template-doc edit (`templates/README.md`)
**Deps**: PREGAT18FORSCA-002, PREGAT18FORSCA-007

## Problem

`templates/README.md` does not name the mechanical-scaffolding register in its authority list, carries no two-stage audit lifecycle paragraph, and its index rows for the per-game templates predate the forward obligation. This ticket updates the authority list, adds the lifecycle paragraph, and refreshes the index rows so the template index makes the standing lifecycle discoverable.

## Assumption Reassessment (2026-06-25)

1. `templates/README.md` carries a foundation-document authority list (~L7-22) and a "Template index" table (~L80-99) with rows for `GAME-MECHANICS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `AGENT-TASK.md`, `GAME-EVIDENCE.md`, and `PRIMITIVE-PRESSURE-LEDGER.md`. Verified this session.
2. The spec (D12, plan §5.15) requires: add the register to the authority list after `MECHANIC-ATLAS.md`; add the two-stage audit lifecycle paragraph; replace the `GAME-MECHANICS` / `GAME-IMPLEMENTATION-ADMISSION` / `AGENT-TASK` / `GAME-EVIDENCE` index rows.
3. Shared contract under audit: the template index's description of each per-game template's owned surface — the rows must match the new sections those templates gain (PREGAT18FORSCA-013/014/015/016).
4. FOUNDATIONS §6 (official games are evidence-heavy) under audit: naming the register in the authority list makes the reuse-first audit a discoverable template obligation, consistent with the §11 invariants.

## Architecture Check

1. Updating the index rows to match the templates' new sections keeps the index an accurate map; adding the register to the authority list mirrors how `MECHANIC-ATLAS.md` is already cited.
2. No backwards-compatibility shim — the lifecycle paragraph and refreshed rows are additive/replacement, not aliased.
3. `engine-core`/`game-stdlib` discipline untouched; the index points to the register as a behavior-free-scaffolding home.

## Verification Layers

1. Authority-list addition → grep-proof the register is listed after `MECHANIC-ATLAS.md`.
2. Index-row fidelity → grep-proof the four refreshed rows name the reuse-first-audit / register-update surfaces.
3. Doc-link integrity → `node scripts/check-doc-links.mjs` (register link resolves via PREGAT18FORSCA-007).

## What to Change

### 1. Authority list

Add `MECHANICAL-SCAFFOLDING-REGISTER.md` to the authority list after `MECHANIC-ATLAS.md`.

### 2. Two-stage audit lifecycle paragraph

Add the paragraph (plan §5.15 draft) describing the pre-implementation audit and post-build register-freshness stages.

### 3. Index rows

Replace the `GAME-MECHANICS` / `GAME-IMPLEMENTATION-ADMISSION` / `AGENT-TASK` / `GAME-EVIDENCE` index rows to name their new audit/register surfaces.

## Files to Touch

- `templates/README.md` (modify)

## Out of Scope

- Editing the per-game template bodies (PREGAT18FORSCA-013/014/015/016).
- The `PRIMITIVE-PRESSURE-LEDGER.md` no-change proof (recorded in the capstone).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "MECHANICAL-SCAFFOLDING-REGISTER" templates/README.md` appears in the authority list.
2. The four index rows name the reuse-first-audit / register-update surfaces (grep-proof).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The register is a listed authority document.
2. Index rows match the templates' post-8F owned surfaces.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- templates/README.md` (review: authority list + lifecycle paragraph + four rows)
3. `grep -n "audit lifecycle" templates/README.md`

## Outcome

Completed: 2026-06-25

Changed `templates/README.md` to make the forward scaffolding lifecycle
discoverable from the template index:

- added `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` to the foundation authority
  list after `docs/MECHANIC-ATLAS.md`;
- added the lifecycle paragraph explaining the pre-implementation
  reuse-first audit and post-implementation register-freshness/prior-game
  closeout flow;
- refreshed the `GAME-MECHANICS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`,
  `AGENT-TASK.md`, and `GAME-EVIDENCE.md` index rows.

Deviation: none. The per-game template bodies remain owned by later tickets.

Verification:

- `grep -n "MECHANICAL-SCAFFOLDING-REGISTER" templates/README.md` confirmed the authority-list and index-row references.
- `grep -n "audit lifecycle\\|reuse-first audit\\|register-freshness\\|prior-game-refactor" templates/README.md` confirmed the lifecycle and row wording.
- `rg -n "GAME-MECHANICS\\.md.*reuse-first|GAME-IMPLEMENTATION-ADMISSION\\.md.*scaffolding reuse-first|AGENT-TASK\\.md.*reuse/track|GAME-EVIDENCE\\.md.*post-build register" templates/README.md` confirmed all four refreshed rows.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
- `git diff --check` passed.
