# GAT7DRALITCOM-018: DraughtsLiteBoard renderer, multi-step input model & dev-panel/replay multi-segment rendering

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/DraughtsLiteBoard.tsx` (new) and `apps/web/src/components/{GamePicker,MatchSetup,DevPanel,ReplayImportExport}.tsx`, `apps/web/src/main.tsx`, `apps/web/src/state/shellReducer.ts`, `apps/web/src/styles.css` (modify). No Rust/engine surface; legality stays in Rust.
**Deps**: 016

## Problem

The public web board must let a user construct a compound move by traversing the Rust action tree — select a legal origin, then a landing, following only Rust-provided continuation children for a forced multi-jump — and submit the complete leaf path as one command. TypeScript presents the Rust-provided tree, metadata, view, and effects; it never computes diagonals, captures, continuation, promotion, or terminal outcomes (FOUNDATIONS §2/§7). The dev panel and replay UI must render multi-segment paths legibly without truncating after the first segment.

## Assumption Reassessment (2026-06-07)

1. `apps/web/src/wasm/client.ts:283` types `action_path: string[]` and the WASM bridge (GAT7DRALITCOM-016) exposes the recursive action tree, view, effects, and multi-segment apply. `apps/web/src/components/{ColumnFourBoard,DirectionalFlipBoard}.tsx`, `GamePicker.tsx`, `MatchSetup.tsx`, `DevPanel.tsx`, `ReplayImportExport.tsx`, `main.tsx`, and `state/shellReducer.ts` are the structural precedents for a board component + picker/setup/dev/replay wiring.
2. The input/render contract is fixed by spec §R14 ("Input model" pointer steps 1–7; "Board rendering" cue list incl. all 64 cells, playable/non-playable, ownership, men/kings, selected origin, legal origins/destinations, forced continuation, recent path, captures, promotion, terminal; "No TypeScript legality" allow/deny lists; "Dev panel and replay UI" multi-segment legibility).
3. Cross-artifact boundary under audit: the renderer consumes the Rust action tree / choice metadata / public view / effects from WASM (016); the pending-path UI state is TS-only and is discarded on cancel — it is never a replay command (spec §R10 "Partial UI selections are UI state only"). Accessibility/reduced-motion/E2E land in GAT7DRALITCOM-019 (same component surface).
4. FOUNDATIONS §2/§7 motivate this ticket: restate before coding — TS may traverse the tree, store the pending path, map metadata to CSS/labels, display Rust-provided legal destinations, submit a complete path, and show diagnostics/effects; TS must NOT determine diagonals, playability beyond rendering Rust/public board data, captures, continuation, promotion, terminal outcomes, or bot moves. Animation is driven by Rust effects (§7), not state diffs.
5. No-leak enforcement surface (§11): the board renders only the Rust-projected public view + viewer-safe action metadata/previews (GAT7DRALITCOM-009/006); confirm no hidden/internal state is read into the DOM, and that the replay export UI round-trips multi-segment commands without corruption.

## Architecture Check

1. Driving the input model entirely from the Rust action tree's `next` chain (the UI advances to the selected choice's children) structurally guarantees the UI can only build paths Rust exposed — forced continuation is enforced by there being no other legal child, not by TS logic.
2. No backwards-compatibility shims; a new board component plus additive shell wiring.
3. `engine-core` is untouched (§3); this is presentation-only. No legality, capture, or promotion computation enters TypeScript (§2, spec §R14) — behavior authority stays in Rust.

## Verification Layers

1. Pointer path construction -> browser E2E (in 019) + manual runbook: clicking a legal origin then a legal landing submits a quiet move; a forced multi-jump keeps the pending path and shows only Rust continuation children until the leaf submits.
2. No TS legality -> code review + grep-proof: `DraughtsLiteBoard.tsx` contains no diagonal/capture/continuation/promotion/terminal computation; it reads Rust-provided metadata only (FOUNDATIONS §2, spec §R14).
3. Multi-segment replay UI -> manual review: dev panel / replay viewer render full `from/ → jump/ → jump/` paths without truncation; replay export round-trips.
4. Effect-driven render -> FOUNDATIONS alignment check: animations/highlights consume Rust effects (009/008), settling to the latest public view (§7).

## What to Change

### 1. DraughtsLiteBoard renderer

Add `DraughtsLiteBoard.tsx` rendering all 64 cells with the §R14 cue set from the Rust public view + action-tree metadata (legal origins/destinations, selected origin, forced continuation, captures, promotion, terminal). Non-playable cells render non-interactive.

### 2. Input model & pending-path state

Implement the §R14 pointer steps over the action tree: select origin → advance to children; append landing segments; submit on leaf; keep pending path + show only Rust continuation children mid-sequence; Escape/cancel clears the TS-only pending path. Wire pending-path state into `shellReducer.ts`.

### 3. Picker/setup/dev/replay wiring

Register Draughts Lite in `GamePicker.tsx` / `MatchSetup.tsx` / `main.tsx`; render multi-segment paths in `DevPanel.tsx` / `ReplayImportExport.tsx` without first-segment truncation; add board styling to `styles.css`.

## Files to Touch

- `apps/web/src/components/DraughtsLiteBoard.tsx` (new)
- `apps/web/src/components/GamePicker.tsx` (modify)
- `apps/web/src/components/MatchSetup.tsx` (modify)
- `apps/web/src/components/DevPanel.tsx` (modify)
- `apps/web/src/components/ReplayImportExport.tsx` (modify)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/state/shellReducer.ts` (modify)
- `apps/web/src/styles.css` (modify)

## Out of Scope

- Accessibility semantics, keyboard navigation, reduced motion, and the browser E2E/a11y smoke (GAT7DRALITCOM-019 — same components, separate reviewable diff).
- Any legality/rules computation in TypeScript (forbidden; FOUNDATIONS §2, spec §R14).
- WASM bridge changes (GAT7DRALITCOM-016).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — the web app builds with the new board + wiring.
2. `npm --prefix apps/web run smoke:ui` — UI smoke passes (board renders from Rust view).

### Invariants

1. TypeScript computes no legality; the UI only constructs paths the Rust action tree exposes (FOUNDATIONS §2; spec §R14).
2. Multi-segment paths render fully in dev/replay UI; pending-path UI state is never a replay command (FOUNDATIONS §7; spec §R10).

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/DraughtsLiteBoard.tsx` — new renderer; behavior covered by the UI/E2E smoke.
2. `None additional in this ticket — the playable-path and forced-capture E2E assertions land in GAT7DRALITCOM-019.`

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. Build + UI smoke are the correct boundary here; the click-path/keyboard/a11y/no-leak browser assertions are GAT7DRALITCOM-019.
