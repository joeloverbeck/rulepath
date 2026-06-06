# CIBUILD-001: Build web artifacts once in gate-1 instead of per smoke step

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: None — CI config only (`.github/workflows/gate-1-game-smoke.yml`); no crate, schema, trace, or product-behavior change.
**Deps**: None

## Problem

The gate-1 workflow (`.github/workflows/gate-1-game-smoke.yml`) runs five web steps that each rebuild the same artifacts from scratch: `smoke:wasm`, `Web build`, `smoke:ui`, `smoke:preview`, and `smoke:e2e`. Because every `apps/web` smoke npm script begins with its own build, gate-1 invokes the release `cargo build -p wasm-api --target wasm32-unknown-unknown` **5 times** and the full `tsc --noEmit && vite build` **4 times** on every push/PR. The WASM release compile dominates gate-1 wall-clock, so this is the bulk of the redundancy.

This is pure CI wall-clock/cost waste. Coverage and correctness are unaffected — the goal is to build the wasm + web artifacts once and have the smoke scripts consume them.

## Assumption Reassessment (2026-06-06)

1. `apps/web/package.json` scripts confirmed: `build:wasm` = `cargo build -p wasm-api --target wasm32-unknown-unknown --release && install … public/wasm_api.wasm`; `build` = `npm run build:wasm && tsc --noEmit && vite build`; and each smoke script prepends a build — `smoke:wasm` = `build:wasm + node scripts/smoke-load-wasm.mjs`, `smoke:ui`/`smoke:preview`/`smoke:e2e` = `npm run build + node …`. So the per-step builds are real and redundant.
2. The node smoke scripts consume **prebuilt artifacts only** and never self-build — verified: `scripts/smoke-load-wasm.mjs` and `scripts/smoke-ui.mjs` read `apps/web/public/wasm_api.wasm`; `scripts/smoke-preview.mjs` and `e2e/shell.smoke.mjs` serve `apps/web/dist/` and fetch `wasm_api.wasm` from it. A single `npm run build` produces both `public/wasm_api.wasm` (via `build:wasm`) and `dist/` (via `vite build`), satisfying all five steps. This is the load-bearing reuse premise for the fix.
3. Cross-artifact boundary under audit: the gate-1 workflow contract and the `apps/web` npm script / node-smoke-script interface it drives. No product code or schema is touched; the change is confined to *how* CI sequences existing, unchanged commands.
4. Adjacent observation (classified as out of scope, not a bug): the coupled `smoke:*` scripts are intentionally convenient for local one-command runs. Decoupling build from smoke in `package.json` would change local DX and is a larger change than this ticket warrants — keep it as a possible future cleanup, not part of this ticket.

## Architecture Check

1. Cleanest low-risk approach: in the workflow, run `npm --prefix apps/web run build` **once**, then invoke the node smoke scripts **directly** (`node apps/web/scripts/smoke-load-wasm.mjs`, `node apps/web/scripts/smoke-ui.mjs`, `node apps/web/scripts/smoke-preview.mjs`, and the four `node apps/web/e2e/*.mjs` files). This requires **no `package.json` change** and preserves local DX — developers keep using the coupled `smoke:*` scripts locally; only CI sequences the build once. The alternative (splitting `package.json` into build-free `smoke:*:built` variants) adds script surface and changes local commands for no CI benefit over the direct-invocation approach.
2. No backwards-compatibility aliasing/shims introduced — the existing `smoke:*` npm scripts are left untouched and still work locally.
3. No `engine-core`/`game-stdlib`/`games/*` change; `engine-core` stays noun-free and the mechanic atlas is untouched. The boundary-check and docs-link steps remain unchanged gating steps.

## Verification Layers

1. Build-once invariant (gate-1 invokes `build:wasm` once and `vite build` once) -> codebase grep-proof: a single `npm … run build` invocation and no `smoke:*` build-prefixed scripts remain in `gate-1-game-smoke.yml`.
2. Coverage-intact invariant (the same smoke assertions still run: wasm load, UI, preview, shell + a11y/no-leak + three_marks + column_four E2E) -> manual review: every previously-covered node script is still invoked exactly once, directly.
3. Workflow-validity invariant (the workflow parses and every step invokes a real command against prebuilt artifacts) -> simulation/CLI run: local dry-run of `npm --prefix apps/web run build` followed by each `node …` smoke script passes green.
4. Existing-gates-intact invariant (Rust simulate/replay/fixture/rule-coverage steps, boundary-check, docs-link remain unchanged and gating) -> manual review.

## What to Change

### 1. `.github/workflows/gate-1-game-smoke.yml`

Replace the five build-prefixed web steps (`WASM smoke` via `smoke:wasm`, `Web build`, `race_to_n UI smoke` via `smoke:ui`, `Static preview smoke` via `smoke:preview`, `Browser shell + a11y/no-leak E2E including column_four` via `smoke:e2e`) with:

- one `Build web artifacts` step: `npm --prefix apps/web run build`;
- then direct node invocations consuming those artifacts:
  - `node apps/web/scripts/smoke-load-wasm.mjs` (was `smoke:wasm`)
  - `node apps/web/scripts/smoke-ui.mjs` (was `smoke:ui`)
  - `node apps/web/scripts/smoke-preview.mjs` (was `smoke:preview`)
  - `node apps/web/e2e/shell.smoke.mjs`, `node apps/web/e2e/a11y-noleak.smoke.mjs`, `node apps/web/e2e/three-marks.smoke.mjs`, `node apps/web/e2e/column-four.smoke.mjs` (was `smoke:e2e`)

Confirm working-directory/path handling: the node scripts use `__dirname`-relative paths to `apps/web/{public,dist}`, so invoking them from the repo root by absolute/`apps/web/...` path is correct; do not `cd`-prefix unless a script assumes CWD. Keep `Install web dependencies`, the Rust per-game steps, `Engine boundary`, and `Docs link check` exactly as they are.

## Files to Touch

- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- `gate-0-hygiene.yml` and `gate-2-benchmarks.yml` — no redundant-build issue there.
- Decoupling build from smoke inside `apps/web/package.json` (would change local DX; possible separate future ticket — see Assumption Reassessment item 4).
- Any caching layer (`actions/cache`, sccache) for the cargo/wasm build — a larger, separate optimization.
- Adding/removing/altering any smoke assertion or game coverage.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -c "run build:wasm\|run smoke:" .github/workflows/gate-1-game-smoke.yml` — returns `0` build-prefixed smoke invocations; a single `run build` step remains.
2. Local dry-run: `npm --prefix apps/web ci && npm --prefix apps/web run build && node apps/web/scripts/smoke-load-wasm.mjs && node apps/web/scripts/smoke-ui.mjs && node apps/web/scripts/smoke-preview.mjs && node apps/web/e2e/shell.smoke.mjs && node apps/web/e2e/a11y-noleak.smoke.mjs && node apps/web/e2e/three-marks.smoke.mjs && node apps/web/e2e/column-four.smoke.mjs` — all pass.
3. `bash scripts/boundary-check.sh` — still green (boundary step unchanged).

### Invariants

1. Every smoke assertion that ran before still runs exactly once; no game's browser/UI/wasm coverage is dropped.
2. `build:wasm` and `vite build` each execute once per gate-1 run.
3. Existing Rust per-game steps, boundary-check, and docs-link steps are unchanged and still gating.

## Test Plan

### New/Modified Tests

1. `None — CI-configuration ticket; the invoked smoke/e2e scripts are unchanged and already cover themselves. Verification is command-based plus a manual workflow review.`

### Commands

1. `grep -nE "run build|run smoke:|node apps/web" .github/workflows/gate-1-game-smoke.yml`
2. `npm --prefix apps/web run build && node apps/web/scripts/smoke-ui.mjs && node apps/web/scripts/smoke-preview.mjs && node apps/web/e2e/column-four.smoke.mjs`
3. The single-build dry-run is the correct verification boundary: it proves the node scripts run against prebuilt artifacts without their npm build wrappers, which is the only behavior this ticket changes.

## Outcome

Completed: 2026-06-06

Changed `.github/workflows/gate-1-game-smoke.yml` so Gate 1 builds web artifacts once with `npm --prefix apps/web run build`, then runs the existing WASM/UI/preview/browser smoke scripts directly with `node` against the prebuilt artifacts.

Deviations from original plan: none.

Verification:

- `grep -c "run build:wasm\|run smoke:" .github/workflows/gate-1-game-smoke.yml` returned `0`.
- `npm --prefix apps/web ci && npm --prefix apps/web run build && node apps/web/scripts/smoke-load-wasm.mjs && node apps/web/scripts/smoke-ui.mjs && node apps/web/scripts/smoke-preview.mjs && node apps/web/e2e/shell.smoke.mjs && node apps/web/e2e/a11y-noleak.smoke.mjs && node apps/web/e2e/three-marks.smoke.mjs && node apps/web/e2e/column-four.smoke.mjs` passed.
- `bash scripts/boundary-check.sh` passed.
