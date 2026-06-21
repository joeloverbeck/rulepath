# GAT17VOWTIDOHHEL-019: E2E smoke, accessibility/no-leak, catalog README reconciliation, CI wiring

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (audit + e2e) — new `apps/web/e2e/vow-tide.smoke.mjs`; modifies `ci/games.json`, `apps/web/README.md`, `README.md`, `.github/workflows/gate-1-game-smoke.yml`
**Deps**: 017, 018

## Problem

Add the browser e2e smoke (3- and 7-seat setup, legal-only bid/card controls, dealer hook, keyboard, hotseat handoff, observer/private no-leak, reduced motion, replay, outcome), reconcile the three catalog README surfaces `check-catalog-docs` enforces, and wire `ci/games.json` + the gate-1 CI step. This is the web-acceptance capstone for the gate.

## Assumption Reassessment (2026-06-21)

1. `apps/web/e2e/` holds per-game `*.smoke.mjs` (e.g. `briar-circuit.smoke.mjs`); `ci/games.json` entries are `{id, sim_flags, e2e}` (briar uses `--seat-count 4`); `.github/workflows/gate-1-game-smoke.yml` runs the smoke layers and `scripts/check-catalog-docs.mjs`.
2. `scripts/check-catalog-docs.mjs` keys off the `crates/wasm-api/src/lib.rs` catalog const (added in 017) and checks the `apps/web/README.md` intro catalog list, the root `README.md` "current official games are" list, and the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet — these go red from 017 until this ticket reconciles them.
3. Cross-artifact boundary under audit: the e2e smoke exercises the 017 WASM bridge + 018 renderer end-to-end; the catalog README surfaces are the `check-catalog-docs` contract; `ci/games.json` drives the CI sim + e2e.
4. FOUNDATIONS §11/§7 under audit: the browser no-leak/a11y smoke proves no hidden DOM/storage/log data and play-first accessibility; this ticket adds new test infra (deliverable-doubles-as-capstone for web acceptance).

## Architecture Check

1. Co-landing the catalog README reconciliation with the e2e/CI wiring closes the `check-catalog-docs` red window opened at 017, the official-game-pattern placement.
2. No shims; additive smoke + CI step.
3. `engine-core`/`game-stdlib` untouched; no legality in the smoke harness.

## Verification Layers

1. 3- and 7-seat play, legal-only controls, hook, hotseat, observer no-leak, reduced motion, replay, outcome → `npm --prefix apps/web run smoke:e2e`.
2. No hidden DOM/storage/log datum (canary) across observer/hotseat/replay → e2e no-leak assertions.
3. Catalog README surfaces consistent → `node scripts/check-catalog-docs.mjs`.
4. CI runs the vow_tide smoke → `ci/games.json` + workflow inspection.

## What to Change

### 1. E2E smoke

`apps/web/e2e/vow-tide.smoke.mjs`: 3- and 7-seat setup, legal-only bid/card interaction, dealer hook, keyboard, hotseat handoff (prior private hand removed), observer/private no-leak canaries, reduced motion, replay, outcome explanation; the 7-seat smoke cycles the viewer selector through all seven seats.

### 2. Catalog README + CI

Add the `vow_tide` row to `ci/games.json` (`"--seat-count 4 --action-cap 2048"`, `"vow-tide.smoke.mjs"`); add Vow Tide to the `apps/web/README.md` intro list + Shell Surface renderer list + Smoke Layers `smoke:e2e` bullet and the root `README.md` catalog list; wire the gate-1 workflow step.

## Files to Touch

- `apps/web/e2e/vow-tide.smoke.mjs` (new)
- `ci/games.json` (modify)
- `apps/web/README.md` (modify)
- `README.md` (modify)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- Native benchmarks (020); trailing docs (021); the spec `Done`-flip (022).
- Any game-logic change.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` — vow-tide smoke passes (3- and 7-seat, no-leak, a11y).
2. `node scripts/check-catalog-docs.mjs` — three README surfaces consistent.
3. `node scripts/check-ci-games.mjs` — `ci/games.json` entry valid.

### Invariants

1. No hidden hand/stock datum appears in DOM/storage/logs across any browser viewer/mode.
2. The catalog README surfaces name Vow Tide; the `check-catalog-docs` red window is closed.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/vow-tide.smoke.mjs` — full browser acceptance + no-leak/a11y.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `node scripts/check-catalog-docs.mjs && node scripts/check-ci-games.mjs`
3. Narrower command rationale: the e2e smoke + catalog check are the browser-acceptance boundary; native evidence is covered by the per-crate suites.

## Outcome

Completed on 2026-06-21.

- Added `apps/web/e2e/vow-tide.smoke.mjs` covering 3-seat setup, 7-seat setup, seven-seat viewer selection, observer/private no-leak, legal bid/card controls, keyboard activation, hotseat handoff, dealer-hook bid exclusion, public replay import/export, reduced motion, and responsive layout.
- Wired Vow Tide into `ci/games.json`, `apps/web/package.json` `smoke:e2e`, root/web catalog README surfaces, and the gate-1 workflow comment describing the manifest-driven per-game e2e lane.
- Updated the cross-cutting rules-display smoke's explicit catalog list for Vow Tide.
- Fixed the browser acceptance bugs exposed by the new seven-seat smoke: Vow WASM views now project active seat labels for the selected seat count, and the shared viewer-id guard permits `seat_6`.

Verification:

- `cargo fmt --all --check`
- `npm --prefix apps/web run build`
- `node apps/web/e2e/rules-display.smoke.mjs`
- `node apps/web/e2e/vow-tide.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`
- `cargo test -p wasm-api`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-ci-games.mjs`
