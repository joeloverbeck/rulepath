# RIVLEDSHOWUX-011: Recompose the table into a central board well + compact rails

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/RiverLedgerBoard.tsx`, `apps/web/src/styles.css`
**Deps**: RIVLEDSHOWUX-003, RIVLEDSHOWUX-004, RIVLEDSHOWUX-005, RIVLEDSHOWUX-009

## Problem

The current layout underuses width — a vertical seat rail plus a blank upper-left region make the table read like a diagnostic panel with cards attached. Recompose `RiverLedgerBoard` into a centered play surface: a central board well, compact multi-seat rails (responsive two-rail / shallow horseshoe for 3–6 seats), and an action/status band — using only Rust/static view data.

## Assumption Reassessment (2026-06-16)

1. Verified: `RiverLedgerBoard.tsx` renders the seat list, board, and action controls; seat order, active seat, roles, contributions, live/folded status, and labels all come from the Rust public view (`RL-UI-SEATS-001`), not TS computation.
2. Verified against spec §6 D8 + §8 WB11 (#10); this ticket composes the board-slot view (RIVLEDSHOWUX-004), seat-ledger fields (005), action rows (003), and the V2 outcome surface (009) — hence the four `Deps`.
3. Shared boundary under audit: `RiverLedgerBoard.tsx` is River-Ledger-local; the recomposition consumes the prior tickets' Rust fields and must not introduce any browser-side legality, evaluation, or seat-ordering logic.
4. FOUNDATIONS §7 (cozy, centered, original board-game surface — not a diagnostic panel, no casino trade dress) + §2 (layout/typography only; seat order/active/roles stay Rust-owned) motivate this ticket.

## Architecture Check

1. A central-board-well + compact-rails layout driven entirely by Rust view data is the minimal way to reclaim width without moving any authority into TS; responsive geometry is pure CSS/layout.
2. No shims; the vertical-rail layout is replaced, not aliased.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); no casino trade dress (§10).

## Verification Layers

1. The table renders as central board well + compact rails, responsive across 3–6 seats -> `npm --prefix apps/web run smoke:ui` (multi-seat-count render).
2. Seat order/active/roles/status come from Rust view data, not TS -> grep `RiverLedgerBoard.tsx` for seat-ordering/active-seat logic (none beyond rendering Rust fields).
3. No hidden-info leak through the recomposed DOM -> `node apps/web/e2e/a11y-noleak.smoke.mjs`.

## What to Change

### 1. `apps/web/src/components/RiverLedgerBoard.tsx`

Recompose into: a seat-rail/viewer-controls strip, left rail / central board well (street banner + board slots + ledger total) / right rail, and an active-seat/action/status band. Place 3–6 seats responsively (two-rail or shallow horseshoe). Render only Rust/static fields.

### 2. `apps/web/src/styles.css`

Add the layout structure (grid/flex) for the board well and rails (River-scoped classes; the token palette is RIVLEDSHOWUX-012).

## Files to Touch

- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)
- `apps/web/src/styles.css` (modify)

## Out of Scope

- River-scoped color tokens / theme (RIVLEDSHOWUX-012).
- Scheduler reveal/showdown pacing (RIVLEDSHOWUX-013).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` + `npm --prefix apps/web run build` — central-board-well layout renders for 3, 4, 5, 6 seats; type-checks.
2. `npm --prefix apps/web run smoke:e2e` — River Ledger play flow unaffected.
3. `node apps/web/e2e/a11y-noleak.smoke.mjs` — no hidden-info leak in the recomposed DOM.

### Invariants

1. Seat order, active seat, roles, contributions, and status come from Rust view data; TS computes none (§2, `RL-UI-SEATS-001`).
2. No casino trade dress; the surface stays original and cozy (§7, §10).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` (modify, as surfaced) — multi-seat-count layout render assertion.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:e2e`
3. `npm --prefix apps/web run build`

## Outcome

Completed on 2026-06-16.

- Replaced the River Ledger diagnostic-style table stack with a Rust-view-driven table shell containing a central board well, compact seat rail, and action/status band.
- Kept seat ordering, active seat, roles, contributions, action rows, and terminal copy sourced from Rust view fields; TypeScript only recomposes presentation markup.
- Added River Ledger multi-seat smoke coverage for 3, 4, 5, and 6 seat views to assert compact rails and central board slots.
- Verified with `npm --prefix apps/web run build`, `npm --prefix apps/web run smoke:ui`, `node apps/web/e2e/a11y-noleak.smoke.mjs`, and `npm --prefix apps/web run smoke:e2e`.
