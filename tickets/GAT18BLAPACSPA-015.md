# GAT18BLAPACSPA-015: grouped partnership board renderer and outcome-explanation web surfaces

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/BlackglassPactBoard.tsx`, `apps/web/src/main.tsx`, `apps/web/src/components/ReplayViewer.tsx`, `apps/web/src/wasm/client.ts`, `apps/web/src/components/outcomeExplanationTemplates.ts`
**Deps**: GAT18BLAPACSPA-014

## Problem

Build the grouped partnership browser renderer and wire the outcome-explanation web surfaces: a dedicated `BlackglassPactBoard.tsx` rendering the two team summary regions + four seat frames + current trick + own hand controls from Rust-supplied legal leaves and Rust-derived score/contract/bag fields, the replay-viewer registration, the `client.ts` viewer-safe rationale mirror, and the `outcomeExplanationTemplates.ts` copy keys. TypeScript decides no legality and computes no score (spec §4.5, Appendix E, `BP-UI-*`, candidate task `GAT18-BLAPAC-013`).

## Assumption Reassessment (2026-06-25)

1. Board components are registered in `apps/web/src/main.tsx` (`import { BriarCircuitBoard }:6`) and referenced for replay in `apps/web/src/components/ReplayViewer.tsx`; the sibling `VowTideBoard.tsx`/`BriarCircuitBoard.tsx` are the convention for a multi-seat trick-taking board.
2. `apps/web/src/components/outcomeExplanationTemplates.ts` carries per-game keys with `allowedGameIds` (`briar_circuit.low_score_win:220`); `apps/web/src/wasm/client.ts` holds the viewer-safe rationale mirror — both consumed by `scripts/check-outcome-explanations.mjs`.
3. Cross-artifact boundary under audit: the renderer reads the Rust-authored view/effects/outcome from the GAT18BLAPACSPA-014 bridge; it adds no rule logic. The outcome surfaces (`client.ts`, `outcomeExplanationTemplates.ts`) here + `UI.md` outcome section (GAT18BLAPACSPA-017) + `RULES.md` rule IDs (GAT18BLAPACSPA-001) are the four `check-outcome-explanations` surfaces.
4. FOUNDATIONS §2 (TS presentation-only) / §7 (legal-only UI, semantic-effect-driven animation) motivate this ticket: controls come only from Rust legal leaves; team contract/bags/ranks come only from Rust fields; "needed tricks" is a Rust field, not client subtraction.

## Architecture Check

1. A dedicated grouped renderer composing the shared `SeatFrame` + team brackets (vs. a generic board) gives legible partnerships while keeping seat focus/identity intact; team grouping is presentation, not authorization.
2. No shims; no client-side legality/score computation; no partner cards mounted face-up under `display:none`.
3. `engine-core` untouched; no `game-stdlib` change; web presentation only.

## Verification Layers

1. Controls/score/contract/bags/ranks come from Rust; no client legality/score math -> `npm --prefix apps/web run smoke:ui` + component assertions.
2. Outcome-explanation web surfaces present + viewer-safe -> `npm --prefix apps/web run build` + `node scripts/check-outcome-explanations.mjs` (passes once UI.md lands in 017).
3. Semantic-effect-driven animation; reduced-motion settles to the safe view; no hidden card identity in DOM/props -> effects smoke + manual no-leak review.

## What to Change

### 1. Board renderer

`apps/web/src/components/BlackglassPactBoard.tsx` (new): match header, two Rust-backed team summary regions, four positional seat frames, current trick, own hand + Rust legal controls (blind/bid/card), score/bag history, terminal outcome; color-independent team identity; hotseat private-subtree erasure.

### 2. Shell + replay registration

`apps/web/src/main.tsx` (modify) register the board by game id; `apps/web/src/components/ReplayViewer.tsx` (modify) render Blackglass Pact replays.

### 3. Outcome-explanation surfaces

`apps/web/src/wasm/client.ts` (modify): viewer-safe rationale mirror; `apps/web/src/components/outcomeExplanationTemplates.ts` (modify): `blackglass_pact.*` copy keys with `allowedGameIds`.

## Files to Touch

- `apps/web/src/components/BlackglassPactBoard.tsx` (new)
- `apps/web/src/main.tsx` (modify), `apps/web/src/components/ReplayViewer.tsx` (modify)
- `apps/web/src/wasm/client.ts` (modify), `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)

## Out of Scope

- e2e smoke + catalog README reconciliation + `ci/games.json` (GAT18BLAPACSPA-016).
- `UI.md` outcome section (GAT18BLAPACSPA-017) — the 4th `check-outcome-explanations` surface.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` (board + outcome surfaces compile).
2. `npm --prefix apps/web run smoke:ui` and `smoke:effects` (legal-only controls; effect-driven animation).
3. Component review: no team contract/bags/ranks computed in TypeScript; no hidden card in DOM/props.

### Invariants

1. Every control and every score/contract/bag/rank field originates in Rust; TypeScript only formats.
2. Team grouping is presentation, not authorization; partner cards are never face-up for non-owning viewers.

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/BlackglassPactBoard.tsx` — grouped renderer with Rust-sourced fields.
2. `apps/web/src/components/outcomeExplanationTemplates.ts` — `blackglass_pact` outcome copy keys.
3. `apps/web` smoke:ui / smoke:effects exercise the board.

### Commands

1. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:effects`
3. `check-outcome-explanations` fully greens only after GAT18BLAPACSPA-017 lands `UI.md`; flag the interim red window.
