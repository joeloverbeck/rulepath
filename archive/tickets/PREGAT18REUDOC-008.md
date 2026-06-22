# PREGAT18REUDOC-008: FOUNDATIONS §4 scaffolding lane + ENGINE-GAME-DATA-BOUNDARY four-lane split

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs-only (`docs/FOUNDATIONS.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`)
**Deps**: 004, 006

## Problem

With ADR 0008 accepted and the scaffolding register authored, the constitution and boundary doc must carry the lane itself: a `FOUNDATIONS.md` §4 mechanical-scaffolding definition + scaffolding-register invariant, and four explicit reuse lanes in `ENGINE-GAME-DATA-BOUNDARY.md` (kernel ergonomics / scaffolding / behavioral mechanics / typed content) with a narrowest-layer-wins rule. Until these land, the lane has authority (the ADR) but no constitutional text.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/FOUNDATIONS.md` §4 currently carries only the behavioral third-use hard gate (lines 83–89); ADR 0008 (ticket 004) names §4/§11/§12 as amended sections, so this edit lands **only after** ADR 0008 is `Accepted`. Hence `Deps: 004` + acceptance precondition.
2. Verified `docs/ENGINE-GAME-DATA-BOUNDARY.md` has a §13 (game-stdlib promotion boundary) but **no** four-lane structure today (confirmed via `/reassess-spec` this session; spec D9 / WB6).
3. Cross-artifact boundary under audit: the §4 scaffolding-register invariant references `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (ticket 006); hence `Deps: 006`.
4. FOUNDATIONS §4 / §11 motivate this: restating the invariant — the behavioral third-use gate stays **word-for-word effective**; the new §4 paragraph + register invariant are meaning-preserving *given* accepted ADR 0008 (spec Assumption A3), and any §11 meaning change beyond what the ADR names is split out, not landed here.
5. Touches the §4 hard gate, §11 invariants, and §12 stop conditions: confirm the edit routes through ADR 0008's named-section supersession (no editorial weakening), preserves the `boundary-check.sh` guarantee, and opens no leak/nondeterminism path.

## Architecture Check

1. Landing the lane definition in §4 plus the boundary's four-lane model (narrowest-layer-wins) gives one coherent lane doctrine across the constitution and boundary doc, rather than scattered mentions — and it is gated on the accepted ADR, the only lawful path.
2. No backwards-compatibility shims.
3. `engine-core` stays free of mechanic nouns (§3); the behavioral earning rule (§4) is preserved verbatim.

## Verification Layers

1. `FOUNDATIONS.md` §4 carries the scaffolding-lane definition + register invariant -> codebase grep-proof.
2. `ENGINE-GAME-DATA-BOUNDARY.md` enumerates four named lanes + narrowest-layer-wins -> grep.
3. Behavioral third-use wording preserved word-for-word -> manual diff against the pre-edit §4 gate text.
4. ADR 0008 `Accepted` precondition -> grep (`^Status: Accepted` on `docs/adr/0008-*.md`).
5. Kernel boundary intact -> `bash scripts/boundary-check.sh`.
6. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. FOUNDATIONS §4

Add the mechanical-scaffolding lane definition + a scaffolding-register invariant (referencing the register), preserving the behavioral third-use gate text verbatim.

### 2. ENGINE-GAME-DATA-BOUNDARY four lanes

Add the four explicit reuse lanes (kernel ergonomics / scaffolding / behavioral mechanics / typed content) with the narrowest-layer-wins rule.

## Files to Touch

- `docs/FOUNDATIONS.md` (modify)
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` (modify)

## Out of Scope

- The `ARCHITECTURE.md` ownership matrix + `MECHANIC-ATLAS.md` split (ticket 009).
- Any §11/§12 meaning change beyond what ADR 0008 names (split out per spec Assumption A3).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "mechanical scaffolding|scaffolding register" docs/FOUNDATIONS.md` returns the §4 lane definition.
2. `grep -niE "kernel ergonomics|narrowest.layer" docs/ENGINE-GAME-DATA-BOUNDARY.md` returns the four-lane model.
3. `bash scripts/boundary-check.sh` and `node scripts/check-doc-links.mjs` pass.

### Invariants

1. The behavioral third-use hard gate remains word-for-word effective.
2. No mechanic noun enters `engine-core`; `boundary-check.sh` stays green.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep, boundary-check, link check) and behavioral-gate preservation is a manual diff named in Assumption Reassessment.`

### Commands

1. `grep -niE "scaffolding|narrowest.layer|four.*lane" docs/FOUNDATIONS.md docs/ENGINE-GAME-DATA-BOUNDARY.md`
2. `bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs`
3. `boundary-check.sh` is the correct boundary for kernel-noun safety; the greps prove the lane text landed.

## Outcome

Completed: 2026-06-22

Updated `docs/FOUNDATIONS.md` §4 with the ADR 0008 mechanical-scaffolding lane
and scaffolding-register invariant while preserving the behavioral third-use
hard gate word-for-word. Updated `docs/ENGINE-GAME-DATA-BOUNDARY.md` with four
explicit reuse lanes: kernel ergonomics, mechanical scaffolding, behavioral
mechanics, and typed content, plus the narrowest-layer-wins rule.

Deviations: none.

Verification:

- `grep -niE "mechanical scaffolding|scaffolding register" docs/FOUNDATIONS.md`
  returned the §4 lane text.
- `grep -niE "kernel ergonomics|narrowest.layer|four.*lane|mechanical scaffolding|behavioral mechanics|typed content" docs/ENGINE-GAME-DATA-BOUNDARY.md`
  returned the four-lane model and narrowest-layer-wins rule.
- `grep -nF "third official use: hard gate. The game MUST NOT proceed until the primitive-pressure ledger decides reuse, narrow promotion, explicit deferral/rejection with rationale, or ADR escalation." docs/FOUNDATIONS.md docs/adr/0008-mechanical-scaffolding-governance.md`
  confirmed the behavioral third-use hard gate stayed word-for-word effective.
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
