# PREGAT18REUDOC-002: Strengthen ADR-TEMPLATE for the scaling/supersession era

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — docs-only (`docs/adr/ADR-TEMPLATE.md`)
**Deps**: None

## Problem

ADR 0008 and ADR 0009 (tickets 004/005) are to be "built from the revised `ADR-TEMPLATE.md`", but the current template carries its scaling/supersession fields under an *Optional* heading and lacks several fields this pass needs (compatibility window, evidence-classification, accepted-exceptions, an effective-only-after-foundation-updates field, a Proposed-ADR review-trigger/expiry). Without the revision, the new ADRs cannot uniformly name their amended sections, compatibility windows, and acceptance preconditions.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/adr/ADR-TEMPLATE.md` already carries `Affected foundation sections` (L25), `Superseded decision, if any` (L26), `Rollback / contamination risk` (L30), a prose `Migration plan` (L29), and split `Determinism impact` / `Replay/hash impact` / `Visibility impact` sections (L68–83) — all under an "Optional scaling / supersession fields" heading (L23). This is the reframed scope from `/reassess-spec` finding I3: do **not** re-add present fields.
2. Verified against spec D7 (reframed): (a) promote the relevant existing optional fields to required; (b) add only the genuinely-absent fields; (c) upgrade prose `Migration plan` → an adopter/migration matrix; (d) keep a single selected `Status` value.
3. Cross-artifact boundary under audit: `docs/adr/ADR-TEMPLATE.md` is the contract that tickets 004 and 005 instantiate; the field set added here is what those ADRs must fill.
4. FOUNDATIONS §13 (ADR triggers) and L3 motivate the strengthening: an ADR amending a foundation principle must name the exact affected sections and state it is effective only after the named foundation updates land — restating that requirement before trusting the spec narrative.

## Architecture Check

1. Promoting the existing optional fields to required (rather than duplicating them as "new" fields) is the clean fix that avoids the I3 mistake of re-introducing present fields; adding only the absent fields keeps the template minimal.
2. No backwards-compatibility shims; ADRs 0001–0007 are not retro-edited (the template governs new ADRs).
3. Docs-only: `engine-core` (§3) and `game-stdlib` (§4) untouched.

## Verification Layers

1. Newly-required fields (compatibility window, evidence-classification, accepted-exceptions, effective-after-foundation-updates, Proposed-ADR review-trigger/expiry) present -> codebase grep-proof.
2. Existing scaling fields promoted out of the "Optional" heading to required -> grep + manual review.
3. `Migration plan` upgraded to an adopter/migration matrix -> manual review.
4. Template links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Promote existing fields to required

Move `Affected foundation sections`, `Superseded decision`, `Rollback / contamination risk`, and the determinism/replay-hash/visibility impact sections out of "Optional scaling / supersession fields" into a required field block.

### 2. Add the absent fields

Add: compatibility window, evidence-classification, accepted-exceptions, an "effective only after named foundation updates land" field, and a Proposed-ADR review-trigger/expiry field.

### 3. Upgrade migration plan + single Status

Replace the prose `Migration plan` line with an adopter/migration matrix; keep `Status` a single selected value.

## Files to Touch

- `docs/adr/ADR-TEMPLATE.md` (modify)

## Out of Scope

- Authoring the actual ADRs 0008/0009 (tickets 004/005).
- Re-editing already-accepted ADRs 0001–0007.
- The README ADR status index (ticket 001).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "compatibility window|evidence.classification|accepted.exception|review.trigger|effective only after" docs/adr/ADR-TEMPLATE.md` returns the new required fields.
2. `grep -niE "migration matrix" docs/adr/ADR-TEMPLATE.md` returns the upgraded field.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The template carries one selected `Status` value, not a behavior-bearing pick-list of statuses in the produced ADR.
2. No previously-present field is duplicated by a re-added "new" field.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage (`check-doc-links.mjs`) is named in Assumption Reassessment.`

### Commands

1. `grep -niE "compatibility window|evidence.classification|accepted.exception|review.trigger|migration matrix" docs/adr/ADR-TEMPLATE.md`
2. `node scripts/check-doc-links.mjs`
3. A field-presence grep is the correct boundary for the template fields; `check-doc-links.mjs` covers link integrity.

## Outcome

Completed: 2026-06-22

Changed `docs/adr/ADR-TEMPLATE.md` so new ADRs start from one selected
`Status: Proposed`, carry a required scaling / supersession field block, include
the missing evidence-classification, compatibility-window, accepted-exception,
foundation-effective, and Proposed-ADR review-trigger fields, and use a migration
matrix instead of a prose migration-plan line.

Deviations: none.

Verification:

- `grep -niE "compatibility window|evidence.classification|accepted.exception|review.trigger|migration matrix" docs/adr/ADR-TEMPLATE.md`
  returned the new required fields and migration matrix.
- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
