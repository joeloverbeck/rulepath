# GAT4THRMARBOA-011: Web ThreeMarksBoard renderer (board-first, accessible)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — new `apps/web/src/components/ThreeMarksBoard.tsx`; `apps/web/src/components/AppShell.tsx`, `apps/web/src/styles.css`, `apps/web/src/components/effectFeedback.ts`
**Deps**: GAT4THRMARBOA-010

## Problem

Gate 4's product proof is a polished, board-first, accessible 3×3 renderer. It must render Rust-provided cells/marks/legal-targets, dispatch Rust placement actions on click/tap/keyboard, show current-player/terminal/draw banners and the Rust-provided winning line, keep occupied cells inert, present a concise effect log and a Level-1 bot explanation, keep the dev panel secondary, and honour reduced motion — all without TypeScript deciding any rule.

## Assumption Reassessment (2026-06-06)

1. `apps/web/src/components/RaceBoard.tsx` is the renderer mirror; `AppShell.tsx` selects the active renderer; `EffectLog.tsx` + `effectFeedback.ts` render semantic effects; `DevPanel.tsx` is the secondary dev surface; `styles.css` holds shell styling. Verified all exist. `ThreeMarksBoard.tsx` is new and selected by the game-id/view discriminant from GAT4THRMARBOA-010.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §12 (normal-play experience, visual direction, TS responsibility limits), §18 (accessibility + reduced motion), §10/§14 (effects + bot explanation), Appendix D (browser acceptance checklist). The Rust public view (legal targets, occupancy, owner, winning line, draw — GAT4THRMARBOA-005) and effects (004) are the only inputs.
3. Cross-artifact boundary under audit: the Rust public-view/effect contract consumed by the renderer (`docs/UI-INTERACTION.md`, `docs/WASM-CLIENT-BOUNDARY.md`) — the renderer maps Rust data and dispatches Rust action ids; it never validates or simulates the game.
4. FOUNDATIONS §7 (public UI is central product work; legal-only, semantic-effect-driven animation, cozy/tactile/original) and §2/§11 (TypeScript decides no legality; renderer settles to the latest viewer-safe view) motivate this ticket: legal empty cells come only from Rust legal targets; occupied/illegal cells are inert; animation is scheduled from Rust effects, not guessed diffs.
5. No-leak firewall enforcement surface (§11): the DOM, UI test ids, effect log, and bot-explanation surfaces are the firewall — name them. Three Marks is perfect-information, so the renderer exposes only public-view data; the Level-1 bot explanation shown is the Rust-provided viewer-safe string (no candidate-ranking or hidden-state leak), and reduced-motion mode disables/simplifies animation without blocking play.

## Architecture Check

1. A dedicated SVG board component consuming the Rust view + effects is the cleanest play-first surface and keeps the generic action list out of normal play (dev/debug only) per spec §12. Alternative (reusing the generic `ActionControls` list as primary UI) is debug-first and rejected by §7/§12.
2. No backwards-compatibility aliasing/shims — new component; `RaceBoard` untouched; renderer chosen by discriminant.
3. `engine-core` untouched; no `game-stdlib` change; TypeScript adds no rule logic — disabled cells are disabled because Rust did not provide them as legal targets (or the view is terminal), not because TS decided.

## Verification Layers

1. Legal-only interaction invariant -> UI smoke + FOUNDATIONS alignment (§7/§11: only Rust legal empty cells are clickable; occupied cells inert; clicking dispatches a Rust action and the board settles to the new Rust view).
2. Terminal/win/draw presentation invariant -> UI smoke (terminal banner, Rust-provided winning-line highlight, draw presentation render from the view).
3. Accessibility invariant -> UI smoke + manual review (keyboard focus/activation of legal cells, accessible names, screen-reader summary derived from Rust view, color-plus-shape marks, visible focus, reduced motion).
4. Animation-causality invariant -> manual review + FOUNDATIONS alignment (§11: placement/turn/win animations are scheduled from Rust semantic effects, not renderer diffs).

## What to Change

### 1. `ThreeMarksBoard.tsx` (new)

Polished 3×3 SVG board: render Rust cells with occupancy/owner and original Rulepath mark tokens (color **plus** shape); highlight Rust legal targets; make occupied cells inert; dispatch the Rust placement action id + freshness token on click/tap/Enter/Space; current-player/status banner, terminal banner, Rust winning-line highlight, draw presentation; concise effect log; Level-1 bot-explanation affordance; keyboard focus + arrow navigation (roving focus or semantic buttons) with visible focus and accessible names; screen-reader board/legal-move summary; reduced-motion support.

### 2. `AppShell.tsx`, `styles.css`, `effectFeedback.ts`

Select `ThreeMarksBoard` by discriminant; keep the dev panel / generic action list secondary in normal play; add board styling (tactile, restrained, original tokens, sufficient contrast, no color-only meaning); map Three Marks semantic effects to board feedback.

## Files to Touch

- `apps/web/src/components/ThreeMarksBoard.tsx` (new)
- `apps/web/src/components/AppShell.tsx` (modify)
- `apps/web/src/styles.css` (modify)
- `apps/web/src/components/effectFeedback.ts` (modify)

## Out of Scope

- Board-aware *replay* view + controls (GAT4THRMARBOA-012).
- Browser smoke test authoring (GAT4THRMARBOA-013) — this ticket builds the surface the smoke exercises.
- Original SVG asset *licensing/source notes* (covered in SOURCES/UI docs, 001/015); assets created here must be project-owned.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — type-checks and builds.
2. `npm --prefix apps/web run smoke:ui` — board renders nine cells with mark/status affordances; a legal cell dispatches a Rust action and the board updates.
3. Reduced-motion and keyboard focus paths are reachable (verified by 013's smoke; manual checklist per Appendix D otherwise).

### Invariants

1. Only Rust-provided legal empty cells are interactive; occupied/illegal cells are inert; the renderer decides no legality and settles to the latest Rust view.
2. Marks use color-plus-shape original tokens; winning line/terminal/draw come from Rust; animation is effect-driven and reduced-motion-safe.

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/ThreeMarksBoard.tsx` — board renderer (exercised by `smoke:ui`/`smoke:e2e`, asserted in 013).

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. Full board/click/win/draw/keyboard/reduced-motion assertions live in the 013 browser smoke; build + render smoke is the correct boundary for the renderer diff.

## Outcome

Completed: 2026-06-06

Changes:
- Added `ThreeMarksBoard.tsx`, a board-first 3x3 renderer driven by the Rust Three Marks public view.
- Rendered Rust-provided cell occupancy, legal targets, active-seat/status/freshness, winning-line state, and bot-choice effect explanations without adding TS legality.
- Wired `main.tsx` to select the Three Marks renderer by view discriminant while keeping Race-to-N on `RaceBoard`.
- Added fixed-grid board styling, visible legal targets, occupied/inert states, color-plus-shape marks, winning-cell emphasis, screen-reader summary, and reduced-motion-safe transitions.
- Extended Three Marks semantic effect feedback in the existing effect log.

Deviations:
- `AppShell.tsx` did not need a code change; active renderer selection already belongs to `main.tsx`, where the play surface is composed.
- Browser smoke authoring remains in GAT4THRMARBOA-013, but this ticket used a one-off local Puppeteer probe to verify the new board renders nine cells and dispatches a Rust legal placement.

Verification:
- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:e2e`
- One-off local Puppeteer probe: selected Three Marks, started a match, confirmed 9 board cells, clicked `r1c1`, and confirmed `render_game_to_text()` still reported `game: "three_marks"` after the Rust view update.
