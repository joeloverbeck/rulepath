# EFFANITUR-008: Catalog adoption sweep + dev settle-assertion

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — TypeScript/React presentation shell only (`apps/web`); Rust/WASM untouched. The dev settle-assertion compares rendered DOM against the authoritative view as a diagnostic only.
**Deps**: EFFANITUR-003

## Problem

The two adopters (`event_frontier`, `flood_watch`) get authored motion, but the remaining 12 catalog games must be verified to get correct baseline motion from the generic tone-keyed presentations, and each catalog game needs a recorded adoption row. Separately, FOUNDATIONS §7/§11 require the renderer to settle to the authoritative view after animation — a dev-only assertion that flags missing effect coverage (lingering ghosts, unanimated transitions) makes settle failures visible without ever driving animation from state diffs (spec D7 / WB8).

## Assumption Reassessment (2026-06-12)

1. There are 14 games under `games/` (confirmed: column_four, directional_flip, draughts_lite, event_frontier, flood_watch, frontier_control, high_card_duel, masked_claims, plain_tricks, poker_lite, race_to_n, secret_draft, three_marks, token_bazaar). Two (event_frontier, flood_watch) adopt authored motion (EFFANITUR-006/007); this ticket sweeps the remaining 12. The scheduler's settle hook (EFFANITUR-002) and the generic tone-keyed presentations (EFFANITUR-003) are the surfaces this ticket verifies and instruments.
2. Spec D7 / WB8: generic-presentation verification + an adoption row (`adopt` / `board-native mapping` / `generic-only` / `not applicable` with rationale) for each of the remaining 12 games; a dev-only settle assertion comparing post-animation DOM against the authoritative view, reporting missing coverage — diagnostics only, excluded from public builds.
3. Cross-artifact boundary under audit: the scheduler settle-hook API (EFFANITUR-002) into which the dev assertion registers, and the generic presentations (EFFANITUR-003) the sweep exercises. The assertion's DOM-vs-view comparison is a diagnostic, never an animation input.
4. FOUNDATIONS §7/§11/§12: the dev settle-assertion uses renderer state diffs strictly as a diagnostic (the §7-sanctioned use); it is excluded from public builds and from normal animation authority, keeping the §12 stop condition ("animation depends on guessed state diffs") clear.
5. No-leak (FOUNDATIONS §11 firewall): generic presentations across all 12 games read only the viewer-filtered stream; redacted effects animate generically. The sweep verifies no game's baseline motion adds a DOM/test-ID/storage leak surface.

## Architecture Check

1. One dev-only settle assertion plugged into the scheduler's existing settle hook is cleaner than per-game settle checks, and verifying the 12 generic-only games through the shared presentations (rather than per-game code) proves the zero-per-game-code baseline the design promises.
2. No backwards-compatibility shim: the assertion is a new diagnostic, not a wrapper around old behavior; it is build-flag-gated out of public bundles.
3. `engine-core` untouched; the assertion and sweep are `apps/web`-local (§3). No `game-stdlib` promotion.

## Verification Layers

1. All 12 non-adopter games render baseline motion from generic presentations and settle -> node sweep smoke over the 12 games (`smoke-catalog-sweep.mjs`) + `smoke:ui`.
2. Dev settle-assertion flags missing coverage (lingering ghost / unanimated transition) and is excluded from public builds -> node smoke (assertion fires on a seeded gap) + grep-proof (dev-flag guard).
3. Each catalog game has a recorded adoption row -> codebase grep-proof (14 rows: 2 `adopt` + 12 classified).
4. Generic motion adds no leak surface -> no-leak visibility test (`a11y-noleak.smoke.mjs` unchanged-or-stronger).

## What to Change

### 1. Dev settle-assertion

Add `apps/web/src/animation/settleAssertion.ts`: a dev-only diagnostic that, after each drain/flush/settle, compares the rendered DOM against the authoritative viewer-safe view and reports missing effect coverage. Register it through the scheduler's settle-hook API (EFFANITUR-002), guarded by the dev build flag so it is absent from public bundles.

### 2. Catalog sweep + adoption matrix

Verify the 12 non-adopter games render correct baseline motion through the shared generic presentations; record a one-row-per-game adoption matrix (`adopt`/`board-native mapping`/`generic-only`/`not applicable` + rationale; the 2 adopters carry `adopt`) in `apps/web/README.md`.

### 3. Sweep smoke

Add `apps/web/scripts/smoke-catalog-sweep.mjs` exercising baseline motion + settle across the catalog and asserting the dev assertion fires on a seeded coverage gap (wiring consolidated in EFFANITUR-009).

## Files to Touch

- `apps/web/src/animation/settleAssertion.ts` (new)
- `apps/web/scripts/smoke-catalog-sweep.mjs` (new)
- `apps/web/README.md` (modify; adoption matrix — shared with EFFANITUR-010, which `Deps` this ticket)

## Out of Scope

- The two authored adoptions (EFFANITUR-006/007).
- Any animation driven by the settle-assertion's diff (it is diagnostics-only — FOUNDATIONS §12).
- Public-build inclusion of the assertion (dev-only).
- The §10/§10A/§19 doc amendments and status flips (EFFANITUR-010).

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/scripts/smoke-catalog-sweep.mjs` — the 12 non-adopter games render baseline motion and settle; the dev assertion fires on a seeded coverage gap.
2. Grep-proof: the settle-assertion is dev-flag-guarded (absent from public builds); the adoption matrix carries 14 rows.
3. `npm --prefix apps/web run smoke:ui` and `build` green.

### Invariants

1. The settle-assertion uses state diffs as a diagnostic only and is excluded from public builds (§7/§11/§12).
2. Every catalog game has a recorded adoption row; generic motion adds no leak surface (§11).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-catalog-sweep.mjs` — catalog baseline-motion + settle + seeded-gap assertion.
2. `apps/web/scripts/smoke-ui.mjs` — extend for generic-presentation coverage across non-adopter boards.

### Commands

1. `node apps/web/scripts/smoke-catalog-sweep.mjs`
2. `npm --prefix apps/web run smoke:ui`
3. `npm --prefix apps/web run build`
