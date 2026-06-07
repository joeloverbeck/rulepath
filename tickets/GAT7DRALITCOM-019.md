# GAT7DRALITCOM-019: Accessibility, reduced motion, browser E2E smoke & a11y/no-leak

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/DraughtsLiteBoard.tsx` (a11y semantics, keyboard nav, reduced motion), `apps/web/e2e/draughts-lite.smoke.mjs` (new), `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` (modify), `apps/web/package.json` (modify — add the smoke to `smoke:e2e`).
**Deps**: 018

## Problem

A public-polish board must be keyboard-operable, screen-reader-legible, reduced-motion-respecting, and leak-free. This ticket adds the WAI-ARIA grid interaction (roving focus / arrow + Home/End + Ctrl+Home/End navigation, Enter/Space activation, Escape cancel), the live-region announcements (turn, mandatory capture, selected piece, destination count, forced continuation, promotion, diagnostics, bot move, terminal), reduced-motion handling, and the browser E2E smoke covering a playable path and a forced-capture path plus the no-leak/a11y checks. This ticket doubles as the web-acceptance capstone (spec §R21).

## Assumption Reassessment (2026-06-07)

1. `apps/web/src/components/DraughtsLiteBoard.tsx` (GAT7DRALITCOM-018) is the component this ticket adds a11y/reduced-motion behavior to. `apps/web/e2e/{a11y-noleak.smoke.mjs,column-four.smoke.mjs,directional-flip.smoke.mjs,three-marks.smoke.mjs}` and `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` are the precedents; `apps/web/package.json` `smoke:e2e` chains the per-game smoke scripts (verified present).
2. The a11y/motion contract is fixed by spec §R15 (grid semantics, keyboard keys, announcements), §R16 (effect feedback + reduced motion), and §R21 (web acceptance: pointer quiet/forced-capture/multi-jump, keyboard origin+destination + forced continuation, legal cues, forced-continuation unmistakable, promotion announced, reduced motion preserves semantics, effect log, replay multi-segment, no TS legality). WAI-ARIA APG grid + WCAG 2.3.3 are the external references (spec §Sources).
3. Cross-artifact boundary under audit: the smoke exercises the full stack (web renderer 018 → WASM 016 → Rust rules/effects/view); it must respect both the app-level reduced-motion preference and the OS `prefers-reduced-motion`. The checklist update governs the public no-leak/a11y contract for Draughts Lite.
4. FOUNDATIONS §7/§11 motivate this ticket: restate before coding — public UI is play-first, accessible, and not debug-dominated; reduced motion must preserve the semantic sequence via static highlights + effect log; no color-only cues. Animation is effect-driven.
5. No-leak firewall enforcement surface (§11): the a11y/no-leak smoke is the end-to-end proof that no hidden information reaches DOM, payloads, test IDs, or replay exports — for Draughts Lite (perfect information) this confirms no engine internals/stale-token state leak into the rendered board or announcements.

## Architecture Check

1. Building the grid interaction on roving focus + Rust-provided legal cues (rather than ad hoc tabindex + TS-computed availability) matches the APG pattern and keeps legality in Rust; reduced motion swaps animation for static emphasis without losing the effect-log semantics.
2. No backwards-compatibility shims; additive a11y behavior + a new smoke script.
3. `engine-core` is untouched (§3); presentation-only. Legal/availability state comes from Rust action-tree metadata (§2), not TS.

## Verification Layers

1. Keyboard navigation -> browser E2E (`draughts-lite.smoke.mjs`): arrows/Home/End/Ctrl+Home/End move focus; Enter/Space activates a legal origin/destination; Escape clears the pending path.
2. Playable + forced-capture path -> browser E2E: a quiet move and at least one forced-capture (multi-jump) path complete via the UI; forced continuation is announced.
3. No-leak / a11y -> no-leak visibility test + a11y smoke: no hidden state in DOM/payloads/test IDs; accessible names + live-region announcements present; non-color cues exist.
4. Reduced motion -> manual runbook + smoke: with reduced motion active, capture/promotion/terminal feedback is static and the effect log stays complete.

## What to Change

### 1. Accessibility & keyboard

In `DraughtsLiteBoard.tsx`, add the grid semantics, roving focus / `aria-activedescendant`, the §R15 key bindings, accessible cell/piece names, and a live region announcing turn/mandatory-capture/selection/destination-count/forced-continuation/promotion/diagnostic/bot/terminal events from Rust effects.

### 2. Reduced motion

Respect the app-level reduced-motion preference and `prefers-reduced-motion`: replace sliding/jumping animations with static emphasis + text; keep the effect log complete; no indefinite pulsing.

### 3. E2E smoke & checklist

Add `apps/web/e2e/draughts-lite.smoke.mjs` (playable path + forced-capture path + keyboard + a11y/no-leak assertions); add it to `smoke:e2e` in `package.json`; update `NO-LEAK-A11Y-CHECKLIST.md` with Draughts Lite keyboard paths, focus visibility, accessible names, reduced motion, non-color cues, and no-TS-legality leakage.

## Files to Touch

- `apps/web/src/components/DraughtsLiteBoard.tsx` (modify — a11y, keyboard, reduced motion)
- `apps/web/e2e/draughts-lite.smoke.mjs` (new)
- `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` (modify)
- `apps/web/package.json` (modify — add `draughts-lite.smoke.mjs` to `smoke:e2e`)

## Out of Scope

- The base renderer / input model / pending-path state (GAT7DRALITCOM-018 — this ticket adds a11y/motion/smoke on top).
- WASM bridge changes (GAT7DRALITCOM-016).
- CI wiring of `smoke:e2e` (GAT7DRALITCOM-020).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` — the Draughts Lite smoke (playable + forced-capture path, keyboard, a11y/no-leak) passes alongside existing games.
2. `npm --prefix apps/web run build` — the web app builds.

### Invariants

1. The board is keyboard-operable and screen-reader-legible; reduced motion preserves semantic feedback (FOUNDATIONS §7; spec §R15/§R16).
2. No hidden information leaks to DOM/payloads/test IDs/replay; no TS legality is introduced (FOUNDATIONS §11; spec §R21).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/draughts-lite.smoke.mjs` — playable path, forced-capture path, keyboard navigation, a11y/no-leak assertions.
2. `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` — Draughts Lite checklist entries.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e`
3. The browser E2E smoke is the correct boundary for web acceptance; CI invokes it via GAT7DRALITCOM-020.
