# PREGAT18FORSCA-005: OFFICIAL-GAME-CONTRACT reuse-first audit + closeout steps

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance/area-doc edit (`docs/OFFICIAL-GAME-CONTRACT.md`)
**Deps**: PREGAT18FORSCA-002, PREGAT18FORSCA-007

## Problem

The official-game workflow asks for a scaffolding decision only "if needed" and has no mandatory reuse-first audit or scaffolding closeout. This ticket inserts the reuse-first audit and scaffolding-closeout steps into the §3 workflow, adds the forward-obligation subsection, and extends the §12 acceptance cluster — so every new game's contract requires the standing lifecycle.

## Assumption Reassessment (2026-06-25)

1. `docs/OFFICIAL-GAME-CONTRACT.md` §3 "Requirements-first implementation workflow" is at L81 (workflow block L84–102); §12 "Official game acceptance check" is at L312 with the mechanic/scaffolding acceptance bullets at L325–326. Verified this session.
2. The spec (D5, plan §5.5) requires: insert reuse-first audit + scaffolding-closeout steps into the §3 workflow block; add a "Mechanical-scaffolding forward obligation" subsection after the workflow explanation; replace the §12 mechanic/scaffolding acceptance bullet cluster.
3. Shared contract under audit: the §3 workflow step ordering (the audit precedes serious implementation; closeout precedes game completion) and the §12 acceptance-cluster bullets that the Gate 1 checker later mirrors.
4. FOUNDATIONS §6 (official games are evidence-heavy) and §11 (acceptance invariants) under audit: the new steps make the reuse-first audit and register-new/queue-or-dispose mandatory contract surfaces, aligned with the §11 invariants landed in PREGAT18FORSCA-002.
5. Cross-doc link: the subsection cites the register's new forward cadence sections; Deps PREGAT18FORSCA-007 so `check-doc-links` heading anchors resolve (the checker validates heading anchors).

## Architecture Check

1. Putting the audit *in the workflow steps* (not an optional note) makes it a gating contract surface, matching FOUNDATIONS §11's "before serious implementation" wording.
2. No backwards-compatibility shim — the §12 cluster is replaced wholesale with the forward-obligation cluster, not aliased.
3. `engine-core`/`game-stdlib` discipline untouched; the workflow keeps Rust the behavior authority — the audit governs scaffolding reuse, not legality.

## Verification Layers

1. Workflow-step presence → grep-proof the §3 block names the reuse-first audit and scaffolding closeout steps.
2. Acceptance-cluster fidelity → grep-proof the §12 cluster names register-new and queue-or-dispose acceptance.
3. Cross-doc anchor integrity (§11) → `node scripts/check-doc-links.mjs` (the register cadence anchors exist via PREGAT18FORSCA-007).

## What to Change

### 1. §3 workflow steps

Insert the reuse-first audit step (before serious implementation) and the scaffolding-closeout step (before game completion) into the §3 workflow block.

### 2. Forward-obligation subsection

Add the "Mechanical-scaffolding forward obligation" subsection after the workflow explanation (plan §5.5 draft), cross-linking the register's forward cadence.

### 3. §12 acceptance cluster

Replace the existing mechanic/scaffolding acceptance bullet cluster with the forward-obligation cluster (audit complete, new shapes registered, prior matches queued or disposed).

## Files to Touch

- `docs/OFFICIAL-GAME-CONTRACT.md` (modify)

## Out of Scope

- Any change to the behavioral primitive-pressure / atlas workflow steps (the audit is parallel, not a replacement).
- Authoring the register cadence sections themselves (PREGAT18FORSCA-007).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -ni "reuse-first audit" docs/OFFICIAL-GAME-CONTRACT.md` ≥ 1 in the §3 workflow.
2. `node scripts/check-doc-links.mjs` passes (register-cadence anchors resolve).
3. The §12 cluster names register-new and queue-or-dispose acceptance (grep-proof).

### Invariants

1. The reuse-first audit is a mandatory workflow step, not an optional note.
2. The §12 acceptance cluster mirrors the §11 forward invariants.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- docs/OFFICIAL-GAME-CONTRACT.md` (review: workflow steps + subsection + §12 cluster)
3. `grep -n "forward obligation" docs/OFFICIAL-GAME-CONTRACT.md`

## Outcome

Completed: 2026-06-25

Changed `docs/OFFICIAL-GAME-CONTRACT.md` so the official-game workflow carries
the Unit 8F forward scaffolding lifecycle:

- inserted the mechanical-scaffolding reuse-first audit before serious
  implementation in the §3 workflow block;
- changed the typed Rust implementation step to use accepted shared scaffolding;
- added the mechanical-scaffolding closeout step before public polish review;
- added the `Mechanical-scaffolding forward obligation` subsection;
- replaced the §12 mechanic/scaffolding acceptance cluster with audit,
  register-new, queue-or-dispose, and debt-disposition checks.

Deviation: none. Behavioral primitive-pressure/atlas workflow remains parallel
and authoritative for behavior-bearing mechanics.

Verification:

- `grep -ni "reuse-first audit" docs/OFFICIAL-GAME-CONTRACT.md` confirmed the
  workflow and acceptance references.
- `grep -n "forward obligation" docs/OFFICIAL-GAME-CONTRACT.md` confirmed the new subsection.
- `rg -n "register-backed|new behavior-free scaffolding|follow-on tracker unit|promotion debt|mechanical-scaffolding closeout" docs/OFFICIAL-GAME-CONTRACT.md` confirmed the §12 cluster and closeout step.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
- `git diff --check` passed.
