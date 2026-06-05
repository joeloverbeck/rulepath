# GAT0REPSKE-002: `wasm-api` WASM artifact and `apps/web` shell

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — extends `crates/wasm-api` with a WASM build surface and a placeholder exported call; adds the `apps/web` React/TypeScript presentation shell.
**Deps**: GAT0REPSKE-001

## Problem

The Rust→WASM→browser path must be proven end to end before any gameplay exists. `wasm-api` must build to a loadable WASM artifact, and the React/TypeScript shell under `apps/web` must load that artifact and invoke a placeholder call that returns a version/string. This is the plumbing proof for the static web shell; no game logic is involved.

## Assumption Reassessment (2026-06-05)

1. `crates/wasm-api` is created by GAT0REPSKE-001 (declared `Deps`); `apps/` is absent (greenfield, verified `test -e` 2026-06-05).
2. Spec §2/§3 WB2 + `docs/ARCHITECTURE.md` §1 (`apps/web` in the tree), §10 (WASM API shape), and §2 dependency direction (`apps/web -> wasm-api` package boundary). Spec assumption A-4 makes the `wasm-bindgen`/`wasm-pack`-style toolchain an implementation choice for this ticket, not fixed by the spec.
3. Cross-artifact boundary under audit: the JS↔WASM package boundary. Per `docs/ARCHITECTURE.md:55`/`:67`, `apps/web` MUST NOT import game Rust internals except through the WASM/API package boundary.
4. §2 Behavior authority: TypeScript/React is presentation-only. The shell holds no legality or rule state; it loads the WASM artifact and renders a Rust-returned string.
5. §11 no-leak firewall (deferred enforcement surface): the placeholder call returns a static version/string carrying no game or hidden state. The viewer-safe browser-payload contract and its no-leak firewall are built later (Gate 3 batched WASM API / Gate 8 hidden information). This ticket introduces no browser payload path that the later firewall would have to undo, and no nondeterministic input enters any canonical form.

## Architecture Check

1. A thin placeholder (version string) over the real WASM boundary proves the toolchain without inventing speculative gameplay API surface — cleaner than stubbing a fake action/view API now.
2. No backwards-compatibility shims — `apps/web` imports `wasm-api` through the package boundary, never Rust internals.
3. `engine-core` is untouched (stays noun-free); no `game-stdlib` change.

## Verification Layers

1. `wasm-api` builds to a WASM artifact -> simulation/CLI run (WASM build command produces a `.wasm`).
2. Web shell builds + loads the artifact -> simulation/CLI run + manual review (`apps/web` production build; load smoke).
3. TypeScript holds no legality -> FOUNDATIONS alignment check (§2 — shell only renders the Rust-returned string).

## What to Change

### 1. `wasm-api` WASM export

Add a `wasm-bindgen`-style exported function returning a version/string, plus the build configuration to compile `wasm-api` to a WASM target.

### 2. `apps/web` presentation shell

React/TypeScript shell: build tooling (`package.json` + bundler config), an entry point, and a component that loads the WASM artifact and displays the returned string.

## Files to Touch

- `crates/wasm-api/Cargo.toml` (modify) — created by GAT0REPSKE-001
- `crates/wasm-api/src/lib.rs` (modify) — created by GAT0REPSKE-001
- `apps/web/package.json` (new)
- `apps/web/index.html` (new)
- `apps/web/src/main.tsx` (new)
- `apps/web/<bundler/tsconfig>` (new)

## Out of Scope

- Batched gameplay WASM API, game picker, view/action/effect stores, replay controls (Gate 3).
- Any legality or rule-state logic in TypeScript (spec §8).
- Any real game (Gate 1).

## Acceptance Criteria

### Tests That Must Pass

1. The WASM build command produces a loadable artifact from `wasm-api`.
2. The `apps/web` production build completes without error.
3. Load smoke: the shell loads the artifact and the placeholder call returns the expected version/string.

### Invariants

1. `apps/web` contains no legality or rule-state logic (§2).
2. `apps/web` imports `wasm-api` only via the package/WASM boundary, not Rust internals (ARCHITECTURE.md §2).

## Test Plan

### New/Modified Tests

1. `apps/web` load-WASM smoke (headless if a harness is practical, else a manual runbook step) — proves the artifact loads and the call returns.
2. `crates/wasm-api/src/lib.rs` smoke test for the exported call.

### Commands

1. `wasm-pack build crates/wasm-api --target web` (or the project-chosen equivalent toolchain) — WASM artifact build.
2. `npm --prefix apps/web run build` (or chosen equivalent) — web shell production build.
3. Load-smoke command/runbook (browser or headless) — a narrower boundary than a full e2e because no gameplay exists yet.
