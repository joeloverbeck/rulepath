# RIVLEDSHOWUX-010: Terminal result live-region for the showdown banner

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/OutcomeExplanationPanel.tsx`
**Deps**: RIVLEDSHOWUX-009

## Problem

The V2 showdown banner (RIVLEDSHOWUX-009) is rendered but not announced: screen-reader users get no atomic status announcement when the result appears. This ticket wraps the terminal result region in a `role="status"` / `aria-atomic` (or the repo-equivalent) live region, makes the detail expanders keyboard-accessible, and ensures reduced motion preserves every reveal fact.

## Assumption Reassessment (2026-06-16)

1. Verified: the V2 River branch lives in `OutcomeExplanationPanel.tsx` (RIVLEDSHOWUX-009); RIVLEDSHO shipped `best_five_accessibility_label` and group a11y labels but no live-region/atomic announcement for the banner.
2. Verified against spec §6 D7 + §8 WB10 (#9); W3C ARIA22 (`role=status`), `RULES.md` `RL-UI-SHOWDOWN-001`.
3. Shared boundary under audit: the shared `OutcomeExplanationPanel` — the live-region wrapper is scoped to the River V2 branch, so other games' panels are unaffected.
4. FOUNDATIONS §7 (accessible, play-first) + WCAG ARIA22 motivate this; the announcement text uses the Rust-authored `result_banner.accessibility_label`, not a TS-synthesized string (§2).

## Architecture Check

1. A `role="status"` atomic region around the Rust-authored banner label is the standard accessible-announcement pattern; reusing the Rust `accessibility_label` keeps the announced text Rust-owned.
2. No shims; the banner gains a live-region wrapper, no aliasing.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); TS presentation only.

## Verification Layers

1. The terminal banner is announced via an atomic status region -> `node apps/web/e2e/a11y-noleak.smoke.mjs` (role=status assertion).
2. Detail expanders are keyboard-accessible -> `npm --prefix apps/web run smoke:ui` (keyboard-focus assertion).
3. Reduced motion preserves all reveal facts -> manual reduced-motion review + `npm --prefix apps/web run smoke:effects`.

## What to Change

### 1. `apps/web/src/components/OutcomeExplanationPanel.tsx`

Wrap the V2 River result banner in a `role="status"` / `aria-atomic="true"` (or repo-equivalent) region announcing `result_banner.accessibility_label`; ensure detail expanders use accessible disclosure semantics and keyboard focus; confirm reduced motion leaves all facts visible.

## Files to Touch

- `apps/web/src/components/OutcomeExplanationPanel.tsx` (modify)

## Out of Scope

- The V2 renderer/card-usage layout itself (RIVLEDSHOWUX-009).
- Scheduler reveal pacing (RIVLEDSHOWUX-013).

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/a11y-noleak.smoke.mjs` — the terminal banner is announced via a status region; the announced text carries no raw id.
2. `npm --prefix apps/web run smoke:ui` + `npm --prefix apps/web run build` — expanders keyboard-accessible; type-checks.
3. `npm --prefix apps/web run smoke:effects` — reduced motion preserves the result facts.

### Invariants

1. The announced text is the Rust-authored `accessibility_label`; TS synthesizes no announcement (§2).
2. The live region is single-game-scoped; no other panel regresses (§7).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/a11y-noleak.smoke.mjs` (modify) — `role=status` atomic announcement assertion on the River banner.

### Commands

1. `node apps/web/e2e/a11y-noleak.smoke.mjs`
2. `npm --prefix apps/web run smoke:ui`
3. `npm --prefix apps/web run smoke:effects`

## Outcome

Completed on 2026-06-16.

- Added a River Ledger V2 showdown banner live region with `role="status"` and `aria-atomic="true"`, using the Rust-authored `result_banner.accessibility_label`.
- Added a focused River Ledger terminal flow to `a11y-noleak.smoke.mjs` asserting the atomic status region, human-facing announcement text, no raw seat ids, and reduced-motion-visible reveal facts.
- Updated outcome smoke expectations so the River Ledger V2 branch is the only outcome panel allowed to mount a scoped live/status region.
- Verified with `cargo fmt --all --check`, `npm --prefix apps/web run build`, `node apps/web/e2e/a11y-noleak.smoke.mjs`, `npm --prefix apps/web run smoke:ui`, `npm --prefix apps/web run smoke:effects`, and `node apps/web/e2e/outcome-explanation.smoke.mjs`.
