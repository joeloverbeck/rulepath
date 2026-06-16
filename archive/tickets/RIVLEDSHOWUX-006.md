# RIVLEDSHOWUX-006: `Seat N` status line via the event_frontier label path

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (river_ledger per-match `ui.seat_labels` view projection), `apps/web/src/components/ModeControls.tsx`, `apps/web/src/components/SeatFrame.tsx`, `apps/web/src/wasm/client.ts`
**Deps**: None

## Problem

The active-actor status line falls through to the `Player N` fallback (`ModeControls.tsx:189`) for River Ledger while seat panels say `Seat N` — mixed vocabulary. `activeActorLabel` (`ModeControls.tsx:181-186`) already consumes `view.ui.seat_labels`, but only for `event_frontier`; River Ledger projects `seat_labels` only on its catalog, not its per-match view. This ticket extends the existing path so River Ledger status copy reads `Seat N to act` — reusing the existing infrastructure, not building a new label type (spec A9).

## Assumption Reassessment (2026-06-16)

1. Verified: `activeActorLabel` (`ModeControls.tsx:181-186`) reads `view.ui.seat_labels.find(...).label` gated to `event_frontier`, else `playerLabel` → `Player N` (`:189`); `SeatFrame.tsx:75` already consumes `game.seat_labels`; `client.ts:105` defines `SeatDisplayLabel {seat,label}`; `catalog_seat_labels_json` emits "Seat N" (`crates/wasm-api/src/lib.rs:449-457,618`).
2. Verified against spec §6 D5 + §8 WB6 + §14 A9; the `event_frontier` per-match projection precedent is `event_frontier_seat_display_label_json` / seat-metadata json (`crates/wasm-api/src/lib.rs:8157-8169`). River Ledger has no `ui.seat_labels` on its per-match view today.
3. Shared boundary under audit: `activeActorLabel` runs for **every** game via the global `ModeControls` — extending its `seat_labels` consumption must not regress the other 12 games (those without `ui.seat_labels`) or `event_frontier`.
4. FOUNDATIONS §2: the label is Rust/catalog-authored ("Seat N" from `seat_labels`); TS renders it and computes no label.
5. Blast radius (shared surface): `ModeControls.tsx` `activeActorLabel` + `playerLabel` are consumed only within `ModeControls`; grep confirms no other component imports them. Extending the gate to "any view carrying `ui.seat_labels`" (or adding a `river_ledger` arm) leaves `playerLabel` as the fallback for games without the field.

## Architecture Check

1. Mirroring the `event_frontier` per-match `ui.seat_labels` projection and extending the existing `activeActorLabel` branch reuses proven infrastructure — no new label type, no wider `activeActorLabel` signature (the catalog-fallback alternative is weighed and rejected in spec A9).
2. No shims; the `Player N` fallback is retained only for games that genuinely have no `ui.seat_labels`.
3. `engine-core` untouched (§3); the per-match projection is `wasm-api` glue over `games/river_ledger` view data; no `game-stdlib` change (§4).

## Verification Layers

1. River Ledger status reads `Seat N to act`, not `Player N` -> `npm --prefix apps/web run smoke:ui` status-line assertion.
2. Cross-game non-regression: `event_frontier` keeps its labels; the 12 games without `ui.seat_labels` keep their current label -> `npm --prefix apps/web run smoke:ui` (multi-game status-line sweep).
3. The projected `ui.seat_labels` carries no hidden state -> `games/river_ledger/tests/visibility.rs` (public-view projection).

## What to Change

### 1. `crates/wasm-api/src/lib.rs`

Mirror `event_frontier`'s per-match `ui.seat_labels` view projection for River Ledger (the catalog already supplies "Seat N"; this adds the same labels to the per-match view the status line reads).

### 2. `apps/web/src/components/ModeControls.tsx`

Extend the `activeActorLabel` `event_frontier` branch to River Ledger (or generalize its gate to "any view carrying `ui.seat_labels`"); leave `playerLabel` as the fallback for games without the field.

### 3. `apps/web/src/components/SeatFrame.tsx` / `client.ts`

Verify `SeatFrame` multi-seat coverage already consumes `game.seat_labels` (likely no change); extend the `RiverLedgerPublicView` `ui` TS type if the per-match `seat_labels` field is newly added.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/src/components/ModeControls.tsx` (modify)
- `apps/web/src/components/SeatFrame.tsx` (modify, verify-only likely)
- `apps/web/src/wasm/client.ts` (modify)

## Out of Scope

- The Rust showdown-string seat-label fix (RIVLEDSHOWUX-001).
- A new `SeatDisplayLabel` shape — the existing `{seat,label}` is reused (spec A9).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` + `npm --prefix apps/web run build` — River Ledger status reads `Seat N to act`; type-checks.
2. `npm --prefix apps/web run smoke:ui` — other games' status copy unchanged (`event_frontier` labels intact; `Player N`-fallback games unchanged).
3. `cargo test -p wasm-api` + `cargo run -p fixture-check -- --game river_ledger` — per-match `ui.seat_labels` projection additive and viewer-safe.

### Invariants

1. The status label is Rust/catalog-authored; TS computes no label (§2).
2. Extending the shared `activeActorLabel` regresses no other game's status copy (§11 acceptance: change is bounded and covered).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/river-ledger.smoke.mjs` (modify, as surfaced) — `Seat N to act` status assertion.
2. `games/river_ledger/tests/visibility.rs` — per-match `ui.seat_labels` carries only public seat labels.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run build`
3. `cargo run -p fixture-check -- --game river_ledger`

## Outcome

Completed on 2026-06-16.

- Added Rust-authored River Ledger per-match `ui.seat_labels` metadata using the existing `{seat,label}` shape.
- Projected those labels through the WASM UI JSON and extended the River Ledger TypeScript metadata type.
- Generalized `ModeControls` to consume `ui.seat_labels` from any public view that carries them, preserving the `Player N` fallback for games without labels.
- Added Rust/WASM/browser assertions for public seat-label metadata and the River Ledger mode status line (`Seat 0 (you) to act`, not `Player 1 to act`).
- Verified with `cargo fmt --all --check`, `cargo test -p wasm-api`, `cargo test -p river_ledger`, `cargo run -p fixture-check -- --game river_ledger`, `npm --prefix apps/web run smoke:ui` (includes `npm --prefix apps/web run build`), and `node apps/web/e2e/river-ledger.smoke.mjs`.
