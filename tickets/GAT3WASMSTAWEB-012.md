# GAT3WASMSTAWEB-012: WASM/API smoke upgrade (version/features, list_games, replay)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — extends the node WASM smoke scripts (`apps/web/scripts`); no Rust/crate change.
**Deps**: 002, 003

## Problem

The existing low-level WASM smoke must cover the new Gate 3 operations so the bridge
basics are proven independently of the rendered shell (spec §19.2). Today
`apps/web/scripts/smoke-load-wasm.mjs` exercises only the original op set; the new
`feature_report`, `list_games`, and replay export/import/step ops (from
GAT3WASMSTAWEB-002/-003) have no smoke coverage. §19.2 enumerates the required
WASM/API smoke surface.

## Assumption Reassessment (2026-06-06)

1. `apps/web/scripts/smoke-load-wasm.mjs` instantiates the WASM artifact directly
   (Node-style) and `smoke-ui.mjs` drives the built app via `render_game_to_text`;
   `apps/web/package.json` wires `smoke:wasm` (`build:wasm && node
   scripts/smoke-load-wasm.mjs`) and `smoke:ui`. Neither covers `feature_report`/
   `list_games`/replay — those ops are added by GAT3WASMSTAWEB-002 (`rulepath_feature_report`,
   `rulepath_list_games`) and GAT3WASMSTAWEB-003 (replay export/import/step externs).
2. Spec §19.2 requires WASM/API smoke to verify: artifact loads/instantiates;
   required exports/ops exist; version/feature report works; new match; public view;
   action tree; legal action apply; bot turn; effect fetching; stale-action
   diagnostic stays safe; replay export/import operations work if exposed at this
   layer.
3. Cross-artifact boundary under audit: the node smoke calls the raw `rulepath_*`
   exports directly (it predates and is independent of the TS client module), so it
   validates the ABI surface — distinct from the browser shell smoke
   (GAT3WASMSTAWEB-013). The new export names must match the `#[no_mangle]` symbols
   added in -002/-003.

## Architecture Check

1. Extending the existing direct-ABI node smoke (rather than only relying on the
   browser E2E harness) keeps a fast, shell-independent proof of the bridge — the
   right layer to catch a missing/renamed export or a broken replay round trip
   before UI debugging.
2. No backwards-compatibility shims: the script is extended in place; no parallel
   smoke harness is introduced.
3. `engine-core` untouched; smoke scripts only; `game-stdlib` untouched.

## Verification Layers

1. New exports exist and respond → codebase grep-proof + simulation: the smoke
   asserts `rulepath_feature_report`/`rulepath_list_games`/replay exports are present
   and return status-0 typed JSON.
2. Replay round-trips at the ABI layer → deterministic replay check: the smoke
   exports a run, imports it, steps, and asserts consistent projected state.
3. Stale-action diagnostic stays safe → no-leak/fail-closed test: the smoke asserts
   the stale path returns a safe typed diagnostic without unauthorized state.
4. Cross-artifact: ABI-surface layer is verified here; rendered-shell behavior is
   GAT3WASMSTAWEB-013 — layers kept distinct.

## What to Change

### 1. `apps/web/scripts/smoke-load-wasm.mjs`

Extend to call `feature_report`, `list_games`, and the replay export/import/step
exports, asserting presence + status-0 typed responses and a replay round trip;
keep the existing op assertions (new match, view, action tree, apply, bot turn,
effects, stale diagnostic).

### 2. `apps/web/scripts/smoke-ui.mjs` (if needed)

Add assertions for any new shell-exposed op surfaced through `render_game_to_text`
that the WASM smoke cannot reach.

## Files to Touch

- `apps/web/scripts/smoke-load-wasm.mjs` (modify) — cover feature/list_games/replay ops + round trip
- `apps/web/scripts/smoke-ui.mjs` (modify) — any shell-only op assertions

## Out of Scope

- Rendered-browser E2E (Puppeteer) — GAT3WASMSTAWEB-013.
- Adding/altering Rust ops — GAT3WASMSTAWEB-002/-003.
- Accessibility/no-leak review — GAT3WASMSTAWEB-014.

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run smoke:wasm` — passes, covering version/features, list_games, new match, view, action tree, apply, bot turn, effects, stale diagnostic, and replay export/import/step.
2. `cd apps/web && npm run build` — typecheck + build remain green.
3. `grep -nE "feature_report|list_games|export_replay|import_replay|replay_step" apps/web/scripts/smoke-load-wasm.mjs` — the new ops are exercised.

### Invariants

1. The WASM/API smoke proves every §19.2 bridge operation at the raw-ABI layer, independent of the rendered shell.
2. The stale-action diagnostic remains viewer-safe under smoke.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-load-wasm.mjs` (modify) — add feature/list_games/replay coverage; rationale: catch missing/renamed exports and replay round-trip breaks at the fast ABI layer.

### Commands

1. `cd apps/web && npm run smoke:wasm`
2. `cd apps/web && npm run build`
3. The direct-ABI smoke is the correct boundary for bridge coverage; rendered-shell flows are verified separately in GAT3WASMSTAWEB-013.
