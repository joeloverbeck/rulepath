# GAT3WASMSTAWEB-013: Browser UI E2E smoke harness (Puppeteer)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — adds a browser E2E smoke harness + dev dependency (`apps/web`); no Rust/crate change.
**Deps**: 008, 009, 011

## Problem

Gate 3 must prove the rendered app as a user-facing shell, not just the low-level
WASM API (spec §19.3; §23.3 "smoke scripts exercise WASM directly and may pass
while the actual browser shell is broken"). The current `apps/web/scripts/*.mjs`
are Node smoke scripts that instantiate/use WASM and cannot prove the rendered DOM.
This ticket adds a lightweight rendered-browser E2E smoke (Puppeteer-preferred per
§6, §19.4) over the served `dist`, exercising the Gate 3 play flows end-to-end. It
is the capstone for the play-mode and replay tickets.

## Assumption Reassessment (2026-06-06)

1. `apps/web/package.json` dependencies are `react`/`react-dom`/`typescript`/`vite`/
   `@vitejs/plugin-react` and `@types/*` — no Puppeteer or Playwright present. The
   existing smoke scripts (`smoke-load-wasm.mjs`, `smoke-ui.mjs`) drive WASM/the app
   via `render_game_to_text`, not the rendered DOM. The served-`dist` plumbing comes
   from GAT3WASMSTAWEB-011 (`smoke:preview`); the play modes and replay UI this
   harness drives come from GAT3WASMSTAWEB-008/-009. Components expose `data-testid`
   hooks today (e.g. `start-match`, `counter`, `turn`, `choice-*`, `effects`,
   `diagnostic`) usable as locators.
2. Spec §19.3 minimum browser smoke: built/preview app loads; normal page not
   dominated by raw JSON/debug UI; game picker shows Race-to-N; setup starts a match;
   public view/status appears; Rust legal actions appear as controls; one human
   action; one bot turn; stale diagnostics safe/debug-labeled if exposed; effect log
   updates; replay/dev panel opens with safe basics; replay export/import/step works
   minimally; bot-vs-bot step/autoplay advances; reduced-motion path enabled/emulated;
   keyboard/focus smoke for the critical flow. §19.4 prefers Puppeteer unless
   Playwright's cross-browser/role-locator/reliability is materially needed; if
   Playwright is adopted, scope it to a few Gate-3 smoke specs only.
3. Cross-artifact boundary under audit: the harness drives the rendered shell over
   the served `dist` (GAT3WASMSTAWEB-011); it asserts user-visible DOM/text/roles,
   complementary to the raw-ABI smoke (GAT3WASMSTAWEB-012). It must remain reliable
   in CI (the gate wiring is GAT3WASMSTAWEB-014/-015's concern; this ticket provides
   the runnable harness).
4. FOUNDATIONS §7 (public UI is play-first, not debug-dominated): restated — the
   harness asserts the normal page is play-first (no raw-JSON-dominant content) and
   that legal controls come from Rust action choices, guarding the spec's product
   intent in CI.

## Architecture Check

1. A narrow Puppeteer smoke over served `dist` (rather than a broad E2E framework)
   minimizes tooling churn per §6/§19.4 while closing the §23.3 gap; one or a few
   smoke specs cover the Gate-3 flows. Playwright is deferred unless a concrete
   cross-browser/role-locator/reliability need is documented (then scoped narrowly).
2. No backwards-compatibility shims: this adds a new test layer; the existing node
   smoke is retained for ABI coverage, not duplicated.
3. `engine-core` untouched; test harness only; `game-stdlib` untouched.

## Verification Layers

1. Rendered shell loads + plays → simulation/CLI run: the Puppeteer smoke loads the
   served `dist`, starts a match, applies a human action and a bot turn, and reads
   the resulting DOM/status.
2. Play-first, Rust-driven controls → manual review + assertion: the smoke asserts
   the normal page is not raw-JSON-dominated and that action controls reflect the
   Rust action tree.
3. Modes + replay + dev panel → simulation: the smoke advances hotseat and
   bot-vs-bot step/autoplay, opens the dev/replay panel, and exercises replay
   export/import/step minimally.
4. Keyboard/reduced-motion critical path → manual review + assertion: the smoke runs
   the critical flow by keyboard and with reduced motion enabled/emulated (full a11y
   audit is GAT3WASMSTAWEB-014).

## What to Change

### 1. `apps/web/package.json`

Add Puppeteer as a devDependency and a `smoke:e2e` (and/or `test:e2e`) script that
builds, serves `dist` (reusing the GAT3WASMSTAWEB-011 preview server), and runs the
browser smoke.

### 2. New `apps/web/e2e/shell.smoke.mjs` (Puppeteer)

Drive the §19.3 flows over the served app using `data-testid`/role/text locators:
load → picker → setup/start → public view/status → legal action controls → human
action → bot turn → effect log update → dev/replay panel open → replay export/
import/step → bot-vs-bot step/autoplay → reduced-motion path → keyboard/focus on the
critical flow. Assert the normal page is not raw-JSON-dominated.

## Files to Touch

- `apps/web/package.json` (modify) — Puppeteer devDependency + `smoke:e2e` script
- `apps/web/e2e/shell.smoke.mjs` (new) — rendered-browser smoke over served dist

## Out of Scope

- New production logic (this ticket exercises prior tickets; it adds no app behavior).
- The repo-wide no-leak review + accessibility checklist + axe-style scan — GAT3WASMSTAWEB-014.
- CI workflow wiring/docs — GAT3WASMSTAWEB-015 (and CI integration §19.7).
- Cross-browser matrix / Playwright migration (deferred unless justified, §19.4, §22).

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run smoke:e2e` — the rendered-browser smoke passes the §19.3 critical flow against served `dist`.
2. `cd apps/web && npm run build` — typecheck + build remain green.
3. `grep -nE "picker|setup|choice|bot|replay|reduced" apps/web/e2e/shell.smoke.mjs` — the required Gate-3 flows are exercised.

### Invariants

1. The browser smoke exercises the rendered shell (DOM/roles/text), not the raw WASM API.
2. The smoke asserts a play-first page (not raw-JSON/debug-dominated) with Rust-action-tree-driven controls.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/shell.smoke.mjs` (new) — rendered-browser capstone for play/replay/modes; rationale: the only layer that proves the user-facing shell works (§23.3).

### Commands

1. `cd apps/web && npm run smoke:e2e`
2. `cd apps/web && npm run build`
3. A rendered-browser harness (not the node ABI smoke) is the correct boundary for shell behavior; raw-ABI coverage stays in GAT3WASMSTAWEB-012.

## Outcome

Completed on 2026-06-06.

Changes:

- Added Puppeteer as an `apps/web` dev dependency and introduced `npm run smoke:e2e`.
- Added `apps/web/e2e/shell.smoke.mjs`, which serves built `dist` under `/rulepath/`, launches system Chrome, and drives the rendered shell through picker/setup, keyboard start, Rust action controls, human/bot play, effect log, developer stale diagnostic, replay export/import/step, bot-vs-bot step/autoplay, and reduced-motion styling.

Deviations:

- The harness uses `/usr/bin/google-chrome` by default and honors `PUPPETEER_EXECUTABLE_PATH`; Puppeteer was installed with browser download scripts skipped because the environment already has system Chrome.
- The live game display name is `Race to 21`; the harness asserts that current catalog text rather than the draft ticket's `Race-to-N` wording.

Verification:

- `npm --prefix apps/web run smoke:e2e`
- `npm --prefix apps/web run build`
- `grep -nE "picker|setup|choice|bot|replay|reduced" apps/web/e2e/shell.smoke.mjs`
