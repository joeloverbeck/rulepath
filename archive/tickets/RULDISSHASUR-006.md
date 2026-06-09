# RULDISSHASUR-006: Wire rules access points (picker, setup, in-play) + mount panel

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/{GamePicker,MatchSetup,ModeControls,AppShell}.tsx`; no Rust/engine/WASM behavior, and no legality moves to TypeScript (FOUNDATIONS §2).
**Deps**: RULDISSHASUR-005

## Problem

The shared `RulesPanel` is useless until reachable. Rules must be openable from three contexts — the game picker (before selection), match setup (after selection, before start), and in play (during a live match) — and the panel must be mounted so it can overlay from any context. Source: `specs/rules-display-shared-surface.md` §7.2 (entry points), §9.1 (web modules).

## Assumption Reassessment (2026-06-09)

1. `apps/web/src/components/GamePicker.tsx` maps `GameCatalogEntry[]` with `game.game_id` available per card; `MatchSetup.tsx` has a selected-game details area before the Start Match action with `selectedGame?.game_id`; `ModeControls.tsx` is an existing in-play controls component; `AppShell.tsx` is the top-level wrapper. The `RulesPanel` component and the `rulesPanelOpened`/`rulesPanelClosed` actions come from RULDISSHASUR-005.
2. Entry-point placement is spec §7.2; the in-play anchor follows the `/reassess-spec` M3 correction — the trigger lives in the in-play controls (`ModeControls.tsx` / `ActionControls.tsx`) reading the active `game_id` from shell state, NOT in `AppShell.tsx`, which is a minimal wrapper that does not receive `game_id`.
3. Cross-artifact boundary under audit: the three triggers dispatch RULDISSHASUR-005's `rulesPanelOpened` action (presentation-only shell state); the panel is mounted once in `AppShell.tsx` so it overlays from any context. This ticket consumes the component + actions 005 introduces (structural producer/consumer dependency).
4. FOUNDATIONS principle restated: §2 (TypeScript presents only). The picker "How to Play" control must NOT select the game, start a match, create a seed, or touch Rust state; the in-play control must NOT change the match or switch viewer perspective.
5. No-leak: each trigger passes only a public catalog `game_id` to the panel — never hidden match state. RULDISSHASUR-007 asserts the match view JSON and legal actions are unchanged when rules open.

## Architecture Check

1. Wiring all three triggers to one shared action + one mounted panel keeps a single source of presentation truth; placing the in-play trigger in the controls component (which has `game_id` via shell state) avoids threading `game_id` into the minimal `AppShell`.
2. No backwards-compatibility shims; the additions are new event handlers + one mount point.
3. `engine-core`/`game-stdlib` untouched; all change is `apps/web` presentation. Triggers decide no legality (§2).

## Verification Layers

1. Picker trigger opens panel without side effects → grep-proof the handler dispatches `rulesPanelOpened` only (no `new_match`/`apply_action`/seed creation).
2. Triggers are real focusable controls with game-specific accessible names → `<button>` with `aria-label` like `How to play Crest Ledger` (full a11y assertion in RULDISSHASUR-007).
3. In-play open does not mutate the match → before/after public-view JSON and legal-action set identical (asserted in RULDISSHASUR-007).
4. Panel mounted once → grep-proof `AppShell.tsx` renders `<RulesPanel>` gated on `rulesPanelOpen`.
5. Behavior-authority alignment → manual review against FOUNDATIONS §2.

## What to Change

### 1. `apps/web/src/components/GamePicker.tsx` (modify)

Add a per-card secondary "How to Play" `<button>` (accessible name `How to play <display_name>`, visually secondary to selection) that opens the panel for that card's `game_id`. It must not select the game or start a match (if the component structure forces a UI-only selection, it must not start a match).

### 2. `apps/web/src/components/MatchSetup.tsx` (modify)

Add a "How to Play / Rules" control in the selected-game details area before Start Match, using `selectedGame.game_id`; it stays available while adjusting seed/mode/seats.

### 3. `apps/web/src/components/ModeControls.tsx` (modify)

Add a persistent in-play "Rules" `<button>` beside the existing match controls, reading the active `game_id` from shell state and opening the panel; it never changes the match, the active seat, or the viewer perspective, and never reads hidden data.

### 4. `apps/web/src/components/AppShell.tsx` (modify)

Mount `<RulesPanel>` (gated on `rulesPanelOpen`) at shell level so it overlays from picker, setup, and in-play.

## Files to Touch

- `apps/web/src/components/GamePicker.tsx` (modify)
- `apps/web/src/components/MatchSetup.tsx` (modify)
- `apps/web/src/components/ModeControls.tsx` (modify)
- `apps/web/src/components/AppShell.tsx` (modify)

## Out of Scope

- The `RulesPanel` component, loader, shell state, styles, and Markdown dependency (RULDISSHASUR-005).
- The e2e / accessibility / no-leak smoke (RULDISSHASUR-007).
- Authoring or generating player docs/assets (RULDISSHASUR-003/-004).
- Any Rust/engine/WASM change; making a tooltip the primary rules affordance.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` succeeds with the wired triggers and mounted panel.
2. `npm --prefix apps/web run smoke:ui` passes.
3. Opening rules from the picker does not create a match (no `new_match` call in the trigger path) — grep/manual confirm.

### Invariants

1. Every entry control is a keyboard-focusable accessible control with a game-specific accessible name; the primary rules access is never hover-only.
2. Opening/closing rules from any context changes no Rust state — no action applied, no seat/viewer change, no replay/effect mutation (FOUNDATIONS §2).

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/{GamePicker,MatchSetup,ModeControls,AppShell}.tsx` — wired triggers + mount; exercised by `smoke:ui` and the RULDISSHASUR-007 e2e.
2. `None additional here — the end-to-end open/close/focus assertions live in RULDISSHASUR-007's smoke.`

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. The cross-surface open/close/focus/no-mutation assertions are RULDISSHASUR-007's boundary (it adds the e2e harness); `build` + `smoke:ui` are correct for the wiring diff.

## Outcome

Completed: 2026-06-09

What changed:

- Added per-game `How to Play` buttons to the game picker without nesting buttons or selecting/starting matches.
- Added a selected-game `How to Play / Rules` button to match setup.
- Added an in-play `Rules` button to `ModeControls`.
- Mounted `RulesPanel` through `AppShell` with presentation-only shell state and callbacks.
- Added secondary button/card layout styles for the new controls.

Deviations from original plan:

- `main.tsx` was also touched to pass the new AppShell rules-panel props and the trigger callbacks into picker/setup/in-play controls.

Verification results:

- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- Grep/manual review confirmed the trigger handlers call `onRulesOpen(game_id)` / dispatch `rulesPanelOpened` only. Existing `newMatch`, action, viewer, replay, and bot calls remain in their pre-existing gameplay paths, not the rules trigger paths.
- `git diff --check` passed.
