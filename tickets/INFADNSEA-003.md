# INFADNSEA-003: Infra A — web setup/catalog seat-range presentation

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/MatchSetup.tsx`, `apps/web/src/components/GamePicker.tsx`
**Deps**: INFADNSEA-002

## Problem

Once Rust projects per-game seat-range metadata into the catalog payload (INFADNSEA-002), the setup and picker UIs must present it — show the supported seat counts / default, and display the Rust-owned validation message when a count is rejected — without inferring any legality in TypeScript. Today `MatchSetup.tsx` and `GamePicker.tsx` assume a fixed two-seat setup; this ticket makes them read the projected metadata and present it.

## Assumption Reassessment (2026-06-14)

1. `apps/web/src/components/MatchSetup.tsx` and `apps/web/src/components/GamePicker.tsx` exist and currently assume two seats; they read catalog data via `apps/web/src/wasm/client.ts` (seat-range types added in INFADNSEA-002).
2. Grounded in `specs/infra-a-d-n-seat-public-infrastructure.md` WB3, and `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md §2` (the browser may present setup controls and validation messages supplied by Rust/WASM but must not decide which counts are legal).
3. Shared boundary under audit: the catalog payload contract consumed by the setup UI — confirm the seat-range fields from INFADNSEA-002 are the only source the UI reads for supported counts (no TS-side default invented).
4. FOUNDATIONS §2 + §7: TypeScript presents Rust-supplied seat metadata and validation messages only; it derives no seat legality, no count list, and no rejection logic of its own (§12 "TypeScript decides legality" stays clear).

## Architecture Check

1. Reading the projected metadata is cleaner than a TS-side seat-count table: the supported counts have one Rust source, so the UI cannot drift from what Rust actually accepts.
2. No backwards-compat shim: the fixed two-seat assumption is replaced by metadata-driven presentation; for current games the metadata still yields exactly two seats.
3. No `engine-core`/`game-stdlib` impact (web-only); behavior authority stays in Rust (§2).

## Verification Layers

1. Setup UI renders the supported seat counts / default from the catalog metadata -> `smoke:ui` covers the setup surface.
2. A rejected count shows the Rust-supplied diagnostic, not a TS-authored message -> manual review + `smoke:ui` assertion that the rejection text originates from the bridge result.
3. No TS-side legality -> FOUNDATIONS §2/§7 alignment check (grep the components for any hardcoded seat-count list or rejection branch; there must be none).

## What to Change

### 1. Drive setup from seat-range metadata

`MatchSetup.tsx` reads `min_seats`/`max_seats`/`default_seats`/`supported_seats` from the catalog entry and offers exactly those counts (for current games, the single value 2). The submit path surfaces the Rust diagnostic on rejection.

### 2. Surface supported counts in the picker

`GamePicker.tsx` shows each game's supported seat count(s) from the same metadata.

## Files to Touch

- `apps/web/src/components/MatchSetup.tsx` (modify)
- `apps/web/src/components/GamePicker.tsx` (modify)

## Out of Scope

- The in-match multi-seat shell frame (INFADNSEA-005/006).
- Any change to Rust seat-count acceptance (INFADNSEA-001) or catalog projection (INFADNSEA-002).
- Casino/trade-dress styling (spec §3.3 / Gate 15 game UI).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` — setup/picker render the projected seat metadata; no console errors.
2. `npm --prefix apps/web run build` — type-check passes against the INFADNSEA-002 `client.ts` types.

### Invariants

1. The supported seat counts and any rejection message come only from Rust/WASM-projected metadata; TypeScript invents no seat-count list or legality branch.
2. Current games still present exactly two seats (no behavior change for shipped games).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` — extend to assert the setup surface reflects catalog seat metadata.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run build`
