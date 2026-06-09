# GAT101PLATRI-017: Plain Tricks browser renderer

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — new `apps/web/src/components/PlainTricksBoard.tsx`; modifies `apps/web/src/main.tsx`, `apps/web/src/wasm/client.ts`, `apps/web/src/components/effectFeedback.ts`, `apps/web/src/components/ActionControls.tsx`. No Rust/engine behavior; legality stays in Rust/WASM.
**Deps**: GAT101PLATRI-016

## Problem

The browser needs a neutral, polished Plain Tricks renderer: own hand (seat view), opponent hand as a face-down count, the current-trick surface, a trick/score ledger, legal-only play controls sourced from the Rust action tree, deal/trick/score effect feedback, an outcome-explanation surface, viewer modes, dev-panel whitelist behavior, replay import/export controls, and reduced-motion behavior — driven entirely by Rust-supplied views/actions/effects.

## Assumption Reassessment (2026-06-09)

1. `apps/web/src/main.tsx` imports nine board components (`PokerLiteBoard` etc.) and selects a renderer per `game_id`; `apps/web/src/wasm/client.ts` has a `PublicView` union (+ `*OutcomeRationale` types); `apps/web/src/components/effectFeedback.ts` and `ActionControls.tsx` exist. The WASM bridge + `GAME_PLAIN_TRICKS` const from GAT101PLATRI-016 supply the data.
2. Spec §4 (web shell) and appendix D fix the additions: `PlainTricksBoard.tsx`; `PlainTricksPublicView` added to `client.ts` (and its `PublicView` union); `main.tsx` gets a `PlainTricksBoard` import + `isPlainTricksView()` guard + render clause + `ActionControls` handling; `effectFeedback.ts` neutral copy for deal/play/trick/score/rotation/terminal; `ActionControls.tsx` no-leak-safe test ids (`choice-plain-tricks-trick-${trick}-${index}`).
3. Shared boundary under audit: the `client.ts` `PublicView` union (additive extension mirroring the Rust public view) and the `ActionControls` test-id convention (`choiceTestId`). The renderer consumes Rust-supplied legal actions; it never computes legality.
4. FOUNDATIONS §2 (TypeScript presentation-only; never decides legality) and §7 (animation driven by Rust semantic effects; legal-only controls; cozy board-game-table visuals) are under audit.
5. Enforcement surface: §11 no-leak firewall at the DOM/test-id/dev-panel layer. The renderer must never place an unplayed opponent card id, suit/rank label, or tail card into DOM text, accessibility names, `data-testid`, local storage, or dev-panel data; opponent hand renders as a face-down count only. Dev panel stays whitelisted. (Exhaustive e2e no-leak checks land in GAT101PLATRI-018.)
6. Extends the `client.ts` `PublicView` union additively with `PlainTricksPublicView`/`PlainTricksOutcomeRationale` (TS mirror of the Rust view); no existing view type changes.

## Architecture Check

1. A dedicated `PlainTricksBoard` consuming Rust views/actions/effects (vs. generic rendering) lets the trick surface and legal-only controls be precise while keeping all behavior in Rust; effect-driven animation settles to the latest viewer-safe view.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; this is presentation-only TS — no legality, scoring, or redaction in TypeScript (FOUNDATIONS §2).

## Verification Layers

1. Legal-only controls sourced from the Rust tree (no TS-invented legality) -> manual review + `smoke:ui`.
2. Opponent hand renders as face-down count; no hidden id in DOM/test-id/dev-panel -> no-leak manual review (exhaustive e2e in GAT101PLATRI-018).
3. Effect-driven feedback + outcome-explanation surface render -> `npm --prefix apps/web run smoke:ui`.
4. Reduced-motion + viewer modes -> manual review / smoke.

## What to Change

### 1. `apps/web/src/components/PlainTricksBoard.tsx`

Render own hand (seat view), opponent face-down count, current-trick surface, trick/score ledger, legal-only play controls from the Rust tree, deal/trick/score effect feedback, the outcome-explanation surface, viewer modes, dev-panel whitelist behavior, replay import/export controls, and reduced-motion behavior.

### 2. `apps/web/src/wasm/client.ts`

Add `PlainTricksPublicView` (+ `PlainTricksOutcomeRationale`) and include it in the `PublicView` union.

### 3. `apps/web/src/main.tsx`

Add the `PlainTricksBoard` import, `isPlainTricksView()` guard, render clause, and `ActionControls` handling alongside the existing hidden-info games.

### 4. `apps/web/src/components/effectFeedback.ts` + `ActionControls.tsx`

Add neutral effect copy for deal/play/trick/score/rotation/terminal; add the `choice-plain-tricks-trick-${trick}-${index}` no-leak-safe test id (never raw card ids in non-actor contexts).

## Files to Touch

- `apps/web/src/components/PlainTricksBoard.tsx` (new)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/effectFeedback.ts` (modify)
- `apps/web/src/components/ActionControls.tsx` (modify)

## Out of Scope

- e2e smoke + catalog README reconciliation + gate-1 e2e step (GAT101PLATRI-018).
- Any Rust behavior; the renderer consumes WASM output only.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` and `npm --prefix apps/web run smoke:ui` pass with the Plain Tricks renderer.
2. Controls are legal-only (sourced from the Rust tree); no illegal play is clickable.
3. Opponent hand renders as a face-down count; no unplayed card id/label appears in DOM/test-id/dev-panel (manual; exhaustive in GAT101PLATRI-018).

### Invariants

1. TypeScript decides no legality, scoring, trick winner, or redaction (FOUNDATIONS §2).
2. Animation is driven by Rust semantic effects; the renderer settles to the latest viewer-safe public view (FOUNDATIONS §7).

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/PlainTricksBoard.tsx` exercised by `smoke:ui`.
2. `ActionControls.tsx` no-leak test-id path for `plain_tricks`.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. `smoke:ui` is the correct boundary for renderer correctness; full e2e + no-leak DOM/storage checks are GAT101PLATRI-018.
