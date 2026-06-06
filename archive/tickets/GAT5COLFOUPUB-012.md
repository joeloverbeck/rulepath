# GAT5COLFOUPUB-012: Column Four WASM exposure & wasm smoke

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — modify `crates/wasm-api/src/lib.rs`; modify `apps/web/scripts/smoke-load-wasm.mjs`
**Deps**: 006, 008

## Problem

The browser shell reaches `column_four` only through the thin WASM bridge. `crates/wasm-api` must expose the game through the existing catalog/match/view/action/effect/bot/replay operations without becoming a rules owner, and the wasm smoke must prove the bridge works while existing games still function (spec §9, §18 WASM smoke).

## Assumption Reassessment (2026-06-06)

1. `crates/wasm-api/src/lib.rs` registers games via a `RegisteredGame` enum (`RaceToN`, `ThreeMarks`), a `MatchRecord` enum, `GAME_*` id/display constants, `list_games`, `resolve_game`, and per-op match arms (`new_match`/`get_view`/`get_action_tree`/`apply_action`/`run_bot_turn`/`get_effects`/`export_replay`/`import_replay`/`replay_step`/`replay_reset`) — verified at lib.rs:117 (`enum RegisteredGame`) and lines 28–29 (`GAME_THREE_MARKS`). Adding `column_four` means a new enum variant, a `MatchRecord::ColumnFour` variant, `GAME_COLUMN_FOUR` constants, and an arm in each op.
2. Spec §9 (WASM operations + catalog) and §18 (WASM smoke) define behavior. The exposed surfaces (view 004, effects 005, replay 006, bots 008, rules 003) already exist. The wasm smoke harness is `apps/web/scripts/smoke-load-wasm.mjs` (run via `npm run smoke:wasm`), verified in `apps/web/package.json`.
3. Cross-artifact boundary under audit: the thin-WASM/client boundary (`docs/WASM-CLIENT-BOUNDARY.md`) and the viewer-safe payload rule. The bridge adapts game types into viewer-safe JSON; it must not own rules, tactical policy, or replay projection (those stay in `games/column_four`).
4. FOUNDATIONS §2 (behavior authority) and §11 (viewer-safe payloads) motivate this ticket. Restating: the WASM layer is a thin bridge — it must not add legality shortcuts or recompute landing/terminal; existing games must remain listed and functional (additive change).
5. Deterministic replay (§11) and the no-leak firewall are the enforcement surfaces: the WASM export/import path must round-trip `column_four` traces deterministically (`game_id`/`rules_version` correct) and expose no hidden/internal field — confirmed by the wasm smoke before the web renderer (014) consumes it.

## Architecture Check

1. Extending the existing enum/match registry (rather than a parallel browser API for `column_four`) keeps the bridge thin and uniform across games — cleaner and required by `docs/WASM-CLIENT-BOUNDARY.md`. Alternative (game-specific JS legality shortcut) is a §12 stop condition.
2. No backwards-compatibility aliasing/shims — additive enum/op arms; existing variants untouched.
3. `engine-core` stays free of mechanic nouns (the bridge passes opaque game payloads); `game-stdlib` untouched.

## Verification Layers

1. Registration invariant -> codebase grep-proof: `column_four` present in `RegisteredGame`, `MatchRecord`, `list_games`, `resolve_game`, and every op arm.
2. Existing-games invariant -> simulation/CLI + wasm smoke: `race_to_n` and `three_marks` still list and play through WASM after the change.
3. Bridge-thinness invariant -> manual review: no rules/legality/landing/terminal recomputed in `wasm-api`; it delegates to `games/column_four`.
4. WASM-behavior invariant -> skill/CLI dry-run (wasm smoke): catalog lists `column_four`; new match returns a 7×6 non-terminal board; action tree returns seven legal columns; apply updates view+effects; full-column + stale diagnostics surface; bot turn yields a public rationale effect; export/import/replay-step project a `column_four` view.
5. No-leak invariant -> no-leak visibility test: WASM payloads expose no hidden/internal field.

## What to Change

### 1. `crates/wasm-api/src/lib.rs`

Add `GAME_COLUMN_FOUR`/display constants, a `RegisteredGame::ColumnFour` variant, a `MatchRecord::ColumnFour { … }` variant (state/effects/commands), and `column_four` arms in `list_games`, `resolve_game`, and every browser op, delegating to `games/column_four`. Keep `column_four` action paths one-segment (spec §9 replay-export compatibility) unless extended with full test coverage.

### 2. `apps/web/scripts/smoke-load-wasm.mjs`

Extend the wasm smoke to exercise `column_four` per spec §18: catalog listing, new match, 7×6 view, seven legal columns, apply, full-column + stale diagnostics, bot turn with rationale, export/import, replay step — and assert existing games still work.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify)

## Out of Scope

- The `ColumnFourBoard` renderer and `ColumnFourPublicView` TS type (GAT5COLFOUPUB-014).
- Browser E2E / a11y smoke (GAT5COLFOUPUB-015) and CI wiring (016).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:wasm` — column_four bridge ops pass and existing games still work.
2. `grep -nE "ColumnFour|GAME_COLUMN_FOUR|column_four" crates/wasm-api/src/lib.rs` — registration present in enum + op arms.
3. `cargo build -p wasm-api` — the crate compiles.

### Invariants

1. The WASM layer adds no rules/legality/landing/terminal logic; it delegates to `games/column_four`.
2. Existing games remain listed and functional; payloads stay viewer-safe.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-load-wasm.mjs` — extended to cover the `column_four` op set and an existing-games regression check.

### Commands

1. `npm --prefix apps/web run smoke:wasm`
2. `cargo build -p wasm-api`
3. `cargo test -p wasm-api` — narrower Rust-side check of the registry arms where unit-testable.
