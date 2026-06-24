# PREGAT18FORSCA-011: specs/README active-epoch intro + spec-format + workflow rewrite

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance/index-doc edit (`specs/README.md`)
**Deps**: PREGAT18FORSCA-002

## Problem

The `specs/README.md` tracker already carries the `8F` row (`Planned`) and the Gate 18 block (`Blocked until 8F is Done`), but its active-epoch intro, spec-format paragraph, and Workflow section do not yet fold in the forward scaffolding reuse-first audit as a standing interlock. This ticket lands those deeper rewrites (plan §5.14 Locations 1, 4, 5). The `8F`→`Done` flip is **not** here — it belongs to the closeout capstone (PREGAT18FORSCA-021).

## Assumption Reassessment (2026-06-25)

1. `specs/README.md` carries: the active-epoch intro (`## Active epoch — public scaling phase`, ~L75-84); the `8F` row at L103 (`Planned`) and the Gate 18 row at L104 (`Blocked until 8F is Done`) — both **already committed** at spec-authoring time (reassess finding M4); the "Spec format" paragraph (~L129-158); the "Workflow" section (~L159-172). Verified this session.
2. The spec (D11, plan §5.14 Locations 1, 4, 5) requires: the active-epoch intro/interlock-note replacement (folding in the reuse-first audit interlock), a spec-format paragraph requiring per-new-game audit fields, and a Workflow-section rewrite folding in the scaffolding audit. Locations 2 (the `8F` row) and 3 (the Gate 18 block) are already present → verify-only.
3. Shared contract under audit: the tracker's standing-interlock wording — the active-epoch intro must name both the behavioral promotion-debt interlock and the new scaffolding reuse-first-audit interlock.
4. FOUNDATIONS §1 (staged ladder is law; the index is the mutable progress tracker) under audit: this ticket edits only the tracker's process prose, not ROADMAP ladder law; the `Done`-flip is deferred to the capstone per the index admission rule.

## Architecture Check

1. Editing the intro/spec-format/workflow (not re-adding the already-present rows) keeps this ticket a clean process-prose diff and avoids duplicating committed rows.
2. No backwards-compatibility shim — the intro/interlock note is replaced with the dual-interlock wording; additive spec-format and workflow steps.
3. `engine-core`/`game-stdlib` discipline untouched; the tracker stays the progress record, with the `Done`-flip reserved for the capstone's exit-evidence gate.

## Verification Layers

1. Dual-interlock intro → grep-proof the active-epoch intro names both the promotion-debt and the scaffolding reuse-first-audit interlocks.
2. Locations 2/3 unchanged → grep-proof the `8F` row stays `Planned` and the Gate 18 block stays `Blocked until 8F` (not flipped here).
3. Doc-link integrity → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Active-epoch intro / interlock note (Location 1)

Replace the intro paragraph with the dual-interlock wording (plan §5.14 Location 1): pick the lowest non-`Done` unit honoring both the behavioral promotion-debt interlock and the new mechanical-scaffolding reuse-first audit.

### 2. Spec-format paragraph (Location 4)

Append the per-new-game audit-field requirement to the "Spec format" section.

### 3. Workflow rewrite (Location 5)

Rewrite the Workflow section to fold in the scaffolding reuse-first audit step alongside the atlas/primitive-pressure check.

## Files to Touch

- `specs/README.md` (modify)

## Out of Scope

- Flipping the `8F` row to `Done` (PREGAT18FORSCA-021 capstone, gated on exit evidence).
- Re-adding the `8F` row or the Gate 18 block (already committed — verify-only).
- Admitting Gate 18 early.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -ni "reuse-first audit" specs/README.md` ≥ 1 in the active-epoch intro and Workflow.
2. The `8F` row still reads `Planned` and the Gate 18 row still reads `Blocked until 8F` (grep-proof — not flipped here).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The active-epoch intro carries both standing interlocks.
2. The `Done`-flip is not performed in this ticket.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- specs/README.md` (review: intro + spec-format + workflow only; rows untouched)
3. `grep -n "8F" specs/README.md`
