# RIVLEDSHO-005: e2e worked-example assertion + browser no-leak sweep

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/e2e/river-ledger.smoke.mjs`
**Deps**: RIVLEDSHO-004

## Problem

The user-facing fix — "Pair of Queens beats Pair of Eights" rendered legibly after showdown — must be locked by an e2e assertion, and the new explanation surface must be swept for hidden-information leaks in the browser (DOM, `data-testid`, storage, console). This ticket extends the River Ledger e2e smoke (spec WB5 / R12 + R7).

## Assumption Reassessment (2026-06-15)

1. Verified against current code: `apps/web/e2e/river-ledger.smoke.mjs` exists and is wired into `smoke:e2e` in `apps/web/package.json`; it already covers observer/wrong-seat browser no-leak, storage/console checks, and terminal outcome rendering.
2. Verified against specs/docs: spec §8 WB5 + §10; `games/river_ledger/docs/UI.md` §Smoke And Tests + §No-Leak Requirements; `RULES.md` `RL-UI-NOLEAK-001`.
3. Cross-artifact boundary under audit: the e2e harness drives the RIVLEDSHO-004 panel rendering of the RIVLEDSHO-003 bridged fields; it adds assertions, no production logic.
4. FOUNDATIONS §11 no-leak invariant motivates this ticket: the new explanation strings, accessibility labels, and `data-testid`s must not expose folded/unrevealed hole cards, future board, or hand-strength of non-revealed seats.
5. §11 no-leak firewall is the enforcement surface: observer and wrong-seat browser contexts contain no unauthorized card label, hidden setup id, candidate ranking, or private rationale in DOM text, attributes, `data-testid`s, local/session storage, or console; folded seats' explanation fields are absent.

## Architecture Check

1. Extending the existing River Ledger e2e smoke (rather than a new harness) reuses the established Puppeteer/no-leak scaffolding and keeps one e2e entry per game.
2. No backwards-compatibility aliasing/shims; assertion-only additions.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4) — e2e test code only.

## Verification Layers

1. After a worked-example showdown the DOM shows "Pair of Queens beats Pair of Eights." (or the deterministic decisive sentence for the played hand) -> `node apps/web/e2e/river-ledger.smoke.mjs` assertion.
2. Observer/wrong-seat contexts leak no unrevealed card or folded-seat hand-strength through DOM/test-id/storage/console -> the smoke's no-leak sweep extended over the new explanation surface.
3. The smoke runs in the `smoke:e2e` lane -> `npm --prefix apps/web run smoke:e2e`.

## What to Change

### 1. `apps/web/e2e/river-ledger.smoke.mjs`

Add a deterministic worked-example showdown path and assert the rendered decisive sentence + named winning hand; extend the no-leak sweep (DOM text, attributes, `data-testid`, storage, console) over the new explanation/accessibility-label surface, including a folded-seat-absence assertion.

## Files to Touch

- `apps/web/e2e/river-ledger.smoke.mjs` (modify)

## Out of Scope

- Panel/layout rendering itself (RIVLEDSHO-004).
- Crate-level reveal-scope no-leak (RIVLEDSHO-002).
- Card component visuals (RIVLEDSHO-006).

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/river-ledger.smoke.mjs` — worked-example decisive sentence asserted; explanation-surface no-leak sweep green.
2. `npm --prefix apps/web run smoke:e2e` — full e2e lane green.
3. `npm --prefix apps/web run build` — shell builds for the e2e run.

### Invariants

1. The decisive explanation is asserted from rendered Rust-authored text, not recomputed in the test (§2).
2. No unrevealed card, folded-seat hand-strength, or private rationale appears in any unauthorized browser context (§11 no-leak firewall).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/river-ledger.smoke.mjs` (modify) — worked-example assertion + explanation-surface no-leak sweep.

### Commands

1. `node apps/web/e2e/river-ledger.smoke.mjs`
2. `npm --prefix apps/web run smoke:e2e`
3. The browser e2e lane is the correct boundary for DOM/storage/log no-leak; crate-level reveal-scope is RIVLEDSHO-002.

## Outcome

Completed: 2026-06-15

Changes:
- Extended `apps/web/e2e/river-ledger.smoke.mjs` with a deterministic four-seat hotseat checkdown using seed `79`, which renders the Rust-authored worked example text `Pair of Queens beats Pair of Eights.`
- Added assertions for the worked-example headline phrase, decisive comparison, comparison basis, four revealed showdown hands, and 20 best-five card labels.
- Extended the browser no-leak sweep over the new explanation surface, including DOM text, attributes, `data-testid`s, storage, and console logs.
- Added an assertion that folded terminal standing rows do not receive hand-strength text.

Verification:
- `node apps/web/e2e/river-ledger.smoke.mjs`
- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:e2e`
- `git diff --check`

Notes:
- No production code changed; this ticket is e2e assertion coverage only.
