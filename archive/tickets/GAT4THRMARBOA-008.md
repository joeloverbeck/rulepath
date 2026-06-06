# GAT4THRMARBOA-008: Three Marks native benchmarks + thresholds

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — new `games/three_marks/benches/three_marks.rs`, `benches/thresholds.json`; `games/three_marks/Cargo.toml` `[[bench]]` entry
**Deps**: GAT4THRMARBOA-003, GAT4THRMARBOA-004, GAT4THRMARBOA-005, GAT4THRMARBOA-006, GAT4THRMARBOA-007

## Problem

Three Marks must carry native Rust benchmarks (legal-action generation, action application, view generation, replay stepping, random-playout throughput, Level 0/1 bot decisions, serialization/replay) with a typed `thresholds.json` enforced by `tools/bench-report`. The spec sets a visible 300,000+ games/sec random-playout target; a miss must be handled via accepted-ADR recalibration, never silent lowering.

## Assumption Reassessment (2026-06-06)

1. `games/race_to_n/benches/race_to_n.rs` is the mirror and `games/race_to_n/benches/thresholds.json` is the schema exemplar (`schema_version: 1`, `game_id`, `rules_version`, per-operation `{operation_name, unit, threshold, rationale_class, rationale}`). `games/race_to_n/Cargo.toml:15-17` registers `[[bench]] name = "race_to_n" harness = false`. `tools/bench-report` enforces thresholds against that JSON. Verified the bench registration and JSON shape.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §15.8 (required benchmarks + the 300,000+ games/sec target, **sourced from the Stage table of `docs/TESTING-REPLAY-BENCHMARKING.md`**) and §6 (target must remain visible). All measured surfaces (actions/apply/view/replay/bots) come from GAT4THRMARBOA-003–007.
3. Cross-artifact boundary under audit: the `thresholds.json` schema (`schema_version: 1`) consumed by `tools/bench-report`, and the benchmark/ADR discipline in `docs/TESTING-REPLAY-BENCHMARKING.md` + `docs/adr/0001-stage-1-random-playout-budget.md`.
4. FOUNDATIONS §11 (tests, benchmarks, and docs cover the change) and §6 (official games are evidence-heavy) motivate this ticket: benchmark evidence must be honest — no false performance claims, thresholds recorded with a rationale class.
5. New `thresholds.json` conforms to the existing `schema_version: 1` contract consumed by `tools/bench-report`; this is additive (a second game's threshold file). `docs/adr/0001-stage-1-random-playout-budget.md` is the precedent: the `race_to_n` validated-playout floor was recalibrated to 100,000 games/sec for the same correctness-preserving harness shape; a `three_marks` `random_playout` miss is resolved by an analogous accepted-ADR decision (Three Marks is ≤9 plies, so 300,000 may well be reachable), and the threshold decision stays visible in `thresholds.json` + `BENCHMARKS.md`.

## Architecture Check

1. A per-game `benches/three_marks.rs` + `thresholds.json` enforced by the shared `tools/bench-report` is the established pattern and keeps benchmark doctrine data-driven without coupling games. Alternative (hardcoding thresholds in the bench harness) is rejected — it hides recalibration decisions.
2. No backwards-compatibility aliasing/shims — new files; `race_to_n` benches untouched.
3. `engine-core` gains no mechanic nouns; benchmarks live under `games/three_marks/benches/`; no `game-stdlib` extraction.

## Verification Layers

1. Benchmark-runs invariant -> benchmark check (`cargo bench -p three_marks` runs all operations; `-- legal_actions` smoke lane works).
2. Threshold-enforcement invariant -> benchmark check (`tools/bench-report` parses `thresholds.json` and gates the measured operations).
3. Honest-threshold invariant -> manual review + FOUNDATIONS alignment check (§11 no false claims; `random_playout` 300,000 target visible with rationale class; any miss carries an ADR-discipline note, not a silent lowering).

## What to Change

### 1. `games/three_marks/benches/three_marks.rs`

Criterion-style (mirror `race_to_n.rs`) benchmarks: legal-action generation, action application, public-view generation, replay reset/step projection, random-playout throughput, Level 0 decision, Level 1 decision, serialization/replay surfaces.

### 2. `games/three_marks/benches/thresholds.json`

`schema_version: 1`, `game_id: "three_marks"`, `rules_version: "three_marks-rules-v1"`, one entry per operation with `unit`, `threshold`, `rationale_class`, `rationale`. Set `random_playout` against the 300,000 games/sec target; if the first measured baseline cannot meet it, record the threshold decision (and ADR-discipline rationale) rather than silently lowering — see GAT4THRMARBOA-015 `BENCHMARKS.md`.

### 3. `games/three_marks/Cargo.toml`

Add `[[bench]] name = "three_marks" harness = false` and the criterion dev-dependency mirroring `race_to_n`.

## Files to Touch

- `games/three_marks/benches/three_marks.rs` (new)
- `games/three_marks/benches/thresholds.json` (new)
- `games/three_marks/Cargo.toml` (modify)

## Out of Scope

- `BENCHMARKS.md` prose (GAT4THRMARBOA-015).
- Any threshold *recalibration ADR* (a separate `docs/adr/` decision if the baseline misses — flagged, not authored here).
- CI gate-2 benchmark lane wiring beyond what `tools/bench-report` already provides.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p three_marks -- legal_actions` — benchmark smoke lane runs.
2. `cargo run -p bench-report -- --game three_marks` (or the established bench-report invocation) parses `thresholds.json` and reports without error.
3. `cargo build -p three_marks` — `[[bench]]` registration is valid.

### Invariants

1. `thresholds.json` conforms to `schema_version: 1` and carries a `random_playout` entry reflecting the 300,000 games/sec target (or an ADR-disciplined recalibration), with no false performance claim.
2. All required §15.8 operations have a benchmark.

## Test Plan

### New/Modified Tests

1. `games/three_marks/benches/three_marks.rs` — the benchmark harness (verification surface, not a unit test).
2. `games/three_marks/benches/thresholds.json` — threshold fixture consumed by `tools/bench-report`.

### Commands

1. `cargo bench -p three_marks -- legal_actions`
2. `cargo build --workspace && cargo run -p bench-report -- --game three_marks`
3. A full `cargo bench -p three_marks` is the authoritative measurement; the `legal_actions` smoke lane is the correct fast boundary for CI/iteration.

## Outcome

Completed: 2026-06-06

What changed:

- Added `games/three_marks/benches/three_marks.rs` custom native benchmark harness with a marked JSON report.
- Added `games/three_marks/benches/thresholds.json` with `schema_version: 1`, Three Marks identity/version metadata, and thresholds for legal actions, apply, public view generation, replay step projection, serialization round-trip, replay throughput, random playout, Level 0 bot decisions, and Level 1 bot decisions.
- Registered `[[bench]] name = "three_marks" harness = false` in `games/three_marks/Cargo.toml`.

Deviations from original plan:

- `tools/bench-report` currently accepts `--input` and `--thresholds`, not `--game`; verification used `cargo run -p bench-report -- --input /tmp/three_marks_bench.json --thresholds games/three_marks/benches/thresholds.json`.
- The first full local native report missed the visible Stage 2 `random_playout` target: about 60,159 games/sec versus 300,000 games/sec. The committed threshold keeps that miss explicit with `rationale_class: measured_baseline_adr_followup_required`; this is a provisional measured floor, not a silent target change. GAT4THRMARBOA-015 must document the miss, and any accepted target recalibration requires ADR discipline.

Verification results:

- `cargo fmt --all --check`
- `cargo bench -p three_marks -- legal_actions`
- `cargo bench -p three_marks > /tmp/three_marks_bench.txt`
- `awk '/BEGIN_THREE_MARKS_BENCHMARK_JSON/{flag=1;next}/END_THREE_MARKS_BENCHMARK_JSON/{flag=0}flag' /tmp/three_marks_bench.txt > /tmp/three_marks_bench.json`
- `cargo run -p bench-report -- --input /tmp/three_marks_bench.json --thresholds games/three_marks/benches/thresholds.json`
- `cargo build --workspace`
- `bash scripts/boundary-check.sh`
