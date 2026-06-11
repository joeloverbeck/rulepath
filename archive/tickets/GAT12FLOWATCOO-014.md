# GAT12FLOWATCOO-014: WASM/API registration

**Status**: ACCEPTED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (catalog, setup, action, bot, effect, view, replay/export/import, no-leak paths); `apps/web/scripts/smoke-load-wasm.mjs` (modify — add `flood_watch` catalog assertion)
**Deps**: GAT12FLOWATCOO-009, GAT12FLOWATCOO-010

## Problem

`flood_watch` must be reachable from the browser through the Rust↔WASM bridge: catalog entry, setup, action tree, action application (with the environment batch on a turn-ending action), bot turns, effects, viewer-safe view, and replay export/import defaulting to the viewer-scoped observation timeline with the undrawn deck redacted. The `smoke:wasm` harness, which hardcodes per-game catalog assertions, must gain a `flood_watch` check.

## Assumption Reassessment (2026-06-11)

1. `crates/wasm-api/src/lib.rs` registers `masked_claims` across ~49 sites (verified): import block, `GAME_MASKED_CLAIMS`/display-name consts, catalog (`list_games`), setup (`new_match`), action (`get_action_tree`/`apply_action`), bot (`run_bot_turn` with `MaskedClaimsLevel1Bot`), effects, view (`get_view` + view-JSON helpers), replay/export/import, and the `RegisteredGame::MaskedClaims` enum variant. `flood_watch` mirrors this. `apps/web/scripts/smoke-load-wasm.mjs` carries hardcoded `catalog.some((game) => game.game_id === "<g>")` assertions (verified) → it is a `(modify)` target, not just a reference path.
2. The spec (§Deliverables "WASM/API", §Implementation reference "WASM/browser wiring", Work-breakdown item 12) fixes: catalog entry with `game_id: flood_watch`, display name `Flood Watch`, cooperative + hidden-information flags, both scenario variants, docs links; `get_view(match_id, viewer_seat)` returns the single public projection (undrawn deck in no projection); `apply_action` returns safe effects + view with the full environment batch on a turn-ending action; `run_bot_turn` routes budgeted actions through the legal tree with viewer-safe decision JSON; `export_replay` defaults to the viewer-scoped observation export with the undrawn deck redacted.
3. Cross-artifact boundary under audit: the WASM bridge consumes the Rust contracts from GAT12FLOWATCOO-005/008/009/010 (legal tree, projection, export, bots). Adding `GAME_FLOOD_WATCH` to the catalog const is the source of truth `scripts/check-catalog-docs.mjs` keys off — **this ticket opens the expected `check-catalog-docs` red window** that GAT12FLOWATCOO-018 closes by reconciling the README catalog surfaces. The `RegisteredGame` enum gains a `FloodWatch` arm (a new dispatch value — every match site on that enum needs the arm).
4. FOUNDATIONS §2 (Rust owns view projection, replay, bot decisions; TS presentation-only) and §11 (browser payloads already safe for the viewer; hidden info does not leak through payloads/replay exports) motivate this ticket: the bridge must surface only the viewer-safe projection and the redacted export, never the undrawn deck order.
5. Enforcement surface: the WASM boundary is the §11 no-leak firewall at the browser edge — `get_view`, `apply_action` effects, `run_bot_turn` decision JSON, and `export_replay` must all be viewer-safe by construction (they consume the already-redacted Rust surfaces). The `RegisteredGame` enum exhaustiveness is a §3-adjacent dispatch-completeness concern: the new arm must be handled at every match.

## Architecture Check

1. Reusing the `masked_claims` registration shape (consume the game's own Rust surfaces, add a `RegisteredGame` arm) keeps the bridge generic — wasm-api dispatches to game-local logic and never re-implements legality, projection, or redaction.
2. No backwards-compatibility aliasing/shims; additive registration + additive enum arm.
3. `engine-core` untouched; wasm-api remains the JSON bridge with no mechanic noun added to the kernel — all flood/deck/role meaning stays in `games/flood_watch`.

## Verification Layers

1. Catalog + dispatch registration -> schema validation: `list_games` includes `flood_watch` (id, display name, cooperative + hidden-info flags, both variants); the `RegisteredGame::FloodWatch` arm is handled at every match site.
2. Viewer-safe view + redacted export -> no-leak visibility test: `get_view` returns the single public projection (no undrawn deck); `export_replay` redacts the undrawn deck.
3. Environment batch + bot turn over the bridge -> simulation/ABI smoke: a turn-ending `apply_action` returns the environment effect batch; `run_bot_turn` returns viewer-safe decision JSON.
4. Smoke harness assertion -> `npm --prefix apps/web run smoke:wasm` includes the `flood_watch` catalog assertion.

## What to Change

### 1. `crates/wasm-api/src/lib.rs`

Add the `flood_watch` import block, `GAME_FLOOD_WATCH` + display-name consts, catalog entry (flags + variants + docs links), `RegisteredGame::FloodWatch` arm at every match, setup/action/bot/effect/view/replay/export/import wiring, and the view-JSON + viewer-scoped-export helpers mirroring `masked_claims`.

### 2. `apps/web/scripts/smoke-load-wasm.mjs`

Add a `flood_watch` catalog assertion (`catalog.some((game) => game.game_id === "flood_watch")`, with variant check) mirroring the existing per-game assertions.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify — full registration + enum arm)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify — catalog assertion)

## Out of Scope

- The React board, effect feedback, outcome templates, client-type mirrors, and `smoke-ui`/`smoke-effect-feedback` (GAT12FLOWATCOO-017).
- README catalog reconciliation and the `check-catalog-docs` red-window closeout (GAT12FLOWATCOO-018).
- `seed-reducer`/`trace-viewer` (they do not enumerate game IDs — verified; no registration needed).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:wasm` passes with the `flood_watch` catalog assertion present.
2. The `RegisteredGame::FloodWatch` arm is handled at every match site (compiles; no non-exhaustive match).
3. A bridge-level check confirms `get_view` exposes no undrawn deck and `export_replay` redacts it.

### Invariants

1. Browser payloads (`get_view`, effects, bot decision JSON, replay export) are viewer-safe by construction; the undrawn deck order never crosses the bridge.
2. wasm-api adds no mechanic noun to `engine-core`; it dispatches to game-local logic.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-load-wasm.mjs` — `flood_watch` catalog assertion (modify).
2. WASM-side no-leak assertion reusing the game's redacted export (the Rust no-leak suite is GAT12FLOWATCOO-011; this verifies the bridge surface).

### Commands

1. `cargo build -p wasm-api`
2. `npm --prefix apps/web run smoke:wasm`
3. `smoke:ui`/`smoke:e2e` are exercised once the renderer and e2e land (GAT12FLOWATCOO-017/018); `smoke:wasm` is the correct boundary for the ABI registration diff.

## Outcome

Accepted on 2026-06-11. Registered `flood_watch` in the WASM/API bridge with
catalog metadata, setup, action tree, action application, bot turn, effects,
viewer-safe view JSON, viewer-scoped public replay export/import, replay-step
timeline import, and no-leak bridge coverage. Added the `flood_watch` catalog
assertion to the web WASM smoke harness. `engine-core` remains untouched; the
bridge dispatches to the game-local Flood Watch Rust surfaces.

Verification:

1. `cargo test -p wasm-api`
2. `cargo build -p wasm-api`
3. `cargo fmt --all --check`
4. `npm --prefix apps/web run smoke:wasm`
5. `cargo clippy -p wasm-api --all-targets -- -D warnings`
