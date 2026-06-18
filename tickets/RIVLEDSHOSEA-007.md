# RIVLEDSHOSEA-007: Make setup prose and role rows use the selected supported seat count

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/MatchSetup.tsx`
**Deps**: RIVLEDSHOSEA-005

## Problem

`apps/web/src/components/MatchSetup.tsx::modeDetail()` and `setupSeatRoles()` build setup prose and role rows from the full catalog label set (`setupLabels(game)`), not the selected seat count. A four-seat River Ledger setup therefore claims six local/automated seats. `selectedSeatCount` is already computed in the component but unused for this copy. This ticket threads the selected supported count (and the corresponding Rust-authored catalog labels in authoritative order) into both functions so setup copy names exactly the chosen seats (spec §8.2 TS / D4).

## Assumption Reassessment (2026-06-18)

1. `apps/web/src/components/MatchSetup.tsx`: `modeDetail(playMode, game)` and `setupSeatRoles(game, playMode)` both read `setupLabels(game)` (all catalog labels); `selectedSeatCount = seatCount ?? defaultSeatCount` is already defined in the component (≈ line 54) but not passed into those functions. Confirmed.
2. Spec D4 / §8.2: before a match exists, setup copy must consume the selected supported seat count and Rust catalog labels in authoritative order — never `max_seats` as the current choice. RIVLEDSHOSEA-005 establishes the capability-vs-active label contract and the one-based catalog labels (via RIVLEDSHOSEA-002) this copy draws from.
3. Shared boundary under audit: the setup-copy path in `MatchSetup.tsx` and the catalog label/count source it reads. End state: `modeDetail`/`setupSeatRoles` receive `selectedSeatCount` (or the resolved selected label rows); they never pad to `max_seats`.
4. FOUNDATIONS §2 (TS presentation-only): the selected count is a user choice already owned by the shell; this ticket only routes it into copy generation. No legality or seat-count validity is decided in TS — Rust still validates the count (RIVLEDSHOSEA-005). Restated before trusting the spec.

## Architecture Check

1. Passing the already-computed `selectedSeatCount` into the two copy builders is the minimal change that removes the phantom-seat copy; failing closed to count-only generic copy on a count/label mismatch is safer than silently showing `max_seats` rows.
2. No shim: the builders stop reading the full catalog list for match-intent copy; no fallback pads to capability.
3. Presentation-only; `engine-core` and Rust ownership untouched (§2).

## Verification Layers

1. Selected-count copy is exact -> component test: at counts `3,4,5,6`, `modeDetail` and `setupSeatRoles` name exactly that many seats (hotseat says Seats 1–N are local; bot-vs-bot says N seats automated).
2. Fail-closed on mismatch -> component test: when selected count and catalog labels disagree, copy falls back to count-only generic text and raises a dev assertion (no phantom seats).
3. No capability leakage into match-intent copy -> grep-proof that `modeDetail`/`setupSeatRoles` no longer read the full `setupLabels(game)` for per-seat rows.

## What to Change

### 1. Thread the selected count

Pass `selectedSeatCount` (or resolved selected label rows) into `modeDetail()` and `setupSeatRoles()`; generate role rows and prose for exactly that many seats using the Rust catalog labels in authoritative order. Keep capability copy ("River Ledger supports up to six seats") allowed elsewhere, just out of match-scoped role rows.

### 2. Fail closed

If the selected count and catalog labels disagree, render count-only generic copy and raise a dev assertion rather than showing `max_seats` rows.

## Files to Touch

- `apps/web/src/components/MatchSetup.tsx` (modify)

## Out of Scope

- The Rust/WASM active-seat projection (RIVLEDSHOSEA-005).
- The live-match seat rail / viewpoint selector (RIVLEDSHOSEA-006).
- Removing capability copy that legitimately states the supported range.
- Any Rust change.

## Acceptance Criteria

### Tests That Must Pass

1. Component tests at counts `3,4,5,6`: setup copy and role rows name exactly that many seats; a four-seat hotseat says Seats 1–4 local and does not mention Seats 5–6; a four-seat bot-vs-bot says all 4 seats automated.
2. Fixed two-seat games still show exactly two setup role rows.
3. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e` (River Ledger + shell setup smoke) green.

### Invariants

1. Match-scoped setup copy reflects the selected supported count, never `max_seats`.
2. A selected-count/label mismatch fails closed to generic count-only copy, never phantom seats.

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/MatchSetup.tsx` (or its test module) — selected-count setup copy + role rows at `3,4,5,6` and the fail-closed path.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web ci && npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e`
3. Narrower `smoke:ui` is the correct boundary because setup copy renders before any match/WASM viewer request.
