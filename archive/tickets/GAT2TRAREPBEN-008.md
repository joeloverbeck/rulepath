# GAT2TRAREPBEN-008: Stage-1 random-playout budget triage + threshold decision

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/race_to_n` benchmark profiling and either a bounded optimization or an accepted target recalibration; `BENCHMARKS.md` records the decision; possibly a new ADR.
**Deps**: GAT2TRAREPBEN-006, GAT2TRAREPBEN-007

## Problem

`BENCHMARKS.md` records `random_playout` ≈ 134,277 games/sec against the Stage-1
budget of 500,000 games/sec — a miss carried forward (the Gate-1-closeout TODO never
resolved it). Spec §D6 forbids silently waiving the target: Gate 2 is not done until
either `random_playout >= 500,000 games/sec` under the accepted benchmark
command/environment, or the target is formally recalibrated through an accepted
decision mechanism (ADR / benchmark-doctrine adjustment).

## Assumption Reassessment (2026-06-05)

1. `games/race_to_n/benches/race_to_n.rs` measures `random_playout`;
   `games/race_to_n/docs/BENCHMARKS.md` records the 134,277.09 games/sec value, the
   500,000 target, the "no silent waiver" stance, and a WSL2 noisy-host caveat. The
   structured JSON output + `thresholds.json` (GAT2TRAREPBEN-006) and the hard-fail
   gate (GAT2TRAREPBEN-007) now exist to measure against.
2. Spec §D6 / §WB7 give the triage questions (build/profile correct? accidental setup
   cost? allocations? validation overhead? WSL/noisy host? harness measuring the right
   thing? target unrealistic?) and the exit stance. `docs/TESTING-REPLAY-BENCHMARKING.md
   §15` marks the Stage-1 budget "to be replaced by measured baselines", so principled
   recalibration is doctrine-anticipated.
3. Conditional deliverable — the triage must *distinguish* its hypotheses before
   choosing a fix: profiling separates setup/allocation/validation/harness overhead
   from a genuine target gap. Whichever branch is taken references correct paths
   (`benches/race_to_n.rs`, `BENCHMARKS.md`); both branches respect FOUNDATIONS.
4. FOUNDATIONS §13 ADR trigger: if the resolution is to recalibrate the Stage-1
   target (a benchmark-doctrine change), that decision requires an accepted ADR or
   doctrine adjustment — restate this before trusting any "just lower the threshold"
   shortcut. Lowering a threshold only to make CI green is forbidden (§D6; §Forbidden).
5. §11 determinism / correctness: any optimization must preserve validation
   correctness and deterministic replay/hash — speed must not be bought by weakening
   validation or correctness. The accepted threshold is then enforced by `bench-report`.

## Architecture Check

1. Triage-before-fix (profile, then optimize-or-recalibrate) is cleaner than a blind
   optimization pass or a silent target edit — it produces evidence either way.
2. No backwards-compatibility shims; no hidden threshold. Any target change is
   documented with rationale and (if doctrine-changing) an ADR.
3. `engine-core` untouched; profiling/optimization stays in `games/race_to_n`; no
   kernel noun, no correctness regression.

## Verification Layers

1. Resolution is real → benchmark check: either `random_playout >= 500,000 games/sec`
   under the accepted command, or `thresholds.json` carries the recalibrated value
   with rationale and `bench-report` hard-fails it.
2. Decision recorded → manual review: `BENCHMARKS.md` records the decision,
   environment, command, and caveats; an ADR exists if the target was recalibrated.
3. Correctness preserved → deterministic replay-hash check + `cargo test -p race_to_n`:
   any optimization leaves rules/replay/hashes unchanged.

## What to Change

### 1. Profile `random_playout` (`games/race_to_n/benches/race_to_n.rs`)

Determine whether setup, allocation, validation, bot construction, or harness overhead
explains the miss. Apply a bounded, low-risk optimization if the cause is obvious and
correctness-neutral.

### 2. Decide the threshold

If the target is met, set the accepted Stage-1 threshold in `thresholds.json`
(GAT2TRAREPBEN-006). If the target is unrealistic for the current correctness scope,
produce an accepted benchmark-doctrine / ADR recalibration and set the new value.

### 3. Record the decision (`games/race_to_n/docs/BENCHMARKS.md`, optional `docs/adr/*`)

Record the decision, environment, command, current value, and caveats. Create an ADR
under `docs/adr/` only if the target is recalibrated or doctrine changes (none exists
yet — `docs/adr/` holds only `ADR-TEMPLATE.md`).

## Files to Touch

- `games/race_to_n/benches/race_to_n.rs` (modify) — profiling + any bounded optimization
- `games/race_to_n/docs/BENCHMARKS.md` (modify) — record the Stage-1 decision, command, environment, caveats
- `docs/adr/NNNN-*.md` (new, conditional) — only if the target is recalibrated / doctrine changes

## Out of Scope

- Broad performance rewrite or replacing correctness with speed (§D6 non-goals).
- Benchmark JSON/threshold-file mechanics (GAT2TRAREPBEN-006) and the gate tool (GAT2TRAREPBEN-007).
- Criterion / Iai-Callgrind migration (spec §D5 out-of-scope by default).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p race_to_n` — reports `random_playout` at or above the accepted threshold (either 500,000 games/sec or the recalibrated value).
2. `cargo run -p bench-report -- --input <report> --thresholds games/race_to_n/benches/thresholds.json` — hard-fails if `random_playout` is below the accepted threshold.
3. `cargo test -p race_to_n` — rules/replay/hashes unchanged by any optimization.

### Invariants

1. The Stage-1 miss is resolved by evidence or formal recalibration — never a silent waiver (§D6).
2. Any target recalibration is recorded with rationale, and is ADR-backed when doctrine changes (§13).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/benches/race_to_n.rs` — optimization (if any) is covered by existing benchmark + `cargo test -p race_to_n` regression.
2. `games/race_to_n/docs/BENCHMARKS.md` — the decision record (manual-review artifact, not a test).

### Commands

1. `cargo bench -p race_to_n`
2. `cargo run -p bench-report -- --input <report> --thresholds games/race_to_n/benches/thresholds.json`
3. `cargo test -p race_to_n` — confirms correctness is preserved through any optimization.
