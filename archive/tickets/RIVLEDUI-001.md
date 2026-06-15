# RIVLEDUI-001: Stop the "Selected" pill from overlapping game-card flag pills

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — TypeScript/React presentation only (`apps/web/src/components/GamePicker.tsx`, `apps/web/src/styles.css`)
**Deps**: none

## Problem

On the "Choose a game" catalog, selecting a card renders a decorative
"● Selected" pill (`.game-selected-mark`) that overlaps the card's flag pills
(`.game-flags`: seat-count, "Hidden info", "N views"). Reproduced live at
1200px on River Ledger: the flags box occupies `top 491 → bottom 538` while the
Selected mark occupies `top 526 → bottom 549`, a ~12px overlap that visually
clips "Hidden info" / "7 views". River Ledger triggers it most because it has
the longest flag set (`3, 4, 5, 6 seats` + `Hidden info` + `7 views`) which
wraps onto two rows.

Root cause: `.game-option` (`styles.css:2150-2166`) is a CSS grid with exactly
**three** explicit rows (`grid-template-rows: minmax(84px, 0.9fr) auto
minmax(28px, auto)`), a fixed `aspect-ratio: 4 / 5`, and `overflow: hidden`.
`GamePicker.tsx:56-60` adds `.game-selected-mark` as a **fourth** in-flow grid
child only when the card is selected. With no fourth track defined and the card
height pinned by the aspect ratio, the mark falls into an implicit overflowing
row that paints on top of the third (flags) row instead of below it.

## Assumption Reassessment (2026-06-15)

1. `apps/web/src/components/GamePicker.tsx:56-60` renders `<span
   className="game-selected-mark" aria-hidden="true">Selected</span>` as a
   direct child of `<button className="game-option">`, sibling to
   `.game-flags` (lines 49-55). Confirmed by read.
2. `apps/web/src/styles.css:2150-2166` sets `.game-option { display: grid;
   aspect-ratio: 4 / 5; overflow: hidden; grid-template-rows: minmax(84px,
   0.9fr) auto minmax(28px, auto); }`; `.game-selected-mark` (lines 2259-2280)
   uses `justify-self: start` (a grid-item property) but has no row reserved.
   Confirmed by read.
3. The mark is `aria-hidden="true"` and purely decorative; the card's selected
   state is already conveyed non-visually via `aria-pressed`
   (`GamePicker.tsx:38`) and the `.game-card.selected` / `.game-option.selected`
   border + box-shadow treatment (`styles.css:2246-2257`). Removing the mark
   from normal grid flow therefore loses no semantics.
4. Mismatch + correction: none. The reported defect matches current code; no
   ticket assumptions need correcting.

## Architecture Check

1. **Chosen approach — take the decorative mark out of grid flow as an absolute
   corner badge.** Because `.game-option` is already `position: relative`
   (`styles.css:2151`) and already hosts an absolutely-positioned decorative
   child (`.game-card-accent`, lines 2185-2191), positioning
   `.game-selected-mark` absolutely removes it from the three-track grid
   entirely. It can no longer consume or overlap the flags row, the fix is
   width- and wrap-count-independent (it holds however many flag pills wrap),
   and uniform card heights are preserved (no aspect-ratio or row-count change).
2. Alternatives considered and rejected: (a) adding a fourth explicit grid row
   and dropping `aspect-ratio` — breaks uniform card heights across the grid;
   (b) moving the mark inside `.game-flags` — that container only renders when
   `hidden_information || viewer_modes?.length` (`GamePicker.tsx:49`), so a game
   with neither would have nowhere to render the mark.
3. No backwards-compatibility aliasing/shims introduced. No `engine-core` or
   `game-stdlib` changes; this is presentation-only and touches no mechanic
   nouns.

## Verification Layers

1. Selected indicator never overlaps flag pills at any card width -> manual
   review (Puppeteer bounding-box check: `.game-selected-mark` and every
   `.game-flags > span` must have non-intersecting rects on the selected card).
2. All flag pills remain fully visible when a card is selected -> UI smoke
   (`apps/web/e2e/shell.smoke.mjs` extended to assert flag-pill visibility on
   the selected card).
3. Selected state remains accessible without the decorative mark -> codebase
   grep-proof (`aria-pressed` retained on `.game-option`).

## What to Change

### 1. Reposition `.game-selected-mark` as an absolute corner badge (`apps/web/src/styles.css`)

In the `.game-selected-mark` rule (`styles.css:2259-2271`), replace the
grid-flow placement (`justify-self: start`) with absolute positioning anchored
to a card corner that does not collide with the title or flag pills — pin it to
the top-right over the `.game-art` region:

- `position: absolute;`
- a top/right inset using existing spacing tokens (e.g. `top: var(--rp-space-4);
  right: var(--rp-space-4);`);
- a `z-index` above `.game-card-accent` and the art so it is not clipped;
- remove `justify-self: start`.

Keep the existing pill visuals (border, radius, padding, background, the
`::before` accent dot). Confirm the badge stays inside the card's
`overflow: hidden` rounded bounds and does not cover the game title or eyebrow.

### 2. (Only if needed) keep the mark last in DOM order (`apps/web/src/components/GamePicker.tsx`)

No JSX change is required for the absolute-badge approach; the element already
renders last (`GamePicker.tsx:56-60`). Do **not** move it into `.game-flags`.

## Files to Touch

- `apps/web/src/styles.css` (modify)
- `apps/web/e2e/shell.smoke.mjs` (modify — add the no-overlap / flags-visible assertion)

## Out of Scope

- Any change to `.game-flags` content, ordering, or the seat-count/views/hidden-info pills themselves.
- The Seats setup control (covered by RIVLEDUI-002).
- Rust catalog metadata (`supported_seats`, `viewer_modes`, `hidden_information`).
- Restyling the card beyond what is required to stop the overlap.

## Acceptance Criteria

### Tests That Must Pass

1. With River Ledger selected, a Puppeteer check confirms the
   `.game-selected-mark` rect does not intersect any `.game-flags > span` rect,
   and every flag pill rect is fully within the `.game-option` rect.
2. `npm --prefix apps/web run build` succeeds (no TypeScript/bundler errors).
3. `npm --prefix apps/web run smoke:e2e` passes (full web e2e suite, including
   the extended `shell.smoke.mjs`).

### Invariants

1. Selecting any catalog card never visually overlaps or clips its flag pills,
   regardless of card width or how many flag pills wrap.
2. The selected state remains conveyed accessibly via `aria-pressed`; the
   decorative mark stays `aria-hidden`.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/shell.smoke.mjs` — select a multi-flag card (River Ledger) and
   assert: (a) `.game-selected-mark` bounding rect does not intersect any
   `.game-flags > span` rect; (b) each flag-pill rect is contained within the
   `.game-option` rect.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:e2e`
3. `node apps/web/e2e/shell.smoke.mjs` — narrower targeted run of the modified
   smoke while iterating (the full `smoke:e2e` is the gate).

## Outcome

Completed: 2026-06-15

What changed:

- Repositioned `.game-selected-mark` as an absolute top-right decorative badge
  inside `.game-option`, removing it from the catalog card grid flow so it no
  longer creates an implicit overflowing row over `.game-flags`.
- Extended `apps/web/e2e/shell.smoke.mjs` to select River Ledger, assert the
  selected badge does not intersect any flag pill, and assert all selected-card
  flag pills remain inside the card bounds before continuing the existing Race
  to 21 shell flow.

Deviations from plan:

- None.

Verification:

- `npm --prefix apps/web run build`
- `node apps/web/e2e/shell.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`
