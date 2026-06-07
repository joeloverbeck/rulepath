# GAT7DRALITCOM-016: WASM exposure + multi-segment replay export/import + wasm smoke

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` (register `draughts_lite`; extend the replay-export and action-path-parse paths to multi-segment), `crates/wasm-api/Cargo.toml` / `Cargo.lock` (game dependency), `apps/web/src/wasm/client.ts` (Draughts Lite and nested action-tree types), `apps/web/scripts/smoke-load-wasm.mjs` (wasm smoke), `games/draughts_lite/tests/golden_traces/wasm-exported.trace.json` (new), `games/draughts_lite/tests/replay.rs` (native trace coverage).
**Deps**: 010, 012

## Problem

The WASM bridge exposes Draughts Lite to the web (list/create/view/action-tree/apply/bot/effects/replay-export) and must stop assuming a one-segment action path. Today `crates/wasm-api/src/lib.rs` rejects multi-segment commands on export (the `unsupported_replay_action_path` guard at ~L132–138 returns only `action_path[0]`) and `parse_action_path` (~L1231) wraps the input string as a single segment without splitting. This ticket registers the game and extends both paths so multi-segment paths round-trip losslessly, while existing one-segment games stay valid. It authors the WASM-exported golden trace and a wasm smoke check.

## Assumption Reassessment (2026-06-07)

1. `crates/wasm-api/src/lib.rs` registers games via static consts + match arms (`GAME_DIRECTIONAL_FLIP`, `resolve_game`, per-game `validate_command`/setup/view/export arms — verified at `lib.rs:45,219,497,798`). The one-segment assumptions are concrete: the export guard `if self.action_path.len() != 1 { … "unsupported_replay_action_path" … } Ok(self.action_path[0].clone())` at `lib.rs:132–138`, and `parse_action_path` at `lib.rs:1231` returning `vec![action_path.to_owned()]` (no split). `apps/web/src/wasm/client.ts:283` already types `action_path: string[]`.
2. The WASM contract is fixed by spec §R13 (registration list; "preserve path segment order exactly"; "must not flatten a path into an ambiguous string"; lossless string→segment parse) and §R10 (multi-segment one-command replay export). `games/draughts_lite/src/replay_support.rs` (010) and `bots.rs` (012) supply the native surfaces the bridge calls.
3. Cross-artifact boundary under audit: the WASM action-tree/command/effect/replay surface is consumed by the web renderer (018) and the web smoke (019); the multi-segment change touches a shared bridge path used by ALL games, so existing one-segment games (`race_to_n`/`three_marks`/`column_four`/`directional_flip`) must continue to export/apply unchanged.
4. FOUNDATIONS §2/§11 motivate this ticket: restate before coding — WASM is a boundary, not a second rules engine; it forwards commands to Rust validation and never decides legality. The export must preserve segment order and not flatten to an ambiguous string.
5. No-leak + determinism + schema-extension surface (§11, §13 boundary): the replay export reaches the browser, so confirm it carries only viewer-safe data and that the multi-segment extension is **additive** to the existing list-shaped `action_path` (no Trace Schema bump → no §13 ADR trigger). The export must be deterministic and the WASM-exported trace must validate natively (spec §R13 boundary tests).

## Architecture Check

1. Generalizing the existing export/parse paths to N segments (rather than special-casing draughts) fixes the bridge for every future multi-segment game and keeps one code path; the change is additive over the already-list-shaped `action_path`.
2. No backwards-compatibility shims; the one-segment guard is replaced by general multi-segment handling, not aliased.
3. `engine-core` stays noun-free (§3) — the bridge moves generic `ActionPath { segments }` values; no draughts noun enters the kernel or the bridge's generic helpers.

## Verification Layers

1. Registration -> wasm smoke: `draughts_lite` appears in `list_games`, a match creates, the initial view serializes, the initial action tree has legal origins.
2. Multi-segment round-trip -> WASM-exported golden trace + native replay-check: a multi-jump path round-trips through action-tree → apply → effects → replay export and validates natively.
3. One-segment compatibility -> `cargo test --workspace` + existing wasm smoke: existing games export/apply unchanged.
4. No flatten / order preserved -> schema/serialization validation: exported `action_path` is the ordered segment list, not a flattened string.

## What to Change

### 1. Register `draughts_lite` in `wasm-api`

Add the game consts (id, rules version, default variant), `resolve_game` arm, and the setup/view/action-tree/apply/bot/effects/replay-export arms, mirroring the `directional_flip` registration.

### 2. Multi-segment export & parse

Replace the `action_path.len() != 1` export guard (L132–138) with general multi-segment serialization preserving order; extend `parse_action_path` (L1231) to split a delimited dev-entry string losslessly into the canonical segment list before validation. Keep one-segment games valid.

### 3. WASM-exported trace & smoke

Author `games/draughts_lite/tests/golden_traces/wasm-exported.trace.json` (a multi-segment exported command that validates natively); extend `apps/web/scripts/smoke-load-wasm.mjs` to cover the Draughts Lite list/create/view/action-tree path.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify — register `draughts_lite`; multi-segment export + parse)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify — Draughts Lite wasm smoke)
- `games/draughts_lite/tests/golden_traces/wasm-exported.trace.json` (new)

## Out of Scope

- The board renderer / input model (GAT7DRALITCOM-018 — consumes this bridge).
- Browser E2E / a11y smoke (GAT7DRALITCOM-019).
- Native golden traces other than the WASM-exported one (GAT7DRALITCOM-014).
- Any Trace Schema version bump (forbidden; spec §R10/§R13).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:wasm` — Draughts Lite loads, lists, creates, serializes the view, and exports a multi-segment replay.
2. `cargo test --workspace` — existing one-segment games' export/apply/replay unchanged.
3. `cargo run -p replay-check -- --game draughts_lite --all` (once registered in 017) validates the WASM-exported trace natively.

### Invariants

1. WASM forwards to Rust validation and decides no legality; multi-segment paths preserve order and never flatten (FOUNDATIONS §2; spec §R13).
2. The multi-segment change is additive — no Trace Schema bump, existing games unaffected (FOUNDATIONS §11/§13; spec §R10).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/tests/golden_traces/wasm-exported.trace.json` — multi-segment exported command validating natively.
2. `apps/web/scripts/smoke-load-wasm.mjs` — Draughts Lite wasm boundary smoke.

### Commands

1. `npm --prefix apps/web run smoke:wasm`
2. `cargo test --workspace && cargo build -p wasm-api`
3. The wasm smoke + workspace tests are the correct boundary; the browser-interaction path is exercised in GAT7DRALITCOM-018/019.

## Outcome

- Registered `draughts_lite` in `wasm-api` for catalog, match creation, public view, nested action tree, human apply, Level 1 bot turn, effect log, replay export/import, replay reset, and replay step.
- Preserved nested `ActionChoice.next` in WASM action-tree JSON and split delimited dev-entry paths with `>` so ordered multi-segment commands such as `from/r3c2>to/r4c1` reach Rust validation as `["from/r3c2", "to/r4c1"]`.
- Added Draughts Lite public-view/effect JSON serialization in the bridge, kept existing one-segment games on their replay support, and added WASM unit coverage for Draughts Lite multi-segment export/import without changing Trace Schema v1.
- Added the WASM-exported Draughts Lite golden trace and included it in native Draughts Lite replay tests. `replay-check --game draughts_lite --all` remains the GAT7DRALITCOM-017 tool-registration proof.
- Extended the web WASM smoke to list, create, view, traverse nested action choices, apply a complete multi-segment path, run a bot, fetch effects, export/import replay, reset, and step Draughts Lite.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo test -p wasm-api`
3. `cargo test -p draughts_lite replay`
4. `npm --prefix apps/web run smoke:wasm`
5. `cargo build -p wasm-api`
6. `cargo test --workspace`
7. `npm --prefix apps/web run build`
8. `git diff --check`
