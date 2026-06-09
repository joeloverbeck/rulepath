# VICEXPSHASUR-010: Web type wiring + board integration for the shared outcome surface

**Status**: DONE
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/wasm/client.ts` (per-game rationale types) and the nine `apps/web/src/components/*Board.tsx` (route terminal display to the shared panel); no Rust/engine, WASM, or behavior surface (TypeScript mirrors and renders Rust-supplied data only).
**Deps**: VICEXPSHASUR-009, VICEXPSHASUR-003, VICEXPSHASUR-004, VICEXPSHASUR-005, VICEXPSHASUR-006, VICEXPSHASUR-007, VICEXPSHASUR-008

## Problem

The Rust rationales (003–008) and the shared panel (009) exist but are not connected: `client.ts` has no rationale types and the boards still own ad-hoc terminal status. Mirror each game's viewer-safe rationale payload into `client.ts` and route every board's terminal display to `OutcomeExplanationPanel`, demoting divergent ad-hoc terminal panels to board-native visualization. Source: `specs/victory-explanation-shared-surface.md` §10.1, §10.3, §15.6.

## Assumption Reassessment (2026-06-09)

1. Verified current code: `apps/web/src/wasm/client.ts` carries per-game view types (the `PublicView` union and per-game `*TerminalView` / `*PublicView` types, e.g. `PokerLiteTerminalView`, `TokenBazaarTerminalView`, `SecretDraftTerminalView`); all nine board components exist with the exact names `RaceBoard.tsx`, `ThreeMarksBoard.tsx`, `ColumnFourBoard.tsx`, `DirectionalFlipBoard.tsx`, `DraughtsLiteBoard.tsx`, `HighCardDuelBoard.tsx`, `TokenBazaarBoard.tsx`, `SecretDraftBoard.tsx`, `PokerLiteBoard.tsx`. The new rationale fields projected in 003–008 must be mirrored into the matching `client.ts` types and each board routed to `OutcomeExplanationPanel` (009).
2. Spec §10.1 (keep types game-specific; no fake universal outcome model; no `determineWinner`/`compareCards`/`findWinningLine`/`resolveTiebreak`/`scoreOutcome`) and §10.3 (route terminal display to the shared panel; local in-play pills may remain; remove or demote divergent ad-hoc terminal panels).
3. Cross-artifact boundary under audit: `client.ts` types are the TypeScript mirror of the Rust view-schema rationales extended in 003–008 (producer); the boards consume `OutcomeExplanationPanel` (009, producer). This ticket is the producer/consumer integration — hence `Deps` on 009 and on every game-rationale ticket (003–008).
4. FOUNDATIONS §2 restated: TypeScript types and renders only — it MUST NOT compute a winner/score/line/tiebreaker/decisive cause, MUST NOT collapse game payloads into a fake universal model that hides game facts, and may use a shared presentation adapter only after Rust has supplied the decisive cause and ordered breakdowns.
5. §11 no-leak firewall: `client.ts` types mirror only the viewer-safe Rust fields; boards must create no hidden DOM text / `aria-label` / `data-testid` / CSS class carrying private values. For the hidden-info games (`high_card_duel`, `secret_draft`, `poker_lite`) the payloads are already viewer-filtered by Rust — the wiring must not widen them.
6. Schema mirror: `client.ts` extends each game's TS view type to match the Rust rationale (additive per-game fields); consumers are the boards + `OutcomeExplanationPanel`. Any ad-hoc terminal sub-component a board demotes is a within-board change — grep that board for its references when removing it (blast radius stays inside the board file).

## Architecture Check

1. Keeping `client.ts` types game-specific (mirroring the Rust payloads) and routing through one shared panel preserves both the per-game decisive-cause fidelity and the single-surface UI (spec §10.1/§10.3); a fake universal outcome model is explicitly rejected because it would hide game facts.
2. No backwards-compatibility shims: ad-hoc terminal panels are removed or demoted to board-native visualization, not aliased behind the shared panel.
3. `engine-core`/`game-stdlib` untouched; no legality/outcome decided in TypeScript.

## Verification Layers

1. Types mirror payloads → `npm --prefix apps/web run build` type-checks the nine rationale payloads against `client.ts`.
2. Boards route to the shared panel → grep-proof each of the nine `*Board.tsx` imports/renders `OutcomeExplanationPanel`; no board retains a divergent authoritative terminal-explanation panel.
3. No TS outcome logic → grep-proof none of `determineWinner`/`compareCards`/`findWinningLine`/`resolveTiebreak`/`scoreOutcome` (and no fake universal outcome model) appears in `client.ts` or the boards; FOUNDATIONS §2 manual review.
4. No-leak → manual review: boards pass only Rust-supplied viewer-safe fields; the full no-leak proof across DOM/labels/test-IDs/storage is the 011 smoke.
5. Builds + existing UI smoke green → `npm --prefix apps/web run build` and `npm --prefix apps/web run smoke:ui` (regression: no existing board view broken).

## What to Change

### 1. `apps/web/src/wasm/client.ts` rationale types

Extend each game's view type with the viewer-safe rationale fields projected in 003–008. Keep types game-specific; use a shared presentation adapter shape only after the Rust decisive cause + ordered breakdowns have crossed the boundary. Do not add `determineWinner`/`compareCards`/`findWinningLine`/`resolveTiebreak`/`scoreOutcome` or any comparison helper.

### 2. Board integration (nine `*Board.tsx`)

Wire each board's terminal state to render `OutcomeExplanationPanel` with the game's rationale payload (via the no-logic adapter). Local in-play status pills may remain; any bespoke terminal panel is removed or reduced to board-native visualization (winning-line highlight, revealed cards, final score) while the shared panel carries the text explanation.

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/RaceBoard.tsx` (modify)
- `apps/web/src/components/ThreeMarksBoard.tsx` (modify)
- `apps/web/src/components/ColumnFourBoard.tsx` (modify)
- `apps/web/src/components/DirectionalFlipBoard.tsx` (modify)
- `apps/web/src/components/DraughtsLiteBoard.tsx` (modify)
- `apps/web/src/components/HighCardDuelBoard.tsx` (modify)
- `apps/web/src/components/TokenBazaarBoard.tsx` (modify)
- `apps/web/src/components/SecretDraftBoard.tsx` (modify)
- `apps/web/src/components/PokerLiteBoard.tsx` (modify)

## Out of Scope

- The `OutcomeExplanationPanel` / templates implementation (009) — this ticket consumes them.
- Any Rust rationale change (003–008) — this ticket mirrors them.
- The browser smoke + smoke registration (011).
- Any legality/scoring/outcome decision in TypeScript; any `engine-core`/`game-stdlib` change.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` passes with all nine rationale payloads typed in `client.ts`.
2. Each of the nine `*Board.tsx` routes terminal explanation to `OutcomeExplanationPanel`; no board keeps a divergent authoritative terminal-explanation panel.
3. `npm --prefix apps/web run smoke:ui` passes (no existing board view regressed).

### Invariants

1. TypeScript only types and renders the supplied payloads — no winner/score/line/tiebreak computation and no fake universal outcome model that hides game facts (FOUNDATIONS §2).
2. Only Rust-supplied viewer-safe fields are wired; no hidden DOM/`data-testid` carrying private values is introduced (FOUNDATIONS §11; full proof in 011).

## Test Plan

### New/Modified Tests

1. `apps/web/src/wasm/client.ts` + the nine `*Board.tsx` — type + wiring changes; behavioral verification is the 011 browser smoke.
2. `None new test file here — the build + `smoke:ui` regression are the verification boundary; the terminal e2e/no-leak/a11y assertions are added in 011.`

### Commands

1. `npm --prefix apps/web run build`
2. `grep -nE 'determineWinner|compareCards|findWinningLine|resolveTiebreak|scoreOutcome' apps/web/src/wasm/client.ts apps/web/src/components/*Board.tsx` (expect no matches)
3. `npm --prefix apps/web run smoke:ui`

## Outcome

Added catalog-wide TypeScript rationale mirrors in
`apps/web/src/wasm/client.ts`, including game-specific `*OutcomeRationale`
type names and `terminal_rationale` fields on every public view type. Added the
catalog-derived `RaceToNPublicView` alias while preserving the existing
`RacePublicView` import surface.

Routed all nine board components to `OutcomeExplanationPanel` on terminal
states:

1. `RaceBoard.tsx`
2. `ThreeMarksBoard.tsx`
3. `ColumnFourBoard.tsx`
4. `DirectionalFlipBoard.tsx`
5. `DraughtsLiteBoard.tsx`
6. `HighCardDuelBoard.tsx`
7. `TokenBazaarBoard.tsx`
8. `SecretDraftBoard.tsx`
9. `PokerLiteBoard.tsx`

Boards pass Rust-supplied `terminal_rationale` through when present and provide
only Rust-projected public fallback fields for standing/breakdown presentation.
Board-native visualization remains in place for grids, pieces, cards, market
state, and reveal/showdown evidence; the shared panel now owns terminal
explanation text.

Verification run:

1. `npm --prefix apps/web run build` — passed.
2. `npm --prefix apps/web run smoke:ui` — passed.
3. `node scripts/check-outcome-explanations.mjs` — passed for all 9 catalog games.
4. `node scripts/check-doc-links.mjs` — passed.
5. `cargo fmt --all --check` — passed.
6. `git diff --check` — passed.
7. `grep -nE 'determineWinner|compareCards|findWinningLine|resolveTiebreak|scoreOutcome' apps/web/src/wasm/client.ts apps/web/src/components/*Board.tsx` — no matches.
8. `rg -n "OutcomeExplanationPanel" apps/web/src/components/*Board.tsx` — found imports/renders in all 9 board files.
