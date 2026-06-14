# INFADNSEA-002: Infra A — per-game seat-range catalog metadata + client.ts types

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api` (catalog payload seat metadata) + `apps/web/src/wasm/client.ts` (presentation types); no `engine-core` or `games/*` behavior change
**Deps**: INFADNSEA-001

## Problem

The catalog payload the browser reads hardcodes `"viewer_modes":["observer","seat_0","seat_1"]` per game (`crates/wasm-api/src/lib.rs:461-520`) and carries no seat-range / supported-count / default / seat-label metadata, so the setup UI has no Rust-owned source for which seat counts a game supports. A `variant.seat_count` value already exists in each game (`games/race_to_n/src/setup.rs:27`), but it is never projected to the catalog the consumer reads. This ticket extends that existing value to a seat-range (min / max / supported set / default + stable seat labels) and projects it into the catalog payload, with matching `client.ts` types — additive, viewer-safe, no behavior fields.

## Assumption Reassessment (2026-06-14)

1. Catalog metadata is assembled per game in `crates/wasm-api/src/lib.rs:461-520` (hardcoded `viewer_modes`, `variants`, `tags`); `variant.seat_count` already exists in game crates (`games/race_to_n/src/setup.rs:27`). The browser type mirror is `apps/web/src/wasm/client.ts`.
2. Grounded in `specs/infra-a-d-n-seat-public-infrastructure.md` WB2 + §2, and `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md §2` (Minimum/Maximum/Default seats, supported sets, seat labels) and §5 (viewer matrix — seat labels are public).
3. Shared boundary under audit: the catalog payload contract (`wasm-api` producer ↔ `client.ts` consumer). This is a presentation-metadata extension, not an `engine-core` contract change.
4. FOUNDATIONS §5 (static data is typed content): seat-range/label metadata is typed content/parameters only — no selectors, branches, or behavior-looking fields; FOUNDATIONS §2: the browser presents this metadata, it does not derive seat legality from it.
5. Schema extension: the catalog payload gains seat-range fields. Consumers: `apps/web/src/wasm/client.ts` (updated here) and the catalog doc checker `scripts/check-catalog-docs.mjs` (keyed on the catalog const — confirm it tolerates the additive fields). The extension is additive-only (new fields with defaults), so existing catalog consumers are unaffected.

## Architecture Check

1. Extending the existing `variant.seat_count` into a projected range is cleaner than inventing a new seat-metadata source: the game crate stays the single seat-count authority and the bridge only projects it.
2. No backwards-compat shim: additive fields with defaults; the hardcoded `viewer_modes` array is derived from the seat range where it was previously a literal.
3. `engine-core` untouched; no `game-stdlib` change. Seat-range metadata lives in `wasm-api`/`games/*`, not the kernel.

## Verification Layers

1. Catalog payload carries seat-range/default/label fields for every game -> `wasm-api` unit/serialization test asserting the fields are present and additive.
2. `client.ts` types match the payload shape -> `npm --prefix apps/web run build` (type-check) + `smoke:wasm`.
3. No behavior-looking fields in the metadata -> manual review + FOUNDATIONS §5 alignment check (fields are scalars/labels only).
4. Catalog doc checker still passes -> `node scripts/check-catalog-docs.mjs`.

## What to Change

### 1. Project seat-range metadata into the catalog payload

For each game, derive and emit `min_seats` / `max_seats` / `default_seats` / `supported_seats` (when discontinuous) / `seat_labels` from the game's existing `variant.seat_count` (range collapses to `[2,2]` for every current game). Derive `viewer_modes` from the seat range rather than the current hardcoded literal.

### 2. Mirror the types in `client.ts`

Add the seat-range fields to the catalog entry type in `apps/web/src/wasm/client.ts` so the setup UI (INFADNSEA-003) can read them; no rendering logic here.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify; shares this file with INFADNSEA-001 — land after it)
- `apps/web/src/wasm/client.ts` (modify)

## Out of Scope

- Setup/catalog UI rendering of the metadata (INFADNSEA-003).
- Any game declaring a range >2 (Gate 15).
- Trace/WASM-schema/hash migration beyond additive viewer-safe catalog fields (spec §3.3).

## Acceptance Criteria

### Tests That Must Pass

1. `wasm-api` test: every catalog entry exposes `min_seats`/`max_seats`/`default_seats`/`seat_labels`, all `[2,2]`/two-label for current games.
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:wasm` — `client.ts` types compile and the bridge loads.
3. `node scripts/check-catalog-docs.mjs` — catalog-doc checker passes with the additive fields.

### Invariants

1. Seat-range metadata is typed content only — no selectors, conditions, or behavior fields (§5).
2. The extension is additive: existing catalog consumers that ignore the new fields still work.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` (`#[cfg(test)]`) — catalog seat-range projection test.
2. `apps/web/src/wasm/client.ts` — type additions exercised by the existing `smoke:wasm` harness.

### Commands

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:wasm && node scripts/check-catalog-docs.mjs`

## Outcome

Completed: 2026-06-14

- Added uniform, additive catalog seat metadata for every registered game: `min_seats`, `max_seats`, `default_seats`, `supported_seats`, `seat_labels`, and derived `viewer_modes`.
- Removed the prior per-game hardcoded `viewer_modes` fragments from catalog JSON assembly so viewer modes now come from the shared seat metadata helper.
- Mirrored the new catalog fields in `apps/web/src/wasm/client.ts`.
- Added a `wasm-api` catalog regression assertion for the new seat metadata fields; current official games project `[2,2]` / two labels as expected.
- Deviations: no setup/catalog rendering was changed; that remains owned by INFADNSEA-003.
- Verification: `cargo test -p wasm-api`; `node scripts/check-catalog-docs.mjs`; `npm --prefix apps/web run build`; `npm --prefix apps/web run smoke:wasm`.
