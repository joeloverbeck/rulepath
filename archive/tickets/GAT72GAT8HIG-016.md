# GAT72GAT8HIG-016: WASM high_card_duel registration (all arms + catalog)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (game imports, `resolve_game`, `new_match`, `get_view`, `get_action_tree`, `apply_action`, `run_bot_turn`, `get_effects`, `export_replay`, `import_replay`, `catalog`)
**Deps**: GAT72GAT8HIG-015, GAT72GAT8HIG-008, GAT72GAT8HIG-009, GAT72GAT8HIG-010, GAT72GAT8HIG-022

## Problem

`high_card_duel` must be exposed through the WASM boundary as a first-class
hidden-information game: registered in the catalog with viewer-mode/hidden-info
tags, and wired into every export so views/action-trees/effects/replay are
viewer-filtered and no hidden state crosses to TypeScript.

## Assumption Reassessment (2026-06-07)

1. Verified the registration pattern: `crates/wasm-api/src/lib.rs` imports each
   game's `apply_action`/`legal_action_tree`/`project_view`/`replay_commands`/
   `setup_match` (lines 22-26 for draughts), defines `GAME_<NAME>` consts (line
   54), `resolve_game` (line 235-area), per-game arms in `new_match`/`get_view`/
   `get_action_tree`/`apply_action`/`run_bot_turn`/`get_effects`/`export_replay`/
   `import_replay`, and a `catalog` entry (lines 179-212). `get_view` is now
   viewer-honoring after GAT72GAT8HIG-015.
2. Verified against the spec: §4.2.5 table fixes each surface's hidden-info
   requirement (viewer-filtered view, actor-private action tree, bot acting from
   allowed input only, viewer-scoped default replay export, public-vs-internal
   import distinction, catalog tags). §8.2 forbids shipping hidden state to React.
3. Cross-artifact boundary under audit: the WASM catalog + per-surface contracts
   and the public/viewer replay-export taxonomy from 009 (ADR 022). `export_replay`
   for `high_card_duel` defaults to the viewer-scoped/public-safe form, not the
   internal full trace.
4. FOUNDATIONS principle under audit (§11 no-leak firewall + §2): every browser-
   bound payload for this game is viewer-authorized in Rust; legality/bot/replay
   stay Rust-owned.
5. Enforcement surface named: the §11 no-leak firewall at every WASM arm.
   Confirm `get_view`/`get_effects` filter by viewer; `get_action_tree` is
   actor-private/authorized; `run_bot_turn` leaks no bot candidates/opponent
   data; `export_replay` is viewer-scoped by default; `import_replay`
   distinguishes public projection from internal full trace.
6. Schema/contract extension classification: additive new-game arms + a catalog
   entry (additive). Consumers: the TS client/catalog (017) and the e2e smoke
   (019). No existing game arm changes (those were hardened in 015).

## Architecture Check

1. Registering `high_card_duel` through the same viewer-aware arms (hardened in
   015) keeps one boundary for all games; the hidden-info filtering is the game's
   `project_view`/effect/replay logic, not a wasm-api special case.
2. No backwards-compatibility shims — additive arms.
3. `engine-core` untouched and noun-free; card vocabulary stays in the game crate
   the wasm-api imports.

## Verification Layers

1. View/effect filtering -> no-leak visibility test: `get_view`/`get_effects` for `high_card_duel` return observer/seat-filtered payloads with no hidden identity.
2. Action-tree privacy -> no-leak test: `get_action_tree` returns only the authorized actor's tree.
3. Replay export default -> no-leak + deterministic replay check: `export_replay` defaults to the viewer-scoped public-safe form; `import_replay` rebuilds a public timeline.
4. Catalog registration -> schema/serialization validation: `catalog` lists `high_card_duel` with viewer-mode/hidden-info tags.

## What to Change

### 1. wasm-api registration

Import the game's surfaces; add `GAME_HIGH_CARD_DUEL` + `resolve_game` arm + the
standard variant/rules-version consts; add per-game arms to `new_match`,
`get_view` (viewer-filtered), `get_action_tree` (authorized), `apply_action`,
`run_bot_turn` (allowed input only), `get_effects` (viewer-filtered),
`export_replay` (viewer-scoped default), `import_replay` (public-vs-internal),
and a `catalog` entry with hidden-info/viewer-mode tags.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)

## Out of Scope

- TS bindings/catalog consumption (017) and the web UI (018).
- The general viewer-hardening of existing games (done in 015).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — including new `high_card_duel` viewer-filtering/no-leak arm tests.
2. `npm --prefix apps/web run smoke:wasm` — the ABI loads with the new game registered.
3. `cargo build -p wasm-api --target wasm32-unknown-unknown --release` — wasm builds.

### Invariants

1. No WASM arm for `high_card_duel` emits hidden state to an unauthorized viewer (§11).
2. `export_replay` default for this game is viewer-scoped/public-safe, not the internal full trace.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api` tests — `high_card_duel` per-arm viewer-filtering + no-leak + replay-export-default assertions.

### Commands

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run smoke:wasm`
3. wasm-api tests + the wasm load smoke are the correct boundary; browser DOM/no-leak is proven in 019.
