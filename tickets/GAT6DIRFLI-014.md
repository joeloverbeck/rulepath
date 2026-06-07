# GAT6DIRFLI-014: Benchmarks, thresholds & BENCHMARKS.md

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/directional_flip/benches/directional_flip.rs`, `games/directional_flip/benches/thresholds.json`, `games/directional_flip/docs/BENCHMARKS.md`.
**Deps**: 004, 011

## Problem

The official-game contract requires honest, baseline-first benchmarks with a documented threshold posture and no fake throughput claims (FOUNDATIONS §6, spec §8.7, §5.2 "no speculative 30,000+ games/sec"). This ticket adds the `directional_flip` benchmark harness, thresholds file, and `BENCHMARKS.md` report covering the hot paths (legality generation, flip scanning, apply, projection, action-tree+previews, effect encoding, replay, serialization, bot moves, simulation throughput).

## Assumption Reassessment (2026-06-07)

1. `games/column_four/benches/column_four.rs` + `games/column_four/benches/thresholds.json` + `games/column_four/docs/BENCHMARKS.md` are the precedents. The surfaces to benchmark exist: rules (005), actions/previews (006), effects (008), replay (009), bots (011). The crate is a workspace member (GAT6DIRFLI-004).
2. Spec §8.7 (required benchmark coverage list + honest-baseline posture) and `docs/TESTING-REPLAY-BENCHMARKING.md` are authoritative; `docs/adr/0002-ci-benchmark-gating-lanes.md` governs benchmark CI gating posture (non-blocking baseline unless measured evidence supports a threshold).
3. Cross-artifact boundary under audit: `benches/thresholds.json` ↔ `tools/bench-report` (GAT6DIRFLI-016) ↔ the Gate 2 benchmark CI lane (`.github/workflows/gate-2-benchmarks.yml`, GAT6DIRFLI-019). The thresholds posture must honor the existing ADR-0002 lane semantics.
4. FOUNDATIONS §6 (official games carry benchmarks) and the spec's honesty constraint motivate this ticket: restate before authoring — `BENCHMARKS.md` reports measured environment/commands/values; no speculative throughput threshold is claimed without measured CI evidence; prefer a non-blocking baseline first (spec §8.7, §13.2).

## Architecture Check

1. A baseline-first non-blocking report (rather than an invented blocking threshold) matches ADR-0002 and the spec's anti-fake-performance stance, and lets real CI numbers set conservative thresholds later.
2. No backwards-compatibility shims; new bench harness + report.
3. `engine-core` untouched; benches are game-local (§3).

## Verification Layers

1. Benchmark runs -> benchmark check: `cargo bench -p directional_flip` executes all listed hot-path benches without error.
2. Honest thresholds -> manual review against spec §8.7/§5.2: `thresholds.json` posture is baseline-first; `BENCHMARKS.md` cites measured values + environment, no fabricated throughput.
3. Bench-report integration -> simulation/CLI run: `tools/bench-report` (after GAT6DIRFLI-016) includes the directional-flip report and threshold posture.

## What to Change

### 1. Benchmark harness

`benches/directional_flip.rs` covering spec §8.7: setup, legality generation, placement validation, flip scanning, apply, forced-pass handling, projection/view, action-tree+previews, semantic-effect encoding, replay export/import, serialization round trip, random bot move, Level 2-lite bot move, random simulation throughput, and a bot-vs-random/bot-vs-bot smoke if not too costly.

### 2. Thresholds & report

`benches/thresholds.json` (baseline-first, non-blocking unless CI requires blocking with measured evidence) and `docs/BENCHMARKS.md` (environment, commands, measured values, threshold posture, no fake claims).

## Files to Touch

- `games/directional_flip/benches/directional_flip.rs` (new)
- `games/directional_flip/benches/thresholds.json` (new)
- `games/directional_flip/docs/BENCHMARKS.md` (new)
- `games/directional_flip/Cargo.toml` (modify — register the `[[bench]]` target, mirroring column_four)

## Out of Scope

- `tools/bench-report` registration (GAT6DIRFLI-016) and the Gate 2 CI lane wiring (019).
- Functional correctness tests (GAT6DIRFLI-012); benches measure, they do not assert rules.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p directional_flip -- legal_actions` (smoke) and `cargo bench -p directional_flip` (full) run without error.
2. Manual review: `BENCHMARKS.md` reports measured values + environment; no speculative throughput threshold.

### Invariants

1. Benchmark thresholds are honest/baseline-first; no fake performance claim (FOUNDATIONS §6, spec §8.7/§5.2).
2. Threshold posture honors ADR-0002 CI gating semantics.

## Test Plan

### New/Modified Tests

1. `games/directional_flip/benches/directional_flip.rs` — the hot-path bench set above.

### Commands

1. `cargo bench -p directional_flip -- legal_actions`
2. `cargo bench -p directional_flip`
3. A bench smoke + full run is the correct boundary; CI lane gating is GAT6DIRFLI-019 and the aggregated report is GAT6DIRFLI-016.
