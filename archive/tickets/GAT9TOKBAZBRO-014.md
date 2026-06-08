# GAT9TOKBAZBRO-014: TypeScript client bindings + catalog wiring

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/wasm/client.ts`, `apps/web/src/main.tsx`, `apps/web/src/components/DevPanel.tsx`, `apps/web/src/components/ReplayViewer.tsx`
**Deps**: GAT9TOKBAZBRO-013

## Problem

The web shell needs typed client bindings and catalog wiring so it can list and
drive `token_bazaar` through the WASM bridge. This ticket extends the TS WASM
client with the token_bazaar view/effect/action-tree/replay types, registers the
game in the shell catalog, and wires the dev panel and replay viewer to recognize
it — all presentation glue, with no rule logic in TypeScript.

## Assumption Reassessment (2026-06-08)

1. `crates/wasm-api` exposes the `token_bazaar` catalog + arms after
   GAT9TOKBAZBRO-013. The existing TS surfaces that reference `high_card_duel`
   establish the wiring sites: `apps/web/src/wasm/client.ts`, `apps/web/src/main.tsx`
   (catalog), `apps/web/src/components/DevPanel.tsx`, and
   `apps/web/src/components/ReplayViewer.tsx` (all verified to contain
   `high_card_duel`/`HighCardDuel` references).
2. The required catalog identity is fixed by
   `specs/gate-9-token-bazaar-browser-proof.md` → "WASM and browser integration
   requirements" (game picker lists `Token Bazaar` with `game_id = token_bazaar`,
   `variant_id = token_bazaar_standard`; hotseat / human-vs-bot / bot-vs-bot modes).
3. Cross-artifact boundary under audit: the TS↔WASM client contract
   (`docs/WASM-CLIENT-BOUNDARY.md`). The client types must mirror the Rust bridge
   payloads from -013 exactly (view fields, effect kinds, action-tree node
   metadata, replay export shape); a drifted TS type would silently mis-render.
4. FOUNDATIONS §2 (TypeScript presentation-only): these bindings only deserialize
   and type Rust payloads and route commands back through WASM. TypeScript computes
   no legality, affordability, refill, winner, terminal outcome, or bot policy —
   restating the invariant before trusting the spec narrative. The dev panel and
   replay viewer must not expose internal/debug fields (none cross the ABI).

## Architecture Check

1. Keeping the client bindings + catalog registration in one presentation ticket
   (separate from the board renderer, -015) gives a reviewable "the shell can now
   see token_bazaar" diff before any pixels; it mirrors the `high_card_duel`
   client/catalog wiring.
2. No backwards-compatibility aliasing/shims — additive catalog entry + types.
3. No engine behavior moves to TypeScript; `engine-core`/`game-stdlib` untouched.
   Behavior authority remains in Rust/WASM (§2).

## Verification Layers

1. Build + type-check succeed -> `npm --prefix apps/web run build`.
2. WASM smoke loads the new catalog entry -> `npm --prefix apps/web run smoke:wasm`.
3. Client types match the bridge payloads -> manual review against the -013 arms +
   `docs/WASM-CLIENT-BOUNDARY.md` (a type-drift would surface as a build error or
   mis-render in -015).
4. No-leak: dev panel / replay viewer expose no internal field -> manual review
   (full browser no-leak assertion in -016).

## What to Change

### 1. `apps/web/src/wasm/client.ts`

Add token_bazaar view/effect/action-tree/replay TypeScript types mirroring the
-013 bridge payloads, plus any client method wiring the catalog needs.

### 2. `apps/web/src/main.tsx`

Register `token_bazaar` (`game_id`, `variant_id = token_bazaar_standard`, modes)
in the shell catalog so the picker can list it.

### 3. `apps/web/src/components/DevPanel.tsx` + `ReplayViewer.tsx`

Recognize the new game id for inspection/replay display, mirroring the existing
per-game handling — presentation only.

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/components/DevPanel.tsx` (modify)
- `apps/web/src/components/ReplayViewer.tsx` (modify)

## Out of Scope

- The `TokenBazaarBoard` renderer + game picker entry + effect log + styles
  (GAT9TOKBAZBRO-015).
- e2e smoke + CI (GAT9TOKBAZBRO-016).
- Any rule/legality logic in TypeScript (forbidden — §2).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — type-checks and builds with the new bindings.
2. `npm --prefix apps/web run smoke:wasm` — the bridge loads the token_bazaar catalog entry.
3. Manual review: client types match the -013 bridge payloads.

### Invariants

1. TypeScript only deserializes/types Rust payloads and forwards commands; it
   decides no legality (§2).
2. The catalog identity is exactly `game_id = token_bazaar`,
   `variant_id = token_bazaar_standard`.

## Test Plan

### New/Modified Tests

1. `None new Rust tests` — verification is the web build + wasm smoke; the board
   render path is tested in GAT9TOKBAZBRO-015/016.

### Commands

1. `npm --prefix apps/web ci && npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:wasm`
3. The build + wasm-smoke pair is the correct boundary; visible rendering and the
   no-leak/a11y e2e are verified in GAT9TOKBAZBRO-015/016.

## Outcome (2026-06-08)

Completed the TypeScript/WASM presentation binding for `token_bazaar`.

- Added Token Bazaar public view, action metadata, terminal, replay hash, and
  outcome shapes to the web WASM client.
- Wired generic shell terminal/status handling, shared action/mode controls,
  the developer panel surface label, and replay snapshot recognition for the
  Rust-provided Token Bazaar payload.
- Tightened `smoke:wasm` so it asserts the exact `token_bazaar` catalog entry
  and `token_bazaar_standard` variant.

Manual review: the TS fields match the GAT9TOKBAZBRO-013 bridge serializer keys
(`supply`, `inventories`, `scores`, `turns_taken`, `market_slots`,
`queue_remaining`, `fulfilled`, `legal_actions`, `terminal`, `recent_effects`,
`private_view_status`, `hidden_fields`, and `ui`). TypeScript still only routes
Rust payloads and commands; legality, terminal state, and bot behavior remain in
Rust/WASM.

Verification:

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:wasm`
