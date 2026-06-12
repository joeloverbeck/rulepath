# GAT13FROCONASY-010: Benchmarks, thresholds, BENCHMARKS.md, and gate-2 CI

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/frontier_control/benches/frontier_control.rs`, `games/frontier_control/benches/thresholds.json`, `games/frontier_control/docs/BENCHMARKS.md`, `.github/workflows/gate-2-benchmarks.yml` (modify)
**Deps**: GAT13FROCONASY-009

## Problem

FOUNDATIONS §6/§11 require benchmark coverage for every official game. Frontier Control needs the benchmark identity list (including the connectivity-scoring hot path and full random playouts), a non-blocking smoke-floor threshold file matching the flood-watch pattern, a `BENCHMARKS.md` documenting them, and a gate-2 CI bench-smoke step. Thresholds start as `baseline_pending_non_blocking` with a named calibration follow-up under ADR 0002/0003/0005.

## Assumption Reassessment (2026-06-11)

1. `games/flood_watch/benches/flood_watch.rs` + `benches/thresholds.json` are the exemplars (the bench file uses the `<game_id>.rs` convention → `frontier_control.rs`); `.github/workflows/gate-2-benchmarks.yml` registers games as explicit per-game steps (verified `flood_watch bench smoke` step present). The finished rules/visibility/bots from earlier tickets are the benchmarked surfaces.
2. Spec §Benchmark operations lists the eleven identities (`legal_actions_garrison_midgame`, `legal_actions_prospectors_midgame`, `validate_action`, `apply_march_with_clash`, `apply_end_turn_round_scoring`, `supply_connectivity_traversal`, `project_public_view_midgame`, `state_hash_terminal`, `garrison_level1_bot_decision`, `prospector_level1_bot_decision`, `random_playout`); spec §Benchmark ADRs aligns thresholds to `baseline_pending_non_blocking`.
3. Cross-artifact boundary under audit: `thresholds.json` is consumed by `tools/bench-report` (registered in GAT13FROCONASY-013); `BENCHMARKS.md` is one of the three docs `tools/rule-coverage` validates — its landing here means the rule-coverage registration (GAT13FROCONASY-013) reaches fully-green only after this ticket lands (expected partial-green window, see Step 6).
4. FOUNDATIONS §6 (evidence-heavy) and the benchmark ADRs 0002/0003/0005 are under audit: PR smoke stays non-gating; variance-aware calibration is a named follow-up once repeated CI measurements exist (multiple representative runs per ADR 0005).

## Architecture Check

1. Non-blocking smoke floors first (vs hard thresholds before CI calibration) match the accepted ADR 0002/0003/0005 posture and avoid flaky gating; the calibration follow-up is named, not silently deferred.
2. No backwards-compatibility aliasing/shims.
3. `engine-core`/`game-stdlib` untouched; benches are game-local.

## Verification Layers

1. Benchmark identity coverage -> benchmark check (`cargo bench -p frontier_control` runs all eleven identities).
2. Threshold posture -> `bench-report` smoke (floors enforced where calibrated; `baseline_pending_non_blocking` non-gating) — exercised after registration in GAT13FROCONASY-013.
3. Evidence doc -> manual review + doc-link check (`BENCHMARKS.md` documents the identities and calibration follow-up).

## What to Change

### 1. Bench harness + thresholds

Author `benches/frontier_control.rs` with the eleven identities and `benches/thresholds.json` with `baseline_pending_non_blocking` floors.

### 2. BENCHMARKS.md

Instantiate from `templates/GAME-BENCHMARKS.md`; document each identity and the named calibration follow-up.

### 3. gate-2 CI

Add a `frontier_control bench smoke` step to `.github/workflows/gate-2-benchmarks.yml` mirroring the flood_watch step.

## Files to Touch

- `games/frontier_control/benches/frontier_control.rs` (new)
- `games/frontier_control/benches/thresholds.json` (new)
- `games/frontier_control/docs/BENCHMARKS.md` (new)
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- `bench-report` tool registration (GAT13FROCONASY-013).
- Threshold calibration to CI floors (named follow-up, not this gate).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p frontier_control -- legal_actions` smoke-runs; the full `cargo bench -p frontier_control` exercises all eleven identities.
2. `node scripts/check-doc-links.mjs` passes with `BENCHMARKS.md` in place.
3. The gate-2 workflow parses and includes the `frontier_control` bench step.

### Invariants

1. Thresholds are non-blocking smoke floors (`baseline_pending_non_blocking`); PR smoke stays non-gating (ADR 0002/0003/0005).
2. The benchmark list includes the connectivity-scoring hot path and full random playouts.

## Test Plan

### New/Modified Tests

1. `games/frontier_control/benches/frontier_control.rs` — the eleven benchmark identities.

### Commands

1. `cargo bench -p frontier_control -- legal_actions`
2. `cargo bench -p frontier_control`
3. Bench-filter smoke is the correct boundary for PR; full calibration is the named ADR 0005 follow-up.

## Outcome

Completed on 2026-06-11.

Changed `games/frontier_control/Cargo.toml`, `games/frontier_control/benches/frontier_control.rs`, `games/frontier_control/benches/thresholds.json`, `games/frontier_control/docs/BENCHMARKS.md`, and `.github/workflows/gate-2-benchmarks.yml`.

Added the harness-free Frontier Control benchmark binary with all eleven required identities, non-blocking smoke-floor thresholds using `baseline_pending_non_blocking`, BENCHMARKS.md documentation with the ADR 0002/0003/0005 calibration follow-up, and gate-2 CI smoke/threshold registration.

Verification:

1. `cargo fmt --all --check` — passed.
2. `cargo bench -p frontier_control -- legal_actions` — passed, ran both legal-action operations.
3. `cargo bench -p frontier_control` — passed, ran all eleven benchmark operations.
4. `node scripts/check-doc-links.mjs` — passed, checked 25 markdown files.
5. Workflow registration grep — passed, found smoke and threshold-gate entries.
6. `cargo clippy -p frontier_control --all-targets -- -D warnings` — passed.
