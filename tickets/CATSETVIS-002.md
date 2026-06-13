# CATSETVIS-002: Original inline-SVG catalog icon registry

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/GameCatalogIcon.tsx` (new)
**Deps**: None

## Problem

Catalog cards are text-only with no per-game visual identity, so 14 games read as an undifferentiated text wall (spec §3 first-impression grounding). This ticket builds an inline-SVG registry — `GameCatalogIcon.tsx` — with 14 original, project-authored abstract icons selected by `game_id`, each recognizable by **shape family** as well as accent and legible in monochrome at small sizes. This is spec WB2 / §6 D3–D4 and the §3.5 motif gallery.

## Assumption Reassessment (2026-06-13)

1. No `GameCatalogIcon.tsx` exists, and `apps/web/src/components/GamePicker.tsx` / `MatchSetup.tsx` currently render no per-game art — verified this session. The component is new and is consumed by CATSETVIS-004 (catalog card) and -007 (setup hero); it is registered against the 14 `game_id`s in the `crates/wasm-api` catalog (`race_to_n`, `three_marks`, `column_four`, `directional_flip`, `draughts_lite`, `high_card_duel`, `token_bazaar`, `secret_draft`, `poker_lite`, `plain_tricks`, `masked_claims`, `flood_watch`, `frontier_control`, `event_frontier`).
2. Spec §5 motif gallery (one abstract motif per game) and §6 D4 govern the system: `viewBox="0 0 24 24"` base grid, `currentColor` + CSS variables (`--game-card-art-line`, `--game-accent-2`), monochrome-legible at 24/48/72 px and 200% zoom; `aria-hidden` next to a visible title, `role="img"` + `<title>` from `a11y_label` when standalone.
3. Cross-artifact boundary: the icon's accent comes from the `--game-*` tokens (CATSETVIS-001) and the per-game `data-game-id` accents (CATSETVIS-003); this component **references** those CSS variables at render time but does not import CSS, so it has no build-time dependency on 001/003.
4. FOUNDATIONS §10 (IP conservatism) + §7 (color-plus-shape) + §2 (behavior authority): every SVG is project-authored abstract geometry — no AI generation, no figurative/proprietary art, no trade-dress proximity per the §3.5 cautions. Selecting *which decorative glyph* renders for a `game_id` is presentation (§2-clean); the canonical `icon_id` is authored in Rust `ui.rs catalog_theme` (CATSETVIS-003) and this TS registry transcribes it rather than inventing a mapping.

## Architecture Check

1. An inline React-SVG registry keyed by `game_id` is the v1 default (React + SVG, FOUNDATIONS §7) — no external image files, no AI art; original geometry is both IP-safer and a better match for the warm/tactile/restrained doctrine than imitative box art.
2. No backwards-compatibility shims; the registry is additive (a new component with no prior caller).
3. `engine-core` / `game-stdlib` untouched; no mechanic nouns. The canonical game→icon mapping is Rust-authored (`ui.rs`, CATSETVIS-003); this component is presentation rendering only (§2).

## Verification Layers

1. 14 icons + `game_id` selection present → codebase grep-proof (`GameCatalogIcon.tsx` has a registry entry for every official `game_id`).
2. Small-size / monochrome legibility → manual review (render each at 24/48/72 px, monochrome, high contrast, 200% zoom).
3. IP originality (per asset) → manual review (project-authored, no trade-dress proximity, no AI/figurative source) — recorded in the CATSETVIS-009 IP closeout table.
4. a11y wiring → manual review (`aria-hidden` when an adjacent title names the game; `role="img"` + `<title>` when standalone) confirmed by `smoke:ui` / e2e a11y in CATSETVIS-008.

## What to Change

### 1. Inline SVG icon components

Author 14 original SVG components on the 24×24 grid using `currentColor` + `--game-card-art-line` / `--game-accent-2`, following the §3.5 motif gallery (e.g. `race_to_n` ascending step path, `column_four` four-token column, `poker_lite` center-pool slab) — abstract pressure, never the commercial look.

### 2. `game_id` → icon registry + selection

A presentation registry mapping each official `game_id` to its icon component, transcribing the Rust-authored `icon_id` (CATSETVIS-003), with a neutral fallback for an unknown id.

### 3. Accessibility props

Expose a `decorative` vs `standalone` mode: `aria-hidden="true"` when a visible title is adjacent; `role="img"` + `<title>` from the `a11y_label` when standalone.

## Files to Touch

- `apps/web/src/components/GameCatalogIcon.tsx` (new)

## Out of Scope

- Catalog card anatomy/states (CATSETVIS-004) and setup hero (CATSETVIS-007).
- Token definitions (CATSETVIS-001) and per-game accent blocks (CATSETVIS-003).
- The per-asset IP closeout table (CATSETVIS-009).
- Any AI-generated or figurative illustration; any external image file.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` succeeds.
2. `npm --prefix apps/web run smoke:ui` is green.
3. `grep -oE '"(race_to_n|three_marks|column_four|directional_flip|draughts_lite|high_card_duel|token_bazaar|secret_draft|poker_lite|plain_tricks|masked_claims|flood_watch|frontier_control|event_frontier)"' apps/web/src/components/GameCatalogIcon.tsx | sort -u | wc -l` returns `14`.

### Invariants

1. Every icon is project-authored inline SVG abstract geometry — no external image import, no AI/figurative source.
2. Each game's identity differs by shape family, not accent color alone (FOUNDATIONS §7; verifiable in monochrome).

## Test Plan

### New/Modified Tests

1. `None — presentation-only component; no test files are added or modified. Verification is `build` + `smoke:ui` plus manual small-size/monochrome legibility and IP-originality review (recorded in CATSETVIS-009).`

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. `grep -c 'game_id' apps/web/src/components/GameCatalogIcon.tsx` — confirms the registry enumerates per-game entries (the 14-id count assertion above is the precise check; legibility/IP are manual-review surfaces, not command-runnable).
