# BENCICAL-001: Harden the Gate 2 benchmark gate to run all games and aggregate failures

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — modifies `.github/workflows/gate-2-benchmarks.yml` only; no crate, schema, trace, or game code changes
**Deps**: ADR 0003 (`docs/adr/0003-ci-calibrated-benchmark-thresholds.md`) accepted

## Problem

The Gate 2 `bench-gate` job (the scheduled / manual / `main`-push lane) runs each
game's `cargo bench` + `bench-report` sequentially under `set -euo pipefail`. When
`race_to_n` breaches a floor, `bench-report` exits non-zero and the step aborts
before `three_marks`, `column_four`, or `directional_flip` ever run. As a result
the gate has never produced `ubuntu-latest` numbers for games 2-4, so they cannot
be recalibrated (`BENCICAL-002` depends on this evidence). The gate must run every
game and aggregate failures so one CI run reports all four games' numbers while
still hard-failing if any game breaches its floor.

## Assumption Reassessment (2026-06-07)

1. The gate logic lives in `.github/workflows/gate-2-benchmarks.yml`, job
   `bench-gate` (`if: github.event_name != 'pull_request'`), step "Run benchmarks
   and enforce thresholds". The run block is `set -euo pipefail` followed by four
   `cargo bench -p <game> | tee /tmp/<game>-benchmark-report.txt` +
   `cargo run -p bench-report -- --input ... --thresholds games/<game>/benches/thresholds.json`
   pairs. Verified by reading the workflow file.
2. `tools/bench-report/src/main.rs` exits non-zero on the first failing operation
   within a single game's report and prints each failing operation
   (`operation <name> below threshold`). It enforces one game at a time via
   `--input` + `--thresholds`; it has no multi-game mode. Verified by reading
   `tools/bench-report/src/main.rs` (`validate_report`, the `--thresholds`/
   `--input` arg parsing, and the `RegisteredGame` table). The aggregation must
   therefore live in the shell step, not in `bench-report`.
3. Cross-artifact boundary under audit: the CI workflow YAML <-> `bench-report`
   CLI contract (`--input <report>`, `--thresholds <thresholds.json>`, non-zero
   exit on breach). This ticket changes only how the workflow sequences and
   aggregates `bench-report` invocations; the CLI contract is unchanged.
4. ADR 0003 §Decision requires the `main`/scheduled/manual lane to "run every
   game's benchmark and `bench-report`, aggregate all failures, and fail non-zero
   if any game breaches a floor … MUST NOT abort at the first failing game."
   This ticket implements exactly that orchestration change and nothing else.
5. The four threshold files exist
   (`games/{race_to_n,three_marks,column_four,directional_flip}/benches/thresholds.json`);
   `directional_flip` floors are all `1` (non-blocking). Verified by listing the
   files. No threshold value is changed in this ticket.
6. Adjacent contradiction classification: the failing threshold *values* are a
   separate concern handled by `BENCICAL-002` (recalibration). This ticket
   deliberately leaves values untouched, so its CI run is expected to still fail
   — its purpose is to make that failure report all four games at once.

## Architecture Check

1. Aggregating in the workflow shell (collect each game's `bench-report` exit
   status, continue on failure, fail the step at the end if any failed) is
   cleaner than the alternatives: it keeps `bench-report` single-game and
   side-effect-free, needs no schema or Rust change, and matches how the PR smoke
   lane already lists independent per-game steps. Adding a multi-game mode to
   `bench-report` would widen the CLI contract for no benefit.
2. No backwards-compatibility aliasing or shims are introduced; the step body is
   replaced, not duplicated behind a flag.
3. `engine-core` is untouched (§3 noun-freedom unaffected); no `game-stdlib`
   change (§4). This is CI orchestration only.

## Verification Layers

1. Gate runs all four games even when an early game fails -> CI run inspection
   (`workflow_dispatch` run shows `race_to_n`, `three_marks`, `column_four`, and
   `directional_flip` benchmark + `bench-report` output in one job).
2. Gate still hard-fails when any game breaches a floor -> CI run inspection (the
   dispatch run exits non-zero while thresholds remain at current values).
3. No threshold value changed -> codebase grep-proof (`git diff` touches only
   `.github/workflows/gate-2-benchmarks.yml`; no `games/*/benches/thresholds.json`
   in the diff).
4. FOUNDATIONS / doctrine alignment: gate remains hard-failing per TESTING §15/§16
   and ADR 0003 -> FOUNDATIONS alignment check (the step still exits non-zero on
   breach).

## What to Change

### 1. Rewrite the `bench-gate` run step to aggregate across games

Replace the `set -euo pipefail` sequential block so that:

- each game runs `cargo bench -p <game> | tee /tmp/<game>-benchmark-report.txt`
  then `cargo run -p bench-report -- --input /tmp/<game>-benchmark-report.txt
  --thresholds games/<game>/benches/thresholds.json`;
- a `bench-report` non-zero exit for one game is recorded (e.g. a `failed`
  accumulator) instead of aborting the whole step;
- after all four games run, the step exits non-zero if any game failed and exits
  zero only if all passed.

Keep `set -u` semantics for undefined-variable safety, but do not let a single
`bench-report` failure short-circuit the remaining games (drop the fail-fast on
the per-game `bench-report` line; keep failures captured). The benchmarks
themselves (`cargo bench`) compiling/running is still a hard prerequisite — a
`cargo bench` build failure should still fail the step.

### 2. Keep the PR smoke job unchanged

The `bench-smoke` job (`if: github.event_name == 'pull_request'`) is unchanged;
ADR 0002's lane split stands.

## Files to Touch

- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- Changing any threshold value in `games/*/benches/thresholds.json` (that is
  `BENCICAL-002`).
- Updating `BENCHMARKS.md` files (that is `BENCICAL-002`).
- Any change to `tools/bench-report` (the CLI contract stays single-game).
- The PR smoke lane and the ADR 0002 lane split.

## Acceptance Criteria

### Tests That Must Pass

1. A `workflow_dispatch` run of `Gate 2 benchmarks` on the ticket branch executes
   the benchmark + `bench-report` for all four games (`race_to_n`, `three_marks`,
   `column_four`, `directional_flip`) in the `bench-gate` job log, rather than
   aborting at `race_to_n`.
2. With thresholds still at current values, that same run exits non-zero (the gate
   still hard-fails on a breach).
3. `git diff --name-only` for the ticket shows only
   `.github/workflows/gate-2-benchmarks.yml`.

### Invariants

1. The `main`/scheduled/manual benchmark lane reports every game's numbers in a
   single run and hard-fails if any game breaches a floor.
2. No threshold value or benchmark policy data changes in this ticket.

## Test Plan

### New/Modified Tests

1. `None — CI-orchestration-only ticket; verification is a `workflow_dispatch`
   run inspection plus a `git diff` name check. No Rust test or game code changes,
   and the per-game benchmark suites named in Assumption Reassessment already
   cover correctness.`

### Commands

1. `gh workflow run "Gate 2 benchmarks" --ref <ticket-branch>` then
   `gh run view <id> --log` — confirm all four games run and the job exits
   non-zero with current thresholds.
2. `git diff --name-only main -- .github/ games/` — confirm only the workflow file
   changed.
3. A narrower local command is not the correct boundary: the failure is
   environment-specific to `ubuntu-latest`, so verification must run on a GitHub
   runner via `workflow_dispatch`, not on the local WSL2 host.

## Outcome

Completed: 2026-06-07

What changed:

- Reworked `.github/workflows/gate-2-benchmarks.yml` `bench-gate` so each game
  still runs `cargo bench` followed by `bench-report`, but individual
  `bench-report` failures set a `failed` accumulator instead of aborting the
  step.
- Preserved hard failure for `cargo bench` build/run failures via
  `set -euo pipefail`; only threshold breaches are aggregated.
- Left the pull-request `bench-smoke` job and all threshold files unchanged.

Deviations from original plan:

- None.

Verification results:

- `sed -n '46,75p' .github/workflows/gate-2-benchmarks.yml | sed 's/^          //' | bash -n`
  passed.
- `git diff --check` passed.
- `git diff --name-only -- .github/ games/ docs/ tickets/ archive/` showed only
  `.github/workflows/gate-2-benchmarks.yml` before archival.
- `gh workflow run "Gate 2 benchmarks" --ref bencical-ci-calibration` dispatched
  run `27087214359`; `gh run view 27087214359 --log` showed `race_to_n`,
  `three_marks`, `column_four`, and `directional_flip` all ran in the
  `bench-gate` step, then the job exited non-zero with "One or more benchmark
  threshold checks failed."
