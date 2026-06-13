# CATSETVIS-001: CSS design-token layer over vanilla CSS

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/styles.css`
**Deps**: None

## Problem

The public catalog and match-setup surfaces are text-first with no coherent theming vocabulary, undercutting the "polished public playable site" priority (FOUNDATIONS §1/§7). Before any card or setup redesign, the shell needs a CSS custom-property design-token layer — primitive tokens (raw color/spacing/radius/elevation/typography) and semantic tokens (UI meaning, per-game-overridable) — so every later ticket themes against one shared vocabulary rather than ad-hoc values. This is spec WB1 / §6 D1.

## Assumption Reassessment (2026-06-13)

1. `apps/web/src/styles.css` is a single vanilla stylesheet (~3,607 lines) with no CSS framework, and `apps/web/package.json` declares no Tailwind/Chakra/MUI/Bootstrap dependency — verified this session. The token layer is added above existing selectors and adopted incrementally.
2. Spec `specs/catalog-setup-visual-redesign.md` §6 D1 (token layer is the required default) and D2 (no UI framework) govern this ticket; the starter palette in the design direction is a starting point, not law — every fg/bg and focus/non-text pair is WCAG-checked before ship.
3. Cross-artifact shared boundary: `apps/web/src/styles.css` is also modified by CATSETVIS-003 (per-game `data-game-id` accents), -004 (`.game-card` anatomy), and -007 (`.match-setup`). This ticket owns **only** the token definitions, the base catalog/setup selectors migrated to tokens, and the reduced-motion block; the per-game and component sections layer on top and are out of scope here.
4. FOUNDATIONS §7 (cozy-premium public UI; React + SVG is the v1 default) is the motivating principle: tokens are presentation-only CSS custom properties — no behavior, no legality, no framework, no renderer change.

## Architecture Check

1. A CSS custom-property token layer over the existing vanilla stylesheet is lower-risk than adopting a UI framework or CSS Modules now (spec D1/D2): it cascades, scopes per-game theming on `data-game-id`, and is adoptable incrementally without a migration cliff.
2. No backwards-compatibility shims or alias token sets — existing selectors migrate onto the new tokens directly.
3. `engine-core` and `game-stdlib` are untouched (presentation-only TypeScript shell asset); no mechanic nouns are introduced.

## Verification Layers

1. Token definitions exist → codebase grep-proof (`--rp-*` primitive/semantic and `--game-*` default custom properties defined in `styles.css`).
2. No UI framework introduced → grep-proof against `apps/web/package.json` (no `tailwindcss`/`@chakra-ui`/`@mui`/`bootstrap` dependency added).
3. Palette contrast → manual review (WCAG text + non-text contrast of every fg/bg and focus/non-text pair before ship).
4. Build + shell integrity → `npm run build` and `smoke:ui` stay green (the token layer must not break existing rendering).

## What to Change

### 1. Primitive tokens

Add a `:root` block of primitive tokens: raw color ramp (parchment/walnut/felt/brass/ink/cream/danger), spacing scale, radius scale, elevation/shadow, and a system-font typography scale (no font-asset risk).

### 2. Semantic tokens + per-game defaults

Add semantic tokens mapping primitives to UI meaning (page/table/card surfaces, text primary/secondary/muted, borders, focus-ring, danger) plus the per-game default group (`--game-accent`, `--game-accent-2`, `--game-accent-contrast`, art-bg/line, pattern opacity) that CATSETVIS-003 overrides per `data-game-id`.

### 3. Base selectors + reduced motion

Migrate the existing catalog/setup base selectors onto the tokens and add the `@media (prefers-reduced-motion: reduce)` block zeroing the transition tokens.

## Files to Touch

- `apps/web/src/styles.css` (modify)

## Out of Scope

- Per-game `data-game-id` accent blocks (CATSETVIS-003).
- `.game-card` anatomy/states (CATSETVIS-004) and `.match-setup`/`.setup-hero` (CATSETVIS-007).
- Any UI framework or CSS Modules adoption.
- Any Rust, behavior, legality, or renderer change.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` succeeds.
2. `npm --prefix apps/web run smoke:ui` is green (existing catalog shape rendering unbroken).
3. `grep -E '\-\-rp-(color|space|radius)|\-\-game-accent' apps/web/src/styles.css` returns the defined token set.

### Invariants

1. `apps/web/package.json` gains no UI/CSS framework dependency.
2. Tokens are CSS custom properties only — no behavior, no legality, no data flow.

## Test Plan

### New/Modified Tests

1. `None — presentation-only CSS change; no test files are added or modified. Verification is command-based (`build` + `smoke:ui`) against the existing pipeline named in Assumption Reassessment.`

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. `grep -E '\-\-rp-|\-\-game-' apps/web/src/styles.css` — confirms the token layer is present (narrower than a full visual diff; the redesign's visual proof lands with CATSETVIS-004/007 + the CATSETVIS-008 screenshot set).

## Outcome

Completed: 2026-06-13

What changed:
- Added a primitive and semantic CSS custom-property token layer to `apps/web/src/styles.css`, including `--rp-*` surface/text/border/focus/timing tokens and default `--game-*` catalog theme tokens.
- Migrated the existing catalog/setup base selectors onto the new token vocabulary without changing markup, behavior, legality, or data flow.
- Added a reduced-motion token override so later catalog/setup motion can share the same timing switch.

Deviations from plan:
- None. This ticket stayed CSS-only and introduced no UI framework or dependency.

Verification:
- `grep -E '\-\-rp-(color|space|radius)|\-\-game-accent' apps/web/src/styles.css` confirmed the token set.
- `rg -n "tailwindcss|@chakra-ui|@mui|bootstrap" apps/web/package.json` returned no matches.
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed.
