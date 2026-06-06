# GAT5COLFOUPUB-014: ColumnFourBoard renderer & shell integration

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — new `apps/web/src/components/ColumnFourBoard.tsx`; modify `apps/web/src/wasm/client.ts`, `apps/web/src/main.tsx`, `apps/web/src/components/ReplayViewer.tsx`, `apps/web/src/styles.css`
**Deps**: 012

## Problem

Gate 5 is the first "Rulepath is a real game" milestone: `column_four` needs a polished, accessible, first-class React+SVG board — seven column controls, Rust-provided previews, effect-log-driven drop animation, clear win/draw states, bot explanations, and a Rust-projected replay view — not a generic board fallback (spec §10, §11). All authority stays in Rust/WASM; TypeScript only presents.

## Assumption Reassessment (2026-06-06)

1. `apps/web/src/components/ThreeMarksBoard.tsx` is the template (React+SVG, Rust-projected cells, Rust legal targets, terminal win/draw, bot explanation, reduced-motion) — verified. Game routing is hardcoded in `apps/web/src/main.tsx` (`selectedGameId === "race_to_n" ? … : isThreeMarksView(view) ? …`), per-game view types live in `apps/web/src/wasm/client.ts` (e.g. `ThreeMarksPublicView` with `game_id: "three_marks"`), and `ReplayViewer.tsx` discriminates via `isThreeMarksView`. The game catalog/list is Rust-driven (`GamePicker.tsx` renders `action.catalog`), so it needs no per-game edit.
2. Spec §10 (web UI), §11 (accessibility), and §8 (public view/action/effect) define behavior: seven column controls (not 42 cells), hover/focus landing preview from Rust data, effect-driven drop animation settling to the Rust view, win-line highlight of exactly the Rust line, reduced-motion, keyboard play, no-leak DOM. The WASM ops + `ColumnFour` payload come from GAT5COLFOUPUB-012.
3. Cross-artifact boundary under audit: the WASM/client viewer-safe payload boundary (`docs/WASM-CLIENT-BOUNDARY.md`) and the TS public-view discriminated union in `client.ts`. This ticket adds `ColumnFourPublicView` and a `game_id === "column_four"` discriminant; it adds no behavior helper that decides rules.
4. FOUNDATIONS §2 (behavior authority) and §7 (public UI is play-first, legal-only, effect-driven) motivate this ticket. Restating: TypeScript MUST NOT compute full-column state, landing rows, win lines, draw, terminal, or bot moves — every such value is read from the Rust view/effects; animation is driven by semantic effects and settles to the latest public view.
5. The no-leak visibility firewall (§11) is the enforcement surface: accessibility labels, `data-testid`s, DOM text, storage, and the replay textarea MUST NOT expose hidden/internal/candidate-ranking state — verified by the a11y/no-leak smoke (GAT5COLFOUPUB-015).
6. This extends the existing TS public-view union/discriminant contract: `client.ts` gains `ColumnFourPublicView`, `main.tsx` and `ReplayViewer.tsx` gain a `column_four` discriminant arm. The extension is additive (new union member + new routing arm); existing `race_to_n`/`three_marks` arms are unchanged.

## Architecture Check

1. A dedicated `ColumnFourBoard` with seven column controls (Rust legal/disabled state) and pure coordinate-to-pixel SVG mapping is the only design that keeps TypeScript presentation-only while delivering public polish — cleaner than 42 cell controls (which would invite TS legality inference, spec §10 interaction model). React+SVG is the default; Canvas/Pixi/WebGL are out of scope without an ADR (spec §10, FOUNDATIONS §13).
2. No backwards-compatibility aliasing/shims — new component; additive union member + routing arm.
3. No engine-core/game-stdlib change; all behavior is read from Rust/WASM. The component computes no legality, landing, terminal, or bot decision.

## Verification Layers

1. Presentation-only invariant -> manual review + grep-proof: `ColumnFourBoard.tsx` reads legality/landing/terminal/winning-line/bot-rationale from the Rust view/effects; no TS computes them (no full-column or line-detection logic in the component).
2. Legal-controls invariant -> UI smoke (GAT5COLFOUPUB-015): only legal columns submit; full columns are inert; terminal boards expose no playable control.
3. Effect-driven-animation invariant -> manual review + UI smoke: drop animation derives source/destination from Rust effect/view data and settles to the latest Rust public view; reduced-motion replaces motion without information loss.
4. Win/draw clarity invariant -> UI smoke: win highlights exactly the Rust-provided line; draw renders clearly; status text names winner/draw.
5. Discriminant invariant -> codebase grep-proof: `main.tsx`/`ReplayViewer.tsx` route `column_four` via `game_id === "column_four"`; replay mode reuses `ColumnFourBoard` non-interactively.

## What to Change

### 1. `apps/web/src/components/ColumnFourBoard.tsx`

New React+SVG renderer: 7×6 board, seven column controls with Rust legal/disabled state, hover/focus landing preview from Rust data, effect-log-driven drop animation that settles to the Rust view, win-line highlight, draw state, bot-explanation display, reduced-motion handling, non-color cues, and an interactive/replay (non-interactive) mode.

### 2. `apps/web/src/wasm/client.ts`

Add the `ColumnFourPublicView` type (describing the Rust view; no rule helpers) and a `column_four` discriminant.

### 3. `apps/web/src/main.tsx` + `apps/web/src/components/ReplayViewer.tsx`

Add play-mode routing to `ColumnFourBoard` (`game_id === "column_four"`) and a `ReplayViewer` arm projecting `ColumnFourBoard` in non-interactive replay mode.

### 4. `apps/web/src/styles.css`

Original, neutral visual language for the board/tokens (no blue-rack/red-yellow trade dress, spec §17); reduced-motion-aware animation styles.

## Files to Touch

- `apps/web/src/components/ColumnFourBoard.tsx` (new)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/components/ReplayViewer.tsx` (modify)
- `apps/web/src/styles.css` (modify)

## Out of Scope

- Browser E2E / a11y / no-leak smoke tests (GAT5COLFOUPUB-015) — this ticket builds the surface they assert.
- WASM bridge changes (GAT5COLFOUPUB-012, already landed as a dep).
- `UI.md` documentation (GAT5COLFOUPUB-017).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — the app builds with the new component and types.
2. `grep -nE "column_four|ColumnFourPublicView|ColumnFourBoard" apps/web/src/main.tsx apps/web/src/wasm/client.ts apps/web/src/components/ReplayViewer.tsx` — routing/type/discriminant present.
3. Manual review: no TypeScript computes legality, landing, win line, draw, terminal, or bot choice.

### Invariants

1. All authoritative values are read from the Rust view/effects; animation settles to the latest public view.
2. The primary interaction model is seven column controls; full/terminal columns submit no action.

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/ColumnFourBoard.tsx` — exercised by the GAT5COLFOUPUB-015 browser smoke (the renderer has no standalone unit harness in this shell).

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui` — the existing UI smoke must still pass.
3. Manual play-first audit of the default page (board renders 7×6, seven column controls, preview/animation/win/draw) — the manual boundary is correct pre-E2E; the automated assertions land in GAT5COLFOUPUB-015.
