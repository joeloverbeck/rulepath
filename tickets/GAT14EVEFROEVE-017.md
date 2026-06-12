# GAT14EVEFROEVE-017: React board and presentation-shell wiring

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/EventFrontierBoard.tsx` (new); `apps/web/src/components/{ActionControls,effectFeedback,outcomeExplanationTemplates,ReplayViewer}.tsx/.ts` (modify); `apps/web/src/wasm/client.ts` (modify); `apps/web/scripts/smoke-effect-feedback.mjs` (modify)
**Deps**: GAT14EVEFROEVE-014

## Problem

The gate's public surface is the board: an SVG graph map with the current/next card display, eligibility indicators, the constrained-choice menu, progressive op construction (site picking bounded by ops value, Rust-supplied at every step), an edict banner, a Reckoning breakdown panel, asymmetric-victory progress indicators (both factions' distances from Rust view data), outcome-explanation templates and rule-ID mirrors, replay controls with Reckoning/epoch markers, reduced motion, and an accessible responsive layout. TypeScript computes **no** eligibility, cost, edict, scoring, or victory logic — it renders the Rust-supplied tree and effects, and animation is driven by semantic effects.

## Assumption Reassessment (2026-06-12)

1. The WASM bridge and view data this renders exist: verified ticket 014 registered `event_frontier` in wasm-api (catalog, view, action, effect, replay/export) and `apps/web/src/wasm/client.ts` is the typed bridge; the sibling board is `apps/web/src/components/FrontierControlBoard.tsx` with `ActionControls.tsx`/`effectFeedback.ts`/`ReplayViewer.tsx` as the shared presentation surfaces.
2. The outcome-explanation surfaces are current: verified `apps/web/src/components/outcomeExplanationTemplates.ts` and `apps/web/src/wasm/client.ts` exist (the spec's reassessment named both as the concrete outcome-explanation surfaces `check-outcome-explanations` reads), and `apps/web/scripts/smoke-effect-feedback.mjs` carries hardcoded per-game effect playthroughs (modify target here).
3. Cross-artifact boundary under audit: the board consumes the Rust legal tree + view + effects only; progressive op construction is driven by the Rust-supplied tree at every step (no TS-side site-bound or cost math). The outcome-explanation templates (code half) pair with `UI.md`/`RULES.md` (doc half, ticket 016) so `check-outcome-explanations` passes (run at ticket 018).
4. FOUNDATIONS §2 (TypeScript presentation-only) and §7 (legal-only UI; semantic-effect-driven animation; cozy premium, original, accessible) motivate this ticket. Restated before trusting the spec: illegal moves are absent from the UI; animation is driven by Rust-emitted semantic effects and settles to the latest viewer-safe view; no legality is invented in TypeScript.
5. No-leak surface (§11): the DOM/test-IDs/dev panels are leak vectors. Confirm the board never renders or stores undrawn deck order (only the public current/next card + deck count); the no-leak DOM smoke is exercised in ticket 018. Presentation-only — no behavior authority moves to TypeScript.

## Architecture Check

1. Driving the board purely from the Rust tree/effects (progressive op construction Rust-supplied at every step) is cleaner and safer than a TS-side move model: it makes invented legality impossible and keeps animation tied to semantic effects.
2. No backwards-compatibility aliasing/shims — new board component; additive modifications to shared presentation surfaces.
3. `engine-core` stays noun-free; no `game-stdlib` promotion; TypeScript gains no legality logic (§2 preserved).

## Verification Layers

1. Legal-only progressive construction (§7) -> `npm --prefix apps/web run smoke:ui` / `smoke:effects` exercise the constrained menu and multi-site op flow rendered from the Rust tree.
2. Effect-driven animation (§11) -> `smoke-effect-feedback.mjs` plays the `event_frontier` effect sequence (card reveal, op steps, edict banner, Reckoning, terminal) and settles to the latest view.
3. Outcome explanation -> the templates + `client.ts` rationale mirror render the victory-type/tiebreak cause; `check-outcome-explanations` green at ticket 018.
4. No-leak DOM -> the board renders only public card surfaces (deck count, current/next), never undrawn order (full DOM no-leak smoke in ticket 018).

## What to Change

### 1. EventFrontierBoard.tsx

Author the board: SVG graph map (sites/trails, agent/settler/depot/cache markers), card panel (current face, public next, deck count without order), eligibility indicators, the constrained choice menu + progressive op constructor (driven by the Rust tree), edict banner (active modifiers + expiry copy from Rust view data), Reckoning breakdown panel, asymmetric-victory progress indicators (both factions from Rust view data), effect-driven animation, replay controls with Reckoning/epoch markers, reduced motion, responsive accessible layout.

### 2. Shared presentation surfaces

Wire `ActionControls.tsx` (constrained menu / progressive op), `effectFeedback.ts` (event_frontier effect copy), `outcomeExplanationTemplates.ts` (victory-type + tiebreak templates), `ReplayViewer.tsx` (Reckoning/epoch markers), and `apps/web/src/wasm/client.ts` (the typed view/effect/outcome-rationale mirror). Update `apps/web/scripts/smoke-effect-feedback.mjs` with the `event_frontier` effect playthrough.

## Files to Touch

- `apps/web/src/components/EventFrontierBoard.tsx` (new)
- `apps/web/src/components/ActionControls.tsx` (modify)
- `apps/web/src/components/effectFeedback.ts` (modify)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)
- `apps/web/src/components/ReplayViewer.tsx` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/scripts/smoke-effect-feedback.mjs` (modify)

## Out of Scope

- The browser E2E smoke, a11y/no-leak, and catalog README reconciliation (ticket 018).
- Any rule/legality logic in TypeScript — forbidden (§2); the board renders the Rust tree only.
- WASM registration (ticket 014).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` succeeds with the new board.
2. `npm --prefix apps/web run smoke:ui` and `smoke:effects` pass for `event_frontier`.
3. The progressive op constructor renders bounded, legible choices at every step driven by the Rust tree (asserted in the effect/ui smoke).

### Invariants

1. TypeScript computes no eligibility, cost, edict, scoring, or victory logic; the board renders the Rust-supplied tree/effects only.
2. The board never renders or stores undrawn deck order.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-effect-feedback.mjs` — `event_frontier` effect playthrough.
2. `apps/web/src/components/EventFrontierBoard.tsx` + shared-surface wiring — exercised by `smoke:ui`/`smoke:effects`.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects`
3. The build + UI/effects smoke is the correct boundary — the full E2E click-path and DOM no-leak land in ticket 018.
