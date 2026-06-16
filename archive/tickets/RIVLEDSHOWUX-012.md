# RIVLEDSHOWUX-012: River-scoped `--rl-*` design tokens and classes

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/styles.css`
**Deps**: RIVLEDSHOWUX-011

## Problem

River Ledger has no scoped token namespace, so the recomposed table (RIVLEDSHOWUX-011) cannot carry a distinct tabletop identity without risking the global palette. Add River-scoped `--rl-*` tokens/classes (table surface, ledger marks, card state, showdown hierarchy) tuned to a warm, original tabletop look — no casino green — without overwriting any global token.

## Assumption Reassessment (2026-06-16)

1. Verified: `apps/web/src/styles.css` is the global design-token substrate shared by all games; it carries no `--rl-*` River-scoped namespace today (highest visual-regression risk lives here).
2. Verified against spec §6 D8 + §8 WB12 (#11); `RULES.md` `RL-UI-NOCASINO-001`; `docs/IP-POLICY.md` (original, no trade dress).
3. Shared boundary under audit: `styles.css` is a shared substrate — the change adds a River-scoped token/class namespace only and must not edit or override any global token consumed by other games.
4. FOUNDATIONS §10 / `docs/IP-POLICY.md` (original tabletop identity, no casino trade dress) + §7 (WCAG-AA contrast) motivate this ticket.

## Architecture Check

1. A River-scoped `--rl-*` namespace + scoped classes is the minimal-blast-radius way to add identity — global tokens are untouched, so no other game regresses.
2. No shims; tokens are additive, not aliases of global tokens.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); presentation only.

## Verification Layers

1. River table carries the `--rl-*` tabletop identity; no global token changed -> `npm --prefix apps/web run smoke:ui` + visual snapshot; grep `styles.css` confirms only additive `--rl-*` / `.river-ledger-*` rules.
2. WCAG-AA contrast on token text/boundaries -> manual contrast check on the new tokens.
3. No casino trade dress / green-felt mimicry -> `node scripts/check-presentation-copy.mjs` + manual IP review.

## What to Change

### 1. `apps/web/src/styles.css`

Add a River-scoped `--rl-*` token family (table surface/rim/glow/board well; ledger ink/muted/accent/winner; card face/red-suit/black-suit/hidden/used-ring; showdown banner/challenger/detail) and the `.river-ledger-*` classes consuming them, tuned away from casino green. Do not edit any global token.

## Files to Touch

- `apps/web/src/styles.css` (modify)

## Out of Scope

- The table layout structure (RIVLEDSHOWUX-011).
- Scheduler/motion (RIVLEDSHOWUX-013).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` + `npm --prefix apps/web run build` — the River table renders with `--rl-*` tokens; no other game's appearance changes; type-checks.
2. `node scripts/check-presentation-copy.mjs` — no casino/chip/money framing introduced.
3. Manual contrast review — token text ≥ 4.5:1, non-text boundaries ≥ 3:1 (WCAG AA).

### Invariants

1. No global design token is edited or overridden; only additive `--rl-*` tokens/classes (§7 no-regression).
2. Original tabletop identity, no casino trade dress (§10, `RL-UI-NOCASINO-001`).

## Test Plan

### New/Modified Tests

1. `None — presentation-token-only ticket; no new test file. Verification is command-based: `smoke:ui` render + build, manual WCAG-AA contrast review, and the no-casino copy audit (named in Assumption Reassessment / Verification Layers).`

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run build`
3. `node scripts/check-presentation-copy.mjs`

## Outcome

Completed on 2026-06-16.

- Added a River-scoped `--rl-*` token family on `.river-ledger-board` for table surface, rim, panels, board well, text, accents, card states, suits, usage rings, and showdown surfaces.
- Updated River Ledger table and River-specific showdown styles to consume the scoped tokens without editing global `:root` tokens or other game selectors.
- Tuned the surface away from casino-green/felt framing and away from a beige-dominant palette after visual review; the final pass uses cooler table/rim surfaces with warm card wells and copper/river accents.
- Manual contrast review: primary/muted/accent/river/red text token pairs are at or above 4.5:1; table rim and card border boundary pairs are above 3:1.
- Verified with `node scripts/check-presentation-copy.mjs`, `npm --prefix apps/web run build`, `npm --prefix apps/web run smoke:ui`, and a Playwright preview visual review of a 6-seat River Ledger table. No screenshot artifact was committed.
