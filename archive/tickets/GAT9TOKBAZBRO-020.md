# GAT9TOKBAZBRO-020: Fix cramped token_bazaar seat/supply inventory chip labels

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — presentation-only; touches `apps/web` (`TokenBazaarBoard.tsx`, `e2e/token-bazaar.smoke.mjs`). No `engine-core`/`game-stdlib`/`games/*`/schema/trace changes.
**Deps**: None

## Problem

In the `SEAT 0`, `PUBLIC SUPPLY`, and `SEAT 1` inventory cards, the resource
labels `amber`, `jade`, and `iron` render one character per line
(`a` / `m` / `b` / `e` / `r` stacked vertically), making them illegible. The
cause is layout: these inventories lay three resource chips side-by-side in a
narrow grid column, and the collapsed label cell with `overflow-wrap: anywhere`
breaks each word apart.

The same `ResourceChips` component renders perfectly in the contract-cost and
action-card contexts because those use the **compact** variant — one chip per
full-width row. The fix is to render the seat/supply inventories with that same
proven compact layout.

## Assumption Reassessment (2026-06-08)

1. `ResourceChips` (`apps/web/src/components/TokenBazaarBoard.tsx:177-197`) takes
   a `compact` prop. The two cramped call sites pass it as default-`false`:
   `SeatInventory` (`TokenBazaarBoard.tsx:168`) and the supply section
   (`TokenBazaarBoard.tsx:77`). The working call sites pass `compact`: contract
   cost (`TokenBazaarBoard.tsx:95`) and action metadata
   (`TokenBazaarBoard.tsx:205-206`).
2. CSS: non-compact `.resource-chips` is `grid-template-columns: repeat(3,
   minmax(0, 1fr))` (`apps/web/src/styles.css:453-457`) — three chips across;
   compact `.resource-chips.compact` is `grid-template-columns: 1fr`
   (`styles.css:459-462`) — stacked rows. The per-chip label cell collapses
   under `overflow-wrap: anywhere` (`styles.css:486-490`). Switching the two
   inventory call sites to `compact` reuses the existing working rule with no
   new CSS.
3. Cross-artifact boundary under audit: none beyond the `apps/web` presentation
   layer. This is a pure CSS/layout change with no data, schema, or Rust
   contract involved.
4. `docs/UI-INTERACTION.md:95` treats layout/presentation as a UI-owned concern;
   no rule legality, hidden state, or behavior is affected, so no FOUNDATIONS
   product-behavior principle is engaged.
5. No hidden-information surface: the inventory chips already display the same
   public resource counts; only their visual arrangement changes. The existing
   `assertNoLeak` passes on the same data.

## Architecture Check

1. Reusing the existing `compact` variant is cleaner than a bespoke fix: it
   makes seat/supply inventories visually consistent with the contract-cost and
   action-card chips that already render correctly, and adds zero new CSS.
   Rejected alternatives — truncating labels with ellipsis (loses legibility:
   `amb…`) or a two-line code+caption redesign (more CSS, no consistency gain).
2. No backwards-compatibility shim: the call sites simply pass an existing prop.
3. No `engine-core`/`game-stdlib`/`games/*` change — presentation-only.
   `engine-core` stays noun-free.

## Verification Layers

1. Seat/supply inventory chips render on a single column (not three cramped
   across), so labels no longer break per-character -> e2e smoke assertion in
   `apps/web/e2e/token-bazaar.smoke.mjs` reading the computed
   `grid-template-columns` of a `.token-seat .resource-chips` container and
   asserting a single track.
2. Chips still expose code, name, and numeric count after the layout switch ->
   existing `assertBoardA11y` resource-chip assertion
   (`e2e/token-bazaar.smoke.mjs:148-151`) continues to pass.

## What to Change

### 1. Render seat/supply inventories with the compact chip layout

In `apps/web/src/components/TokenBazaarBoard.tsx`, pass `compact` to the two
inventory `ResourceChips` call sites:

- `SeatInventory` (`TokenBazaarBoard.tsx:168`):
  `<ResourceChips counts={inventory.resources} compact />`
- supply section (`TokenBazaarBoard.tsx:77`):
  `<ResourceChips counts={view.supply} compact />`

No CSS change is required — `.resource-chips.compact` (`styles.css:459-462`)
already provides the stacked single-column layout.

### 2. Add a layout-regression assertion to the e2e smoke

In `apps/web/e2e/token-bazaar.smoke.mjs`, after `assertBoardA11y`, read the
computed `grid-template-columns` of a `.token-seat .resource-chips` container
and assert it resolves to a single track (the compact layout), guarding against
a regression back to the three-across cramming.

## Files to Touch

- `apps/web/src/components/TokenBazaarBoard.tsx` (modify)
- `apps/web/e2e/token-bazaar.smoke.mjs` (modify)

## Out of Scope

- Issue 2 (action cards showing zero deltas) — tracked in GAT9TOKBAZBRO-019.
- Removing the now-unused non-compact `.resource-chips` 3-column CSS rule
  (`styles.css:453-457`): harmless dead rule; defer to a separate cleanup to
  keep this diff minimal and reviewable.
- Any Rust, schema, trace, or data change.

## Acceptance Criteria

### Tests That Must Pass

1. New smoke assertion: a `.token-seat .resource-chips` container computes to a
   single-column grid track.
2. Existing `assertBoardA11y` resource-chip assertion still passes (chips keep
   code/name/count).
3. `npm --prefix apps/web run smoke:e2e` passes.
4. `npm --prefix apps/web run build` succeeds.

### Invariants

1. Resource chips remain accessible: every chip exposes a non-empty code, name,
   and numeric count (`e2e/token-bazaar.smoke.mjs:148-151`).
2. Inventory and action/contract chips share one chip component and the compact
   layout, keeping presentation consistent.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/token-bazaar.smoke.mjs` — add a single-column layout assertion
   for seat inventory chips.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run build`
3. The web app has no unit-test framework; the Puppeteer e2e smoke is the
   project's sanctioned UI verification boundary, so the layout assertion
   belongs there rather than in a new harness.

## Outcome

Completed: 2026-06-08

What changed:

- `apps/web/src/components/TokenBazaarBoard.tsx` now renders public supply and
  seat inventory resource chips with the existing compact layout.
- `apps/web/e2e/token-bazaar.smoke.mjs` now asserts seat inventory resource
  chips compute to a single grid track.

Deviations from original plan:

- None.

Verification results:

- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:e2e` passed.
