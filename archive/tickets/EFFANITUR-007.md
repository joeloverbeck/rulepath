# EFFANITUR-007: flood_watch animation adoption

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — TypeScript/React presentation shell only (`apps/web`); Rust/WASM untouched. Registers game-specific effect→animation mappings against the shared registry.
**Deps**: EFFANITUR-003

## Problem

`flood_watch` is the second motivating adopter: its flood phases and environment automation are auto-resolved advances that currently land as instant state swaps (the predecessor's O5 "auto-resolved phases happen invisibly" finding in concrete form). This ticket registers authored flood_watch effect→animation mappings so those automated bursts play out legibly on the timeline (spec D8 / WB7) and records its adoption row.

## Assumption Reassessment (2026-06-12)

1. `apps/web/src/components/FloodWatchBoard.tsx` renders the flood_watch board and is the registration site. The registry (`apps/web/src/animation/registry.ts`) and presenters come from EFFANITUR-003. flood_watch is one of the two games already adopting `TurnReportPanel` (`ADOPTED_GAMES = ["event_frontier","flood_watch"]`, `TurnReportPanel.tsx:9`), so its bursts are already narrated text — animation rides the same viewer-filtered stream.
2. Spec D8 / WB7: register animations for flood phases and environment-automation bursts; record an `adopt` adoption row. Automated-phase bursts are segmented by the shared burst module (EFFANITUR-001) per automated advance, so each flood phase is its own animatable burst.
3. Cross-artifact boundary under audit: the registry contract from EFFANITUR-003 and the flood_watch viewer-filtered effect stream as the sole input; the burst segmentation (EFFANITUR-001) that splits automated phases.
4. FOUNDATIONS §7/§11: flood-phase and environment-automation animations map Rust-emitted effect kinds; the renderer settles to the viewer-safe public view after each automated burst. No transition is inferred from state diffs.
5. No-leak / generic redacted (FOUNDATIONS §11 firewall): any redacted flood_watch effect animates with the generic viewer-safe presentation from EFFANITUR-003; registrations read no hidden field and add no new payload category or DOM/test-ID leak surface.

## Architecture Check

1. Registering flood-phase animations on the shared registry makes automated advances visible (closing O5 for this game) while keeping the manager single-owner — cleaner than bespoke per-phase timers in the board component.
2. No backwards-compatibility shim: authored flood motion replaces generic-only for the registered effects; no dual path.
3. `engine-core` untouched; registrations are `apps/web`-local (§3). No `game-stdlib` promotion.

## Verification Layers

1. flood_watch registers animations for flood phases / environment-automation bursts -> codebase grep-proof + `smoke:e2e` (`flood-watch.smoke.mjs`).
2. Automated bursts animate and settle to the viewer-safe view -> `smoke:e2e` animate-and-settle assertion across an automated phase.
3. Redacted effects (if any) animate generically; no leak surface -> no-leak visibility test (`a11y-noleak.smoke.mjs` unchanged-or-stronger) + grep-proof.

## What to Change

### 1. flood_watch effect→animation registrations

In `FloodWatchBoard.tsx`, register the flood_watch effect kinds onto the shared registry: flood-phase progression, environment-automation bursts, and the associated board highlights/transitions.

### 2. Adoption row

Record flood_watch's `adopt` adoption row (matrix consolidated in EFFANITUR-008's catalog sweep / closeout).

## Files to Touch

- `apps/web/src/components/FloodWatchBoard.tsx` (modify)
- `apps/web/e2e/flood-watch.smoke.mjs` (modify; registered-animation + automated-phase settle assertions — shared with EFFANITUR-009, which `Deps` this ticket)

## Out of Scope

- `event_frontier` adoption (EFFANITUR-006) and the 12-game catalog sweep (EFFANITUR-008).
- Any new Rust flood_watch effect semantics (a missing effect for a visible transition rides the ordinary fixture/trace path, spec §4.3).
- The dedicated `animation.smoke.mjs` and existing-smoke auto-advance updates (EFFANITUR-009).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` (`flood-watch.smoke.mjs`): flood phases and environment-automation bursts animate and settle to the viewer-safe view.
2. Grep-proof: flood_watch registrations exist on the shared registry and read no hidden field.
3. `npm --prefix apps/web run smoke:ui` and `build` green.

### Invariants

1. Every flood_watch registration maps a Rust-emitted effect kind; automated phases animate from effects, not state diffs (§7/§11).
2. Redacted effects animate generically; the no-leak firewall holds (§11).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/flood-watch.smoke.mjs` — flood-phase animate-and-settle + automated-burst + no-leak assertions.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run smoke:ui`
3. `npm --prefix apps/web run build`

## Outcome

Completed on 2026-06-12.

Flood Watch now registers authored effect animations on the shared registry for forecast reveal, environment-phase start, storm card draw, deck exhaustion, district bail/reinforce/levee absorption/flood rise/inundation, and terminal settlement.

Public animation targets were added to Flood Watch turn/budget/undrawn metrics, storm deck flow, district table, per-district flood/levee counters, and outcome status. The registrations only read Rust-emitted public effect payload fields (`type` and `district`) and do not read hidden deck order or private state.

The Flood Watch browser smoke now instruments `Element.animate` and asserts that forecast reveal animates the deck target, reinforce animates a public levee target, and the automated storm burst animates deck plus district targets before the viewer-safe board settles. `environment_phase_began` is registered for completeness, but the smoke asserts the visible automated burst effects after segmentation because marker entries are not animated as visible entries.

Verification:

1. `npm --prefix apps/web run build` -> passed.
2. `node apps/web/e2e/flood-watch.smoke.mjs` -> passed.
3. `npm --prefix apps/web run smoke:ui` -> passed.
4. `npm --prefix apps/web run smoke:e2e` -> passed.
5. `rg -n "animationRegistry\\.register\\(\"flood_watch\"" apps/web/src/components/FloodWatchBoard.tsx` -> registrations present.
6. `rg -n "full_deck_order|deck_order|hidden_state|private_state|internal_state|debug_state|seed_evidence|candidate_ranking|bot_candidate" apps/web/src/components/FloodWatchBoard.tsx apps/web/src/main.tsx` -> no matches.
