# GAT17VOWTIDOHHEL-017: WASM registration, operation groups, bridge no-leak, player rules

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `crates/wasm-api/src/games/vow.rs`, `games/vow_tide/docs/HOW-TO-PLAY.md`, `apps/web/public/rules/vow_tide.md`; modifies `crates/wasm-api/src/{lib,constants,games,catalog}.rs`, `apps/web/scripts/smoke-load-wasm.mjs`, `scripts/check-player-rules.mjs`, `apps/web/public/rules/manifest.json`
**Deps**: 009, 010, 011, 012

## Problem

Expose Vow Tide through the WASM bridge: constants, catalog entry (3–7 seat metadata, hidden-info/trick-taking/bidding/multi-seat tags, viewer modes), the `vow.rs` module with setup/action/view/effect/replay/bot/outcome operation groups, and the exhaustive pairwise no-leak dispatch. Author `HOW-TO-PLAY.md` and the generated public rules, and register the game in the player-rules checker — all keyed off the new catalog const.

## Assumption Reassessment (2026-06-21)

1. `crates/wasm-api/src/games.rs` declares per-game modules (`plain`, `briar`, `river`, …); `constants.rs` declares `GAME_*`/`*_DISPLAY_NAME`/`*_TRACE_RULES_VERSION`/`VARIANT_*_STANDARD` (briar at `:40`/`:81`/`:99`); `catalog.rs` `list_games()` matches `RegisteredGame` with seat metadata + tags (briar at `:258`); `lib.rs` dispatches on `game_id` (`:338`). All are `(modify)` targets; `games/vow.rs` is `(new)`.
2. `apps/web/scripts/smoke-load-wasm.mjs` carries hardcoded per-game catalog assertions (confirmed at harvest) → `(modify)`. `scripts/check-player-rules.mjs` has a `HIDDEN_INFO_GAMES` set (`:27`, includes `briar_circuit`) → vow_tide must be added (hidden-info game). `apps/web/public/rules/<game>.md` is generated from `HOW-TO-PLAY.md` via `scripts/copy-player-rules.mjs`.
3. Cross-artifact boundary under audit: the bridge reuses the 010 viewer projections + 011 exports; no public API schema expansion is expected; the catalog const is the source of truth keying `check-player-rules`/`check-catalog-docs`/`check-outcome-explanations`.
4. FOUNDATIONS §11 no-leak firewall is the principle under audit: the bridge must add no get-all-state/debug escape hatch; the pairwise dispatch proves no private datum reaches any unauthorized WASM viewer.
5. §11 enforcement surface: HOW-TO-PLAY's hidden-information section is required for `HIDDEN_INFO_GAMES`; player-rules markdown is generated, not hand-edited; the bridge no-leak harness mirrors the native 010 matrix across the JSON boundary.

## Architecture Check

1. Co-landing the catalog const, the player-rules surface, and `HIDDEN_INFO_GAMES` registration in the WASM ticket avoids the multi-PR red window the validators (`check-player-rules`) would otherwise show.
2. No shims; additive module + enum arm + const.
3. `engine-core`/`game-stdlib` untouched; the bridge reuses Rust projections, decides no legality in JS.

## Verification Layers

1. Catalog entry + viewer modes + seat metadata correct → `npm --prefix apps/web run smoke:wasm`.
2. Bridge pairwise no-leak across N=3..7 → WASM bridge no-leak dispatch test (reuses 010 canaries).
3. Player-rules generated + hidden-info section present → `node scripts/check-player-rules.mjs`.
4. No get-all-state escape hatch → manual review of `vow.rs` (only authorized viewer ops).

## What to Change

### 1. Constants + catalog + module + dispatch

Add `GAME_VOW_TIDE`/display/trace-version/variant constants; the `RegisteredGame::VowTide` catalog entry (min 3/max 7/default 4/supported `{3..7}`, labels, viewer modes, tags hidden_info/viewer_filtered/public_replay_export/trick_taking/bidding/multi_seat); `pub(crate) mod vow;` + `games/vow.rs` with setup/action/view/effect/replay/bot/outcome op groups; the `lib.rs` dispatch arm; the bridge pairwise no-leak dispatch.

### 2. Player rules

Author `HOW-TO-PLAY.md` (goal/setup/bid/play/score/finish + hidden-information section); generate `apps/web/public/rules/vow_tide.md` via `scripts/copy-player-rules.mjs`; add the manifest.json row; add `vow_tide` to `HIDDEN_INFO_GAMES`; update `smoke-load-wasm.mjs` per-game assertion.

## Files to Touch

- `crates/wasm-api/src/games/vow.rs` (new)
- `crates/wasm-api/src/games.rs` (modify)
- `crates/wasm-api/src/constants.rs` (modify)
- `crates/wasm-api/src/catalog.rs` (modify)
- `crates/wasm-api/src/lib.rs` (modify)
- `games/vow_tide/docs/HOW-TO-PLAY.md` (new)
- `apps/web/public/rules/vow_tide.md` (new, generated)
- `apps/web/public/rules/manifest.json` (modify)
- `scripts/check-player-rules.mjs` (modify)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify)

## Out of Scope

- Web renderer + outcome TS surfaces (018); e2e + catalog README (019).
- Any public API schema expansion or debug escape hatch.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — bridge + pairwise no-leak dispatch green.
2. `npm --prefix apps/web run smoke:wasm` — catalog/viewer modes load.
3. `node scripts/check-player-rules.mjs` — generated rules + hidden-info section valid.

### Invariants

1. No private datum reaches any unauthorized WASM viewer for any N; no get-all-state hatch exists.
2. `apps/web/public/rules/vow_tide.md` is generated from `HOW-TO-PLAY.md`, not hand-edited.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/games/vow.rs` — bridge no-leak dispatch test (reuses 010 canaries).
2. `apps/web/public/rules/vow_tide.md` — generated player-rules artifact (parity-checked).

### Commands

1. `cargo test -p wasm-api`
2. `npm --prefix apps/web run smoke:wasm && node scripts/check-player-rules.mjs`
3. Narrower command rationale: smoke:wasm + the bridge test are the boundary; the catalog README surfaces co-land with the web smoke (019).

## Outcome

Completed: 2026-06-21

Implemented Vow Tide WASM registration across constants, catalog metadata, the game module registry, match storage, view/action/effect/bot/replay dispatch, and the API surface snapshot. Added `crates/wasm-api/src/games/vow.rs` for Vow Tide setup, seat/viewer mapping, Rust legal action dispatch, filtered views/effects, bot dispatch, viewer-scoped replay export/import, and a pairwise JSON projection no-leak test over supported seat counts 3 through 7.

Added the player-facing `games/vow_tide/docs/HOW-TO-PLAY.md`, generated `apps/web/public/rules/vow_tide.md` and the public rules manifest row with `scripts/copy-player-rules.mjs`, registered `vow_tide` as a hidden-information game in `scripts/check-player-rules.mjs`, and extended `apps/web/scripts/smoke-load-wasm.mjs` to smoke the Vow Tide catalog entry, seven-seat setup, observer/seat redaction, unauthorized tree redaction, Rust bid application, bot turn, effect log, and viewer-scoped replay import/reset path.

Deviations: generic command replay import/step reconstructs Vow Tide with the default four-seat setup because the existing generic replay parser has no seat-count field. The live match export path preserves the match seat count and exports the viewer-scoped public replay used by the browser smoke.

Verification:

- `cargo fmt --all --check`
- `cargo test -p wasm-api`
- `npm --prefix apps/web run smoke:wasm`
- `node scripts/check-player-rules.mjs`
