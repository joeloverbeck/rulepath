# GAT18BLAPACSPA-016: dedicated e2e smoke, ci/games.json, and catalog README reconciliation

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (audit + e2e) — `apps/web/e2e/blackglass-pact.smoke.mjs`, `apps/web/package.json`, `ci/games.json`, `apps/web/README.md`, `README.md`
**Deps**: GAT18BLAPACSPA-015

## Problem

Add the dedicated browser e2e smoke and reconcile the catalog-enforced surfaces: `blackglass-pact.smoke.mjs` (setup, blind phase, bidding, trick play, team grouping, score/bags, observer, hotseat, replay, a11y, no-leak, terminal), the `smoke:e2e` wiring, the `ci/games.json` record, and the three `check-catalog-docs.mjs`-enforced README lists. Landing these closes the catalog red window opened at GAT18BLAPACSPA-014 (spec §4.5, §10.6–§10.7, Appendix E.11, candidate task `GAT18-BLAPAC-013`).

## Assumption Reassessment (2026-06-25)

1. `apps/web/package.json` `smoke:e2e` enumerates each game smoke explicitly (`:17`, ending `... briar-circuit.smoke.mjs && vow-tide.smoke.mjs && animation.smoke.mjs`) → append `node e2e/blackglass-pact.smoke.mjs`. `ci/games.json` records `{id, sim_flags, e2e}` (`briar_circuit:17`).
2. `scripts/check-catalog-docs.mjs` keys off the `crates/wasm-api` catalog const (added in GAT18BLAPACSPA-014) and mechanically checks the `apps/web/README.md` intro list, the root `README.md` "current official games are" list, and the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet; the README Shell Surface renderer bullet is process-enforced.
3. Cross-artifact boundary under audit: the e2e harness drives the GAT18BLAPACSPA-015 renderer end-to-end; `ci/games.json`'s `e2e` field must name the smoke file landed here so gate-1 CI resolves it.
4. FOUNDATIONS §11 (no-leak) / §7 (play-first UI) motivate this ticket: the smoke asserts no card identities during blind commitment and no leaked private values in DOM/storage/log, plus keyboard/a11y/reduced-motion.

## Architecture Check

1. Co-landing `ci/games.json` + the README lists with the e2e smoke (vs. a trailing docs ticket) closes the `check-catalog-docs` red window at the same PR that wires the smoke (the Gate-9 token_bazaar aftermath miss).
2. No shims; the smoke uses the existing node e2e harness convention.
3. `engine-core` untouched; no `game-stdlib` change; web smoke + catalog docs.

## Verification Layers

1. e2e smoke covers the Appendix E.11 click-path incl. no-leak DOM/storage/log -> `node apps/web/e2e/blackglass-pact.smoke.mjs` (via `smoke:e2e`).
2. Catalog README surfaces consistent with the wasm catalog const -> `node scripts/check-catalog-docs.mjs`.
3. `ci/games.json` record valid and resolves the e2e file -> `node scripts/check-ci-games.mjs`.

## What to Change

### 1. e2e smoke

`apps/web/e2e/blackglass-pact.smoke.mjs` (new): catalog selection, fixed-four setup, blind-ineligible first hand, blind declare/decline fixture path, no-card-during-blind assertions, post-deal own hand + hidden partner/opponents, sequential bids + public combined contract, blocked/legal spade lead, follow-suit, trick winner/effect, score breakdown with nil + bag rollover, observer, hotseat erasure, replay samples, target-tie continuation, terminal outcome, a11y/keyboard/reduced-motion, no console errors/leaks.

### 2. smoke + CI wiring

`apps/web/package.json` (modify): append the smoke to `smoke:e2e`; `ci/games.json` (modify): add `{ "id": "blackglass_pact", "sim_flags": "--seat-count 4 --action-cap 4096", "e2e": "blackglass-pact.smoke.mjs" }`.

### 3. Catalog README reconciliation

`apps/web/README.md` (modify): intro/catalog list + Shell Surface renderer bullet + Smoke Layers `smoke:e2e` bullet; `README.md` (modify): "current official games are" list.

## Files to Touch

- `apps/web/e2e/blackglass-pact.smoke.mjs` (new)
- `apps/web/package.json` (modify), `ci/games.json` (modify)
- `apps/web/README.md` (modify), `README.md` (modify)

## Out of Scope

- Trailing game docs incl. `UI.md` outcome section (GAT18BLAPACSPA-017).
- Atlas/register/forward-v1 receipt closeout (GAT18BLAPACSPA-018).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` (includes the new smoke; no leaks/console errors).
2. `node scripts/check-catalog-docs.mjs` (green — closes the red window).
3. `node scripts/check-ci-games.mjs` (valid `blackglass_pact` record).

### Invariants

1. The e2e smoke proves no card identities during blind commitment and no private values in DOM/storage/log.
2. The wasm catalog const, `ci/games.json`, and the three README lists agree.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/blackglass-pact.smoke.mjs` — full Appendix E.11 browser smoke.
2. `apps/web/package.json` — `smoke:e2e` wiring.
3. `ci/games.json` — `blackglass_pact` record.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `node scripts/check-catalog-docs.mjs && node scripts/check-ci-games.mjs`
3. The e2e smoke + catalog/CI checkers are the correct boundary; they close the GAT18BLAPACSPA-014 red window.
