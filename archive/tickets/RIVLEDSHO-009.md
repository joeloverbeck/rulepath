# RIVLEDSHO-009: Seat and turn-flow affordances

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/RiverLedgerBoard.tsx`
**Deps**: RIVLEDSHO-006

## Problem

Seat roles, the active seat, and the street progression are hard to scan. This ticket adds button/small-blind/big-blind and active-seat affordances by text + icon (not color alone), a `Preflop → Flop → Turn → River → Showdown` street strip from public state, and a reduced-motion-aware board reveal that rides the existing semantic effects (spec WB9 / D7).

## Assumption Reassessment (2026-06-15)

1. Verified against current code: `RiverLedgerBoard.tsx` renders seats and the public board from the viewer-safe public view fields (`phase`, `street`, `active_seat`, `pending_seats`, `seats[]` roles/statuses per `games/river_ledger/docs/UI.md` §Board Layout); no new Rust fields are needed.
2. Verified against specs/docs: spec §6 D7 + §8 WB9; `games/river_ledger/docs/UI.md` §Board Layout + §Accessibility And Motion (reduced-motion preserves all facts; color not the only channel).
3. Cross-artifact boundary under audit: the affordances consume existing public street/seat view fields and existing semantic effects; presentation-only, no new payload.
4. FOUNDATIONS §7 (play-first, polished, accessible) motivates this ticket: state is conveyed by text + icon + focus, not color alone.
5. §11 (semantic effects drive animation; renderer diffs are diagnostics only) is the enforcement surface: the board reveal is driven by Rust-emitted semantic effects and must not preload hidden future board cards into DOM, accessibility labels, or `data-testid`s; after animation the renderer settles to the latest viewer-safe public view.

## Architecture Check

1. Driving the street strip and reveal from public street state + existing semantic effects (not DOM-inferred ordering) keeps Rust the causal authority and avoids guessed-state-diff animation.
2. No backwards-compatibility aliasing/shims; presentation-only affordance additions.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4) — `apps/web` presentation only.

## Verification Layers

1. Button/blind/active states render by text + icon (not color alone); street strip highlights the current street from public state -> `npm --prefix apps/web run smoke:ui`.
2. Board reveal rides semantic effects and preloads no hidden future board into DOM/a11y/test-id -> `node apps/web/e2e/river-ledger.smoke.mjs` no-leak sweep + `npm --prefix apps/web run smoke:effects`.
3. Reduced-motion preserves all facts via text/status -> manual accessibility review (§7).

## What to Change

### 1. `apps/web/src/components/RiverLedgerBoard.tsx`

Add seat role/active affordances (text + icon + focus state), a public-state-driven street strip, and a reduced-motion-aware board reveal driven by the existing semantic effects; settle to the latest public view after animation.

## Files to Touch

- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)

## Out of Scope

- Action-panel cost copy (RIVLEDSHO-008).
- Card component visuals (RIVLEDSHO-006).
- Casino-vocabulary copy audit (RIVLEDSHO-011).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` — seat role/active affordances and the street strip render from public state.
2. `npm --prefix apps/web run smoke:effects` — board reveal is driven by semantic effects.
3. `node apps/web/e2e/river-ledger.smoke.mjs` — no hidden future board card appears in DOM/a11y/test-id during reveal.

### Invariants

1. State is conveyed by text + icon + focus, not color alone (§7, WCAG 1.4.1).
2. Animation is driven by Rust semantic effects; no hidden future board is preloaded; the renderer settles to the latest public view (§11).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` (modify, as surfaced) — seat/street-strip affordance assertions.
2. `apps/web/e2e/river-ledger.smoke.mjs` (modify, as surfaced) — reveal no-leak assertion.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:effects`
3. The UI + effects smokes plus the e2e reveal no-leak sweep are the correct boundary; no Rust behavior changes.

## Outcome

Completed: 2026-06-15

Changes:
- Added a public-state street strip for `Preflop -> Flop -> Turn -> River -> Showdown`, with `aria-current="step"` on the current street and text/symbol state for completed/current/upcoming steps.
- Expanded seat affordances to include text plus visible symbols for active seat, button, small blind, and big blind.
- Kept board reveal on the existing semantic-effect-driven `.reveal` path with reduced-motion gating already present in the board class/CSS.
- Extended River Ledger e2e assertions for seat markers, street strip during play, terminal showdown strip state, and no-leak browser surface.
- Updated `smoke-effect-feedback.mjs` to start River Ledger through the existing WASM seat-count entrypoint so `smoke:effects` can include the game without setup failure.

Verification:
- `npm --prefix apps/web run build`
- `node apps/web/e2e/river-ledger.smoke.mjs`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:e2e`
- `cargo fmt --all --check`
- `git diff --check`

Notes:
- `apps/web/scripts/smoke-ui.mjs` remained unchanged because it does not mount DOM seat/street affordances. The DOM assertions live in `river-ledger.smoke.mjs`, and the required `smoke:ui` command still passed.
