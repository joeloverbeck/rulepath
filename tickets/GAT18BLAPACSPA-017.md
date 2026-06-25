# GAT18BLAPACSPA-017: trailing game docs — MECHANICS, UI (incl. outcome section), admission and release checklist

**Status**: PENDING
**Priority**: LOW
**Effort**: Medium
**Engine Changes**: None — game-local docs (`games/blackglass_pact/docs/{MECHANICS,UI,GAME-IMPLEMENTATION-ADMISSION,PUBLIC-RELEASE-CHECKLIST}.md`)
**Deps**: GAT18BLAPACSPA-015, GAT18BLAPACSPA-016

## Problem

Finalize the trailing game-local documents now that behavior, WASM, and web surfaces exist: `UI.md` (grouped partnership table, legal-only interaction, Rust-safe previews, effects, replay, hotseat erasure, the complete viewer/no-leak + accessibility matrix, and the **Outcome / victory explanation** section), `MECHANICS.md` final mechanic inventory, the `GAME-IMPLEMENTATION-ADMISSION.md` post-build state, and `PUBLIC-RELEASE-CHECKLIST.md`. The `UI.md` outcome section is the fourth `check-outcome-explanations` surface, closing that red window (spec §4.2, §10.5, Appendix E, candidate task `GAT18-BLAPAC-015`).

## Assumption Reassessment (2026-06-25)

1. `MECHANICS.md` (created in GAT18BLAPACSPA-002) is finalized here; `UI.md`/`PUBLIC-RELEASE-CHECKLIST.md` are new (sibling `games/briar_circuit/docs/` convention). `GAME-IMPLEMENTATION-ADMISSION.md` (created in 002) gains its post-build state.
2. `scripts/check-outcome-explanations.mjs` requires a "Outcome / victory explanation" section in `games/<g>/docs/UI.md` alongside the `RULES.md` rule IDs (001), `client.ts` mirror + `outcomeExplanationTemplates.ts` keys (015); landing `UI.md` here closes the red window opened at GAT18BLAPACSPA-014.
3. Cross-artifact boundary under audit: `UI.md` must agree with the shipped `BlackglassPactBoard.tsx` (015) and the no-leak matrix (008); `MECHANICS.md` must agree with the atlas/register classification finalized in 018.
4. FOUNDATIONS §7 (play-first, accessible UI) / §11 (no-leak) motivate this ticket: `UI.md` records the accessibility acceptance floor and the no-leak datum/surface matrix as documentation of the enforced behavior.

## Architecture Check

1. Authoring the trailing docs after the surfaces exist (vs. up front) keeps them faithful to shipped behavior; co-locating the `UI.md` outcome section here closes the last outcome-explanation surface coherently.
2. No shims; docs only.
3. `engine-core` untouched; no `game-stdlib` change.

## Verification Layers

1. `UI.md` has the Outcome / victory explanation section -> `node scripts/check-outcome-explanations.mjs` (now green).
2. `UI.md` no-leak + accessibility matrix matches shipped behavior -> manual review against §7.5 + Appendix E.10.
3. `MECHANICS.md`/admission agree with atlas/register -> manual cross-check (final receipt in 018); `node scripts/check-doc-links.mjs`.

## What to Change

### 1. UI.md

Grouped partnership table, phase controls, legal-only interaction, Rust-safe previews, effects, replay, public observer, hotseat erasure, the viewer/no-leak matrix, the accessibility acceptance floor, responsive budgets, e2e acceptance, and the **Outcome / victory explanation** section.

### 2. MECHANICS.md + admission

Finalize the mechanic inventory and the post-build implementation-admission state (boundary checks, primitive decisions, required evidence profiles, no active stop condition).

### 3. PUBLIC-RELEASE-CHECKLIST.md

Public rules, IP/assets, catalog, UI, accessibility, no-leak, viewer exports, outcomes, bots, benchmarks, e2e, receipts, and pending human sign-off.

## Files to Touch

- `games/blackglass_pact/docs/UI.md` (new)
- `games/blackglass_pact/docs/MECHANICS.md` (modify; created by GAT18BLAPACSPA-002)
- `games/blackglass_pact/docs/GAME-IMPLEMENTATION-ADMISSION.md` (modify; created by GAT18BLAPACSPA-002)
- `games/blackglass_pact/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)

## Out of Scope

- Repo-level atlas/register/forward-v1 receipt (GAT18BLAPACSPA-018).
- Final exit-criteria run + `Done` flip (GAT18BLAPACSPA-019).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-outcome-explanations.mjs` (green — `UI.md` outcome section present).
2. `node scripts/check-doc-links.mjs` and `node scripts/check-presentation-copy.mjs` pass.
3. `UI.md` records the full viewer/no-leak + accessibility matrix.

### Invariants

1. `UI.md` matches the shipped renderer and no-leak behavior; no client legality/score is implied.
2. `PUBLIC-RELEASE-CHECKLIST.md` carries pending human IP/release review until signed.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; the behavior the docs describe is verified in GAT18BLAPACSPA-008/015/016.`

### Commands

1. `node scripts/check-outcome-explanations.mjs`
2. `node scripts/check-doc-links.mjs && node scripts/check-presentation-copy.mjs`
3. Doc-only ticket — the outcome-explanation + doc-link/presentation checkers are the correct boundary.
