# GAT5COLFOUPUB-016: Column Four CI workflow integration

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — modify `.github/workflows/gate-1-game-smoke.yml`, `.github/workflows/gate-2-benchmarks.yml`
**Deps**: 011, 012, 013, 015

## Problem

`column_four` must run in CI alongside existing games: per-game simulation, replay drift, fixture validation, rule coverage, WASM smoke, web build, UI/browser smoke, plus the benchmark lane — without weakening existing checks or broadening into a CI redesign (spec §18 CI/testing/benchmark scope).

## Assumption Reassessment (2026-06-06)

1. `.github/workflows/gate-1-game-smoke.yml` invokes per-game checks with the game id hardcoded per step (e.g. `cargo run -p simulate -- --game three_marks --games 1000`, `replay-check --game three_marks --all`, `fixture-check --game three_marks`, `rule-coverage --game three_marks`) — verified. Adding `column_four` means parallel hardcoded steps. `gate-2-benchmarks.yml` runs the benchmark lanes (per `docs/adr/0002-ci-benchmark-gating-lanes.md`).
2. Spec §18 (CI/testing/benchmark) requires `column_four` in simulation/replay-check/fixture-check/rule-coverage (registered in GAT5COLFOUPUB-013), WASM smoke (012), web build + UI/browser smoke (014/015), benchmark lane (011), and that boundary-check + docs-link checks keep running.
3. Cross-artifact boundary under audit: the CI workflow contract (the gate lanes) and the tools/smoke commands they invoke. All dependencies (tools 013, wasm smoke 012, e2e 015, benches 011) must land first; this ticket only wires them into CI.
4. FOUNDATIONS §3 (kernel boundary, enforced by `scripts/boundary-check.sh` in CI) and §6 (evidence in CI) motivate this ticket: CI must keep running boundary-check and docs-link checks; `column_four` must not trip boundary-check.

## Architecture Check

1. Adding parallel hardcoded per-game steps follows the established workflow convention — cleaner and lower-risk than refactoring CI to a game matrix this gate (spec §18 forbids unrelated CI redesign).
2. No backwards-compatibility aliasing/shims — additive steps.
3. No engine-core/game-stdlib change; CI invokes existing tools/smoke. Boundary-check remains a gating step.

## Verification Layers

1. CI-coverage invariant -> codebase grep-proof: `column_four` appears in gate-1 simulation/replay-check/fixture-check/rule-coverage/wasm/web-smoke steps and the gate-2 benchmark lane.
2. Existing-checks-intact invariant -> manual review: existing `race_to_n`/`three_marks` steps and `boundary-check`/docs-link steps are unchanged and still gating.
3. Workflow-validity invariant -> manual review / YAML lint: the workflow files parse and the new steps invoke real commands (validated against GAT5COLFOUPUB-011/012/013/015).
4. Boundary invariant -> FOUNDATIONS alignment check (§3): `bash scripts/boundary-check.sh` remains a CI step and passes for `column_four`.

## What to Change

### 1. `.github/workflows/gate-1-game-smoke.yml`

Add `column_four` steps mirroring the existing per-game ones: `simulate --game column_four --games 1000`, `replay-check --game column_four --all`, `fixture-check --game column_four`, `rule-coverage --game column_four`, the wasm smoke (`smoke:wasm`), web build, and the browser smoke (`column-four.smoke.mjs` via `smoke:e2e`). Keep `boundary-check` and docs-link steps.

### 2. `.github/workflows/gate-2-benchmarks.yml`

Add the `column_four` benchmark lane (non-gating smoke on PRs / threshold gating per the existing lane model and `docs/adr/0002-ci-benchmark-gating-lanes.md`).

## Files to Touch

- `.github/workflows/gate-1-game-smoke.yml` (modify)
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- The tools/smoke/bench implementations (GAT5COLFOUPUB-011/012/013/015, deps).
- Status/doc updates (GAT5COLFOUPUB-018).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -c "column_four" .github/workflows/gate-1-game-smoke.yml` — the per-game steps are present.
2. Local dry-run of the gate-1 commands for `column_four` (`simulate`/`replay-check`/`fixture-check`/`rule-coverage`/`smoke:wasm`/`smoke:e2e`) all pass.
3. `bash scripts/boundary-check.sh` — still green; CI boundary step intact.

### Invariants

1. Existing games' CI steps and the boundary/docs-link gating steps are unchanged.
2. Every new CI step invokes a command that resolves against the post-013/012/015/011 tree.

## Test Plan

### New/Modified Tests

1. `None — CI-configuration ticket; verification is command-based (the invoked tools/smokes/benches are tested in their own tickets) plus a manual workflow review.`

### Commands

1. `grep -nE "column_four" .github/workflows/gate-1-game-smoke.yml .github/workflows/gate-2-benchmarks.yml`
2. `cargo run -p replay-check -- --game column_four --all && cargo run -p fixture-check -- --game column_four && cargo run -p rule-coverage -- --game column_four`
3. `bash scripts/boundary-check.sh` — the boundary check is the correct narrow gate-cleanliness surface.
