# GAT9TOKBAZBRO-016: e2e no-leak/a11y smoke + gate-1 CI registration

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/e2e/token-bazaar.smoke.mjs` (new), `apps/web/package.json` (modify), `.github/workflows/gate-1-game-smoke.yml` (modify)
**Deps**: GAT9TOKBAZBRO-012, GAT9TOKBAZBRO-015

## Problem

Token Bazaar must have a browser end-to-end smoke proving a human action, a bot
action, replay step/export-import, the dev panel, the reduced-motion path, and the
no-leak/a11y checklist on the live page — and the gate-1 CI lane must run both that
e2e smoke and the four native game-id tool steps. This ticket adds the e2e smoke
harness, its package.json script, and the gate-1 workflow steps.

## Assumption Reassessment (2026-06-08)

1. The board renders after GAT9TOKBAZBRO-015 and the tools are registered after
   GAT9TOKBAZBRO-012. The sibling `apps/web/e2e/high-card-duel.smoke.mjs` +
   `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` + `apps/web/e2e/a11y-noleak.smoke.mjs`
   establish the e2e house pattern (verified present), and
   `.github/workflows/gate-1-game-smoke.yml` already runs the `high_card_duel`
   simulate/replay-check/fixture-check/rule-coverage steps + a combined browser
   e2e step (verified at lines 45-125) this ticket extends for `token_bazaar`.
2. The e2e coverage is fixed by `specs/gate-9-token-bazaar-browser-proof.md` →
   "Acceptance criteria → Browser" (e2e smoke covers human action, bot action,
   replay/export-import or stepping, dev panel, reduced-motion path, and
   no-leak/a11y checklist items; resource info not by color alone).
3. Cross-artifact boundary under audit: the gate-1 workflow file is shared with the
   tool registration (-012) — but -012 changes only tool *code*; all gate-1 *steps*
   (tool CLIs + e2e) land here to keep one CI-ownership diff. The e2e harness drives
   the live page produced by -015 and the tool steps invoke the arms from -012,
   hence both deps.
4. FOUNDATIONS §11 (no-leak; viewer-safe; accessible play-first UI): restating
   before trusting the spec — the smoke asserts no hidden/debug/candidate field is
   present in DOM, local storage, replay export, dev panel, or bot rationale (the
   game is public, so this is a regression guard), and the a11y checklist (color
   independence, keyboard reachability) passes.
5. Browser no-leak firewall: this e2e is the live-page enforcement of the no-leak
   firewall — the negative assertions from -006/-009 are re-proved against the
   rendered DOM and the export path, closing the loop the Rust-side tests opened.

## Architecture Check

1. Bundling the e2e harness + its npm script + all gate-1 game steps in one ticket
   gives a single "Token Bazaar is CI-gated end-to-end" diff; this ticket
   doubles as the browser-acceptance capstone for the gate (it ships the e2e
   harness, so the "no new production logic" capstone caveat does not apply).
2. No backwards-compatibility aliasing/shims — new e2e file + additive script + CI steps.
3. No engine behavior in the harness; it drives the live page and asserts no leak.
   `engine-core`/`game-stdlib` untouched.

## Verification Layers

1. e2e smoke passes -> simulation/CLI run: `node apps/web/e2e/token-bazaar.smoke.mjs`
   (and `npm --prefix apps/web run smoke:e2e`).
2. No-leak on the live page -> no-leak visibility test: the smoke asserts no
   internal field in DOM/storage/export/dev-panel/rationale.
3. a11y checklist -> manual + scripted checks against `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`.
4. gate-1 CI runs the tool + e2e steps -> CI dry-read + local run of each gate-1
   token_bazaar step.

## What to Change

### 1. `apps/web/e2e/token-bazaar.smoke.mjs` (new)

Puppeteer smoke: start a match, take a human action, run a bot turn, step/export-
import a replay, open the dev panel, exercise the reduced-motion path, and assert
the no-leak + a11y checklist (color-independence, keyboard reachability, no
internal field in DOM/storage/export/rationale).

### 2. `apps/web/package.json` (modify)

Add the token_bazaar e2e invocation to the `smoke:e2e` script (alongside the
existing per-game smokes).

### 3. `.github/workflows/gate-1-game-smoke.yml` (modify)

Add token_bazaar steps: `simulate` quick sim, `replay-check --all`, `fixture-check`,
`rule-coverage`, and the browser e2e (extend the combined e2e step to run
`token-bazaar.smoke.mjs`).

## Files to Touch

- `apps/web/e2e/token-bazaar.smoke.mjs` (new)
- `apps/web/package.json` (modify)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- gate-2 benchmark CI (GAT9TOKBAZBRO-011).
- Tool *code* arms (GAT9TOKBAZBRO-012) — this ticket only invokes them in CI.
- The spec/index Done-flip + atlas row (GAT9TOKBAZBRO-018).

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/token-bazaar.smoke.mjs` — the full click-path smoke passes.
2. `npm --prefix apps/web run smoke:e2e` — includes token_bazaar.
3. gate-1 workflow parses; each token_bazaar step runs locally
   (`cargo run -p simulate -- --game token_bazaar --games 1000`, etc.).

### Invariants

1. No hidden/debug/candidate/internal field appears in DOM, local storage, replay
   export, dev panel, or bot rationale on the live page (§11).
2. Resource information is not color-only; dense controls are keyboard reachable (§7).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/token-bazaar.smoke.mjs` — human/bot/replay/dev-panel/reduced-motion/no-leak/a11y.

### Commands

1. `npm --prefix apps/web ci && npm --prefix apps/web run build && node apps/web/e2e/token-bazaar.smoke.mjs`
2. `npm --prefix apps/web run smoke:e2e`
3. The live-page e2e is the correct boundary for browser no-leak/a11y — Rust-side
   no-leak is already proved in GAT9TOKBAZBRO-009; this closes the loop on the DOM.
