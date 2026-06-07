# GAT7DRALITCOM-015: Benchmarks, thresholds & BENCHMARKS.md

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/draughts_lite/benches/draughts_lite.rs` (native bench harness), `games/draughts_lite/benches/thresholds.json` (calibrated thresholds), `games/draughts_lite/docs/BENCHMARKS.md`, `games/draughts_lite/Cargo.toml` (bench target), `tools/bench-report/src/main.rs` (game registration).
**Deps**: 007, 012

## Problem

Gate 7's ROADMAP §9 exit requires "legal tree and bot benchmarks". This ticket adds native benchmarks for legal generation, validate/apply, view projection, replay throughput, and bot selection, with conservative thresholds calibrated from measured baselines and documented in `BENCHMARKS.md`. It follows the repository benchmark-lane discipline: PR smoke runs without hard threshold enforcement; scheduled/main gates may enforce committed thresholds.

## Assumption Reassessment (2026-06-07)

1. `games/draughts_lite/src/{rules.rs,actions.rs,replay_support.rs,bots.rs}` (GAT7DRALITCOM-005/006/007/010/012) supply the surfaces benchmarked. `games/directional_flip/benches/directional_flip.rs` + `benches/thresholds.json` are the structural precedents, and `tools/bench-report/src/main.rs` is the report tool that recognizes per-game benches. The bench target is declared in the crate `Cargo.toml`.
2. The benchmark set is fixed by spec §R19 (standard setup; initial legal tree; midgame no-capture tree; midgame mandatory-capture tree; capture-rich multi-jump generation; validate/apply quiet/single-capture/multi-jump; public view projection; replay-check throughput; Level 0 selection; Level 1 selection) and the lane discipline of `docs/adr/0002-ci-benchmark-gating-lanes.md` and `docs/adr/0003-ci-calibrated-benchmark-thresholds.md`.
3. Cross-artifact boundary under audit: the bench names are consumed by `tools/bench-report` (recognition) and the CI Gate 2 benchmark workflow (GAT7DRALITCOM-020); names must be stable and documented in `BENCHMARKS.md`. `docs/ROADMAP.md` §9 forbids "search without benchmarks" — the bot benches double as the no-search evidence.
4. FOUNDATIONS §6/§11 motivate this ticket: restate before coding — benchmark reports include version, command, environment, and thresholds (Gate 2 contract), and thresholds are calibrated from real measurements, not invented aggressively (spec §R19 "Threshold calibration").

## Architecture Check

1. Calibrating thresholds from measured baselines + keeping PR smoke non-enforcing (per ADR-0002/0003) avoids turning CI into "runner roulette" while still catching severe regressions on the scheduled lane.
2. No backwards-compatibility shims; new bench target + thresholds.
3. `engine-core` stays noun-free (§3); benches are game-local. No bot search is introduced (§8) — the Level 1 bench measures the bounded heuristic policy.

## Verification Layers

1. Benches compile & run -> `cargo bench -p draughts_lite -- --test` (or a smoke subset): each named bench runs.
2. Threshold shape -> `tools/bench-report` recognizes `draughts_lite` and reads `thresholds.json`.
3. Documentation -> manual review: `BENCHMARKS.md` lists each bench name, rationale, and the calibration approach (conservative, measured baselines).
4. Lane discipline -> FOUNDATIONS/ADR alignment check: PR smoke is non-enforcing; scheduled gate may enforce (ADR-0002/0003).

## What to Change

### 1. Benches & thresholds

In `benches/draughts_lite.rs`, add the §R19 benches with stable names; add `benches/thresholds.json` with conservative measured baselines; declare the bench target in `Cargo.toml`.

### 2. BENCHMARKS.md

Author `docs/BENCHMARKS.md` from `templates/GAME-BENCHMARKS.md`: list each bench name, its rationale, and the threshold-calibration approach.

## Files to Touch

- `games/draughts_lite/benches/draughts_lite.rs` (new)
- `games/draughts_lite/benches/thresholds.json` (new)
- `games/draughts_lite/docs/BENCHMARKS.md` (new)
- `games/draughts_lite/Cargo.toml` (modify — declare the bench target)

## Out of Scope

- The CI Gate 2 workflow wiring (GAT7DRALITCOM-020).
- Bot policy implementation (GAT7DRALITCOM-012; benchmarked here).
- WASM/web perf smoke (GAT7DRALITCOM-016/019).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p draughts_lite -- --test` — benches compile and run a smoke pass.
2. `cargo run -p bench-report -- --game draughts_lite` (or the tool's equivalent) — recognizes the game and reads thresholds.

### Invariants

1. Benchmark names are stable and documented; thresholds are calibrated from measured baselines (FOUNDATIONS §6; spec §R19; ADR-0003).
2. PR bench smoke does not rely on unstable runner-specific hard thresholds (ADR-0002).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/benches/draughts_lite.rs` — the §R19 bench set.
2. `games/draughts_lite/benches/thresholds.json` — calibrated baselines.

### Commands

1. `cargo bench -p draughts_lite -- --test`
2. `cargo bench -p draughts_lite legal_actions` (smoke a single bench, mirroring the `race_to_n` smoke convention)
3. A bench smoke run (`-- --test`) is the correct PR-lane boundary; full threshold enforcement is the scheduled Gate 2 lane (GAT7DRALITCOM-020), per ADR-0002.

## Outcome

- Added a native `draughts_lite` bench target with stable §R19 operation names for standard setup, initial/no-capture/capture/multi-jump legal trees, quiet/single-capture/multi-jump validate/apply, public view generation, replay throughput, and Level 0/Level 1 bot selection.
- Added `games/draughts_lite/benches/thresholds.json` with non-blocking smoke floors and `games/draughts_lite/docs/BENCHMARKS.md` with the operation list, threshold posture, first local full measurements, and legal-actions smoke measurements.
- Registered `draughts_lite` in `tools/bench-report` and pinned that registration with a focused unit test so `--game draughts_lite` resolves the committed threshold file.

Verification passed:

1. `cargo bench -p draughts_lite -- --test`
2. `cargo bench -p draughts_lite legal_actions`
3. `cargo run -p bench-report -- --game draughts_lite --input /tmp/draughts_lite_bench_015.txt`
4. `cargo test -p bench-report`
5. `cargo fmt --all --check`
