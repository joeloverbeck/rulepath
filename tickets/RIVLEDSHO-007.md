# RIVLEDSHO-007: Always-available hand-ranking reference

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/ui.rs`, `apps/web/src/wasm/client.ts`, `apps/web/src/components/RiverLedgerBoard.tsx`
**Deps**: RIVLEDSHO-003, RIVLEDSHO-006

## Problem

A player who struggles to read Hold 'Em hand strength has no in-app reference for the category order. This ticket adds an always-available hand-ranking ladder (Straight flush > … > High card) with Rust/static-supplied labels and definitions — default-visible after showdown (winning category marked), collapsible during play (spec WB7 / D5).

## Assumption Reassessment (2026-06-15)

1. Verified against current code: `games/river_ledger/src/ui.rs` already carries the `UiMetadata` struct + `ui_metadata()` (`ui.rs:4,21`) — the established channel for Rust/static presentation metadata; ladder labels/definitions extend it, reaching the client via the WASM catalog/view like other `ui` metadata.
2. Verified against specs/docs: spec §6 D5 + §8 WB7; `games/river_ledger/docs/UI.md` §UI Metadata (inert presentation support, no behavior).
3. Cross-artifact boundary under audit: `ui.rs` ladder metadata → `crates/wasm-api` / `client.ts` types → the React ladder render; the winning-category mark consumes the RIVLEDSHO-003 explanation `category` field.
4. FOUNDATIONS §2 behavior authority motivates this ticket: the ladder labels, definitions, and ordering are Rust/static-supplied; TypeScript lays them out and marks the current category from the projected field — it computes no ordering or evaluation.

## Architecture Check

1. Carrying the ladder via the existing `UiMetadata` channel (not a TS-side constant) keeps category labels/order Rust-authored and IP-clean, consistent with the established per-game `ui.rs` pattern.
2. No backwards-compatibility aliasing/shims; additive metadata + a new render slot.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4) — game-local `ui.rs` metadata + `apps/web` render.

## Verification Layers

1. The ladder is reachable during play (collapsible) and default-visible after showdown with the winning category marked -> `npm --prefix apps/web run smoke:ui`.
2. Ladder labels/order come from Rust `ui.rs` metadata, not a TS constant -> grep the web ladder render for any hardcoded category-order array (none) + manual review.
3. The metadata reaches the client through the bridge -> `npm --prefix apps/web run build` type-check against the `client.ts` `ui`-metadata type.

## What to Change

### 1. `games/river_ledger/src/ui.rs`

Add hand-ranking ladder rows (category key, label, short definition, high-to-low order) to the `UiMetadata` channel.

### 2. `apps/web/src/wasm/client.ts`

Add the TS type for the ladder metadata rows.

### 3. `apps/web/src/components/RiverLedgerBoard.tsx`

Render the ladder: collapsible during play, default-visible after showdown, current winning category marked from the projected explanation `category`.

## Files to Touch

- `games/river_ledger/src/ui.rs` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)

## Out of Scope

- Card visuals (RIVLEDSHO-006).
- Teaching-strength aid (RIVLEDSHO-010).
- Action-panel / seat affordances (RIVLEDSHO-008/009).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` — ladder present, collapsible in play, default-visible + winning-category-marked after showdown.
2. `cargo test -p river_ledger` — `ui.rs` metadata additions compile and serialize.
3. `npm --prefix apps/web run build` — `client.ts` ladder type compiles.

### Invariants

1. Category labels/order are Rust/static-supplied; TS renders and marks only (§2).
2. The ladder metadata is inert presentation support — no behavior/selector fields (§5).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/src/ui.rs` (modify) — ladder metadata + its unit/serialization coverage.
2. `apps/web/scripts/smoke-ui.mjs` (modify, as surfaced) — assert ladder presence and post-showdown default-visible state.

### Commands

1. `cargo test -p river_ledger`
2. `npm --prefix apps/web run smoke:ui`
3. `smoke:ui` plus the crate metadata test is the correct boundary; the ladder carries no behavior to replay-check.
