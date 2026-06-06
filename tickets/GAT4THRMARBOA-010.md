# GAT4THRMARBOA-010: Web shell catalog + match setup for Three Marks

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/GamePicker.tsx`, `apps/web/src/components/MatchSetup.tsx`, `apps/web/src/state/shellReducer.ts`, `apps/web/src/wasm/client.ts`
**Deps**: GAT4THRMARBOA-009

## Problem

The Gate 3 shell presents a single game. Gate 4 must let users choose and start `three_marks` alongside `race_to_n` from the static catalog the Rust `list_games` operation returns, and select supported local modes (human-vs-bot, hotseat, bot-vs-bot where the shell already supports them). This is a catalog/registry presentation extension — no plugin system, no TypeScript legality.

## Assumption Reassessment (2026-06-06)

1. `apps/web/src/components/GamePicker.tsx`, `MatchSetup.tsx`, `apps/web/src/state/shellReducer.ts`, and `apps/web/src/wasm/client.ts` are the existing shell surfaces; the picker is driven by the WASM `list_games` catalog (generalized to include `three_marks` in GAT4THRMARBOA-009). Verified all four files exist.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §11.2 (static minimal registry, discriminated rendering, no TS rules), §11.3 (setup modes; document any mode not practical for Three Marks while preserving Race to N), §5.2 (`apps/web` selection/setup).
3. Cross-artifact boundary under audit: the WASM client boundary (`docs/WASM-CLIENT-BOUNDARY.md`) — the client transports catalog/setup JSON; TypeScript discriminates renderer type by game id/view discriminant, never decides legality.
4. FOUNDATIONS §2 (TypeScript presentation-only) and §7 (public UI is play-first) motivate this ticket: the picker/setup consume Rust-provided catalog metadata and dispatch Rust new-match calls; the TS layer chooses *which renderer* by discriminant but infers no rule state.

## Architecture Check

1. Driving the picker from the Rust `list_games` catalog (rather than a hardcoded TS game list) keeps the game set authoritative in Rust and makes adding games a registry change, not a UI rewrite — cleaner and matching the Gate 3 data flow. Alternative (TS-side game registry) duplicates authority and is rejected.
2. No backwards-compatibility aliasing/shims — the existing single-game path generalizes to read the catalog; `race_to_n` selection is preserved.
3. `engine-core` untouched; no `game-stdlib` change; TypeScript adds no rule logic (discriminant only).

## Verification Layers

1. Catalog-render invariant -> UI smoke (`smoke:ui`/`smoke:e2e`: picker shows both Race to N and Three Marks from the Rust catalog).
2. Start-match invariant -> UI smoke (selecting Three Marks dispatches a Rust `new_match` for `three_marks_standard` and transitions shell state).
3. TS-no-legality invariant -> FOUNDATIONS alignment check (§2/§11: setup/picker carry no legality; renderer selection is a discriminant, documented in `docs/UI.md` GAT4THRMARBOA-015).

## What to Change

### 1. `wasm/client.ts` + `shellReducer.ts`

Surface the multi-game catalog and a `game_id`/view discriminant in shell state; carry the selected game through match setup and into the active match. Preserve the existing `race_to_n` flow.

### 2. `GamePicker.tsx` + `MatchSetup.tsx`

Render the catalog (both games) from Rust metadata; allow starting a `three_marks_standard` match with the supported local modes. Where a current mode is not practical for Three Marks in Gate 4, document the concrete reason (carried into `docs/UI.md`) and keep Race to N behaviour intact.

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/state/shellReducer.ts` (modify)
- `apps/web/src/components/GamePicker.tsx` (modify)
- `apps/web/src/components/MatchSetup.tsx` (modify)

## Out of Scope

- The `ThreeMarksBoard` renderer (GAT4THRMARBOA-011) and board-aware replay view (012).
- Browser UI smoke test authoring (GAT4THRMARBOA-013).
- Any WASM registry change (done in 009).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — type-checks and builds with the multi-game catalog state.
2. `npm --prefix apps/web run smoke:ui` — picker shows both games; a Three Marks match starts.
3. Existing Race to N selection/start path still works (smoke green).

### Invariants

1. The catalog/game set is sourced from Rust `list_games`; TypeScript chooses the renderer by discriminant and decides no legality.
2. Race to N setup/selection is non-regressed.

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/GamePicker.tsx` / `MatchSetup.tsx` — multi-game selection/start (exercised by `smoke:ui`/`smoke:e2e`).

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. Full board-interaction smoke is added in 013; build + picker/start smoke is the correct boundary for the catalog/setup diff.
