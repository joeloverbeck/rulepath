# CATSETVIS-008: Smoke + a11y verification (`description?` assertion + full sweep)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/scripts/smoke-ui.mjs`
**Deps**: 004, 006, 007

## Problem

The redesigned catalog/setup and the new `description?` projection need an enforced shape gate and an accessibility/no-leak sweep. This ticket extends `apps/web/scripts/smoke-ui.mjs` with the variant-`description` shape assertion (a positive described multi-variant case and a negative omission case; length ≤120; no behavior-like prose) and runs the full verification sweep (`smoke:wasm`/`ui`/`effects`/`e2e`) confirming no raw IDs, no engine/debug normal-mode copy, focus-visible, and reduced-motion. It is the §Deliverable-doubles-as-capstone verification ticket (it ships smoke infra). Spec WB8 / §6 D11–D12; spec §9 exit criteria 6–7.

## Assumption Reassessment (2026-06-13)

1. `apps/web/scripts/smoke-ui.mjs` uses a custom `assert(...)` helper (no `node:test`) with `hasVariant(game, id, label)` at line 53 and **no** `description` assertion — verified this session. **Change rationale:** §6 D11 adds the description shape assertion; the existing custom-assert node-smoke convention is the test vehicle (no new runner). D12 keeps `smoke-effect-feedback.mjs` untouched.
2. Spec §6 D11 (a positive described-variant assertion on a multi-variant game + a negative omission assertion; presence/absence, length ≤120, no behavior token `/\b(if|when|then|selector|trigger|valid_if|legal|effect|action)\b/i`) and D12 (`smoke-effect-feedback.mjs` stays green/untouched) govern; spec §9 exit criteria 6–7 are this ticket's acceptance surface.
3. Cross-artifact boundary: this ticket depends on the projected `description` (CATSETVIS-006) and the rendered card/setup (CATSETVIS-004/007) being present so the assertion and the no-raw-ID / focus / reduced-motion checks pass. The positive target is a described variant on one of `flood_watch`/`frontier_control`/`event_frontier`; the negative target is a game intentionally left without a description.
4. FOUNDATIONS §11 (acceptance invariants): the smoke is the fail-closed **shape** gate (it enforces ≤120 and no behavior token, and that an absent description is structurally absent), and the `smoke:e2e` no-leak/a11y suite enforces no hidden state and no raw IDs reaching the DOM.

## Architecture Check

1. Extending the existing custom-assert node-smoke keeps the test vehicle consistent (no new test runner introduced); the positive + negative pair proves both the projection and the optional-omission contract in one place.
2. No backwards-compatibility shims; the assertion is additive to `hasVariant` (or a sibling helper).
3. `engine-core` / `game-stdlib` untouched; this is web test infrastructure only.

## Verification Layers

1. `description` shape (present / absent / ≤120 / no behavior token) → the new `smoke:ui` assertion (this ticket's deliverable).
2. No raw IDs / no engine vocabulary in normal-mode DOM → `smoke:e2e` (`a11y-noleak.smoke.mjs`).
3. Focus-visible + reduced-motion equivalence → manual review + the smoke screenshot/observation set (feeds the CATSETVIS-009 acceptance evidence).
4. Full web sweep green, effects untouched → `smoke:wasm` + `smoke:ui` + `smoke:effects` + `smoke:e2e`.

## What to Change

### 1. `smoke-ui.mjs` description assertion

Extend `hasVariant` (or add a sibling assertion) to cover: when `description` is absent, the property is absent; when present, it is a non-empty string ≤120 chars with no behavior token. Add one positive assertion on an authored described multi-variant variant and one negative omission assertion on an undescribed game.

### 2. Run the verification sweep

Run `smoke:wasm`/`ui`/`effects`/`e2e` and confirm no raw `game_id`/engine copy in normal mode, focus-visible present, and reduced-motion equivalence (capture the screenshot set for CATSETVIS-009).

## Files to Touch

- `apps/web/scripts/smoke-ui.mjs` (modify)

## Out of Scope

- `apps/web/scripts/smoke-effect-feedback.mjs` (untouched except incidental build compatibility — §6 D12).
- The card/setup rendering (CATSETVIS-004/007) and the `description` projection (CATSETVIS-006).
- The doc lift, IP closeout table, and `specs/README.md` Done-flip (CATSETVIS-009).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` is green, including the new positive (described multi-variant) and negative (omission) `description` assertions.
2. `npm --prefix apps/web run smoke:wasm`, `smoke:effects`, and `smoke:e2e` are green; `smoke-effect-feedback.mjs` output is unchanged.
3. `smoke:e2e` confirms no raw `game_id` / engine-debug copy in normal-mode DOM; focus-visible and reduced-motion observations recorded.

### Invariants

1. The description assertion enforces ≤120 chars, no behavior token, and structural omission when absent.
2. `smoke-effect-feedback.mjs` is untouched (save incidental build compatibility).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` — variant-`description` shape assertions: one positive (described multi-variant) and one negative (omission), with length and no-behavior-token checks.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:effects && npm --prefix apps/web run smoke:e2e`
3. Manual focus-visible + reduced-motion check and screenshot capture (desktop/mobile catalog, selected setup, multi-variant setup, focus-visible card/control) — the UI-observation portion that the node smokes cannot assert, recorded for CATSETVIS-009.

## Outcome

Completed 2026-06-13.

- Added `assertVariantDescription(...)` to `apps/web/scripts/smoke-ui.mjs`, covering the described `flood_watch_standard` multi-variant case and the structurally omitted `token_bazaar_standard` case.
- The smoke assertion enforces non-empty string descriptions, the 120-character ceiling, the behavior-token guard, and absence of the `description` property when no Rust-authored description exists.
- Confirmed `apps/web/scripts/smoke-effect-feedback.mjs` remained untouched.

Verification:

- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:e2e`
