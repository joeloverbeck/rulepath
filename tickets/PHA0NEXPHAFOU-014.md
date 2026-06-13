# PHA0NEXPHAFOU-014: ROADMAP — add the public scaling phase and restate Gate P as the tail

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — `docs/ROADMAP.md` edit only.
**Deps**: PHA0NEXPHAFOU-001

## Problem

`docs/ROADMAP.md` records a completed public ladder through Gate 14 with a Gate P appendix, and does not encode the settled next phase. Once ADR 0007 is accepted, ROADMAP must add the public scaling phase (Gate 15+) after Gate 14 and restate Gate P as the very tail — last, private, optional, non-architectural. This is the implementing edit that ADR 0007 authorizes; it cannot land before that ADR is accepted.

## Assumption Reassessment (2026-06-13)

1. No code change. `docs/ROADMAP.md:3` carries the header law (*"A stage or gate may be skipped or reordered only by accepted ADR"*); its §1 stage/gate crosswalk ends at Gate 14, and §15 is the Gate P appendix (verified this session).
2. Docs: `docs/ROADMAP.md`; ADR 0007 (PHA0NEXPHAFOU-001) is the authority that admits this edit; `specs/README.md`'s active-epoch tracker already mirrors the Gate 15+ ladder and is reconciled by PHA0NEXPHAFOU-015.
3. Cross-artifact boundary under audit: the ROADMAP gate ladder + ADR 0007; shared surface = the §1 stage/gate crosswalk and the Gate P placement.
4. FOUNDATIONS principle restate: §1 priority order (the public ladder ranks above the private Gate P) and the ROADMAP header law / §13 (gate reorder requires an accepted ADR). This edit is the change ADR 0007 authorizes.
5. The gate-ladder change is a doc-governed-contract addition (not a rename/removal): its blast radius is `specs/README.md` (the active-epoch tracker, reconciled by PHA0NEXPHAFOU-015) and the archival workflow; grep those surfaces to confirm consistency rather than divergence.

## Architecture Check

1. Encoding the settled phase in ROADMAP only after ADR 0007 is accepted is the FOUNDATIONS-sanctioned path; a bare edit without the ADR would cross the header law (§12).
2. No backwards-compatibility aliasing/shims introduced.
3. `engine-core` is untouched and stays noun-free; ROADMAP remains prescriptive ladder law, not a progress diary.

## Verification Layers

1. ADR 0007 is accepted before this edit → codebase grep-proof (`docs/adr/0007-*.md` `Status: Accepted`).
2. New public scaling phase rows present after Gate 14 and Gate P restated as the tail → manual review.
3. ROADMAP gate ladder is consistent with the `specs/README.md` active-epoch tracker → grep cross-check.
4. Links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/ROADMAP.md`

After Gate 14, add the public scaling phase: Gate 15 (River Ledger / Texas Hold'Em) first, then the Gate 15.1–23 ladder, the maintenance interlocks and admission rules, and the medium-heavy public ceiling (capstone). Restate Gate P as last, private, optional, and non-architectural. Update the §1 stage/gate crosswalk table to list Gate 15+ and the moved Gate P. Keep ROADMAP prescriptive — do not record per-spec progress here (that is `specs/README.md`).

## Files to Touch

- `docs/ROADMAP.md` (modify)

## Out of Scope

- Editing `docs/ROADMAP.md` before ADR 0007 is `Accepted` (hard sequencing gate).
- Writing any Gate 15+ spec.
- Recording per-spec progress in ROADMAP, or turning it into a progress diary (`specs/README.md` is the tracker, reconciled by PHA0NEXPHAFOU-015).

## Acceptance Criteria

### Tests That Must Pass

1. `docs/ROADMAP.md` records the public scaling phase after Gate 14 (Gate 15 first) and restates Gate P as the tail; the §1 crosswalk lists Gate 15+ and the moved Gate P.
2. `docs/adr/0007-*.md` carries `Status: Accepted` (the authorizing precondition).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. ROADMAP remains prescriptive ladder law (no per-spec progress entries).
2. The edit is authorized by an accepted ADR 0007 — the header law is not crossed.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -niE "Status:.*Accepted" docs/adr/0007-*.md`
2. `grep -niE "Gate 15|Gate P" docs/ROADMAP.md`
3. `node scripts/check-doc-links.mjs`
