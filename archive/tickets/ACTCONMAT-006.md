# ACTCONMAT-006: TurnReportPanel — narrate non-interactive advances

**Status**: COMPLETE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only for the panel) — `apps/web/src/components/TurnReportPanel.tsx` (new), `apps/web/src/main.tsx`; plus `games/event_frontier` / `games/flood_watch` adoption notes + per-game audit rows. Re-presents the existing viewer-filtered effect payload — no new data crosses the boundary.
**Deps**: ACTCONMAT-001

## Problem

Confirming one Survey can trigger a funds spend, a full Reckoning auto-resolution, and two card transitions in one click with no board narration — the only record is the effect log at the very bottom of the page. `flood_watch` environment automation and every bot turn have the same problem: auto-resolved phases and bot turns happen invisibly. A shared turn report must narrate what happened since the player's last decision, in authored vocabulary, adjacent to the board.

## Assumption Reassessment (2026-06-12)

1. The page already receives a viewer-filtered effect-log payload, rendered bottom-of-page by `apps/web/src/components/EffectLog.tsx` (mounted in `main.tsx`). No near-board narration surface exists. Authored-label vocabulary depends on the resolved labels from ACTCONMAT-001 (so the report reads "First Reckoning", not "reckoning 1").
2. Spec D6 / §4.2 / A5: a shared `TurnReportPanel` sits adjacent to the action surface and shows the viewer-filtered effects since the player's last decision, grouped by resolution burst, in authored-label copy. It is a re-presentation of the existing effect-log payload — no new data crosses the boundary. Adopted by `event_frontier` and `flood_watch`; other games get an audit row ("not applicable" when no automation bursts).
3. Cross-artifact boundary under audit: the viewer-filtered effect-log payload (read side) re-presented near the board; the bottom effect log remains as full history.
4. FOUNDATIONS §7/§11 (play-first UI, no leaks): the turn report renders the same viewer-filtered effects the page already receives — no new payload category, no new exposure.
5. No-leak firewall surface: confirm the panel reads ONLY the already-viewer-filtered effect payload and introduces no path to unfiltered effects (A5: if a game's effects are too sparse to narrate, that game's effect coverage is the gap, fixed Rust-side, not via TS reaching for hidden data).

## Architecture Check

1. Re-presenting the existing effect payload near the board (rather than emitting new Rust data) gives glanceable narration at zero new boundary surface — the minimal change that satisfies the narration goal. Grouping by resolution burst matches how the effects already arrive.
2. No shim: the bottom effect log stays as full history; the panel is an additive surface, not a replacement with a compatibility bridge.
3. No engine change; `engine-core` untouched. The panel is a shared TS component; per-game adoption is a `ui.rs`/audit-row note, no `game-stdlib` addition.

## Verification Layers

1. Turn report narrates the Reckoning burst in authored vocabulary -> UI smoke (`apps/web/e2e/event-frontier.smoke.mjs`).
2. Panel renders only viewer-filtered effects -> no-leak visibility test (`apps/web/e2e/a11y-noleak.smoke.mjs` coverage).
3. `flood_watch` automation burst narrated; other games carry an audit row -> UI smoke (`flood-watch.smoke.mjs`) + codebase grep-proof of audit rows.

## What to Change

### 1. TurnReportPanel component

Add `apps/web/src/components/TurnReportPanel.tsx`: viewer-filtered effects since the player's last decision, grouped per resolution burst with authored-label copy ("First Reckoning resolved — no instant victory. Sites scored: Charter +2, Freeholders +2. Income: +2 funds, +2 provisions. Edicts expired.").

### 2. Mount adjacent to the action surface

Mount the panel in `main.tsx` adjacent to the action surface (the bottom `EffectLog` remains as full history).

### 3. Per-game adoption + audit rows

`event_frontier` and `flood_watch` adopt it; record a one-row audit per catalog game (most have no automation bursts → "not applicable").

## Files to Touch

- `apps/web/src/components/TurnReportPanel.tsx` (new)
- `apps/web/src/main.tsx` (modify; mount the panel)
- `apps/web/e2e/event-frontier.smoke.mjs` (modify; reckoning-burst narration)
- `apps/web/e2e/flood-watch.smoke.mjs` (modify; automation-burst narration)

## Out of Scope

- Effect-driven board animation / animation scheduler (spec §4.3; deferred) — the turn report is narration, not animation.
- Auto-running bot turns (deferred) — this narrates around the existing manual trigger.
- New Rust effect data — re-presentation only (A5).

## Acceptance Criteria

### Tests That Must Pass

1. `apps/web/e2e/event-frontier.smoke.mjs` asserts a readable turn report adjacent to the board after a Reckoning burst, in authored vocabulary.
2. `apps/web/e2e/flood-watch.smoke.mjs` asserts the environment-automation burst is narrated; `flood_watch` parity.
3. `npm --prefix apps/web run smoke:e2e` and `npm --prefix apps/web run smoke:effects` green.

### Invariants

1. The panel renders only the viewer-filtered effect payload already delivered to the page — no new data crosses the boundary (§11 no-leak).
2. Every catalog game has a turn-report adoption or an explicit "not applicable" audit row.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/event-frontier.smoke.mjs` — reckoning-burst turn-report assertion.
2. `apps/web/e2e/flood-watch.smoke.mjs` — automation-burst narration.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run smoke:effects`
3. `npm --prefix apps/web run build`

## Outcome

Completed 2026-06-12.

- Added `TurnReportPanel`, mounted adjacent to the play surface for Event Frontier and Flood Watch, reusing only the existing viewer-filtered `EffectEntry[]` payload and `feedbackForEffect` copy.
- Added compact responsive styling for the near-board report while preserving the bottom `EffectLog` as full history.
- Added Event Frontier and Flood Watch turn-report assertions for Reckoning and storm automation bursts.
- Added per-game `TURN_REPORT_ADOPTION` / `TURN_REPORT_AUDIT` rows in `games/*/src/ui.rs`.

Verification:

- `cargo fmt --all --check`
- `npm --prefix apps/web run build`
- `node apps/web/e2e/event-frontier.smoke.mjs`
- `node apps/web/e2e/flood-watch.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`
- `npm --prefix apps/web run smoke:effects`
