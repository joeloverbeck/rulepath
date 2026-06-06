# GAT5COLFOUPUB-011: Column Four benchmarks & thresholds

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — new `games/column_four/benches/column_four.rs`, `games/column_four/benches/thresholds.json`, `games/column_four/docs/BENCHMARKS.md`; modify `games/column_four/Cargo.toml` (`[[bench]]`)
**Deps**: 002, 008

## Problem

`docs/OFFICIAL-GAME-CONTRACT.md` requires native benchmarks for legal actions, apply, view/effect projection, replay/serialization, simulation throughput, and bot decisions. `docs/TESTING-REPLAY-BENCHMARKING.md` records a provisional `column_four` random-playout target of 100,000+ games/sec; Gate 5 must measure honestly and document the result, not fake a threshold (spec §14).

## Assumption Reassessment (2026-06-06)

1. `games/three_marks/benches/` contains `three_marks.rs` + `thresholds.json`, wired via `[[bench]]` (`name = "three_marks"`, `harness = false`) in `games/three_marks/Cargo.toml` (verified). `column_four` mirrors this; the crate `Cargo.toml` was created in GAT5COLFOUPUB-002, so this ticket adds the `[[bench]]` entry (create-then-modify on 002's file).
2. Spec §14 (benchmark surfaces, honest thresholds) and `docs/TESTING-REPLAY-BENCHMARKING.md` line for `column_four` (100,000+ games/sec provisional) define the targets. The benchmarked surfaces (legal actions, apply, view/effect, bots) come from GAT5COLFOUPUB-003/004/005/008.
3. Cross-artifact boundary under audit: the benchmark/threshold contract in `docs/TESTING-REPLAY-BENCHMARKING.md` and the `tools/bench-report` gating lanes (`docs/adr/0002-ci-benchmark-gating-lanes.md`). This ticket produces benches + a measured threshold file + `BENCHMARKS.md`; CI lane wiring is GAT5COLFOUPUB-016.
4. FOUNDATIONS §6 (benchmarks are required evidence) motivates this ticket; honest measured thresholds (spec §14.2) avoid aspirational CI failures.

## Architecture Check

1. Mirroring the `three_marks` bench shape (criterion-style harness + `thresholds.json`) keeps `column_four` inside the existing `bench-report` gating model — cleaner than a bespoke benchmark format. Provisional-vs-measured divergence is documented in `BENCHMARKS.md`, not hidden.
2. No backwards-compatibility aliasing/shims — new bench files; the only modify is the additive `[[bench]]` stanza.
3. No engine-core/game-stdlib change; benchmarks measure the game-local surfaces only.

## Verification Layers

1. Bench-surface invariant -> benchmark check: benches exist for legal-actions, apply, view/effect projection, simulation throughput, and Level 0 + Level 2 bot decisions.
2. Honest-threshold invariant -> manual review: `thresholds.json` values are measured on the dev machine and `BENCHMARKS.md` records environment + any miss against the 100,000+ provisional target.
3. Bench-runs invariant -> benchmark check: `cargo bench -p column_four` runs to completion without panics.
4. Doc-consistency invariant -> codebase grep-proof: `BENCHMARKS.md` names the bench surfaces and the threshold file (consumed later by `tools/rule-coverage` in GAT5COLFOUPUB-013).

## What to Change

### 1. `games/column_four/benches/column_four.rs` + `thresholds.json`

Benchmark legal-action generation, action apply, public-view/effect projection, random-playout simulation throughput, and Level 0 + Level 2 bot decisions; record measured baselines in `thresholds.json`.

### 2. `games/column_four/Cargo.toml`

Add the `[[bench]]` stanza (`name = "column_four"`, `harness = false`).

### 3. `games/column_four/docs/BENCHMARKS.md`

From `templates/GAME-BENCHMARKS.md`: bench surfaces, environment notes, measured results, threshold rationale, CI lane behavior, public-latency relevance, and an honest caveat if the 100,000+ provisional target is missed.

## Files to Touch

- `games/column_four/benches/column_four.rs` (new)
- `games/column_four/benches/thresholds.json` (new)
- `games/column_four/docs/BENCHMARKS.md` (new)
- `games/column_four/Cargo.toml` (modify)

## Out of Scope

- CI benchmark-lane wiring in `.github/workflows/gate-2-benchmarks.yml` (GAT5COLFOUPUB-016).
- Unrelated recalibration of existing games' thresholds (spec §14.3 forbids hiding it here).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p column_four -- --test` (or the repo's smoke-bench invocation) — benches compile and run.
2. `test -f games/column_four/benches/thresholds.json && test -f games/column_four/docs/BENCHMARKS.md` — threshold + doc present.
3. `grep -q '\[\[bench\]\]' games/column_four/Cargo.toml` — bench wired.

### Invariants

1. Thresholds are measured, not aspirational; a miss against the provisional target is documented, not faked.
2. No unrelated benchmark recalibration of other games lands in this ticket.

## Test Plan

### New/Modified Tests

1. `games/column_four/benches/column_four.rs` — the benchmark suite.
2. `games/column_four/benches/thresholds.json` — measured baselines.

### Commands

1. `cargo bench -p column_four` (smoke: append the crate's quick-run flag if defined)
2. `cargo build -p column_four --benches`
3. `cargo run -p bench-report -- --game column_four` — note: full gating-lane behavior lands with GAT5COLFOUPUB-016; here the bench compile+run is the boundary.
