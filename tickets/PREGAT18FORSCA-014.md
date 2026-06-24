# PREGAT18FORSCA-014: GAME-MECHANICS reuse-first audit table + register update

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance/template-doc edit (`templates/GAME-MECHANICS.md`)
**Deps**: PREGAT18FORSCA-002, PREGAT18FORSCA-007

## Problem

`GAME-MECHANICS.md` owns game-local mechanic classification but does not require a mechanical-scaffolding reuse-first audit or a register update, so a new game's first-use behavior-free shape can stay invisible until another game recreates it. This ticket adds the reuse-first audit table + audit rules and replaces "Required repo atlas update" with an atlas/register update section, then appends the review checks.

## Assumption Reassessment (2026-06-25)

1. `templates/GAME-MECHANICS.md` owns "game-local mechanic classification and mechanic/scaffolding pressure analysis" and carries a "Required repo atlas update" line plus a mechanic-inventory table. Verified this session.
2. The spec (D14, plan §5.17) requires: add the "Mechanical scaffolding reuse-first audit" table + audit rules; replace "Required repo atlas update" with the atlas/register update section; append the review checks.
3. Shared contract under audit: the reuse-first audit table's columns must produce the register-entry fields (exclusions, surfaces, decision state, next-review trigger) that the register (PREGAT18FORSCA-007) and the receipt (PREGAT18FORSCA-017) consume.
4. FOUNDATIONS §4 (`game-stdlib` earned) and §11 (mechanical-scaffolding invariant) under audit: the audit table records reuse-or-register decisions without authorizing promotion on first use, parallel to (not replacing) the behavioral primitive-pressure analysis the template already owns.
5. Cross-doc link: the atlas/register update section names the register's forward cadence; Deps PREGAT18FORSCA-007 so the cited heading anchors resolve.

## Architecture Check

1. Adding the reuse-first audit table next to the existing mechanic inventory keeps mechanic classification and scaffolding audit co-located in the template that already owns pressure analysis.
2. No backwards-compatibility shim — "Required repo atlas update" is replaced with the broader atlas/register update section, not aliased.
3. `engine-core`/`game-stdlib` discipline untouched; the audit table keeps behavior-bearing shapes routed to the atlas, behavior-free shapes to the register.

## Verification Layers

1. Audit-table presence → grep-proof the "Mechanical scaffolding reuse-first audit" table and rules are present.
2. Atlas/register update → grep-proof the section names both the atlas and the register as update targets.
3. Cross-doc anchor integrity → `node scripts/check-doc-links.mjs` (register cadence anchors via PREGAT18FORSCA-007).

## What to Change

### 1. Reuse-first audit table + rules

Add the "Mechanical scaffolding reuse-first audit" table and audit rules (plan §5.17 draft).

### 2. Atlas/register update section

Replace "Required repo atlas update" with the atlas/register update section naming both update targets.

### 3. Review checks

Append the review checks confirming the audit ran and new shapes are registered.

## Files to Touch

- `templates/GAME-MECHANICS.md` (modify)

## Out of Scope

- Editing the other per-game templates.
- Authoring the register cadence sections (PREGAT18FORSCA-007).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -ni "reuse-first audit" templates/GAME-MECHANICS.md` ≥ 1 (table + rules).
2. The atlas/register update section names the register as an update target (grep-proof).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. First use registers without authorizing promotion.
2. Behavior-bearing shapes stay routed to the atlas; behavior-free shapes to the register.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- templates/GAME-MECHANICS.md` (review: audit table + atlas/register section + review checks)
3. `grep -n "register" templates/GAME-MECHANICS.md`
