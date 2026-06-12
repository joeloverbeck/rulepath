# EFFANITUR-003: Presentation layer + per-game animation registry

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — TypeScript/React presentation shell only (`apps/web`); Rust/WASM untouched. Generic presentations key off the existing Rust-authored effect-tone taxonomy.
**Deps**: EFFANITUR-002

## Problem

The scheduler core (EFFANITUR-002) owns timing but draws nothing: it needs a presentation layer that realizes each step as restrained motion, plus a registration surface so games can map specific effects to specific animations. Without baseline generic presentations, the 12 non-adopter catalog games would get no motion; without a registry, the two adopters (`event_frontier`, `flood_watch`) have nowhere to declare authored animations. This ticket builds both (spec D2/D8 / WB3).

## Assumption Reassessment (2026-06-12)

1. `feedbackForEffect(entry): EffectFeedback` (`apps/web/src/components/effectFeedback.ts:14`) already classifies effects into the tone taxonomy `neutral | movement | turn | terminal` (`:11`) — the keying surface for generic presentations. Board SVGs render in `EventFrontierBoard.tsx` / `FloodWatchBoard.tsx` and siblings. `apps/web/src/styles.css` exists and is the home for overlay-layer / highlight / fade / reduced-motion styles. The scheduler step interface comes from `scheduler.ts` (EFFANITUR-002).
2. Spec D2/D8 / WB3 require: WAAPI helpers on transform/opacity of SVG `<g>` wrappers (`transform-box: fill-box`), FLIP measurement for moves, a ghost/overlay layer for cross-container transitions, and generic tone-keyed presentations (`movement`→FLIP/slide where targets registered else highlight; `turn`→actor-banner; `terminal`→outcome settle; `neutral`→brief highlight; redacted→generic face-down/stir), plus a per-game registry surface.
3. Cross-artifact boundary under audit: the scheduler step contract (EFFANITUR-002) and the `feedbackForEffect` tone taxonomy. Presenters are consumed by the scheduler; the registry is read by the scheduler when selecting a step's presentation. WAAPI animates CSS properties only — `transform` on SVG `<g>` (presentation attribute since SVG 2) with `transform-box: fill-box` is the safe path; raw geometry attributes are not animated.
4. FOUNDATIONS §7/§11: presentations realize Rust-emitted effects; React owns the settled state and motion is a transient layer on top (reverse staging only within a burst via the overlay, underlying DOM always the authoritative view, so an interrupt/flush is correct by construction).
5. No-leak (FOUNDATIONS §11 firewall, §12): redacted/hidden effects get a single generic viewer-safe presentation (face-down slide / unspecified stir) derived only from what the filtered stream already says — no new payload category, no DOM/test-ID/storage leak surface. The presenter for a redacted effect reads no hidden field because none is present in the filtered stream.

## Architecture Check

1. Raw WAAPI + FLIP + a ghost overlay covers the restrained-motion doctrine without inverting control toward render-state (the reason against adopting GSAP/Motion by default, spec A2). Tone-keyed generics give all 14 games baseline motion with zero per-game code, and the registry lets adopters layer authored motion without forking the manager.
2. No backwards-compatibility shim: generic presentations are the new baseline, not a fallback wrapper around old text-only rendering.
3. `engine-core` untouched; presenters/registry are `apps/web`-local (§3). No `game-stdlib` promotion — presentation shapes are not atlas pressure (spec P4 governance).

## Verification Layers

1. WAAPI transform/opacity on SVG `<g>` with `transform-box: fill-box`; FLIP measures first/last rects -> `smoke:ui` + manual review of the transform discipline.
2. Ghost/overlay layer clones for cross-container moves; underlying DOM stays the authoritative view -> `smoke:ui` overlay assertion.
3. Generic tone-keyed presentations cover all four tones + redacted -> node smoke keyed off `feedbackForEffect` outputs.
4. Redacted effects animate generically with no new leak surface -> no-leak visibility test (DOM/test-ID/storage) + grep-proof (no hidden-field read in the redacted presenter).
5. Registry surface accepts per-game effect→animation registrations consumed by the scheduler -> codebase grep-proof + `smoke:ui`.

## What to Change

### 1. Presenters module

Add `apps/web/src/animation/presenters.ts`: WAAPI helpers (transform/opacity on SVG `<g>` wrappers), FLIP measurement for moves, a ghost/overlay clone-and-reparent helper, and the generic tone-keyed presentations keyed off `feedbackForEffect`.

### 2. Registry module

Add `apps/web/src/animation/registry.ts`: a registration surface mapping a game's effect kinds to presentation builders, read by the scheduler when selecting each step's presentation; generic tone-keyed presentation is the default when no registration matches.

### 3. Overlay + motion styles

Add the overlay/portal layer, highlight/fade treatments, `transform-box: fill-box` rules, and reduced-motion styles to `apps/web/src/styles.css`.

### 4. Presentation smoke

Add `apps/web/scripts/smoke-presenters.mjs` asserting tone→presentation selection (including the redacted generic) and registry override resolution (wiring consolidated in EFFANITUR-009).

## Files to Touch

- `apps/web/src/animation/presenters.ts` (new)
- `apps/web/src/animation/registry.ts` (new)
- `apps/web/src/styles.css` (modify)
- `apps/web/scripts/smoke-presenters.mjs` (new)

## Out of Scope

- Per-game adopter registrations for `event_frontier` (EFFANITUR-006) / `flood_watch` (EFFANITUR-007) and the catalog sweep (EFFANITUR-008).
- Orchestration / replay wiring (EFFANITUR-004/005).
- Any new Rust effect semantics or visibility-contract change (redacted stays redacted).
- New animation-library dependency (spec A2 — deferred unless implementation proves pressure).

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/scripts/smoke-presenters.mjs` — each tone (`neutral`/`movement`/`turn`/`terminal`) and the redacted case resolve to their presentation; a registry registration overrides the generic.
2. No-leak: the redacted presenter reads no hidden field (grep-proof) and adds no test-ID/DOM/storage leak surface (`smoke:ui` / a11y-noleak assertions unchanged-or-stronger).
3. `npm --prefix apps/web run smoke:ui` and `npm --prefix apps/web run build` green.

### Invariants

1. Motion is a transient layer; the underlying DOM is always the authoritative settled view (§7/§11).
2. Redacted effects animate generically with no new payload category or leak surface (§11 no-leak firewall).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-presenters.mjs` — tone→presentation + registry override + redacted-generic selection.
2. `apps/web/scripts/smoke-ui.mjs` — extend for overlay-layer presence and SVG `<g>` transform discipline.

### Commands

1. `node apps/web/scripts/smoke-presenters.mjs`
2. `npm --prefix apps/web run smoke:ui`
3. `npm --prefix apps/web run build`

## Outcome

Completed: 2026-06-12

What changed:

- Added `apps/web/src/animation/presenters.ts` with generic tone-keyed presentation selection, WAAPI transform/opacity helpers, FLIP helper, and ghost-overlay cloning.
- Added `apps/web/src/animation/registry.ts` with per-game effect-type registration and generic fallback resolution.
- Added overlay, ghost, SVG `<g>` transform-box, and reduced-motion CSS hooks in `apps/web/src/styles.css`.
- Added `apps/web/scripts/smoke-presenters.mjs` covering all four feedback tones, redacted generic selection, and registry override resolution.

Deviations from the plan:

- The node smoke stubs the React import from `effectFeedback.ts` during in-memory transpilation because this ticket tests the pure `feedbackForEffect` selection path, not the React reduced-motion hook.

Verification results:

- `node apps/web/scripts/smoke-presenters.mjs` passed.
- `rg -n "hidden|private|deck|card|hand|mask|commit" apps/web/src/animation/presenters.ts` showed only the explicit redacted-effect type list and `aria-hidden` on cloned ghosts; no hidden/private payload fields are read.
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- `node apps/web/e2e/a11y-noleak.smoke.mjs` passed.
