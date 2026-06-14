# GAT15RIVLEDTEX-017: Web renderer — RiverLedgerBoard and app-shell registration

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/RiverLedgerBoard.tsx`, `apps/web/src/main.tsx`, `apps/web/src/wasm/client.ts`
**Deps**: GAT15RIVLEDTEX-016

## Problem

River Ledger needs a polished, neutral public browser surface that presents only Rust/WASM state: N-seat seat frames, button/SB/BB markers, active/pending indicators, public community board, the abstract contribution ledger, legal-action controls, safe previews, the final outcome breakdown, and viewer-safe explanations — with no casino trade dress and no legality in TypeScript.

## Assumption Reassessment (2026-06-14)

1. `apps/web/src/components/PokerLiteBoard.tsx` is the hidden-info/showdown precedent; `apps/web/src/main.tsx` registers boards via a `is<Game>View(view)` type-guard chain; `apps/web/src/wasm/client.ts` defines `GameCatalogEntry` (with `supported_seats`) and the `PublicView` union; `MatchSetup.tsx` already consumes `supportedSeatCounts` generically and `SeatFrame.tsx` is already catalog-driven and generic (verified) — so both are **reused as-is, not modified**.
2. `specs/...-base.md` §4.3 (web rows), §5 G15-RL-011, and §6 exit row 5 fix the renderer requirements.
3. Cross-artifact boundary under audit: `client.ts` gains the River Ledger view/outcome types + `isRiverLedgerView` guard; `main.tsx` adds the guard arm rendering `RiverLedgerBoard`; both consume the WASM catalog/views registered in 016. No new legality crosses into TS.
4. FOUNDATIONS §2 (TypeScript presentation-only) motivates this ticket: the renderer maps Rust action IDs to controls and renders Rust views/previews/outcomes; it computes no legal actions, call prices, hand ranks, winner selection, split allocation, or hidden-card availability.

## Architecture Check

1. Following the `PokerLiteBoard` + type-guard-chain pattern keeps the renderer a thin presentation of Rust views and reuses the generic `SeatFrame`/`MatchSetup`, avoiding bespoke N-seat plumbing.
2. No backwards-compatibility aliasing/shims — new component + additive guard arm.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); no legality in TypeScript (§2).

## Verification Layers

1. Renderer presents Rust views/controls/outcome without inventing state -> `npm --prefix apps/web run build` + `smoke:ui`.
2. Seat order, active/pending, button/SB/BB, board, ledger render from catalog/view payloads -> `smoke:ui` catalog/variant assertions.
3. No legality/eval logic in TypeScript -> `node scripts/check-presentation-copy.mjs` + manual review of `RiverLedgerBoard.tsx`.

## What to Change

### 1. `apps/web/src/components/RiverLedgerBoard.tsx`

N-seat River Ledger board: seat frames (reusing `SeatFrame`), button/SB/BB markers, active/pending indicators, public community cards, abstract contribution ledger, legal-action controls, safe previews, final outcome breakdown, viewer-safe explanation — neutral original visuals.

### 2. `apps/web/src/wasm/client.ts` + `apps/web/src/main.tsx`

Add the River Ledger view/outcome-rationale types and `isRiverLedgerView` guard in `client.ts`; add the guard arm rendering `RiverLedgerBoard` in `main.tsx`.

## Files to Touch

- `apps/web/src/components/RiverLedgerBoard.tsx` (new)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/wasm/client.ts` (modify)

## Out of Scope

- `MatchSetup.tsx` / `SeatFrame.tsx` (reused as-is — already generic; no modify).
- E2E smoke, `ci/games.json`, README catalog reconciliation (GAT15RIVLEDTEX-018).
- Casino trade dress / chip/cash language; any TypeScript legality (§2).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — type-checks and builds with the new renderer.
2. `npm --prefix apps/web run smoke:ui` — River Ledger catalog/variant + seat presentation render.
3. `node scripts/check-presentation-copy.mjs` — no debug vocabulary / raw internal IDs / casino language in `RiverLedgerBoard.tsx`.

### Invariants

1. TypeScript presents Rust state only; no legality/eval/winner/split in TS (§2).
2. Visuals are neutral and original; no casino trade dress (§10).

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/RiverLedgerBoard.tsx` (new) — exercised by `smoke:ui`.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui && node scripts/check-presentation-copy.mjs`
3. `smoke:ui` is the correct boundary for renderer presentation; DOM no-leak and e2e are exercised in GAT15RIVLEDTEX-018.
