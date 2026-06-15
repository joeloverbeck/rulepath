# GAT15RIVLEDTEX-016: WASM registration, player-rules docs, and catalog smoke harnesses

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api` (`src/lib.rs`, `Cargo.toml`); `games/river_ledger/docs/HOW-TO-PLAY.md`; generated `apps/web/public/rules/river_ledger.md`; `scripts/check-player-rules.mjs`; `apps/web/scripts/{smoke-load-wasm,smoke-ui}.mjs`
**Deps**: GAT15RIVLEDTEX-010, GAT15RIVLEDTEX-013

## Problem

River Ledger must be reachable from the browser through the Rust↔WASM JSON bridge: catalog entry with supported seat counts 3–6, seat labels, viewer modes, and dispatch for setup/action/view/effects/replay/bot — plus the player-rules docs the CI checker requires for a hidden-info game. TypeScript must decide no legality.

## Assumption Reassessment (2026-06-14)

1. `crates/wasm-api/src/lib.rs` has `enum RegisteredGame` (14 variants, exhaustive matches), `enum MatchRecord`, `list_games`, `with_catalog_seat_metadata`/`catalog_seat_metadata_fields`/`catalog_viewer_modes_json`, and `#[cfg(test)]` `pairwise_no_leak_result`/`assert_pairwise_no_leak`; `scripts/check-player-rules.mjs` `HIDDEN_INFO_GAMES` includes `poker_lite`; `apps/web/scripts/{smoke-load-wasm,smoke-ui}.mjs` carry hardcoded per-game catalog assertions (verified).
2. `specs/...-base.md` §4.3 (WASM row), §10.6/§10.7, and the reassess clarification fix the dispatch set; `RegisteredGame`/`MatchRecord` and the no-leak helpers are internal/test-only symbols — extend their dispatch in place, do not export them.
3. Cross-artifact boundary under audit: adding `RiverLedger` to the exhaustive `RegisteredGame`/`MatchRecord` matches is additive (new arm at each site); it consumes the bots (013) and replay export (010). `apps/web/public/rules/river_ledger.md` is a **generated artifact** — author `HOW-TO-PLAY.md` and run `scripts/copy-player-rules.mjs`; `scripts/check-player-rules.mjs` enforces source↔generated parity and requires the hidden-information section for `HIDDEN_INFO_GAMES` members.
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: the bridge serializes Rust-decided legal actions, views, previews, effects, and outcomes; TypeScript renders them and never computes legality, call price, hand rank, winner, or split.
5. §11 no-leak enforcement surface under audit: every browser payload (`list_games`, views, action trees, effects, replay export, bot explanations) is viewer-safe by construction — no hole/burn/deck-tail/future-community/private-diagnostic fact reaches an unauthorized viewer; the `#[cfg(test)]` pairwise no-leak harness dispatch gains a `RiverLedger` arm.

## Architecture Check

1. Registering one new arm at each existing dispatch site keeps the bridge exhaustive-match-safe and the no-leak guarantee a Rust-side projection, matching every prior game's WASM wiring.
2. No backwards-compatibility aliasing/shims — additive variant + arms; no exported internal symbols.
3. `engine-core` stays noun-free (§3); the bridge reuses generic contracts; no `game-stdlib` change (§4).

## Verification Layers

1. Catalog lists River Ledger with supported seats 3–6, labels, viewer modes -> `npm --prefix apps/web run smoke:wasm` (smoke-load-wasm asserts the new arm).
2. WASM dispatch round-trips setup/action/view/effects/replay/bot -> `cargo test -p wasm-api`.
3. Player-rules parity + hidden-info section -> `node scripts/check-player-rules.mjs`.
4. Browser payloads carry no hidden fact -> wasm-api no-leak test (`assert_pairwise_no_leak` River Ledger arm, §11).

## What to Change

### 1. `crates/wasm-api`

Add the `river_ledger` dependency + import; add `RegisteredGame::RiverLedger`; update `list_games`, catalog metadata/seat labels/viewer modes (3–6), `MatchRecord`, setup/action/view/effects/replay/bot dispatch, and the `#[cfg(test)]` `pairwise_no_leak_result`/`assert_pairwise_no_leak` dispatch.

### 2. Player-rules docs + checker + smoke harnesses

Author `games/river_ledger/docs/HOW-TO-PLAY.md` (incl. hidden-information section); run `scripts/copy-player-rules.mjs` to generate `apps/web/public/rules/river_ledger.md`; add `river_ledger` to `HIDDEN_INFO_GAMES` in `scripts/check-player-rules.mjs`; add the River Ledger catalog assertion to `apps/web/scripts/smoke-load-wasm.mjs` and `smoke-ui.mjs`.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `crates/wasm-api/Cargo.toml` (modify)
- `games/river_ledger/docs/HOW-TO-PLAY.md` (new)
- `apps/web/public/rules/river_ledger.md` (new — generated via `scripts/copy-player-rules.mjs`)
- `scripts/check-player-rules.mjs` (modify — `HIDDEN_INFO_GAMES`)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify)
- `apps/web/scripts/smoke-ui.mjs` (modify)

## Out of Scope

- The `RiverLedgerBoard.tsx` renderer + app-shell registration (GAT15RIVLEDTEX-017).
- `ci/games.json`, `smoke:e2e`, and README catalog reconciliation (GAT15RIVLEDTEX-018) — `check-catalog-docs` shows an expected red window until then.
- Any TypeScript legality (forbidden, §2).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` and `cargo check --workspace` — dispatch compiles; no-leak harness arm passes.
2. `npm --prefix apps/web run smoke:wasm` — catalog includes `river_ledger` with seats 3–6.
3. `node scripts/check-player-rules.mjs` — source↔generated parity + hidden-info section present.

### Invariants

1. The bridge serializes Rust-decided state; TypeScript decides no legality (§2).
2. No browser payload leaks a hidden fact (§11); internal/test-only symbols stay unexported.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` (modify) — `RiverLedger` dispatch + `#[cfg(test)]` pairwise no-leak arm.
2. `apps/web/scripts/{smoke-load-wasm,smoke-ui}.mjs` (modify) — catalog presence assertions.

### Commands

1. `cargo test -p wasm-api && cargo check --workspace`
2. `npm --prefix apps/web run smoke:wasm && node scripts/check-player-rules.mjs`
3. These cover the bridge + player-rules boundary; full e2e/catalog-docs reconciliation is in GAT15RIVLEDTEX-018.

## Outcome

Completed 2026-06-14.

- Added River Ledger to `crates/wasm-api` dependencies, catalog registration, match records, setup/view/action/effects/bot/replay dispatch, trace rules versioning, and the hidden-info pairwise no-leak harness.
- Added a 3-6 seat River Ledger catalog entry with seat labels, viewer modes, hidden-info metadata, and smoke harness assertions.
- Authored `games/river_ledger/docs/HOW-TO-PLAY.md`, generated `apps/web/public/rules/river_ledger.md`, updated the public-rules manifest, and added River Ledger to hidden-info player-rules validation.
- Replay dispatch covers River Ledger command-trace import/step/reset and public replay export. Viewer-scoped public-export import remains on the existing game-local timeline API boundary and is not part of this ticket's smoke path.

Verification passed:

1. `cargo test -p wasm-api`
2. `cargo check --workspace`
3. `npm --prefix apps/web run smoke:wasm`
4. `node scripts/check-player-rules.mjs`
5. `npm --prefix apps/web run smoke:ui`
6. `cargo fmt --all --check`
7. `node scripts/check-doc-links.mjs`
8. `bash scripts/boundary-check.sh`
9. `git diff --check`
