# GAT101PLATRI-015: Native benchmarks, thresholds, BENCHMARKS.md, and gate-2 CI

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — new `games/plain_tricks/benches/plain_tricks.rs`, `games/plain_tricks/benches/thresholds.json`, `games/plain_tricks/docs/BENCHMARKS.md`; modifies `tools/bench-report/src/main.rs`, `.github/workflows/gate-2-benchmarks.yml`. No `engine-core`/`game-stdlib` change.
**Deps**: GAT101PLATRI-013

## Problem

FOUNDATIONS §6 and ROADMAP Gate 10 ("native benchmarks exist") require a native benchmark lane. This ticket adds the Criterion benches, provisional thresholds, `BENCHMARKS.md`, a `bench-report` registration arm, and gate-2 CI wiring per the ADR 0002/0003 lane discipline.

## Assumption Reassessment (2026-06-09)

1. `games/poker_lite/benches/` and `tools/bench-report/src/main.rs` (`resolve_game()` registers the five threshold games) exist as templates; `.github/workflows/gate-2-benchmarks.yml` has non-gating PR smoke + gating lanes. The poker_lite bench file is `benches/poker_lite.rs`, so `plain_tricks` uses `benches/plain_tricks.rs` (game-specific name, not a reused name).
2. Spec §4/§5 item 10, §7, and appendix E fix the bench operations (setup+shuffle+deal, legal-tree generation per phase, validate/apply, trick resolution, observer/seat projection, public export/import, full random-legal playout, Level 2 playout) and the provisional native floor (≥ 2,000 completed random-legal matches/sec), with a named calibration follow-up.
3. Shared boundary under audit: the `bench-report` threshold-file contract and the ADR 0002/0003 CI lane structure. `bench-report` registers games-with-threshold-files; adding `plain_tricks` requires both `thresholds.json` and a `resolve_game()` arm.
4. FOUNDATIONS §6 (benchmarks required) and ADR 0002 (lanes) / ADR 0003 (calibrated CI floors) are under audit. ADR 0005 (variance-aware floors) is **Proposed, not accepted** — implementation MUST NOT claim it accepted; apply its discipline only if it is accepted by then.

## Architecture Check

1. A game-specific `benches/plain_tricks.rs` + `thresholds.json` + `bench-report` arm (the established pattern) keeps benchmark gating uniform; provisional thresholds with a named calibration follow-up avoid premature hard floors.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; no `game-stdlib` change.

## Verification Layers

1. Benches compile and run for all named operations -> benchmark check (`cargo bench -p plain_tricks`).
2. `bench-report` validates the threshold file -> `bench-report` CLI run against the bench output.
3. Native floor target recorded -> manual review of `BENCHMARKS.md` (≥ 2,000 matches/sec provisional; CI floors per ADR 0002/0003; no ADR-0005-accepted claim).
4. gate-2 CI wiring -> manual review of `gate-2-benchmarks.yml`.

## What to Change

### 1. `games/plain_tricks/benches/plain_tricks.rs` + `thresholds.json`

Criterion benches for the appendix-E operations; provisional thresholds per ADR 0002/0003 lanes.

### 2. `games/plain_tricks/docs/BENCHMARKS.md`

Record the native target (≥ 2,000 random-legal matches/sec), the benched operations, CI floor strategy per ADR 0002/0003, and a named calibration follow-up. Do not claim ADR 0005 accepted.

### 3. `tools/bench-report/src/main.rs` + `.github/workflows/gate-2-benchmarks.yml`

Add a `plain_tricks` `resolve_game()` arm and gate-2 bench smoke (non-gating PR) + gating-lane steps mirroring an existing threshold game.

## Files to Touch

- `games/plain_tricks/benches/plain_tricks.rs` (new)
- `games/plain_tricks/benches/thresholds.json` (new)
- `games/plain_tricks/docs/BENCHMARKS.md` (new)
- `tools/bench-report/src/main.rs` (modify)
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- Native tool registration / RULE-COVERAGE / gate-1 CI (GAT101PLATRI-014).
- WASM / web / e2e (GAT101PLATRI-016/017/018).
- Threshold calibration itself (named follow-up, not blocking this gate).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p plain_tricks` runs all benched operations.
2. `cargo run -p bench-report` validates `games/plain_tricks/benches/thresholds.json` against the bench output.
3. `cargo test --workspace` passes (benches compile).

### Invariants

1. `BENCHMARKS.md` records a native floor and CI-floor strategy without claiming ADR 0005 is accepted (FOUNDATIONS §11; ADR honesty).
2. Benchmark gating follows ADR 0002 lanes and ADR 0003 calibrated floors.

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/benches/plain_tricks.rs` + `thresholds.json` — the bench lane + thresholds.
2. `games/plain_tricks/docs/BENCHMARKS.md` — target + calibration follow-up.

### Commands

1. `cargo bench -p plain_tricks`
2. `cargo bench -p plain_tricks | tee /tmp/plain_tricks-bench.txt && cargo run -p bench-report -- --input /tmp/plain_tricks-bench.txt --thresholds games/plain_tricks/benches/thresholds.json`
3. The bench + bench-report run is the correct boundary; CLI flag names are confirmed against `tools/bench-report` at implementation time.
