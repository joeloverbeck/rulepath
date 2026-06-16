# RIVLEDSHOWUX-013: Route board reveal + staged showdown pacing through the shared scheduler

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/RiverLedgerBoard.tsx`, `apps/web/src/animation/scheduler.ts`, `apps/web/src/animation/registry.ts`, `apps/web/src/animation/bursts.ts`, `apps/web/src/animation/presenters.ts`, `apps/web/src/animation/settleAssertion.ts`
**Deps**: RIVLEDSHOWUX-011, RIVLEDSHOWUX-012

## Problem

Board reveal rides the existing reduced-motion path (shipped by RIVLEDSHO), but the showdown is not staged through one scheduler burst. Route board reveal and a staged showdown burst (banner → board usage highlights → standings settle) through the shared scheduler with a reduced-motion path and settle assertions — no ad-hoc timers, no hidden future board in DOM/a11y/test-IDs/animation ghosts.

## Assumption Reassessment (2026-06-16)

1. Verified: the shared scheduler lives in `apps/web/src/animation/{scheduler,registry,bursts,presenters,settleAssertion}.ts`; River Ledger board reveal already uses reduced-motion-aware effects; no staged showdown burst exists.
2. Verified against spec §6 D8 + §8 WB13 (#12); `RULES.md` `RL-UI-SHOWDOWN-001`; `docs/UI-INTERACTION.md` (semantic effects drive animation).
3. Shared boundary under audit: the shared animation scheduler — River pacing must ride the existing burst/settle primitives and reduced-motion path, adding no component-local timers.
4. FOUNDATIONS §11 (semantic effects drive animation; renderer diffs are diagnostics only; reduced motion preserves all facts; no future-card preload) + §7 motivate this ticket.

## Architecture Check

1. Driving the staged showdown through the existing scheduler burst (vs component-local timers) keeps animation effect-driven and reduced-motion-safe, and lets the settle assertion guard the final viewer-safe view.
2. No shims; reveal pacing is expressed as scheduler bursts, not ad-hoc `setTimeout`.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); presentation only.

## Verification Layers

1. Board reveal + staged showdown run as scheduler bursts; renderer settles to the latest viewer-safe view -> `npm --prefix apps/web run smoke:effects` (settle assertion).
2. Reduced motion preserves all reveal facts -> `npm --prefix apps/web run smoke:effects` reduced-motion path + manual review.
3. No future-board preload into DOM/a11y/test-IDs/ghosts -> `node apps/web/e2e/a11y-noleak.smoke.mjs`.

## What to Change

### 1. `apps/web/src/animation/*` + `apps/web/src/components/RiverLedgerBoard.tsx`

Register a River Ledger staged-showdown burst (banner → board usage highlights → standings settle) and route board reveal through the shared scheduler with the reduced-motion path; add settle assertions. No component-local timers; no hidden future board.

## Files to Touch

- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)
- `apps/web/src/animation/scheduler.ts` (modify)
- `apps/web/src/animation/registry.ts` (modify)
- `apps/web/src/animation/bursts.ts` (modify)
- `apps/web/src/animation/presenters.ts` (modify)
- `apps/web/src/animation/settleAssertion.ts` (modify)

## Out of Scope

- Table layout (RIVLEDSHOWUX-011); tokens (RIVLEDSHOWUX-012); the V2 renderer DOM (RIVLEDSHOWUX-009).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:effects` — board reveal + staged showdown run as scheduler bursts; settle assertions pass.
2. `npm --prefix apps/web run smoke:effects` (reduced motion) — all reveal facts visible without animation.
3. `node apps/web/e2e/a11y-noleak.smoke.mjs` — no future-board preload in DOM/a11y/test-IDs.

### Invariants

1. Animation is driven by semantic effects through the shared scheduler; no ad-hoc timers; renderer settles to the latest viewer-safe view (§11).
2. No hidden future board reaches any viewer surface during reveal (§11 no-leak).

## Test Plan

### New/Modified Tests

1. `apps/web/src/animation/settleAssertion.ts` (modify, as surfaced) — River staged-showdown settle assertion.

### Commands

1. `npm --prefix apps/web run smoke:effects`
2. `node apps/web/e2e/a11y-noleak.smoke.mjs`
3. `npm --prefix apps/web run smoke:ui`

## Outcome

Completed on 2026-06-16.

- Restored River Ledger semantic action effects from Rust-owned state transitions: contribution changes, street/board reveals, and terminal showdown/foldout resolution now flow through the WASM effect log.
- Serialized River effects with structured `payload.type` fields and public seat labels so the shared web scheduler and feedback renderer can consume them without raw-seat public copy.
- Registered River Ledger scheduler presenters for board reveal and staged showdown surfaces using the shared animation registry/presenters, with no component-local timers.
- Added public animation targets for the board reveal, status, V2 showdown banner, board usage, and ranked standings; extended settle assertions for River showdown staged surfaces.
- Expanded `smoke:effects` to include the browser animation/settle smoke and added River Ledger staged-showdown target assertions.
- Verified with `npm --prefix apps/web run smoke:effects`, `node apps/web/e2e/a11y-noleak.smoke.mjs`, `npm --prefix apps/web run smoke:ui`, `cargo test -p river_ledger`, and `cargo test -p wasm-api`.
- Additional check: `cargo clippy -p river_ledger -p wasm-api --all-targets -- -D warnings` was attempted but is blocked by existing River Ledger lint debt (`large_enum_variant` in terminal/showdown enums and `filter_map_bool_then` in `ui.rs`), unrelated to the scheduler wiring.
