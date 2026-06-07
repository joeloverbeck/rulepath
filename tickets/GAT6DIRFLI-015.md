# GAT6DIRFLI-015: WASM exposure & wasm smoke

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api/src/lib.rs` + `crates/wasm-api/Cargo.toml` (thin glue: register `directional_flip`, route view/action-tree/apply/bot/effects/replay), `apps/web/src/wasm/client.ts` (TS types + call wrappers), `apps/web/scripts/smoke-load-wasm.mjs` (wasm smoke coverage).
**Deps**: 009, 011

## Problem

The web shell reaches `directional_flip` only through `wasm-api`. This ticket registers the game in the WASM bridge as **thin glue** (route/serialize/deserialize Rust-owned data to JSON — no rule/pass/flip/legality/bot/preview logic) across the standard public surfaces (catalog, new match, view, action tree, apply, bot turn, effects, replay export/import/step/reset), adds the TypeScript view/effect types (without making TS authoritative for legality), and extends the wasm load smoke. Realizes spec §12.1/§12.2.

## Assumption Reassessment (2026-06-07)

1. `crates/wasm-api/src/lib.rs` registers games explicitly: per-game `const GAME_*`/`GAME_*_DISPLAY_NAME`/`*_TRACE_RULES_VERSION`/`VARIANT_*_STANDARD` constants (lines 32–62), a `RegisteredGame` enum (line 134) with `RaceToN`/`ThreeMarks`/`ColumnFour` arms, a `list`/`resolve_game` dispatch, and per-op routing (`new_match` at line 190, etc.). `directional_flip` adds one arm at each dispatch site. Confirmed against the current file.
2. Spec §12.1 (WASM public surfaces + "wasm-api must stay thin") and §12.2 (allowed/forbidden TS client behavior) are authoritative. The TS client `apps/web/src/wasm/client.ts` adds a `directional_flip` public-view type + effect metadata + call wrappers only.
3. Cross-crate boundary under audit: `wasm-api` ↔ `games/directional_flip` (the Rust ops it routes) and ↔ `apps/web/src/wasm/client.ts` (JSON shape consumed by TS). This is an **enum/dispatch blast-radius** change: every `match RegisteredGame` site in `wasm-api/src/lib.rs` needs a new `DirectionalFlip` arm. Enumerate all match sites before editing (grep `RegisteredGame::`).
4. FOUNDATIONS §2 (Rust owns behavior; TS presentation-only) motivates this ticket: restate before coding — `wasm-api` may route/serialize/map only; it must contain no game rule/pass/flip/legal-target/bot/preview logic (spec §12.1). The TS client computes no legality, pass availability, flip consequences, or action paths (spec §12.2 forbidden).
5. This ticket extends the WASM JSON surface (an effect/view/action-tree serialization boundary, FOUNDATIONS §11): confirm the JSON payloads carry only the viewer-safe Rust views/effects (no-leak firewall) and that replay export/import over WASM stays deterministic (the `wasm-exported` golden trace, GAT6DIRFLI-013, is captured from this surface). The new `RegisteredGame::DirectionalFlip` arm is additive to a closed enum — all consumers are in-crate dispatch sites, updated here.

## Architecture Check

1. Mirroring the existing per-game constant + `RegisteredGame` arm + per-op routing pattern keeps `wasm-api` uniformly thin and auditable; no game logic crosses into the bridge or the browser.
2. No backwards-compatibility shims; additive registration.
3. `engine-core` untouched; the bridge passes opaque game-local payloads, no mechanic noun in the kernel (§3). TS gains types only, never legality (§2).

## Verification Layers

1. Thin-glue invariant -> manual review + codebase grep-proof: no rule/pass/flip/legality/bot/preview logic appears in `wasm-api`; it only routes to `games/directional_flip` ops.
2. Dispatch completeness -> codebase grep-proof: every `match RegisteredGame` site has a `DirectionalFlip` arm (no non-exhaustive fallthrough).
3. Payload no-leak -> no-leak visibility test: WASM JSON view/effect payloads carry only viewer-safe fields (FOUNDATIONS §11).
4. WASM determinism -> deterministic replay-hash check: replay export/import over WASM reproduces the `wasm-exported` trace hash (GAT6DIRFLI-013).
5. TS-not-authoritative -> manual review against spec §12.2: `client.ts` adds types/wrappers only.

## What to Change

### 1. `wasm-api` registration & routing

In `crates/wasm-api/src/lib.rs`, add `GAME_DIRECTIONAL_FLIP`/display-name/`DIRECTIONAL_FLIP_TRACE_RULES_VERSION` (`"directional_flip-rules-v1"`)/`VARIANT_DIRECTIONAL_FLIP_STANDARD` constants, a `RegisteredGame::DirectionalFlip` arm, and routing at every op site (list, new_match, view, action tree, apply, bot turn, effects, export/import/step/reset, effect + view + bot-rationale JSON mapping). `crates/wasm-api/Cargo.toml` gains the `directional_flip` dependency.

### 2. TypeScript client

In `apps/web/src/wasm/client.ts`, add the `directional_flip` public-view type, effect metadata types, and call wrappers — types and rendering data shape only, no legality.

### 3. WASM smoke

Extend `apps/web/scripts/smoke-load-wasm.mjs` to cover loading/instantiating a `directional_flip` match through the bridge.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `crates/wasm-api/Cargo.toml` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/scripts/smoke-load-wasm.mjs` (modify)

## Out of Scope

- The `DirectionalFlipBoard` renderer and shell integration (GAT6DIRFLI-017).
- Browser E2E / a11y smoke (GAT6DIRFLI-018).
- Any game rule logic (owned by `games/directional_flip`; FOUNDATIONS §2).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:wasm` — wasm load smoke covers `directional_flip`.
2. `cargo build -p wasm-api` (with `wasm32-unknown-unknown` target) — bridge compiles with the new arm.
3. `bash scripts/boundary-check.sh` — no mechanic noun leaked into the kernel.

### Invariants

1. `wasm-api` contains no game rule/pass/flip/legality/bot/preview logic; TS computes no legality (FOUNDATIONS §2, spec §12.1/§12.2).
2. WASM JSON payloads are viewer-safe and replay over WASM is deterministic (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-load-wasm.mjs` — directional-flip load/instantiate coverage.
2. `games/directional_flip/tests/golden_traces/wasm-exported.trace.json` — captured/validated from this surface (corpus owned by GAT6DIRFLI-013).

### Commands

1. `npm --prefix apps/web run smoke:wasm`
2. `cargo build -p wasm-api && npm --prefix apps/web run build`
3. The wasm smoke + build is the correct boundary; the in-browser play smoke is GAT6DIRFLI-018.
