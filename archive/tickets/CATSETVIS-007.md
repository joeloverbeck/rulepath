# CATSETVIS-007: Match-setup polish (hero + variant description + seat/faction labels)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/MatchSetup.tsx`, `apps/web/src/styles.css`
**Deps**: 001, 002, 003, 006

## Problem

Match-setup is functional but flat: it shows the display name, a variant `<select>`, mode radios, and a seat grid, but carries no selected-game identity, never displays a variant description, and never surfaces faction labels (which exist in the catalog metadata but go unshown). This ticket redesigns setup around a selected-game hero, displays the selected variant's optional description, and adds a prominent "Players & roles" section surfacing `seat_labels` and `faction_labels`, with user-facing mode copy. Spec WB7 / §6 D6 / §7.

## Assumption Reassessment (2026-06-13)

1. `apps/web/src/components/MatchSetup.tsx` renders `display_name`, a variant `<select>`, mode radios, a How-to-Play button, and a `.seat-roles` grid from `ui.seat_labels` (`MatchSetup.tsx:169`). `ui.faction_labels` exists in the catalog type (`client.ts:97-98`) **and is projected** through wasm-api (`lib.rs:7395`/`7417`) but is **not surfaced** in setup — verified this session. **Change rationale:** surface `faction_labels` prominently and add the hero + variant-description display per §6 D6; the existing variant-selection and seat-mapping logic are preserved (presentation-only).
2. Spec §6 D6 (selected-game hero; variant-description beneath the selector; "Players & roles" with `seat_labels` + `faction_labels`; fallback "Player 1/2" only when Rust UI metadata is absent; faction labels near the hero for asymmetric games; user-facing mode copy — never "Rust legal bot"/engine phrasing) and §7 (How-to-Play reachable from the hero) govern.
3. Cross-artifact boundary: consumes `GameCatalogIcon` (CATSETVIS-002) for the hero, the `--rp-*`/`--game-*` tokens (CATSETVIS-001) and `data-game-id` accents (CATSETVIS-003), and the variant description via `selectVariantDescription` / `GameVariantCatalogEntry.description?` (CATSETVIS-006). `faction_labels` is read from the already-projected catalog `ui` JSON — no new projection needed. The `.match-setup`/`.setup-hero` CSS is this ticket's section of the shared `styles.css` (coordinate with 001/003/004).
4. FOUNDATIONS §7 (cozy-premium setup; How-to-Play surface) + §2 (presentation-only; reuse the existing variant-selection/seat logic, decide no legality) + §11 no-leak: `seat_labels`/`faction_labels` are public projected metadata; cooperative role labels must be explicit but must **not** reveal hidden/private state.

## Architecture Check

1. A selected-game hero that carries identity forward plus a prominent seat/faction section reduces re-parsing after selection (spec §3 progressive-disclosure grounding) and reuses the existing functional variant selector and seat mapping rather than re-speccing them.
2. No backwards-compatibility shims — the flat setup markup is replaced, not aliased.
3. `engine-core` / `game-stdlib` untouched; presentation only — `faction_labels` is consumed from the existing Rust projection, so no behavior moves into TypeScript.

## Verification Layers

1. `faction_labels` surfaced + `seat_labels` mapped → manual review + `smoke:ui` (Players & roles renders the projected labels; "Player 1/2" fallback fires only when metadata is absent).
2. Variant description shown when present, omitted when absent → `smoke:ui` (the CATSETVIS-008 assertion) + manual review of the trim-and-omit path.
3. No engine/debug copy, no hidden-role leak → `smoke:e2e` no-leak/a11y suite + manual review (mode copy is user-facing; cooperative role labels reveal no private state).
4. How-to-Play reachable from the hero → manual review + `smoke:ui`.

## What to Change

### 1. Setup hero

`header.setup-hero[data-game-id]` with `GameCatalogIcon`, `display_name`, a public summary, flag chips, and a How-to-Play button.

### 2. Variant selector + description

Keep the existing functional selector; render the selected variant's `description` (via `selectVariantDescription`, CATSETVIS-006) beneath it, omitting the paragraph entirely when absent/blank.

### 3. Players & roles + mode copy

Add a "Players & roles" section surfacing `seat_labels` (seat + actor) and `faction_labels` (chips, near the hero for asymmetric games), with "Player 1/2" fallback only when metadata is absent; rewrite mode copy to user-facing phrasing.

### 4. `styles.css` setup section

Add the `.match-setup` / `.setup-hero` styles on the CATSETVIS-001 tokens.

## Files to Touch

- `apps/web/src/components/MatchSetup.tsx` (modify)
- `apps/web/src/styles.css` (modify; `.match-setup` / `.setup-hero` section)

## Out of Scope

- Catalog card redesign (CATSETVIS-004), icon authoring (CATSETVIS-002).
- The variant `description` Rust field / projection (CATSETVIS-005/006) and any variant-selection behavior change.
- The `smoke-ui.mjs` description assertion (CATSETVIS-008).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` succeeds and `npm --prefix apps/web run smoke:ui` is green.
2. `npm --prefix apps/web run smoke:e2e` is green (no hidden role/private state in the DOM, no raw `game_id`, no engine/debug copy in normal mode).
3. Manual: hero identity, the selected variant description (when present), and faction chips render; "Player 1/2" appears only when metadata is absent.

### Invariants

1. `seat_labels`/`faction_labels` render from Rust-projected metadata; the "Player 1/2" fallback fires only when that metadata is absent.
2. No hidden/private state appears in role labels; normal-mode mode copy carries no "Rust legal bot"/engine vocabulary.

## Test Plan

### New/Modified Tests

1. `None — presentation-only setup redesign; no test files change here. The variant-description assertion is added in CATSETVIS-008. Verification is `build` + `smoke:ui` + the existing `smoke:e2e` a11y/no-leak suite.`

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. `npm --prefix apps/web run smoke:e2e` — the no-leak/a11y boundary for hidden-role and raw-ID checks in setup.

## Outcome

Completed: 2026-06-13

What changed:
- Redesigned `MatchSetup.tsx` around a selected-game hero with `GameCatalogIcon`, display name, setup summary, How-to-Play action, and per-game `data-game-id` theming.
- Rendered the selected variant's optional description through `selectVariantDescription(...)`, omitting the element when absent.
- Added prominent "Players & roles" presentation for Rust-projected `seat_labels`, plus faction chips when `ui.faction_labels` is present.
- Rewrote normal-mode play-mode details to user-facing automated-opponent copy without "Rust legal bot" wording.
- Added setup hero, faction chip, variant-description, and players/roles CSS.

Deviations from plan:
- The first `smoke:e2e` run failed because `rules-display.smoke.mjs` still expects the setup How-to-Play button under `.setup-summary`. The hero action container now carries `setup-summary` as a compatibility class while preserving the new hero layout; the rerun passed.

Verification:
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- `npm --prefix apps/web run smoke:e2e` passed after the selector compatibility fix, including a11y/no-leak coverage.
- Manual review confirmed variant descriptions render only when present, faction labels surface from Rust-projected catalog metadata, and fallback player labels remain available when metadata is absent.
