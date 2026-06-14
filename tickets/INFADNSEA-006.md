# INFADNSEA-006: Infra C — adopt the seat frame across boards + replay/observer

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/AppShell.tsx`, `apps/web/src/components/ReplayViewer.tsx`, per-game `apps/web/src/components/*Board.tsx` (as surfaced)
**Deps**: INFADNSEA-005

## Problem

The shared `SeatFrame` (INFADNSEA-005) must be adopted by the shell so seat presentation is uniform: `AppShell` mounts the frame, `ReplayViewer` uses it for observer/viewer selection during replay, and each existing board either adopts it or records a board-native exception (a board whose own layout already presents seats better). This proves the frame works across the catalog before Gate 15 relies on it.

## Assumption Reassessment (2026-06-14)

1. `apps/web/src/components/AppShell.tsx` and `apps/web/src/components/ReplayViewer.tsx` exist; `apps/web/src/components/` holds 14 `*Board.tsx` files (the adoption-or-exception set, resolved "as surfaced" by `smoke:ui`/`smoke:e2e`). `SeatFrame` is created in INFADNSEA-005.
2. Grounded in `specs/infra-a-d-n-seat-public-infrastructure.md` WB6, and `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md §4`/§7 (active/pending seats, observer mode are Rust-projected).
3. Shared boundary under audit: each board's seat presentation versus the shared frame — adoption must not change what view a board renders, only how seats are framed; a board-native exception must be recorded with a one-line reason.
4. FOUNDATIONS §2 + §12: adopting the frame must not move any legality/turn-order decision into the boards; they continue rendering Rust-projected state.

## Architecture Check

1. Adopting one frame (or recording a justified board-native exception) is cleaner than leaving each board's ad-hoc seat display: it removes drift and gives Gate 15 a proven surface.
2. No backwards-compat shim: boards switch to the frame; no parallel old-and-new seat code is retained.
3. No `engine-core`/`game-stdlib` impact; behavior authority unchanged (§2).

## Verification Layers

1. Shell + replay use the frame -> `smoke:ui` and `smoke:e2e` exercise `AppShell`/`ReplayViewer` with the frame mounted.
2. Each board adopts or has a recorded exception -> an adoption-audit note (one row per board: adopt / board-native exception + reason) in the ticket's deliverable; `smoke:e2e` per-game smokes stay green.
3. No behavior change -> existing per-game `e2e/*.smoke.mjs` pass unchanged (the frame reframes seats, it does not alter rendered views).

## What to Change

### 1. Mount the frame in the shell and replay viewer

`AppShell.tsx` mounts `SeatFrame` for live matches; `ReplayViewer.tsx` uses it for observer/viewer selection during replay.

### 2. Per-board adoption audit

For each of the 14 `*Board.tsx`, adopt the frame or record a board-native exception with a one-line reason; apply only the adoptions (board-native boards are left as-is, exception recorded).

## Files to Touch

- `apps/web/src/components/AppShell.tsx` (modify)
- `apps/web/src/components/ReplayViewer.tsx` (modify)
- `apps/web/src/components/*Board.tsx` (modify; as surfaced by the adoption audit — `apps/web/src/components/` is the verified parent dir, 14 candidates)

## Out of Scope

- The `SeatFrame` component itself (INFADNSEA-005).
- No-leak harness assertions (INFADNSEA-007/008).
- Any Rust view-projection change.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` — shell mounts the frame; no console errors.
2. `npm --prefix apps/web run smoke:e2e` — all per-game smokes pass unchanged; replay observer/viewer selection works through the frame.
3. `npm --prefix apps/web run build` — type-check passes.

### Invariants

1. Every board either adopts the frame or carries a recorded board-native exception (no silent skip).
2. No board moves a legality or turn-order decision into TypeScript (§2/§12); rendered views are unchanged.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` and `apps/web/e2e/*.smoke.mjs` — extend/confirm coverage of the frame in shell + replay.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:e2e`
