# GAT18BLAPACSPA-014: WASM adapter, catalog, dispatch, and player-rules surfaces

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api` (adapter/catalog/dispatch), `apps/web/public/rules`, `games/blackglass_pact/docs/HOW-TO-PLAY.md`, `scripts/check-player-rules.mjs`, `apps/web/scripts/smoke-load-wasm.mjs`
**Deps**: GAT18BLAPACSPA-009, GAT18BLAPACSPA-011

## Problem

Expose Blackglass Pact through the Rust↔browser bridge: a `crates/wasm-api/src/games/blackglass.rs` adapter, the `lib.rs` enum variant + `use` + action/view/effect/replay/export/import/bot dispatch + catalog record (fixed-four, public + four seat viewers), the player-facing `HOW-TO-PLAY.md` with its hidden-information section, the generated `apps/web/public/rules/blackglass_pact.md` + manifest entry, `HIDDEN_INFO_GAMES` registration, and the `smoke-load-wasm.mjs` catalog assertion (spec §4.5, §10.6–§10.7, candidate task `GAT18-BLAPAC-012`).

## Assumption Reassessment (2026-06-25)

1. `crates/wasm-api/src/lib.rs` carries per-game state-enum variants (`BriarCircuit { game_id, ..., state }:265`) + `use games::briar::*;`; the per-game adapter lives in `crates/wasm-api/src/games/<g>.rs` (sibling `vow.rs`). Blackglass Pact follows that pattern.
2. `scripts/check-player-rules.mjs` has a `HIDDEN_INFO_GAMES` set (`:27`) and requires a real "Hidden information and reveal timing" section (≥80 chars, not "not applicable") for listed games — Blackglass Pact is hidden-info, so it must register there and fill that section. `apps/web/public/rules/<g>.md` is generated from `HOW-TO-PLAY.md` via `scripts/copy-player-rules.mjs` and guarded by `check-player-rules.mjs` (`manifest.json` carries the sha256).
3. `apps/web/scripts/smoke-load-wasm.mjs` carries hardcoded `catalog.some(... game_id === "<id>")` assertions (`:65+`) → a `(modify)` target here.
4. FOUNDATIONS §2 (TS presentation-only) / §11 (no-leak) motivate this ticket: the WASM bridge transports already-safe viewer payloads and recomputes no legality/score/visibility; the export carries no full hidden state.

## Architecture Check

1. One thin adapter + catalog record reusing the shared `wasm-api` transport (vs. a per-game transport) keeps legality/score/visibility in Rust and the bridge presentation-only.
2. No shims; the generated player-rules md is produced via `copy-player-rules.mjs`, never hand-edited.
3. `engine-core` untouched; no `game-stdlib` change; bridge adapter only.

## Verification Layers

1. Catalog lists `blackglass_pact` with fixed-four + 5 viewer modes; Rust/WASM parity -> `cargo test -p wasm-api` + `npm --prefix apps/web run smoke:wasm`.
2. Player-rules generated + valid; hidden-info section present -> `node scripts/copy-player-rules.mjs && node scripts/check-player-rules.mjs`.
3. No hidden state in WASM payloads/exports -> wasm-api no-leak tests (reuse the seat-private export checks).

## What to Change

### 1. WASM adapter + dispatch

`crates/wasm-api/src/games/blackglass.rs` (new) + `lib.rs` (modify): `use games::blackglass::*;`, state-enum variant, action/view/effect/replay/export/import/bot dispatch, fixed-four setup, catalog record (display name, rules/data versions, variants, viewer modes); add `blackglass_pact` dep to `crates/wasm-api/Cargo.toml`; bounded API-snapshot test rows.

### 2. Player-rules surfaces

`games/blackglass_pact/docs/HOW-TO-PLAY.md` (new) incl. the "Hidden information and reveal timing" section; generate `apps/web/public/rules/blackglass_pact.md` + `manifest.json` entry via `copy-player-rules.mjs`; add `blackglass_pact` to `HIDDEN_INFO_GAMES` in `scripts/check-player-rules.mjs`.

### 3. Smoke catalog assertion

`apps/web/scripts/smoke-load-wasm.mjs`: add the `blackglass_pact` catalog assertion.

## Files to Touch

- `crates/wasm-api/src/games/blackglass.rs` (new)
- `crates/wasm-api/src/lib.rs` (modify), `crates/wasm-api/Cargo.toml` (modify)
- `games/blackglass_pact/docs/HOW-TO-PLAY.md` (new)
- `apps/web/public/rules/blackglass_pact.md` (new — generated), `apps/web/public/rules/manifest.json` (modify)
- `scripts/check-player-rules.mjs` (modify), `apps/web/scripts/smoke-load-wasm.mjs` (modify)

## Out of Scope

- React board renderer + outcome-explanation web surfaces (GAT18BLAPACSPA-015).
- e2e smoke + catalog README reconciliation (GAT18BLAPACSPA-016).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` (catalog/setup/actions/views/effects/replay/exports/bot dispatch + snapshot rows).
2. `npm --prefix apps/web run smoke:wasm` (catalog includes `blackglass_pact`).
3. `node scripts/copy-player-rules.mjs && node scripts/check-player-rules.mjs` (hidden-info game has real reveal-timing prose).

### Invariants

1. The bridge decides no legality/score/visibility; payloads/exports are already viewer-safe.
2. The generated player-rules md is produced from `HOW-TO-PLAY.md`, never hand-edited; manifest sha256 matches.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api` snapshot/dispatch tests — `blackglass_pact` rows.
2. `apps/web/scripts/smoke-load-wasm.mjs` — catalog assertion.
3. `apps/web/public/rules/blackglass_pact.md` — generated, checked by `check-player-rules.mjs`.

### Commands

1. `cargo test -p wasm-api && npm --prefix apps/web run smoke:wasm`
2. `node scripts/copy-player-rules.mjs && node scripts/check-player-rules.mjs`
3. Expected red window: `check-catalog-docs` / `check-outcome-explanations` stay red until GAT18BLAPACSPA-016/017 land the README + UI.md surfaces.
