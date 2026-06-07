# GAT6DIRFLI-018: Browser E2E smoke & a11y/no-leak

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None (browser E2E / a11y test harness under `apps/web/e2e/` — no Rust/engine surface). Adds `apps/web/e2e/directional-flip.smoke.mjs` and extends `apps/web/e2e/a11y-noleak.smoke.mjs` + `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`.
**Deps**: 017

## Problem

Before public exposure, `directional_flip` needs a browser E2E smoke (start, legal-move display, keyboard activation, human action, bot action, forced pass, replay stepping, reduced motion, dev-toggle safety) plus the no-leak/accessibility checklist coverage (FOUNDATIONS §7/§11, spec §12.3, §14 "web smoke once exposed"). This ticket adds the directional-flip smoke and extends the shared a11y/no-leak smoke + checklist. Realizes `DF-UI-001`/`002` and acceptance rows 20–22.

## Assumption Reassessment (2026-06-07)

1. `apps/web/e2e/column-four.smoke.mjs` is the precedent; `apps/web/e2e/a11y-noleak.smoke.mjs` and `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` exist and are shared across games (confirmed present). The renderer + shell wiring from GAT6DIRFLI-017 is what these tests drive.
2. Spec §12.3 (smoke coverage list + keyboard table), §14 (web-smoke exit rows), and rule ids `DF-UI-001`/`002` are authoritative.
3. Cross-artifact boundary under audit: the e2e harness ↔ the rendered DOM/payloads produced by `DirectionalFlipBoard.tsx` (017) and the WASM client (015). The no-leak smoke asserts that hidden/internal state never reaches the DOM, local storage, or logs.
4. FOUNDATIONS §11 (hidden information must not leak through DOM/payloads/logs; public UI accessible) motivates this ticket: restate before authoring — the no-leak smoke is a negative test (asserts forbidden state is absent), and the a11y smoke verifies keyboard/non-color-only/reduced-motion behavior per WCAG 2.2 + WAI-ARIA grid (spec §4).
5. This ticket is the browser-side **no-leak firewall** check (FOUNDATIONS §11): confirm the smoke asserts no engine internals/RNG/candidate-ranking/bot-hidden-state appear in the DOM or test-id payloads, and that the dev/learning toggle does not expose unsafe data in normal mode. (Per `references/decomposition-patterns.md`, this verification-harness ticket doubles as part of the gate's distributed acceptance.)

## Architecture Check

1. A dedicated directional-flip smoke plus extension of the shared a11y/no-leak harness mirrors the column_four precedent and keeps the no-leak contract asserted at the browser boundary, where leaks would actually surface.
2. No backwards-compatibility shims; new/extended test harness only.
3. `engine-core` untouched; this is presentation-layer verification (§3). No legality is introduced in TS (the smoke only drives Rust-provided choices).

## Verification Layers

1. Play-path smoke (`DF-UI-001`) -> simulation/CLI run (browser E2E): start, legal-move display, keyboard activation, human action, bot action, forced pass, replay stepping all succeed using Rust-provided choices/previews/effects.
2. Accessibility (`DF-UI-002`) -> manual + automated a11y smoke: keyboard grid, forced-pass control, reduced-motion, non-color-only encoding pass.
3. No-leak -> no-leak visibility test (browser): no hidden/internal state in DOM/payloads/logs/test-ids; dev toggle safe in normal mode (FOUNDATIONS §11).

## What to Change

### 1. Directional-flip browser smoke

`apps/web/e2e/directional-flip.smoke.mjs`: cover start, legal-move display, keyboard activation, human action, bot action, forced pass, replay stepping, reduced motion, and dev-toggle safety (spec §12.3).

### 2. Shared a11y/no-leak coverage

Extend `apps/web/e2e/a11y-noleak.smoke.mjs` with directional-flip checks and add the applicable rows to `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`.

## Files to Touch

- `apps/web/e2e/directional-flip.smoke.mjs` (new)
- `apps/web/e2e/a11y-noleak.smoke.mjs` (modify)
- `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` (modify)

## Out of Scope

- The renderer itself (GAT6DIRFLI-017).
- CI lane wiring that runs these smokes (GAT6DIRFLI-019).
- Public picker exposure decision (GAT6DIRFLI-021).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` — directional-flip browser smoke passes (start → legal move → keyboard activate → bot → forced pass → replay step → reduced motion).
2. The a11y/no-leak smoke passes for `directional_flip`; checklist rows are present.

### Invariants

1. No hidden/internal state reaches the DOM, payloads, storage, or logs (FOUNDATIONS §11, `DF-UI-001`).
2. Keyboard, non-color-only encoding, and reduced-motion behavior pass (FOUNDATIONS §7, `DF-UI-002`).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/directional-flip.smoke.mjs` — full play-path + reduced-motion + dev-toggle-safety smoke.
2. `apps/web/e2e/a11y-noleak.smoke.mjs` — directional-flip a11y + no-leak assertions.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui`
3. The browser smoke suite is the correct boundary for UI/no-leak acceptance; CI invocation is GAT6DIRFLI-019.
