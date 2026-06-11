# GAT11MASCLABLU-019: Browser E2E smoke, a11y/no-leak, and catalog README reconciliation

**Status**: DONE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation/CI) — new `apps/web/e2e/masked-claims.smoke.mjs`; modifies `apps/web/package.json` (`smoke:e2e`), `apps/web/README.md`, `README.md`, `.github/workflows/gate-1-game-smoke.yml` (no Rust/behavior surface)
**Deps**: GAT11MASCLABLU-017, GAT11MASCLABLU-018

## Problem

The gate's `reaction UI smoke tests pass` exit line needs an end-to-end browser smoke covering the full reaction-window flow plus a11y/no-leak assertions, and the web-shell catalog READMEs must be reconciled to name `masked_claims` — co-landed here so the catalog drift is not deferred to an aftermath pass.

## Assumption Reassessment (2026-06-10)

1. The board (GAT11MASCLABLU-017) and player-rules (GAT11MASCLABLU-018) provide the UI to smoke. The model is `apps/web/e2e/plain-tricks.smoke.mjs` (confirmed); `apps/web/package.json` `smoke:e2e` chains all per-game smokes, ending `&& node e2e/plain-tricks.smoke.mjs` (confirmed).
2. Spec Deliverables "Browser smoke" requires coverage of human claim, reaction-window prompt and waiting state, accept resolution, challenge resolution with reveal, bot claim, bot response, replay step/export/import, reduced motion, and no hidden tile ID in DOM/storage/logs/test-IDs for unrevealed masks. The §Documentation-updates catalog closeout surfaces are the three mechanically-checked surfaces plus the process-enforced renderer list.
3. Cross-artifact boundary under audit: `scripts/check-catalog-docs.mjs` keys on the `crates/wasm-api/src/lib.rs` `GAME_*` const (confirmed) and checks the `apps/web/README.md` intro catalog list, the root `README.md` "current official games are" list, and the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet; the `apps/web/README.md` Shell Surface renderer bullet is process-enforced (manual). Adding `masked_claims` here CLOSES the `check-catalog-docs` red window opened by the `GAME_MASKED_CLAIMS` const in GAT11MASCLABLU-014. `.github/workflows/gate-1-game-smoke.yml` is ALSO modified by GAT11MASCLABLU-015 (native lanes) — a shared-file overlap requiring a mechanical merge.
4. FOUNDATIONS §7 (UI smoke; play-first) and §11 (a11y; hidden info does not leak through DOM/storage/logs/test-IDs) are the principles under audit.
5. No-leak firewall enforcement surface: the a11y/no-leak smoke asserts no unrevealed tile ID appears in the DOM, `data-testid`, local storage, or logs for the pedestal pre-reveal, veiled galleries, hands, or reserve.

## Architecture Check

1. This ticket doubles as the browser capstone: it ships new smoke infrastructure (the e2e harness) while exercising the gate's UI exit criteria end-to-end. Co-landing the catalog READMEs with the `smoke:e2e` wiring avoids the deferred-aftermath drift seen in the Gate 9 `token_bazaar` closeout.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; this is presentation/CI wiring only.

## Verification Layers

1. Reaction-window UI flows -> `node apps/web/e2e/masked-claims.smoke.mjs`.
2. a11y / no-leak in the browser -> the `a11y-noleak` smoke + the new smoke's DOM/storage/test-ID assertions.
3. Catalog surfaces name `masked_claims` -> `node scripts/check-catalog-docs.mjs` (this closes the GAT11MASCLABLU-014 red window).
4. Full E2E suite green -> `npm --prefix apps/web run smoke:e2e`.

## What to Change

### 1. `apps/web/e2e/masked-claims.smoke.mjs`

Cover human claim, reaction prompt + claimant waiting state, accept resolution, challenge resolution with reveal animation, bot claim, bot response, replay step/export/import, reduced motion, and the no-hidden-tile-ID assertions.

### 2. `apps/web/package.json`

Append `&& node e2e/masked-claims.smoke.mjs` to the `smoke:e2e` script.

### 3. Catalog READMEs

`apps/web/README.md`: intro catalog list, Shell Surface renderer list, and Smoke Layers `smoke:e2e` bullet. `README.md`: "current official games are" list.

### 4. `.github/workflows/gate-1-game-smoke.yml`

Add the `masked_claims` web build + E2E registration step (coordinate the shared-file merge with GAT11MASCLABLU-015's native lanes).

## Files to Touch

- `apps/web/e2e/masked-claims.smoke.mjs` (new)
- `apps/web/package.json` (modify)
- `apps/web/README.md` (modify)
- `README.md` (modify)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- Native tool/CI lanes (GAT11MASCLABLU-015).
- Admission evidence and the spec/index `Done` flip (GAT11MASCLABLU-020).

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/masked-claims.smoke.mjs` passes, covering the reaction-window flows listed under the spec Exit criteria.
2. `node scripts/check-catalog-docs.mjs` passes (the `check-catalog-docs` red window is closed).
3. `npm --prefix apps/web run smoke:e2e` passes with the new smoke registered.

### Invariants

1. The reaction UI is exercised end-to-end; animation is semantic-effect-driven with reduced-motion support (FOUNDATIONS §7).
2. No unrevealed tile ID appears in DOM, `data-testid`, storage, or logs (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/masked-claims.smoke.mjs` — the reaction-window browser smoke + no-leak assertions.

### Commands

1. `node apps/web/e2e/masked-claims.smoke.mjs`
2. `npm --prefix apps/web run smoke:e2e && node scripts/check-catalog-docs.mjs`
3. The full `smoke:e2e` plus `check-catalog-docs` is the correct browser-acceptance boundary; native-pipeline acceptance is GAT11MASCLABLU-015's.

## Outcome

Added the Masked Claims browser E2E smoke covering hotseat claim/accept, reaction prompt controls, challenge reveal resolution, bot-vs-bot claimant waiting and response flow, reduced-motion rendering, public replay export/import, and DOM/storage/console/`data-testid` no-leak assertions for unrevealed mask IDs. Registered the smoke in `smoke:e2e` and CI, reconciled the root/web catalog README surfaces, and updated the rules-display catalog smoke for the eleventh game. Normalized replay import JSON in the web import component so pretty-printed public exports round-trip through Rust.

Verification:

1. `node apps/web/e2e/masked-claims.smoke.mjs`
2. `node scripts/check-catalog-docs.mjs`
3. `npm --prefix apps/web run smoke:e2e`
