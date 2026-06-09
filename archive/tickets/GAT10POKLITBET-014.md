# GAT10POKLITBET-014: WASM registration and viewer-scoped browser bridge

**Status**: DONE
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (new `RegisteredGame`/`MatchRecord` variant, `GAME_*` consts, serializers, viewer-scoped bridge), `crates/wasm-api/Cargo.toml` (dep), `games/poker_lite/tests/golden_traces/wasm-exported.trace.json`. No `engine-core`/`game-stdlib` change.
**Deps**: GAT10POKLITBET-010

## Problem

The browser reaches `poker_lite` through the Rust↔WASM JSON bridge. `wasm-api` must register the game (constants, `RegisteredGame::PokerLite`, `MatchRecord::PokerLite`), expose viewer-safe view/action-tree/apply/bot-turn/effects/export-import/replay endpoints, and enforce action-tree authorization (a non-actor viewer gets an empty tree) with redaction tests — all without leaking hidden cards.

## Assumption Reassessment (2026-06-08)

1. The registration pattern is verified this session: `crates/wasm-api/src/lib.rs` has `enum RegisteredGame { …, SecretDraft }` (~L236) and `enum MatchRecord { …, SecretDraft { game_id: String, seed: u64, state: SecretDraftState, effects: EffectLog<SecretDraftEffect>, commands: Vec<AppliedCommand> } }` (~L137/187). The proposed `PokerLite` variant takes the identical shape with `PokerLiteState`/`PokerLiteEffect`. `list_games()` (~L303–326) emits per-game JSON with `viewer_modes`/`hidden_information`/`tags`.
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §4 wasm-api bullet, §E "WASM/browser specifics") fixes the additions: consts `GAME_POKER_LITE`, display name **Crest Ledger**, `VARIANT_POKER_LITE_STANDARD`, trace rules version `poker-lite-rules-v1`; list-games tags `hidden_info`/`viewer_filtered`/`public_replay_export`/`public_accounting`/`bounded_pledge`; `get_action_tree_for_viewer` returning the actor's tree only for the actor's viewer (observer/opponent get an empty tree); bot-turn (L0/L2), export/import with public-timeline redaction, replay step/reset, redaction tests. `wasm-exported.trace.json` (deferred from GAT10POKLITBET-009) lands here.
3. Cross-artifact boundary + schema-extension under audit: `RegisteredGame` and `MatchRecord` are extended **additively** with a new variant; their consumers are the exhaustive `match` sites within `wasm-api/src/lib.rs` itself — each must gain a `PokerLite` arm (no external consumer of these private enums). `crates/wasm-api/Cargo.toml` gains a `poker_lite` dep. **`scripts/check-catalog-docs.mjs` keys off the `GAME_*` catalog const**, so adding `GAME_POKER_LITE` here starts an expected red `check-catalog-docs` CI window that GAT10POKLITBET-016 closes by reconciling the README catalog surfaces.
4. FOUNDATIONS §2 (behavior authority — Rust supplies views/trees/effects/bot decisions; TS presents) and §11 (viewer-safe payloads; no leak) motivate this ticket. Restated before trusting the spec narrative.
5. No-leak firewall surface under audit (§11/§12): every WASM-exported payload (view, action tree, effects, export) must already be safe for the receiving viewer — `get_action_tree_for_viewer` returns an empty tree to a non-actor (matching the existing hidden-info pattern), and redaction unit tests must assert no hidden card id/rank reaches an observer/opponent payload or the public export. The `wasm-exported.trace.json` proves the exported surface is leak-free.
6. Schema-extension shape: `RegisteredGame`/`MatchRecord` extension is additive-only (new variant); no existing variant changes, so no consumer outside the new arms breaks. The list-games JSON gains an additive entry.

## Architecture Check

1. Mirroring the `SecretDraft` variant shape and the viewer-scoped action-tree authorization reuses the proven hidden-info bridge pattern exactly — no new export semantics, so no ADR trigger. An empty-tree-for-non-actor is the established no-leak default.
2. No backwards-compatibility aliasing/shims — additive variant + endpoints.
3. `engine-core` untouched (§3); `wasm-api` is the bridge, not a behavior owner (§2); no `game-stdlib` promotion (§4).

## Verification Layers

1. Registration + serialization (game resolves; view/effect/match-record serialize) -> `wasm-api` unit tests + `npm --prefix apps/web run smoke:wasm` (after the renderer wiring, exercised in GAT10POKLITBET-015/016; the Rust-side serializer tests run here).
2. Viewer-scoped action-tree authorization (non-actor gets empty tree) -> redaction unit test in `wasm-api`.
3. No-leak over WASM payloads (observer/opponent/export carry no hidden card) -> redaction unit tests + `wasm-exported.trace.json` assertion.
4. Additive schema-extension (new variant; all match sites exhaustive) -> `cargo build -p wasm-api` (non-exhaustive match would fail to compile) + grep-proof of the new arms.

## What to Change

### 1. `crates/wasm-api/src/lib.rs`

Add `GAME_POKER_LITE` / display-name / `VARIANT_POKER_LITE_STANDARD` / rules-version consts; `RegisteredGame::PokerLite`; `MatchRecord::PokerLite { game_id, seed, state: PokerLiteState, effects: EffectLog<PokerLiteEffect>, commands: Vec<AppliedCommand> }`; list-games entry with the §E tags; view/action-tree/apply/bot-turn/effects/export-import/replay-step-reset endpoints; `get_action_tree_for_viewer` authorization (empty tree for non-actor); redaction tests.

### 2. `crates/wasm-api/Cargo.toml` + `games/poker_lite/tests/golden_traces/wasm-exported.trace.json`

Add the `poker_lite` dep; author the `wasm-exported` golden trace proving the exported surface is leak-free and deterministic.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `crates/wasm-api/Cargo.toml` (modify)
- `games/poker_lite/tests/golden_traces/wasm-exported.trace.json` (new)

## Out of Scope

- The TS renderer, client types, dispatch (GAT10POKLITBET-015).
- The web smoke, package.json chain, and catalog README reconciliation that closes the `check-catalog-docs` red window (GAT10POKLITBET-016).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p wasm-api` and `cargo test -p wasm-api` pass (registration, serializers, redaction tests).
2. Viewer-scoped action-tree test: a non-actor viewer receives an empty action tree.
3. Redaction tests: no hidden card id/rank in any observer/opponent payload or public export; `wasm-exported.trace.json` replays leak-free.

### Invariants

1. Every WASM payload is viewer-safe at the boundary; legality/views/bot decisions come from Rust (§2/§11).
2. `RegisteredGame`/`MatchRecord` extension is additive; all match sites stay exhaustive (compile-enforced).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` (inline `#[cfg(test)]`) — registration, serialization, viewer-scoped tree, redaction.
2. `games/poker_lite/tests/golden_traces/wasm-exported.trace.json` — deterministic leak-free exported trace.

### Commands

1. `cargo test -p wasm-api`
2. `cargo build -p wasm-api --target wasm32-unknown-unknown` (requires `rustup target add wasm32-unknown-unknown`)
3. Note: `check-catalog-docs` is expected red from this ticket until GAT10POKLITBET-016 reconciles the README catalog surfaces — an expected mid-gate state, not a regression.

## Outcome

Completed on 2026-06-09.

- Added the additive `poker_lite` dependency, game constants, registered-game
  entry, match record variant, list-games metadata, view/action-tree/apply
  action/bot/effects/export/import/replay bridge arms, and support helpers.
- Serialized `poker_lite` Rust view/effect payloads through the WASM boundary
  with viewer filtering at the boundary and empty action trees for non-actor
  viewers.
- Added redaction tests for observer/opponent payloads, viewer-scoped action
  trees, Level 2 bot turn output, public export/import, and replay reset.
- Added `games/poker_lite/tests/golden_traces/wasm-exported.trace.json` as the
  deterministic public observer export fixture.

Verification:

- `cargo fmt --all --check`
- `cargo test -p wasm-api`
- `cargo build -p wasm-api`
- `cargo build -p wasm-api --target wasm32-unknown-unknown`
