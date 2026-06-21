# GAT17VOWTIDOHHEL-020: Native benchmarks, helper before/after, calibrated CI floors by seat count

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence + docs) — new `games/vow_tide/benches/vow_tide.rs`, `games/vow_tide/benches/thresholds.json`, `games/vow_tide/docs/BENCHMARKS.md`; modifies `games/vow_tide/Cargo.toml`
**Deps**: 002, 009, 010, 011, 012

## Problem

Add native benchmarks for setup/deal, bid/card legal-action generation, validation/apply, trick resolution, scoring, projections, effect filtering, replay/export, bots, and full matches for every seat count (`_3p`–`_7p`), plus the promoted-helper before/after microbench evidence and calibrated CI throughput floors. Native is the performance source of truth.

## Assumption Reassessment (2026-06-21)

1. Sibling `games/briar_circuit/benches/{briar_circuit.rs,thresholds.json}` + `Cargo.toml` `[[bench]]` (criterion) are the precedent; `games/vow_tide/benches/` is new. The 002 helper microbench supplies the `game-stdlib` before/after baseline.
2. Spec §7.7/§7.8 fix the operation names (seat-suffixed where the dimension matters), the provisional targets (p95 `<1ms` helper/legal-tree/validation/trick/scoring/one-projection; `<50ms` full-match observer export; `≥75 matches/s` per seat count), and the machine-readable seat-keyed summary.
3. Cross-artifact boundary under audit: `rule-coverage` reads `BENCHMARKS.md` (the 016 partial-green window closes here); the thresholds feed CI bench floors.
4. FOUNDATIONS §6/§11 under audit: benchmarks are required official-game evidence; helper conformance must show no material Plain Tricks/Briar Circuit regression (any regression above noise is investigated, not accepted as abstraction tax).

## Architecture Check

1. One benches ticket covering all operations + helper before/after + thresholds keeps the performance source of truth and its calibration in one place.
2. No shims; additive bench harness.
3. `engine-core` untouched; the `game-stdlib` helper before/after is the §4 promotion evidence, not a new promotion.

## Verification Layers

1. All named operations benchmark across N=3..7 → `cargo bench -p vow_tide`.
2. Helper before/after shows no material back-port regression → `cargo bench -p game-stdlib -p plain_tricks -p briar_circuit` compared to the 002/003/004 baselines.
3. `BENCHMARKS.md` documents environment + targets + calibrated floors → `cargo run -p rule-coverage -- --game vow_tide` (now fully green).
4. Seat-keyed machine-readable summary present → inspect bench report.

## What to Change

### 1. Bench harness + thresholds

`benches/vow_tide.rs` (criterion): setup/deal, bid/card legal-action generation (first/middle/dealer-hook; unconstrained/forced-follow/void), validation/apply, trick resolution, scoring, observer + every-seat projection, effect filtering, replay/export, L0/L1 decisions, full seeded match `_3p`–`_7p`. `benches/thresholds.json` with calibrated floors; `Cargo.toml` `[[bench]]`.

### 2. BENCHMARKS.md

Document operations/fixtures by seat count, helper before/after evidence, environment, provisional targets, calibrated floors, native/WASM distinction.

## Files to Touch

- `games/vow_tide/benches/vow_tide.rs` (new)
- `games/vow_tide/benches/thresholds.json` (new)
- `games/vow_tide/docs/BENCHMARKS.md` (new)
- `games/vow_tide/Cargo.toml` (modify)

## Out of Scope

- WASM/browser latency (smoke only, 017/019); trailing docs (021); capstone (022).
- Re-tuning Plain Tricks/Briar Circuit benchmarks beyond the conformance comparison.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p vow_tide` — all operations run, thresholds present.
2. `cargo bench -p game-stdlib` + `cargo bench -p plain_tricks` + `cargo bench -p briar_circuit` — no material back-port regression.
3. `cargo run -p rule-coverage -- --game vow_tide` — fully green (BENCHMARKS.md present).

### Invariants

1. Native benchmarks cover every supported seat count and the largest surfaces.
2. Helper conformance shows no material regression in the back-ported games.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/benches/vow_tide.rs` + `thresholds.json` — the bench harness and floors.

### Commands

1. `cargo bench -p vow_tide`
2. `cargo run -p rule-coverage -- --game vow_tide`
3. Narrower command rationale: `cargo bench` is the native performance boundary; rule-coverage confirms `BENCHMARKS.md` closes the 016 partial-green window.
