# GAT13FROCONASY-015: React board and presentation-shell wiring

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/FrontierControlBoard.tsx` (new); `apps/web/src/components/{ActionControls,effectFeedback,outcomeExplanationTemplates,ReplayViewer}.tsx/.ts` (modify); `apps/web/src/wasm/client.ts` (modify); `apps/web/scripts/smoke-effect-feedback.mjs` (modify)
**Deps**: GAT13FROCONASY-012

## Problem

The browser needs an SVG graph-map renderer and per-faction presentation for Frontier Control: sites as nodes, edges as trails, per-faction unit/stake/fort markers, Rust-supplied supply-connectivity highlighting, per-faction budgeted action panels (faction-specific grouping, remaining-budget display, waiting state), clash/scoring animation driven by semantic effects, the outcome-explanation surface, and replay controls. TypeScript stays presentation-only: no adjacency, connectivity, clash, or score computation in the browser.

## Assumption Reassessment (2026-06-11)

1. `apps/web/src/components/FloodWatchBoard.tsx` is the board exemplar; `ActionControls.tsx` is game-agnostic (Rust-tree-driven) with per-game `game_id` test-ID/terminal branches (verified L51-85); `effectFeedback.ts` registers per-effect feedback via a `switch(payload.type)` (verified flood_watch entries L445-518); `outcomeExplanationTemplates.ts` registers per-game templates with `ruleRefLabel` rule-ID mirrors (verified flood_watch L241-254); `client.ts` defines per-game `*PublicView`/`*OutcomeRationale` types (verified L806/842-860); `ReplayViewer.tsx` dispatches to the board via a type guard. `apps/web/scripts/smoke-effect-feedback.mjs` hardcodes a `flood_watch` effect-count/effect-name block (verified L158-202) — a `(modify)` target here.
2. Spec §Browser and §WASM/browser wiring define the SVG renderer, per-faction panels, supply highlighting from view data, effect-driven animation, and the outcome surface with rule-ID mirrors.
3. Cross-crate boundary under audit: the board consumes the Rust public-view type (`client.ts` `FrontierControlPublicView`), the action tree, and semantic effects; `effectFeedback.ts` entries map the Rust effect kinds (march/patrol/clash/stake/dismantle/muster/reinforce/round-scoring/terminal); the outcome templates interpolate Rust-supplied parameters keyed to the `RULES.md` rule IDs (GAT13FROCONASY-001).
4. FOUNDATIONS §2 (behavior authority) and §7 (public UI) under audit: TypeScript computes no adjacency, connectivity, clash, or score; legal-only controls appear only when the Rust tree contains them; the waiting faction gets a waiting state; the renderer settles to the latest view.
5. §11 no-leak + effect-driven animation enforcement surface: the supply-connectivity highlight comes only from Rust view data (never a TS graph traversal — connectivity is score-bearing behavior); animation is driven by semantic effects with reduced-motion support, not guessed state diffs. Perfect information means no hidden-info leak path, but the connectivity-from-Rust rule still holds.
6. Schema extension: new `effectFeedback.ts` entries, `outcomeExplanationTemplates.ts` keys, and `client.ts` types extend the presentation surfaces additively (new game, no existing consumer changed); the smoke-effect-feedback harness gains a `frontier_control` block.

## Architecture Check

1. Driving the board entirely from the Rust view + effects (vs a TS-side game model) keeps behavior authority in Rust and makes the renderer a pure projection — the §2/§7 contract; an SVG node-and-trail map fits React+SVG v1 default (no Canvas/Pixi ADR needed).
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; this is presentation-only TypeScript — no rule logic, no legality, no scoring in the browser.

## Verification Layers

1. Presentation-only / no-TS-legality (§2/§7) -> `npm --prefix apps/web run smoke:ui` + manual review (no adjacency/connectivity/clash/score computation in TS).
2. Effect-driven feedback -> `npm --prefix apps/web run smoke:effects` (the `frontier_control` effect block asserts each effect renders) + codebase grep-proof (effect entries present).
3. Outcome templates -> codebase grep-proof (`frontier_control.*` template keys + `ruleRefLabel` mirrors present; full `check-outcome-explanations` runs in GAT13FROCONASY-016).
4. Build integrity -> `npm --prefix apps/web run build`.

## What to Change

### 1. FrontierControlBoard.tsx

SVG graph map (nodes from typed layout metadata, edges as trails), unit/stake/fort markers, supplied/cut highlighting from view data, per-faction action panels grouped from the Rust tree, score tracks with breakdown disclosure, budget/round indicators, effect log, replay controls, reduced-motion, responsive layout.

### 2. Shell wiring

`client.ts` `FrontierControlPublicView`/`FrontierControlOutcomeRationale` types; `ReplayViewer.tsx` board dispatch + type guard; `ActionControls.tsx` faction-grouped budgeted-phase support + terminal detection; `effectFeedback.ts` entries for march/patrol/clash/stake/dismantle/muster/reinforce/round-scoring/terminal; `outcomeExplanationTemplates.ts` winner/tiebreak templates with rule-ID mirrors.

### 3. Effects smoke harness

Add a `frontier_control` effect-count/effect-name block to `apps/web/scripts/smoke-effect-feedback.mjs`.

## Files to Touch

- `apps/web/src/components/FrontierControlBoard.tsx` (new)
- `apps/web/src/components/ActionControls.tsx` (modify)
- `apps/web/src/components/effectFeedback.ts` (modify)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)
- `apps/web/src/components/ReplayViewer.tsx` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/scripts/smoke-effect-feedback.mjs` (modify)

## Out of Scope

- Browser E2E smoke, a11y/no-leak, catalog README reconciliation, gate-1 E2E lane (GAT13FROCONASY-016).
- Any Rust/engine behavior change (all logic already lands in GAT13FROCONASY-005–007).
- Player-rules markdown (GAT13FROCONASY-014).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` and `npm --prefix apps/web run smoke:ui` pass.
2. `npm --prefix apps/web run smoke:effects` passes with the `frontier_control` effect block.
3. Manual review confirms no adjacency/connectivity/clash/score computation in TypeScript.

### Invariants

1. TypeScript decides no legality and computes no connectivity/clash/score; the supply highlight comes from Rust view data only (§2/§7).
2. Animation is driven by semantic effects with reduced-motion support; the renderer settles to the latest view (§11).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-effect-feedback.mjs` — `frontier_control` effect-count/effect-name assertions.

### Commands

1. `npm --prefix apps/web run smoke:effects`
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui`
3. The UI/effects smoke harnesses are the correct boundary; the full E2E click-path lands in GAT13FROCONASY-016.

## Outcome

Completed: 2026-06-11

Outcome amended: 2026-06-11

What changed:
- Added `apps/web/src/components/FrontierControlBoard.tsx` with an SVG graph-map renderer, Rust-view-driven site state, supplied/cut stake highlighting, score/budget status, faction panels, grouped Rust legal choices, effect feedback, and terminal outcome explanation rendering.
- Wired Frontier Control into the web shell, replay viewer, terminal-mode checks, TypeScript WASM public-view types, effect feedback, outcome explanation templates, and the effect-feedback smoke harness.
- Kept Frontier browser code presentation-only: TypeScript renders fixed visual site/trail layout and Rust-projected view/effect fields, but does not compute legality, connectivity, clash resolution, scoring, terminal state, or bot policy.

Deviations:
- `apps/web/src/main.tsx`, `apps/web/src/components/ModeControls.tsx`, and `apps/web/src/styles.css` were also touched because the board needs shell dispatch, terminal-mode handling, and responsive presentation styles. No Rust behavior or engine boundary changed.

Verification:
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:effects` passed, including Frontier Control coverage for crew march, stake placement, guard patrol, clash, dismantle, muster, reinforce, round scoring, and terminal effects.
- `npm --prefix apps/web run smoke:ui` passed.

Post-completion refinement:
- During GAT13FROCONASY-016 browser smoke work, the TypeScript Frontier terminal mirror and board outcome rendering were corrected to read Rust's `terminal.winner` field instead of a non-existent `terminal.faction` field.
