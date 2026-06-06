# GAT3WASMSTAWEB-010: Dev/replay panel — viewer-safe and secondary

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — TypeScript/presentation only (`apps/web`); consumes the feature/version report op (GAT3WASMSTAWEB-002).
**Deps**: 001, 002, 004, 007, 009

## Problem

Gate 3 requires a dev/replay panel that is secondary to play, keyboard-accessible,
and restricted to viewer-safe data (spec §7.6, §16). It must NOT expose full
internal state by default, and a CSS toggle is not a security boundary (§16.4,
FOUNDATIONS §11 no-leak firewall). Today the only "dev" affordances are the
permanent "Submit Stale" button and inline diagnostics in the main play row; there
is no labeled, collapsible dev panel. This ticket consolidates the safe dev
surface and relocates the stale-action demo here.

## Assumption Reassessment (2026-06-06)

1. Current `apps/web/src/main.tsx` exposes a `data-testid="stale-action"` "Submit
   Stale" button in the main action row and a `data-testid="diagnostic"` block, with
   no dedicated dev panel. GAT3WASMSTAWEB-006 removes the stale button from the main
   row; this ticket re-homes it. The feature/version report op
   (`rulepath_feature_report`, GAT3WASMSTAWEB-002) supplies API version + op list;
   its typed client wrapper is added here (`apps/web/src/wasm/client.ts`,
   module created by GAT3WASMSTAWEB-001).
2. Spec §16.2 lists the allowed viewer-safe fields (API version, feature flags,
   game id/display name, rules/data/schema versions, match handle if safe, seed/
   setup, mode, actor/viewer, freshness token, action-tree summary/count, selected
   path, pending status, effect cursor, effect log summaries, replay metadata,
   command log if safe, normalized diagnostics); §16.3 forbids full/hidden/private
   state, bot-only facts, panic/backtrace, secrets in DOM/test-ids/classes/storage/
   logs/exports; §16.4 requires a keyboard-accessible labeled toggle and that unsafe
   data not be loaded into the payload at all; §16.5 makes the stale demo optional
   and dev-panel-only.
3. Cross-artifact boundary under audit: the panel reads reducer state
   (GAT3WASMSTAWEB-004), effect-cursor info (GAT3WASMSTAWEB-007), and replay metadata
   (GAT3WASMSTAWEB-009), plus the feature report. It surfaces summaries only — it is
   a read view, not a second authority.
4. FOUNDATIONS §11/§16 (no-leak; play-first): restated — for `race_to_n` full state
   is not hidden, but the panel MUST NOT normalize public internal-state dumps; it
   shows only the §16.2 viewer-safe fields so the pattern is safe before
   hidden-information games arrive.
5. §11 enforcement substrate (no-leak visibility firewall): the dev panel is the
   most likely place to leak future hidden state. Name the surface: §16.2 allowed
   fields are the firewall whitelist; the panel renders only those, never a raw
   internal-state object, and the toggle is not treated as a security boundary
   (unsafe data is simply not fetched/loaded). No leakage path is introduced; the
   later enforcement gate is the hidden-information game gates (ROADMAP Gate 8+).

## Architecture Check

1. A single collapsible `DevPanel` reading reducer-derived safe summaries (rather
   than scattered debug affordances in the play row) gives the spec's §10.3 normal/
   debug separation and one place to audit for leaks, with the stale demo clearly
   debug-labeled.
2. No backwards-compatibility shims: the inline stale/diagnostic affordances are
   consolidated/relocated, not duplicated.
3. `engine-core` untouched; the panel is a read view with no mechanic logic;
   `game-stdlib` untouched.

## Verification Layers

1. Panel shows only allowed fields → no-leak visibility test: rendered fields are a
   subset of §16.2; no raw internal-state object, no panic/backtrace, no bot-only
   facts (grep-proof: no full-state dump in `DevPanel.tsx`).
2. Toggle is keyboard-accessible and not a boundary → manual review: labeled button,
   keyboard operable; unsafe data is never loaded into the payload (not merely
   hidden by CSS).
3. Stale demo is dev-only and safe → simulation/CLI run: the relocated stale action
   uses viewer-safe diagnostics and refreshes view/action/effects after use (§16.5).
4. Secondary to play → FOUNDATIONS §7/§11 alignment check (play-first, not
   debug-dominated).

## What to Change

### 1. New `apps/web/src/components/DevPanel.tsx`

A collapsible, keyboard-accessible, visually-subordinate panel rendering the §16.2
viewer-safe fields from reducer state + the feature report. Include the relocated
stale-action demo (clearly debug-labeled) and normalized diagnostics. No full-state
inspector, no raw JSON as the normal way to understand the game.

### 2. `apps/web/src/wasm/client.ts`

Add a typed `featureReport()` method wrapping `rulepath_feature_report`
(GAT3WASMSTAWEB-002).

### 3. `apps/web/src/state/shellReducer.ts` + `apps/web/src/main.tsx`

Dev-panel visibility state + transition; mount `DevPanel` as a secondary surface.

## Files to Touch

- `apps/web/src/components/DevPanel.tsx` (new)
- `apps/web/src/wasm/client.ts` (modify) — add `featureReport()` wrapper
- `apps/web/src/state/shellReducer.ts` (modify) — dev-panel visibility state
- `apps/web/src/main.tsx` (modify) — mount dev panel

## Out of Scope

- Documenting dev-panel data-source safety in repo docs — GAT3WASMSTAWEB-015.
- The no-leak review/checklist + accessibility smoke — GAT3WASMSTAWEB-014.
- Any new Rust op (feature report already exists from -002).

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run build` — typecheck + Vite build succeed.
2. `cd apps/web && npm run smoke:ui` — the dev panel toggles open, shows safe summaries, and the relocated stale demo refreshes state safely.
3. `grep -rnE "JSON.stringify\\(state\\)|fullState|internalState|backtrace|panic" apps/web/src/components/DevPanel.tsx` — returns nothing (no full/internal-state dump, no panic text).

### Invariants

1. The dev panel renders only §16.2 viewer-safe fields; unsafe data is never loaded into the browser payload.
2. The panel is secondary to play and keyboard-accessible; the CSS toggle is not treated as a security boundary.

## Test Plan

### New/Modified Tests

1. `None — UI ticket; verification is `smoke:ui` + `tsc`; the no-leak negative assertions land in GAT3WASMSTAWEB-014 and browser E2E in GAT3WASMSTAWEB-013.`

### Commands

1. `cd apps/web && npm run build`
2. `cd apps/web && npm run smoke:ui`
3. The repo-wide no-leak review is GAT3WASMSTAWEB-014's boundary; here the component-scoped grep-proof + node smoke verify the panel's field whitelist.

## Outcome

Completed: 2026-06-06

What changed:

- Added typed `FeatureReport` and `RulepathApi.featureReport()` for the Rust feature/version report.
- Stored the feature report in reducer state during WASM bootstrap.
- Added a collapsible `DevPanel` that renders viewer-safe summaries only: API/features, game, match, setup, mode, actor/freshness, action count, effect cursor/count, pending state, replay metadata, and normalized diagnostics.
- Re-homed the stale-action demo into the dev panel behind the labeled toggle.
- Added subordinate dev-panel styles.

Deviations from original plan:

- None. Repo-wide no-leak assertions remain the later ticket boundary.

Verification results:

- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed with mode and replay coverage.
- `grep -rnE "JSON.stringify\\(state\\)|fullState|internalState|backtrace|panic" apps/web/src/components/DevPanel.tsx` returned no matches.
