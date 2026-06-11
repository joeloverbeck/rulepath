# GAT12FLOWATCOO-018: Browser E2E smoke, a11y/no-leak, and catalog README reconciliation

**Status**: ACCEPTED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/e2e/flood-watch.smoke.mjs` (new); `apps/web/package.json` (modify — `smoke:e2e`); `apps/web/README.md` + root `README.md` (modify — catalog surfaces). No Rust behavior.
**Deps**: GAT12FLOWATCOO-017

## Problem

The cooperative browser flows need an E2E smoke that exercises the human multi-action budget, a visible role-power difference, forecast reveal, the environment-phase animation, win and loss flows, the bot-teammate turn, bot-vs-bot cooperative replay, replay step/export/import, reduced motion, and the DOM/storage/test-ID no-leak assertions. This ticket also reconciles the web-shell catalog README surfaces as an in-gate closeout — resolving the `check-catalog-docs` red window opened when GAT12FLOWATCOO-014 added the catalog const.

## Assumption Reassessment (2026-06-11)

1. `apps/web/e2e/` holds per-game smoke files with hyphen-slug naming (verified `masked-claims.smoke.mjs`) → `flood-watch.smoke.mjs`. `apps/web/package.json` `smoke:e2e` chains the per-game smoke files (verified). `scripts/check-catalog-docs.mjs` mechanically checks three surfaces (verified header): the `apps/web/README.md` intro catalog list, the root `README.md` "current official games are" list, and the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet; the `apps/web/README.md` Shell Surface renderer list is process-enforced (manual closeout), not mechanically checked.
2. The spec (§Deliverables "Browser smoke", §Documentation-updates "Reconcile the web-shell catalog surfaces"; reassessment finding M3) fixes the smoke coverage and the catalog closeout surfaces: the three mechanically-checked surfaces plus the Shell Surface renderer list as a manual closeout surface (M3: `check-catalog-docs.mjs` deliberately does not check it). The smoke must assert no undrawn-deck data in DOM/storage/logs/test IDs.
3. Cross-artifact boundary under audit: the catalog README surfaces are keyed off the `crates/wasm-api/src/lib.rs` `GAME_FLOOD_WATCH` const (added in GAT12FLOWATCOO-014) — `check-catalog-docs.mjs` runs continuously and shows red between that ticket and this one; this ticket closes the window by reconciling all surfaces. The E2E smoke consumes the rendered board (GAT12FLOWATCOO-017) and the redacted export (GAT12FLOWATCOO-009/014).
4. FOUNDATIONS §11 (hidden info does not leak through DOM/local storage/UI test IDs/replay exports; public UI is play-first, accessible) and §7 (semantic effects drive animation; reduced motion) motivate this ticket. This is a §Deliverable-doubles-as-capstone ticket — it ships new E2E test infrastructure while serving as the browser acceptance surface.
5. Enforcement surface: the browser no-leak firewall (§11) — the smoke's negative assertions are the authority that the undrawn deck order never reaches DOM/`data-testid`/storage/logs, including post-terminal. The catalog reconciliation makes `check-catalog-docs.mjs` green (closing the red window).

## Architecture Check

1. Co-landing the E2E smoke with the catalog README reconciliation closes the `check-catalog-docs` red window in the same diff that first wires `smoke:e2e`, exactly the in-gate closeout the spec (and `specs/README.md` §10) require — avoiding the aftermath-pass drift that bit an earlier gate.
2. No backwards-compatibility aliasing/shims; additive smoke file + additive catalog list entries.
3. `engine-core`/`game-stdlib` untouched; this is `apps/web` presentation + docs only.

## Verification Layers

1. Cooperative flows -> simulation/e2e run: `flood-watch.smoke.mjs` covers budget, role-power difference, forecast reveal, environment animation, win + loss, bot-teammate turn, bot-vs-bot replay, replay step/export/import, reduced motion.
2. Browser no-leak -> no-leak visibility test: DOM/storage/logs/test-IDs carry no undrawn-deck order/identities, including post-terminal.
3. Catalog reconciliation -> `node scripts/check-catalog-docs.mjs` passes (the three mechanically-checked surfaces name `flood_watch`/`Flood Watch`); the Shell Surface renderer list updated manually.
4. a11y -> manual review / a11y checklist update per the spec.

## What to Change

### 1. E2E smoke

Author `apps/web/e2e/flood-watch.smoke.mjs` covering every flow above plus the DOM/storage/test-ID no-leak assertions. Add `node e2e/flood-watch.smoke.mjs` to the `smoke:e2e` script in `apps/web/package.json`.

### 2. Catalog README reconciliation

Add `flood_watch`/`Flood Watch` to: the `apps/web/README.md` intro catalog list, the root `README.md` "current official games are" list, the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet (the three mechanically-checked surfaces), and the `apps/web/README.md` Shell Surface renderer list (manual closeout per M3). Update the a11y/no-leak checklist.

## Files to Touch

- `apps/web/e2e/flood-watch.smoke.mjs` (new)
- `apps/web/package.json` (modify — `smoke:e2e`)
- `apps/web/README.md` (modify — intro list, smoke:e2e bullet, Shell Surface renderer list)
- `README.md` (modify — "current official games are" list)

## Out of Scope

- The React board + shell wiring (GAT12FLOWATCOO-017).
- `specs/README.md` index `Done`-flip, spec Status flip, `progress.md`, and the closeout admission docs (GAT12FLOWATCOO-019).
- Any Rust/WASM change — this ticket exercises the existing bridge.

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/flood-watch.smoke.mjs` (via `npm --prefix apps/web run smoke:e2e`) covers all listed cooperative flows.
2. The browser no-leak assertions confirm no undrawn-deck data in DOM/storage/logs/test IDs, including post-terminal.
3. `node scripts/check-catalog-docs.mjs` passes (red window closed).

### Invariants

1. No undrawn-deck order reaches any browser surface at any point, including post-terminal.
2. The three mechanically-checked catalog surfaces name `flood_watch`/`Flood Watch`; the Shell Surface renderer list is reconciled manually.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/flood-watch.smoke.mjs` — full cooperative E2E + DOM/storage no-leak assertions (new test infrastructure; this ticket doubles as the browser acceptance surface).
2. `apps/web/package.json` `smoke:e2e` — `flood-watch.smoke.mjs` registered.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `node scripts/check-catalog-docs.mjs && npm --prefix apps/web run build`
3. The native pipeline (`cargo test`, `simulate`, `replay-check`) is covered by GAT12FLOWATCOO-011/015; the E2E smoke + catalog check are the correct boundary for the browser acceptance diff.

## Outcome

Accepted on 2026-06-11.

Added `apps/web/e2e/flood-watch.smoke.mjs` and registered it in `smoke:e2e`. The smoke serves the built browser bundle, drives Flood Watch through hotseat, human-vs-bot, and bot-vs-bot flows, verifies forecast reveal, role labels, multi-action budget spending, environment effects, shared win/loss terminals, public replay export/import, reduced motion, responsive layout, and DOM/storage/test-ID/console no-leak constraints for deck-order and internal-state terms.

Reconciled the web/root catalog documentation surfaces for `flood_watch` / Flood Watch, updated the Shell Surface renderer list and Smoke Layers text, and extended the no-leak/a11y checklist. Updated `rules-display.smoke.mjs` to include Flood Watch's How to Play trigger and refreshed the generated rules manifest hash.

Verification passed:

- `node apps/web/e2e/flood-watch.smoke.mjs`
- `node apps/web/e2e/rules-display.smoke.mjs`
- `node scripts/check-catalog-docs.mjs`
- `npm --prefix apps/web run smoke:e2e`
- `npm --prefix apps/web run build`
