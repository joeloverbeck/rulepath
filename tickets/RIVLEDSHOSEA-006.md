# RIVLEDSHOSEA-006: Scope SeatFrame to active seats and make the viewer callback generic

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/main.tsx`, `apps/web/src/components/SeatFrame.tsx`, `apps/web/src/styles.css`
**Deps**: RIVLEDSHOSEA-005

## Problem

`apps/web/src/main.tsx::changeSeatFrameViewerMode()` contains a hardcoded two-seat allowlist — `if (viewerMode.seat === "seat_0" || viewerMode.seat === "seat_1")` — so clicks for `seat_2`..`seat_5` are silently discarded, while `SeatFrame` renders buttons for them from catalog metadata. `SeatFrame.catalogSeatLabels()` also feeds the full six-label catalog list to both the viewpoint row and seat rail. This ticket consumes the RIVLEDSHOSEA-005 active-seat projection, replaces the allowlist with a generic active-ID guard, normalizes stale selections fail-closed, and gives the selector accessible one-of-many semantics (spec §9.2 / D5).

## Assumption Reassessment (2026-06-18)

1. `apps/web/src/main.tsx::changeSeatFrameViewerMode(viewerMode)` forwards observer directly and otherwise gates on `viewerMode.seat === "seat_0" || viewerMode.seat === "seat_1"` before calling `changeViewerMode({ kind: "seat", seat })`. `ViewerMode = { kind: "observer" } | { kind: "seat"; seat: ViewerSeatId }` (`apps/web/src/wasm/client.ts`). Confirmed — the allowlist is the exact defect.
2. `apps/web/src/components/SeatFrame.tsx::catalogSeatLabels(game)` returns the full catalog `seat_labels` (fallback two seats) and feeds both the viewpoint row and the seat rail. RIVLEDSHOSEA-005 adds the Rust-projected active-seat-label field this ticket consumes. Confirmed.
3. Shared boundary under audit: the shell's viewer-selection path (`SeatFrame` -> `changeSeatFrameViewerMode` -> `changeViewerMode` -> WASM viewer API) and the active-seat list it must validate against. End state: any requested ID present in the Rust-projected active set is forwarded; anything else makes no seat-private request.
4. FOUNDATIONS §2 + §11 no-leak firewall: viewing authority is validated against Rust-projected active IDs, never a hardcoded list or a cast of an arbitrary string; selecting another viewpoint requests only that seat's already-authorized projection and grants no action authority. Restated before trusting the spec.
5. No-leak surface (§11/§12): changing viewer must not fetch full state and hide it, preload every seat's private cards, or retain another seat's private payload in component state, cache, accessible name, `data-testid`, animation payload, or replay export. Stale selections (after match replacement, replay reset/import, seat-count change) normalize to observer before any viewer request. This is the load-bearing invariant of the ticket.
6. Renaming/removing the hardcoded allowlist: grep shows the `seat_0 || seat_1` guard is local to `main.tsx::changeSeatFrameViewerMode`; the fixed two-seat games rely on it only through this same generic path, so removing it and validating against the active set preserves their Observer/Seat 1/Seat 2 behavior (proved across the catalog in RIVLEDSHOSEA-008).

## Architecture Check

1. A generic "requested ID must be in the Rust-projected active set" guard is one rule for every game (fixed-two-seat and 3–6-seat alike), eliminating the per-game shell hardcode. Cleaner and safer than extending the allowlist.
2. No shim: the allowlist is deleted, not widened; no `string`→`ViewerSeatId` cast survives without active-list validation.
3. Presentation-only — no behavior authority moves to TypeScript; legality/active-seat truth stays Rust-owned (§2). `engine-core` untouched.

## Verification Layers

1. Generic active-ID validation -> component/unit test: every active ID and observer are accepted; an unknown/stale ID makes no seat-private request and normalizes to observer.
2. No private residue across switches -> no-leak test (`a11y-noleak.smoke.mjs`) asserting Observer → Seat A → Seat B → Observer leaves no Seat A private card in DOM, accessible text, logs, storage, animation payload, or replay export.
3. Accessible one-of-many control -> keyboard e2e: Tab entry, arrow movement, Space selection, visible focus; radiogroup/`aria-checked` semantics; 24px target / spacing baseline.
4. Six-seat coverage -> `river-ledger.smoke.mjs` selecting Observer and Seats 1–6 and verifying each renders only that seat's authorized projection (visible label = internal index + 1).

## What to Change

### 1. `main.tsx` — generic viewer mapping

Replace the `seat_0 || seat_1` allowlist with a type guard against the current Rust-projected active seat IDs. Forward `{ kind: "seat", seat: requestedId }` only when the ID is active; otherwise make no seat-private request and normalize to observer. Normalize selected viewer state whenever match ID, game, active seat set, replay document, or replay cursor changes and the seat is no longer present.

### 2. `SeatFrame.tsx` — active-scoped, accessible selector

Consume the RIVLEDSHOSEA-005 active-seat-label field for both the viewpoint row and seat rail so they cannot drift. Render Observer + active seats as one named one-of-many control (native radios in a `fieldset`/`legend`, or `role="radiogroup"`/`radio` + `aria-checked` + roving `tabIndex` + Space/arrow keys). Inactive seats do not appear in a match-scoped selector. Keep focus visible and targets ≥24px; wrap/reflow 3–6 rows without horizontal overflow (styles.css).

## Files to Touch

- `apps/web/src/main.tsx` (modify)
- `apps/web/src/components/SeatFrame.tsx` (modify)
- `apps/web/src/styles.css` (modify)

## Out of Scope

- The Rust/WASM active-seat projection (RIVLEDSHOSEA-005 — consumed, not built).
- Setup-mode role copy (RIVLEDSHOSEA-007).
- Cross-catalog no-leak regression matrix for the 14 fixed-two-seat games (RIVLEDSHOSEA-008).
- Making inactive seats appear-but-disabled (forbidden by §16.6).

## Acceptance Criteria

### Tests That Must Pass

1. Unit/component: active-ID validation, observer mapping, stale-selection normalization, selected-state semantics.
2. `river-ledger.smoke.mjs` six-seat: Observer + Seats 1–6 respond to pointer and keyboard; selecting internal `seat_2` renders only `seat_2`'s projection with visible label "Seat 3".
3. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e && npm --prefix apps/web run smoke:ui`; `a11y-noleak.smoke.mjs` green.

### Invariants

1. A requested viewer ID is forwarded to WASM only if it is in the Rust-projected active set; an unknown/stale ID never broadens the projection.
2. Switching viewpoints changes only which authorized projection is requested — no private-state residue, no change to legal actor/bot scheduling/terminal outcome.

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/SeatFrame.tsx` (or its test module) — active-ID validation + accessible selector behavior.
2. `apps/web/e2e/river-ledger.smoke.mjs` — six-seat pointer/keyboard selection + per-seat owner-label assertion.
3. `apps/web/e2e/a11y-noleak.smoke.mjs` — Observer→A→B→Observer no-residue assertion.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web ci && npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e`
3. `npm --prefix apps/web run smoke:effects` (selector reflow + focus regressions in the shared shell).
