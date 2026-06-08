# BENCICAL-003: Recalibrate three_marks and column_four to variance-aware CI floors

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — benchmark policy data (`games/three_marks/benches/thresholds.json`, `games/column_four/benches/thresholds.json`), audit `games/race_to_n/benches/thresholds.json`, and per-game docs (`games/three_marks/docs/BENCHMARKS.md`, `games/column_four/docs/BENCHMARKS.md`). No Rust code, schema, or workflow change.
**Deps**: ADR 0005 (`docs/adr/0005-variance-aware-ci-benchmark-floors.md`) accepted

## Problem

The `Gate 2 benchmarks / Benchmark threshold gate` job hard-fails on every
`main`-push merge (runs `27101213584`, `27111487150`, `27114668009`).
`three_marks` and `column_four` breach floors that ADR 0003 calibrated below a
single, unrepresentatively fast CI sample (`BENCICAL-001` run `27087214359`),
and some operations were never CI-recalibrated at all. ADR 0005 establishes the
variance-aware calibration doctrine: floors must sit ≥15% below the minimum
observed across at least three representative CI runs. This ticket commits the
recalibrated floors and updates the per-game benchmark notes so `main` stops
being permanently red without hiding any performance figure.

## Assumption Reassessment (2026-06-08)

1. The breaching operations and their `rationale_class` were confirmed by reading
   `games/three_marks/benches/thresholds.json` and
   `games/column_four/benches/thresholds.json`: `column_four` `apply_action`
   (threshold 200000, `measured_baseline`) and `three_marks`
   `public_view_generation` (200000, `measured_baseline`) /
   `level1_bot_decision` (35000, `measured_baseline`) were never CI-recalibrated;
   `column_four` `random_playout` (9000), `replay_throughput` (3200),
   `replay_step_projection` (33000), `level2_bot_decision` (3000) and
   `three_marks` `random_playout` (35000) carry `conservative_ci_floor` set below
   the single `BENCICAL-001` sample.
2. ADR 0005 (`docs/adr/0005-variance-aware-ci-benchmark-floors.md`) is the
   governing doctrine; its Decision section mandates floor ≤ `0.85 × min_observed`
   across ≥3 representative runs and forbids using `BENCICAL-001` (`27087214359`)
   as the calibration minimum. `docs/TESTING-REPLAY-BENCHMARKING.md` §15 forbids
   lowering a threshold "only to make CI green" and requires the native baseline
   stay visible — satisfied here because BENCHMARKS.md retains native targets.
3. Shared boundary under audit: the `thresholds.json` schema consumed by
   `tools/bench-report`. This ticket changes only `threshold` numeric values and
   `rationale`/`rationale_class` strings; `schema_version` stays 1 and no field is
   added or removed, so `bench-report`'s parser is untouched.
4. FOUNDATIONS principle restated: static data stays content/parameters, never
   behavior. `thresholds.json` remains typed benchmark policy data; no selector,
   condition, or trigger is introduced. The §11 "do not hide performance"
   expectation is met by recording native targets and CI minima in BENCHMARKS.md.
5. Mismatch + correction: ADR 0003's BENCHMARKS.md prose for `column_four` and
   `three_marks` claims the committed threshold is "the enforced CI floor below
   the stable measurement"; ADR 0005 supersedes "stable measurement" with
   "minimum across ≥3 representative runs." The BENCHMARKS.md prose and tables
   MUST be updated to cite ADR 0005 and the new evidence, not silently left
   pointing at ADR 0003's single-sample story.

## Architecture Check

1. Variance-aware floors derived from multiple representative runs are more
   robust than ADR 0003's single-sample floors: the three steady-state runs are
   tightly clustered (`column_four` `level2_bot_decision`: 1438 / 1450 / 1370),
   so a ≥15% margin below their minimum absorbs the residual fleet variance that
   one sample cannot.
2. No backwards-compatibility shim: floors are replaced in place; no dual-value
   or legacy threshold field is introduced (regression-relative gating is a
   separate future ADR, not aliased here).
3. `engine-core` is untouched (no Rust change). No `game-stdlib` primitive is
   introduced. Only typed per-game policy data and per-game docs change.

## Verification Layers

1. Floor ≤ `0.85 × min_observed` for every recalibrated operation -> arithmetic
   check against the extracted minima table committed in BENCHMARKS.md.
2. Gate passes on representative CI evidence -> `cargo bench` + `bench-report`
   per game locally, and the `main`-push / `workflow_dispatch` Gate 2 lane green.
3. No schema drift -> `thresholds.json` `schema_version` still 1, diff shows only
   `threshold` / `rationale` / `rationale_class` value changes (grep-proof).
4. Native targets remain visible -> each game's BENCHMARKS.md retains the native
   target/baseline column alongside the new CI floor (manual review).

## What to Change

### 1. Extract representative CI minima

For each blocking operation in `three_marks` and `column_four`, extract the
`current_value` from the full benchmark JSON (`BEGIN_…_BENCHMARK_JSON` …
`END_…_BENCHMARK_JSON`) of the three representative `main`-push runs
`27101213584`, `27111487150`, `27114668009` (via `gh run view <id> --log`). Take
the per-operation minimum. Do NOT include `BENCICAL-001` run `27087214359` (the
unrepresentative fast outlier). If fewer than three runs carry an operation's
value, trigger additional `workflow_dispatch` Gate 2 runs until ≥3 samples exist.

Illustrative minima and resulting floors (`floor = round_down(0.85 × min)`) — the
implementer MUST confirm each min against the freshly extracted JSON:

| Game / operation | min observed | suggested floor |
|---|---:|---:|
| column_four `apply_action` | ~146,481 | 120,000 |
| column_four `replay_step_projection` | ~30,494 | 25,000 |
| column_four `replay_throughput` | ~2,879 | 2,400 |
| column_four `random_playout` | ~5,939 | 5,000 |
| column_four `level2_bot_decision` | ~1,370 | 1,100 |
| three_marks `public_view_generation` | ~196,178 | 165,000 |
| three_marks `random_playout` | ~34,134 | 29,000 |
| three_marks `level1_bot_decision` | ~34,966 | 29,000 |

### 2. Recalibrate `thresholds.json`

Update the breaching `threshold` values in
`games/three_marks/benches/thresholds.json` and
`games/column_four/benches/thresholds.json` to the derived floors. Set
`rationale_class` to `conservative_ci_floor` for every recalibrated operation
(including the previously `measured_baseline` ones). Rewrite each `rationale` to
cite ADR 0005, the observed minimum, and the three evidence runs — e.g.
"Variance-aware floor 15% below the minimum 1,370.21 decisions/sec observed
across runs 27101213584/27111487150/27114668009; native baseline in BENCHMARKS.md
per ADR 0005." Leave passing operations unchanged.

### 3. Audit race_to_n margin

Extract `race_to_n` operation values from the same three runs. For any operation
whose committed floor is within 15% of the observed minimum, widen it to
`0.85 × min` and update its rationale to cite ADR 0005. If all `race_to_n` floors
already clear the 15% margin, make no value change and record the audit result in
its BENCHMARKS.md.

### 4. Update BENCHMARKS.md notes

In `games/three_marks/docs/BENCHMARKS.md` and
`games/column_four/docs/BENCHMARKS.md`, update the "CI-calibrated floors" table
(and add rows for the newly-recalibrated `measured_baseline` operations) to show
three columns: native target/baseline, observed CI minimum across the three runs,
and the committed variance-aware floor. Repoint the ADR-0003 prose references to
ADR 0005 and note the single-sample calibration is superseded. Bump
"Last updated" to 2026-06-08.

## Files to Touch

- `games/three_marks/benches/thresholds.json` (modify)
- `games/column_four/benches/thresholds.json` (modify)
- `games/race_to_n/benches/thresholds.json` (modify — only if audit finds margin under 15%)
- `games/three_marks/docs/BENCHMARKS.md` (modify)
- `games/column_four/docs/BENCHMARKS.md` (modify)
- `games/race_to_n/docs/BENCHMARKS.md` (modify — record audit result)

## Out of Scope

- Regression-relative gating / stored-baseline mechanism — that is ADR 0005's
  committed next step in a separate future ADR, not this ticket.
- `.github/workflows/gate-2-benchmarks.yml` — unchanged; the lane already runs all
  games and aggregates failures.
- `thresholds.json` schema changes (no new field, no `schema_version` bump).
- `directional_flip`, `draughts_lite`, `high_card_duel` (non-blocking `1` floors).
- `docs/TESTING-REPLAY-BENCHMARKING.md` edits — handled by BENCICAL-004.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p three_marks` then `cargo run -p bench-report -- --input <three_marks.json> --thresholds games/three_marks/benches/thresholds.json` reports all operations passing on a representative `ubuntu-latest` run.
2. `cargo bench -p column_four` then `cargo run -p bench-report -- --input <column_four.json> --thresholds games/column_four/benches/thresholds.json` reports all operations passing on a representative `ubuntu-latest` run.
3. A `workflow_dispatch` run of `Gate 2 benchmarks` completes green (all six games pass the aggregating gate).

### Invariants

1. Every recalibrated `threshold` ≤ `0.85 × min_observed` across the three named representative runs (ADR 0005 doctrine).
2. `thresholds.json` `schema_version` remains 1 and no field is added/removed; only `threshold`/`rationale`/`rationale_class` values differ.
3. Each game's BENCHMARKS.md still records the native target/baseline alongside the CI floor — no performance figure removed.

## Test Plan

### New/Modified Tests

1. `None — benchmark-policy-data + documentation ticket; verification is command-based via cargo bench + bench-report and the Gate 2 lane, as named in Assumption Reassessment.`

### Commands

1. `cargo bench -p column_four | tee /tmp/c4.txt && cargo run -p bench-report -- --input /tmp/c4.txt --thresholds games/column_four/benches/thresholds.json`
2. `cargo bench -p three_marks | tee /tmp/tm.txt && cargo run -p bench-report -- --input /tmp/tm.txt --thresholds games/three_marks/benches/thresholds.json`
3. Trigger the full lane via `gh workflow run "Gate 2 benchmarks"` (or push to `main`) and confirm the `Benchmark threshold gate` job is green — this is the only environment that reproduces the heterogeneous shared-runner behavior the floors are calibrated against.
