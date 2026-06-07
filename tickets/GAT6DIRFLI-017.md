# GAT6DIRFLI-017: DirectionalFlipBoard renderer & shell integration

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/DirectionalFlipBoard.tsx` (new) plus shell integration in `apps/web/src/components/{AppShell,GamePicker,ReplayViewer,effectFeedback}` , `apps/web/src/{main.tsx,state/shellReducer.ts,styles.css}`. No Rust/engine surface; no legality in TypeScript.
**Deps**: 015

## Problem

`directional_flip` needs a polished, accessible, play-first public renderer equal to Column Four (FOUNDATIONS §7, spec §12.3): React + SVG board, Rust-driven legal-cell display, exact flip previews on focus/hover from Rust payloads, grouped flip animation with reduced-motion fallback, forced-pass control when Rust exposes it, non-color-only seat encoding, keyboard grid navigation, and no TypeScript legality. This ticket builds the renderer and wires it into the app shell. It is presentation-only.

## Assumption Reassessment (2026-06-07)

1. `apps/web/src/components/ColumnFourBoard.tsx` is the renderer precedent; the shell files exist (`AppShell.tsx`, `GamePicker.tsx`, `ReplayViewer.tsx`, `effectFeedback.ts`, `main.tsx`, `state/shellReducer.ts`, `styles.css` — all confirmed present). The `directional_flip` WASM client types/wrappers exist from GAT6DIRFLI-015.
2. Spec §12.3 (public UI requirements + keyboard table) and §7.4 (viewer-safe views) are authoritative. WAI-ARIA grid + WCAG 2.2 (use-of-color, animation-from-interactions) guidance from spec §4 shapes the accessibility behavior.
3. Cross-artifact boundary under audit: the renderer consumes the Rust public view, legal-target metadata, previews, and grouped flip effects via `apps/web/src/wasm/client.ts` (015); it adds presentation only. `GamePicker.tsx`/`shellReducer.ts` register the new game in the catalog/state machine; this must not enable picker exposure before the release checklist (gated by GAT6DIRFLI-021).
4. FOUNDATIONS §2 (TS presentation-only) and §7 (Rust supplies legal trees/previews/effects; TS presents; animation driven by semantic effects, not state diffs) motivate this ticket: restate before coding — the renderer computes no legal placements, no pass availability, no flip consequences, and synthesizes no action paths (spec §12.2 forbidden); it renders Rust-provided choices and animates from grouped Rust flip effects, settling to the latest viewer-safe public view.

## Architecture Check

1. A dedicated `DirectionalFlipBoard.tsx` mirroring the `ColumnFourBoard` contract (props = Rust view + effects + action submitter) keeps legality in Rust and confines this diff to presentation, reusing the shell's existing renderer-switch pattern.
2. No backwards-compatibility shims; new component + additive shell wiring.
3. `engine-core` untouched; no behavior authority moves to TypeScript (§2). Animation authority is the Rust grouped flip effect, not a renderer state diff (§7/§11).

## Verification Layers

1. No-TS-legality invariant -> manual review + codebase grep-proof (spec §12.2): the component contains no legal-placement/pass/flip computation or action-path synthesis.
2. Effect-driven animation -> manual review (§7/§11): grouped flip animation is driven by the Rust `DiscsFlipped` effect; renderer settles to the latest public view.
3. Accessibility -> manual review against spec §12.3 keyboard table + WCAG: arrow/Home/End/Enter/Space/Tab/Escape behavior, roving focus, non-color-only seat encoding, visible focus, reduced-motion fallback. (Automated a11y smoke is GAT6DIRFLI-018.)
4. Build -> simulation/CLI run: `npm --prefix apps/web run build` succeeds with the new component.

## What to Change

### 1. DirectionalFlipBoard renderer

`apps/web/src/components/DirectionalFlipBoard.tsx`: React + SVG 8×8 board, legal cells visibly marked from Rust metadata, exact flip preview on focus/hover from Rust preview payloads, selected/just-placed and flipped-disc highlights from Rust effects, grouped flip animation with reduced-motion fallback, forced-pass control when Rust exposes it, non-color-only seat encoding (color + shape/pattern), keyboard grid per spec §12.3, visible focus, screen-reader labels from Rust metadata.

### 2. Shell integration

Wire the renderer into `AppShell.tsx`, register the game in `GamePicker.tsx` and `state/shellReducer.ts`, map effects in `effectFeedback.ts`, ensure `ReplayViewer.tsx` handles directional-flip traces, add styles in `styles.css`, and reference from `main.tsx` as needed.

## Files to Touch

- `apps/web/src/components/DirectionalFlipBoard.tsx` (new)
- `apps/web/src/components/AppShell.tsx` (modify)
- `apps/web/src/components/GamePicker.tsx` (modify)
- `apps/web/src/components/ReplayViewer.tsx` (modify)
- `apps/web/src/components/effectFeedback.ts` (modify)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/state/shellReducer.ts` (modify)
- `apps/web/src/styles.css` (modify)

## Out of Scope

- Browser E2E / a11y-no-leak smoke (GAT6DIRFLI-018).
- Public picker *exposure gating* / release checklist (GAT6DIRFLI-021) — registration here must not flip public exposure before the checklist passes.
- Any Rust/legality change (FOUNDATIONS §2).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — app builds with the new renderer.
2. `npm --prefix apps/web run smoke:ui` — existing UI smoke still passes.
3. Manual review: no TS legality/flip/pass computation; animation is effect-driven; keyboard + non-color-only encoding present.

### Invariants

1. TypeScript computes no legality/pass/flip and synthesizes no action paths (FOUNDATIONS §2, spec §12.2).
2. Animation is driven by Rust semantic effects; the renderer settles to the latest viewer-safe public view (FOUNDATIONS §7/§11).

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/DirectionalFlipBoard.tsx` — presentation component; behavior covered by the browser smoke in GAT6DIRFLI-018.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. Build + existing UI smoke is the in-ticket boundary; the directional-flip-specific play/keyboard/no-leak E2E is GAT6DIRFLI-018.
