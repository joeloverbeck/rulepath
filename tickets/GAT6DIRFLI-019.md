# GAT6DIRFLI-019: CI workflow integration

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None (CI configuration — `.github/workflows/*.yml`). No Rust/engine surface.
**Deps**: 014, 015, 016, 018

## Problem

Gate 6 evidence must run in CI: Rust build/test, game docs/checks, trace/replay/fixture/rule-coverage checks, simulation smoke, benchmarks/report, web build, WASM smoke/load, web e2e smoke, no-leak/a11y smoke, and boundary checks (spec §13.2). This ticket extends the existing CI lanes to include `directional_flip` — without broad CI redesign and honoring the ADR-0002 benchmark gating posture.

## Assumption Reassessment (2026-06-07)

1. The workflow files exist: `.github/workflows/gate-0-hygiene.yml`, `gate-1-game-smoke.yml`, `gate-2-benchmarks.yml` (confirmed). The per-game smoke lane (`gate-1-game-smoke.yml`) and benchmark lane (`gate-2-benchmarks.yml`) are where `column_four` was wired (GAT5COLFOUPUB-016 precedent); the directional-flip steps mirror that.
2. Spec §13.2 (CI evidence list + "do not create broad CI redesign" + "do not hard-fail on speculative thresholds") is authoritative. `docs/adr/0002-ci-benchmark-gating-lanes.md` governs benchmark gating.
3. Cross-artifact boundary under audit: the CI lanes invoke the tools (GAT6DIRFLI-016), benches (014), wasm smoke (015), and web e2e/a11y smoke (018). Each invoked command must already accept `--game directional_flip` (so this ticket depends on those). Confirm each referenced npm script / cargo command resolves before wiring.
4. The change is additive to existing lanes (new steps/matrix entries), introducing no new workflow file unless the existing lane structure requires it; benchmark gating stays non-blocking per measured baseline (ADR-0002), upholding the spec's anti-fake-threshold stance (FOUNDATIONS §6).

## Architecture Check

1. Extending existing lanes (rather than authoring a new workflow) matches the spec's "no broad CI redesign" constraint and keeps Gate 6 evidence in the same gates the prior games use.
2. No backwards-compatibility shims; additive CI steps.
3. `engine-core` untouched; CI config only (§3).

## Verification Layers

1. CI lane coverage -> manual review against spec §13.2: each required evidence category (build/test, trace, replay, fixture, rule-coverage, simulation, bench, web build, wasm smoke, web e2e, no-leak/a11y, boundary) has a directional-flip step.
2. Command resolution -> codebase grep-proof / dry-run: every command the lane invokes resolves (the tools accept `--game directional_flip` from GAT6DIRFLI-016; npm smoke scripts exist).
3. Benchmark gating posture -> FOUNDATIONS alignment check (ADR-0002): no hard-fail on speculative thresholds.

## What to Change

### 1. Game-smoke lane

In `.github/workflows/gate-1-game-smoke.yml`, add directional-flip steps: simulate, replay-check `--all`, fixture-check, rule-coverage, web build, wasm smoke, web e2e smoke, no-leak/a11y smoke, boundary check (mirroring the column_four steps).

### 2. Benchmark lane

In `.github/workflows/gate-2-benchmarks.yml`, add the directional-flip benchmark/report step with the ADR-0002 non-blocking baseline posture.

## Files to Touch

- `.github/workflows/gate-1-game-smoke.yml` (modify)
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- The tools/benches/smokes themselves (GAT6DIRFLI-014/015/016/018) — this ticket only invokes them in CI.
- Status/index flips and public exposure (GAT6DIRFLI-021).

## Acceptance Criteria

### Tests That Must Pass

1. The directional-flip steps appear in the game-smoke and benchmark lanes and invoke resolvable commands.
2. Locally re-runnable equivalents pass: `cargo test --workspace`, `cargo run -p replay-check -- --game directional_flip --all`, `npm --prefix apps/web run smoke:wasm`, `npm --prefix apps/web run smoke:ui`, `bash scripts/boundary-check.sh`.

### Invariants

1. CI extends existing lanes additively; no broad redesign (spec §13.2).
2. Benchmark gating does not hard-fail on speculative thresholds (FOUNDATIONS §6, ADR-0002).

## Test Plan

### New/Modified Tests

1. `None — CI-configuration ticket; verification is the lane definitions plus the locally re-runnable command equivalents above.`

### Commands

1. `cargo test --workspace && cargo run -p simulate -- --game directional_flip --games 1000`
2. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && bash scripts/boundary-check.sh`
3. Re-running the lane's underlying commands locally is the correct boundary; the workflow YAML itself is validated by review + the next CI run.
