# PREGAT18FORSCA-006: MECHANIC-ATLAS §5B parallel scaffolding check + §11 bullets

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — governance/area-doc edit (`docs/MECHANIC-ATLAS.md`)
**Deps**: PREGAT18FORSCA-002

## Problem

The mechanic atlas governs the behavioral primitive-pressure path but has no parallel mechanical-scaffolding seam, so a new game's atlas stage advancement does not check the scaffolding lane. This ticket adds §5B (parallel mechanical-scaffolding check) and §11 stage-advancement bullets — **while preserving §4 and §5A byte-for-byte**, since the behavioral third-use hard gate is a load-bearing preservation criterion of Unit 8F.

## Assumption Reassessment (2026-06-25)

1. `docs/MECHANIC-ATLAS.md` §4 "First, second, and third use rule" is at L67 with the third-use hard-gate sentence at L73 (`Hard gate. The game MUST NOT proceed until a primitive-pressure ledger decides reuse, promotion, explicit deferral/rejection, or ADR escalation.`); §5 at L77, §5A "Promotion conformance lifecycle" at L88, §11 "Stage advancement check" at L270. No §5B exists yet. Verified this session.
2. The spec (D6, plan §5.6, and §Not-allowed) requires a new §5B "Parallel mechanical-scaffolding check" after §5A and §11 stage-advancement bullets, with **§4 and §5A word-for-word unchanged** (the explicit preservation criterion).
3. Shared contract under audit: the atlas section spine (§5A behavioral conformance, §5B scaffolding parallel) and the §4/§5A byte-exact text — the highest-risk surface in this ticket.
4. FOUNDATIONS §4 (`game-stdlib` earned) under audit: §5B keeps the scaffolding lane parallel and narrower than the behavioral path; it must not paraphrase or weaken the §4 third-use gate.
5. Enforcement / preservation surface: the behavioral third-use hard gate (§4 L73) is the exact text the Gate 1 checker's known-shape fingerprint layer (PREGAT18FORSCA-018) and the capstone (PREGAT18FORSCA-021) prove unchanged. This ticket touches the same doc, so byte-exact preservation of §4 and §5A is the acceptance gate.

## Architecture Check

1. A parallel §5B (not an edit to §4/§5A) is the only design that adds the scaffolding seam without touching the behavioral gate — exactly the separation FOUNDATIONS §4 L93–106 draws between the atlas and the scaffolding register.
2. No backwards-compatibility shim — §5B is additive; §4/§5A are frozen.
3. `engine-core` stays noun-free; §5B reaffirms that scaffolding covers behavior-free plumbing only, never legality/scoring/reveal/turn semantics.

## Verification Layers

1. §4/§5A preservation → source comparison: §4 (esp. L73) and §5A bodies are byte-for-byte unchanged after the edit.
2. Additive §5B placement → grep-proof §5B sits after §5A and before §6.
3. Stage-advancement coverage → grep-proof §11 names the scaffolding-lane advancement check alongside the behavioral one.
4. Doc-link integrity → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. New §5B — Parallel mechanical-scaffolding check

Insert §5B (plan §5.6 draft) after §5A: the parallel scaffolding check (reuse-first audit, first-use registration, queue-or-dispose) stated as an atlas seam parallel to the behavioral §5A conformance lifecycle.

### 2. §11 stage-advancement bullets

Add the scaffolding-lane stage-advancement bullets to §11, alongside the existing behavioral-debt advancement checks.

## Files to Touch

- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- **Any edit to §4 or §5A** (byte-for-byte preservation criterion — Unit 8F must not alter the behavioral third-use hard gate).
- Promoting any helper or changing the primitive-pressure ledger semantics.

## Acceptance Criteria

### Tests That Must Pass

1. §4 (including the L73 hard-gate sentence) and §5A are byte-for-byte unchanged (source comparison against pre-edit).
2. `grep -n "5B" docs/MECHANIC-ATLAS.md` confirms the new section after §5A.
3. `node scripts/check-doc-links.mjs` and `bash scripts/boundary-check.sh` pass.

### Invariants

1. The behavioral third-use hard-gate sentence is byte-identical to its pre-8F form.
2. §5B is parallel and narrower than §4/§5A; it adds no behavior-bearing classification.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `git show HEAD:docs/MECHANIC-ATLAS.md | sed -n '67,100p'` compared against the post-edit §4/§5A region (byte-exact preservation proof)
2. `node scripts/check-doc-links.mjs`
3. `bash scripts/boundary-check.sh`
