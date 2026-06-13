# MODECTRL-001: Fix shared Mode panel layout (Bot why placement + over-stretched control buttons)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — presentation only. Touches `apps/web/src/components/ModeControls.tsx` and `apps/web/src/styles.css`. No Rust crates, schemas, traces, or behavior.
**Deps**: None

## Problem

The shared in-play Mode panel (`ModeControls`, rendered for all 13 catalog games) is visually broken, most visibly on Event Frontier where the "Bot why" disclosure is present:

1. **"Bot why" is a third flex-child of a 2-column header row.** `.mode-controls` is `display:flex; justify-content:space-between; flex-wrap:nowrap` and was designed as *label | actions*. `ModeControls.tsx` renders the `<details className="bot-note bot-why">` as a sibling of that row, and `styles.css` papers over it with `.bot-why { flex: 1 0 100% }`. Because the parent is `nowrap`, the `100%` basis cannot wrap to its own line: at desktop width the disclosure squishes into the header row; at narrow width it collapses into a cramped ~116px yellow box. Broken at every width.
2. **Short-label control buttons over-stretch.** At the narrow breakpoint, `.mode-actions button { flex: 1 1 140px }` forces Rules / Skip / Pause to each grow to ~195px (confirmed via computed style: `flex-grow:1; flex-basis:140px; width:194.7px`), producing oversized buttons for one-word labels.

This is a shared-component defect: fixing `ModeControls` fixes the panel for every game. Live-reproduced on Event Frontier, Human vs bot, at `http://127.0.0.1:4173/` (2026-06-13).

## Assumption Reassessment (2026-06-13)

1. `apps/web/src/components/ModeControls.tsx:55-124` renders `<section className="mode-controls">` containing exactly three direct children when a bot decision exists: the label `<div>` (`Mode` / mode name / status), `<div className="mode-actions">` (buttons), and `<details className="bot-note bot-why">`. Confirmed live: `.mode-controls` direct children are `["DIV", "mode-actions", "bot-note bot-why"]`.
2. `apps/web/src/styles.css:1554-1575` defines `.mode-controls` (`display:flex; align-items:end; justify-content:space-between; gap:14px; flex-wrap` defaults to `nowrap`) and `.mode-actions` (`display:flex; flex-wrap:wrap; justify-content:end; gap:10px`). `styles.css:1518-1520` defines `.bot-why { flex: 1 0 100% }`. `styles.css:3299-3318` is the narrow-breakpoint block that sets `.mode-controls { align-items:start; flex-direction:column }`, `.mode-actions { width:100%; justify-content:stretch }`, and `.mode-actions button { flex: 1 1 140px }`.
3. This ticket touches only the TypeScript/React presentation shell. FOUNDATIONS §2 reserves presentation/layout to TypeScript and §10A states orchestration/pacing is presentation policy in TypeScript; no behavior, legality, view-projection, effect, or determinism surface is in scope. No engine boundary is crossed.
4. The "Bot why" disclosure body is gated `gameId === "event_frontier"` (`ModeControls.tsx:117`). This ticket fixes its *placement/layout* only; it does not change which games render it (see Out of Scope).
5. Adjacent contradiction classification: the `gameId === "event_frontier"` hardcode inside a shared component is a latent architecture smell but is **future cleanup**, not a consequence of this layout fix; it must become its own ticket if pursued.

## Architecture Check

1. The clean fix is structural, not another flex hack: stop making `.bot-why` a flex sibling of the header row. Give `.mode-controls` a vertical structure — a header row (`label | actions`) plus the bot-why disclosure beneath it — and remove the `.bot-why { flex: 1 0 100% }` workaround. Capping button growth at the breakpoint (so one-word buttons size to content rather than stretching to a 140px basis) is a value change in the same rule. This is simpler and more robust than escalating the `nowrap`/`flex-basis:100%` conflict with more overrides.
2. No backwards-compatibility aliasing or shims introduced; the old `.bot-why { flex: 1 0 100% }` hack is removed, not aliased.
3. `engine-core` is untouched and remains free of mechanic nouns (§3); no `game-stdlib` change, so the mechanic-atlas earned-helper rule (§4) does not apply. Per `docs/UI-INTERACTION.md §10A`, repeated per-game presentation shapes are not mechanic-atlas promotion pressure.

## Verification Layers

1. Panel structure is correct (bot-why no longer a flex sibling of the header row) -> codebase grep-proof + manual UI play-first audit (screenshot at desktop and narrow widths).
2. Control buttons no longer over-stretch for short labels -> manual review of computed button width at the narrow breakpoint (one-word buttons size to content / a sane cap, not ~195px).
3. No regression to the existing in-play shell across games -> simulation/CLI-equivalent: `npm --prefix apps/web run smoke:ui` and `npm --prefix apps/web run smoke:e2e` (drives Event Frontier among all catalog games).
4. FOUNDATIONS alignment: presentation-only, no Rust/behavior surface -> FOUNDATIONS alignment check (§2 presentation ownership, §10A presentation-policy boundary).

## What to Change

### 1. `ModeControls.tsx` — restructure the panel so bot-why is not in the header flex row

Wrap the label `<div>` and the `<div className="mode-actions">` in a single header-row container (e.g. `<div className="mode-controls-header">`), and render the `<details className="bot-note bot-why">` as a sibling *of that header container*, beneath it — so `.mode-controls` becomes a vertical stack of `[header-row, bot-why]` rather than a `nowrap` row of `[label, actions, bot-why]`. Preserve all existing props, button wiring, `aria-label`s, and the `data-testid="bot-explanation"`.

### 2. `styles.css` — layout rules

- Make `.mode-controls` a vertical container (`flex-direction: column` with appropriate `gap`), and introduce `.mode-controls-header` as the row that carries the previous `space-between` / `align-items: end` behavior (label left, actions right).
- Remove the `.bot-why { flex: 1 0 100% }` hack (the disclosure no longer needs to force a wrap). Keep the `.bot-note` / `.bot-why` visual styling (border, background, summary cursor/weight).
- At the narrow breakpoint (`styles.css:3299-3318`), stop short-label control buttons from over-stretching: replace `.mode-actions button { flex: 1 1 140px }` with growth that sizes one-word buttons to content (e.g. `flex: 0 1 auto` with a safe touch `min-width`), keeping touch targets comfortable per `docs/UI-INTERACTION.md §18`.
- Re-check the desktop `.mode-controls`/`.mode-actions` rules and the narrow-breakpoint overrides remain coherent with the new column structure (the breakpoint no longer needs to flip `.mode-controls` to `column` if it is already column; adjust to avoid dead rules).

## Files to Touch

- `apps/web/src/components/ModeControls.tsx` (modify)
- `apps/web/src/styles.css` (modify)

## Out of Scope

- Changing the Skip / Pause / Resume enablement or visibility logic (that is MODECTRL-002).
- Generalizing the "Bot why" disclosure to games other than Event Frontier, or removing the `gameId === "event_frontier"` hardcode (separate future-cleanup ticket).
- Any Rust, WASM, effect, view, or bot change.
- Visual redesign beyond fixing the broken layout (no new palette, iconography, or motion).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` (includes `tsc --noEmit`) passes.
2. `npm --prefix apps/web run smoke:ui` passes.
3. `npm --prefix apps/web run smoke:e2e` passes (drives Event Frontier and all catalog games end-to-end).

### Invariants

1. `.mode-controls` has no flex-child that relies on a `flex-basis: 100%` wrap hack; the "Bot why" disclosure renders on its own line beneath the header row at all widths.
2. One-word control buttons (Rules / Skip / Pause / Resume) size to content or a sane cap and never stretch to ~1/3 of the panel width at any breakpoint.
3. No Rust/behavior/effect/view/legality change; `git diff` touches only `apps/web/src/components/ModeControls.tsx` and `apps/web/src/styles.css`.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/event-frontier.smoke.mjs` — extend (or confirm) an assertion that the rendered `.mode-controls` structure places `.bot-why` outside the header row (e.g. assert the header-row container exists and `.bot-why` is not its sibling-in-row), so the layout regression cannot silently return.
2. Manual visual review — screenshot the Event Frontier Mode panel (Human vs bot) at a desktop width and a narrow width; confirm Bot why sits on its own line and the buttons are content-sized. Attach screenshots to the PR.

### Commands

```bash
npm --prefix apps/web run build
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:e2e
```
