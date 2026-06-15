# RIVLEDSHO-004: River-Ledger showdown rendering path in the shared outcome panel

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/OutcomeExplanationPanel.tsx`, `apps/web/src/components/outcomeExplanationTemplates.ts`
**Deps**: RIVLEDSHO-003

## Problem

The post-showdown surface renders generic engine jargon ("One showdown hand has the strongest Rust-evaluated five-card result.") and raw category/vector text — the decisive "who won and why" is never shown. This ticket renders the Rust-authored explanation fields (RIVLEDSHO-003) as a River-Ledger-specific showdown layout: decisive sentence first, winner vs. closest-challenger contrast, raw vectors/rule IDs behind a "Details" disclosure (spec WB4 / D3).

## Assumption Reassessment (2026-06-15)

1. Verified against current code: `OutcomeExplanationPanel.tsx` is a **shared catalog surface** mounted by 15 boards; `RiverLedgerBoard.tsx:12,165` consumes it via `outcomeSurfaceData()`/`outcomeAnnouncementText()`. `outcomeExplanationTemplates.ts` holds the static template copy, including the engine-jargon string.
2. Verified against specs/docs: spec §6 D3 + §8 WB4; `games/river_ledger/docs/UI.md` terminal template keys (`river_ledger.showdown_best_hand_win` / `_split_pot` / `last_live_fold_win`); `RULES.md` `RL-UI-SHOWDOWN-001`.
3. Cross-artifact boundary under audit: the shared panel is governed by `archive/specs/victory-explanation-shared-surface.md`. The rich showdown layout is a **River-Ledger-specific rendering path keyed to the new explanation fields' presence**, so the other 14 games' outcome rendering is unchanged; a cross-game non-regression check guards it.
4. FOUNDATIONS §7 (play-first, not debug-dominated) motivates this ticket: engine jargon (`Rust-evaluated`) is removed and TypeScript renders only the Rust-authored fields — it computes no category label, hand name, winner, or comparison (§2).

## Architecture Check

1. A field-presence-keyed River-Ledger rendering path inside the shared panel (rather than restructuring the shared panel for all games) is the minimum-blast-radius design — the 14 other boards keep their current rendering, and the new layout activates only when the explanation fields are present.
2. No backwards-compatibility aliasing/shims; the jargon template copy is replaced, not aliased.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4) — TS presentation only; no promotion of the panel logic.

## Verification Layers

1. The panel shows the decisive sentence + named hands + best-five + challenger contrast, vectors behind Details -> `npm --prefix apps/web run smoke:ui`.
2. No regression to the other 14 games' outcome rendering -> `node apps/web/e2e/outcome-explanation.smoke.mjs` + `npm --prefix apps/web run smoke:e2e`.
3. No TS-computed category/hand-name/winner/comparison -> manual review + grep of the panel for evaluator/winner logic (none).

## What to Change

### 1. `apps/web/src/components/OutcomeExplanationPanel.tsx`

Add a River-Ledger showdown rendering path, activated when the new explanation fields are present: headline → decisive comparison + basis → winner (best-five, rank explanation, allocation/contribution, screen-reader summary) → closest challenger (loses-because) → other revealed hands → hand-ranking reference slot (RIVLEDSHO-007) → "Details" disclosure (raw vectors + rule IDs).

### 2. `apps/web/src/components/outcomeExplanationTemplates.ts`

Replace the `river_ledger` jargon copy ("strongest Rust-evaluated five-card result") with the Rust-authored field-driven copy; leave other games' templates untouched.

## Files to Touch

- `apps/web/src/components/OutcomeExplanationPanel.tsx` (modify)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)

## Out of Scope

- The neutral card component for best-five visuals (RIVLEDSHO-006).
- The hand-ranking reference content (RIVLEDSHO-007).
- e2e worked-example assertion + browser no-leak (RIVLEDSHO-005).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` — River-Ledger showdown panel renders the decisive sentence, named hands, and challenger contrast.
2. `node apps/web/e2e/outcome-explanation.smoke.mjs` + `npm --prefix apps/web run smoke:e2e` — the other 14 games' outcome rendering is unchanged.
3. `npm --prefix apps/web run build` — type-checks against the RIVLEDSHO-003 view types.

### Invariants

1. TypeScript renders only Rust-authored fields; it computes no category label, hand name, winner, or comparison (§2).
2. The rich showdown layout activates only for River Ledger (field-presence keyed); no other game regresses (§7).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` (modify, as surfaced) — assert the River-Ledger decisive sentence and named-hand copy.
2. `apps/web/e2e/outcome-explanation.smoke.mjs` (modify, as surfaced) — cross-game non-regression assertion.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:e2e`
3. The shared-panel non-regression smoke is the correct boundary for the 15-board surface; the worked-example DOM/no-leak assertion is RIVLEDSHO-005.
