# GAT72GAT8HIG-017: TypeScript client bindings + catalog viewer-mode types

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/wasm/client.ts`, `apps/web/src/state/shellReducer.ts`
**Deps**: GAT72GAT8HIG-016

## Problem

The TypeScript client must consume the viewer-aware WASM surface for
`high_card_duel`: typed bindings that encode viewer mode (Seat 0 / Seat 1 /
Observer), surface the new catalog entry, and prevent accidental `any` /
raw-hidden-state pass-through — without TypeScript ever deciding legality or
reconstructing hidden state.

## Assumption Reassessment (2026-06-07)

1. Verified the client surface: `apps/web/src/wasm/client.ts` wraps the WASM
   exports; `apps/web/src/state/shellReducer.ts:38,72,99,128-136` holds
   `catalog: GameCatalogEntry[]` and selects `selectedGameId` from the
   Rust-supplied catalog. The catalog is Rust-authoritative (GamePicker shows
   "Waiting for the Rust catalog…").
2. Verified against the spec: §4.2.5 (TypeScript bindings) requires types that
   encode viewer mode and block `any`/raw hidden-state pass-through; §8.1 forbids
   TS recomputing legality/reveal/shuffle/score.
3. Cross-artifact boundary under audit: the WASM↔TS JSON contract
   (`docs/WASM-CLIENT-BOUNDARY.md`). The new viewer-mode parameter threads from
   the client wrapper into `get_view`/`get_action_tree`/`get_effects` calls.
4. FOUNDATIONS principle under audit (§2 behavior authority): TypeScript stays
   presentation-only; it forwards the viewer to Rust and renders the returned
   viewer-safe payload, never deriving hidden facts.

## Architecture Check

1. Typing the viewer mode at the client boundary (a discriminated union, not
   `any`) makes "no raw hidden state in TS" a compile-time property — cleaner
   than runtime trust.
2. No backwards-compatibility shims — additive bindings; existing game calls are
   unchanged except for the now-honored viewer argument.
3. No engine/`game-stdlib` change; this is the presentation shell consuming Rust
   output. Legality/views remain Rust-authoritative.

## Verification Layers

1. Viewer-mode typing -> schema/serialization validation: client types encode Seat 0/Seat 1/Observer and reject `any` hidden-state pass-through (`tsc --noEmit`).
2. Catalog consumption -> manual review: `high_card_duel` appears via the Rust catalog with its viewer modes.
3. No TS legality -> FOUNDATIONS alignment check: the client forwards the viewer and renders Rust payloads; no legality/reveal computed in TS (§2).

## What to Change

### 1. `client.ts`

Add typed wrappers threading `viewerMode` into `get_view`/`get_action_tree`/
`get_effects`/`export_replay`; type the `high_card_duel` view/effect payloads as
viewer-safe shapes (no field that could carry a hidden id in an unauthorized
mode).

### 2. `shellReducer.ts`

Surface the viewer mode in shell state and the catalog entry; default selection
remains catalog-driven.

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/state/shellReducer.ts` (modify)

## Out of Scope

- The `HighCardDuelBoard` component and shell integration (GAT72GAT8HIG-018).
- e2e smoke (019).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — `tsc --noEmit` + vite build pass with the new typed bindings.
2. `npm --prefix apps/web run smoke:wasm` — the client loads the catalog including `high_card_duel`.

### Invariants

1. The client never holds raw hidden state typed as `any`; viewer mode is a typed parameter forwarded to Rust (§2).
2. The catalog drives game/viewer selection (Rust-authoritative).

## Test Plan

### New/Modified Tests

1. `apps/web` type-level coverage via `tsc --noEmit` (build) — the viewer-mode types compile and reject untyped hidden-state pass-through.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:wasm`
3. The TS build + wasm load smoke are the correct boundary; rendered no-leak behavior is proven in 019.
