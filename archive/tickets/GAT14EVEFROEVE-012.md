# GAT14EVEFROEVE-012: Benchmarks, thresholds, BENCHMARKS.md, and gate-2 CI

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/event_frontier/benches/event_frontier.rs` (new), `games/event_frontier/benches/thresholds.json` (new), `games/event_frontier/docs/BENCHMARKS.md` (new), `.github/workflows/gate-2-benchmarks.yml` (modify)
**Deps**: GAT14EVEFROEVE-011

## Problem

The gate must carry benchmarks proving "action trees remain usable" at the largest branching factor of any official game, plus the ROADMAP §15-budget row `event_frontier: 100+ turns/sec`. This ticket adds the Criterion bench harness over the spec's benchmark identities (setup, shuffle, peak-op-branching tree generation, event/op application, edict projection, Reckoning pipeline, bot latency, serialize, full playout), the `thresholds.json` with smoke floors, `BENCHMARKS.md`, and the gate-2 CI registration — smoke floors first (`baseline_pending_non_blocking`), variance-aware calibration as a named follow-up per ADRs 0002/0003/0005.

## Assumption Reassessment (2026-06-12)

1. The benchmarked surfaces exist: verified tickets 004–010 implement setup/shuffle, peak-op-branching tree generation, event/op application, edict projection, the Reckoning pipeline, the bots, and serialization — the identities the bench exercises. The sibling bench shape is `games/frontier_control/benches/frontier_control.rs` + `thresholds.json`.
2. The budget row and CI lane are current: verified `docs/TESTING-REPLAY-BENCHMARKING.md` §15 already lists `| 14 | event_frontier | 100+ turns/sec |` (line ~328) and that `.github/workflows/gate-2-benchmarks.yml` registers games per-game (bench-smoke + bench-gate jobs, e.g. frontier_control at lines ~50/~118); the benchmark ADRs 0002/0003/0005 exist under `docs/adr/`.
3. Cross-artifact boundary under audit: `BENCHMARKS.md` is one of the three docs `tools/rule-coverage` reads (with `RULES.md`/`RULE-COVERAGE.md`); since `RULE-COVERAGE.md` lands with the rule-coverage registration (ticket 015), a fully-green `rule-coverage --game event_frontier` depends on this ticket's `BENCHMARKS.md` having landed (partial-green window flagged in Step 6). `thresholds.json` is consumed by `tools/bench-report` (registered in ticket 015).
4. FOUNDATIONS §11 (benchmarks cover the change) and the benchmark ADRs motivate this ticket. Restated before trusting the spec: smoke floors are non-blocking on the PR lane (`baseline_pending_non_blocking`); variance-aware calibration is a named follow-up once representative CI runs exist (ADR 0005), not a hard gate now.
5. Determinism note (§11): benchmarks must not introduce nondeterministic inputs into canonical forms — they measure throughput over the deterministic setup/RNG; no wall-clock seeding enters replayed state. No replay/hash semantics change.

## Architecture Check

1. Smoke floors first with named calibration follow-up is cleaner than guessing hard thresholds before representative CI variance is known (ADR 0005): it gates regressions without false-failing on cold-runner variance.
2. No backwards-compatibility aliasing/shims — additive bench + threshold + CI step.
3. `engine-core` stays noun-free; no `game-stdlib` promotion.

## Verification Layers

1. Bench identities run -> `cargo bench -p event_frontier` executes every identity (setup, shuffle, `legal_tree_peak_op_branching`, apply event/op, edict projection, Reckoning, bot latency, serialize, `full_random_playout`).
2. Threshold registration -> `tools/bench-report` reads `thresholds.json` (smoke floors); `bench-report` enforces floors where calibrated (verified in ticket 015 registration).
3. CI lane -> `gate-2-benchmarks.yml` adds the bench-smoke and bench-gate steps for `event_frontier`.
4. Doc consistency -> `BENCHMARKS.md` records the identities and the 100+ turns/sec budget; `node scripts/check-doc-links.mjs` passes.

## What to Change

### 1. Bench harness + thresholds

Author `benches/event_frontier.rs` (Criterion) over the spec's "Benchmark identities" and `benches/thresholds.json` with smoke floors (`baseline_pending_non_blocking`). Register the bench target in `games/event_frontier/Cargo.toml`.

### 2. BENCHMARKS.md

Instantiate from `templates/GAME-BENCHMARKS.md`; record the identities, the 100+ turns/sec stage budget, and the calibration-follow-up posture (ADRs 0002/0003/0005).

### 3. gate-2 CI

Add `event_frontier` bench-smoke and bench-gate steps to `.github/workflows/gate-2-benchmarks.yml`, mirroring the sibling game's steps.

## Files to Touch

- `games/event_frontier/benches/event_frontier.rs` (new)
- `games/event_frontier/benches/thresholds.json` (new)
- `games/event_frontier/docs/BENCHMARKS.md` (new)
- `games/event_frontier/Cargo.toml` (modify; created by 003) — register the bench target
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- `bench-report` tool registration (ticket 015) — this ticket authors the thresholds it reads.
- Variance-aware hard-threshold calibration — a named follow-up once representative CI runs exist (ADR 0005), not this ticket.
- `RULE-COVERAGE.md` (ticket 015).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p event_frontier` runs the full benchmark identity list.
2. `node scripts/check-doc-links.mjs` passes with `BENCHMARKS.md` present.
3. The gate-2 CI steps for `event_frontier` are syntactically valid (workflow lints / dry parse).

### Invariants

1. Smoke floors are non-blocking on the PR lane; no hard threshold is asserted before calibration.
2. The bench covers peak-op-branching tree generation and the full-playout turns/sec budget.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/benches/event_frontier.rs` — the benchmark identities.
2. `games/event_frontier/benches/thresholds.json` — smoke floors.

### Commands

1. `cargo bench -p event_frontier -- --warm-up-time 1 --measurement-time 1` (smoke run)
2. `cargo bench -p event_frontier`
3. A smoke bench run is the correct boundary on the PR lane (ADR 0002 non-gating); full calibrated thresholds follow once CI variance is characterized.

## Outcome

- Added the custom native bench target `games/event_frontier/benches/event_frontier.rs` and registered it in `games/event_frontier/Cargo.toml`.
- Added smoke-floor thresholds for the 12 emitted benchmark identities in `games/event_frontier/benches/thresholds.json`.
- Added `games/event_frontier/docs/BENCHMARKS.md` documenting the identities, smoke-floor posture, ADR 0002/0003/0005 calibration follow-up, and the 100+ turns/sec full-playout budget.
- Registered `event_frontier` in `.github/workflows/gate-2-benchmarks.yml` for pull-request bench smoke and non-PR threshold gating.

## Verification

- `cargo fmt --all --check`
- `cargo bench -p event_frontier -- --warm-up-time 1 --measurement-time 1`
- `cargo bench -p event_frontier`
- `cargo run -p bench-report -- --input /tmp/event_frontier-benchmark-report.txt --thresholds games/event_frontier/benches/thresholds.json`
- `node scripts/check-doc-links.mjs`
- `python3 -c "import yaml, pathlib; yaml.safe_load(pathlib.Path('.github/workflows/gate-2-benchmarks.yml').read_text()); print('workflow yaml ok')"`
