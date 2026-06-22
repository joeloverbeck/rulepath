# SEATFRAME-001: Stack the shared web-shell seat frame so the multi-seat layout never starves the rail

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: None — web shell only (`apps/web/src/styles.css`, and an assertion in `apps/web/e2e/vow-tide.smoke.mjs`). No Rust, no `engine-core`/`game-stdlib`/`games/*`, no schema, no trace.
**Deps**: none

## Problem

The shared web-shell seat frame renders an empty gap at high seat counts. At 7
seats on a desktop width, the viewer selector and the seat rail visibly mislay
out: the selector buttons sit in one wide row with a large blank area beneath
them, while the per-seat status rail is squeezed into a single narrow vertical
column on the right.

Root cause (verified by live measurement at 7 seats, ~1180px desktop width):
`.seat-frame` is a fixed two-column grid
`grid-template-columns: minmax(180px, auto) minmax(0, 1fr)`. The `auto` max on
column 1 lets the `.seat-frame-viewers` flex-wrap fieldset expand to its
single-row max-content (Observer + 7 seat buttons ≈ 702px), which starves
column 2 to ≈154px. The rail's `auto-fit minmax(128px, 1fr)` then collapses to
one column (7 cards stacked, ≈468px tall), and `align-items: stretch` inflates
the one-row viewers fieldset to that same ≈468px height — producing the empty
gap. Measured computed `grid-template-columns`: `702.344px 153.656px`.

This is shared chrome shown for every game, so the defect appears for any game
played at 5–7 seats, not just `vow_tide` where it was found.

## Assumption Reassessment (2026-06-22)

1. Root-cause CSS is `.seat-frame` at `apps/web/src/styles.css:234-243`
   (`grid-template-columns: minmax(180px, auto) minmax(0, 1fr)`,
   `align-items: stretch`). The rail at `apps/web/src/styles.css:301-308`
   already uses `grid-template-columns: repeat(auto-fit, minmax(128px, 1fr))`,
   so it reflows responsively once it is given full width. The viewers fieldset
   at `:245-254` is already `display: flex; flex-wrap: wrap`. Confirmed by live
   measurement (`702.344px 153.656px` at 7 seats / desktop).
2. Governing law is `docs/UI-INTERACTION.md` §10B "Multi-seat layout"
   (lines 246-270): "Multi-seat layouts must avoid fixed two-column or
   left/right assumptions"; "small-screen layouts may collapse the seat rail,
   but must preserve active seat, pending responder, local viewer, and outcome
   information without relying on color alone." The current fixed two-column
   grid is exactly the assumption §10B forbids, so this fix is also a
   doc-alignment correction.
3. Shared boundary under audit: the shared web-shell seat frame —
   `apps/web/src/components/SeatFrame.tsx` plus the `.seat-frame*` rules in
   `apps/web/src/styles.css`. It is rendered by `apps/web/src/main.tsx:569`
   (interactive, for every game) and `apps/web/src/components/ReplayViewer.tsx:85`
   (replay). There is exactly one shared surface and no per-game seat-frame CSS,
   so a single CSS change is the reuse-maximizing fix for all games. The
   `SeatFrame.tsx` markup (a `<fieldset>` followed by an `<ol>`) already supports
   vertical stacking, so no component/markup change is required.
4. No-leak surface: this change is CSS layout only; it does not alter what
   `SeatFrame` renders (viewer-safe catalog seat labels and Rust/WASM-projected
   active/pending/viewing status). The FOUNDATIONS §11 hidden-information no-leak
   firewall is untouched; `apps/web/e2e/a11y-noleak.smoke.mjs` remains the guard.
   No determinism/replay/hash surface is involved.
8. Adjacent contradiction (out of scope): the viewer selector and the rail each
   independently list all seats (selection vs. status). De-duplicating them into
   a single seat row is a pre-existing design choice, not a consequence of this
   layout fix; if wanted it must become its own ticket.

## Architecture Check

1. The vertical stack (chosen approach) removes the fixed two-column assumption
   at its root rather than capping or patching it. The two alternatives —
   capping column 1 (`minmax(180px, 240px)` + `align-items: start`) or a
   responsive `auto-fit` two-column grid — both still encode a two-column /
   left-right layout that §10B discourages and require breakpoint tuning. The
   rail's existing `auto-fit minmax(128px, 1fr)` already provides responsive
   reflow once it receives the full content width, so the fix is mostly
   deletion of the column template: the cleanest option and the most reused
   (one shared rule, applied to every game).
2. No backwards-compatibility shims or alias paths: this is a direct CSS
   replacement, no fallback layout retained.
3. `engine-core` / `game-stdlib`: N/A. This is web-shell CSS only; it introduces
   no mechanic nouns and no Rust changes, so the §3 noun-free kernel boundary and
   the §4 earned-`game-stdlib` rule are not engaged.

## Verification Layers

1. Seat frame no longer starves the rail at 7 seats on desktop (rail occupies
   ~full content width; viewers fieldset is not stretched to the rail height)
   -> UI smoke (extend `apps/web/e2e/vow-tide.smoke.mjs`, which already starts a
   7-seat match).
2. All seat orientation info (labels, active, pending, viewing, outcome) stays
   present at 3–7 seats and at mobile width, without a fixed two-column
   assumption -> UI smoke (existing `shell.smoke.mjs` /
   `seat-label-consistency.smoke.mjs` rail-row assertions) + manual review
   against `docs/UI-INTERACTION.md` §10B.
3. Presentation-only change introduces no hidden-information leak -> no-leak
   visibility test (`apps/web/e2e/a11y-noleak.smoke.mjs` and the `vow-tide.smoke.mjs`
   DOM/storage/console no-leak scan remain green) + FOUNDATIONS §11 alignment check.
4. The seat frame stays generic for all games (no per-game CSS added) -> manual
   review (single shared `.seat-frame` rule changed; no game-scoped selector
   introduced).

## What to Change

### 1. Replace the fixed two-column `.seat-frame` grid with a vertical stack

In `apps/web/src/styles.css`, change `.seat-frame` (currently lines 234-243)
from the two-column grid to a single-column vertical stack so the viewer
selector and the rail are each laid out at full width:

- Remove `grid-template-columns: minmax(180px, auto) minmax(0, 1fr)`.
- Remove `align-items: stretch` (the source of the stretched empty gap).
- Keep `display: grid; gap: 12px;` (now a single implicit column) — or
  equivalent `display: flex; flex-direction: column;`. Keep the existing border,
  radius, padding, and background.

Result: the viewer selector wraps naturally at full width, and the rail's
existing `repeat(auto-fit, minmax(128px, 1fr))` flows into several columns on
desktop (≈5–6 across) and collapses to 1–2 on mobile — no starvation, no gap.

### 2. Optional spacing polish (only if visually needed)

If stacking the selector above the rail looks cramped, add a small
`margin-block-end` to `.seat-frame-viewers` or rely on the existing `.seat-frame`
`gap`. Do not introduce any game-scoped selector. No change to
`SeatFrame.tsx` markup is expected.

### 3. Add a regression assertion to the 7-seat smoke

In `apps/web/e2e/vow-tide.smoke.mjs`, at the existing 7-seat setup, assert that
the seat frame is no longer a starved two-column layout: e.g. measure
`.seat-frame-rail` and `.seat-frame` bounding widths and assert the rail spans
roughly the full frame content width (rail width ≥ ~0.8 × frame inner width),
and/or that the rail's computed `grid-template-columns` resolves to ≥2 tracks at
the desktop viewport. Mirror the existing responsive-measurement idiom already
in that file (the `!columns.includes(" 0px ")` check).

## Files to Touch

- `apps/web/src/styles.css` (modify — `.seat-frame` rule; optional minor spacing on `.seat-frame-viewers`)
- `apps/web/e2e/vow-tide.smoke.mjs` (modify — add the seat-frame layout regression assertion)

## Out of Scope

- Merging or de-duplicating the viewer selector and the rail (both list seats).
- Any per-game board seat rail (e.g. `.vow-tide-seat-rail`, `.briar-seat`); those are board-native and unaffected.
- Any Rust/WASM, view-projection, schema, or `SeatFrame.tsx` markup change.
- Restyling colors, typography, or seat-card content — layout only.

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/vow-tide.smoke.mjs` — passes, including the new seat-frame layout assertion at 7 seats.
2. `node apps/web/e2e/shell.smoke.mjs` and `node apps/web/e2e/seat-label-consistency.smoke.mjs` — pass (seat-frame viewer/rail behavior unregressed).
3. `node apps/web/e2e/a11y-noleak.smoke.mjs` — passes (no hidden-info leak introduced).
4. `npm --prefix apps/web run smoke:e2e` — full e2e suite green.

### Invariants

1. `.seat-frame` uses no fixed two-column or left/right track template (`docs/UI-INTERACTION.md` §10B).
2. At every seat count 3–7 and at mobile width, the rail occupies the available full width and renders all seats with no stretched empty region; active/pending/viewing/outcome cues remain visible.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/vow-tide.smoke.mjs` — add an assertion at the existing 7-seat setup that the `.seat-frame-rail` spans ~full frame width (not the ≈154px starved column) and the viewers fieldset is not stretched to the rail height; reuses the file's existing viewport/measurement helpers.

### Commands

1. `npm --prefix apps/web run build && node apps/web/e2e/vow-tide.smoke.mjs` — targeted: rebuild dist, then run the 7-seat smoke with the new assertion.
2. `npm --prefix apps/web run smoke:e2e` — full-pipeline: all shell + per-game e2e smokes (build is included by the script).
3. A narrower single-file command is the correct primary boundary because the change is CSS-only in shared shell chrome; the full `smoke:e2e` run is the regression boundary because every game renders through the same `.seat-frame`.

## Outcome

Completed: 2026-06-22

What changed:

- Replaced the shared `.seat-frame` fixed two-column grid with the existing
  implicit single-column grid stack, preserving the frame border, padding,
  background, and gap.
- Added a 7-seat Vow Tide smoke assertion that measures the shared seat frame
  and fails if the rail is starved, resolves to only one nonzero track, or the
  viewer selector stretches into a tall empty panel.

Deviations:

- No optional spacing polish was needed; the existing `.seat-frame` gap is
  sufficient after stacking.
- No `SeatFrame.tsx`, Rust/WASM, schema, trace, or game-specific CSS changes
  were made.

Verification:

- `npm --prefix apps/web run build` passed.
- `node apps/web/e2e/vow-tide.smoke.mjs` passed with the new layout assertion.
- `node apps/web/e2e/shell.smoke.mjs` passed.
- `node apps/web/e2e/seat-label-consistency.smoke.mjs` passed.
- `node apps/web/e2e/a11y-noleak.smoke.mjs` passed.
- `node apps/web/e2e/animation.smoke.mjs` passed when rerun after the first
  full-suite tail timeout.
- First `npm --prefix apps/web run smoke:e2e` attempt passed through
  `vow-tide.smoke.mjs` and then timed out in the final `animation.smoke.mjs`
  `assertAnimationTargets` wait.
- Rerun `npm --prefix apps/web run smoke:e2e` passed end to end.
