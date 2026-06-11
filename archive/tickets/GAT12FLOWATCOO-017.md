# GAT12FLOWATCOO-017: React board and presentation-shell wiring

**Status**: ACCEPTED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/FloodWatchBoard.tsx` (new); `apps/web/src/components/{effectFeedback.ts,outcomeExplanationTemplates.ts}`, `apps/web/src/wasm/client.ts`, `apps/web/src/state/shellReducer.ts` (modify); `apps/web/public/rules/flood_watch.md` (new, generated); `scripts/check-player-rules.mjs`, `apps/web/scripts/{smoke-ui.mjs,smoke-effect-feedback.mjs}` (modify). No Rust behavior — legality, automation, outcome, and redaction stay in Rust.
**Deps**: GAT12FLOWATCOO-014, GAT12FLOWATCOO-016, GAT12FLOWATCOO-001

## Problem

`flood_watch` needs its browser board and shell wiring: the five district gauges with levees, the deck count + remaining-composition panel, the forecast display, the budget indicator, role cards, the environment-phase animation driven by Rust semantic effects (reduced-motion aware), the shared-outcome explanation, the effect-log feedback, the viewer-safe rationale type mirror, the generated player-rules copy, and the per-game smoke-harness assertions. TypeScript stays presentation-only — it invents no legality and runs no automation.

## Assumption Reassessment (2026-06-11)

1. `apps/web/src/components/` holds the `*Board.tsx` components (verified `MaskedClaimsBoard.tsx`; PascalCase+`Board` convention) plus `ActionControls.tsx`, `EffectLog.tsx`, `effectFeedback.ts`, `outcomeExplanationTemplates.ts`; `apps/web/src/wasm/client.ts` and `apps/web/src/state/shellReducer.ts` carry the per-game shell types. `apps/web/scripts/smoke-ui.mjs` and `smoke-effect-feedback.mjs` carry hardcoded per-game assertions (verified `["masked_claims", …]`) → both are `(modify)` targets. `scripts/copy-player-rules.mjs` generates `apps/web/public/rules/<game>.md` from `HOW-TO-PLAY.md`; `scripts/check-player-rules.mjs` holds `HIDDEN_INFO_GAMES = new Set([... "masked_claims"])` (verified).
2. The spec (§Deliverables "Browser", §Implementation reference "WASM/browser wiring", Work-breakdown item 13; reassessment finding M1) fixes: `FloodWatchBoard.tsx`; `ActionControls` budgeted-phase support (remaining-budget display, teammate waiting state) without TS legality; `EffectLog`/`effectFeedback.ts` entries for action/draw/absorption/rise/inundation/terminal; **shared-outcome** entries in `outcomeExplanationTemplates.ts` — the first *cooperative shared-win/shared-loss* outcome, but the existing `*_draw` templates already render winner-free results with `requiredParams: []`, so the `OutcomeExplanationTemplate` type is reused unchanged (reassessment M1); `FloodWatchOutcomeRationale` mirror in `client.ts`; player-rules copy + `flood_watch` added to `HIDDEN_INFO_GAMES`; reduced-motion support.
3. Cross-artifact boundary under audit: the renderer consumes the WASM bridge (GAT12FLOWATCOO-014) — legal tree, view, effects, bot decisions, redacted export. `outcomeExplanationTemplates.ts` + `client.ts` + `UI.md` (GAT12FLOWATCOO-016) + `RULES.md` terminal IDs (GAT12FLOWATCOO-001) are jointly validated by `scripts/check-outcome-explanations.mjs`; the generated player-rules file + `HIDDEN_INFO_GAMES` are validated by `scripts/check-player-rules.mjs`. The shared-outcome template reuses the existing type (no schema change, per M1).
4. FOUNDATIONS §2 (TypeScript presentation-only; never decides legality) + §7 (legal-only UI; semantic effects drive animation; renderer settles to the latest viewer-safe view) + §11 (hidden info does not leak through DOM/storage/test IDs) motivate this ticket: the board renders only what Rust's tree contains, the teammate sees a waiting state, the environment animation plays from Rust effects (reduced-motion preserves order), and anchors use district/turn IDs — never deck data.
5. Enforcement surface: the DOM/storage/test-ID no-leak firewall (§11) — the board must expose no undrawn-deck order in markup, `data-testid`, local storage, or dev-panel output. The browser DOM/storage assertions are authored in GAT12FLOWATCOO-018; this ticket must build the surface so those assertions pass.

## Architecture Check

1. Driving the board entirely from the Rust view + effect stream (no TS legality, no client timer for the environment phase) is the FOUNDATIONS §2/§7 contract; reusing the existing `OutcomeExplanationTemplate` type for the team outcome (M1) avoids a needless schema change since winner-free templates already exist.
2. No backwards-compatibility aliasing/shims; additive component + additive per-game shell entries.
3. `engine-core`/`game-stdlib` untouched; this is `apps/web` presentation only — behavior authority stays in Rust.

## Verification Layers

1. Legal-only, presentation-only UI -> manual review + FOUNDATIONS §2/§7 alignment: controls render only from Rust's tree; teammate waiting state; no TS legality.
2. Effect-driven animation, reduced-motion -> manual review: the environment batch animates from Rust effects in order; reduced motion preserves order/copy.
3. Outcome-explanation registration -> `node scripts/check-outcome-explanations.mjs` passes with the flood-watch shared-outcome templates + rule-ID mirrors.
4. Player-rules sync -> `node scripts/check-player-rules.mjs` passes with `flood_watch` in `HIDDEN_INFO_GAMES` and the generated file in sync.
5. Smoke-harness assertions -> `npm --prefix apps/web run smoke:ui` and `smoke:effects` include `flood_watch` assertions.

## What to Change

### 1. Board + shell wiring

Author `FloodWatchBoard.tsx` (gauges, levees, deck count + remaining-composition panel, forecast, budget indicator, role cards, effect-driven environment animation, replay controls, reduced-motion). Extend `ActionControls`/`EffectLog` usage for the budgeted phase + teammate waiting state. Add `effectFeedback.ts` entries (action/draw/absorption/rise/inundation/terminal) and shared-outcome entries in `outcomeExplanationTemplates.ts` (reuse the existing type). Add the `FloodWatchOutcomeRationale` mirror + `terminal_rationale` field in `client.ts` and the shell reducer/client types in `shellReducer.ts`.

### 2. Player-rules + smoke harnesses

Generate `apps/web/public/rules/flood_watch.md` via `scripts/copy-player-rules.mjs` from `HOW-TO-PLAY.md`; add `flood_watch` to `HIDDEN_INFO_GAMES` in `scripts/check-player-rules.mjs`. Add `flood_watch` assertions to `apps/web/scripts/smoke-ui.mjs` (catalog + variant + view) and `smoke-effect-feedback.mjs` (effect feedback entries).

## Files to Touch

- `apps/web/src/components/FloodWatchBoard.tsx` (new)
- `apps/web/src/components/effectFeedback.ts` (modify)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/state/shellReducer.ts` (modify)
- `apps/web/public/rules/flood_watch.md` (new — generated from `HOW-TO-PLAY.md`)
- `scripts/check-player-rules.mjs` (modify — `HIDDEN_INFO_GAMES`)
- `apps/web/scripts/smoke-ui.mjs` (modify — `flood_watch` assertions)
- `apps/web/scripts/smoke-effect-feedback.mjs` (modify — `flood_watch` effect entries)

## Out of Scope

- The E2E smoke, a11y/no-leak DOM assertions, and README catalog reconciliation (GAT12FLOWATCOO-018).
- Any Rust/WASM behavior change — this ticket consumes the GAT12FLOWATCOO-014 bridge.
- `ActionControls.tsx`/`EffectLog.tsx` structural rewrites — extend their existing per-game usage, do not refactor the shared components.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` and `smoke:effects` pass with `flood_watch` assertions.
2. `node scripts/check-outcome-explanations.mjs` and `node scripts/check-player-rules.mjs` pass.
3. `npm --prefix apps/web run build` succeeds with the new board and shell types.

### Invariants

1. TypeScript decides no legality and runs no automation; the board renders only Rust's tree + effects and settles to the latest viewer-safe view.
2. No undrawn-deck order appears in DOM, `data-testid`, local storage, or dev-panel output.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs`, `smoke-effect-feedback.mjs` — `flood_watch` assertions (modify).
2. `apps/web/public/rules/flood_watch.md` — generated player-rules artifact (sync-checked).

### Commands

1. `npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects`
2. `node scripts/check-outcome-explanations.mjs && node scripts/check-player-rules.mjs && npm --prefix apps/web run build`
3. The E2E browser smoke + DOM/storage no-leak assertions are the GAT12FLOWATCOO-018 boundary; `smoke:ui`/`smoke:effects` + the check scripts are the correct boundary for the renderer diff.

## Outcome

Accepted on 2026-06-11.

Implemented the Flood Watch web presentation surface with `FloodWatchBoard.tsx`, Rust-view-only action rendering, district flood/levee counters, public remaining-composition/deck accounting, forecast and role displays, effect-driven status feedback, and shared win/loss outcome explanations. Wired the board into the React shell, client public-view types, terminal/text helpers, replay snapshot summaries, and generic terminal guards without adding TypeScript legality or automation.

Generated `apps/web/public/rules/flood_watch.md`, registered Flood Watch as a hidden-information player-rules game, and extended smoke coverage for catalog metadata, public view redaction, legal action tree exposure, public storm effects, bot policy output, public replay export/import, and effect-feedback rendering.

Verification passed:

- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `node scripts/check-outcome-explanations.mjs`
- `node scripts/check-player-rules.mjs`
- `npm --prefix apps/web run build`
