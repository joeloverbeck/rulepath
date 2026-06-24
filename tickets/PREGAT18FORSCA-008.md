# PREGAT18FORSCA-008: AGENT-DISCIPLINE §8B new-game scaffolding reuse-and-track protocol

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance/area-doc edit (`docs/AGENT-DISCIPLINE.md`)
**Deps**: PREGAT18FORSCA-002, PREGAT18FORSCA-007

## Problem

`AGENT-DISCIPLINE.md` §8A governs scaffold-refactor tasks, but a task only becomes subject to it after someone has already classified it as a scaffold refactor. New-game work needs a standing reuse-and-track protocol that applies even when the task profile is not `scaffold-refactor`. This ticket adds §8B and the §13 review-check bullets so coding agents on any new-game task run the reuse-first audit and register/queue duties.

## Assumption Reassessment (2026-06-25)

1. `docs/AGENT-DISCIPLINE.md` §8A "Scaffold-refactor protocol" is at L171; §13 "Review check" is at L266 with review-check bullets. No §8B exists yet. Verified this session.
2. The spec (D8, plan §5.8) requires a new §8B "New-game scaffolding reuse-and-track protocol" after §8A (applies even when `Task profile` ≠ `scaffold-refactor`) plus §13 review-check bullets.
3. Shared contract under audit: the §8 protocol spine (§8A scaffold-refactor, §8B new-game reuse-and-track) and the §13 review checks that gate every agent task review.
4. FOUNDATIONS §11 (acceptance invariants) and the AGENT-DISCIPLINE bounded-task law under audit: §8B keeps agent scope bounded — it adds a mandatory audit/register/queue checklist, not an open-ended "generalize" license (which §12 forbids).
5. Cross-doc link: §8B references the register's Forward Per-Game Maintenance Cadence; Deps PREGAT18FORSCA-007 so the cited heading anchors resolve under `check-doc-links`.

## Architecture Check

1. A standing §8B (parallel to §8A) is cleaner than overloading §8A's scaffold-refactor framing — it binds new-game tasks regardless of profile, matching the FOUNDATIONS §11 "every new official game" scope.
2. No backwards-compatibility shim — §8B is additive; §8A is unchanged.
3. `engine-core`/`game-stdlib` discipline untouched; §8B keeps agent tasks bounded and reviewable, never authorizing kernel generalization.

## Verification Layers

1. Protocol presence → grep-proof §8B sits after §8A and states the audit/register/queue duties for non-`scaffold-refactor` profiles.
2. Review-check coverage → grep-proof §13 names the reuse-first-audit / register-new / queue-or-dispose review checks.
3. Cross-doc anchor integrity → `node scripts/check-doc-links.mjs` (register cadence anchors exist via PREGAT18FORSCA-007).

## What to Change

### 1. New §8B — New-game scaffolding reuse-and-track protocol

Insert §8B (plan §5.8 draft) after §8A: the protocol applies to any new-game task (even when `Task profile` ≠ `scaffold-refactor`); it requires the reuse-first audit, first-use registration, and queue-or-dispose of prior matches before closeout.

### 2. §13 review-check bullets

Add the review-check bullets so a reviewer confirms the audit ran, new shapes are registered, and prior matches are queued or disposed.

## Files to Touch

- `docs/AGENT-DISCIPLINE.md` (modify)

## Out of Scope

- Any change to §8A scaffold-refactor protocol text.
- Authoring the register cadence (PREGAT18FORSCA-007) or the task-packet template section (PREGAT18FORSCA-016).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "8B" docs/AGENT-DISCIPLINE.md` confirms the new section after §8A.
2. §13 names the forward review checks (grep-proof).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. §8B applies regardless of task profile; agent scope stays bounded (no "generalize the engine" license).
2. §8A is unchanged.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- docs/AGENT-DISCIPLINE.md` (review: §8B + §13 bullets)
3. `grep -n "Task profile" docs/AGENT-DISCIPLINE.md`
