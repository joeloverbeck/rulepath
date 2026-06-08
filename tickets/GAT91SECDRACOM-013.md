# GAT91SECDRACOM-013: secret_draft WASM registration (wasm-api arms + catalog)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (modify); `apps/web/scripts/smoke-load-wasm.mjs` (modify, if it enumerates games). No `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-007, GAT91SECDRACOM-008

## Problem

The browser shell reaches Rust only through `crates/wasm-api`. `secret_draft` must be registered across the catalog, setup, action-tree, apply, bot, effect, view, and replay/export/import paths, with viewer-scoped redaction enforced — `get_view(match_id, viewer_seat)` must honor viewer scope, and pre-reveal choices must be redacted for both observer and seat views (spec A6). This is the bridge that makes the game playable and no-leak in the browser.

## Assumption Reassessment (2026-06-08)

1. `crates/wasm-api/src/lib.rs` carries the registration surfaces (verified by grep): `GAME_TOKEN_BAZAAR` / `GAME_TOKEN_BAZAAR_DISPLAY_NAME` catalog consts, `new_match`, `get_view`, `get_action_tree`, `apply_action`, `run_bot_turn`, `export_replay`. `secret_draft` adds the analogous `GAME_SECRET_DRAFT` const pair (display name `Veiled Draft`) and per-path arms, mirroring the token_bazaar/high_card_duel wiring.
2. The Rust game surfaces (setup/actions/rules/effects/visibility/replay/bots from GAT91SECDRACOM-003–008) are the inputs the WASM arms call. Spec §"WASM/browser wiring" + §Deliverables (WASM/API row) define: catalog entry with hidden-information flag + viewer modes + variants + docs links; viewer-scoped `get_view`; action tree only for uncommitted seats; first-commit result with no item ID; second-commit reveal batch; viewer-safe `run_bot_turn` JSON; viewer-scoped `export_replay`.
3. Cross-artifact boundary under audit: the WASM JSON bridge contract and the no-leak redaction path. `check-catalog-docs.mjs` treats the `GAME_*` const pair as the source of truth for the catalog README surfaces — adding `GAME_SECRET_DRAFT` here is what makes those README updates due (handled in GAT91SECDRACOM-016).
4. §2 behavior authority + §11 no-leak are the motivating principles: restate before trusting spec — the WASM layer is a presentation bridge; it decides no legality and must not emit any pre-reveal committed item ID through view/action/apply/bot/export JSON. Observer view redacts pre-reveal choices; seat view also redacts the submitted choice (A6). `run_bot_turn` JSON is viewer-safe (no hidden candidates).
5. Determinism: `export_replay` defaults to the viewer-scoped observation timeline (ADR 0004); the `wasm-exported` golden trace (GAT91SECDRACOM-010) asserts this export shape and is re-exercised by the WASM smoke (`apps/web/scripts/smoke-load-wasm.mjs` covers raw ABI: version/features, catalog, match, action, view, bot, export — add a `secret_draft` path if the smoke enumerates games per game).

## Architecture Check

1. Reusing the game's own viewer-scoped projection/export (GAT91SECDRACOM-006/007) inside the WASM arms — rather than re-redacting at the boundary — keeps the firewall single-sourced and prevents browser-only leak drift.
2. No backwards-compatibility aliasing/shims — additive catalog + arm registration.
3. `engine-core` stays noun-free; wasm-api is the generic JSON bridge over game-local APIs, no mechanic noun enters the kernel. No `game-stdlib` helper.

## Verification Layers

1. Catalog registration -> grep-proof `GAME_SECRET_DRAFT` const pair + arm at each dispatch path in `crates/wasm-api/src/lib.rs`.
2. WASM no-leak -> ABI smoke (`smoke:wasm`) asserts get_view/apply/export carry no pre-reveal item ID; backed by the redaction in GAT91SECDRACOM-006/007.
3. Viewer-scope -> `get_view(match_id, viewer_seat)` returns observer-redacted and seat-redacted (A6) projections.
4. Export shape -> `wasm-exported` golden trace conformance (GAT91SECDRACOM-010) via the export path.

## What to Change

### 1. `crates/wasm-api/src/lib.rs`

Add `GAME_SECRET_DRAFT` + `GAME_SECRET_DRAFT_DISPLAY_NAME` catalog consts (hidden-information flag, viewer modes, variants, docs links), and register `secret_draft` in `new_match`, `get_view`, `get_action_tree`, `apply_action`, `run_bot_turn`, `export_replay` (and import/no-leak redaction paths), mirroring `token_bazaar`/`high_card_duel`.

### 2. `apps/web/scripts/smoke-load-wasm.mjs`

If the raw-ABI smoke enumerates games (per-game catalog/match/action/view/bot/export coverage), add `secret_draft`; otherwise no change (its generic catalog assertions already cover the new entry).

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify)

## Out of Scope

- TypeScript client bindings / catalog wiring (GAT91SECDRACOM-014) and the React board (GAT91SECDRACOM-015).
- The catalog README reconciliation that `check-catalog-docs` enforces (GAT91SECDRACOM-016) — adding `GAME_SECRET_DRAFT` here makes it due, but the README edits land with the web-smoke ticket.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:wasm` passes with `secret_draft` registered.
2. WASM `get_view`/`apply_action`/`export_replay` for `secret_draft` carry no pre-reveal committed item ID (observer + seat).
3. `cargo build -p wasm-api` (and the wasm target build) succeeds.

### Invariants

1. WASM bridge decides no legality and leaks no pre-reveal choice through any JSON path (§2/§11).
2. `export_replay` defaults to the viewer-scoped observation timeline (ADR 0004).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-load-wasm.mjs` — `secret_draft` ABI coverage (if game-enumerated).

### Commands

1. `npm --prefix apps/web run smoke:wasm`
2. `cargo build -p wasm-api`
3. `smoke:wasm` (raw ABI) is the correct boundary for WASM registration; rendered-browser no-leak is GAT91SECDRACOM-016.
