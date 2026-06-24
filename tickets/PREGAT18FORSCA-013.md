# PREGAT18FORSCA-013: GAME-IMPLEMENTATION-ADMISSION reuse-first-audit gating rows

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance/template-doc edit (`templates/GAME-IMPLEMENTATION-ADMISSION.md`)
**Deps**: PREGAT18FORSCA-002

## Problem

The admission template asks for a scaffolding decision only "if needed" via its "Novel Mechanics and Pressure" table, so a new game can be admitted without a completed reuse-first audit. This ticket replaces that table with reuse-first-audit gating rows, adds the three required-evidence-profile rows, and adds an admission-blocked paragraph — making the audit a gating admission surface.

## Assumption Reassessment (2026-06-25)

1. `templates/GAME-IMPLEMENTATION-ADMISSION.md` carries a "Novel Mechanics and Pressure" table (mechanic inventory / primitive-pressure decision / scaffolding decision / ADR needed). Verified this session.
2. The spec (D13, plan §5.16) requires: replace the "Novel Mechanics and Pressure" table with reuse-first-audit gating rows; add the three required-evidence-profile rows; add the admission-blocked paragraph.
3. Shared contract under audit: the admission gating rows must align with the §11 forward invariants (PREGAT18FORSCA-002) and the official-game-contract acceptance cluster (PREGAT18FORSCA-005) so admission and acceptance agree on the audit surface.
4. FOUNDATIONS §6 (official games are evidence-heavy) and §11 (acceptance invariants) under audit: making the reuse-first audit a gating admission row enforces "before serious implementation" at the admission boundary, with a `not applicable` result requiring a rationale (never silence).

## Architecture Check

1. Replacing the optional "if needed" table with gating rows is the minimal change that makes the audit a hard admission gate rather than advisory.
2. No backwards-compatibility shim — the table is replaced, not aliased; the admission-blocked paragraph names the failure state explicitly.
3. `engine-core`/`game-stdlib` discipline untouched; the audit governs scaffolding reuse, not legality — Rust stays the behavior authority.

## Verification Layers

1. Gating-row presence → grep-proof the reuse-first-audit gating rows and the three required-evidence-profile rows replace the old table.
2. Admission-blocked state → grep-proof the admission-blocked paragraph names a missing/invalid audit as blocking.
3. Doc-link integrity → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Replace the "Novel Mechanics and Pressure" table

Replace it with the reuse-first-audit gating rows (plan §5.16 draft): audit completed, register reviewed, new shapes identified, `not applicable` requires rationale.

### 2. Required-evidence-profile rows

Add the three required-evidence-profile rows (pre-code audit, register-freshness, prior-game disposition).

### 3. Admission-blocked paragraph

Add the paragraph stating admission is blocked when the audit record is missing or invalid.

## Files to Touch

- `templates/GAME-IMPLEMENTATION-ADMISSION.md` (modify)

## Out of Scope

- Editing the other per-game templates (PREGAT18FORSCA-014/015/016).
- The CI checker that enforces the audit at Gate 1 (PREGAT18FORSCA-018).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -ni "reuse-first audit" templates/GAME-IMPLEMENTATION-ADMISSION.md` ≥ 1 in the gating rows.
2. The admission-blocked paragraph is present (grep-proof).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The reuse-first audit is a gating admission row; a `not applicable` result requires a rationale.
2. The admission gating rows mirror the §11 forward invariants.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- templates/GAME-IMPLEMENTATION-ADMISSION.md` (review: gating rows + evidence-profile rows + blocked paragraph)
3. `grep -n "not applicable" templates/GAME-IMPLEMENTATION-ADMISSION.md`
