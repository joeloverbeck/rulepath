# EFFANITUR-010: Closeout — lift amendments, README, status flips

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — documentation and status surfaces only (`docs/`, `templates/`, `apps/web/README.md`, `specs/`, `brainstorming/`); no code, Rust, or WASM change.
**Deps**: EFFANITUR-001, EFFANITUR-002, EFFANITUR-003, EFFANITUR-004, EFFANITUR-005, EFFANITUR-006, EFFANITUR-007, EFFANITUR-008, EFFANITUR-009

## Problem

The scheduler/orchestration work realizes doctrine that the spec deliberately keeps as lift-ready amendment text "applied at WB10, not before": `docs/UI-INTERACTION.md` §10 acceptance criteria, a new §10A turn-orchestration/pacing doctrine + presentation-shape governance paragraph, §19 acceptance rows, a `templates/GAME-UI.md` adoption-status row, the `apps/web/README.md` surface/smoke updates, the `specs/README.md` index flip to Done, and the brainstorm P1/P4 Done markers. This capstone lands them atomically once all implementation tickets ship (spec §10 / §12 / WB10).

## Assumption Reassessment (2026-06-12)

1. The implementation surfaces this docs ticket describes exist after EFFANITUR-001–009: `apps/web/src/animation/{bursts,scheduler,presenters,registry,settleAssertion}.ts`, the orchestration machine in `main.tsx`/`shellReducer.ts`, the adopters, and `e2e/animation.smoke.mjs` wired into `smoke:e2e`. This ticket asserts them by grep, then lands the amendments.
2. `docs/UI-INTERACTION.md` currently has `## 10. Effect-log-driven animation` then `## 11. Settle-to-view rule` — no §10A (a clean insertion point). `docs/MECHANIC-ATLAS.md` already uses `## 10A. Open promotion-debt register`, so the §10A name is occupied *there* (spec A3) — the new §10A lands in UI-INTERACTION only. `templates/GAME-UI.md` has the "Semantic effect-to-animation mapping" / "Settle-to-view checks" / "Reduced-motion behavior" sections. `apps/web/README.md` has Shell Surface (L52) and Smoke Layers (L104). The lift text is quoted verbatim in spec §10.
3. Cross-artifact boundary under audit: the doc-governed UI law (`UI-INTERACTION.md` §10/§10A/§19), the per-game template (`GAME-UI.md`), the `specs/README.md` index row, and the source brainstorm. The README adoption matrix is shared with EFFANITUR-008 (this ticket `Deps` it; create-then-modify ordering holds via the leaf set).
4. FOUNDATIONS §13: the spec records C3 (staged multi-target encoding) and C4 (visibility-contract moves) as dormant ADR triggers and defers the §4 presentation-helper promotion via the §10A governance paragraph (presentation shapes are not mechanic-atlas pressure). This closeout lands that governance text without making any architecture-changing decision itself.

## Architecture Check

1. A single trailing docs capstone landing the lift text atomically — after the machinery it describes exists — keeps UI law from describing non-existent behavior (the reason the spec defers amendments to WB10), cleaner than per-ticket doc edits that would each describe a half-built scheduler.
2. No backwards-compatibility shim: doctrine is amended in place; no parallel legacy doc path.
3. `engine-core` untouched; all surfaces are docs/templates/index/status (§3). No `game-stdlib` promotion — the §10A governance paragraph records exactly why presentation helpers stay out of the atlas (§4).

## Verification Layers

1. §10 acceptance criteria + §10A doctrine + §19 rows present in `UI-INTERACTION.md` -> codebase grep-proof (exact-string match of the lifted headings/clauses) + `node scripts/check-doc-links.mjs`.
2. `GAME-UI.md` adoption-status row present -> grep-proof.
3. `apps/web/README.md` names the scheduler/orchestration surface + animation smoke -> grep-proof + `node scripts/check-catalog-docs.mjs`.
4. `specs/README.md` row flipped Planned→Done with evidence; brainstorm P1/P4 marked Done -> grep-proof on both files.
5. Presentation-copy + boundary guards stay green -> `node scripts/check-presentation-copy.mjs` + `bash scripts/boundary-check.sh`.

## What to Change

### 1. UI-INTERACTION amendments

Apply the spec §10 lift text to `docs/UI-INTERACTION.md`: §10 scheduler acceptance criteria, a new `## 10A. Turn orchestration and pacing` (with the presentation-shape governance paragraph), and the §19 acceptance rows.

### 2. Template + README

Add the adoption-status row to `templates/GAME-UI.md`; update `apps/web/README.md` Shell Surface (scheduler, orchestration, burst grouping) and Smoke Layers (animation smoke), alongside the adoption matrix from EFFANITUR-008.

### 3. Index + brainstorm status flips

Flip the `specs/README.md` row for this spec from `Planned` to `Done` with evidence; in `brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md` mark P1 (§4) **Status: DONE** naming this spec, mark §5 sequence rows 1/3/4 and §8 next-step 1 Done, and mark the P4 register Done (P2's row untouched).

## Files to Touch

- `docs/UI-INTERACTION.md` (modify)
- `templates/GAME-UI.md` (modify)
- `apps/web/README.md` (modify; shared with EFFANITUR-008)
- `specs/README.md` (modify)
- `brainstorming/2026-06-12-deferred-ui-infrastructure-priorities.md` (modify)

## Out of Scope

- Any production code change (EFFANITUR-001–009 own all implementation; this ticket only documents and flips status).
- Writing the C3/C4 ADRs (recorded as dormant triggers only — spec §13).
- Editing the source spec body (the spec is the contract, not an output of this ticket).
- P2's brainstorm row (it remains the next candidate spec).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`, `node scripts/check-presentation-copy.mjs`, and `bash scripts/boundary-check.sh` all pass.
2. Grep-proof: `## 10A. Turn orchestration and pacing` exists in `UI-INTERACTION.md`; `specs/README.md` shows this spec `Done`; brainstorm P1/P4 show Done.
3. `npm --prefix apps/web run smoke:e2e` green (regression — the doc lift does not touch code).

### Invariants

1. UI law is amended only after the machinery it describes exists (no doctrine for non-existent behavior).
2. No architecture-changing decision is made here; C3/C4 stay dormant ADR triggers (§13).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (doc-link / catalog-docs / presentation-copy / boundary checks) and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && node scripts/check-presentation-copy.mjs && bash scripts/boundary-check.sh`
2. `npm --prefix apps/web run smoke:e2e`
3. A docs-only closeout is correctly verified by the doc/boundary check scripts plus a regression smoke; no new unit test is the right boundary since this ticket adds no code.

## Outcome

Completed on 2026-06-12.

Lifted the effect-animation closeout doctrine into `docs/UI-INTERACTION.md`: §10 now names the shared scheduler as the single owner of play-path animation/pacing, the new `## 10A. Turn orchestration and pacing` governs bot/automation/autoplay/replay pacing plus presentation-shape promotion discipline, and §19 now includes scheduler, auto-advance, input-not-blocked, and reduced-motion acceptance rows.

Updated `templates/GAME-UI.md` with the scheduler adoption/orchestration adoption status text. Updated `apps/web/README.md` Shell Surface and Smoke Layers for the scheduler/orchestration surface, `smoke:animation`, and `e2e/animation.smoke.mjs`. Flipped `specs/README.md` for `effect-animation-and-turn-orchestration.md` to `Done` with evidence. Marked brainstorm P1 and P4 done, and updated the sequence/next-step rows while leaving P2 as the next candidate.

During verification, `check-catalog-docs.mjs` correctly exposed that the new non-game `animation.smoke.mjs` needed a non-game smoke whitelist entry, and `check-presentation-copy.mjs` exposed debug-vocabulary in the Event Frontier/Flood Watch registration helpers. Those guard-facing fixes were included so the closeout checks remain meaningful.

Verification:

1. `node scripts/check-doc-links.mjs` -> passed.
2. `node scripts/check-catalog-docs.mjs` -> passed.
3. `node scripts/check-presentation-copy.mjs` -> passed.
4. `bash scripts/boundary-check.sh` -> passed.
5. `npm --prefix apps/web run build` -> passed.
6. `npm --prefix apps/web run smoke:e2e` -> passed.
7. Grep-proof: `## 10A. Turn orchestration and pacing` exists in `docs/UI-INTERACTION.md`; `specs/README.md` shows `effect-animation-and-turn-orchestration.md` as `Done`; brainstorm P1/P4 have `Status: DONE`; `apps/web/README.md` names `smoke:animation` and `animation.smoke.mjs`; `templates/GAME-UI.md` names scheduler/orchestration adoption status.
