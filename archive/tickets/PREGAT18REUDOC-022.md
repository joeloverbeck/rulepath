# PREGAT18REUDOC-022: ROADMAP pre-Gate-18 + per-gate scaffolding/trace debt interlocks

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — docs-only (`docs/ROADMAP.md`)
**Deps**: 004, 005, 008, 009, 010, 011

## Problem

`ROADMAP.md` frames Gate 18 but carries no pre-Gate-18 or per-gate scaffolding/trace debt interlocks. With the scaffolding lane and trace/fixture taxonomy landed, the roadmap should add those debt interlocks (without reordering the ladder or becoming a progress diary).

## Assumption Reassessment (2026-06-22)

1. Verified `docs/ROADMAP.md` frames Gate 18 (Spades; partnerships/teams) but has **no** pre-Gate-18 or per-gate scaffolding/trace debt interlocks today (confirmed via `/reassess-spec` this session; spec §Assumptions A9). The interlocks reference the doctrine landed in tickets 004/005 (ADRs accepted), 008/009 (foundation lane), 010/011 (trace/seat authority); hence the `Deps` set.
2. Verified against spec D13 / WB11 (reframed per reassess I2): the ROADMAP interlocks are the genuinely-new index/roadmap work; the `specs/README.md` Part C (`8C`) seed and Gate 18 interlock already exist and are reconciled by the capstone (ticket 023), not here.
3. Cross-artifact boundary under audit: the interlocks reference the scaffolding lane (`FOUNDATIONS` §4 / register), the trace authority (`TESTING`/`TRACE-SCHEMA`/`WASM`), and the accepted ADRs 0008/0009.
4. FOUNDATIONS / spec Forbidden-changes motivate the constraint: keep the public mechanic ladder **unchanged** — add debt interlocks only; do not rewrite `ROADMAP.md` as a progress diary.

## Architecture Check

1. Adding scaffolding/trace debt interlocks (vs reordering the ladder) keeps the roadmap's prescriptive ladder intact while making the pre-Gate-18 debt gate explicit — the minimal robust change.
2. No backwards-compatibility shims.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; docs-only.

## Verification Layers

1. `ROADMAP.md` carries pre-Gate-18 + per-gate scaffolding/trace debt interlocks -> codebase grep-proof.
2. The public mechanic ladder gate rows are unchanged -> manual diff (only interlock prose added).
3. The gating doctrine landed -> grep (`^Status: Accepted` on ADRs 0008/0009; scaffolding-lane text in `FOUNDATIONS.md` §4).
4. Links + boundary intact -> `node scripts/check-doc-links.mjs` and `bash scripts/boundary-check.sh`.

## What to Change

### 1. ROADMAP debt interlocks

Add a pre-Gate-18 scaffolding/trace debt interlock and a per-gate atlas + scaffolding-debt review note to `docs/ROADMAP.md`, leaving the ladder rows unchanged.

## Files to Touch

- `docs/ROADMAP.md` (modify)

## Out of Scope

- The `specs/README.md` Part C (`8C`) seed + Gate 18 interlock (already present; reconciled by the capstone, ticket 023).
- Reordering or rewriting the public mechanic ladder.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "scaffolding.*debt|trace.*debt|pre-Gate-18" docs/ROADMAP.md` returns the interlocks.
2. The Gate 15–23 ladder rows are unchanged (manual diff shows only added interlock prose).
3. `node scripts/check-doc-links.mjs` and `bash scripts/boundary-check.sh` pass.

### Invariants

1. The public mechanic ladder is unchanged; only debt interlocks are added.
2. `ROADMAP.md` is not rewritten as a progress diary.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (interlock grep, ladder diff, link/boundary check) named in Assumption Reassessment.`

### Commands

1. `grep -niE "scaffolding.*debt|trace.*debt|pre-Gate-18" docs/ROADMAP.md`
2. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
3. The interlock grep + ladder diff is the correct boundary; docs-only with the boundary guard as a safety net.

## Outcome

Completed on 2026-06-22. `docs/ROADMAP.md` now has a pre-Gate-18 debt
interlock for mechanical scaffolding debt and trace debt, plus a per-gate debt
review note covering mechanic-atlas pressure, scaffolding debt, trace/fixture
profile debt, seat/viewer grammar debt, replay/hash migration debt, and
evidence-receipt blockers. The product mechanic ladder rows were not changed.

Verification:

1. `grep -niE "scaffolding.*debt|trace.*debt|pre-Gate-18" docs/ROADMAP.md` returned the new interlock prose.
2. `grep -n "^Status: Accepted" docs/adr/0008-mechanical-scaffolding-governance.md docs/adr/0009-replay-fixture-hash-taxonomy.md` confirmed the referenced ADRs are accepted.
3. `grep -niE "mechanical-scaffolding lane|Mechanical-scaffolding decisions" docs/FOUNDATIONS.md` confirmed the scaffolding-lane doctrine exists.
4. `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
5. `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).
6. `git diff -- docs/ROADMAP.md` showed only added interlock prose and no ladder-row edits.
