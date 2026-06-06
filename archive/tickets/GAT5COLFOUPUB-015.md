# GAT5COLFOUPUB-015: Column Four browser E2E smoke & a11y/no-leak

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — new `apps/web/e2e/column-four.smoke.mjs`; modify `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`, `apps/web/package.json`
**Deps**: 014

## Problem

Gate 5's public-polish and accessibility claims must be proven by an automated browser smoke: play-first default, seven column controls, keyboard play, Rust-provided previews, win/draw, bot rationale, replay projection, reduced motion, and a no-leak/a11y pass (spec §18 web smoke, §11 accessibility). This is also the gate's primary end-to-end acceptance harness.

## Assumption Reassessment (2026-06-06)

1. `apps/web/e2e/` contains `three-marks.smoke.mjs`, `shell.smoke.mjs`, `a11y-noleak.smoke.mjs`, and `NO-LEAK-A11Y-CHECKLIST.md`; the smoke chain is wired in `apps/web/package.json` (`smoke:e2e` runs build + the smoke scripts) — verified. This ticket adds `column-four.smoke.mjs` mirroring `three-marks.smoke.mjs` and extends the checklist + chain.
2. Spec §18 (web smoke) and §11 (accessibility) define the assertions: game picker shows Column Four; play-first default; 7×6 board; seven column controls; pointer + keyboard (Enter/Space) play; Rust preview on hover/focus; full columns become inert; win-line highlight; draw; human-vs-bot rationale; bot-vs-bot stops on terminal; replay export/import/step renders `ColumnFourBoard`; reduced-motion; no-leak DOM/attrs/storage/console/replay; dev panel secondary. The renderer under test is GAT5COLFOUPUB-014.
3. Cross-artifact boundary under audit: the no-leak/a11y contract in `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` and the WASM/client viewer-safe boundary. This ticket asserts the contract; it changes no production behavior.
4. FOUNDATIONS §11 (public UI is play-first, accessible, no hidden-info leak) and §7 motivate this ticket.
5. The no-leak visibility firewall (§11) is the enforcement surface under audit: the smoke is a fail-closed negative test that hidden/internal/candidate-ranking vocabulary never appears in DOM text, attributes, `data-testid`s, local/session storage, console logs, or replay textareas — this is where the 004/005/014 viewer-safe claims are finally enforced for the web surface.

## Architecture Check

1. A dedicated `column-four.smoke.mjs` mirroring the existing smoke harness keeps the gate's acceptance in the project's established Puppeteer-style E2E lane — cleaner than a bespoke test rig. This ticket doubles as the gate's primary end-to-end acceptance harness (deliverable-as-capstone).
2. No backwards-compatibility aliasing/shims — new smoke + additive checklist/chain entries.
3. No engine-core/game-stdlib change; the smoke drives the built app and asserts observable behavior.

## Verification Layers

1. Play-first / legal-controls invariant -> UI smoke: default page is play-first; seven column controls; only legal columns submit; full columns inert; terminal inert.
2. Keyboard/a11y invariant -> UI smoke: Tab reaches the column group; Enter/Space activates a legal column; focus visible; accessible names present; non-color cues for seat/terminal.
3. Preview/animation invariant -> UI smoke: hover/focus shows Rust preview; reduced-motion replaces drop motion without information loss.
4. Win/draw/bot invariant -> UI smoke: win highlights the Rust line; draw renders; human-vs-bot shows rationale; bot-vs-bot stops on terminal.
5. Replay invariant -> UI smoke: replay export/import/step renders the `ColumnFourBoard` projection at the Rust-projected landing row.
6. No-leak invariant -> no-leak visibility test: no hidden/internal/candidate-ranking term in DOM text, attributes, storage, console, or replay textareas.

## What to Change

### 1. `apps/web/e2e/column-four.smoke.mjs`

New browser smoke asserting the spec §18 web-smoke list and the §11 accessibility behaviors against the built app, including the no-leak negative checks.

### 2. `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`

Extend with the `column_four`-specific no-leak/a11y items (seven-column keyboard path, landing-preview labels, win-line non-color cue, replay textarea no-leak).

### 3. `apps/web/package.json`

Add `node e2e/column-four.smoke.mjs` to the `smoke:e2e` chain.

## Files to Touch

- `apps/web/e2e/column-four.smoke.mjs` (new)
- `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` (modify)
- `apps/web/package.json` (modify)

## Out of Scope

- The renderer/board itself (GAT5COLFOUPUB-014, dep).
- CI invocation of the smoke (GAT5COLFOUPUB-016) — this ticket makes it runnable locally; CI wires it.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` — the full chain including `column-four.smoke.mjs` passes.
2. `node apps/web/e2e/column-four.smoke.mjs` (against a built app) — column_four play/keyboard/preview/win/draw/bot/replay/reduced-motion/no-leak assertions pass.
3. `grep -q "column-four.smoke.mjs" apps/web/package.json` — wired into the smoke chain.

### Invariants

1. The smoke fails closed on any hidden-information leak in DOM/attrs/storage/console/replay.
2. The game is fully playable via keyboard with visible focus and accessible names.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/column-four.smoke.mjs` — the browser smoke + no-leak negative test.
2. `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` — updated coverage record.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run build && node apps/web/e2e/column-four.smoke.mjs`
3. The browser-smoke boundary is correct here: these behaviors (focus, animation, no-leak DOM) are only observable in a rendered browser, not in Rust tests.
