# GAT20STACROSTA-014: WASM/API + catalog registration and player rules

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api/src/{constants,lib,catalog,games}.rs` + `games/starbridge_crossing.rs`, `apps/web/scripts/smoke-load-wasm.mjs`; game-local `HOW-TO-PLAY.md` + generated `apps/web/public/rules/starbridge_crossing.md`
**Deps**: GAT20STACROSTA-001, GAT20STACROSTA-011

## Problem

The browser shell reaches the game through the WASM bridge. This ticket registers `starbridge_crossing` across the wasm-api surfaces and the build-step WASM smoke, and lands `HOW-TO-PLAY.md` + its generated player-rules markdown, which `check-player-rules` validates (keyed off the catalog const).

## Assumption Reassessment (2026-06-27)

1. WASM registration sites (mirroring meldfall): `crates/wasm-api/src/constants.rs` (`GAME_STARBRIDGE_CROSSING` + display-name consts), `lib.rs` (`use games::starbridge_crossing::*`, `RegisteredGame::StarbridgeCrossing`, `MatchRecord` arm, default-seats, dispatch), `catalog.rs` (`RegisteredGame::StarbridgeCrossing` + `list_games` JSON entry), `games.rs` (`pub(crate) mod starbridge_crossing;`), and a new `crates/wasm-api/src/games/starbridge_crossing.rs` — all confirmed against the meldfall registration sites.
2. `apps/web/scripts/smoke-load-wasm.mjs` carries a per-game `assert(catalog.some(... game_id === "<g>"))` list (confirmed line 65+) — a `(modify)` target here.
3. Player-rules pattern: `scripts/copy-player-rules.mjs` generates `apps/web/public/rules/<id>.md` from `games/<id>/docs/HOW-TO-PLAY.md`, guarded by `scripts/check-player-rules.mjs` (keyed off the catalog const). Starbridge is **perfect-information**, so it is NOT added to `HIDDEN_INFO_GAMES`; no hidden-information section is required.
4. §2 (behavior authority) motivates this ticket: the WASM bridge serializes Rust-owned state/action/effect/public-view surfaces; TypeScript receives already-safe payloads and decides no legality.
5. Determinism/visibility surface (§11): the WASM public-view bridge must match the native public view (010) byte-for-equivalent; confirm the exported state/action/effect surface carries no datum absent from the native public view (perfect information — nothing private to redact).

## Architecture Check

1. Following the established wasm-api registration shape keeps the bridge uniform; co-landing `HOW-TO-PLAY.md` + generated rules with the catalog-const ticket avoids a `check-player-rules` red window.
2. No backwards-compatibility shims.
3. `engine-core` untouched; `wasm-api` is the bridge crate (correct placement); no mechanic noun added to the kernel.

## Verification Layers

1. Catalog registration -> `cargo test -p wasm-api` + `npm --prefix apps/web run smoke:wasm` (list_games includes `starbridge_crossing`).
2. WASM public-view parity (§11) -> schema/serialization validation: the bridge view equals the native public view; add the `wasm-exported` golden trace (catalog from 011).
3. Player-rules generation -> `node scripts/copy-player-rules.mjs && node scripts/check-player-rules.mjs`.
4. Boundary -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. Register across `crates/wasm-api/src/`

`constants.rs`, `lib.rs`, `catalog.rs`, `games.rs`, and new `games/starbridge_crossing.rs` (view/create-match glue), mirroring meldfall; `list_games` JSON with `hidden_information:false`, `min/max/default/supported_seats` `{2,3,4,6}`, seat labels, doc paths (`RULES.md`/`SOURCES.md`/`HOW-TO-PLAY.md`).

### 2. Author `games/starbridge_crossing/docs/HOW-TO-PLAY.md` + generate player rules

Original player-facing rules; run `copy-player-rules.mjs` to produce `apps/web/public/rules/starbridge_crossing.md`.

### 3. Update `apps/web/scripts/smoke-load-wasm.mjs`

Add the `list_games includes starbridge_crossing` assertion (+ variant check).

## Files to Touch

- `crates/wasm-api/src/constants.rs` (modify)
- `crates/wasm-api/src/lib.rs` (modify)
- `crates/wasm-api/src/catalog.rs` (modify)
- `crates/wasm-api/src/games.rs` (modify)
- `crates/wasm-api/src/games/starbridge_crossing.rs` (new)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify)
- `games/starbridge_crossing/docs/HOW-TO-PLAY.md` (new)
- `apps/web/public/rules/starbridge_crossing.md` (new; generated)
- `games/starbridge_crossing/tests/golden_traces/wasm-exported.trace.json` (new; catalog from 011)

## Out of Scope

- The board renderer component + UI smoke — GAT20STACROSTA-015.
- Catalog README surfaces + `ci/games.json` + `smoke:e2e` — GAT20STACROSTA-017 (interim `check-catalog-docs` red window expected).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run smoke:wasm`
3. `node scripts/check-player-rules.mjs && bash scripts/boundary-check.sh`

### Invariants

1. The WASM bridge exposes only Rust-owned, viewer-safe public facts (§2/§11).
2. `HOW-TO-PLAY.md` and its generated markdown pass `check-player-rules` (perfect-info; not in `HIDDEN_INFO_GAMES`).

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/tests/golden_traces/wasm-exported.trace.json` — bridge surface matches native expectation.
2. `apps/web/scripts/smoke-load-wasm.mjs` — catalog-inclusion assertion (modified).

### Commands

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run smoke:wasm && node scripts/check-player-rules.mjs`
3. `check-catalog-docs` is intentionally deferred to 017; the WASM + player-rules checks are the correct boundary here.
