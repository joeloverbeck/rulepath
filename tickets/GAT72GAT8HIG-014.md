# GAT72GAT8HIG-014: Benchmarks + thresholds + BENCHMARKS.md + gate-2 CI

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/high_card_duel/benches/{high_card_duel.rs,thresholds.json}`, `games/high_card_duel/docs/BENCHMARKS.md`, `.github/workflows/gate-2-benchmarks.yml`
**Deps**: GAT72GAT8HIG-008, GAT72GAT8HIG-010

## Problem

Official games carry native benchmark coverage with calibrated thresholds. Gate
8 needs benches for setup/shuffle, legal-action generation, validation, apply,
view projection (public + seat), effect filtering, replay export/reconstruction,
serialization, random playout, and Level 0 bot latency, plus a CI bench-smoke
lane matching the existing per-game lanes.

## Assumption Reassessment (2026-06-07)

1. Verified the bench convention: sibling `games/draughts_lite/benches/
   draughts_lite.rs` + `thresholds.json`, registered via `[[bench]] name =
   "draughts_lite" harness = false` (the `high_card_duel` `[[bench]]` target was
   added in GAT72GAT8HIG-003). `.github/workflows/gate-2-benchmarks.yml:32-38`
   carries a per-game bench-smoke step (`column_four`, `directional_flip`,
   `draughts_lite`).
2. Verified against the spec: §4.2.11 enumerates the bench surfaces and requires
   thresholds from measured baselines (not fabricated), conservative thresholds
   under high variance, and CI updates that do not weaken existing gates.
3. Cross-artifact boundary under audit: the benchmark threshold/lane contract
   (`docs/adr/0002`, `docs/adr/0003` calibrated thresholds;
   `docs/TESTING-REPLAY-BENCHMARKING.md`). The new lane must follow the existing
   ADR lane conventions.
4. FOUNDATIONS principle under audit (§6 evidence-heavy + §11 evidence coverage):
   benchmarks are part of the official-game evidence set; thresholds must be
   measured, not invented.

## Architecture Check

1. Mirroring the existing criterion-style bench + `thresholds.json` + CI lane is
   cleaner than a bespoke harness — it plugs into the calibrated-threshold ADR
   lanes already in CI.
2. No backwards-compatibility shims — additive bench + CI step.
3. `engine-core`/`game-stdlib` untouched; benches exercise the game crate.

## Verification Layers

1. Benches run -> benchmark check: `cargo bench -p high_card_duel` executes all named benches.
2. Threshold calibration -> manual review: `thresholds.json` values derive from a documented baseline sample (`BENCHMARKS.md` records the rationale).
3. CI lane parity -> manual review: the new gate-2 step matches existing per-game lane shape and does not weaken existing gates.

## What to Change

### 1. `benches/high_card_duel.rs` + `thresholds.json`

Benches for setup+shuffle, legal-action gen (lead/reply), validation, apply
(commit/reveal/refill), public + seat-private view projection, effect filtering,
public replay export, internal replay reconstruction, serialization, random
playout, Level 0 bot latency; thresholds from measured baselines.

### 2. `docs/BENCHMARKS.md`

Record baseline samples and threshold rationale (template `GAME-BENCHMARKS.md`).

### 3. `.github/workflows/gate-2-benchmarks.yml`

Add a `high_card_duel` bench-smoke step alongside the existing lanes.

## Files to Touch

- `games/high_card_duel/benches/high_card_duel.rs` (new)
- `games/high_card_duel/benches/thresholds.json` (new)
- `games/high_card_duel/docs/BENCHMARKS.md` (new)
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- The `[[bench]]` Cargo target (already added in GAT72GAT8HIG-003).
- Web/browser benchmarks (the native set is the gate requirement).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p high_card_duel` — all benches run.
2. `cargo bench -p high_card_duel -- <smoke filter>` — the CI smoke filter runs quickly.

### Invariants

1. Thresholds are measured, not fabricated (§6/§11 evidence).
2. The new CI lane does not weaken existing benchmark gates.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/benches/high_card_duel.rs` + `thresholds.json` — the bench suite + calibrated thresholds.

### Commands

1. `cargo bench -p high_card_duel`
2. `cargo build --workspace` (confirms the bench target compiles in-tree)
3. `cargo bench` is the correct boundary — benchmarks are not part of `cargo test`.
