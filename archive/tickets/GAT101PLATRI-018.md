# GAT101PLATRI-018: Browser e2e smoke, catalog README reconciliation, and gate-1 e2e step

**Status**: COMPLETE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation/test infra) — new `apps/web/e2e/plain-tricks.smoke.mjs`; modifies `apps/web/package.json` (`smoke:e2e` chain), `apps/web/README.md`, `README.md`, `.github/workflows/gate-1-game-smoke.yml` (e2e step). No Rust/engine behavior.
**Deps**: GAT101PLATRI-017, GAT101PLATRI-014

## Problem

The gate needs a browser e2e no-leak smoke for Plain Tricks, the catalog README surfaces reconciled (so `scripts/check-catalog-docs.mjs` goes green), and the gate-1 e2e step wired. This closes the `check-catalog-docs` red window opened when GAT101PLATRI-016 added `GAME_PLAIN_TRICKS`.

## Assumption Reassessment (2026-06-09)

1. `apps/web/package.json` `smoke:e2e` is a hardcoded `node e2e/*.smoke.mjs` `&&`-chain (ends at `poker-lite.smoke.mjs`); `apps/web/e2e/` holds per-game `*.smoke.mjs` files; `apps/web/README.md` has an intro catalog list, a Shell Surface renderer list, and a Smoke Layers `smoke:e2e` bullet; root `README.md` has a "current official games are" list. The renderer + WASM bridge exist (GAT101PLATRI-017/016).
2. Spec §4 (E2E + catalog docs/scripts) and §10 fix: append `plain-tricks.smoke.mjs` to the chain; update `apps/web/README.md` (intro/catalog, Shell Surface renderer list, Smoke Layers list) and root `README.md`; satisfy `scripts/check-catalog-docs.mjs`. The script is **parametric** (keyed off the `GAME_PLAIN_TRICKS` const) and needs no edit; it checks the two README catalog lists + the Smoke Layers bullet but NOT the Shell Surface renderer bullet (that addition is manual/process-enforced).
3. Shared boundary under audit: the catalog source-of-truth chain (`wasm-api` `GAME_*` const → `check-catalog-docs.mjs` → the three README surfaces) and the hardcoded `smoke:e2e` chain. This ticket is a §Deliverable-doubles-as-capstone (it ships e2e test infra).
4. FOUNDATIONS §11 (no hidden-info leak through DOM, accessibility names, test IDs, local storage, replay export, dev panel) and §7 (play-first UI) are under audit.
5. Enforcement surface: §11 no-leak firewall at the full browser surface. The e2e smoke must assert no unplayed-card identifier appears in DOM text, accessibility names, `data-testid`, local storage, replay export text, or dev-panel content — the end-to-end extension of the GAT101PLATRI-009/016 firewalls.

## Architecture Check

1. Co-landing the catalog README reconciliation with the web-smoke/gate-1 ticket (vs. a trailing docs ticket) closes the `check-catalog-docs` red window at the earliest green point — the established rule that avoids a multi-PR red-CI gap.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; e2e + docs only, no behavior in TypeScript.

## Verification Layers

1. Human-vs-bot full match; forced-follow UI; outcome explanation; replay export/import; reduced motion; a11y -> browser e2e (`plain-tricks.smoke.mjs` via `smoke:e2e`).
2. Observer no-leak (no opponent hand identities); seat-private own-hand view; dev-panel whitelist; stale diagnostic -> e2e no-leak DOM/storage/export/dev-panel assertions.
3. Catalog reconciliation green -> `node scripts/check-catalog-docs.mjs`.
4. gate-1 e2e step wired -> manual review of `gate-1-game-smoke.yml`.

## What to Change

### 1. `apps/web/e2e/plain-tricks.smoke.mjs`

Human-vs-bot full match; forced-follow state; observer no-leak; seat-private own-hand; outcome-explanation surface; public replay export/import; reduced motion; keyboard/a11y; stale-diagnostic; dev-panel whitelist. No-leak checks scan DOM text, accessibility names, `data-testid`, local storage, replay export text, and dev-panel content for unplayed-card identifiers.

### 2. `apps/web/package.json`

Append `&& node e2e/plain-tricks.smoke.mjs` to the `smoke:e2e` chain.

### 3. `apps/web/README.md` + root `README.md`

Add **Plain Tricks** to the intro/catalog list (web + root "current official games are"), add `PlainTricksBoard` to the Shell Surface renderer list (manual — not script-checked), and add `plain-tricks.smoke.mjs` / Plain Tricks to the Smoke Layers `smoke:e2e` bullet.

### 4. `.github/workflows/gate-1-game-smoke.yml`

Add the `plain-tricks.smoke.mjs` e2e step (or confirm it runs via `smoke:e2e`) alongside the existing hidden-info games.

## Files to Touch

- `apps/web/e2e/plain-tricks.smoke.mjs` (new)
- `apps/web/package.json` (modify)
- `apps/web/README.md` (modify)
- `README.md` (modify)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- Trailing game docs (MECHANICS/UI/PUBLIC-RELEASE-CHECKLIST/HOW-TO-PLAY) (GAT101PLATRI-019).
- Capstone status reconciliation (specs index / progress / atlas) (GAT101PLATRI-020).
- Any Rust behavior or the renderer itself (GAT101PLATRI-016/017).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` passes including `plain-tricks.smoke.mjs`.
2. `node scripts/check-catalog-docs.mjs` passes (red window closed).
3. e2e no-leak assertions find no unplayed-card identifier in DOM/accessibility/test-id/storage/replay-export/dev-panel.

### Invariants

1. No hidden information leaks through any browser-facing surface (FOUNDATIONS §11 no-leak firewall).
2. The catalog source-of-truth (`GAME_PLAIN_TRICKS`) and the three README surfaces stay in sync (`check-catalog-docs` green).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/plain-tricks.smoke.mjs` — full-match + no-leak browser smoke.
2. `apps/web/package.json` `smoke:e2e` chain — includes the new smoke.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `node scripts/check-catalog-docs.mjs && node scripts/check-doc-links.mjs`
3. The e2e + catalog checks are the correct full-browser boundary; native verification is GAT101PLATRI-014/015.

## Outcome

Completed 2026-06-09. Added `plain-tricks.smoke.mjs` with browser no-leak, viewer, dev-panel, replay export/import, reduced-motion, responsive, and human-vs-bot terminal coverage. Wired it into `smoke:e2e` and the Gate 1 workflow. Reconciled the root and web README catalog/smoke surfaces so the catalog-docs check reflects all 10 registered games.

Verification:

1. `npm --prefix apps/web run build`
2. `node apps/web/e2e/plain-tricks.smoke.mjs`
3. `npm --prefix apps/web run smoke:e2e`
4. `node scripts/check-catalog-docs.mjs`
5. `node scripts/check-doc-links.mjs`
