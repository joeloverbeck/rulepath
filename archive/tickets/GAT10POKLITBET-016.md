# GAT10POKLITBET-016: Browser e2e smoke, catalog README reconciliation, and gate-1 e2e step

**Status**: DONE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/e2e/poker-lite.smoke.mjs`, `apps/web/package.json`, `apps/web/README.md`, root `README.md`, `.github/workflows/gate-1-game-smoke.yml`. New e2e test infra (this ticket doubles as the browser-acceptance capstone). No Rust/engine behavior.
**Deps**: GAT10POKLITBET-015, GAT10POKLITBET-012

## Problem

The gate needs a rendered-browser no-leak/a11y e2e smoke for **Crest Ledger**, wired into the `smoke:e2e` chain and gate-1 CI, plus reconciliation of the three catalog README surfaces that `scripts/check-catalog-docs.mjs` validates — which closes the expected red `check-catalog-docs` CI window opened when GAT10POKLITBET-014 added the `GAME_POKER_LITE` const. This ticket doubles as the browser-acceptance capstone (it ships harness code and exercises the full web path end-to-end).

## Assumption Reassessment (2026-06-08)

1. The e2e + catalog pattern is verified this session: `apps/web/e2e/secret-draft.smoke.mjs` is the freshest game smoke; `apps/web/package.json` `smoke:e2e` (~L13) is a hardcoded `node e2e/*.smoke.mjs` `&&`-chain (no positional arg) — `poker-lite.smoke.mjs` must be appended to it. `scripts/check-catalog-docs.mjs` keys off the `wasm-api` `GAME_*` const and checks three surfaces: the `apps/web/README.md` intro catalog list, the root `README.md` "current official games are" list, and the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet (Shell Surface bullet is process-enforced, not mechanically checked).
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §4 E2E + Catalog docs/scripts bullets, §E e2e no-leak, §10 apps/web/README reconciliation, §7 corrected commands) fixes: add `poker-lite.smoke.mjs`; e2e no-leak checks DOM text/accessibility names/`data-testid`/local storage/replay-export text/dev-panel for hidden ids before reveal; reconcile all three README catalog surfaces in-gate (not aftermath).
3. Cross-artifact boundary under audit: `check-catalog-docs.mjs` (run in `gate-1-game-smoke.yml`) keyed off the `GAME_POKER_LITE` const from GAT10POKLITBET-014; the `smoke:e2e` chain in `package.json`; and `gate-1-game-smoke.yml`, which GAT10POKLITBET-012 already modified (native steps) — this ticket adds the e2e step, so **Deps: 012** serializes the two gate-1 edits. Adding the catalog README surfaces here closes the red `check-catalog-docs` window from 014.
4. FOUNDATIONS §11 (no hidden-info leak through DOM/local storage/logs/replay exports/UI test ids) and §7 (play-first UI) motivate this ticket. Restated before trusting the spec narrative.
5. No-leak firewall surface under audit (§11/§12) — final web-surface sweep: the e2e smoke must assert no hidden card id/rank/label appears in DOM text, accessibility names, `data-testid`, local storage, replay-export text, or dev-panel content before the rule-defined reveal point. This complements the Rust-side no-leak tests (GAT10POKLITBET-007) and WASM redaction (014) on the browser surface.

## Architecture Check

1. Co-landing the three catalog README surfaces with the web-smoke / gate-1 ticket (not a trailing docs ticket or the capstone) is the established fix for catalog drift — it closes the `check-catalog-docs` red window at the earliest coherent point. The e2e smoke doubling as the browser-acceptance capstone avoids a synthetic acceptance-only ticket.
2. No backwards-compatibility aliasing/shims — additive e2e + README list entries.
3. `engine-core` untouched (§3); presentation/test infra only — no behavior moved to TS (§2); no `game-stdlib` change (§4).

## Verification Layers

1. E2E smoke passes (human-vs-bot path, yield path, observer no-leak, seat-private own-card, replay export/import, reduced motion, a11y) -> `npm --prefix apps/web run smoke:e2e`.
2. Browser no-leak sweep (no hidden id in DOM/`data-testid`/local storage/replay-export/dev-panel pre-reveal) -> the no-leak assertions inside `poker-lite.smoke.mjs` + the existing `a11y-noleak.smoke.mjs` (already in the chain).
3. Catalog reconciliation (three README surfaces name poker_lite) -> `node scripts/check-catalog-docs.mjs` (green — closes the red window).
4. CI wiring (gate-1 e2e step present) -> grep-proof in `gate-1-game-smoke.yml`.

## What to Change

### 1. `apps/web/e2e/poker-lite.smoke.mjs` (new)

Rendered-browser smoke: human-vs-bot press/match/showdown, yield path, observer no-leak before showdown, seat-private own-card view, public replay export/import, reduced motion, keyboard/a11y, stale-diagnostic, dev-panel whitelist — with explicit hidden-id no-leak assertions over DOM/accessibility/`data-testid`/local storage/replay-export/dev-panel.

### 2. `apps/web/package.json` (modify)

Append `&& node e2e/poker-lite.smoke.mjs` to the `smoke:e2e` chain.

### 3. `apps/web/README.md` + root `README.md` (modify)

Add **Crest Ledger** / `poker_lite` to: the `apps/web/README.md` intro catalog list, the Shell Surface renderer list (`PokerLiteBoard`), and the Smoke Layers `smoke:e2e` bullet; and the root `README.md` "current official games are" list.

### 4. `.github/workflows/gate-1-game-smoke.yml` (modify)

Add the `poker_lite` e2e step (the native steps were added in GAT10POKLITBET-012).

## Files to Touch

- `apps/web/e2e/poker-lite.smoke.mjs` (new)
- `apps/web/package.json` (modify)
- `apps/web/README.md` (modify)
- `README.md` (modify)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- Rust/WASM behavior and renderer component logic (GAT10POKLITBET-014/015).
- `progress.md`, `specs/README.md` index, `docs/MECHANIC-ATLAS.md`, spec Status flip (GAT10POKLITBET-018).
- Trailing game docs MECHANICS/UI/PUBLIC-RELEASE-CHECKLIST (GAT10POKLITBET-017).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` passes with `poker-lite.smoke.mjs` in the chain.
2. `node scripts/check-catalog-docs.mjs` is green (closes the red window from GAT10POKLITBET-014).
3. `npm --prefix apps/web run smoke:wasm` passes (ABI smoke).

### Invariants

1. No hidden card id/rank reaches DOM/`data-testid`/local storage/replay-export/dev-panel before its reveal point (§11 no-leak).
2. All three `check-catalog-docs` README surfaces name `poker_lite` (catalog reconciled in-gate, not aftermath).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/poker-lite.smoke.mjs` — rendered-browser path + browser no-leak sweep.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `node scripts/check-catalog-docs.mjs`
3. `npm --prefix apps/web run smoke:wasm` — ABI smoke; together with smoke:e2e these distribute browser acceptance (no separate acceptance-only capstone).

## Outcome

Completed on 2026-06-09.

- Added the Crest Ledger rendered-browser e2e smoke covering observer no-leak,
  hotseat private view, legal Rust-supplied controls, keyboard activation,
  stale diagnostics, public replay export/import, grouped showdown reveal, yield
  terminal behavior, reduced motion, mobile layout, and browser storage checks.
- Added `poker_lite` to the `smoke:e2e` chain and gate-1 game-smoke workflow.
- Reconciled the catalog README surfaces for the ninth official game and the
  Crest Ledger renderer/smoke coverage.
- Classified `poker_lite` as a viewer-filtered game in the dev panel so the
  browser no-leak smoke can assert the expected diagnostic surface.

Verification:

- `node scripts/check-catalog-docs.mjs`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run build`
- `node apps/web/e2e/poker-lite.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`

Note: the browser e2e commands required localhost server permission in the
sandboxed environment.
