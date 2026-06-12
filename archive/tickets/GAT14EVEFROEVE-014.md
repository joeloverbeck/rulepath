# GAT14EVEFROEVE-014: WASM/API registration and player rules

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (modify) + `crates/wasm-api/Cargo.toml` (modify); `apps/web/scripts/{smoke-load-wasm,smoke-ui}.mjs` (modify); `scripts/check-player-rules.mjs` (modify); `games/event_frontier/docs/HOW-TO-PLAY.md` (new); generated `apps/web/public/rules/event_frontier.md`
**Deps**: GAT14EVEFROEVE-009, GAT14EVEFROEVE-010

## Problem

The browser shell reaches the game only through the WASM bridge. This ticket registers `event_frontier` across `crates/wasm-api/src/lib.rs` — catalog entry, setup, action, bot, effect, view, replay/export/import paths — so `get_view` projections are output-equivalent across seats/observers and never contain undrawn deck order, and replay export follows the ADR 0004 taxonomy as `flood_watch` proved. It also authors `HOW-TO-PLAY.md` (filling the hidden-information section for real this time — undrawn deck order), registers `event_frontier` in `HIDDEN_INFO_GAMES`, generates the player-rules markdown, and updates the hardcoded catalog assertions in the WASM/UI smoke harnesses.

## Assumption Reassessment (2026-06-12)

1. The Rust surfaces wired here exist: verified tickets 005–010 implement setup, action application, bots, effects, visibility (`get_view`), and replay/export; the wasm-api integration pattern is the `frontier_control` registration in `crates/wasm-api/src/lib.rs` (catalog const + `MatchRecord` variant + dispatch + export paths, ~40 sites).
2. The smoke-harness and player-rules surfaces are current: verified `apps/web/scripts/smoke-load-wasm.mjs` and `smoke-ui.mjs` carry **hardcoded** `catalog.some(game => game.game_id === "...")` assertions (modify targets here), `scripts/check-player-rules.mjs` defines `HIDDEN_INFO_GAMES` (line ~27, currently includes `flood_watch`), and `scripts/copy-player-rules.mjs` generates `apps/web/public/rules/<game>.md` from `HOW-TO-PLAY.md`.
3. Cross-crate boundary under audit: registering the catalog const opens the `check-catalog-docs` red window (resolved at ticket 018); `check-player-rules` requires the generated rules md + the `HIDDEN_INFO_GAMES` entry + the filled hidden-info section to go green, so those co-land here (avoiding a player-rules red window). `get_view` output-equivalence across seats/observer must hold through the WASM boundary.
4. FOUNDATIONS §2 (Rust behavior authority; TS presentation-only) and §11 (viewer-safe views; no hidden-info leak) motivate this ticket. Restated before trusting the spec: the WASM bridge transports Rust-projected views only; TypeScript decides no legality; no `get_view`/export payload contains undrawn deck order.
5. No-leak + determinism surface (§11): the WASM boundary is a leak vector (payloads/exports cross to JS). Confirm every WASM-exposed view/export is output-equivalent across viewers and excludes undrawn order, and that export/import reproduces the ADR 0004 taxonomy deterministically. No replay/hash semantics change — ADR 0004 reused.

## Architecture Check

1. Mirroring `frontier_control`'s wasm-api registration (single catalog const + `MatchRecord` variant + dispatch) is cleaner than a parallel bridge: the game joins the existing enum-dispatch with no new bridge contract.
2. No backwards-compatibility aliasing/shims — additive catalog/variant/dispatch.
3. `engine-core` stays noun-free; no `game-stdlib` promotion; TypeScript gains no legality logic.

## Verification Layers

1. WASM catalog + dispatch -> `npm --prefix apps/web run smoke:wasm` passes with the new catalog assertion; the game sets up/acts/views/replays through wasm-api.
2. View no-leak across the boundary (§11) -> the WASM `get_view` is output-equivalent across seats/observer and excludes undrawn order (re-exercised by the browser no-leak smoke, ticket 018).
3. Player rules -> `node scripts/check-player-rules.mjs` passes with `event_frontier` in `HIDDEN_INFO_GAMES`, the hidden-info section filled, and the generated `apps/web/public/rules/event_frontier.md` present.
4. UI catalog smoke -> `npm --prefix apps/web run smoke:ui` passes the updated catalog assertion.

## What to Change

### 1. wasm-api registration

In `crates/wasm-api/src/lib.rs`: add the `GAME_EVENT_FRONTIER` catalog const + display name + variants + trace rules-version, the `MatchRecord::EventFrontier` variant, and the setup/action/bot/effect/view/replay/export/import dispatch arms, mirroring `frontier_control`. Add the `event_frontier` dependency to `crates/wasm-api/Cargo.toml`.

### 2. Smoke-harness catalog assertions

Update `apps/web/scripts/smoke-load-wasm.mjs` and `smoke-ui.mjs` to assert the `event_frontier` catalog membership + `event_frontier_standard` variant.

### 3. Player rules + hidden-info

Author `games/event_frontier/docs/HOW-TO-PLAY.md` (from `templates/GAME-HOW-TO-PLAY.md`) with the hidden-information section filled (undrawn deck order). Add `event_frontier` to `HIDDEN_INFO_GAMES` in `scripts/check-player-rules.mjs`. Generate `apps/web/public/rules/event_frontier.md` via `scripts/copy-player-rules.mjs`.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `crates/wasm-api/Cargo.toml` (modify)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify)
- `apps/web/scripts/smoke-ui.mjs` (modify)
- `scripts/check-player-rules.mjs` (modify) — add `event_frontier` to `HIDDEN_INFO_GAMES`
- `games/event_frontier/docs/HOW-TO-PLAY.md` (new)
- `apps/web/public/rules/event_frontier.md` (new, generated)

## Out of Scope

- The React board / renderer and outcome-explanation templates (ticket 017).
- The browser E2E smoke + catalog README reconciliation (ticket 018) — resolves the `check-catalog-docs` red window this ticket opens.
- MECHANICS/UI/AI docs (ticket 016).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:wasm` and `smoke:ui` pass with the new catalog assertions.
2. `node scripts/check-player-rules.mjs` passes with `event_frontier` in `HIDDEN_INFO_GAMES` and the hidden-info section filled.
3. `cargo build -p wasm-api` succeeds with the new dispatch arms.

### Invariants

1. Every WASM-exposed view/export is output-equivalent across viewers and excludes undrawn deck order.
2. TypeScript decides no legality; the bridge transports Rust-projected views only.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-load-wasm.mjs`, `smoke-ui.mjs` — catalog-assertion updates.
2. `games/event_frontier/docs/HOW-TO-PLAY.md` + generated `apps/web/public/rules/event_frontier.md` — player-rules surface.

### Commands

1. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui`
2. `node scripts/check-player-rules.mjs`
3. The WASM/UI smoke plus player-rules check is the correct boundary — the bridge and viewer-safety are provable here; the DOM no-leak smoke lands in ticket 018.

## Outcome (2026-06-12)

Registered `event_frontier` in `wasm-api` across catalog, setup, view,
action-tree, action application, Level 1 bot turn, effects, public replay
export/import, replay reset, and trace rules-version dispatch. The WASM public
view is output-equivalent for observer and both seats, and `smoke:wasm` now
exercises setup/action/bot/effects/export/import without exposing undrawn deck
order.

Authored `games/event_frontier/docs/HOW-TO-PLAY.md`, generated
`apps/web/public/rules/event_frontier.md`, added `event_frontier` to
`HIDDEN_INFO_GAMES`, and updated WASM/UI smoke catalog assertions.

Verification:

- `cargo fmt --all --check`
- `cargo build -p wasm-api`
- `node scripts/check-player-rules.mjs`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:ui`
- `node scripts/check-doc-links.mjs`
