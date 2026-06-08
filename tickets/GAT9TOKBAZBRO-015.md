# GAT9TOKBAZBRO-015: TokenBazaarBoard + shell integration + effect log + styles

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/TokenBazaarBoard.tsx` (new), `GamePicker.tsx`, `AppShell.tsx`, `EffectLog.tsx`, `effectFeedback.ts`, `ActionControls.tsx`, `apps/web/src/styles.css` (modify)
**Deps**: GAT9TOKBAZBRO-014

## Problem

This ticket builds the visible Token Bazaar table: a board rendering inventories,
public supply, the market row (contract label / cost chips / point value /
empty-slot state), scores, turn count, active seat, the Rust-sourced legal
controls, and recent accounting effects, plus the game-picker entry and effect-log
labels. Resource state must not rely on color alone, dense controls must be
keyboard reachable, and TypeScript must compute no legality — it only presents the
Rust view/action-tree/effect payloads.

## Assumption Reassessment (2026-06-08)

1. The TS client + catalog + types exist after GAT9TOKBAZBRO-014. The sibling
   `apps/web/src/components/HighCardDuelBoard.tsx` establishes the per-game board
   pattern, and the shell sites `GamePicker.tsx`, `AppShell.tsx`, `EffectLog.tsx`,
   `effectFeedback.ts`, `ActionControls.tsx`, `styles.css` exist (verified
   present). `TokenBazaarBoard.tsx` is `(new)`.
2. The board requirements are fixed by
   `specs/gate-9-token-bazaar-browser-proof.md` → "Browser requirements" (the
   "Main board shows…" list; resource state not by color alone; dense controls
   keyboard reachable; TS must not compute legality/affordability/refill/winner/
   terminal/bot policy). `ActionControls.tsx` is touched only if the generic action
   renderer needs metadata-presentation support (per the spec's conditional).
3. Cross-artifact boundary under audit: the view/action-tree/effect payloads from
   Rust (-006) via WASM (-013) and the TS client (-014). The board consumes these
   read-only; the legal controls come from the Rust action tree, not a TS-built
   list.
4. FOUNDATIONS §2 + §7 (presentation-only; play-first, accessible UI): restating
   before trusting the spec — TypeScript presents Rust payloads and must not invent
   legality; illegal moves are absent/inert (legal-only normal mode); resource
   information uses text/counts/icons-plus-text, not color alone; dense market/
   inventory controls are keyboard reachable. Animation, if any, is driven by the
   Rust semantic effects, settling to the latest public view.
5. Browser no-leak surface: the rendered DOM, controls, and effect log must expose
   no internal/candidate/debug field (none cross the ABI). This ticket is the DOM
   enforcement point; the e2e no-leak/a11y smoke (-016) asserts it across the
   live page.

## Architecture Check

1. A dedicated `TokenBazaarBoard.tsx` consuming Rust payloads (separate from the
   client/catalog ticket) keeps the renderer a focused reviewable diff and matches
   the per-game board pattern; reusing `EffectLog`/`effectFeedback` for accounting
   effect labels avoids a bespoke log.
2. No backwards-compatibility aliasing/shims — new board + additive shell wiring.
3. No engine behavior moves to TypeScript; legality/affordability/refill/winner
   come from Rust. `engine-core`/`game-stdlib` untouched (§2).

## Verification Layers

1. Build + render -> `npm --prefix apps/web run build` and `npm --prefix apps/web run smoke:ui`.
2. Legal-only controls sourced from Rust -> manual review (controls map 1:1 to the
   Rust action tree; no TS-computed availability).
3. Accessibility (color-independence + keyboard reachability) -> a11y checklist in
   the e2e smoke (-016) + manual review against `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`.
4. No-leak DOM -> the e2e no-leak assertion (-016); manual DOM review here.

## What to Change

### 1. `apps/web/src/components/TokenBazaarBoard.tsx` (new)

Render active seat + turn count, both scores, both inventories, central public
supply, the three market slots (label / cost chips / points / empty state), the
Rust-sourced collect/exchange/fulfill controls, recent accounting effects, and the
terminal outcome + tie-break explanation. Text/icon-plus-text for resources;
keyboard-reachable controls.

### 2. `GamePicker.tsx` + `AppShell.tsx` (modify)

List `Token Bazaar` in the picker and route it to `TokenBazaarBoard`.

### 3. `EffectLog.tsx` + `effectFeedback.ts` (modify)

Map the new public accounting effect kinds to labels/icons/text.

### 4. `ActionControls.tsx` (modify, only if needed)

Add metadata-presentation support only if the generic renderer cannot show
cost/gain/points from the Rust action metadata.

### 5. `apps/web/src/styles.css` (modify)

Styles for inventory chips, supply, market cards, and effect rows.

## Files to Touch

- `apps/web/src/components/TokenBazaarBoard.tsx` (new)
- `apps/web/src/components/GamePicker.tsx` (modify)
- `apps/web/src/components/AppShell.tsx` (modify)
- `apps/web/src/components/EffectLog.tsx` (modify)
- `apps/web/src/components/effectFeedback.ts` (modify)
- `apps/web/src/components/ActionControls.tsx` (modify)
- `apps/web/src/styles.css` (modify)

## Out of Scope

- e2e smoke + gate-1 CI registration (GAT9TOKBAZBRO-016).
- Any legality/affordability/refill/winner computation in TypeScript (forbidden — §2).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — board + shell build.
2. `npm --prefix apps/web run smoke:ui` — the UI smoke renders Token Bazaar.
3. Manual review: every control maps to a Rust action-tree node; resource state is
   not color-only; controls are keyboard reachable.

### Invariants

1. TypeScript presents Rust view/action-tree/effect payloads; it computes no
   legality, affordability, refill, winner, terminal outcome, or bot policy (§2).
2. Resource information is conveyed by text/counts/shapes/icons-plus-text, not
   color alone; dense controls are keyboard reachable (§7).

## Test Plan

### New/Modified Tests

1. `None new Rust tests` — presentation ticket; the live page is asserted by the
   e2e no-leak/a11y smoke in GAT9TOKBAZBRO-016.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. The build + UI smoke pair is the correct boundary; the full click-path +
   a11y/no-leak assertions live in the e2e smoke (GAT9TOKBAZBRO-016).
