# GAT9TOKBAZBRO-013: WASM registration (wasm-api arms + catalog)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api/Cargo.toml` (modify), `crates/wasm-api/src/lib.rs` (modify)
**Deps**: GAT9TOKBAZBRO-007

## Problem

The browser reaches Rust only through the WASM bridge. This ticket registers
`token_bazaar` across the `wasm-api` surface — catalog entry, setup, legal action
tree, command validate/apply, public-view projection, effects, and public replay
export/import — so the React shell can drive the game entirely through Rust. WASM
must honor the viewer (public view) and never expose internal data.

## Assumption Reassessment (2026-06-08)

1. The full game surface exists: setup/actions/rules/effects/visibility/replay
   (GAT9TOKBAZBRO-002…007). `crates/wasm-api/src/lib.rs` registers `high_card_duel`
   heavily (verified, ~89 refs across command/view/action-tree/replay arms +
   catalog) and `crates/wasm-api/Cargo.toml` carries the path dep (verified). This
   ticket adds the analogous `token_bazaar` path dep + arms.
2. The WASM surface to register is fixed by
   `specs/gate-9-token-bazaar-browser-proof.md` → "WASM and browser integration
   requirements" (game picker lists Token Bazaar with `game_id = token_bazaar`,
   `variant_id = token_bazaar_standard`; hotseat / human-vs-bot / bot-vs-bot;
   board data; legal controls from the Rust action tree; replay import/export).
3. Cross-artifact boundary under audit: the WASM JSON bridge contract from
   `docs/WASM-CLIENT-BOUNDARY.md`. Every arm must conform to the existing bridge
   shape (catalog, `get_view`, action-tree, command, effects, replay) so the TS
   client (-014) binds without bespoke shapes.
4. FOUNDATIONS §2 (behavior authority): WASM exposes Rust-computed legality/views/
   effects/bot decisions; it computes no rule logic itself and lets TypeScript
   decide nothing. `get_view` returns the viewer-safe public view from -006.
5. FOUNDATIONS §11 (viewer-safe, deterministic, no-leak): the action-tree and view
   the bridge exports must be the same deterministic, public-safe artifacts proved
   in -006/-007 — no internal/candidate/debug field crosses the ABI. Since the
   game is fully public there is no per-seat redaction, but `get_view` must still
   route through the viewer projection rather than dumping raw state, and replay
   export must carry no bot/debug field (proved by the `wasm-exported` trace, -010).
6. WASM schema arms: this ticket adds catalog + view + action-tree + command +
   effects + replay arms for `token_bazaar`. Consumers: the TS client + board
   (-014/-015). Additive — a new game alongside existing registrations; no
   existing arm is changed.

## Architecture Check

1. Mirroring the proven `high_card_duel` registration (minus the viewer-scoped
   export split, which a fully-public game does not need) keeps the bridge uniform
   and the diff reviewable; routing `get_view` through the -006 projection keeps
   the no-leak seam intact.
2. No backwards-compatibility aliasing/shims — additive arms + path dep.
3. `engine-core` untouched; `wasm-api` dispatches on the opaque game id and
   transports game-local payloads. No `game-stdlib` helper introduced.

## Verification Layers

1. WASM build succeeds with the new game -> `cargo build -p wasm-api --target wasm32-unknown-unknown`
   (and `npm --prefix apps/web run smoke:wasm` in -014/-016).
2. Bridge conforms to `docs/WASM-CLIENT-BOUNDARY.md` -> schema/serialization
   validation of catalog/view/action-tree/command/effect/replay arms.
3. `get_view` is viewer-safe -> no-leak check that the exported view/action-tree/
   replay carry no internal field (extends the -006/-007/-010 assertions across the ABI).
4. Determinism preserved across the ABI -> the `wasm-exported` golden trace (-010)
   reproduces through the WASM export path.

## What to Change

### 1. `crates/wasm-api/Cargo.toml` (modify)

Add `token_bazaar = { path = "../../games/token_bazaar" }`.

### 2. `crates/wasm-api/src/lib.rs` (modify)

Add the `token_bazaar` catalog entry (`game_id`, `variant_id =
token_bazaar_standard`, modes) and every dispatch arm: setup, `get_view` (viewer
projection), legal action tree, validate/apply command, effects, and public
replay export/import — mirroring the `high_card_duel` arms minus the hidden-info
viewer-scoped split.

## Files to Touch

- `crates/wasm-api/Cargo.toml` (modify)
- `crates/wasm-api/src/lib.rs` (modify)

## Out of Scope

- TS client bindings / catalog types (GAT9TOKBAZBRO-014).
- The React board + shell (GAT9TOKBAZBRO-015) and e2e smoke (GAT9TOKBAZBRO-016).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p wasm-api` — the crate compiles with the new game registered.
2. `cargo build -p wasm-api --target wasm32-unknown-unknown` — WASM target builds.
3. `cargo test -p wasm-api` — any bridge conformance tests pass for `token_bazaar`.

### Invariants

1. WASM exposes only Rust-computed legality/views/effects/bot decisions; no rule
   logic in the bridge (§2).
2. `get_view` returns the viewer-safe public projection; no internal/candidate/
   debug field crosses the ABI (§11).
3. Replay export through WASM reproduces the `wasm-exported` golden trace.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` (unit, if the bridge has per-game conformance tests) — token_bazaar arms.

### Commands

1. `cargo build -p wasm-api && cargo test -p wasm-api`
2. `rustup target add wasm32-unknown-unknown && cargo build -p wasm-api --target wasm32-unknown-unknown`
3. The browser-level `smoke:wasm` run is exercised in GAT9TOKBAZBRO-014/016 once
   the TS client binds the new catalog entry.
