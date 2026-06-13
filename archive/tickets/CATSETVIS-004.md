# CATSETVIS-004: Catalog card redesign (`GamePicker.tsx`)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/GamePicker.tsx`, `apps/web/src/styles.css`
**Deps**: 001, 002, 003

## Problem

The catalog renders text-first cards with no per-game art, no fixed anatomy, and minimal state design. This ticket rebuilds `GamePicker.tsx` into one repeated fixed-aspect card anatomy — accent rail, art well (the original SVG), eyebrow, `display_name` title, summary, flag chips, and a shape-plus-text selected mark — on a responsive fixed-slot grid, with full hover / focus-visible / active / selected / loading / empty / failure states and keyboard traversal. Whole-card selection and a separate How-to-Play action are preserved. This is spec WB4 / §6 D3, D7.

## Assumption Reassessment (2026-06-13)

1. `apps/web/src/components/GamePicker.tsx` currently renders a `.game-list` grid of text-first `.game-card` entries with a whole-card click target and a secondary How-to-Play button (verified this session). **Change rationale (no silent retcon):** the spec deliberately replaces the text-first anatomy with the fixed-slot, art-bearing anatomy per §6 D3 — this is the intended P2 redesign, not a regression of the shipped whole-card-click behavior, which is preserved.
2. Spec §6 D3 (fixed-aspect card, stable slots, `-webkit-line-clamp`), D7 (keep select-game and open-How-to-Play as two distinct actions; avoid nested buttons), and §4.3 (TS never derives legality; descriptions/identity never parsed for behavior) govern this ticket.
3. Cross-artifact boundary: this ticket imports `GameCatalogIcon` (CATSETVIS-002 — producer/consumer dependency), styles against the `--rp-*`/`--game-*` tokens (CATSETVIS-001), and renders per-game identity via `data-game-id` (CATSETVIS-003). The `.game-list` / `.game-card` CSS section in `apps/web/src/styles.css` is this ticket's section of that shared file (coordinate mechanical merges with 001/003/007).
4. FOUNDATIONS §7 (legal-only, play-first, color-plus-shape) + §2 (TypeScript presentation-only): titles render `display_name`, never raw `game_id`; every state (focus, selected) uses outline/shape/text, not accent color alone; no legality or rule state is decided in TypeScript.

## Architecture Check

1. One repeated fixed-aspect anatomy with stable slots and text clamping scans better than variable-height masonry for a 14-game wall (spec §3.3 grounding) and preserves the existing whole-card-click + separate How-to-Play interaction rather than nesting buttons.
2. No backwards-compatibility shims — the text-card markup is replaced outright, not aliased behind a flag.
3. `engine-core` / `game-stdlib` untouched; this is presentation-only TypeScript + CSS.

## Verification Layers

1. No raw IDs / engine vocabulary in catalog DOM → e2e no-leak/a11y smoke (`smoke:e2e` `a11y-noleak`) + manual review (cards show `display_name` and public copy only).
2. Two distinct actions preserved → manual review + `smoke:ui` (select-game and How-to-Play each reachable, keyboard-traversable, not nested).
3. States are not color-only → manual review (focus-visible = outline + offset; selected = accent rail + chip/check shape + text).
4. Build + shell integrity → `npm run build` and `smoke:ui` green.

## What to Change

### 1. `GamePicker.tsx` card anatomy

Rebuild each card as: decorative accent rail; a primary whole-card selection button containing art well (`GameCatalogIcon`), eyebrow, `display_name` title, summary (variant count / selected-variant label), and flag chips; a shape-plus-text selected mark; and a separate How-to-Play button (outer card ignores clicks originating in the How-to-Play control).

### 2. `styles.css` catalog section

Add the `.game-list` responsive grid (`repeat(auto-fit, minmax(...))`), the fixed card aspect ratio + summary clamp, and the hover / focus-visible / active / selected / loading-skeleton / empty / failure state styles, all on the CATSETVIS-001 tokens.

### 3. Keyboard traversal

Native document order (primary button + How-to-Play button as tab stops); any optional arrow-key enhancement must not hide the How-to-Play control from keyboard users.

## Files to Touch

- `apps/web/src/components/GamePicker.tsx` (modify)
- `apps/web/src/styles.css` (modify; `.game-list` / `.game-card` section)

## Out of Scope

- Match-setup redesign (CATSETVIS-007).
- Icon authoring (CATSETVIS-002), token definitions (CATSETVIS-001), per-game accents (CATSETVIS-003).
- Variant `description` rendering (CATSETVIS-006/007) and any variant-selection behavior.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` succeeds and `npm --prefix apps/web run smoke:ui` is green.
2. `npm --prefix apps/web run smoke:e2e` is green (the `a11y-noleak` smoke confirms no raw `game_id`/engine vocabulary in card DOM).
3. Manual: focus-visible (outline + offset) and selected (rail + chip/check shape + text) are present and not color-only.

### Invariants

1. Card titles render `display_name`, never a raw `game_id`.
2. Select-game and How-to-Play remain two distinct, keyboard-reachable actions; no nested buttons.

## Test Plan

### New/Modified Tests

1. `None — presentation-only redesign; no test files change here. The variant-description and full a11y/no-raw-ID assertions are added in CATSETVIS-008; this ticket is verified by `build` + `smoke:ui` + the existing `smoke:e2e` a11y/no-leak suite.`

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. `npm --prefix apps/web run smoke:e2e` — the `e2e/a11y-noleak.smoke.mjs` step is the no-raw-ID / no-leak verification boundary for the card DOM.

## Outcome

Completed: 2026-06-13

What changed:
- Rebuilt `apps/web/src/components/GamePicker.tsx` cards around fixed slots: accent rail, art well using `GameCatalogIcon`, eyebrow, display-name title, summary, flags, selected mark, and separate How-to-Play action.
- Added `data-game-id` to card containers so the CATSETVIS-003 per-game token overrides can theme the card without TypeScript deciding behavior.
- Added responsive fixed-aspect catalog-card CSS, including hover, focus-visible, active, selected, and text-clamp states on the CATSETVIS-001 token layer.

Deviations from plan:
- None. Whole-card selection and the separate How-to-Play button were preserved without nested buttons.

Verification:
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- `npm --prefix apps/web run smoke:e2e` passed, including the a11y/no-leak raw-identifier guard.
- Manual CSS review confirmed focus-visible uses outline/offset, selected state uses rail + text chip + dot shape, and titles render `display_name` rather than raw ids.
