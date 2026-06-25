# GAT19MELLEDFIV-020: Web renderer, large-surface UI proof, UI.md, and outcome explanations

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/MeldfallLedgerBoard.tsx`, `GamePicker.tsx`, `main.tsx`, `wasm/client.ts`, `outcomeExplanationTemplates.ts`, `animation/registry.ts`; `games/meldfall_ledger/docs/UI.md`
**Deps**: GAT19MELLEDFIV-019

## Problem

Gate 19 exit requires a usable large private hand + public multi-seat meld tableau + stock/discard zones in the browser, driven entirely by Rust legal actions/previews/effects (no TypeScript legality, no drag-only interaction). This ticket adds `MeldfallLedgerBoard.tsx`, the catalog/shell dispatch, the progressive keyboard-safe action builder, the outcome-explanation surfaces (`client.ts` rationale mirror + `outcomeExplanationTemplates.ts` copy keys), and `UI.md` (including the Outcome/victory-explanation section `check-outcome-explanations` requires).

## Assumption Reassessment (2026-06-25)

1. `apps/web/src/components/{BlackglassPactBoard,RiverLedgerBoard,VowTideBoard}.tsx` are renderer exemplars; `apps/web/src/main.tsx` dispatches boards via a view type-guard chain; `GamePicker.tsx`, `wasm/client.ts`, `components/outcomeExplanationTemplates.ts`, and `animation/registry.ts` exist (confirmed during reassessment). The WASM bridge + catalog const exist from GAT19MELLEDFIV-019.
2. Spec §4.3 (Web catalog row), §6 (larger hand/tableau affordances exit criterion), Appendix E.2 (web renderer minimum), and Appendix B.4 (a11y/keyboard notes) define the surface.
3. Cross-artifact: `check-outcome-explanations.mjs` requires four co-dependent surfaces — `RULES.md` rule IDs (GAT19MELLEDFIV-001), `UI.md` Outcome section (here), the `client.ts` rationale mirror (here), and `outcomeExplanationTemplates.ts` keys (here); landing all four here closes its red window. `apps/web/scripts/smoke-effect-feedback.mjs` carries per-game branches and an expected-game list and is a `(modify)` target.
4. FOUNDATIONS §2/§7: the board renders Rust legal action trees and previews/effects only — it groups/lays out choices but invents no legality; animation is driven by semantic effects, settling to the latest viewer-safe public view.
5. FOUNDATIONS §11 no-leak: no hidden card text, stock order, or hidden action labels in DOM, a11y labels, or `data-testid` values; public tableau a11y labels use score-credit text, not raw hidden identifiers (Appendix B.4).

## Architecture Check

1. A progressive Rust-owned action builder (choose draw source → selected discard → meld/lay-off → discard) renders only Rust-provided legal continuations, keeping legality in Rust and giving a keyboard/click path without drag — the spec's large-surface usability proof.
2. No backwards-compatibility shims.
3. `engine-core` untouched; this is presentation only — Rust-free, but it ships web code, so `Engine Changes` is `Yes (presentation-only)`, not `None`.

## Verification Layers

1. Board renders large hand + public tableau + stock/discard via keyboard-only path, no drag-only -> `npm --prefix apps/web run smoke:ui` + the e2e smoke (GAT19MELLEDFIV-021).
2. Outcome explanation surfaces consistent across the four files -> `node scripts/check-outcome-explanations.mjs`.
3. No hidden text in DOM/a11y/test-ids -> `smoke:effects` no-leak assertions + the e2e a11y/no-leak smoke (GAT19MELLEDFIV-021).

## What to Change

### 1. `MeldfallLedgerBoard.tsx` + dispatch

Board with private-hand zone (or count badges for opponent/public views), stock-count zone, ordered public discard zone with Rust-supplied pickup affordances, public meld tableau grouped by meld/origin/score-credit, score ledger panel, and a progressive keyboard-safe action builder with click/select (no drag-only). Wire the catalog entry in `GamePicker.tsx` and the type-guard + board switch in `main.tsx`.

### 2. Outcome explanations

`wasm/client.ts` viewer-safe rationale mirror and `components/outcomeExplanationTemplates.ts` copy keys for the scoring/terminal rule IDs; effect presenter mapping in `animation/registry.ts` if needed; update `apps/web/scripts/smoke-effect-feedback.mjs` per-game handling.

### 3. `UI.md`

Large-hand/tableau layout, keyboard-only operation, no-drag-required interaction, effect grouping, no-leak a11y labels, and the Outcome/victory-explanation section.

## Files to Touch

- `apps/web/src/components/MeldfallLedgerBoard.tsx` (new)
- `apps/web/src/components/GamePicker.tsx` (modify)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)
- `apps/web/src/animation/registry.ts` (modify)
- `apps/web/scripts/smoke-effect-feedback.mjs` (modify)
- `games/meldfall_ledger/docs/UI.md` (new)

## Out of Scope

- The e2e browser smoke, `ci/games.json`, and catalog README lists (GAT19MELLEDFIV-021).
- Any legality in TypeScript (forbidden — all from Rust/WASM).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` and `smoke:effects` pass with the new board.
2. `node scripts/check-outcome-explanations.mjs` passes (all four surfaces consistent).
3. `npm --prefix apps/web run build` succeeds.

### Invariants

1. No legality is decided in TypeScript; animation is effect-driven (FOUNDATIONS §2/§7).
2. No hidden card text / stock order / hidden labels in DOM, a11y, or `data-testid` (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/MeldfallLedgerBoard.tsx` + `smoke-effect-feedback.mjs` handling — board render + grouped effects.
2. `games/meldfall_ledger/docs/UI.md` — Outcome section validated by `check-outcome-explanations`.

### Commands

1. `npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects`
2. `node scripts/check-outcome-explanations.mjs`
3. `npm --prefix apps/web run build`
