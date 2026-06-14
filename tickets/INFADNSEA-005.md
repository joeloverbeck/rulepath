# INFADNSEA-005: Infra C — shared multi-seat shell frame component

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/SeatFrame.tsx` (new), `apps/web/src/state/shellReducer.ts`
**Deps**: INFADNSEA-002

## Problem

The web shell has no shared frame for presenting N-seat structure: each board hand-rolls seat display, and there is no common surface for the seat rail, active/pending-seat indication, observer mode, and viewer selection. Gate 15 (3–6 seats) needs one. This ticket adds a shared `SeatFrame` component plus presentation-only viewer/active-seat selection state in `shellReducer`, rendering only Rust/WASM-projected active/pending/turn-order/viewer state — no TypeScript turn-order or legality inference.

## Assumption Reassessment (2026-06-14)

1. `apps/web/src/state/shellReducer.ts` and `apps/web/src/components/AppShell.tsx` exist; there is no shared seat-frame component today (`apps/web/src/components/` has per-game `*Board.tsx` but no `SeatFrame.tsx`). Seat metadata types come from `apps/web/src/wasm/client.ts` (INFADNSEA-002).
2. Grounded in `specs/infra-a-d-n-seat-public-infrastructure.md` WB5, and `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md §4` (turn-order model: current active seat, active set, pending responder set, pass/wait obligations — all Rust-projected) and §5 (viewer matrix).
3. Shared boundary under audit: the public/private view projection the frame renders — the frame must consume Rust-projected active/pending/viewer fields and never infer who can act from seat index, DOM, or labels.
4. FOUNDATIONS §2 + §12 ("TypeScript decides legality" / turn-order inference): the frame visualizes projected active/pending data only; viewer selection is presentation state (which seat's authorized view to display), not a legality or turn-order decision. Component name `SeatFrame` is indicative per spec A6.

## Architecture Check

1. One shared frame is cleaner than per-board seat rendering: it centralizes the viewer-safe seat presentation so every game (and Gate 15) inherits identical, audited behavior rather than re-implementing it.
2. No backwards-compat shim: a new component + additive reducer state; existing boards are unaffected until they adopt it (INFADNSEA-006).
3. No `engine-core`/`game-stdlib` impact (web-only). Behavior authority stays in Rust — the frame renders projected state and holds only presentation (viewer-selection) state.

## Verification Layers

1. Frame renders seat rail / active / pending / observer from projected state -> `smoke:ui` exercises the frame against a current game's projected view.
2. Viewer selection switches the displayed authorized view without leaking another seat's private data -> no-leak assertion deferred to INFADNSEA-008's harness; here, `smoke:ui` confirms the selector drives only which projected view is shown.
3. No TS turn-order/legality inference -> FOUNDATIONS §2/§12 alignment check (grep the component for any seat-index-derived "can act" logic; there must be none).

## What to Change

### 1. Add the `SeatFrame` component

`apps/web/src/components/SeatFrame.tsx`: renders the seat rail (labels from catalog metadata), active-seat and pending-responder indicators, observer mode, and a viewer selector — all from Rust/WASM-projected fields.

### 2. Presentation-only viewer/active state

`apps/web/src/state/shellReducer.ts`: add presentation state for the selected viewer (which authorized seat view to show) and observer toggle; no legality or turn-order state.

## Files to Touch

- `apps/web/src/components/SeatFrame.tsx` (new)
- `apps/web/src/state/shellReducer.ts` (modify)

## Out of Scope

- Adopting the frame across existing boards / replay / observer wiring (INFADNSEA-006).
- The no-leak harness and its assertions (INFADNSEA-007/008).
- Any Rust view-projection change — the frame consumes existing projected fields.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` — the `SeatFrame` renders seat rail / active / pending / observer / viewer selector from projected state; no console errors.
2. `npm --prefix apps/web run build` — type-check passes against the INFADNSEA-002 seat metadata types.

### Invariants

1. The frame renders only Rust/WASM-projected active/pending/turn-order/viewer state; it infers no actor from seat index, DOM, or labels (§2/§12).
2. Reducer additions are presentation-only (viewer selection / observer toggle); no legality or turn-order state enters TypeScript.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` — extend to mount and exercise `SeatFrame`.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run build`
