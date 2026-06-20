# GAT16BRICIRTRI-014: Public web renderer, accessibility, and outcome-explanation copy

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/BriarCircuitBoard.tsx`, `main.tsx`, `wasm/client.ts`, `GameCatalogIcon.tsx`, `outcomeExplanationTemplates.ts`
**Deps**: 013

## Problem

Briar Circuit needs a polished four-seat React/SVG table that maps Rust legal actions and views to controls and visuals without ever reconstructing legality or hidden facts: legal-only card and pass controls, the 2♣ opening, a private hotseat handoff, pending-seat/trick-winner/score/moon/terminal presentation, keyboard operation, accessible names, and reduced motion. It also wires the outcome-explanation web surfaces that `check-outcome-explanations` validates.

## Assumption Reassessment (2026-06-20)

1. `apps/web/src/components/{PlainTricksBoard,RiverLedgerBoard}.tsx` are the renderer exemplars; boards are registered in `apps/web/src/main.tsx` and `apps/web/src/wasm/client.ts`. `SeatFrame.tsx`, `OutcomeExplanationPanel.tsx`, `ReplayImportExport.tsx`, `GameCatalogIcon.tsx`, and `GamePicker.tsx` exist for reuse. `apps/web/src/components/outcomeExplanationTemplates.ts` holds static copy keys; `apps/web/src/wasm/client.ts` holds viewer-safe rationale mirrors (consumed by `scripts/check-outcome-explanations.mjs`).
2. `specs/gate-16-briar-circuit-trick-taking.md` §4.4 (Web renderer/Effects rows), §10.4, Appendix A `BC-UI-001`, and Appendix D (UI/accessibility acceptance details) fix the surface; the WASM catalog/adapter from GAT16BRICIRTRI-013 supplies the legal actions and views.
3. Cross-artifact boundary under audit: the renderer consumes Rust legal action IDs and viewer-safe views only; it must not infer legal cards, broken state, winner, score, pass destination, threshold, or any private fact. It reuses `SeatFrame`/outcome/replay components rather than rebuilding the shell.
4. FOUNDATIONS §2 (TypeScript presentation only) and §7 (legal-only public UI) are under audit: illegal cards are absent/inert; animation is driven by Rust semantic effects, settling to the latest viewer-safe public view; the hotseat handoff overlay never stores or pre-renders the next viewer's hand.

## Architecture Check

1. Reusing `SeatFrame`/`OutcomeExplanationPanel`/`ReplayImportExport` (over a bespoke four-seat shell) keeps the renderer thin and presentation-only, mapping Rust output to controls.
2. No backwards-compatibility aliasing/shims — a new board component + additive renderer registration.
3. `engine-core`/games untouched by TypeScript (§2/§3); the renderer decides no legality and invents no rule state.

## Verification Layers

1. Controls expose legal actions only; no legality derived in TS -> `npm --prefix apps/web run smoke:ui` + boundary review (`BC-UI-001`).
2. Outcome explanation copy present and viewer-safe -> `node scripts/check-outcome-explanations.mjs` (closes once `UI.md` outcome section lands in 017) + `node scripts/check-presentation-copy.mjs`.
3. Accessibility: keyboard card/pass controls, accessible names, reduced motion, hotseat handoff privacy -> `smoke:ui` a11y assertions + manual Appendix D review (full DOM/storage no-leak in 015).

## What to Change

### 1. `apps/web/src/components/BriarCircuitBoard.tsx`

The four-seat table: owner hand as a semantic card list, legal-only card/pass controls with the `0/3…3/3` pass count and a Rust-authorized confirm, four seat rails (active/pending/dealer/leader from Rust), current trick, score/trick history, moon/terminal surfaces, hotseat handoff overlay, reduced-motion path; reuse `SeatFrame`/`OutcomeExplanationPanel`/`ReplayImportExport`.

### 2. Renderer registration

`apps/web/src/main.tsx` and `apps/web/src/wasm/client.ts` — map `briar_circuit` to `BriarCircuitBoard` and add the viewer-safe outcome rationale mirror; `GameCatalogIcon.tsx` neutral icon.

### 3. Outcome-explanation copy

`apps/web/src/components/outcomeExplanationTemplates.ts` — static copy keys for the Briar Circuit outcome (per-seat breakdown, moon, threshold/tie reason, winner).

## Files to Touch

- `apps/web/src/components/BriarCircuitBoard.tsx` (new)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/GameCatalogIcon.tsx` (modify)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)

## Out of Scope

- The e2e smoke, catalog README reconciliation, and `smoke:e2e` wiring (GAT16BRICIRTRI-015).
- Trailing `UI.md`/`MECHANICS.md`/`AI.md` docs (GAT16BRICIRTRI-017) — `check-outcome-explanations` stays red until the `UI.md` outcome section lands there.
- Any Rust/legality change — this ticket is presentation-only.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` — legal-only controls, 2♣ opening, pending seats, trick winner/next leader, score/moon/terminal surfaces.
2. `node scripts/check-presentation-copy.mjs` — no debug vocabulary or raw internal identifiers in display copy.
3. `npm --prefix apps/web run build` — type-checks and builds.

### Invariants

1. TypeScript decides no legality and invents no rule/private state (§2/§7); illegal cards are absent/inert.
2. Animation is driven by Rust semantic effects; the renderer settles to the latest viewer-safe public view (§7/§11).

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/BriarCircuitBoard.tsx` — exercised by `smoke:ui`.
2. `apps/web/src/components/outcomeExplanationTemplates.ts` — outcome copy keys (validated by `check-outcome-explanations` once 017 lands `UI.md`).
3. `apps/web/src/wasm/client.ts` — renderer mapping + viewer-safe rationale mirror.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run build && node scripts/check-presentation-copy.mjs`
3. `smoke:ui` is the correct boundary for renderer behavior; DOM/storage no-leak and the full e2e path are proven in GAT16BRICIRTRI-015.
