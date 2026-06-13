# EFFANITUR-006: event_frontier animation adoption

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — TypeScript/React presentation shell only (`apps/web`); Rust/WASM untouched. Registers game-specific effect→animation mappings against the shared registry.
**Deps**: EFFANITUR-003

## Problem

`event_frontier` is a motivating adopter: its Reckoning bursts, card transitions, and funds/score changes are exactly the visible transitions effect-driven animation exists to render, but with only generic tone-keyed motion they read flatly. This ticket registers authored event_frontier effect→animation mappings on the shared registry (spec D8 / WB6) and records its adoption row.

## Assumption Reassessment (2026-06-12)

1. `apps/web/src/components/EventFrontierBoard.tsx` renders the EF board (sites, cards, resource tiles) and is the registration site. The registry surface (`apps/web/src/animation/registry.ts`) and presenters (FLIP/slide, overlay, count-up) come from EFFANITUR-003. EF emits Reckoning/card/funds effects on the viewer-filtered stream (the burst the predecessor's `TurnReportPanel` already narrates — confirmed via spec §2 / actconmat A5).
2. Spec D8 / WB6: register game-specific animations for reckoning bursts, card transitions, funds/score changes, and map highlights; record an `adopt` adoption row. Where a visible transition lacks an effect, adding it is ordinary Rust-side per-game work via the fixture/trace path (spec §4.3 / actconmat A5) — out of scope here unless an adopter transition is unanimatable from the existing stream.
3. Cross-artifact boundary under audit: the registry contract from EFFANITUR-003 (effect kind → presentation builder) as consumed by EF registrations; the EF viewer-filtered effect stream as the sole input.
4. FOUNDATIONS §7/§11: every EF registration maps a Rust-emitted effect kind to a presentation; none infers a transition the stream did not state. Reverse staging stays within the burst via the overlay (DOM remains the authoritative view).
5. No-leak (FOUNDATIONS §11 firewall): EF's `EF-VIS-002` undrawn-deck-order stance is unchanged — redacted/hidden EF effects animate with the generic viewer-safe presentation (face-down/stir) from EFFANITUR-003, never a registration that reads a hidden field. The adoption introduces no new payload category and no DOM/test-ID leak.

## Architecture Check

1. Registering EF animations on the shared registry (rather than hardcoding motion in the board component) keeps adopters layered on the one manager and proves the registry surface end-to-end with a real game.
2. No backwards-compatibility shim: authored EF motion replaces generic-only for the registered effects; no dual path.
3. `engine-core` untouched; EF registrations are `apps/web`-local (§3). No `game-stdlib` promotion — presentation shapes are not atlas pressure.

## Verification Layers

1. EF registers animations for reckoning bursts / card transitions / funds-score changes / map highlights -> codebase grep-proof (registrations present) + `smoke:e2e` (`event-frontier.smoke.mjs`).
2. Registered effects animate and settle to the viewer-safe view -> `smoke:e2e` animate-and-settle assertion.
3. Redacted EF effects animate generically; EF-VIS-002 unchanged -> no-leak visibility test (`a11y-noleak.smoke.mjs` unchanged-or-stronger) + grep-proof (no hidden-field read in EF registrations).

## What to Change

### 1. EF effect→animation registrations

In `EventFrontierBoard.tsx`, register the EF effect kinds onto the shared registry: Reckoning-burst grouped reveal, card transitions (overlay/ghost moves), funds/score count-ups, and map/site highlights.

### 2. Adoption row

Record EF's `adopt` adoption row (the spec adoption matrix consolidated in EFFANITUR-008's catalog sweep / closeout).

## Files to Touch

- `apps/web/src/components/EventFrontierBoard.tsx` (modify)
- `apps/web/e2e/event-frontier.smoke.mjs` (modify; registered-animation + settle assertions — shared with EFFANITUR-009, which `Deps` this ticket)

## Out of Scope

- `flood_watch` adoption (EFFANITUR-007) and the 12-game catalog sweep (EFFANITUR-008).
- Any new Rust EF effect semantics or visibility-contract change (EF-VIS-002 stance unchanged).
- The dedicated `animation.smoke.mjs` and existing-smoke auto-advance updates (EFFANITUR-009).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` (`event-frontier.smoke.mjs`): Reckoning bursts, card transitions, and funds/score changes animate and settle to the viewer-safe view.
2. Grep-proof: EF registrations exist on the shared registry and read no hidden field.
3. `npm --prefix apps/web run smoke:ui` and `build` green.

### Invariants

1. Every EF registration maps a Rust-emitted effect kind; no TS-inferred transition (§7/§11).
2. Redacted EF effects animate generically; EF-VIS-002 and the no-leak firewall hold (§11).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/event-frontier.smoke.mjs` — registered-animation animate-and-settle + redacted-generic + no-leak assertions.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run smoke:ui`
3. `npm --prefix apps/web run build`

## Outcome

Completed on 2026-06-12.

Event Frontier now registers authored effect animations on the shared registry for public deck/card flow, resource changes, operation/site changes, Reckoning bursts, and terminal settlement. The scheduler is wired through the registry presenter, and hotseat effect refreshes now drain through the scheduler so human Event Frontier actions exercise the same animation path.

Public animation targets were added to Event Frontier metrics, deck flow, map, outcome, and site nodes. The registrations only read Rust-emitted public effect payload fields (`type`, `faction`, `sites`, `site`, `from`, `to`, and `site_breakdown.site`) and do not read hidden deck order or private state.

The Event Frontier browser smoke now instruments `Element.animate` and asserts that an event action animates the deck target, a Reckoning animates map/resource/score targets, and at least one public site target animates before the board settles.

Verification:

1. `npm --prefix apps/web run build` -> passed.
2. `node apps/web/e2e/event-frontier.smoke.mjs` -> passed.
3. `npm --prefix apps/web run smoke:ui` -> passed.
4. `npm --prefix apps/web run smoke:e2e` -> passed.
5. `rg -n "animationRegistry\\.register\\(\"event_frontier\"" apps/web/src/components/EventFrontierBoard.tsx` -> registrations present.
6. `rg -n "full_deck_order|internal_trace_full_deck_hash|private_state|hidden_state|bot_candidate|candidate_ranking" apps/web/src/components/EventFrontierBoard.tsx apps/web/src/main.tsx` -> no matches.
