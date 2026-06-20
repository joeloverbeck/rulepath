# GAT16BRICIRTRI-013: WASM catalog, operation groups, adapter, and player-rules generation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api/src/{constants,catalog,games}.rs` + `src/games/briar.rs`, `apps/web/scripts/smoke-load-wasm.mjs`, `apps/web/public/rules/{briar_circuit.md,manifest.json}`, `scripts/check-player-rules.mjs`
**Deps**: 002, 012

## Problem

Briar Circuit must be exposed through the Rust↔browser WASM bridge: catalog metadata with fixed-four-seat support, the game adapter and dispatch, the setup/action/view/effect/replay/bot/outcome operation groups, viewer modes, and the pairwise no-leak harness dispatch — translating opaque envelopes only, with no new exported API schema. This ticket also generates the player-rules asset and registers the game as hidden-info so `check-player-rules` passes.

## Assumption Reassessment (2026-06-20)

1. `crates/wasm-api/src/games.rs` declares per-game modules (`plain`, `river`, …); `catalog.rs` holds the registered-game entries; `constants.rs` holds `GAME_*`/`GAME_*_DISPLAY_NAME` const pairs. Adapters (`games/plain.rs`, `games/river.rs`) are pure JSON translation. `apps/web/scripts/smoke-load-wasm.mjs` has **hardcoded per-game catalog assertions** (`catalog.some(g => g.game_id === "...")`) — a new game needs an added assertion. `scripts/check-player-rules.mjs` carries a `HIDDEN_INFO_GAMES` set.
2. `specs/gate-16-briar-circuit-trick-taking.md` §4.4 (WASM crate / WASM game adapter / Public rules rows), §10.5, and the reassessment-added concrete sites (`constants.rs` consts; `games/briar.rs` adapter) fix the registration. Adapter naming follows the short-name convention → `crates/wasm-api/src/games/briar.rs`.
3. Cross-artifact boundary under audit: the WASM payload applies the same viewer filtering as the Rust projection (no convenience "all state" field, debug field, or hidden fallback); `apps/web/public/rules/briar_circuit.md` is generated from `games/briar_circuit/docs/HOW-TO-PLAY.md` (002) via `scripts/copy-player-rules.mjs` and validated by `scripts/check-player-rules.mjs`.
4. FOUNDATIONS §11 no-leak firewall is under audit at the bridge: the WASM JSON must carry no private datum the Rust views forbid; the pairwise no-leak harness dispatch runs the same canary checks as the native visibility tests (009) against the serialized payloads.
5. Schema-extension check: adding `briar_circuit` extends the WASM catalog/operation surface **additively** (a new game-id arm using existing operation-group shapes); no new exported API schema, no breaking change — consumers read the same envelope contract (spec §4.4, Assumption A1).

## Architecture Check

1. A pure-translation `briar.rs` adapter (over reimplementing any rule in the bridge) keeps behavior in `games/briar_circuit` and the bridge presentation-agnostic, matching the `plain`/`river` adapters.
2. No backwards-compatibility aliasing/shims — additive catalog/module entries; no API-schema change.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); the adapter translates opaque envelopes only.

## Verification Layers

1. Catalog lists `briar_circuit` with fixed-four-seat metadata; all operation groups dispatch -> `npm --prefix apps/web run smoke:wasm` (+ added `smoke-load-wasm.mjs` assertion).
2. WASM JSON applies the same viewer filtering; pairwise no-leak harness passes over the payload -> `cargo test -p wasm-api` + harness dispatch (same canaries as 009).
3. Player-rules asset generated and valid; game registered hidden-info -> `node scripts/copy-player-rules.mjs && node scripts/check-player-rules.mjs`.
4. No new exported API schema -> `cargo test -p wasm-api` api-surface snapshot unchanged except the additive game arm.

## What to Change

### 1. WASM registration

`constants.rs` (`GAME_BRIAR_CIRCUIT`/`…_DISPLAY_NAME`), `catalog.rs` (registered-game entry + fixed-four-seat metadata + variant copy), `games.rs` (`mod briar` + dispatch), and `crates/wasm-api/src/games/briar.rs` (the translation adapter with setup/action/view/effect/replay/bot/outcome operation groups, viewer modes, pairwise no-leak harness dispatch).

### 2. `apps/web/scripts/smoke-load-wasm.mjs`

Add the `catalog.some(g => g.game_id === "briar_circuit")` assertion.

### 3. Player rules + hidden-info registration

Generate `apps/web/public/rules/briar_circuit.md` from `HOW-TO-PLAY.md` via `scripts/copy-player-rules.mjs`, add the `apps/web/public/rules/manifest.json` row, and add `briar_circuit` to `HIDDEN_INFO_GAMES` in `scripts/check-player-rules.mjs`.

## Files to Touch

- `crates/wasm-api/src/constants.rs` (modify)
- `crates/wasm-api/src/catalog.rs` (modify)
- `crates/wasm-api/src/games.rs` (modify)
- `crates/wasm-api/src/games/briar.rs` (new)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify)
- `apps/web/public/rules/briar_circuit.md` (new — generated)
- `apps/web/public/rules/manifest.json` (modify)
- `scripts/check-player-rules.mjs` (modify — `HIDDEN_INFO_GAMES`)

## Out of Scope

- The React board renderer and outcome-explanation web copy (GAT16BRICIRTRI-014) — this ticket opens an expected `check-catalog-docs` red window (catalog const added) until README reconciliation in 015.
- The e2e smoke and `smoke:e2e` wiring (GAT16BRICIRTRI-015).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — adapter, fixed-seat metadata, operation groups, pairwise no-leak harness; api-surface snapshot additive-only.
2. `npm --prefix apps/web run smoke:wasm` — catalog includes `briar_circuit`; all operation groups load.
3. `node scripts/copy-player-rules.mjs && node scripts/check-player-rules.mjs` — player-rules asset valid; hidden-info section present.

### Invariants

1. WASM payloads carry no private datum the Rust views forbid (§11 no-leak); no "all state" escape hatch.
2. The adapter reimplements no rule; behavior stays in `games/briar_circuit` (§2).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/games/briar.rs` — translation adapter exercised by `cargo test -p wasm-api`.
2. `apps/web/scripts/smoke-load-wasm.mjs` — added `briar_circuit` catalog assertion.
3. `apps/web/public/rules/briar_circuit.md` (generated) — validated by `check-player-rules`.

### Commands

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run smoke:wasm`
3. `node scripts/copy-player-rules.mjs && node scripts/check-player-rules.mjs`
