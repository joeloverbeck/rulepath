# GAT11MASCLABLU-017: React board and shell reducer/effects

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — new `apps/web/src/components/MaskedClaimsBoard.tsx`; modifies `apps/web/src/main.tsx`, `apps/web/src/wasm/client.ts`, `apps/web/src/components/{effectFeedback.ts,ActionControls.tsx,EffectLog.tsx,outcomeExplanationTemplates.ts}` (no Rust/behavior surface — TypeScript presents Rust-provided data only)
**Deps**: GAT11MASCLABLU-014

## Problem

The browser shell needs a Masked Claims board that renders the own hand, the face-down pedestal with its declared grade, the reaction prompt or claimant waiting copy, the veiled/exposed galleries, scores, the effect log, and replay controls — driving every legality and animation from the Rust action trees and semantic effects, never from TypeScript. No DOM, `data-testid`, or storage anchor may carry an unrevealed tile ID.

## Assumption Reassessment (2026-06-10)

1. The WASM bridge (GAT11MASCLABLU-014) supplies viewer-scoped views, per-phase action trees, semantic effects, bot turns, and replay export/import as JSON. The wiring model: `apps/web/src/components/PlainTricksBoard.tsx` is dispatched from `apps/web/src/main.tsx` (confirmed), and `apps/web/src/wasm/client.ts` carries the reducer/type coverage (confirmed, 19 `plain_tricks` references). `effectFeedback.ts`, `ActionControls.tsx`, `EffectLog.tsx`, and `outcomeExplanationTemplates.ts` are confirmed present and take per-game additions.
2. Spec Deliverables Browser row + §"WASM/browser wiring": the board renders own hand / pedestal / reaction prompt or waiting state / galleries / scores / effect log / replay; response buttons render only when Rust's tree contains them; pending anchors use seat/turn IDs (never tile IDs); veiled-gallery anchors use position indices. `effectFeedback.ts` entries for claim, window-open, accept, challenge, reveal, and resolution; outcome templates plus viewer-safe rationale mirrors in `client.ts`.
3. Cross-artifact boundary under audit: the shell reducer/client types (`client.ts`), the renderer dispatch (`main.tsx`), and the shared components (`ActionControls`/`EffectLog`/`effectFeedback`/`outcomeExplanationTemplates`) — additive per-game arms, no contract change.
4. FOUNDATIONS §2 (TypeScript presents only; legality comes from the Rust tree) and §7 (legal-only UI, semantic-effect-driven animation, reduced motion) are the principles under audit.
5. No-leak firewall enforcement surface: DOM / `data-testid` / local storage. Confirm response buttons render only when the Rust tree contains them, and pending / veiled-gallery anchors use seat/turn/position IDs — never a tile ID for an unrevealed mask.

## Architecture Check

1. Rendering exclusively from Rust action trees and semantic effects (no TS-side legality, no guessed state diffs) keeps behavior authority in Rust and animation causal — the §2/§7 contract.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; the renderer is presentation-only and adds no game noun to any shared contract.

## Verification Layers

1. Legal-only UI in both phases (responder choices vs claimant waiting) -> manual review + browser E2E smoke (GAT11MASCLABLU-019).
2. Semantic-effect-driven animation + reduced motion -> manual review + E2E smoke (GAT11MASCLABLU-019).
3. No unrevealed tile ID in DOM / `data-testid` / storage -> a11y/no-leak smoke (GAT11MASCLABLU-019) + manual audit.
4. Shell builds with the new types -> `npm --prefix apps/web run build`.

## What to Change

### 1. `apps/web/src/components/MaskedClaimsBoard.tsx`

Render own hand, pedestal (face-down + declared grade), reaction prompt or waiting copy, galleries (veiled as face-down count + declared grades; exposed face-up), scores/counters, effect log, replay controls. Response buttons appear only when Rust's tree contains them. Anchors use seat/turn/position IDs.

### 2. Shell wiring

`main.tsx` renderer dispatch; `client.ts` reducer/type coverage + viewer-safe rationale mirrors; `effectFeedback.ts` entries (claim, window-open, accept, challenge, reveal, resolution); `ActionControls.tsx` (responder choices; claimant waiting state) without TS legality; `EffectLog.tsx` entries; `outcomeExplanationTemplates.ts` masked-claims templates.

### 3. `apps/web/scripts/smoke-ui.mjs`

The `smoke:ui` harness carries **hardcoded** per-game catalog and start-match assertions (confirmed `["plain_tricks"]` start-match + catalog checks), not auto-discovery. Add the `masked_claims` catalog + start-match assertion so `smoke:ui` exercises the new board.

## Files to Touch

- `apps/web/src/components/MaskedClaimsBoard.tsx` (new)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/effectFeedback.ts` (modify)
- `apps/web/src/components/ActionControls.tsx` (modify)
- `apps/web/src/components/EffectLog.tsx` (modify)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)
- `apps/web/scripts/smoke-ui.mjs` (modify)

## Out of Scope

- Player-rules copy generation + `check-outcome-explanations` registration (GAT11MASCLABLU-018).
- Browser E2E smoke + a11y/no-leak + catalog README reconciliation (GAT11MASCLABLU-019).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` succeeds with the new board and types.
2. Response controls render only when the Rust action tree contains them; the claimant sees a waiting state.
3. No DOM node, `data-testid`, or storage value carries an unrevealed tile ID.

### Invariants

1. TypeScript decides no legality and invents no rule state (FOUNDATIONS §2).
2. Animation is driven by Rust semantic effects and settles to the latest viewer-safe view (FOUNDATIONS §7).

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/MaskedClaimsBoard.tsx` — presentation component; behavioral coverage is the E2E smoke in GAT11MASCLABLU-019.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. The build + UI smoke are the boundary here; the reaction-window click-path and no-leak DOM assertions land in GAT11MASCLABLU-019.
