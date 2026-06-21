# GAT17VOWTIDOHHEL-021: Trailing game docs — mechanics, UI, implementation admission

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — game-local docs only (`games/vow_tide/docs/{MECHANICS,UI,GAME-IMPLEMENTATION-ADMISSION}.md`)
**Deps**: 009, 010, 011, 018

## Problem

Author the trailing game documentation: `MECHANICS.md` (full mechanic inventory, variable-seat pressure, bidding first use, promoted helper use, hidden stock/hands), `UI.md` (Rust/React boundary, 3–7-seat layouts, viewer matrix, legal controls, the outcome/victory-explanation section, accessibility/reduced-motion/no-leak), and the final implementation receipt in `GAME-IMPLEMENTATION-ADMISSION.md`.

## Assumption Reassessment (2026-06-21)

1. The implemented behavior these docs describe lands in 005–012/017/018; `templates/GAME-MECHANICS.md`, `templates/GAME-UI.md`, `templates/GAME-IMPLEMENTATION-ADMISSION.md` are the structural source; sibling `games/briar_circuit/docs/{MECHANICS,UI}.md` is the precedent.
2. `scripts/check-outcome-explanations.mjs` requires the "Outcome / victory explanation" section in `UI.md` (the last of its four co-dependent surfaces — `RULES.md` IDs (001), `client.ts` + `outcomeExplanationTemplates.ts` (018) are the others); authoring it here closes that validator's red window.
3. Cross-artifact boundary under audit: `UI.md`'s outcome section + the 018 web surfaces + `RULES.md` scoring IDs form the `check-outcome-explanations` contract; `MECHANICS.md` records the bidding first-use + helper-use mechanic inventory consistent with the §10 atlas rows (002/007).
4. FOUNDATIONS §6 under audit: official games need a complete mechanic inventory + UI notes; the admission receipt records final implementation evidence.

## Architecture Check

1. Trailing these docs after the implementation + web surfaces lets them describe as-built behavior and close the `check-outcome-explanations` window without a drift gap.
2. No shims; new docs.
3. `engine-core`/`game-stdlib` untouched; docs carry no behavior.

## Verification Layers

1. Docs complete + link-clean → `node scripts/check-doc-links.mjs`.
2. `UI.md` outcome section present → `node scripts/check-outcome-explanations.mjs` (now green).
3. Mechanic inventory matches implemented behavior + atlas rows → manual cross-check.

## What to Change

### 1. MECHANICS.md

Full mechanic inventory: variable-seat surface pressure, bidding first use, promoted helper use, hidden stock/hands, outcome and benchmark pressure.

### 2. UI.md

Rust/React boundary, 3–7-seat layouts, viewer matrix, legal bid/card controls, the outcome/victory-explanation section, replay/accessibility/reduced-motion/no-leak details.

### 3. GAME-IMPLEMENTATION-ADMISSION.md

Complete the final implementation receipt (requirements receipt was authored in 001).

## Files to Touch

- `games/vow_tide/docs/MECHANICS.md` (new)
- `games/vow_tide/docs/UI.md` (new)
- `games/vow_tide/docs/GAME-IMPLEMENTATION-ADMISSION.md` (modify)

## Out of Scope

- PUBLIC-RELEASE-CHECKLIST + central atlas/SOURCES + `Done`-flip (022).
- Any game-logic/web-code change.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — docs link-clean.
2. `node scripts/check-outcome-explanations.mjs` — `UI.md` outcome section closes the red window.
3. Manual review: mechanic inventory matches implemented behavior + §10 atlas rows.

### Invariants

1. The `UI.md` outcome section + 018 web surfaces + `RULES.md` scoring IDs satisfy `check-outcome-explanations`.
2. Docs encode no behavior.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; the described behavior is verified by the implementation tickets' suites and the outcome-explanation checker.`

### Commands

1. `node scripts/check-doc-links.mjs && node scripts/check-outcome-explanations.mjs`
2. `node scripts/check-catalog-docs.mjs` (confirms no doc surface regressed)
3. Narrower command rationale: docs are prose; their factual backing is the green implementation suites + the outcome-explanation checker.
