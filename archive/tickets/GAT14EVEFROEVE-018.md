# GAT14EVEFROEVE-018: Browser E2E smoke, a11y/no-leak, and catalog reconciliation

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/e2e/event-frontier.smoke.mjs` (new); `apps/web/package.json` (modify); `apps/web/README.md` (modify); `README.md` (modify); `.github/workflows/gate-1-game-smoke.yml` (modify)
**Deps**: GAT14EVEFROEVE-016, GAT14EVEFROEVE-017

## Problem

The gate needs a browser E2E smoke covering both factions' choice menus, an event resolution, a full op with multi-site selection, a pass, an edict activating and expiring, a Reckoning breakdown, each victory type reachable in scripted fixtures (Charter instant, Freeholder instant, final fallback), bot turns for both factions, bot-vs-bot replay, replay step/export/import with no deck-order leak, and reduced motion — plus a11y/no-leak checks. It also reconciles the web-shell catalog surfaces as the in-gate closeout (per `specs/README.md` §10) and resolves the `check-catalog-docs` / `check-outcome-explanations` CI red window opened when the wasm catalog const landed (ticket 014). This ticket ships the E2E harness, so it doubles as the gate's browser-acceptance capstone.

## Assumption Reassessment (2026-06-12)

1. The board and bridge this exercises exist: verified ticket 017's `EventFrontierBoard.tsx` + shared-surface wiring and ticket 014's wasm-api registration; the sibling E2E harness is `apps/web/e2e/frontier-control.smoke.mjs`, chained in `apps/web/package.json`'s `smoke:e2e` script.
2. The catalog-docs contract is current: verified `scripts/check-catalog-docs.mjs` keys off the `crates/wasm-api/src/lib.rs` `GAME_*` const and checks the `apps/web/README.md` intro catalog list, the root `README.md` "current official games are" list, and the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet; `check-outcome-explanations.mjs` reads `UI.md`/`RULES.md` (ticket 016) + the templates (ticket 017). These surfaces co-land here to close the red window.
3. Cross-artifact boundary under audit: this ticket modifies `gate-1-game-smoke.yml` (shared with ticket 015's native lanes — mutually independent; coordinate the mechanical merge) and the two catalog READMEs; the `check-catalog-docs` red window opened at ticket 014 closes here. The no-leak E2E proves undrawn deck order never reaches the DOM, storage, test IDs, or replay export.
4. FOUNDATIONS §11 (no-leak firewall; play-first accessible UI; semantic-effect animation) and §7 (legal-only UI) motivate this ticket. Restated before trusting the spec: the E2E asserts legal-only menus, effect-driven animation, reduced motion, accessibility, and that no surface leaks undrawn deck order.
5. No-leak enforcement surface (§11): this is the DOM/storage/export firewall proof. Confirm the replay export/import E2E asserts no deck-order leak and the a11y/no-leak smoke (`a11y-noleak`) covers `event_frontier`. Presentation-only — no behavior authority in TypeScript.

## Architecture Check

1. Co-landing the catalog README reconciliation with the E2E/gate-1 ticket (rather than a trailing docs ticket) closes the `check-catalog-docs` red window in the same PR that wires `smoke:e2e`, per the established Gate 9 lesson.
2. No backwards-compatibility aliasing/shims — additive E2E harness, additive catalog entries, additive CI step.
3. `engine-core` stays noun-free; no `game-stdlib` promotion; the E2E is presentation-only.

## Verification Layers

1. E2E flows -> `node apps/web/e2e/event-frontier.smoke.mjs` covers both factions' menus, event/op/pass, edict activate+expire, Reckoning, each victory type, bot turns, bot-vs-bot replay, replay step/export/import, reduced motion.
2. No-leak DOM/export (§11) -> the a11y/no-leak smoke and the replay export/import step assert no undrawn-deck-order leak to DOM/storage/test-IDs/export.
3. Catalog reconciliation -> `node scripts/check-catalog-docs.mjs` and `node scripts/check-outcome-explanations.mjs` pass (red window closed).
4. CI lane -> `gate-1-game-smoke.yml` registers the `event_frontier` E2E step.

## What to Change

### 1. E2E smoke harness

Author `apps/web/e2e/event-frontier.smoke.mjs` covering the flows listed in the Problem; add it to the `smoke:e2e` chain in `apps/web/package.json`. Ensure the `a11y-noleak` smoke covers `event_frontier`.

### 2. Catalog reconciliation

Add `event_frontier` / `Event Frontier` to the `apps/web/README.md` intro catalog list, the Shell Surface renderer list, and the Smoke Layers `smoke:e2e` bullet; add it to the root `README.md` "current official games are" list. Register the E2E step in `.github/workflows/gate-1-game-smoke.yml`.

## Files to Touch

- `apps/web/e2e/event-frontier.smoke.mjs` (new)
- `apps/web/package.json` (modify) — add to `smoke:e2e` chain
- `apps/web/README.md` (modify) — intro catalog, Shell Surface, Smoke Layers
- `README.md` (modify) — official-games list
- `.github/workflows/gate-1-game-smoke.yml` (modify) — E2E step (shared with ticket 015)

## Out of Scope

- The board component itself (ticket 017) and WASM registration (ticket 014).
- The capstone status flip (`specs/README.md` / spec Status `Done`, `progress.md`, ADMISSION/PUBLIC-RELEASE-CHECKLIST docs) — ticket 019.
- Native tool/CI lanes (ticket 015), though they share `gate-1-game-smoke.yml`.

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/event-frontier.smoke.mjs` passes all listed flows.
2. `node scripts/check-catalog-docs.mjs` and `node scripts/check-outcome-explanations.mjs` pass (red window closed).
3. The a11y/no-leak smoke confirms no undrawn-deck-order leak to DOM/storage/test-IDs/export.

### Invariants

1. No browser surface (DOM, storage, test IDs, replay export) contains undrawn deck order.
2. The web-shell catalog surfaces name `event_frontier`; `check-catalog-docs` is green.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/event-frontier.smoke.mjs` — the gate's browser-acceptance harness (this ticket doubles as the browser capstone).

### Commands

1. `npm --prefix apps/web run build && node apps/web/e2e/event-frontier.smoke.mjs`
2. `node scripts/check-catalog-docs.mjs && node scripts/check-outcome-explanations.mjs`
3. The E2E run plus the catalog/outcome checks is the correct boundary — it exercises the whole browser surface and closes the CI red window in one PR.

## Outcome

Implemented the Event Frontier browser acceptance capstone:

1. Added `apps/web/e2e/event-frontier.smoke.mjs` covering both faction menus, event resolution, a multi-site operation, pass, edict activation/expiry through Reckoning, bot-vs-bot terminal paths for Charter instant, Freeholder instant, and final fallback, public replay export/import/step, reduced motion, and no-leak assertions.
2. Extended `a11y-noleak.smoke.mjs` with Event Frontier board accessibility, reduced-motion, and no-leak coverage.
3. Registered the Event Frontier E2E in `smoke:e2e` and the Gate 1 CI browser lane, then reconciled the root and web README catalog surfaces.
4. Fixed two browser-facing Rust/WASM boundary issues found by the E2E: the bridge now completes Event Frontier automated Reckoning phases after human/bot actions, and web action paths percent-encode path segments so Rust action segments containing `>` are submitted without TypeScript parsing game rules.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo build -p wasm-api`
3. `npm --prefix apps/web run build`
4. `node apps/web/e2e/event-frontier.smoke.mjs`
5. `node apps/web/e2e/a11y-noleak.smoke.mjs`
6. `npm --prefix apps/web run smoke:wasm`
7. `npm --prefix apps/web run smoke:ui`
8. `npm --prefix apps/web run smoke:effects`
9. `node scripts/check-catalog-docs.mjs`
10. `node scripts/check-outcome-explanations.mjs`
11. `node scripts/check-doc-links.mjs`
