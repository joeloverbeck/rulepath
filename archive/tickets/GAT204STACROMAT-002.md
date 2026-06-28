# GAT204STACROMAT-002: Consume Rust seat labels in the Starbridge board + turn bar; drop `formatPoint`

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/wasm/client.ts` (Starbridge view types), `apps/web/src/components/StarbridgeCrossingBoard.tsx` (consume labels, remove `formatPoint`), `apps/web/e2e/starbridge-crossing.smoke.mjs` (turn-bar assertion)
**Deps**: GAT204STACROMAT-001

## Problem

With the Rust public view projecting `ui.seat_labels` + per-seat `label`/`target_label` (GAT204STACROMAT-001), the web shell must consume those Rust-owned labels and stop synthesizing seat names in TypeScript. Today `StarbridgeCrossingBoard` title-cases the lowercase `home` token via a local `formatPoint` helper for the seat name **and** the legend's `to {formatPoint(seat.target)}` destination, and the shared `ModeControls` turn-status bar shows `Seat 1 to act` because it found no `view.ui.seat_labels`. This ticket types the new view fields, has the board consume both labels and remove `formatPoint` entirely, confirms `ModeControls` resolves through its existing shared path, and extends the browser smoke to cover the turn bar — bringing every in-match surface to point names with no TypeScript deriving the name from a token or index (`docs/UI-INTERACTION.md` §3/§19; `SC-UI-001`).

## Assumption Reassessment (2026-06-28)

1. `apps/web/src/components/StarbridgeCrossingBoard.tsx` uses `formatPoint` (defined at `:476`) at exactly two sites: `seatNameMap` (`:461`, `formatPoint(seat.home)`) and the legend `to {formatPoint(seat.target)}` (`:290`). Removing the helper requires a Rust label for **both** home and target — supplied by 001's `seat.label` / `seat.target_label`. Confirmed.
2. `ModeControls.seatLabelsForView` (`apps/web/src/components/ModeControls.tsx:189`–`192`) reads **only** `view.ui.seat_labels`, then resolves via `resolveSeatLabel(seat, { activeSeatLabels })` (`apps/web/src/seatLabels.ts`). Once 001 projects `view.ui.seat_labels`, the turn bar resolves to the point name with **no `ModeControls`/`seatLabels.ts` code change** — this ticket only verifies it. Confirmed (verify-only; no game-specific branching is added to shared components).
3. Cross-artifact boundary under audit: the Starbridge view types in `apps/web/src/wasm/client.ts` — `StarbridgeCrossingSeatView` (`:1589`, currently `{seat_id, seat_index, home, target, finish_rank}`, no `label`/`target_label`) and `StarbridgeCrossingPublicView` (`:1603`, no `ui` field). Both gain additive optional fields matching 001's projection; `SeatDisplayLabel` is already typed (`client.ts:114`).
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: TypeScript stays presentation-only — it renders the Rust label and decides no display name. Removing `formatPoint` eliminates the last TS token-synthesis (home and target), satisfying the spec's Not-allowed clause naming the `home`/`target` token.
5. Schema extension (consumer side): `StarbridgeCrossingSeatView` gains `label?: string` and `target_label?: string`; `StarbridgeCrossingPublicView` gains `ui?: { seat_labels?: SeatDisplayLabel[] }`. Additive-only, consuming 001's additive projection; no existing field renamed.

## Architecture Check

1. Consuming the Rust `label`/`target_label` (and the shared `ui.seat_labels` for the turn bar) keeps one Rust source of truth and deletes the parallel TS title-casing that §2 forbids. Routing the turn bar through the existing `seatLabelsForView` → `resolveSeatLabel` path (rather than a Starbridge-specific branch) keeps the shared shell game-agnostic.
2. No backwards-compatibility shim: `formatPoint` is removed outright, not retained behind a fallback; the board reads the Rust labels directly.
3. `engine-core` stays free of mechanic nouns — this is presentation-only TypeScript; no shell-side legality, no game-specific coupling in `ModeControls`/`seatLabels.ts` (§3, §7).

## Verification Layers

1. Board names seats by point (no token derivation) → `apps/web/e2e/starbridge-crossing.smoke.mjs` `assertSeatDisplayNames` (board heading/active-seat/legend incl. the `to {…}` target) plus a grep-proof that `formatPoint` no longer appears in `StarbridgeCrossingBoard.tsx`.
2. Shared turn bar names the acting seat by point → extend `assertSeatDisplayNames` to assert the `ModeControls` turn-status bar reads a point name (e.g. `North to act` / `North turn in progress`) and never `Seat N`, for a multi-seat match.
3. No-leak (all-public) → `assertNoLeak` already in the Starbridge smoke; the labels are public point names, adding no forbidden term to payloads/DOM (§11).

## What to Change

### 1. Type the new view fields (`client.ts`)

Add `label?: string` and `target_label?: string` to `StarbridgeCrossingSeatView` (`:1589`) and `ui?: { seat_labels?: SeatDisplayLabel[] }` to `StarbridgeCrossingPublicView` (`:1603`), reusing the existing `SeatDisplayLabel` type.

### 2. Consume labels + remove `formatPoint` (`StarbridgeCrossingBoard.tsx`)

Source the seat name from `seat.label` (via `seatNameMap`, `:461`) and/or the projected `ui.seat_labels`, and the legend destination from `seat.target_label` (`:290`). Delete the `formatPoint` helper (`:476`) and both call sites. No token title-casing remains in the board.

### 3. Verify `ModeControls` + extend the browser smoke

Confirm (no code change) that `ModeControls` renders the point name through `seatLabelsForView` → `resolveSeatLabel` now that `view.ui.seat_labels` is present. Extend `assertSeatDisplayNames` in `apps/web/e2e/starbridge-crossing.smoke.mjs` so it also asserts the turn-status bar reads a point name (never `Seat N`) for a multi-seat match.

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/StarbridgeCrossingBoard.tsx` (modify)
- `apps/web/e2e/starbridge-crossing.smoke.mjs` (modify)

## Out of Scope

- The Rust view projection itself (GAT204STACROMAT-001) and all docs/closeout (GAT204STACROMAT-003).
- Any `ModeControls` / `seatLabels.ts` code change or game-specific branching in shared components (verify-only here).
- Any movement/finish/terminal/visibility/bot change; changing the Gate 20.2 setup-preview labels.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:e2e` (or the Starbridge e2e entry) — `assertSeatDisplayNames` passes for board heading, active-seat status, legend incl. `to {…}` target, and the `ModeControls` turn-status bar, with no `Seat N` in normal mode.
2. `npm --prefix apps/web run build` and `npm --prefix apps/web run smoke:ui` pass.
3. `grep -n "formatPoint" apps/web/src/components/StarbridgeCrossingBoard.tsx` returns nothing (helper fully removed).

### Invariants

1. No TypeScript derives a seat or destination display name from the `home`/`target` token or seat index — all names come from the Rust view labels.
2. No game-specific (`starbridge_crossing`) branching is introduced in shared shell components (`ModeControls`, `seatLabels.ts`).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/starbridge-crossing.smoke.mjs` — extend `assertSeatDisplayNames` to cover the `ModeControls` turn-status bar (point name, never `Seat N`) for a multi-seat match.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:e2e` (Starbridge browser smoke, incl. the extended `assertSeatDisplayNames`)
3. `grep -n "formatPoint" apps/web/src/components/StarbridgeCrossingBoard.tsx` — narrow grep-proof that the interim helper is gone; the e2e smoke is the full presentation boundary.

## Outcome

Completed: 2026-06-28

Implemented the web consumption half of Gate 20.4:

- Added additive Starbridge TypeScript view fields for `seats[].label`, `seats[].target_label`, and `ui.seat_labels`.
- Updated `StarbridgeCrossingBoard` to render seat names and legend target names from Rust-projected labels and removed the interim `formatPoint` helper entirely.
- Extended `apps/web/e2e/starbridge-crossing.smoke.mjs` so `assertSeatDisplayNames` checks the board legend targets and the shared `ModeControls` turn-status text for point names with no `Seat N` fallback.

Deviations from plan: none. No `ModeControls` or `seatLabels.ts` code was changed; the turn bar resolves through the existing shared `view.ui.seat_labels` path.

Verification:

- `npm --prefix apps/web run build` passed after the final source changes.
- `node apps/web/e2e/starbridge-crossing.smoke.mjs` passed after the final source changes.
- `npm --prefix apps/web run smoke:ui` passed after the final source changes.
- `npm --prefix apps/web run smoke:e2e` passed before the final no-behavior type-helper cleanup; the affected Starbridge entry and build were rerun afterward as listed above.
- `rg -n "formatPoint" apps/web/src/components/StarbridgeCrossingBoard.tsx` returned no matches.
