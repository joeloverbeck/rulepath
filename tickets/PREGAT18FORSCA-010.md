# PREGAT18FORSCA-010: ROADMAP pre-Gate-18 governance subsection + Gate 18 admission/exit

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance/area-doc edit (`docs/ROADMAP.md`)
**Deps**: PREGAT18FORSCA-002

## Problem

`ROADMAP.md` carries only a retroactive "Pre-Gate-18 debt interlock" prose note (Gates 15-17 debt closure); it has no forward scaffolding-reuse governance subsection and no Gate 18 admission/exit hooks for the standing obligation. This ticket adds the forward subsection and the Gate 18 admission sentence + exit bullet, **reconciling with the existing interlock** so there is one coherent pre-Gate-18 obligation.

## Assumption Reassessment (2026-06-25)

1. `docs/ROADMAP.md` lists Gate 17 (L34 / §"Gate 17: Oh Hell" L526) and Gate 18 (L35 / §"Gate 18: Spades" L540, "Partnerships, teams, and grouped outcomes"). An existing "Pre-Gate-18 debt interlock" prose note sits at L47-57 (in §1, after the ladder table), referencing mechanical-scaffolding debt + ADR 0008 + the register in a retroactive framing. Verified this session.
2. The spec (D10, plan §5.10, and reassess finding M2) requires: a pre-Gate-18 forward scaffolding-reuse governance subsection (purpose + exit + not-allowed) after Gate 17, before Gate 18; a Gate 18 admission sentence; a Gate 18 exit bullet — **reconciled with the existing L47-57 interlock** (extend or cross-reference it, not a second divergent statement).
3. Shared contract under audit: the ROADMAP's two pre-Gate-18 statements (the existing L47-57 interlock and the new forward subsection) must not diverge; Gate 18 is admitted only when `8F` is `Done`.
4. FOUNDATIONS §1 (priority order / staged ladder is law) under audit: ROADMAP is ladder law, not a progress diary — the subsection states the standing obligation; progress stays in `specs/README.md`.

## Architecture Check

1. Reconciling the new forward subsection with the existing L47-57 interlock (rather than adding an unrelated block) keeps one authoritative pre-Gate-18 statement, preventing the silent divergence the reassessment flagged.
2. No backwards-compatibility shim — the existing interlock is extended/cross-referenced; the Gate 18 hooks are additive.
3. `engine-core`/`game-stdlib` discipline untouched; ROADMAP stays ladder law and is not rewritten as a progress diary.

## Verification Layers

1. Reconciliation → grep-proof the forward subsection and the existing interlock cross-reference each other (no two divergent pre-Gate-18 statements).
2. Gate 18 admission/exit → grep-proof Gate 18 names the `8F`-`Done` admission precondition and the scaffolding-audit exit bullet.
3. Doc-link integrity → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Pre-Gate-18 forward governance subsection

Add the subsection (plan §5.10 draft) after Gate 17, before Gate 18: purpose + exit + not-allowed for the standing forward scaffolding-reuse governance, cross-referencing the existing L47-57 "Pre-Gate-18 debt interlock".

### 2. Reconcile the existing interlock

Update the L47-57 interlock to point forward to (or fold into) the new subsection, so the two statements are coherent.

### 3. Gate 18 admission + exit

Add the Gate 18 admission sentence (admitted only when `8F` is `Done`) and the Gate 18 exit bullet (first `forward-v1` reuse-first audit recorded).

## Files to Touch

- `docs/ROADMAP.md` (modify)

## Out of Scope

- Authoring any Gate 18 / Spades design, rules, or game content.
- Editing `specs/README.md` (PREGAT18FORSCA-011 / the capstone own the tracker).
- Rewriting ROADMAP as a progress diary.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -ni "forward scaffolding-reuse governance" docs/ROADMAP.md` ≥ 1 between Gate 17 and Gate 18.
2. The new subsection and the L47-57 interlock cross-reference each other (grep-proof; no divergence).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. There is one coherent pre-Gate-18 obligation statement.
2. Gate 18 admission is gated on `8F` = `Done`.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- docs/ROADMAP.md` (review: subsection + interlock reconciliation + Gate 18 hooks)
3. `grep -n "Pre-Gate-18" docs/ROADMAP.md`
