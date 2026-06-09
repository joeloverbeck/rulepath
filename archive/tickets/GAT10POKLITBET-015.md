# GAT10POKLITBET-015: Crest Ledger browser renderer

**Status**: DONE
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/PokerLiteBoard.tsx`, `apps/web/src/wasm/client.ts`, `apps/web/src/main.tsx`, `apps/web/src/components/effectFeedback.ts`, `apps/web/src/components/ActionControls.tsx`. Behavior authority stays in Rust/WASM; TypeScript renders only.
**Deps**: GAT10POKLITBET-014

## Problem

**Crest Ledger** needs a neutral board-game-table renderer that presents the Rust-supplied view: shared pool and contribution ledger, center-crest hidden/revealed state, own private crest in seat view only, grouped showdown reveal, yield terminal without private reveal, and legal-only action controls from the Rust action tree. TypeScript presents; it never decides legality.

## Assumption Reassessment (2026-06-08)

1. The renderer + dispatch pattern is verified this session: `apps/web/src/components/SecretDraftBoard.tsx` is the freshest hidden-info board; `apps/web/src/main.tsx` holds the board imports (~L6–21), the render dispatch (`isSecretDraftView(view) ? <SecretDraftBoard … />`, ~L373–415), and the `is<Game>View()` type guards (~L560–570); `apps/web/src/wasm/client.ts` defines per-game `*PublicView` types and the `PublicView` union (~L538). The new board must wire into all of these.
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §4 Web shell bullet — corrected to name `apps/web/src/main.tsx` and the `PublicView` union — and §E "Browser additions") fixes the renderer contract: neutral surface (no casino felt), shared pool/contribution ledger, center hidden/revealed, own private crest only in seat view, grouped showdown reveal, yield terminal without reveal, legal action buttons from the Rust tree, no-leak-safe test ids (e.g. `choice-poker-lite-round-${round}-${index}`), reduced-motion behavior.
3. Cross-artifact boundary under audit: the WASM view/action/effect JSON contract from GAT10POKLITBET-014, the `main.tsx` dispatch site, and the `client.ts` `PublicView` union. A new `PokerLitePublicView` type joins the union, an `isPokerLiteView()` guard joins the guards, and a render clause joins the dispatch — additive, mirroring `SecretDraft`.
4. FOUNDATIONS §2 (TypeScript presentation-only; never decides legality) and §7 (public UI is play-first, legal-only, cozy not casino; semantic effects drive animation) motivate this ticket. Restated before trusting the spec narrative.
5. No-leak firewall surface under audit (§11/§12): the renderer consumes only the viewer-safe view/effects from WASM; it must not reconstruct or display a hidden card, and `data-testid`/DOM text must use no-leak-safe ids carrying no hidden card identity. The exhaustive DOM/`data-testid`/local-storage no-leak sweep is the e2e ticket (GAT10POKLITBET-016); this ticket must not introduce a leak path in the component itself.

## Architecture Check

1. Reusing the `SecretDraftBoard` + `main.tsx` dispatch + `client.ts` union pattern keeps the renderer consistent with the established shell and ensures legality/views come from Rust. Animation is driven by the Rust semantic effects, not renderer diffs.
2. No backwards-compatibility aliasing/shims — additive component + union member + guard + dispatch clause.
3. `engine-core` untouched (§3); TypeScript adds no legality (§2); no `game-stdlib` change (§4).

## Verification Layers

1. Renderer wiring (board imports, `PokerLitePublicView` in union, `isPokerLiteView` guard, dispatch clause) -> `npm --prefix apps/web run build` (tsc) + codebase grep-proof in `main.tsx`/`client.ts`.
2. Legal-only controls (buttons come from the Rust action tree; none invented) -> manual UI review + `npm --prefix apps/web run smoke:ui`.
3. Viewer-safe rendering (own card only in seat view; center gated; grouped showdown) -> manual viewer-mode review (e2e no-leak in GAT10POKLITBET-016).
4. Neutral presentation (no casino imagery/terms; reduced-motion honored) -> manual IP/UI audit.

## What to Change

### 1. `apps/web/src/components/PokerLiteBoard.tsx` (new)

Render the Crest Ledger surface from the Rust view + action tree + effects: pool/contribution ledger, center hidden/revealed, own private crest (seat view), grouped showdown reveal, yield terminal without reveal, legal action buttons, reduced-motion behavior. Neutral board-game framing.

### 2. `apps/web/src/wasm/client.ts` (modify)

Add the `PokerLitePublicView` type and include it in the `PublicView` union.

### 3. `apps/web/src/main.tsx` (modify)

Add the `PokerLiteBoard` import, the `PokerLitePublicView` type import, an `isPokerLiteView()` type guard, and a render clause dispatching to `PokerLiteBoard` (mirroring `isSecretDraftView`/`isHighCardDuelView`); update the `ActionControls` conditional if needed.

### 4. `apps/web/src/components/effectFeedback.ts` + `ActionControls.tsx` (modify)

Add neutral pledge/reveal effect copy (no casino/chip/payout/ante/blind/rake words); add no-leak-safe `data-testid`s for poker_lite controls.

## Files to Touch

- `apps/web/src/components/PokerLiteBoard.tsx` (new)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/components/effectFeedback.ts` (modify)
- `apps/web/src/components/ActionControls.tsx` (modify)

## Out of Scope

- The e2e smoke, package.json `smoke:e2e` chain, gate-1 e2e step, and catalog README reconciliation (GAT10POKLITBET-016).
- Any Rust/WASM behavior (GAT10POKLITBET-014); TypeScript adds no legality.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` (tsc + vite) succeeds with the new board wired into `main.tsx`/`client.ts`.
2. `npm --prefix apps/web run smoke:ui` passes.
3. Manual viewer-mode review: own crest visible only in seat view; center gated until reveal; showdown is a single grouped reveal; yield shows no private card.

### Invariants

1. All legality/views/effects come from Rust/WASM; the renderer invents no legality (§2).
2. No casino imagery/terminology; reduced-motion honored; controls are legal-only (§7).

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/PokerLiteBoard.tsx` — new renderer (exercised by smoke:ui + e2e in GAT10POKLITBET-016).

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. Note: the full rendered-browser no-leak/a11y e2e is GAT10POKLITBET-016; `smoke:ui` is the correct boundary for renderer wiring here.

## Outcome

Completed on 2026-06-09.

- Added `PokerLiteBoard` as a presentation-only Crest Ledger renderer for the
  Rust/WASM view, action tree, and effects.
- Added `PokerLitePublicView` and supporting client types to the WASM client
  union.
- Wired `PokerLiteBoard` into `main.tsx`, terminal/text-view handling, replay
  snapshot summaries, and no-leak-safe action test ids.
- Added neutral pledge/reveal/ledger feedback copy and responsive/reduced-motion
  styles for the board.
- Source review: private crest rendering uses only `private_view.own_private`,
  observer rendering shows hidden placeholders, center rendering is gated by
  `center.status`, showdown rendering is grouped, yield terminal copy does not
  reveal private crests, and action buttons are mapped only from the Rust action
  tree.

Verification:

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `rg -n "casino|poker|chip|payout|ante|blind|rake|felt" ...` found only
  `poker_lite` identifiers/classes and pre-existing Token Bazaar resource-chip
  styles, not new user-facing forbidden terminology.
