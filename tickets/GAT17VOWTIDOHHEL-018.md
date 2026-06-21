# GAT17VOWTIDOHHEL-018: Web renderer, board dispatch, catalog/icon, outcome surfaces

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — new `apps/web/src/components/VowTideBoard.tsx`; modifies `ReplayViewer.tsx`, `GamePicker.tsx`/catalog, `outcomeExplanationTemplates.ts`, `apps/web/src/wasm/client.ts`, `apps/web/scripts/smoke-ui.mjs`, `apps/web/scripts/smoke-effect-feedback.mjs`
**Deps**: 017

## Problem

Add the responsive 3–7-seat `VowTideBoard` renderer and wire it at every `game_id` dispatch site, with the web catalog entry/icon, the viewer-safe outcome-explanation template + rationale mirror, and the bid/card/trick/score presentation. TypeScript renders Rust-supplied legal actions/effects/views and decides no legality.

## Assumption Reassessment (2026-06-21)

1. `apps/web/src/components/` holds per-game boards (`BriarCircuitBoard.tsx`, `RiverLedgerBoard.tsx`) + the shared `SeatFrame.tsx`; `ReplayViewer.tsx` imports each board and branches on `view.game_id` (`:283`+) — a `vow_tide` case/import is required there and at the live board selector. `GameCatalogIcon.tsx` + `GamePicker.tsx` hold catalog/icon wiring.
2. `apps/web/src/components/outcomeExplanationTemplates.ts` + `apps/web/src/wasm/client.ts` are the `check-outcome-explanations` web surfaces (keyed off the 017 catalog const); they co-land here. `apps/web/scripts/smoke-{ui,effect-feedback}.mjs` carry hardcoded per-game assertions → `(modify)`.
3. Cross-artifact boundary under audit: the renderer consumes the 017 WASM views/effects/legal trees + the 009 outcome model; it must render the supplied ranking and never re-sort/re-rank or compute bid/card legality.
4. FOUNDATIONS §2/§7 under audit: TypeScript is presentation-only; the board is `Yes (presentation-only)` because it ships web code; normal UI is legal-only and effect-driven; semantic effects drive animation, renderer diffs are diagnostics only.

## Architecture Check

1. Reusing `SeatFrame` + shared action/replay/outcome surfaces and sizing from the Rust-projected current hand/seat count (not hardcoded 4-seat/10-card) keeps one renderer for all N — cleaner than per-count layouts.
2. No shims; additive board + dispatch arms.
3. `engine-core`/`game-stdlib` untouched; no legality in TypeScript.

## Verification Layers

1. Board mounts for 3–7 seats; legal-only bid/card controls → `npm --prefix apps/web run smoke:ui`.
2. Semantic effects drive bid/play/trick/score feedback → `npm --prefix apps/web run smoke:effects`.
3. Outcome panel renders the Rust ranking without re-sorting → `check-outcome-explanations` (with 021's `UI.md` section) + manual review.
4. No legality computed in TS → manual review of `VowTideBoard.tsx` (no bid-sum/follow-suit logic).

## What to Change

### 1. Renderer + dispatch

`VowTideBoard.tsx`: 3–7-seat ring/rail via `SeatFrame`, public bid rail, trump indicator, current trick, score/history table, viewer selector — all from Rust projections. Register the board at `ReplayViewer.tsx` and the live board selector; add the catalog metadata + neutral original icon + rules link.

### 2. Outcome + effect surfaces

Add the viewer-safe outcome template to `outcomeExplanationTemplates.ts` and the rationale mirror to `client.ts`. Update `smoke-ui.mjs` / `smoke-effect-feedback.mjs` per-game assertions.

## Files to Touch

- `apps/web/src/components/VowTideBoard.tsx` (new)
- `apps/web/src/components/ReplayViewer.tsx` (modify)
- `apps/web/src/components/GamePicker.tsx` (modify)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/scripts/smoke-ui.mjs` (modify)
- `apps/web/scripts/smoke-effect-feedback.mjs` (modify)

## Out of Scope

- E2E smoke + a11y/no-leak + catalog README reconciliation + CI (019).
- Any legality/scoring computation in TypeScript.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — renderer compiles.
2. `npm --prefix apps/web run smoke:ui` — 3–7-seat board, legal-only controls.
3. `npm --prefix apps/web run smoke:effects` — semantic-effect-driven feedback.

### Invariants

1. TypeScript computes no bid/card legality, winner, score, or ranking; it renders Rust-supplied facts.
2. The board sizes from Rust-projected hand/seat count, not hardcoded assumptions.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` / `smoke-effect-feedback.mjs` — vow_tide assertions added.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects`
3. Narrower command rationale: build + UI/effects smokes are the presentation boundary; e2e/no-leak/a11y are the 019 capstone.
