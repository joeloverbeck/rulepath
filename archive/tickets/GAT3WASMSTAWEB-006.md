# GAT3WASMSTAWEB-006: Race-to-N renderer and action-tree-driven legal controls

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — TypeScript/presentation only (`apps/web`); renders Rust-supplied view/action-tree data.
**Deps**: 004, 005

## Problem

The active-play surface must replace the raw harness feel with an intentional
Race-to-N renderer (counter, target, active seat, winner, effect summary) and map
each Rust action choice to a control — with no TypeScript-side legality
calculation (spec §12, §13.1; FOUNDATIONS §2/§7, §11 "TypeScript does not decide
legality"). Today the controls render from `tree?.choices` but the surrounding code
also exposes a permanent "Submit Stale" demo button in the main action row (§10.3
forbids this in normal play), and the renderer is a minimal scoreboard/track.

## Assumption Reassessment (2026-06-06)

1. `apps/web/src/main.tsx` currently renders legal controls by mapping
   `tree?.choices` to buttons keyed on `choice.segment`, disabled when
   `view.active_seat !== "seat_0" || view.winner !== null` (lines ~403–413), plus a
   permanent `data-testid="stale-action"` "Submit Stale" button in the same row
   (lines ~414–421). The action tree comes from `get_action_tree` →
   `ActionChoice { segment, label, accessibility_label }` (client types). `PublicView`
   carries `counter`, `target`, `active_seat`, `winner`, `freshness_token`
   (and Rust-side `max_add`, `legal_additions`, `schema_version`, `rules_version`).
2. Spec §12.2 lists required status fields (counter, target, max-add if useful,
   active seat, active/terminal, winner, latest action/effect); §13.1 makes the Rust
   action tree the only source of normal clickable actions; §13.3/§10.2 forbid
   TS-derived legality and using `legal_additions` as the clickable-action source
   when the action tree is available; §10.3 moves the stale demo out of the main row
   (into the dev panel, GAT3WASMSTAWEB-010).
3. Cross-artifact boundary under audit: the renderer/controls consume the
   `PublicView` and `ActionTree` types from `apps/web/src/wasm/client.ts`
   (GAT3WASMSTAWEB-001) and reducer state (GAT3WASMSTAWEB-004); they mount inside
   the `AppShell` play region (GAT3WASMSTAWEB-005). Action dispatch preserves the
   Rust action `segment`/path and the freshness token (§13.1).
4. FOUNDATIONS §2/§7/§11 (no TS legality): restated — controls render exactly the
   Rust action-tree choices; the UI computes no add-1/add-2/add-3 legality and does
   not read `legal_additions` for clickability. Disabling is "operation in flight"
   or "not this actor's turn / terminal" only.

## Architecture Check

1. A dedicated renderer (`RaceBoard`/status) + an action-controls component driven
   solely by the action tree is cleaner than the inline scoreboard + mixed
   action/demo row: it isolates presentation, removes the stale-demo from normal
   play, and preserves action path/segment identity for dispatch and future
   compound actions (§13.4).
2. No backwards-compatibility shims: the inline control row is replaced; the stale
   demo relocates to the dev panel ticket rather than being duplicated.
3. `engine-core` untouched; no mechanic-legality logic enters React; `game-stdlib`
   untouched.

## Verification Layers

1. Controls come only from the Rust action tree → codebase grep-proof: no
   `legal_additions`-driven clickability and no add-value legality arithmetic in
   `apps/web/src/components`.
2. Action dispatch preserves Rust identity → schema validation: dispatched commands
   carry the action-tree `segment`/path + freshness token unchanged (client types).
3. Renderer settles to Rust view → simulation/CLI run: after each action/bot turn
   `smoke:ui` shows counter/turn/winner matching the Rust view (§14.5 settle-to-view).
4. No TS legality → FOUNDATIONS §2/§11 alignment check.

## What to Change

### 1. New `apps/web/src/components/RaceBoard.tsx` (+ status)

Render the Race-to-N visual model (counter, target, active seat, winner/terminal,
recent increment/effect summary) as React + SVG/semantic HTML (§12.1, §12.2);
original, neutral, no proprietary trade dress.

### 2. New `apps/web/src/components/ActionControls.tsx`

Map each Rust `ActionChoice` to a button with a visible label + accessible name +
preserved `segment`/path; disabled only while an op is in flight or when it is not
the actor's turn / the match is terminal. No TS legality. Terminal state renders
controls absent/inert with clear status (§12.4).

### 3. `apps/web/src/main.tsx` (+ reducer wiring)

Mount the renderer/controls in the play region; remove the permanent "Submit Stale"
button from the main action row (it moves to the dev panel in GAT3WASMSTAWEB-010).

## Files to Touch

- `apps/web/src/components/RaceBoard.tsx` (new)
- `apps/web/src/components/ActionControls.tsx` (new)
- `apps/web/src/main.tsx` (modify) — mount renderer/controls; drop stale-demo from main row

## Out of Scope

- Effect log + animation feedback + reduced motion — GAT3WASMSTAWEB-007.
- Multi-mode play (hotseat/bot-vs-bot) seat handling — GAT3WASMSTAWEB-008.
- Stale-action diagnostic relocation into the dev panel — GAT3WASMSTAWEB-010.
- Learning/debug "show unavailable choices" mode (not required for Race-to-N, §13.3).

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run build` — typecheck + Vite build succeed.
2. `cd apps/web && npm run smoke:ui` — a human action applies via an action-tree control and the board settles to the Rust view.
3. `grep -rnE "legal_additions|add-?[123].*legal|counter \+ [0-9]" apps/web/src/components` — returns nothing (no TS legality / no `legal_additions`-driven clickability).

### Invariants

1. Normal clickable actions are exactly the Rust action-tree choices; the UI computes no legality.
2. After every action the renderer settles to the latest Rust-projected view (§14.5).

## Test Plan

### New/Modified Tests

1. `None — renderer/controls UI ticket; verification is `smoke:ui` + `tsc`; rendered-DOM/role assertions land in GAT3WASMSTAWEB-013.`

### Commands

1. `cd apps/web && npm run build`
2. `cd apps/web && npm run smoke:ui`
3. Role/label DOM assertions are the Puppeteer harness's job (GAT3WASMSTAWEB-013); node smoke + typecheck are the correct boundary for renderer wiring + the no-legality grep-proof.

## Outcome

Completed: 2026-06-06

What changed:

- Added `RaceBoard` to render the Rust-projected counter, target, active/terminal status, freshness token, SVG progress track, and latest effect summary.
- Added `ActionControls` to render exactly the Rust action-tree choices as buttons with Rust-provided labels/accessibility labels.
- Removed the permanent stale-action demo button from normal play.
- Updated `main.tsx` to mount the new board and action controls in the play surface.
- Added baseline board/action-panel styles.

Deviations from original plan:

- None. Stale-action relocation remains deferred to the dev-panel ticket.

Verification results:

- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed with version `rulepath-wasm-api/0.1.0`, match `race_to_n-1`, counter `2`, `8` effects, and stale-action diagnostic coverage.
- `grep -rnE "legal_additions|add-?[123].*legal|counter \+ [0-9]" apps/web/src/components` returned no matches.
