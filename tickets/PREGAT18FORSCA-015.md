# PREGAT18FORSCA-015: GAME-EVIDENCE pre-code-audit / register-freshness / disposition rows

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance/template-doc edit (`templates/GAME-EVIDENCE.md`)
**Deps**: PREGAT18FORSCA-002, PREGAT18FORSCA-007

## Problem

`GAME-EVIDENCE.md` records mechanic/scaffolding decisions but has no pre-code-audit receipt, post-build register-freshness receipt, prior-game-disposition row, or CI-audit row, so a new game's closeout can omit the scaffolding evidence. This ticket replaces the "Mechanic and Scaffolding Decisions" table with those receipt rows and appends the receipt-review checks.

## Assumption Reassessment (2026-06-25)

1. `templates/GAME-EVIDENCE.md` is a receipt-based evidence template with a "Mechanic and scaffolding conformance" / "Mechanic and Scaffolding Decisions" surface and named trace-profile / viewer / no-leak matrices. Verified this session.
2. The spec (D15, plan §5.18) requires: replace the "Mechanic and Scaffolding Decisions" table with pre-code-audit / register-freshness / prior-game-disposition / CI-audit rows; append the receipt-review checks.
3. Shared contract under audit: the evidence rows must cite the same register IDs and source/evidence paths the Gate 1 checker (PREGAT18FORSCA-018) validates against committed files, and the receipt (PREGAT18FORSCA-017) the CI-audit row points to.
4. FOUNDATIONS §6 (official games are evidence-heavy) and §11 (acceptance invariants) under audit: the receipt rows make the post-build register freshness and prior-game disposition non-silent closeout evidence, completing the audit → register → queue-or-dispose loop.
5. Cross-doc link: the rows reference the register's forward cadence; Deps PREGAT18FORSCA-007 for anchor resolution under `check-doc-links`.

## Architecture Check

1. Replacing the decisions table with explicit receipt rows makes each closeout step (pre-code audit, register freshness, prior-game disposition, CI audit) a named evidence artifact rather than free prose.
2. No backwards-compatibility shim — the table is replaced with the receipt rows, not aliased.
3. `engine-core`/`game-stdlib` discipline untouched; the CI-audit row points to the static receipt, keeping evidence behavior-free.

## Verification Layers

1. Receipt-row presence → grep-proof the four receipt rows (pre-code audit, register-freshness, prior-game disposition, CI audit) replace the old table.
2. Review-check coverage → grep-proof the appended receipt-review checks.
3. Cross-doc anchor integrity → `node scripts/check-doc-links.mjs` (register cadence anchors via PREGAT18FORSCA-007).

## What to Change

### 1. Replace the "Mechanic and Scaffolding Decisions" table

Replace it with the pre-code-audit / register-freshness / prior-game-disposition / CI-audit rows (plan §5.18 draft).

### 2. Receipt-review checks

Append the receipt-review checks confirming each row cites committed register IDs and evidence paths.

## Files to Touch

- `templates/GAME-EVIDENCE.md` (modify)

## Out of Scope

- Editing the other per-game templates.
- Implementing the CI checker the CI-audit row references (PREGAT18FORSCA-018).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -ni "register-freshness\|register freshness" templates/GAME-EVIDENCE.md` ≥ 1.
2. The prior-game-disposition and CI-audit rows are present (grep-proof).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. Post-build register freshness and prior-game disposition are non-silent closeout receipts.
2. Each row cites committed register IDs / evidence paths.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- templates/GAME-EVIDENCE.md` (review: four receipt rows + review checks)
3. `grep -n "prior-game" templates/GAME-EVIDENCE.md`
