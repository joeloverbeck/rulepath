# GAT11MASCLABLU-014: WASM/API registration

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modifies `crates/wasm-api/src/lib.rs` (catalog, setup, action, bot, effect, view, replay/export/import, no-leak redaction paths)
**Deps**: GAT11MASCLABLU-008, GAT11MASCLABLU-009

## Problem

The browser needs `masked_claims` exposed through the Rust↔browser JSON bridge: a catalog entry, viewer-scoped views, per-phase action trees, action application, bot turns, and viewer-scoped replay export/import — all honoring the no-leak redaction the native crate already enforces.

## Assumption Reassessment (2026-06-10)

1. The crate's views/export (GAT11MASCLABLU-008) and bots (GAT11MASCLABLU-009) provide the surfaces to bridge. The registration model is the `plain_tricks` block in `crates/wasm-api/src/lib.rs`: a `use plain_tricks::{...}` import block (confirmed lines 44–53), `GAME_*` and `GAME_*_DISPLAY_NAME` consts (confirmed lines 97–112), and the bridge fns `get_view(match_id, viewer_seat)`, `get_action_tree(match_id, actor_seat)`, `apply_action`, `run_bot_turn(match_id, actor_seat, bot_seed)`, `export_replay`, `import_replay` (all confirmed present with those signatures).
2. Spec Deliverables WASM/API row: catalog entry `game_id: masked_claims`, display `Masked Claims`, hidden-information flag, viewer modes, variants, docs links; `get_view` honors viewer scope; `export_replay` defaults to the viewer-scoped observation timeline under ADR 0004 with claim action paths redacted to declared grades.
3. Cross-artifact boundary under audit: the `crates/wasm-api/src/lib.rs` `GAME_*` catalog const is the source of truth for `scripts/check-catalog-docs.mjs` (run in `gate-1-game-smoke.yml`). Adding `GAME_MASKED_CLAIMS` here OPENS an expected `check-catalog-docs` red window that stays red until the catalog READMEs are reconciled in GAT11MASCLABLU-019.
4. FOUNDATIONS §2 (Rust/WASM owns behavior; the bridge carries JSON only, TypeScript decides nothing) and §11 (browser payloads are already viewer-safe) are the principles under audit.
5. No-leak firewall enforcement surface: the WASM view/export/bot-explanation JSON paths. Confirm `get_view` returns the viewer-scoped projection, `export_replay` defaults to viewer-scoped with claim-path redaction, and `run_bot_turn` JSON is viewer-safe — no unrevealed tile ID crosses the bridge.

## Architecture Check

1. Registering through the existing generic bridge fns (no new public surface, no game noun in the bridge contract) keeps `wasm-api` a thin JSON adapter over Rust-owned behavior.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; no `game-stdlib` change; the bridge adds only a dispatch arm.

## Verification Layers

1. Catalog/setup/action/view/bot/export wired -> `cargo build -p wasm-api` + `npm --prefix apps/web run smoke:wasm` (run in the web pipeline).
2. Viewer-scoped view/export no-leak across the bridge -> the native no-leak suite (GAT11MASCLABLU-010) + WASM smoke.
3. `check-catalog-docs` red window opened here -> flagged as expected mid-gate state, resolved in GAT11MASCLABLU-019.

## What to Change

### 1. `crates/wasm-api/src/lib.rs`

Add the `use masked_claims::{...}` import block; `GAME_MASKED_CLAIMS` + `GAME_MASKED_CLAIMS_DISPLAY_NAME` consts and the catalog entry (hidden-info flag, viewer modes, variants, docs links); the setup/action-tree/apply/bot/effect/view/replay/export/import dispatch arms; and the no-leak redaction paths (claim-path → declared grade on export).

### 2. `apps/web/scripts/smoke-load-wasm.mjs`

The `smoke:wasm` harness carries **hardcoded** per-game catalog assertions (confirmed `assert(catalog.some(... === "plain_tricks" ...))`), not auto-discovery. Add a `masked_claims` (and `masked_claims_standard` variant) assertion so `smoke:wasm` exercises the new catalog entry.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify)

## Out of Scope

- The React board and shell reducer/types (GAT11MASCLABLU-017).
- Tool and CI registration (GAT11MASCLABLU-015).
- Catalog README reconciliation that closes the `check-catalog-docs` window (GAT11MASCLABLU-019).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p wasm-api` succeeds with the new dispatch arms.
2. `npm --prefix apps/web run smoke:wasm` exercises `masked_claims` setup/view/action/bot/export (after `rustup target add wasm32-unknown-unknown`).
3. No unrevealed tile ID appears in any bridge payload (`get_view`, `export_replay`, `run_bot_turn`).

### Invariants

1. The bridge carries viewer-safe JSON only; TypeScript decides no legality (FOUNDATIONS §2).
2. `export_replay` defaults to the viewer-scoped, claim-redacted timeline (ADR 0004; FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` `#[cfg(test)]` — `masked_claims` catalog/view/export bridge coverage mirroring the existing per-game wasm-api tests.

### Commands

1. `cargo build -p wasm-api`
2. `cargo test -p wasm-api`
3. `npm --prefix apps/web run smoke:wasm` is the full ABI boundary, exercised once the web pipeline (GAT11MASCLABLU-017/019) consumes the bridge.
