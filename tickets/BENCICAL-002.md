# BENCICAL-002: Recalibrate Gate 2 benchmark thresholds to CI-runner floors

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — modifies `games/{race_to_n,three_marks,column_four}/benches/thresholds.json`, the matching benchmark harness report annotations, the matching `games/*/docs/BENCHMARKS.md`, and `docs/TESTING-REPLAY-BENCHMARKING.md`; no gameplay, engine, trace, or schema-version change
**Deps**: ADR 0003 (`docs/adr/0003-ci-calibrated-benchmark-thresholds.md`) accepted; `BENCICAL-001` merged and its `workflow_dispatch` run captured (provides `ubuntu-latest` numbers for `three_marks` and `column_four`)

## Problem

The Gate 2 `main`/scheduled/manual benchmark gate is calibrated to local WSL2
baselines but runs on a shared GitHub `ubuntu-latest` runner that is consistently
~34% slower. `race_to_n` breaches four floors on CI
(`serialization_roundtrip` 192,697/200,000; `replay_throughput` 231,094/250,000;
`random_playout` 66,050/100,000; `bot_decision` 985,488/1,000,000), so `main` has
been red on every merge since Gate 3. Per ADR 0003, each `thresholds.json`
`threshold` must become a conservative floor below the stable `ubuntu-latest`
measurement, with the faster native/WSL2 baseline preserved in each game's
`BENCHMARKS.md` as a documented aspirational target. This ticket commits those
recalibrated floors and the supporting doc updates.

## Assumption Reassessment (2026-06-07)

1. The four threshold files are
   `games/{race_to_n,three_marks,column_four,directional_flip}/benches/thresholds.json`,
   each with `schema_version: 1` and a `thresholds` array of objects carrying
   `operation_name`, `unit`, `threshold`, `rationale_class`, `rationale`. Verified
   by reading all four files. `directional_flip` floors are all `1`
   (`baseline_pending_non_blocking`) and need no change — confirmed.
2. `tools/bench-report/src/main.rs` parses these fields with a hand-rolled JSON
   reader (`ThresholdSet::parse`, `Threshold`) and requires
   `operation_name`/`unit`/`threshold`/`rationale_class`/`rationale` to be present;
   `rationale_class` and `rationale` are free-form strings it prints but does not
   constrain. Verified by reading the parser. Therefore reusing
   `conservative_ci_floor` and keeping `accepted_adr` for `random_playout` needs
   no tool change, and the `schema_version` stays 1.
3. Cross-artifact boundary under audit: `thresholds.json` (benchmark policy data)
   <-> `bench-report` (consumer) <-> `BENCHMARKS.md` (human record) <->
   `TESTING-REPLAY-BENCHMARKING.md` §15/§17 (doctrine). This ticket changes
   numeric `threshold` values and `rationale` strings only; it does not change the
   schema, the parser, or the field set, so the data/Rust boundary is unchanged
   (`thresholds.json` stays typed content, not behavior — FOUNDATIONS §5).
4. FOUNDATIONS principle under audit: TESTING §15 forbids lowering a threshold
   "only to make CI green" and requires keeping a miss visible. ADR 0003 is the
   accepted supersession that authorizes recalibration to the CI environment and
   requires the native baseline to stay recorded in `BENCHMARKS.md`. This ticket
   must implement both halves: lower the gate floor AND record the native target,
   or it violates the doctrine.
5. Schema-extension check: this is an additive-free value edit. No field is added,
   removed, or renamed; `schema_version`, `rules_version`, `data_version`,
   `engine_version` are unchanged. The consumers of `thresholds.json` are
   `bench-report` and the Gate 2 workflow; both are value-agnostic.
6. Live-code correction: each native benchmark harness also carries duplicated
   threshold/report annotations in its local `THRESHOLDS` table so the human
   table and marked benchmark JSON can print `threshold`, `pass`,
   `rationale_class`, and caveat values. Those harness constants are not gameplay
   behavior and are not the enforcement authority (`bench-report` still reads
   `thresholds.json`), but leaving them stale would make the CI log contradict
   the recalibrated gate. This ticket must update only those duplicated
   benchmark-report annotations for the same recalibrated operations.
7. Evidence dependency: `race_to_n` CI numbers are already known from run
   `27086098697`. `three_marks` and `column_four` CI numbers are NOT yet known
   because the pre-`BENCICAL-001` gate aborts at `race_to_n`. This ticket MUST use
   the `ubuntu-latest` numbers from the `BENCICAL-001` `workflow_dispatch` run to
   set games-2-4 floors; deriving them from WSL2 numbers by a slowdown factor is
   not acceptable (TESTING §14 "do not claim performance without benchmark
   evidence"). If a `three_marks`/`column_four` operation passes on CI at its
   current floor, that floor is left unchanged.
8. Adjacent contradiction classification: the gate-aggregation change is a
   required prerequisite handled by `BENCICAL-001` (separate ticket), not folded
   here. `directional_flip` non-blocking floors are out of scope until it has
   stable CI measurements (a future ticket, per its `rationale_class`).

## Architecture Check

1. Re-flooring the single `threshold` value per operation (rather than adding a
   `ci_threshold` field) is the cleaner design ADR 0003 accepted: it needs no
   schema bump and no parser change, and the native baseline already has a home in
   `BENCHMARKS.md`. Every floor is set conservatively below the observed CI value
   so normal runner variance does not re-flake the gate.
2. No backwards-compatibility aliasing or shims: old values are replaced in place,
   not kept behind a flag or a second field.
3. `engine-core` is untouched (§3); no `game-stdlib` change (§4). Only benchmark
   policy data, benchmark report annotations, and docs change.

## Verification Layers

1. Recalibrated floors pass on CI -> CI run inspection (a `workflow_dispatch` /
   post-merge `bench-gate` run exits zero with all four games reported).
2. No floor is set above its observed CI value -> manual review (each changed
   `threshold` is below the `current_value` recorded in the `BENCICAL-001`
   evidence run; cite the value in the `rationale`).
3. Native baseline preserved, miss kept visible -> manual review (each touched
   `BENCHMARKS.md` records both the native WSL2 number and the CI floor; TESTING
   §15 satisfied).
4. Benchmark logs do not contradict the gate -> simulation/CLI run (`cargo bench`
   emits recalibrated threshold/pass annotations matching `thresholds.json` for
   the touched operations).
5. `thresholds.json` schema unchanged -> schema/serialization validation
   (`schema_version` stays 1; `bench-report` parses every file without error:
   `cargo run -p bench-report -- --input <report> --thresholds <file>`).
6. Doctrine alignment -> FOUNDATIONS alignment check (TESTING §15/§17 reference
   ADR 0003; gate stays hard-failing).

## What to Change

### 1. Recalibrate `race_to_n` thresholds

In `games/race_to_n/benches/thresholds.json`, lower the four breaching operations
to conservative floors below their observed CI values (from run `27086098697`):
`serialization_roundtrip` (CI 192,697), `replay_throughput` (CI 231,094),
`random_playout` (CI 66,050), `bot_decision` (CI 985,488). For the three
`measured_baseline` operations set `rationale_class` to `conservative_ci_floor`
with a rationale naming the observed CI value and the run. For `random_playout`
keep `rationale_class: accepted_adr` and re-point its rationale to ADR 0003 (the
100,000 native target stays in `BENCHMARKS.md`). Operations already passing on CI
(`legal_actions`, `apply_action`, `public_view_generation`, `effect_filtering`)
are left unchanged.

### 2. Recalibrate `three_marks` and `column_four` thresholds from CI evidence

Using the `ubuntu-latest` numbers captured by the `BENCICAL-001`
`workflow_dispatch` run, lower any breaching operation in
`games/three_marks/benches/thresholds.json` and
`games/column_four/benches/thresholds.json` to a conservative `conservative_ci_floor`
below its observed CI value, with the rationale naming that value. Leave any
operation that already passes on CI unchanged. Do not touch `directional_flip`.

### 3. Keep benchmark harness report annotations aligned

In the matching `games/*/benches/*.rs` harness files, update only the duplicated
`THRESHOLDS` entries for recalibrated operations so the emitted human table and
marked JSON report show the same threshold, pass/fail, rationale class, and
caveat as `thresholds.json`.

### 4. Update the per-game `BENCHMARKS.md`

In `games/race_to_n/docs/BENCHMARKS.md`, `games/three_marks/docs/BENCHMARKS.md`,
and `games/column_four/docs/BENCHMARKS.md`, record for each recalibrated operation
both the native WSL2 baseline (the prior `threshold` / measured native number) and
the new CI floor, plus a one-line note that the gate enforces the CI floor while
the native number is the aspirational target (ADR 0003). Keep the miss visible.

### 5. Update the testing doctrine references

In `docs/TESTING-REPLAY-BENCHMARKING.md` §15 and §17, add a reference to ADR 0003
for the CI-calibration doctrine (the committed `threshold` is the CI-runner floor;
the native baseline lives in `BENCHMARKS.md`). This extends, and does not replace,
the existing ADR 0002 lane-split reference.

## Files to Touch

- `games/race_to_n/benches/thresholds.json` (modify)
- `games/three_marks/benches/thresholds.json` (modify)
- `games/column_four/benches/thresholds.json` (modify)
- `games/race_to_n/benches/race_to_n.rs` (modify benchmark report annotations only)
- `games/three_marks/benches/three_marks.rs` (modify benchmark report annotations only)
- `games/column_four/benches/column_four.rs` (modify benchmark report annotations only)
- `games/race_to_n/docs/BENCHMARKS.md` (modify)
- `games/three_marks/docs/BENCHMARKS.md` (modify)
- `games/column_four/docs/BENCHMARKS.md` (modify)
- `docs/TESTING-REPLAY-BENCHMARKING.md` (modify)

## Out of Scope

- `.github/workflows/gate-2-benchmarks.yml` aggregation logic (that is
  `BENCICAL-001`).
- `games/directional_flip/benches/thresholds.json` (non-blocking `1` floors;
  recalibrated by a future ticket once it has stable CI measurements).
- Any `tools/bench-report` change or `thresholds.json` schema-version bump.
- Any gameplay, engine, replay, bot-policy, or benchmark measurement-loop change.
- Touching operations that already pass on CI at their current floor.

## Acceptance Criteria

### Tests That Must Pass

1. A `bench-gate` run (`workflow_dispatch` on the ticket branch, then the
   post-merge `main` push) exits zero: all four games reported, no operation below
   its floor.
2. `cargo run -p bench-report -- --input /tmp/<game>-benchmark-report.txt
   --thresholds games/<game>/benches/thresholds.json` parses every recalibrated
   file without a schema error for `race_to_n`, `three_marks`, and `column_four`.
3. Each recalibrated `threshold` is strictly below the observed CI `current_value`
   for that operation (manual diff review against the evidence run).
4. The benchmark harness logs for recalibrated operations do not print stale
   pre-recalibration thresholds or false failures after the new CI floors are in
   place.

### Invariants

1. `thresholds.json` `schema_version` remains 1 and the field set is unchanged
   (value-only edit; FOUNDATIONS §5 — data stays content, not behavior).
2. Every recalibrated operation's native baseline is recorded in the matching
   `BENCHMARKS.md` (no performance hidden; TESTING §15).
3. Benchmark harness changes are limited to report annotations; measurement loops
   and gameplay calls are unchanged.

## Test Plan

### New/Modified Tests

1. `games/race_to_n/benches/thresholds.json`,
   `games/three_marks/benches/thresholds.json`,
   `games/column_four/benches/thresholds.json` — recalibrated CI floors; the gate
   is the test (a green `bench-gate` run with values below observed CI numbers).
2. `games/{race_to_n,three_marks,column_four}/docs/BENCHMARKS.md` — record native
   baseline + CI floor; verified by manual review.

### Commands

1. `gh workflow run "Gate 2 benchmarks" --ref <ticket-branch>` then
   `gh run view <id> --log` — confirm the `bench-gate` job is green with all four
   games reported.
2. `cargo run -p bench-report -- --input /tmp/race_to_n-benchmark-report.txt
   --thresholds games/race_to_n/benches/thresholds.json` (and the `three_marks` /
   `column_four` equivalents) — confirm parse + pass.
3. The authoritative verification must run on `ubuntu-latest` via
   `workflow_dispatch` / `main` push, not locally: WSL2 numbers are ~34% faster
   and would pass trivially, so a local run cannot prove the CI floors are correct.
