# GAT20STACROSTA-017: Web e2e smoke, CI registration, and catalog READMEs

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (audit + e2e) — `apps/web/e2e/starbridge-crossing.smoke.mjs`, `apps/web/package.json`, `ci/games.json`, `apps/web/README.md`, root `README.md`
**Deps**: GAT20STACROSTA-014, GAT20STACROSTA-015

## Problem

The 121-space board needs an end-to-end browser smoke (render, legal previews, jump-chain path building, keyboard, replay, reduced motion, no-leak), wired into `smoke:e2e` and `ci/games.json`. This ticket also lands the three catalog README surfaces that `check-catalog-docs` requires, closing the red window opened when the WASM const landed (014).

## Assumption Reassessment (2026-06-27)

1. `smoke:e2e` is a hardcoded chain of `node apps/web/e2e/<game>.smoke.mjs` (no `--game` filter) — confirmed `apps/web/package.json:18`; Starbridge adds `apps/web/e2e/starbridge-crossing.smoke.mjs` appended to that chain (the reassessed spec already fixed the acceptance command).
2. `ci/games.json` entries are `{id, sim_flags, e2e}` (confirmed); add `{ "id": "starbridge_crossing", "sim_flags": "--seat-count 6 --action-cap <cap>", "e2e": "starbridge-crossing.smoke.mjs" }` — one representative seat config per the reassessed §10.
3. `check-catalog-docs.mjs` keys off the wasm catalog const and checks three surfaces: `apps/web/README.md` intro catalog list, root `README.md` "current official games are" list, and the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet (confirmed in the script header). All three land here.
4. §7 (public UI) / §11 (no-leak) motivate this ticket: the e2e smoke proves the board renders and animates without leaking rule decisions into TypeScript (perfect info — nothing private), and the catalog surfaces keep the web shell's game list accurate.

## Architecture Check

1. Co-landing the e2e file, `ci/games.json` `e2e` reference, `smoke:e2e` wiring, and the catalog READMEs in one ticket resolves the `check-catalog-docs` red window atomically and avoids a dangling `e2e` reference.
2. No backwards-compatibility shims.
3. No legality in TypeScript (§2); the smoke asserts Rust-driven behavior; deliverable-doubles-as-capstone (ships new e2e test infra).

## Verification Layers

1. e2e smoke -> `npm --prefix apps/web run smoke:e2e` (the new Starbridge smoke runs in the chain).
2. CI registry -> `node scripts/check-ci-games.mjs` (entry well-formed; `e2e` file exists).
3. Catalog docs -> `node scripts/check-catalog-docs.mjs` (red window closed; all three surfaces name Starbridge Crossing).
4. No-leak DOM (§11) -> the e2e a11y/no-leak assertions (no hidden fact in DOM/test-ids).

## What to Change

### 1. Author `apps/web/e2e/starbridge-crossing.smoke.mjs`

121-space render, legal-preview update, jump-chain keyboard path building, replay viewer, reduced motion, responsive layout, a11y/no-leak assertions.

### 2. Wire `smoke:e2e` + `ci/games.json`

Append the smoke file to `apps/web/package.json`'s `smoke:e2e`; add the `ci/games.json` entry.

### 3. Catalog README surfaces

Add Starbridge Crossing to the `apps/web/README.md` intro catalog list + Smoke Layers `smoke:e2e` bullet + Shell Surface renderer list, and the root `README.md` "current official games are" list.

## Files to Touch

- `apps/web/e2e/starbridge-crossing.smoke.mjs` (new)
- `apps/web/package.json` (modify)
- `ci/games.json` (modify)
- `apps/web/README.md` (modify)
- `README.md` (modify)

## Out of Scope

- Renderer component logic — GAT20STACROSTA-015.
- The `specs/README.md` Done-flip — GAT20STACROSTA-020.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e`
2. `node scripts/check-ci-games.mjs && node scripts/check-catalog-docs.mjs`
3. `npm --prefix apps/web run build`

### Invariants

1. The catalog README surfaces name Starbridge Crossing; `check-catalog-docs` is green (red window closed).
2. The e2e smoke exercises the board without TypeScript deciding legality (§2/§11).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/starbridge-crossing.smoke.mjs` — the e2e smoke above.
2. `ci/games.json` + `apps/web/package.json` — registration (modified).

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `node scripts/check-ci-games.mjs && node scripts/check-catalog-docs.mjs`
3. The e2e run + catalog/CI checks are the correct boundary; this ticket is the gate-1 web acceptance surface.
