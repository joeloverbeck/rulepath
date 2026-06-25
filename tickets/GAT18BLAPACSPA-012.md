# GAT18BLAPACSPA-012: native benchmarks and BENCHMARKS.md

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (benchmark) — `games/blackglass_pact/benches`, `games/blackglass_pact/docs/BENCHMARKS.md`
**Deps**: GAT18BLAPACSPA-009

## Problem

Add the criterion benchmark suite and `BENCHMARKS.md` for Blackglass Pact: workload IDs and seed/fixture manifest, native measurements for setup/blind/deal, legal-tree generation, scoring, views, exports, bots, and full match, with by-seat/by-team output, variance-aware thresholds, and the honest provisional posture (spec §7.8–§7.9, §4.1 benches, candidate task `GAT18-BLAPAC-011`).

## Assumption Reassessment (2026-06-25)

1. The sibling `games/briar_circuit/benches/{briar_circuit.rs,thresholds.json}` layout is the convention; benches exercise the behavior shipped in GAT18BLAPACSPA-004–009.
2. Spec §7.8 fixes the surface/action-fanout budgets and §7.9 the provisional performance posture (native p95 <1 ms/op, export <50 ms p95, ≥75 matches/s, zero unexplained cap breaches in the 1,000-match corpus).
3. Cross-artifact boundary under audit: `BENCHMARKS.md` is also read by `tools/rule-coverage` (GAT18BLAPACSPA-013) — so the rule-coverage ticket depends on this doc landing; the benchmark thresholds are calibrated via the accepted benchmark ADR process, not silently waived.
4. FOUNDATIONS §6 (evidence-heavy official games) motivates this ticket: benchmarks are a required acceptance surface; a budget may be recalibrated honestly but not deleted because the partnership game is slower than a sibling.

## Architecture Check

1. A criterion bench suite mirroring the sibling layout (vs. ad-hoc timing) gives variance-aware, reproducible thresholds and a by-seat/by-team output shape consistent with the simulator.
2. No shims; provisional budgets are recorded with their original target + reason, not waived.
3. `engine-core` untouched; no `game-stdlib` change; benches only.

## Verification Layers

1. Bench suite compiles and runs with stable workload IDs -> `cargo bench -p blackglass_pact` (or its smoke filter).
2. `BENCHMARKS.md` records workloads, seed/fixture manifest, variance posture, by-seat/by-team output -> manual review against §7.9.
3. Promoted-helper workloads show no regression from the Gate 17 baseline without investigation -> bench comparison note.

## What to Change

### 1. Benchmark suite

`benches/blackglass_pact.rs` + `benches/thresholds.json`: setup/blind/deal, blind/bid/card legal trees, promoted-helper conformance, trick transition, hand scoring, views (observer + 4 seats), replay/export, bots (L0/L1), full match (all-L0 + L1-bearing).

### 2. BENCHMARKS.md

Workload IDs, seed/fixture manifest, native + (provisional) browser measurements, variance/floors, by-seat/by-team output, regression decisions, hardware/toolchain context.

## Files to Touch

- `games/blackglass_pact/benches/blackglass_pact.rs` (new)
- `games/blackglass_pact/benches/thresholds.json` (new)
- `games/blackglass_pact/Cargo.toml` (modify — add `[[bench]]`)
- `games/blackglass_pact/docs/BENCHMARKS.md` (new)

## Out of Scope

- Browser benchmark capture beyond the provisional posture (recorded as provisional).
- `rule-coverage`/`RULE-COVERAGE.md` (GAT18BLAPACSPA-013, which reads this `BENCHMARKS.md`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p blackglass_pact` (suite compiles and runs; bench filters for a smoke run).
2. `BENCHMARKS.md` names every §7.9 workload + the provisional budgets with original target/reason.
3. `cargo build --workspace` with the new `[[bench]]`.

### Invariants

1. No performance budget is silently deleted; recalibration cites the accepted benchmark process.
2. Benchmark output is keyed by stable seat and team IDs.

## Test Plan

### New/Modified Tests

1. `games/blackglass_pact/benches/blackglass_pact.rs` — criterion workloads per §7.9.
2. `games/blackglass_pact/benches/thresholds.json` — variance-aware thresholds.
3. `games/blackglass_pact/docs/BENCHMARKS.md` — workload/seed/variance manifest.

### Commands

1. `cargo bench -p blackglass_pact`
2. `cargo build --workspace`
3. The bench crate is the correct boundary; rule-coverage consumes `BENCHMARKS.md` downstream in GAT18BLAPACSPA-013.
