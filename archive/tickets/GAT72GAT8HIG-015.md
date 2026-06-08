# GAT72GAT8HIG-015: WASM viewer-aware hardening (get_view honors viewer; action-tree authorization)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (viewer-aware `get_view`; action-tree viewer authorization)
**Deps**: GAT72GAT8HIG-001

## Problem

The WASM `get_view` export currently ignores its viewer-seat argument and
projects a public-equivalent view for every game; `get_action_tree` takes an
actor seat with no viewer authorization. Before any hidden-information game is
browser-visible, the boundary must honor the viewer so a hidden-info game can
filter and the browser cannot request another seat's private data.

## Assumption Reassessment (2026-06-07)

1. Verified the current behavior (change rationale — no silent retcon):
   `crates/wasm-api/src/lib.rs:365` — `pub fn get_view(match_id, _viewer_seat)`
   ignores `_viewer_seat` and every arm passes `Viewer { seat_id: None }`
   (lines 373-393). `get_effects` already builds a per-seat viewer
   (`race_viewer_for_seat` etc., lines 788-907). `get_action_tree(match_id,
   actor_seat)` (line 399) has no viewer-authorization layer. This ticket
   intentionally changes `get_view` to honor the viewer — rationale: it is the
   prerequisite for hidden-info filtering and is currently a stub.
2. Verified against the spec: §4.2.5 requires `get_view` to honor `viewer_seat`
   for all games (perfect-info games may return equivalent projections; hidden-
   info games filter) and to add viewer/authorization context to the action-tree
   surface; §4.2.5 also requires existing perfect-info games to remain
   regression-free.
3. Cross-artifact boundary under audit: the WASM public/private view + action-
   tree contract (`docs/WASM-CLIENT-BOUNDARY.md`, `docs/ENGINE-GAME-DATA-
   BOUNDARY.md`). Blast radius: all five existing games' `get_view` arms
   (`three`/`column`/`directional`/`draughts`/`race`) plus their viewer helpers.
4. FOUNDATIONS principle under audit (§11 viewer-safe views + §2 behavior
   authority): the viewer is honored in Rust; the browser receives only
   viewer-authorized JSON.
5. Enforcement surface named: the §11 no-leak firewall at the WASM boundary.
   Confirm perfect-info games return projections equivalent to today's public
   view for every viewer (no behavior change), and the action-tree surface
   rejects/empties a request for a non-authorized seat. Regression proven by the
   existing per-game smoke tests.
6. Schema/contract extension classification: `get_view` becomes viewer-honoring
   (behavior change, additive at the type level — signature already carries the
   seat arg); the action-tree authorization is a new optional context. Consumers:
   the TS client (017) and every existing game smoke. Existing games' docs/tests
   are not rewritten broadly (spec §4.2.5).

## Architecture Check

1. Honoring the viewer in `get_view` (reusing the existing `*_viewer_for_seat`
   helpers proven in `get_effects`) is cleaner than a parallel hidden-info-only
   path — one viewer-aware boundary for all games.
2. No backwards-compatibility shims — the ignored-`_viewer_seat` stub is
   replaced, not aliased; perfect-info games keep equivalent output by
   construction, not by a compat layer.
3. `engine-core` untouched and noun-free; this is a `wasm-api` boundary change.

## Verification Layers

1. Viewer honored -> schema/serialization validation: `get_view` builds a per-seat `Viewer` for each game (parity with `get_effects`).
2. Perfect-info regression-free -> simulation/CLI run: existing per-game wasm/ui smokes still pass (`npm --prefix apps/web run smoke:ui`).
3. Action-tree authorization -> no-leak visibility test: a request for a non-authorized seat's action tree returns empty/error, not another seat's private tree.
4. Boundary contract -> FOUNDATIONS alignment check: browser receives only viewer-authorized JSON (§11).

## What to Change

### 1. `get_view` viewer honoring

Replace the `Viewer { seat_id: None }` stub in each game arm with a per-seat
viewer derived from `viewer_seat` (reuse the `*_viewer_for_seat` helper pattern);
perfect-info games return equivalent projections for all viewers.

### 2. Action-tree authorization

Add viewer/authorization context to the action-tree surface so the browser
cannot request another seat's private action tree.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)

## Out of Scope

- Registering `high_card_duel` itself (GAT72GAT8HIG-016 — depends on this).
- TS client/type changes (017).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — passes.
2. `npm --prefix apps/web run smoke:wasm` and `smoke:ui` — existing games regression-free.
3. `cargo build --workspace` — boundary change compiles.

### Invariants

1. `get_view` honors the viewer for all games; perfect-info games are output-equivalent to today (§11, no regression).
2. The action-tree surface cannot return a non-authorized seat's private tree.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api` tests — viewer-honoring `get_view` + action-tree authorization assertions for an existing game.

### Commands

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run smoke:ui`
3. The wasm-api unit tests + existing-game UI smoke are the correct boundary — they prove the hardening without the new game present yet.

## Outcome (2026-06-07)

Hardened the WASM viewer boundary:

1. `get_view` now constructs per-game viewers from the supplied viewer seat instead of always projecting observer/public views.
2. Added viewer-aware FFI exports for `rulepath_get_view_for_viewer` and `rulepath_get_action_tree_for_viewer`.
3. Added `get_action_tree_for_viewer`; unauthorized or observer requests for a seat's private action tree return an empty tree at the current freshness token.
4. Kept existing JS-facing `rulepath_get_view` and `rulepath_get_action_tree` behavior compatible for current web callers; `get_action_tree` now delegates as an authorized same-seat request.
5. Added wasm-api unit tests for viewer-honoring perfect-info view parity and action-tree authorization.

Deviations: the existing FFI exports remain for current web compatibility; the new viewer-aware exports provide the hardened surface for the TS client update in GAT72GAT8HIG-017.

Verification:

1. `cargo test -p wasm-api` — passed.
2. `npm --prefix apps/web run smoke:wasm` — passed.
3. `npm --prefix apps/web run smoke:ui` — passed.
4. `cargo build --workspace` — passed.
5. `cargo fmt --all --check` — passed.
