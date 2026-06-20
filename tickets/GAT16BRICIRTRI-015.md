# GAT16BRICIRTRI-015: E2E smoke, DOM/storage no-leak, and catalog-doc reconciliation

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (audit + e2e) — `apps/web/e2e/briar-circuit.smoke.mjs`, `apps/web/package.json`, `apps/web/README.md`, root `README.md`
**Deps**: 014

## Problem

Briar Circuit needs an end-to-end browser smoke proving pairwise/observer DOM no-leak, keyboard play, hotseat handoff privacy, the reduced-motion path, replay import/export, and moon/terminal explanation — plus the `smoke:e2e` wiring and the catalog-doc reconciliation that `scripts/check-catalog-docs.mjs` enforces. This ticket closes the `check-catalog-docs` red window opened when the WASM catalog const landed (GAT16BRICIRTRI-013).

## Assumption Reassessment (2026-06-20)

1. `apps/web/e2e/{plain-tricks,river-ledger}.smoke.mjs` are the e2e exemplars; `apps/web/package.json` `smoke:e2e` runs the per-game e2e list; `scripts/check-catalog-docs.mjs` keys off the wasm catalog const and checks the `apps/web/README.md` intro list, the root `README.md` "current official games are" list, and the `apps/web/README.md` `smoke:e2e` bullet. The `BriarCircuitBoard` renderer and WASM catalog entry exist after GAT16BRICIRTRI-013/014.
2. `specs/gate-16-briar-circuit-trick-taking.md` §4.4 (E2E/Catalog-documentation rows), §10.4 (the reassessment-added root `README.md` + package.json `smoke:e2e` surfaces), §7.5 (DOM/storage no-leak taxonomy), and Appendix D fix this content.
3. Cross-artifact boundary under audit: the e2e smoke asserts the same canaries as the native pairwise tests (009) at the DOM/storage/accessibility layer; the `ci/games.json` `e2e` field (set in 012) references `briar-circuit.smoke.mjs`, so this ticket closes the interim red e2e matrix step.
4. FOUNDATIONS §11 no-leak firewall at the browser layer is under audit: no hidden card/pass datum in text, ARIA, DOM keys, `data-*`, test IDs, CSS, local/session/IndexedDB storage, clipboard, or URLs; face-down nodes use count/position only.

## Architecture Check

1. A dedicated e2e smoke that re-runs the canary search at the DOM/storage layer (over trusting the native tests alone) proves the firewall survives serialization and rendering — the layer where a leak would actually reach a user.
2. No backwards-compatibility aliasing/shims — a new e2e file + additive `smoke:e2e` list entry + README catalog rows.
3. No Rust/legality change; this ticket is verification + catalog docs. This ticket ships test infrastructure (it doubles as the gate's browser-acceptance vehicle), not `None`.

## Verification Layers

1. Pairwise/observer DOM, keyboard play, hotseat handoff privacy, reduced motion, replay import/export, moon/terminal explanation -> `node apps/web/e2e/briar-circuit.smoke.mjs`.
2. No hidden datum in DOM/storage/clipboard/URL -> the e2e canary scan (§7.5).
3. Catalog parity across the three `check-catalog-docs` surfaces -> `node scripts/check-catalog-docs.mjs`.

## What to Change

### 1. `apps/web/e2e/briar-circuit.smoke.mjs`

The browser smoke: observer + four seat-private DOM/storage canary scans, keyboard card/pass play, the 2♣ opening, hotseat handoff privacy, reduced-motion path, replay import/export, and moon + terminal explanation assertions.

### 2. `smoke:e2e` wiring

Add `node e2e/briar-circuit.smoke.mjs` to the `apps/web/package.json` `smoke:e2e` script.

### 3. Catalog-doc reconciliation

Add Briar Circuit to the `apps/web/README.md` intro catalog list + `BriarCircuitBoard` Shell Surface renderer bullet + `smoke:e2e` bullet, and to the root `README.md` "current official games are" list.

## Files to Touch

- `apps/web/e2e/briar-circuit.smoke.mjs` (new)
- `apps/web/package.json` (modify — `smoke:e2e`)
- `apps/web/README.md` (modify)
- `README.md` (modify — root catalog list)

## Out of Scope

- Trailing game docs and the public-release checklist (GAT16BRICIRTRI-017).
- Benchmarks (GAT16BRICIRTRI-016).
- The final `Done`-flip and exit-evidence capstone (GAT16BRICIRTRI-018).

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/briar-circuit.smoke.mjs` — full e2e path including pairwise/observer no-leak.
2. `npm --prefix apps/web run smoke:e2e` — the briar-circuit smoke runs in the suite.
3. `node scripts/check-catalog-docs.mjs` — all three catalog surfaces reconciled (closes the red window).

### Invariants

1. No hidden card/pass datum reaches DOM, storage, clipboard, URL, ARIA, test IDs, or CSS (§11 no-leak).
2. Catalog README surfaces match the wasm catalog const (`check-catalog-docs` green).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/briar-circuit.smoke.mjs` — DOM/storage pairwise + observer no-leak, keyboard, hotseat, replay, moon/terminal.
2. `apps/web/package.json` — `smoke:e2e` list entry.
3. `apps/web/README.md` / `README.md` — catalog reconciliation (verified by `check-catalog-docs`).

### Commands

1. `node apps/web/e2e/briar-circuit.smoke.mjs`
2. `npm --prefix apps/web run smoke:e2e`
3. `node scripts/check-catalog-docs.mjs`
