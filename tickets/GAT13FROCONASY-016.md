# GAT13FROCONASY-016: Browser E2E smoke, a11y/no-leak, and catalog reconciliation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/e2e/frontier-control.smoke.mjs` (new); `apps/web/package.json` (modify); `apps/web/README.md` (modify); `README.md` (modify); `.github/workflows/gate-1-game-smoke.yml` (modify)
**Deps**: GAT13FROCONASY-014, GAT13FROCONASY-015

## Problem

The gate needs a browser E2E smoke covering both factions' action panels (disjoint sets visible), a march/patrol move, a clash animation, stake placement + supply-connectivity display, a round-scoring effect, a full game to each faction's outcome surface, each faction's bot turn, bot-vs-bot replay, replay step/export/import, and reduced motion. It also reconciles the web-shell catalog surfaces as an in-gate closeout (per `specs/README.md` §10) and resolves the `check-catalog-docs` / `check-outcome-explanations` CI red window opened when the wasm catalog const landed. This ticket ships the E2E harness, so it doubles as the gate's browser-acceptance capstone.

## Assumption Reassessment (2026-06-11)

1. `apps/web/e2e/flood-watch.smoke.mjs` is the exemplar; `apps/web/package.json` `smoke:e2e` chains per-game `node e2e/<game>.smoke.mjs` files (verified, ends `… && node e2e/flood-watch.smoke.mjs`). `scripts/check-catalog-docs.mjs` mechanically checks three surfaces — the `apps/web/README.md` intro catalog list, the root `README.md` "current official games are" list, and the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet (Shell Surface renderer list is process-enforced, not mechanical) — keyed off the wasm `GAME_*` const (verified). `scripts/check-outcome-explanations.mjs` validates the wasm catalog + `UI.md` outcome section + `RULES.md` rule IDs + `client.ts` mirrors + `outcomeExplanationTemplates.ts` keys.
2. Spec §Browser smoke lists the required flows; §Documentation reconciles the catalog surfaces in-gate; §Exit "effect logs stay readable" asserts the log renders readable entries for a clash, a round scoring, and both terminal outcomes.
3. Cross-artifact boundary under audit: the E2E exercises the board (GAT13FROCONASY-015) + wasm bridge (GAT13FROCONASY-012); the catalog reconciliation closes the `check-catalog-docs` red window opened at GAT13FROCONASY-012 (expected mid-gate red CI between those tickets); `check-outcome-explanations` needs the `UI.md` outcome section (GAT13FROCONASY-014), the `RULES.md` rule IDs (GAT13FROCONASY-001), and the TS templates/mirrors (GAT13FROCONASY-015). The gate-1 E2E + web-build lanes co-land here (the native lanes are in GAT13FROCONASY-013 — both modify `gate-1-game-smoke.yml`, a flagged shared file).
4. FOUNDATIONS §7 (public UI play-first) and §11 are under audit: the E2E is play-first, accessible, reduced-motion-aware, and the a11y/no-leak smoke asserts the DOM/payloads carry no information a viewer should not see.
5. §11 no-leak firewall enforcement surface: the a11y-noleak smoke is the no-leak proof surface; for a perfect-information game every viewer sees everything, so the assertion confirms the public projection only (no private state, no `HIDDEN_INFO_GAMES` path) reaches the DOM/exports.

## Architecture Check

1. Co-landing the catalog reconciliation with the E2E/web-smoke ticket (vs a trailing docs pass) closes the `check-catalog-docs` red window inside the gate — the documented fix for the Gate 9 aftermath miss; the E2E-as-capstone avoids a synthetic acceptance-only ticket.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; presentation/test infra + docs only.

## Verification Layers

1. Browser flows + readable effect log (§7) -> simulation/CLI run (`node apps/web/e2e/frontier-control.smoke.mjs` covers the per-faction flows incl. clash/round-scoring/both terminals).
2. a11y / no-leak (§11) -> no-leak visibility test (a11y-noleak smoke; DOM/payloads carry only the public projection).
3. Catalog reconciliation -> `node scripts/check-catalog-docs.mjs` (intro + root-README + smoke:e2e surfaces name `frontier_control` / `Frontier Control`).
4. Outcome explanations -> `node scripts/check-outcome-explanations.mjs` (templates + rule-ID mirrors registered).

## What to Change

### 1. E2E smoke

Author `apps/web/e2e/frontier-control.smoke.mjs` covering the spec's per-faction flows + reduced motion; add it to the `smoke:e2e` chain in `apps/web/package.json`.

### 2. Catalog reconciliation

Update the `apps/web/README.md` intro catalog list + Smoke Layers `smoke:e2e` bullet + Shell Surface renderer list, and the root `README.md` "current official games are" list, to name `frontier_control` / `Frontier Control`.

### 3. gate-1 CI E2E + web-build lanes

Add the web-build + `frontier-control` E2E registration to `.github/workflows/gate-1-game-smoke.yml`.

## Files to Touch

- `apps/web/e2e/frontier-control.smoke.mjs` (new)
- `apps/web/package.json` (modify)
- `apps/web/README.md` (modify)
- `README.md` (modify)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- Admission + public-release docs, spec Status / index `Done` flip, progress.md (GAT13FROCONASY-017).
- Any Rust/engine behavior change.

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/frontier-control.smoke.mjs` covers the per-faction flows and asserts a readable effect log for a clash, a round scoring, and both terminal outcomes.
2. `node scripts/check-catalog-docs.mjs` and `node scripts/check-outcome-explanations.mjs` pass.
3. `npm --prefix apps/web run smoke:e2e` (full chain incl. frontier-control) passes.

### Invariants

1. Public UI is play-first, accessible, reduced-motion-aware; the no-leak smoke confirms only the public projection reaches the DOM (§7/§11).
2. The catalog reconciliation closes the `check-catalog-docs` red window in-gate (not a later aftermath pass).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/frontier-control.smoke.mjs` — the per-faction E2E flows + a11y/no-leak + reduced motion.

### Commands

1. `node apps/web/e2e/frontier-control.smoke.mjs`
2. `node scripts/check-catalog-docs.mjs && node scripts/check-outcome-explanations.mjs && npm --prefix apps/web run smoke:e2e`
3. The E2E + catalog/outcome checks are the correct browser-acceptance boundary; native acceptance is distributed across GAT13FROCONASY-009/010/013.
