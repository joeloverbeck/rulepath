# GAT19MELLEDFIV-021: Web e2e smoke, ci/games.json, and catalog README reconciliation

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (audit + e2e) — `apps/web/e2e/meldfall-ledger.smoke.mjs`, `ci/games.json`, `apps/web/package.json`; catalog README docs
**Deps**: GAT19MELLEDFIV-020

## Problem

Gate 19 needs the browser e2e smoke (setup for 2/4/6, large-hand/tableau render, keyboard-only action builder, no-drag alternative, stock draw, discard-pile pickup, meld, lay-off, discard, replay import/export, and no hidden card text in public-observer DOM/a11y), wired into the `smoke:e2e` chain. It also adds the `ci/games.json` entry and reconciles the catalog README surfaces. This ticket closes the `check-catalog-docs` red window opened at GAT19MELLEDFIV-019; it ships new test infrastructure (puppeteer harness), so it doubles as the gate's browser-smoke capstone.

## Assumption Reassessment (2026-06-25)

1. `apps/web/e2e/blackglass-pact.smoke.mjs` is a puppeteer harness exemplar; `smoke:e2e` in `apps/web/package.json` is a hardcoded `node e2e/<game>.smoke.mjs` chain (no `--game` flag — confirmed during reassessment). `ci/games.json` entries are `{id, sim_flags, e2e}` and `check-ci-games.mjs` requires the named `e2e` file to exist in `apps/web/e2e/`. The board + WASM exist from GAT19MELLEDFIV-019/020.
2. Spec §4.3 (Web catalog/Web docs rows), §7.1 (smoke command), and §6 (larger hand/tableau exit criterion) define the smoke + reconciliation; `check-catalog-docs.mjs` checks the `apps/web/README.md` intro list, the root `README.md` "current official games are" list, and the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet against the wasm catalog const.
3. Cross-artifact: `check-catalog-docs` keys off the wasm catalog const (added GAT19MELLEDFIV-019); this ticket lands the three README surfaces that close its red window, plus the `ci/games.json` entry whose `e2e` field must name the smoke file created here (create-then-reference ordering).
4. FOUNDATIONS §11 no-leak: the e2e smoke asserts no hidden card text / stock order in public-observer DOM, a11y labels, `data-testid`, storage, or console logs.
5. FOUNDATIONS §7: the smoke proves a play-first, keyboard-operable large surface (no drag-only), not a debug-dominated shell.

## Architecture Check

1. Co-landing the e2e smoke, `ci/games.json`, and the catalog README lists in one ticket closes the `check-catalog-docs` red window atomically and gives a single browser-acceptance diff.
2. No backwards-compatibility shims.
3. `engine-core` untouched; this is e2e/CI infrastructure + docs, deciding no legality.

## Verification Layers

1. Browser e2e covers setup/draw/pickup/meld/lay-off/discard/replay + no-leak -> `node apps/web/e2e/meldfall-ledger.smoke.mjs` (in `smoke:e2e`).
2. Catalog README surfaces match the wasm catalog const -> `node scripts/check-catalog-docs.mjs`.
3. `ci/games.json` entry is shape-valid and names an existing e2e file -> `node scripts/check-ci-games.mjs`.

## What to Change

### 1. e2e smoke + `smoke:e2e` chain

`apps/web/e2e/meldfall-ledger.smoke.mjs` (puppeteer): setup 2/4/6, large hand/tableau render, keyboard-only action builder, no-drag alternative, stock draw, multi-card discard pickup, meld, lay-off, discard, replay import/export, public-observer no-leak (DOM/a11y/test-id/storage/console). Add `node e2e/meldfall-ledger.smoke.mjs` to the `smoke:e2e` chain in `apps/web/package.json`.

### 2. `ci/games.json`

Add `{ "id": "meldfall_ledger", "sim_flags": "--seat-count 4 --action-cap 4096", "e2e": "meldfall-ledger.smoke.mjs" }`.

### 3. Catalog README reconciliation

`apps/web/README.md` intro catalog list + Shell Surface renderer list + Smoke Layers `smoke:e2e` list; root `README.md` "current official games are" list.

## Files to Touch

- `apps/web/e2e/meldfall-ledger.smoke.mjs` (new)
- `apps/web/package.json` (modify — `smoke:e2e` chain)
- `ci/games.json` (modify)
- `apps/web/README.md` (modify)
- `README.md` (modify)

## Out of Scope

- Trailing game docs + the forward-v1 governance receipt + the `specs/README.md` `Done`-flip (GAT19MELLEDFIV-022/023).
- Any production rule logic (exercised here, not modified).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` runs `meldfall-ledger.smoke.mjs` green (setup/draw/pickup/meld/lay-off/discard/replay + no-leak).
2. `node scripts/check-catalog-docs.mjs` and `node scripts/check-ci-games.mjs` pass.
3. `npm --prefix apps/web run build` succeeds.

### Invariants

1. No hidden card text / stock order in public-observer DOM, a11y, `data-testid`, storage, or console (FOUNDATIONS §11).
2. The large surface is keyboard-operable with a non-drag path (FOUNDATIONS §7).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/meldfall-ledger.smoke.mjs` — full browser play-path + no-leak/a11y smoke.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `node scripts/check-catalog-docs.mjs && node scripts/check-ci-games.mjs`
3. `npm --prefix apps/web run build`

## Outcome

Completed: 2026-06-26

Implemented the browser e2e and catalog reconciliation capstone:

- Added `apps/web/e2e/meldfall-ledger.smoke.mjs` covering 2/4/6 setup, large private-hand render, public observer redaction, keyboard-focusable non-drag controls, discard-pile pickup, meld, lay-off, finish/discard, replay export/import, responsive layout, storage/console no-leak checks, and a separate stock-draw path.
- Added the smoke to `apps/web/package.json` `smoke:e2e` and registered `meldfall_ledger` in `ci/games.json`.
- Reconciled `apps/web/README.md`, root `README.md`, and `rules-display.smoke.mjs` catalog expectations for the 19th browser catalog game.
- Fixed the Rust Meldfall legal action tree to expose discard-pile draw choices and table-phase meld/lay-off/go-out/finish choices from Rust, with pending discard-pickup filtering so the browser can remain presentation-only.

Deviations:

- The ticket was scoped as e2e/docs, but the browser smoke exposed an incomplete Rust action tree from the prior WASM bridge work. A narrow `games/meldfall_ledger` fix was required to make the UI prove Rust-owned meld and lay-off controls without TypeScript legality.
- Direct `node apps/web/e2e/meldfall-ledger.smoke.mjs` first failed inside the sandbox with `listen EPERM` on `127.0.0.1`; it passed after rerun through the approved localhost browser-smoke escalation.

Verification:

- `cargo fmt --all --check` — passed.
- `cargo test -p meldfall_ledger` — passed.
- `cargo test -p wasm-api` — passed.
- `node scripts/check-catalog-docs.mjs` — passed; 19 games reflected.
- `node scripts/check-ci-games.mjs` — passed; 19 games in sync.
- `npm --prefix apps/web run build` — passed; Vite emitted the existing >500 kB chunk-size warning.
- `node apps/web/e2e/meldfall-ledger.smoke.mjs` — passed after approved localhost rerun.
- `npm --prefix apps/web run smoke:e2e` — passed, including `meldfall_ledger setup actions replay noleak responsive`.
- `node scripts/check-doc-links.mjs` — passed; checked 31 markdown files.
