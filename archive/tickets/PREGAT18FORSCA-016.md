# PREGAT18FORSCA-016: AGENT-TASK new-game scaffolding reuse/track section

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance/template-doc edit (`templates/AGENT-TASK.md`)
**Deps**: PREGAT18FORSCA-002

## Problem

`templates/AGENT-TASK.md` has a Scaffold-Refactor Profile but no standing new-game scaffolding reuse/track section, so a new-game task packet need not carry the reuse-first audit, register, and queue-or-dispose status. This ticket adds the register to the authority list, adds the "New-game scaffolding reuse/track status" section, adds the acceptance-evidence / implementation-boundary / documentation-required rows, and appends the forbidden-changes + review checks — keeping the existing Scaffold-Refactor Profile intact.

## Assumption Reassessment (2026-06-25)

1. `templates/AGENT-TASK.md` carries an authority list (~L17-31), task-profile options including `scaffold-refactor` (~L7), a "Target" section (~L51-60), a "Mechanics and primitive-pressure status" table (~L65-71), and the existing Scaffold-Refactor Profile. Verified this session.
2. The spec (D16, plan §5.19) requires: add the register to the authority list; add the "New-game scaffolding reuse/track status" section; add acceptance-evidence, implementation-boundary, documentation-required rows; append forbidden-changes + review checks; **keep the existing Scaffold-Refactor Profile**.
3. Shared contract under audit: the task-packet's reuse/track section must align with AGENT-DISCIPLINE §8B (PREGAT18FORSCA-008) so a packet's status fields mirror the agent protocol's duties.
4. FOUNDATIONS §11 (acceptance invariants) and the bounded-task law under audit: the new section binds new-game packets (even when the profile is not `scaffold-refactor`) to the audit/register/queue duties, keeping scope bounded and reviewable.

## Architecture Check

1. Adding a dedicated reuse/track section (alongside, not replacing, the Scaffold-Refactor Profile) keeps both governance modes available: refactor packets use the profile, new-game packets use the reuse/track section.
2. No backwards-compatibility shim — the section and rows are additive; the Scaffold-Refactor Profile is preserved.
3. `engine-core`/`game-stdlib` discipline untouched; the section keeps the task packet bounded — no "generalize the engine" license.

## Verification Layers

1. Section presence → grep-proof the "New-game scaffolding reuse/track status" section and its rows are present.
2. Profile preservation → grep-proof the Scaffold-Refactor Profile section is unchanged.
3. Doc-link integrity → `node scripts/check-doc-links.mjs` (register authority link resolves).

## What to Change

### 1. Authority list + reuse/track section

Add the register to the authority list; add the "New-game scaffolding reuse/track status" section (plan §5.19 draft) with the audit / register / prior-game disposition status fields.

### 2. Rows + checks

Add the acceptance-evidence, implementation-boundary, and documentation-required rows; append the forbidden-changes entries and review checks.

## Files to Touch

- `templates/AGENT-TASK.md` (modify)

## Out of Scope

- Any change to the existing Scaffold-Refactor Profile section (preserved).
- Editing the other per-game templates.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -ni "reuse/track" templates/AGENT-TASK.md` ≥ 1 (new section).
2. The Scaffold-Refactor Profile section is still present and unchanged (grep-proof).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. New-game packets carry the audit/register/disposition status; the Scaffold-Refactor Profile is preserved.
2. The task packet stays bounded and reviewable.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- templates/AGENT-TASK.md` (review: authority list + reuse/track section + rows + checks; profile untouched)
3. `grep -n "Scaffold-Refactor Profile" templates/AGENT-TASK.md`

## Outcome

Completed. `templates/AGENT-TASK.md` now names
`docs/MECHANICAL-SCAFFOLDING-REGISTER.md` in the authority list, adds the
standing "New-game scaffolding reuse/track status" section, and adds the
scaffolding reuse/register/retrofit receipt, mechanical-scaffolding boundary,
register/CI/follow-on documentation rows, forbidden-change bullets, and review
checks required by the plan. The existing Scaffold-Refactor Profile section
remains present and unmodified.

Verification:

- `grep -ni "reuse/track" templates/AGENT-TASK.md`
- `grep -n "Scaffold-Refactor Profile" templates/AGENT-TASK.md`
- `grep -n "MECHANICAL-SCAFFOLDING-REGISTER\\|scaffolding reuse/register/retrofit\\|mechanical scaffolding\\|ci/scaffolding-audits.json\\|Register, game evidence" templates/AGENT-TASK.md`
- `git diff -- templates/AGENT-TASK.md`
- `node scripts/check-doc-links.mjs`
- `git diff --check`
