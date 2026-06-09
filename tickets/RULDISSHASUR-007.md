# RULDISSHASUR-007: E2E + accessibility + hidden-info no-leak smoke for the rules surface

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/e2e/rules-display.smoke.mjs` + `apps/web/package.json` smoke wiring; no Rust/engine/WASM behavior.
**Deps**: RULDISSHASUR-004, RULDISSHASUR-006

## Problem

The rules surface must be proven to work from all three contexts, meet the accessibility baseline, and — critically — never leak hidden information or mutate match state. This ticket adds the end-to-end smoke that exercises the panel against real generated assets and the wired access points; it doubles as the gate's verification harness. Source: `specs/rules-display-shared-surface.md` §10.3 (UI smoke), §10.4 (a11y smoke), §10.5 (hidden-info no-leak).

## Assumption Reassessment (2026-06-09)

1. `apps/web/e2e/` already holds `.mjs` smoke files chained by the `smoke:e2e` script in `apps/web/package.json` (nine files, including `a11y-noleak.smoke.mjs`); the new `rules-display.smoke.mjs` follows that runner/format and chains in. The hidden-information games are `poker_lite`, `secret_draft`, `high_card_duel`. The access points + mounted panel come from RULDISSHASUR-006; the generated assets from RULDISSHASUR-003/-004.
2. Coverage requirements are spec §10.3/§10.4/§10.5; the accessibility baseline is `docs/UI-INTERACTION.md` §16.
3. Cross-artifact boundary under audit: the smoke drives the built app across picker → setup → in-play, asserting the public-view JSON, replay export, and effect log are identical before and after opening rules. It consumes the assets (RULDISSHASUR-004) and the wired triggers (RULDISSHASUR-006).
4. FOUNDATIONS principle restated: §11 no-leak firewall — hidden information must not reach DOM text, `data-testid`, storage, logs, effect logs, replay export, or test screenshots via the rules panel; plus the §16 accessibility baseline.
5. No-leak enforcement surface (this ticket IS the negative test): for each hidden-info game, start a seeded match, capture the public/observer view before opening rules, open rules, assert the panel contains only static authored text, and assert replay export + effect log are byte-identical before/after — proving the panel introduces no private IDs, opponent private labels, hidden deck order, unrevealed choices, or seed-derived values. Deterministic via the seeded match.

## Architecture Check

1. A dedicated rules-display smoke that reuses the existing `e2e/*.smoke.mjs` harness keeps verification consistent with the project's UI-test convention; folding a11y + no-leak into the same suite (mirroring `a11y-noleak.smoke.mjs`) avoids a parallel harness.
2. No backwards-compatibility shims; the smoke and its script wiring are additive.
3. `engine-core`/`game-stdlib` untouched; the smoke only drives the presentation shell and inspects viewer-safe state.

## Verification Layers

1. Discoverability → every game card exposes a keyboard-focusable "How to Play" control (e2e assertion).
2. Correct content → panel heading matches the game display name; `At a glance`, `Actions`, `Scoring and winning` present; no rule-ID validation tables (e2e).
3. Accessibility → modal uses `role="dialog"` semantics, `Esc` closes, focus is trapped while modal and returns to the trigger on close, accessible names present, visible focus, no hover-only path (a11y e2e per `docs/UI-INTERACTION.md` §16).
4. No match mutation → opening rules leaves the public-view JSON and legal-action set unchanged (e2e diff).
5. No hidden-info leak → seeded-match negative test for `poker_lite`/`secret_draft`/`high_card_duel`: rules panel is static-only; replay export + effect log identical before/after; no private/seed-derived values appear.

## What to Change

### 1. `apps/web/e2e/rules-display.smoke.mjs` (new)

Load the app; confirm every card exposes a focusable "How to Play" control; open rules from the picker for a perfect-info game and for `poker_lite`; assert heading + `At a glance`/`Actions`/`Scoring and winning`; close and confirm focus returns to the trigger; select `poker_lite`, open from `MatchSetup`, close; start a `poker_lite` match, open in play, close; assert opening rules does not change active legal actions or the match view JSON; assert no console errors. Add accessibility assertions (accessible names, `role="dialog"`/`Esc`/focus containment, visible focus, no hover-only). Add hidden-info no-leak assertions for `poker_lite`, `secret_draft`, `high_card_duel` (capture view before, assert static-only panel, assert replay export + effect log unchanged).

### 2. `apps/web/package.json` (modify)

Chain `node e2e/rules-display.smoke.mjs` into the existing `smoke:e2e` script.

## Files to Touch

- `apps/web/e2e/rules-display.smoke.mjs` (new)
- `apps/web/package.json` (modify)

## Out of Scope

- The panel component, loader, state, styles (RULDISSHASUR-005) and access-point wiring (RULDISSHASUR-006).
- Authoring or generating player docs/assets (RULDISSHASUR-003/-004).
- Weakening, skipping, or narrowing any existing smoke (`docs/AGENT-DISCIPLINE.md` failing-test protocol applies).
- Any Rust/engine/WASM change.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` passes including the new `rules-display.smoke.mjs`.
2. The hidden-info no-leak assertions pass for `poker_lite`, `secret_draft`, and `high_card_duel` (replay export + effect log identical before/after opening rules).
3. `npm --prefix apps/web run smoke:ui` and the existing e2e smokes still pass (no existing gate weakened).

### Invariants

1. Opening rules never alters the match: public-view JSON, legal actions, replay export, and effect log are unchanged before vs. after (FOUNDATIONS §2/§11).
2. No hidden information reaches the DOM, `data-testid`, storage, logs, effect logs, replay exports, or screenshots via the rules panel (FOUNDATIONS §11 no-leak firewall).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/rules-display.smoke.mjs` — the e2e + a11y + hidden-info no-leak smoke for the rules surface.
2. `apps/web/package.json` — `smoke:e2e` chains the new smoke (no existing smoke removed).

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui`
3. `smoke:e2e` is the correct full-pipeline boundary (it drives the built app end-to-end); `cargo test --workspace` remains green because no Rust changes (regression evidence, not a feature boundary).
