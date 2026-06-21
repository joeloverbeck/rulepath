# SEAT-004: Migrate the VIEWER panel and viewer-mode selectors to the shared resolver

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `apps/web` only (`SeatFrame.tsx` and per-board viewer-mode option lists)
**Deps**: SEAT-003

## Problem

The VIEWER panel (`apps/web/src/components/SeatFrame.tsx`) resolves seat labels
inline and ends in a hardcoded 0-based fallback (`SeatFrame.tsx:97-98`,
`"Seat 0"/"Seat 1"`). Some boards also build their own viewer-mode option lists
with hardcoded labels (e.g. `HighCardDuelBoard.tsx:28-29` `VIEWER_OPTIONS`). These
are the surfaces that show "Seat 0".."Seat 3" today. They must consume the shared
resolver (SEAT-003) so the VIEWER reflects the Rust-supplied labels (1-based default
from SEAT-001; "Player N" for Race/Directional from SEAT-002).

## Assumption Reassessment (2026-06-21)

1. `SeatFrame.tsx:85-98` resolves `active_seat_labels` → `game.seat_labels`/`ui.seat_labels`
   → hardcoded `["Seat 0","Seat 1"]`. The hardcoded branch is the only TS-invented
   label here and must be replaced by `resolveSeatLabels` (SEAT-003). Confirmed.
2. Per-board viewer selectors with hardcoded labels: confirm the full set at
   implementation time via
   `grep -rn "VIEWER_OPTIONS\|ViewerMode" apps/web/src/components` and route each
   label through `resolveSeatLabel`. Known instance: `HighCardDuelBoard.tsx:28-29`.
3. Shared boundary: the `SeatDisplayLabel` `{seat,label}` contract consumed by
   SeatFrame and viewer-mode lists; resolver from SEAT-003 is the single seam.
8. Adjacent: `ModeControls.tsx` may also render seat-scoped viewer labels — verify
   and migrate if it formats a seat label (otherwise leave to SEAT-005 if it is a
   play-area label).

## Architecture Check

1. Routing the VIEWER through the resolver removes the last TS-invented seat label
   on the viewer surface; the panel becomes a pure presenter of Rust labels.
2. No backwards-compatibility shim: the hardcoded fallback array is deleted, not
   kept as a secondary path.
3. `apps/web` presentation only; no Rust/legality change.

## Verification Layers

1. VIEWER buttons use Rust-supplied labels for every game -> UI smoke
   (`npm --prefix apps/web run smoke:ui` after migration; Briar shows "Seat 1".."Seat 4").
2. No hardcoded seat-label literal remains in the viewer surfaces -> codebase
   grep-proof (`grep -rn '"Seat 0"\|"Seat 1"\|"Player 1"' apps/web/src/components/SeatFrame.tsx`
   and migrated viewer-option lists return nothing).
3. Resolver wiring compiles and renders -> build (`npm --prefix apps/web run build`).

## What to Change

### 1. SeatFrame

Replace the inline resolution + hardcoded fallback in `SeatFrame.tsx` with
`resolveSeatLabels(...)` from SEAT-003, passing the view payload and game catalog
sources already in scope.

### 2. Per-board viewer-mode option lists

Replace hardcoded labels in board-local viewer-mode lists (e.g.
`HighCardDuelBoard.tsx` `VIEWER_OPTIONS`) with `resolveSeatLabel` calls; enumerate
the full set via grep first.

## Files to Touch

- `apps/web/src/components/SeatFrame.tsx` (modify)
- `apps/web/src/components/HighCardDuelBoard.tsx` (modify — `VIEWER_OPTIONS`)
- additional viewer-mode option lists found by grep (modify)

## Out of Scope

- Play-area `seatLabel()` helpers inside boards (SEAT-005).
- Any Rust change.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run build`

### Invariants

1. The VIEWER panel renders only Rust-supplied labels.
2. No hardcoded seat-label literal remains on viewer surfaces.

## Test Plan

### New/Modified Tests

1. Existing UI smoke updated/extended to assert VIEWER labels match the catalog
   `seat_labels` for at least one multi-seat game (Briar Circuit: "Seat 1".."Seat 4").

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run build`
