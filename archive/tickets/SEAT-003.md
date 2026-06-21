# SEAT-003: Add a shared TypeScript seat-label resolver

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `apps/web` only (new presentation helper; no Rust/engine change)
**Deps**: none

## Problem

Seat labels are formatted independently in ~13 board components plus the VIEWER
panel, each inventing its own scheme (0-based "Seat N", 1-based "Seat N", 1-based
"Player N", `seat.slice(-1)+1`, ternaries, regexes). This duplication is what let
Briar Circuit's VIEWER (0-based, from Rust catalog) drift from its play area
(1-based, invented in `BriarCircuitBoard.tsx:379`). `docs/UI-INTERACTION.md` §3/§10B
require TS to display Rust-supplied labels, not invent them.

This ticket adds a single shared resolver that maps a `seat` id to its
Rust-supplied label, with one defensive fallback. It is a pure addition — no
caller is migrated here (SEAT-004/005 do that), so it can land in parallel with
the Rust tickets.

## Assumption Reassessment (2026-06-21)

1. Label sources already present in payloads (`apps/web/src/components/SeatFrame.tsx:85-98`):
   `projection.active_seat_labels` (Rust view payload), then
   `game.seat_labels ?? game.ui.seat_labels` (catalog metadata), then a hardcoded
   0-based fallback. Confirmed. The resolver formalizes this precedence and removes
   the 0-based fallback.
2. `SeatDisplayLabel` shape is `{ seat: string; label: string }`
   (`SeatFrame.tsx:14` `active_seat_labels?: SeatDisplayLabel[]`). Confirm the
   exact exported type/location at implementation time and reuse it; do not
   redeclare.
3. Shared boundary: the `{seat,label}` contract emitted by the Rust catalog
   (SEAT-001/002) and view payloads. The resolver is the single TS consumer seam.
8. Adjacent: the current SeatFrame hardcoded fallback (`"Seat 0"/"Seat 1"`) is a
   TS-invented label; folding it into a defensive, dev-warned 1-based fallback is a
   required consequence of this ticket, applied to SeatFrame in SEAT-004.

## Architecture Check

1. One resolver replacing 13+ ad-hoc formatters removes the duplication that caused
   the drift and makes future games correct-by-default — strictly cleaner than
   per-board helpers.
2. No backwards-compatibility shim: the resolver does not wrap or alias the old
   per-board helpers; SEAT-004/005 delete them.
3. `apps/web` presentation only; no legality, no Rust change. TS consumes
   Rust-authored labels, honoring `docs/FOUNDATIONS.md` §2/§7.

## Verification Layers

1. Resolver returns the Rust-supplied label for a known seat -> unit test
   (resolver given `active_seat_labels`/`game.seat_labels` returns that label).
2. Precedence is view-payload → catalog metadata → defensive fallback -> unit test
   (each source layer exercised).
3. Fallback is 1-based and dev-warns (never silently invents in prod) -> unit test
   (empty inputs → `Seat N` with a dev-only warning).

## What to Change

### 1. Shared resolver module

Add `apps/web/src/seatLabels.ts` (or co-located util) exporting:
- `resolveSeatLabel(seat: string, sources): string` — returns the matching
  `label` from, in order, the view payload `active_seat_labels`, then catalog
  `seat_labels`/`ui.seat_labels`; defensive last-resort `Seat {n+1}` derived from
  the `seat_{n}` id, emitting a `import.meta.env.DEV` warning.
- `resolveSeatLabels(seats, sources): SeatDisplayLabel[]` — list form for VIEWER
  option lists.

Reuse the existing `SeatDisplayLabel` type; do not redeclare it.

## Files to Touch

- `apps/web/src/seatLabels.ts` (new)
- `apps/web/src/seatLabels.test.ts` (new)

## Out of Scope

- Migrating SeatFrame or any board to the resolver (SEAT-004/005).
- Any Rust change.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web test -- seatLabels` (or the repo's web unit-test command)
   — resolver precedence and fallback covered.
2. `npm --prefix apps/web run build`

### Invariants

1. The resolver never invents a label when Rust supplies one.
2. The defensive fallback is 1-based and dev-warned.

## Test Plan

### New/Modified Tests

1. `apps/web/src/seatLabels.test.ts` — view-payload precedence, catalog fallback,
   defensive 1-based fallback with dev warning.

### Commands

1. `npm --prefix apps/web test -- seatLabels`
2. `npm --prefix apps/web run build`

## Outcome

Completed: 2026-06-21

Changed:
- Added `apps/web/src/seatLabels.ts` with `resolveSeatLabel` and
  `resolveSeatLabels`.
- The resolver prefers Rust-projected `active_seat_labels`, then catalog
  `seat_labels`, then catalog `ui.seat_labels`, and only then uses a defensive
  1-based fallback.
- Added `apps/web/src/seatLabels.test.ts` covering source precedence, list
  resolution, 1-based fallback behavior, non-seat fallback behavior, and the
  dev-warning path.

Deviations:
- The package has no `npm --prefix apps/web test` script. The TypeScript test was
  executed by bundling it with the existing `esbuild` dependency and running the
  bundled module with Node.

Verification:
- `npm exec esbuild -- src/seatLabels.test.ts --bundle --format=esm --platform=node --outfile=/tmp/rulepath-seatLabels.test.mjs` passed from `apps/web/`.
- `node /tmp/rulepath-seatLabels.test.mjs` passed.
- `npm --prefix apps/web run build` passed.
