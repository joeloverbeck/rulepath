# GAT101PLATRI-016: WASM registration and viewer-scoped browser bridge

**Status**: COMPLETE
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — modifies `crates/wasm-api/src/lib.rs` (new `RegisteredGame::PlainTricks`, `MatchRecord::PlainTricks`, `GAME_PLAIN_TRICKS` const + display name, list-games entry, serializers, bot-turn, export/import, replay branches, redaction tests). No `engine-core`/`game-stdlib` change.
**Deps**: GAT101PLATRI-013

## Problem

The browser needs a Rust↔WASM bridge for `plain_tricks`: game registration with hidden-information tags, two-seat setup, view/action/effect serializers, actor-only action-tree authorization (non-actor viewers get an empty tree), bot turns, viewer-scoped export/import, and replay stepping — all viewer-safe with redaction tests.

## Assumption Reassessment (2026-06-09)

1. `crates/wasm-api/src/lib.rs` defines `RegisteredGame` and `MatchRecord` enums (nine variants each, `RaceToN`…`PokerLite`) and a `GAME_*`/`GAME_*_DISPLAY_NAME` const set; mirror the `PokerLite` wiring. The `plain_tricks` crate surface (views, actions, effects, bots, replay) exists from GAT101PLATRI-005…013.
2. Spec §4 ("WASM, tools, CI, and web shell") and appendix D fix the additions: constants `GAME_PLAIN_TRICKS` / display **Plain Tricks** / `VARIANT_PLAIN_TRICKS_STANDARD` / rules version `plain-tricks-rules-v1`; `RegisteredGame::PlainTricks`; `MatchRecord::PlainTricks`; list-games entry with tags `hidden_info`, `viewer_filtered`, `public_replay_export`, `trick_taking`; actor-only `get_action_tree_for_viewer`; bot-turn branch (L0/L2); export/import branch with public-timeline redaction.
3. Shared boundary under audit: the WASM bridge surface (the two enums + list-games catalog const + serializer/authorization/export branches). The `GAME_PLAIN_TRICKS` const is the source of truth for `scripts/check-catalog-docs.mjs`.
4. FOUNDATIONS §2 (Rust owns view/action/effect/bot/replay; TS presentation-only) and §11 (viewer-safe payloads; no hidden-info leak) are under audit.
5. Enforcement surface: §11 no-leak firewall at the browser boundary. The action-tree authorization MUST return an empty tree to non-actor viewers; export/import MUST redact per ADR 0004; redaction unit tests must prove no unplayed card / tail / seed reaches a browser payload. No determinism change beyond exposing existing Rust behavior.
6. Extends the WASM bridge contract additively (new enum variants + list-games entry + branches); consumers are `scripts/check-catalog-docs.mjs` (catalog const) and the web shell (GAT101PLATRI-017). **Note:** adding `GAME_PLAIN_TRICKS` makes `check-catalog-docs` go red until GAT101PLATRI-018 reconciles the README catalog surfaces — an expected mid-gate window.

## Architecture Check

1. Registering `plain_tricks` through the existing enum/serializer/authorization pattern keeps the bridge uniform and keeps all behavior in Rust; the actor-only tree authorization is the browser-boundary no-leak guard.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched and noun-free; bridge payloads delegate to the `plain_tricks` crate. No `game-stdlib` change.

## Verification Layers

1. Registration present (enums, const, list-games tags) -> codebase grep-proof on `crates/wasm-api/src/lib.rs`.
2. Non-actor viewer gets an empty action tree -> WASM redaction/authorization unit test.
3. Export/import redaction (no unplayed card / tail / seed) -> no-leak redaction unit test + `wasm-exported` golden trace refresh.
4. Bot-turn / replay-step branches work end-to-end -> WASM unit tests; `npm --prefix apps/web run smoke:wasm`.

## What to Change

### 1. `crates/wasm-api/src/lib.rs`

Add `GAME_PLAIN_TRICKS` + display name + `VARIANT_PLAIN_TRICKS_STANDARD` + rules-version constants; `RegisteredGame::PlainTricks`; `MatchRecord::PlainTricks`; list-games entry with the four hidden-information tags; two-seat new-match setup; view/action/effect JSON serializers; actor-only `get_action_tree_for_viewer`; run-bot-turn branch (L0/L2); export/import branch with public-timeline redaction; replay-step/reset; redaction tests.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)

## Out of Scope

- The TS renderer / `client.ts` view types / `main.tsx` wiring (GAT101PLATRI-017).
- e2e smoke + catalog README reconciliation (GAT101PLATRI-018) — which closes the `check-catalog-docs` red window this ticket opens.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` (or workspace) redaction/authorization tests pass: non-actor empty tree; export omits seed/tail/unplayed cards.
2. `npm --prefix apps/web run smoke:wasm` loads and exercises `plain_tricks`.
3. `grep -n "GAME_PLAIN_TRICKS\|RegisteredGame::PlainTricks\|MatchRecord::PlainTricks" crates/wasm-api/src/lib.rs` resolves.

### Invariants

1. Non-actor viewers never receive the action tree or any hidden card (FOUNDATIONS §11 no-leak firewall).
2. All behavior (legality, scoring, bot, replay, redaction) stays in Rust; the bridge only serializes (FOUNDATIONS §2).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` redaction/authorization unit tests for `plain_tricks`.
2. Refresh `games/plain_tricks/tests/golden_traces/wasm-exported.trace.json` with the real WASM export.

### Commands

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run smoke:wasm`
3. The WASM unit tests + wasm smoke are the correct boundary; full browser rendering is GAT101PLATRI-017 and e2e is GAT101PLATRI-018.

## Outcome

Completed 2026-06-09. Registered `plain_tricks` in the WASM catalog, match store, live view/action/effect/bot branches, public replay export/import, and replay stepping. Added viewer-scoped redaction and actor-authorization tests proving non-actors receive an empty action tree and public exports omit seed evidence plus unplayed hidden cards. Refreshed the `wasm-exported` golden trace and extended `npm --prefix apps/web run smoke:wasm` to exercise the viewer bridge and public replay path for Plain Tricks.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p wasm-api`
3. `npm --prefix apps/web run smoke:wasm`
4. `grep -n "GAME_PLAIN_TRICKS\|RegisteredGame::PlainTricks\|MatchRecord::PlainTricks" crates/wasm-api/src/lib.rs`
