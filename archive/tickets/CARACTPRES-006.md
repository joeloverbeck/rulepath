# CARACTPRES-006: Flood Watch DeckFlowPanel adoption

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web` (Flood Watch board adoption, client types, smoke); no Rust surface touched
**Deps**: CARACTPRES-003, CARACTPRES-005

## Problem

Flood Watch's board renders its storm deck as raw view strings — `view.forecast ?? "None"` (`apps/web/src/components/FloodWatchBoard.tsx:103`) and a hand-rolled "Storm deck" history section (`:139-141`) — with no card meaning. With CARACTPRES-003 projecting resolved card faces + `ui` metadata and CARACTPRES-005 shipping the shared `DeckFlowPanel`, this ticket completes the WB5 split: Flood Watch adopts the shared surface, including the public `undrawn_count` as a real count badge (unlike Event Frontier's redacted pile).

## Assumption Reassessment (2026-06-12)

1. `FloodWatchBoard.tsx` exists and renders the raw forecast/drawn data (verified at `:103,139-141` this session); the per-game smoke is `apps/web/e2e/flood-watch.smoke.mjs` (verified). The flood_watch view exposes `undrawn_count: u8` publicly (`games/flood_watch/src/visibility.rs:31,100`) — the count badge is rule-legal here.
2. Spec authority: `specs/card-and-action-presentation-shared-surfaces.md` §6 D3, §8 WB5, §9 exit criterion 5 (parity). This is the rest-of-split of the pilot-vs-rest decomposition noted at Step 4 (WB5 → 005+006).
3. Cross-artifact boundary under audit: `apps/web/src/wasm/client.ts` `FloodWatchPublicView` types — updated to CARACTPRES-003's resolved-face + `ui` projection. Consumers: `FloodWatchBoard.tsx` and `main.tsx`'s view guard (grep-verify no others at implementation). Coordinated breaking-for-TS change within the spec, same as 005.
4. FOUNDATIONS §2/§7/§11 restated: rendered deck text comes from Rust faces/`ui` only; the drawn-history and forecast surfaces are public by rule, so no new leak surface arises; DOM/test-IDs for the face-down pile carry count (public) but no identity/order facts.

## Architecture Check

1. Adopting the shared panel (slot mapping: forecast → next slot; current resolution flow → current slot; drawn history → discard/disclosure list; undrawn → face-down slot with count badge) beats keeping a bespoke storm-deck section: one designed surface, per-game data only — and it exercises the panel's public-count variant, proving D3's count-badge-only-when-public switch with a real consumer.
2. No backwards-compatibility aliasing/shims: the bespoke storm-deck markup is removed in the same diff.
3. `engine-core`/`game-stdlib` untouched; presentation-layer mechanic nouns are legitimate in `apps/web`.

## Verification Layers

1. No TS-derived or raw-ID deck text remains -> codebase grep-proof over `FloodWatchBoard.tsx` (no `view.forecast ?? "None"` raw rendering; no ID-string interpolation).
2. Count badge renders the Rust-projected `undrawn_count`, not a TS-computed length -> UI smoke assertion in `flood-watch.smoke.mjs`.
3. No-leak: face-down pile exposes count only, no identity/order -> `a11y-noleak.smoke.mjs` assertions for flood_watch.
4. Single-layer scope note: not applicable — three invariants map to three distinct surfaces above.

## What to Change

### 1. Client types

Update `FloodWatchPublicView` in `apps/web/src/wasm/client.ts` to CARACTPRES-003's resolved card-face + `ui` shape.

### 2. Board adoption

Replace the bespoke storm-deck/forecast markup in `FloodWatchBoard.tsx` with `DeckFlowPanel` (forecast → next slot, drawn cards → disclosure list, undrawn → face-down slot with count badge); route panel copy from `view.ui`; update the sr-only summary to authored labels.

### 3. Smoke coverage

Extend `flood-watch.smoke.mjs` (resolved labels/summaries render; count badge matches view; raw IDs absent) and `a11y-noleak.smoke.mjs` (flood_watch face-down assertions).

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/FloodWatchBoard.tsx` (modify)
- `apps/web/src/styles.css` (modify — only if slot-variant styles need additions)
- `apps/web/e2e/flood-watch.smoke.mjs` (modify)
- `apps/web/e2e/a11y-noleak.smoke.mjs` (modify)

## Out of Scope

- `DeckFlowPanel` component changes beyond what the count-badge variant already supports (variant gaps route back as 005 follow-ups, not forked panel logic here).
- Flood Watch action-panel work — CARACTPRES-008 audit territory.
- Catalog copy hygiene — CARACTPRES-009.
- Any Rust change — CARACTPRES-003 owns the crate side.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui` green.
2. Extended e2e: forecast/current slots show authored labels + summaries; count badge equals the view's `undrawn_count`; drawn-history disclosure lists resolved labels; no raw card IDs in rendered text.
3. `npm --prefix apps/web run smoke:effects` green.

### Invariants

1. Every visible deck string originates from Rust view data or the `ui` channel (FOUNDATIONS §2).
2. The face-down pile exposes exactly the view-projected count and nothing else (§11).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/flood-watch.smoke.mjs` — slot rendering, count-badge fidelity, raw-ID absence.
2. `apps/web/e2e/a11y-noleak.smoke.mjs` — flood_watch face-down no-leak assertions.

### Commands

1. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:e2e`
3. Narrow boundary rationale: presentation-only adoption diff; Rust gates ran at CARACTPRES-003.

## Outcome

Completed on 2026-06-12.

- Updated Flood Watch web types to match the Rust/WASM card-face and `ui` projection from CARACTPRES-003.
- Replaced the bespoke Storm deck section with shared `DeckFlowPanel`, mapping latest drawn card, forecast card, drawn-card disclosure, and the public `undrawn_count` badge.
- Extended Flood Watch and shared a11y/no-leak smokes for authored forecast text, count-badge fidelity, expandable drawn-card disclosure, and raw-ID absence.
- Kept Rust untouched; this is a presentation-only web adoption.

Verification:

- `npm --prefix apps/web run build`
- `node apps/web/e2e/flood-watch.smoke.mjs`
- `node apps/web/e2e/a11y-noleak.smoke.mjs`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:e2e`
