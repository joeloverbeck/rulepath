# GAT4THRMARBOA-013: Three Marks browser UI smoke tests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (test harness) — new `apps/web/e2e/three-marks.smoke.mjs`; `apps/web/package.json`, `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`
**Deps**: GAT4THRMARBOA-011, GAT4THRMARBOA-012

## Problem

Gate 4's UI acceptance requires browser smoke coverage of the Three Marks board: picker load, start, board render, click/tap a legal cell, occupied-cell inertness, bot turn, win highlight, draw, board-aware replay step, dev-panel-secondary, reduced motion, and keyboard/focus. Without automated smoke, the play-first/accessible/no-leak UI contract (FOUNDATIONS §7/§11) is unverified.

## Assumption Reassessment (2026-06-06)

1. `apps/web/e2e/shell.smoke.mjs` and `apps/web/e2e/a11y-noleak.smoke.mjs` are the existing e2e smoke harness; `apps/web/package.json:13` chains them via `smoke:e2e` (`npm run build && node e2e/shell.smoke.mjs && node e2e/a11y-noleak.smoke.mjs`); `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` is the manual a11y/no-leak checklist. Verified all exist. The Three Marks board (GAT4THRMARBOA-011) and board-aware replay (012) are the surfaces under test.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §15.7 (browser smoke matrix), §18 (accessibility + reduced motion, tested path), Appendix D (browser acceptance checklist).
3. Cross-artifact boundary under audit: the e2e smoke harness contract (how `shell.smoke.mjs`/`a11y-noleak.smoke.mjs` drive the built shell and assert DOM/observable state) — the new smoke mirrors that harness.
4. FOUNDATIONS §7 (public UI is play-first, polished, accessible) and §11 (hidden information does not leak through DOM/UI test ids; semantic effects drive animation) motivate this ticket: the smoke asserts legal-only interaction, accessible names/keyboard paths, reduced-motion usability, and no hidden-state leak in the DOM.
5. No-leak visibility-test enforcement surface (§11): the DOM and UI test ids are the firewall — name them. The a11y/no-leak smoke asserts the Three Marks DOM exposes only public-view data (perfect information → nothing to redact, but the negative assertion still runs) and that occupied/illegal cells are inert rather than disabled-but-clickable.

## Architecture Check

1. Extending the existing `e2e/*.smoke.mjs` harness (rather than introducing a new framework) keeps one browser-smoke contract and reuses the build pipeline — cleaner and matching Gate 3. This ticket is the UI-acceptance capstone-double (it ships test infrastructure that also serves as the §15.7 acceptance surface).
2. No backwards-compatibility aliasing/shims — new smoke file + script chaining; existing smokes untouched.
3. `engine-core` untouched; no `game-stdlib` change; the smoke asserts TypeScript decides no legality (only Rust legal cells are interactive).

## Verification Layers

1. Play-path invariant -> UI smoke (`three-marks.smoke.mjs`: picker→start→render nine cells→click legal cell updates board→occupied cell inert→bot turn advances→win highlight→draw→replay step shows board).
2. Accessibility invariant -> UI smoke + manual checklist (keyboard focus/activation of legal cells; accessible names; reduced-motion does not block play/replay; dev panel secondary).
3. No-leak invariant -> no-leak visibility test (DOM/UI-test-id assertion: only public-view data present; occupied/illegal cells inert).

## What to Change

### 1. `apps/web/e2e/three-marks.smoke.mjs` (new)

Drive the built shell through the §15.7 matrix: load game picker (both games), start Three Marks, render board (nine cells + status), click/tap a legal cell (Rust action dispatched, board updates), occupied cell not clickable, bot turn advances using the Rust bot, win highlight from Rust cells, draw presentation, replay step shows board (not JSON-only), dev panel secondary, reduced-motion mode, keyboard/focus selection of a legal cell.

### 2. `apps/web/package.json` (modify)

Chain `node e2e/three-marks.smoke.mjs` into the `smoke:e2e` script after the existing smokes.

### 3. `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` (modify)

Add Three Marks rows for keyboard, reduced motion, color-plus-shape, and DOM no-leak checks.

## Files to Touch

- `apps/web/e2e/three-marks.smoke.mjs` (new)
- `apps/web/package.json` (modify)
- `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` (modify)

## Out of Scope

- The renderer/replay-view implementation (GAT4THRMARBOA-011/012) — this ticket exercises them.
- Native CLI/benchmark smoke (008/014) and CI workflow wiring (GAT4THRMARBOA-016).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` — Three Marks smoke passes alongside the existing shell/a11y smokes.
2. The smoke asserts: legal cell click dispatches a Rust action; occupied cells are inert; win highlight + draw render from Rust; replay step shows a board.
3. Keyboard/focus and reduced-motion paths are asserted (or checklisted where automation is impractical).

### Invariants

1. Only Rust-provided legal cells are interactive in the smoke; no hidden state appears in the DOM/UI test ids.
2. The Three Marks smoke is chained into `smoke:e2e` and runs in CI gate-1 (wired in GAT4THRMARBOA-016).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/three-marks.smoke.mjs` — full §15.7 browser smoke matrix.
2. `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` — manual a11y/no-leak rows for Three Marks.

### Commands

1. `npm --prefix apps/web run build && node apps/web/e2e/three-marks.smoke.mjs`
2. `npm --prefix apps/web run smoke:e2e`
3. `smoke:e2e` is the correct full-pipeline boundary (it builds the WASM+shell and runs every browser smoke); the standalone node invocation is the fast iteration lane.
