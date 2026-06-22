# PREGAT18REUDOC-021: AGENT-TASK reference-not-copy law + scaffold-refactor profile

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — docs-only (`templates/AGENT-TASK.md`)
**Deps**: 012, 016

## Problem

`AGENT-TASK.md` already references foundation docs rather than copying them, but the reference-not-copy law should be made explicit (and extended to the new `GAME-EVIDENCE.md` receipt), and the template needs a scaffold-refactor profile that applies the bounded scaffold-refactor protocol from `AGENT-DISCIPLINE.md`.

## Assumption Reassessment (2026-06-22)

1. Verified `templates/AGENT-TASK.md` already lists foundation docs to read/update (lines ≈11–35) and references — does not copy — law/evidence (confirmed during the `/reassess-spec` validation this session). spec D12: make the reference-not-copy law explicit + add a scaffold-refactor profile.
2. Verified the scaffold-refactor protocol is authored in `AGENT-DISCIPLINE.md` (ticket 016) and the receipt in `GAME-EVIDENCE.md` (ticket 012); hence `Deps: 016, 012`.
3. Cross-artifact boundary under audit: `AGENT-TASK.md` references the scaffold-refactor protocol (016) and the `GAME-EVIDENCE.md` receipt (012); both must exist.
4. FOUNDATIONS §11 (agent output bounded/reviewable) + §13 motivate this: restating — reference-not-copy keeps the task packet bounded and prevents law drift; the scaffold-refactor profile applies the bounded protocol so a scaffolding extraction task stays in scope.

## Architecture Check

1. An explicit reference-not-copy law + a scaffold-refactor profile is cleaner than re-pasting law into each task packet (which drifts) and gives scaffolding refactors a bounded task shape.
2. No backwards-compatibility shims.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; docs-only.

## Verification Layers

1. `AGENT-TASK.md` states the reference-not-copy law and references `GAME-EVIDENCE.md` -> codebase grep-proof.
2. A scaffold-refactor profile is present and points at the `AGENT-DISCIPLINE.md` protocol -> grep.
3. `AGENT-DISCIPLINE.md` protocol (Deps 016) and `GAME-EVIDENCE.md` (Deps 012) exist -> grep / `test -f`.
4. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Reference-not-copy law

Make the reference-not-copy law explicit in `AGENT-TASK.md` and extend it to reference the `GAME-EVIDENCE.md` receipt rather than copying evidence.

### 2. Scaffold-refactor profile

Add a scaffold-refactor task profile that applies the bounded scaffold-refactor protocol authored in `AGENT-DISCIPLINE.md` (ticket 016).

## Files to Touch

- `templates/AGENT-TASK.md` (modify)

## Out of Scope

- Authoring the scaffold-refactor protocol (ticket 016) or the `GAME-EVIDENCE.md` receipt (ticket 012).
- The template-slimming clusters (017/018/019/020).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "reference.*not.*copy|GAME-EVIDENCE" templates/AGENT-TASK.md` returns the reference-not-copy law + receipt reference.
2. `grep -niE "scaffold.refactor" templates/AGENT-TASK.md` returns the profile.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The task packet references law/evidence; it never copies law text (no drift).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (reference-law grep, profile grep, link check) named in Assumption Reassessment.`

### Commands

1. `grep -niE "reference|GAME-EVIDENCE|scaffold.refactor" templates/AGENT-TASK.md`
2. `node scripts/check-doc-links.mjs`
3. The reference-law + profile grep + link check is the correct boundary; docs-only with no code surface.

## Outcome

Completed on 2026-06-22. `templates/AGENT-TASK.md` now has an explicit
reference-not-copy law, links task evidence to `GAME-EVIDENCE.md` receipt rows,
and adds a `scaffold-refactor` task profile that points at
`docs/AGENT-DISCIPLINE.md` §8A and the mechanical scaffolding register.

Verification:

1. `grep -niE "reference.*not.*copy|GAME-EVIDENCE" templates/AGENT-TASK.md` returned the new law and receipt references.
2. `grep -niE "scaffold.refactor" templates/AGENT-TASK.md` returned the new task profile.
3. `grep -niE "Scaffold-refactor protocol|scaffold-refactor" docs/AGENT-DISCIPLINE.md` confirmed the dependency protocol exists.
4. `test -f templates/GAME-EVIDENCE.md` passed.
5. `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
