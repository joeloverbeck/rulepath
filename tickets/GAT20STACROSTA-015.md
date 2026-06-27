# GAT20STACROSTA-015: Large-board web renderer, previews, animation, and accessibility

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web` board renderer + catalog/state + smoke harness modifications; no Rust/engine surface
**Deps**: GAT20STACROSTA-014

## Problem

The 121-space star board needs a polished React/SVG renderer driven entirely by Rust-supplied legal paths, previews, and semantic effects, with keyboard path construction and accessibility for the full board. This ticket lands the renderer, catalog/state wiring, and the build-step UI/effect smoke updates.

## Assumption Reassessment (2026-06-27)

1. The shell wires renderers via `apps/web/src/components/` (board renderer component), `GamePicker.tsx` (catalog entry), and `shellReducer.ts` (presentation state) — confirmed these files exist; new renderer follows the sibling renderer pattern.
2. `apps/web/scripts/smoke-ui.mjs` carries a custom-renderer **exclusion list** (`["river_ledger","briar_circuit","vow_tide","blackglass_pact","meldfall_ledger"]`, line 185) — Starbridge has a custom board renderer, so it joins that list; `smoke-effect-feedback.mjs` carries per-game effect handling + a display-name regex (line 346) → both are `(modify)` targets.
3. Cross-artifact boundary: the renderer consumes the WASM public view + action tree + effects (014) and the Rust outcome-explanation rule IDs; `client.ts` (viewer-safe rationale mirror) and `outcomeExplanationTemplates.ts` (static copy keys) are the outcome-explanation web surfaces.
4. §2/§7 (behavior authority / public UI) motivates this ticket: TypeScript presents Rust-supplied legal action trees, previews, and effects; it computes no adjacency, enumerates no jumps, validates no paths, decides no legality. Animation is driven by semantic effects, settling to the latest public view.

## Architecture Check

1. A React/SVG renderer consuming Rust previews/effects keeps all legality in Rust and animation effect-driven, matching the §7 default renderer; keyboard path construction (peg → step/hop target → continue → stop) mirrors the action-tree shape.
2. No backwards-compatibility shims.
3. No legality in TypeScript (§2); no `engine-core`/`game-stdlib` change (web-only).

## Verification Layers

1. Renders + updates -> `npm --prefix apps/web run smoke:ui` (121-space board renders; Starbridge in the custom-renderer set).
2. Effect-driven animation (§7) -> `npm --prefix apps/web run smoke:effects` (grouped jump-chain effect handled; display-name regex updated).
3. No-leak DOM (§11) -> the a11y/no-leak e2e smoke (GAT20STACROSTA-017) asserts no hidden fact in DOM/test-ids (perfect info — nothing private).
4. Legality stays in Rust (§2) -> manual review: previews/landings come from Rust, not TS-derived.

## What to Change

### 1. Board renderer component (`apps/web/src/components/`)

Original abstract star board (no trade dress); color + shape/pattern/label for up to six seats; Rust-provided legal previews (full path, landings, jumped-over spaces); grouped multi-hop animation with reduced-motion; keyboard/roving-focus path construction; ARIA labels and target sizing.

### 2. Catalog + state wiring

`GamePicker.tsx` catalog entry; `shellReducer.ts` presentation state; `client.ts` viewer-safe outcome rationale mirror; `outcomeExplanationTemplates.ts` static copy keys.

### 3. Smoke harness updates

Add Starbridge to `smoke-ui.mjs` custom-renderer set; add effect handling + display-name to `smoke-effect-feedback.mjs`.

## Files to Touch

- `apps/web/src/components/` board renderer component (new)
- `apps/web/src/components/GamePicker.tsx` (modify)
- `apps/web/src/state/shellReducer.ts` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)
- `apps/web/scripts/smoke-ui.mjs` (modify)
- `apps/web/scripts/smoke-effect-feedback.mjs` (modify)

## Out of Scope

- The e2e smoke file + `ci/games.json` + `smoke:e2e` wiring + catalog READMEs — GAT20STACROSTA-017.
- Any Rust/engine change (presentation only).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:effects`
3. `npm --prefix apps/web run build`

### Invariants

1. TypeScript decides no legality; previews/landings/effects come from Rust (§2).
2. Animation is semantic-effect-driven and settles to the public view; color is never the sole affordance (§7, a11y).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` — Starbridge custom-renderer assertion (modified).
2. `apps/web/scripts/smoke-effect-feedback.mjs` — grouped jump-chain effect feedback (modified).

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:effects && npm --prefix apps/web run build`
3. The build-step smoke harnesses are the correct boundary for renderer behavior; the full e2e lands in 017.
