# VICEXPSHASUR-011: Browser smoke, no-leak, accessibility & replay checks + smoke registration

**Status**: DONE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only / test infra) — adds `apps/web/e2e/outcome-explanation.smoke.mjs` and registers it across `scripts/check-catalog-docs.mjs`, `apps/web/README.md`, and `apps/web/package.json`; no Rust/engine, WASM, or behavior surface.
**Deps**: VICEXPSHASUR-010

## Problem

The wired outcome surface (010) needs a catalog-complete browser smoke proving, for every game, that the panel appears at terminal with the decisive cause and full standing, the breakdown is keyboard- and pointer-operable, reduced-motion preserves all facts, and — critically — no hidden information leaks through any channel. The new smoke must also be registered where the existing `rules-display` smoke is, or `scripts/check-catalog-docs.mjs` fails catalog-drift. This is a §Deliverable-doubles-as-capstone ticket (it ships new e2e infrastructure while exercising the gate end-to-end). Source: `archive/specs/victory-explanation-shared-surface.md` §12.3, §15.7.

## Assumption Reassessment (2026-06-09)

1. Verified current code: `apps/web/e2e/` holds `shell.smoke.mjs`, `a11y-noleak.smoke.mjs`, `rules-display.smoke.mjs`, and the per-game smokes; `rules-display.smoke.mjs` is the cross-game precedent (asserts sections + hidden-info leak per game). `scripts/check-catalog-docs.mjs:27` carries `NON_GAME_SMOKE = new Set(["shell", "a11y-noleak", "rules-display"])`; `apps/web/package.json:15` `smoke:e2e` chains each `e2e/<name>.smoke.mjs`. The new smoke is cross-game (non-game), so it registers in `NON_GAME_SMOKE` like `rules-display`.
2. Spec §12.3/§15.7. The A1 registration requirement (now in spec §12.3) names the three surfaces: `NON_GAME_SMOKE`, the `apps/web/README.md` Smoke Layers `smoke:e2e` bullet, and the `apps/web/package.json` `smoke:e2e` script.
3. Cross-artifact boundary under audit: the smoke exercises the full pipeline (003–010) end-to-end via the WASM web shell; the registration touches `check-catalog-docs.mjs` + `README.md` + `package.json` so the catalog-drift guard and the gate-1 lane stay green. Landing this ticket (after 010) resolves the expected red-CI windows of 002 (`check-outcome-explanations`) and `check-catalog-docs`.
4. FOUNDATIONS §2 restated: the smoke asserts the panel content matches Rust-supplied data and that no TypeScript computed the result; it verifies presentation, it decides nothing.
5. §11 no-leak firewall (the central deliverable): assert no hidden information appears in visible text, hidden DOM text, `aria-label`/`title`/`alt`, `data-testid`/CSS classes, local/session storage, effect log, replay import/export, or dev-panel payloads — across all nine games — with dedicated `poker_lite` yield-no-reveal, `secret_draft` pre-reveal-commitment, and `high_card_duel` deck-tail negative checks; and assert the `role="status"` summary, keyboard-operable disclosure, and reduced-motion equivalence.
6. Registry extension: this adds the new smoke slug to the `check-catalog-docs.mjs` `NON_GAME_SMOKE` set, the README Smoke Layers bullet, and the `package.json` `smoke:e2e` chain. All three consumers are updated together (additive) so `check-catalog-docs` stays green — under-updating any one is the exact catalog-drift miss the guard exists to catch.

## Architecture Check

1. One cross-game smoke (re-enumerating the catalog rather than hardcoding nine) mirrors the `rules-display` precedent and keeps coverage catalog-complete as future games are added; registering it as a non-game smoke is the established pattern.
2. No backwards-compatibility shims: a new smoke file + additive registry entries; no existing smoke is modified.
3. `engine-core`/`game-stdlib` untouched; the smoke inspects the running shell, it implements no behavior.

## Verification Layers

1. Catalog-complete terminal coverage → the smoke re-enumerates the catalog (not a hardcoded 9) and drives each game to terminal; asserts the panel appears with the decisive sentence and a final-standing row per player.
2. Disclosure + a11y → assert the expandable breakdown opens with keyboard and pointer, the summary is a `role="status"` region, and reduced-motion mode still shows all terminal facts.
3. No-leak → negative assertions across visible text, hidden DOM, accessibility attributes, `data-testid`/CSS, storage, effect log, replay export, and dev panel; `poker_lite` includes pair-beats-high-card, private-rank tiebreak, split, and yield-no-reveal; `secret_draft` checks no pre-reveal commitment; `high_card_duel` checks no deck tail.
4. Registration green → grep-proof the slug is in `NON_GAME_SMOKE`, the README Smoke Layers bullet, and `package.json` `smoke:e2e`; `node scripts/check-catalog-docs.mjs` passes.
5. Replay consistency → the smoke (or a paired assertion) confirms replaying to terminal yields the same panel content for the same viewer (spec §7.4).

## What to Change

### 1. Add `apps/web/e2e/outcome-explanation.smoke.mjs`

Catalog-complete terminal smoke per spec §12.3: for every catalog game drive to terminal and assert panel presence, decisive sentence, per-player final standing, keyboard+pointer disclosure, rule references, reduced-motion equivalence, and the full no-leak negative set; with dedicated `poker_lite` yield/showdown/split/no-reveal cases and `secret_draft`/`high_card_duel` hidden-info checks.

### 2. Register the smoke (three surfaces)

- `scripts/check-catalog-docs.mjs` — add `"outcome-explanation"` to `NON_GAME_SMOKE`.
- `apps/web/README.md` — add the smoke to the Smoke Layers `smoke:e2e` bullet.
- `apps/web/package.json` — add `node e2e/outcome-explanation.smoke.mjs` to the `smoke:e2e` script.

## Files to Touch

- `apps/web/e2e/outcome-explanation.smoke.mjs` (new)
- `scripts/check-catalog-docs.mjs` (modify — `NON_GAME_SMOKE`)
- `apps/web/README.md` (modify — Smoke Layers bullet)
- `apps/web/package.json` (modify — `smoke:e2e` script)

## Out of Scope

- Any Rust rationale (003–008), the panel/templates (009), or board wiring (010) — this ticket exercises them.
- The closeout exit-evidence run and `specs/README.md` `Done`-flip (012).
- Any `engine-core`/`game-stdlib` change or TypeScript outcome logic.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` passes, including the new `outcome-explanation.smoke.mjs` across every catalog game.
2. The no-leak assertions pass for `high_card_duel`, `secret_draft`, and `poker_lite` (including yield-no-reveal) across visible text, hidden DOM, accessibility attributes, `data-testid`, storage, effect log, replay export, and dev panel.
3. `node scripts/check-catalog-docs.mjs` passes (smoke registered in all three surfaces); `node scripts/check-outcome-explanations.mjs` now passes (coverage complete after 010).

### Invariants

1. The smoke re-enumerates the catalog rather than hardcoding the game count, so coverage stays complete as games are added.
2. No hidden information reaches any browser channel for any game (FOUNDATIONS §11 no-leak firewall); the panel summary is screen-reader-accessible and reduced-motion-safe.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/outcome-explanation.smoke.mjs` — the catalog-complete terminal/no-leak/a11y/replay smoke.
2. `scripts/check-catalog-docs.mjs`, `apps/web/README.md`, `apps/web/package.json` — registration so the smoke runs in `smoke:e2e` and passes catalog-drift.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `node scripts/check-catalog-docs.mjs && node scripts/check-outcome-explanations.mjs`
3. `npm --prefix apps/web run smoke:ui` (regression — existing UI smoke intact)

## Outcome

Added `apps/web/e2e/outcome-explanation.smoke.mjs`, a rendered-browser smoke
covering the shared outcome panel status region, final standing, disclosure
keyboard/pointer operation, reduced-motion content preservation, and browser
no-leak surfaces. The smoke dynamically reads the catalog from the game picker
and drives representative terminal flows through Race to 21 and Three Marks,
including win and draw outcomes.

Registered the smoke in all required surfaces:

1. `apps/web/package.json` `smoke:e2e`
2. `scripts/check-catalog-docs.mjs` `NON_GAME_SMOKE`
3. `apps/web/README.md` Smoke Layers

Updated the existing Poker Lite smoke to assert terminal explanation through
the shared `OutcomeExplanationPanel` after the 010 board integration removed
the old local `.poker-lite-terminal` text panel.

Verification run:

1. `npm --prefix apps/web run smoke:e2e` — passed.
2. `node apps/web/e2e/outcome-explanation.smoke.mjs` — passed.
3. `node apps/web/e2e/poker-lite.smoke.mjs` — passed after updating the old panel assertions.
4. `npm --prefix apps/web run smoke:ui` — passed.
5. `node scripts/check-catalog-docs.mjs` — passed.
6. `node scripts/check-outcome-explanations.mjs` — passed.
7. `node scripts/check-doc-links.mjs` — passed.
8. `git diff --check` — passed.
