# GAT151RIVLED-014: Web renderer and e2e smoke path

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web` components/state/styles + `e2e/river-ledger.smoke.mjs`
**Deps**: GAT151RIVLED-013

## Problem

The web shell must render the new stack setup controls, per-seat remaining stacks, all-in indicators, ordered current pot tiers, eligibility, uncalled returns, and per-pot terminal allocations — displaying Rust-authored text and projections only, computing no legality or allocation. The e2e smoke exercises an asymmetric all-in hand through terminal allocation.

## Assumption Reassessment (2026-06-20)

1. Code: `apps/web/src/components/MatchSetup.tsx`, `RiverLedgerBoard.tsx`, `OutcomeExplanationPanel.tsx`, `EffectLog.tsx`, `apps/web/src/state/shellReducer.ts`, and `apps/web/src/styles.css` already render the base game; `apps/web/e2e/river-ledger.smoke.mjs` is the existing smoke. They consume the WASM JSON extended in GAT151RIVLED-013.
2. Docs: spec §4 (web seam) + §7.3 — render stack setup, remaining stacks, all-in indicators, ordered pots, eligibility, returns, and per-pot outcome details; do not compute legality or allocation; the e2e smoke exercises an asymmetric all-in hand and asserts accessible labels, public pot accounting, and no hidden-card leakage.
3. Cross-artifact boundary under audit: the WASM JSON view fields (GAT151RIVLED-013) ↔ the React render; the renderer settles to the latest viewer-safe public view after animation.
4. (§2/§7 behavior authority) Restate: TypeScript renders Rust projections only — no cap, all-in, eligibility, winner, or remainder is computed in the client. Confirm every displayed value traces to a WASM-projected field.
5. (effect-schema consumer) `EffectLog.tsx` consumes the new effect kinds (`StackChanged`/`SeatBecameAllIn`/…) additively; animation is driven by the Rust semantic effects, with renderer diffs as diagnostics only (§7/§11).

## Architecture Check

1. Rendering Rust-authored labels/projections keeps the client presentation-only and avoids a second allocation path in TypeScript.
2. No backwards-compatibility shims; components extend their existing render of the base game.
3. No legality/mechanic logic enters TypeScript; `engine-core`/`games` boundaries are unaffected (§2/§7).

## Verification Layers

1. Stack/all-in/pot-tier/eligibility/return/allocation render from projected fields -> component render checks + e2e assertions.
2. Asymmetric all-in hand reaches terminal allocation -> `river-ledger.smoke.mjs` end-to-end run.
3. Accessible labels + live regions present -> a11y assertions in the smoke.
4. No hidden-card leakage / no duplicate outcome computation in DOM -> e2e DOM/no-leak assertions.

## What to Change

### 1. Renderers + setup controls

Extend `MatchSetup.tsx` with neutral stack setup controls (equal/preset/direct numeric, validated Rust-side) and `RiverLedgerBoard.tsx`/`OutcomeExplanationPanel.tsx`/`EffectLog.tsx` with remaining stacks, all-in indicators, ordered pot tiers, eligibility, returns, and per-pot outcomes; update `shellReducer.ts` and `styles.css` accordingly.

### 2. e2e smoke

Extend `e2e/river-ledger.smoke.mjs` to drive an asymmetric all-in hand through terminal allocation with accessible-label, public-pot-accounting, and no-leak assertions.

## Files to Touch

- `apps/web/src/components/MatchSetup.tsx` (modify)
- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)
- `apps/web/src/components/OutcomeExplanationPanel.tsx` (modify)
- `apps/web/src/components/EffectLog.tsx` (modify)
- `apps/web/src/state/shellReducer.ts` (modify)
- `apps/web/src/styles.css` (modify)
- `apps/web/e2e/river-ledger.smoke.mjs` (modify)

## Out of Scope

- The a11y/no-leak smoke harness `a11y-noleak.smoke.mjs` (GAT151RIVLED-016).
- Catalog README reconciliation (GAT151RIVLED-019/-020).
- Any Rust behavior change.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — the shell builds with the new renderers.
2. `npm --prefix apps/web run smoke:ui` and `npm --prefix apps/web run smoke:effects` — UI and effect-feedback smokes pass.
3. `npm --prefix apps/web run smoke:e2e` — the asymmetric all-in hand runs to terminal allocation with a11y/no-leak assertions.

### Invariants

1. Every displayed stack/all-in/pot/eligibility/return/allocation value traces to a Rust-projected WASM field.
2. No hidden-card content and no duplicate/contradictory outcome computation appears in the DOM.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/river-ledger.smoke.mjs` — asymmetric all-in hand through terminal allocation with accessibility and no-leak assertions.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:e2e`
3. `npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects` — UI/effect smokes are the correct presentation-layer boundary; Rust behavior is covered by earlier tickets.
