# GAT13FROCONASY-012: WASM/API registration

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (modify); `apps/web/scripts/smoke-load-wasm.mjs` (modify); `apps/web/scripts/smoke-ui.mjs` (modify)
**Deps**: GAT13FROCONASY-008, GAT13FROCONASY-009

## Problem

The browser shell reaches the game only through the WASM/API bridge. Frontier Control must be registered in the `crates/wasm-api/src/lib.rs` catalog and the setup/action/bot/effect/view/replay/export/import paths, with `get_view(match_id, viewer_seat)` returning output-equivalent projections for all viewers (perfect information) and a full-public-timeline replay export needing no redaction surface. The two WASM smoke harnesses carry hardcoded per-game catalog assertions, so they are `(modify)` targets of this registration.

## Assumption Reassessment (2026-06-11)

1. `crates/wasm-api/src/lib.rs` registers `flood_watch` via `use flood_watch::{…}`, `const GAME_FLOOD_WATCH`, variant consts (`VARIANT_FLOOD_WATCH_STANDARD/_DELUGE`), a JSON catalog entry, and setup/validate/effect-text/export/import dispatch (verified lines 35/121/171-172/405). Frontier mirrors this with `GAME_FRONTIER_CONTROL` + `VARIANT_FRONTIER_CONTROL_STANDARD/_HIGHLANDS`.
2. Spec §WASM/browser wiring defines the raw-ABI contract surface and the perfect-information `get_view`/export posture (no viewer redaction).
3. Cross-crate boundary under audit: `wasm-api` bridges the finished `frontier_control` crate (rules/visibility/replay/bots from GAT13FROCONASY-005–008); `apps/web/scripts/smoke-load-wasm.mjs` (catalog `game_id === "…"` asserts, verified L54-61) and `apps/web/scripts/smoke-ui.mjs` (catalog presence asserts, verified L150-160) hardcode per-game catalog assertions and must gain a `frontier_control` assertion — consumer-set blast radius of this registration, not mere reference paths.
4. FOUNDATIONS §2 behavior authority under audit: `wasm-api` is a JSON bridge that calls Rust; no legality, validation, or rule state moves into JS/TS.
5. §11 no-leak firewall: `get_view` is output-equivalent for `seat_0`/`seat_1`/observer and the replay export is the full public timeline — there is no redaction surface and no hidden-info leak path (perfect information; ADR 0004 not engaged).
6. Schema extension: the catalog entry, ABI dispatch arms, and variant consts extend the wasm-api surface additively (new game id, no existing consumer changed); `check-catalog-docs.mjs` keys off the new `GAME_*` const (reconciled in GAT13FROCONASY-016 — expected red-CI window until then).

## Architecture Check

1. Registering through the same raw-ABI contract that served symmetric games proves the bridge never assumed symmetry; a bespoke frontier ABI path would be unnecessary and divergent.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; `wasm-api` consumes the game crate's public surface only; no mechanic noun enters the kernel.

## Verification Layers

1. Catalog + ABI registration -> `npm --prefix apps/web run smoke:wasm` (catalog includes `frontier_control`; setup/view/action/bot/replay ABI paths resolve).
2. All-viewer output-equivalence (§11) -> `smoke:wasm` view check + the native no-leak visibility test (GAT13FROCONASY-009) — `get_view` identical across viewers.
3. Smoke-harness assertion update -> codebase grep-proof (`frontier_control` assertion present in `smoke-load-wasm.mjs` and `smoke-ui.mjs`).

## What to Change

### 1. wasm-api registration

Add the `use`, `GAME_FRONTIER_CONTROL` + variant consts, the JSON catalog entry (display name `Frontier Control`, perfect-information marker, both variants, docs links), and the setup/action/bot/effect/view/replay/export/import dispatch arms.

### 2. Smoke-harness assertions

Add the `frontier_control` catalog-presence assertion to `apps/web/scripts/smoke-load-wasm.mjs` and `apps/web/scripts/smoke-ui.mjs` (with the standard variant).

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify)
- `apps/web/scripts/smoke-ui.mjs` (modify)

## Out of Scope

- React board + effect feedback (GAT13FROCONASY-015).
- Player-rules asset + outcome-explanation templates (GAT13FROCONASY-014/015/016).
- Catalog README reconciliation (GAT13FROCONASY-016).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:wasm` passes (catalog includes `frontier_control`; ABI paths resolve).
2. `npm --prefix apps/web run smoke:ui` passes with the new catalog assertion.
3. `cargo build -p wasm-api` succeeds.

### Invariants

1. No legality/validation/rule state moves into JS/TS; the bridge calls Rust only (§2).
2. `get_view` is output-equivalent for all viewers; the export carries the full public timeline (§11).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-load-wasm.mjs` — `frontier_control` catalog assertion.
2. `apps/web/scripts/smoke-ui.mjs` — `frontier_control` catalog assertion.

### Commands

1. `npm --prefix apps/web run smoke:wasm`
2. `npm --prefix apps/web run smoke:ui`
3. The WASM/UI smoke harnesses are the correct boundary; the full browser E2E flow lands in GAT13FROCONASY-016.

## Outcome

Completed on 2026-06-11.

Implemented Frontier Control registration in the WASM/API bridge:

- Added the `frontier_control` dependency to `crates/wasm-api`.
- Added `GAME_FRONTIER_CONTROL`, standard/highlands variant constants, catalog metadata, and dispatch arms for setup, view projection, action tree, action application, Level 1 bot turns, effects, replay export/import, replay step/reset, and trace rules-version routing.
- Added Frontier Control public-view/effect JSON serializers that expose Rust-owned sites, factions, scores, supply status, scoring breakdowns, and terminal outcome rationale.
- Updated `apps/web/scripts/smoke-load-wasm.mjs` and `apps/web/scripts/smoke-ui.mjs` with Frontier Control catalog assertions; `smoke-load-wasm` also exercises setup, output-equivalent viewer projection, legal action, bot turn, and public replay export/import.

Scope correction: the current `npm --prefix apps/web run smoke:ui` pipeline runs
`scripts/copy-player-rules.mjs`, which scans `GAME_*` constants from
`crates/wasm-api/src/lib.rs`. Once `GAME_FRONTIER_CONTROL` exists, the UI smoke
cannot pass without `games/frontier_control/docs/HOW-TO-PLAY.md` and the
generated `apps/web/public/rules/frontier_control.md` asset. Those two
player-rules files and the generated manifest entry were added here to satisfy
the ticket's required UI smoke. GAT13FROCONASY-014 was updated to treat them as
already created and to own review/regeneration plus the remaining
`MECHANICS.md`, `UI.md`, and `AI.md` docs.

Verification:

- `cargo fmt --all --check` passed.
- `cargo build -p wasm-api` passed.
- `cargo clippy -p wasm-api --all-targets -- -D warnings` passed.
- `npm --prefix apps/web run smoke:wasm` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- `node scripts/check-player-rules.mjs` passed.
- `node scripts/check-doc-links.mjs` passed.
