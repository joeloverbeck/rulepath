# GAT202STACROACT-002: Web shell consumes the Rust active-seat-by-count mapping

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web` (`src/wasm/client.ts` bridge type, `src/components/MatchSetup.tsx` setup-label resolver, `e2e/starbridge-crossing.smoke.mjs`)
**Deps**: GAT202STACROACT-001

## Problem

The browser setup screen names the wrong active seats for 2/3/4-seat Starbridge
matches because `setupLabelsForCount` returns `labels.slice(0, selectedSeatCount)`
over the flat six-point ring (`apps/web/src/components/MatchSetup.tsx:339-359`).
That "first *N*" slice is a presentation surface deciding a Rust-owned setup fact,
contrary to `SC-UI-001`. GAT202STACROACT-001 added the Rust-owned active-seat
indices to the Starbridge catalog entry; this ticket makes the shell consume them
for the setup preview (`modeDetail`, `setupSeatRoles`, and any sibling setup-label
surface), retaining the slice only as a fallback for games that do not provide the
mapping.

## Assumption Reassessment (2026-06-28)

1. `setupLabelsForCount` (`apps/web/src/components/MatchSetup.tsx:339`) calls
   `setupLabels(game)` (the catalog six-point ring) and slices the first
   `selectedSeatCount`; its result feeds `modeDetail` (`MatchSetup.tsx:220`) and
   `setupSeatRoles` (`MatchSetup.tsx:261`) via `selectedSetupLabels`
   (`MatchSetup.tsx:63`). The slice is correct only for contiguous active seats.
2. The bridge type `GameCatalogEntry` (`apps/web/src/wasm/client.ts:95`, with
   `supported_seats?` / `seat_labels?` at 103-104) must declare the new catalog
   field so it is typed and available; without it the field is unreachable from TS.
3. Cross-artifact boundary under audit: the catalog JSON field added by
   GAT202STACROACT-001 (the active ring indices per seat count) is the value this
   ticket reads. The shell indexes the existing `seat_labels` ring by those
   indices; it must not recompute the active set. This is a §2 behavior-authority
   boundary (Rust owns setup; TS presents).
4. FOUNDATIONS §2 and `SC-UI-001`: TypeScript must not decide which seats are
   active. Consuming the Rust mapping (with a slice fallback only for games that
   genuinely have contiguous active seats — Assumption A3) keeps legality/setup in
   Rust. Removing the "first *N*" derivation for Starbridge closes a live §12
   "TypeScript decides … behavior" crossing.
5. Schema extension (consumer side): `GameCatalogEntry` gains one additive
   optional field mirroring the catalog JSON. The producer is GAT202STACROACT-001;
   the change is additive-only and the slice fallback preserves every other game's
   current behavior. Other catalog consumers (`SeatFrame.tsx`, `GamePicker.tsx`,
   `shellReducer.ts`) read `seat_labels`/`supported_seats` and are unaffected.

## Architecture Check

1. Consuming a Rust-provided index mapping and indexing the existing ring is
   cleaner than the current slice: it makes the discontinuous active set correct
   without the shell encoding any per-game seat topology, and keeps the fallback
   path explicit for contiguous-seat games.
2. No backwards-compatibility aliasing/shims: the slice is retained only as a
   typed fallback for games without the mapping, not as a dual code path for
   Starbridge.
3. `engine-core` untouched; no `game-stdlib` change. The shell stays
   presentation-only (§2) — it reads Rust data, never computes the active set.

## Verification Layers

1. Starbridge setup preview names the Rust-correct seats ->
   `apps/web/e2e/starbridge-crossing.smoke.mjs` asserting the 2-seat Players &
   roles list contains `South` and not `North East` (and the 3/4-seat sets per
   `SC-SETUP-003`).
2. Contiguous-seat games unchanged -> the smoke / `setupLabelsForCount` fallback
   path keeps games without the mapping on the existing slice (manual review +
   existing `e2e/seat-label-consistency.smoke.mjs` staying green).
3. Behavior authority -> FOUNDATIONS alignment check: `setupLabelsForCount`
   consumes the Rust field and does not recompute the active set (§2, `SC-UI-001`).

## What to Change

### 1. Declare the new field on the bridge type

In `apps/web/src/wasm/client.ts`, add the active-seat-by-count field to
`GameCatalogEntry` (optional, mirroring the catalog JSON shape from
GAT202STACROACT-001 — a per-seat-count map of active ring indices).

### 2. Consume the mapping in the setup-label resolver

In `apps/web/src/components/MatchSetup.tsx`, change `setupLabelsForCount` to, when
the game provides the active-seat mapping for the selected seat count, resolve the
labels by indexing the catalog ring (`setupLabels(game)`) at the Rust-provided
indices, instead of `labels.slice(0, selectedSeatCount)`. Retain the slice as the
fallback when the mapping is absent. The corrected labels flow unchanged into
`modeDetail` and `setupSeatRoles`.

### 3. Extend the Starbridge setup smoke

In `apps/web/e2e/starbridge-crossing.smoke.mjs`, add a setup-preview assertion:
for a 2-seat Starbridge setup, the Players & roles list contains `South` and not
`North East`; assert the 3- and 4-seat active sets match `SC-SETUP-003`.

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/MatchSetup.tsx` (modify)
- `apps/web/e2e/starbridge-crossing.smoke.mjs` (modify)

## Out of Scope

- The Rust catalog metadata + snapshot + wasm-api test — GAT202STACROACT-001.
- Docs reconciliation / `specs/README.md` `Done`-flip — GAT202STACROACT-003.
- Any in-match view/legend change (already correct via the Rust public view),
  renderer-list/smoke-list membership change (game already listed), or ring-label
  renaming.
- Changing the fallback behavior of any contiguous-seat game.

## Acceptance Criteria

### Tests That Must Pass

1. `node apps/web/e2e/starbridge-crossing.smoke.mjs` — 2-seat Players & roles
   shows `South`, not `North East`; 3/4-seat sets match `SC-SETUP-003`.
2. `npm --prefix apps/web run build` && `npm --prefix apps/web run smoke:e2e`
   (contiguous-seat games' setup previews unchanged).
3. `npm --prefix apps/web run smoke:ui`.

### Invariants

1. The shell consumes the Rust-provided active-seat mapping; no TypeScript code
   derives the active-seat set for any game by position except the documented
   fallback (§2, `SC-UI-001`).
2. Contiguous-seat games' setup previews are byte-for-byte unchanged.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/starbridge-crossing.smoke.mjs` — setup-preview active-seat
   assertions for 2/3/4 seats (failing-first against `main` before the resolver
   change).

### Commands

1. `npm --prefix apps/web run build && node apps/web/e2e/starbridge-crossing.smoke.mjs`.
2. `npm --prefix apps/web run smoke:e2e` (full web smoke set; confirms no
   regression for other games).
3. `npm --prefix apps/web run build` (the e2e smoke requires a prior build; it is
   the correct boundary because the change is shell-only TypeScript).
