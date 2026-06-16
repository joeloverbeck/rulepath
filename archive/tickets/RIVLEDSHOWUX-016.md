# RIVLEDSHOWUX-016: Original River Ledger catalog SVG icon

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/GameCatalogIcon.tsx`
**Deps**: None

## Problem

`river_ledger` is absent from the `GameCatalogIcon` icon map, so the showcase game falls back to the generic icon. Add an original `RiverLedgerIcon` SVG (abstract cards forming a river bend / ledger fan, a thin brass ledger line with tally marks, monochrome-friendly shapes) with an accessible title from the catalog entry — no chips, cash, green felt, casino oval, branded backs, or poker-room imagery.

## Assumption Reassessment (2026-06-16)

1. Verified: `GameCatalogIcon.tsx:14` defines `const ICONS: Record<string, IconComponent>`; it has no `"river_ledger"` key (it falls back to a generic icon); siblings like `"poker_lite"` / `"event_frontier"` are present.
2. Verified against spec §6 D10 + §8 WB16 (#15); the icon map is separate from `scripts/check-catalog-docs.mjs` (which keys off the wasm-api catalog const and the README lists), so adding an icon does not change the catalog-docs surfaces — `check-catalog-docs` stays green.
3. Shared boundary under audit: the shared `GameCatalogIcon` `ICONS` map — the change is a single River-local entry consumed by `GamePicker` / `MatchSetup`; no picker/setup redesign.
4. FOUNDATIONS §10 / `docs/IP-POLICY.md` (original, project-owned asset; no casino trade dress; accessible label from the catalog entry, not hard-coded hidden text) motivates this ticket.

## Architecture Check

1. Adding one `"river_ledger": RiverLedgerIcon` entry to the existing `ICONS` map is the minimal-blast-radius change — no picker/setup restructuring, and every other game's icon is untouched.
2. No shims; the generic fallback is replaced for `river_ledger` by the dedicated icon.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); presentation only.

## Verification Layers

1. The catalog shows the River Ledger icon (no generic fallback) -> `npm --prefix apps/web run smoke:ui` + visual check.
2. Icon legible at display size -> render at 16 px (the smallest real catalog size) and confirm legibility (screenshot evidence).
3. Original asset, no casino iconography, accessible label present -> manual IP review + a11y label check.

## What to Change

### 1. `apps/web/src/components/GameCatalogIcon.tsx`

Add `RiverLedgerIcon` (original SVG: abstract cards / river bend / brass ledger line + tally marks; monochrome-friendly shape differences; River token colors) and register `"river_ledger": RiverLedgerIcon` in the `ICONS` map; source the accessible title from the catalog entry.

## Files to Touch

- `apps/web/src/components/GameCatalogIcon.tsx` (modify)

## Out of Scope

- `GamePicker` / `MatchSetup` redesign (the icon flows through the shared map).
- River-scoped table tokens (RIVLEDSHOWUX-012).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` + `npm --prefix apps/web run build` — `river_ledger` resolves to the dedicated icon, not the generic fallback; type-checks.
2. `node scripts/check-catalog-docs.mjs` — stays green (icon map is orthogonal to the catalog-docs surfaces).
3. Render-at-display-size review — the icon is legible at 16 px with an accessible label.

### Invariants

1. The icon asset is original and project-owned; no casino trade dress (§10, `docs/IP-POLICY.md`).
2. The accessible label comes from the catalog entry, not hard-coded hidden text.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` (modify, as surfaced) — assert `river_ledger` maps to a dedicated icon.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `node scripts/check-catalog-docs.mjs`
3. `npm --prefix apps/web run build`

## Outcome

Added an original River Ledger catalog icon to `GameCatalogIcon`: an abstract fan of card shapes with a river/ledger line and tally marks, using existing icon color variables and avoiding chips, cash, felt, casino ovals, branded card backs, or poker-room imagery.

Registered `"river_ledger"` in the icon map and kept the asset local to the shared catalog icon seam. River's picker/setup icon now exposes the caller-provided catalog title (`River Ledger icon`) when rendered, while other game icons remain decorative as before.

Added shell browser coverage that verifies River Ledger renders the dedicated icon geometry, carries the accessible title from the catalog display name, and does not use the generic fallback square.

Verification:

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. `node scripts/check-catalog-docs.mjs`
4. `node apps/web/e2e/shell.smoke.mjs`
