# GAT7DRALITCOM-020: CI workflow integration (gate-0/1/2)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — CI workflow configuration only (`.github/workflows/gate-1-game-smoke.yml`, `.github/workflows/gate-2-benchmarks.yml`; `gate-0-hygiene.yml` already covers the workspace).
**Deps**: 015, 016, 017, 019

## Problem

CI must exercise Draughts Lite without weakening prior gates. This ticket adds the per-game steps to the Gate 1 game-smoke workflow (simulate / replay-check / fixture-check / rule-coverage, plus the browser smoke now includes Draughts Lite) and the Gate 2 benchmark workflow (Draughts Lite bench), mirroring the existing per-game step pattern.

## Assumption Reassessment (2026-06-07)

1. `.github/workflows/gate-1-game-smoke.yml` enumerates per-game steps (verified: `race_to_n`/`three_marks`/`column_four`/`directional_flip` each have simulate/replay-check/fixture-check/rule-coverage steps, plus a "Browser shell + a11y/no-leak E2E" step). `.github/workflows/gate-2-benchmarks.yml` runs the benchmark lane; `gate-0-hygiene.yml` runs `cargo fmt/clippy/build/test --workspace` (auto-covers the new crate). `draughts` is not yet referenced in CI (verified).
2. The CI contract is fixed by spec §R23 "CI Gate 0/1/2 workflows pass after updating game lists" and the tool/bench surfaces from GAT7DRALITCOM-017 (tools) and 015 (benches); the browser smoke (`smoke:e2e`) gains Draughts Lite in GAT7DRALITCOM-019.
3. Cross-artifact boundary under audit: the workflows invoke the registered tools (017), the benches (015), and the web smoke (019); this ticket only wires steps and depends on those surfaces existing. It must not weaken existing gates' steps (spec §R22 "CI must pass without weakening prior gates").
4. FOUNDATIONS §6 motivates this ticket: restate before editing — every official game carries CLI simulation, replay, fixture, rule-coverage, benchmark, and UI-smoke evidence; CI is where that evidence is enforced per gate.

## Architecture Check

1. Adding parallel per-game steps (the established workflow pattern) is the minimal, lowest-risk wiring; it leaves existing gates' steps intact.
2. No backwards-compatibility shims; additive workflow steps.
3. `engine-core` is untouched (§3); CI configuration only.

## Verification Layers

1. Gate 1 steps -> workflow review + local dry-run: the simulate/replay-check/fixture-check/rule-coverage commands for `draughts_lite` match the registered tool invocations (017) and run locally.
2. Gate 2 bench step -> workflow review + local dry-run: the Draughts Lite bench step matches the bench target (015).
3. Browser smoke -> `npm --prefix apps/web run smoke:e2e` includes Draughts Lite (019).
4. No prior-gate weakening -> workflow diff review: existing per-game steps are unchanged.

## What to Change

### 1. Gate 1 game-smoke

Add `draughts_lite` simulate / replay-check / fixture-check / rule-coverage steps mirroring the `directional_flip` steps; confirm the browser E2E step covers Draughts Lite (via `smoke:e2e`, wired in 019).

### 2. Gate 2 benchmarks

Add the Draughts Lite benchmark step on the benchmark lane, following the existing per-game bench pattern and the ADR-0002/0003 lane discipline.

## Files to Touch

- `.github/workflows/gate-1-game-smoke.yml` (modify)
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- Tool/bench/smoke implementation (GAT7DRALITCOM-015/016/017/019; this ticket only invokes them).
- `gate-0-hygiene.yml` (already covers the workspace via `--workspace`; no edit needed unless a game list is enumerated there).
- The spec/index `Done`-flip (GAT7DRALITCOM-022).

## Acceptance Criteria

### Tests That Must Pass

1. Local dry-run of each added command: `cargo run -p simulate -- --game draughts_lite --games 1000`, `cargo run -p replay-check -- --game draughts_lite --all`, `cargo run -p fixture-check -- --game draughts_lite`, `cargo run -p rule-coverage -- --game draughts_lite` all pass.
2. `npm --prefix apps/web run smoke:e2e` passes (includes Draughts Lite).

### Invariants

1. CI exercises Draughts Lite across Gate 0/1/2 without weakening prior gates (FOUNDATIONS §6; spec §R22/§R23).
2. Added steps mirror the registered tool/bench/smoke surfaces (no drift from 015/016/017/019).

## Test Plan

### New/Modified Tests

1. `None — CI workflow configuration; verification is dry-running the invoked commands locally (they are exercised by GAT7DRALITCOM-013/014/015/017/019).`

### Commands

1. `cargo run -p simulate -- --game draughts_lite --games 1000 && cargo run -p replay-check -- --game draughts_lite --all`
2. `cargo run -p fixture-check -- --game draughts_lite && cargo run -p rule-coverage -- --game draughts_lite && npm --prefix apps/web run smoke:e2e`
3. Local dry-runs of the workflow commands are the correct boundary; the workflow YAML itself is validated by the CI run on the branch.
