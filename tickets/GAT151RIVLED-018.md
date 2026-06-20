# GAT151RIVLED-018: Simulation, benchmarks, and BENCHMARKS.md

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence + docs) — `games/river_ledger/benches/{river_ledger.rs,thresholds.json}`, `docs/BENCHMARKS.md`, `ci/games.json`
**Deps**: GAT151RIVLED-017

## Problem

Exercise all seat counts and stack profiles in simulation, add the maximum-layer construction and multi-pot allocation hot paths to the benchmark suite, and reconcile `BENCHMARKS.md` with the new lanes under the accepted CI benchmark process. `BENCHMARKS.md` is a `tools/rule-coverage`-validated doc, so it co-lands with the benches here (not the trailing docs ticket).

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/benches/river_ledger.rs` defines the base lanes (`setup_deal_6p`, `legal_actions_initial_6p`, `apply_call_6p`, `project_all_viewers_6p`, `public_export_import_6p`, `evaluator_showdown_batch_6p`, `level2_full_playout_6p`) with `benches/thresholds.json`; `ci/games.json` line for river_ledger carries `sim_flags`. `tools/rule-coverage` reads `docs/BENCHMARKS.md`.
2. Docs: spec §7.7 names the required hot paths (`setup_3p_equal_stacks`, `setup_6p_asymmetric_stacks`, `legal_actions_short_stack`, `apply_short_all_in_raise`, `construct_side_pots_6p_max_layers`, `allocate_side_pots_6p_split_winners`, `resolve_all_in_showdown_6p`, `project_view_6p_multi_pot`, `serialize_replay_6p_multi_pot`, `bot_policy_6p_short_stack`, `full_game_6p_all_in_pressure`) and that thresholds are calibrated under accepted ADRs 0002–0003, not invented in the spec; proposed ADR 0005 is not binding.
3. Cross-artifact boundary under audit: the benches ↔ `thresholds.json` ↔ `BENCHMARKS.md` (read by `rule-coverage`) ↔ `ci/games.json` sim flags — the largest fixture must exercise six distinct caps, folded money, ≥3 contestable pots, a returned top layer, and a split pot.
4. (§11 deterministic + ADR scope) Restate: native Rust is the primary perf lane; thresholds are calibrated under ADRs 0002–0003 and `docs/TESTING-REPLAY-BENCHMARKING.md`, not asserted here; WASM/browser timing is smoke evidence only. Confirm benchmark fixtures are deterministic.

## Architecture Check

1. Co-locating `BENCHMARKS.md` with the benches keeps the `rule-coverage`-validated doc in sync with the lanes it documents, avoiding a partial-green coverage window across tickets.
2. No backwards-compatibility shims; new lanes extend the existing bench harness.
3. No production logic change; thresholds follow the accepted CI process (ADRs 0002–0003), not this ticket.

## Verification Layers

1. New hot-path lanes run deterministically -> `cargo bench -p river_ledger` (bench filters for a smoke run).
2. Largest fixture exercises six caps / folded money / ≥3 pots / returned layer / split -> fixture assertions in the bench setup.
3. Simulation across seat counts + stack profiles -> `simulate` CLI run.
4. `BENCHMARKS.md` consistent with lanes -> `rule-coverage` reads the doc clean.

## What to Change

### 1. Benchmark lanes + fixtures

Add the §7.7 hot paths to `benches/river_ledger.rs` with a maximum-layer multi-pot fixture; record calibrated thresholds in `thresholds.json` per the accepted CI process (command, build/profile, environment, sample distribution, threshold rationale).

### 2. Simulation flags + BENCHMARKS.md

Update `ci/games.json` sim flags to exercise asymmetric stacks/all-in pressure; reconcile `docs/BENCHMARKS.md` with the new lanes and environment.

## Files to Touch

- `games/river_ledger/benches/river_ledger.rs` (modify)
- `games/river_ledger/benches/thresholds.json` (modify)
- `games/river_ledger/docs/BENCHMARKS.md` (modify)
- `ci/games.json` (modify)

## Out of Scope

- Per-game player/mechanic docs and RULE-COVERAGE/RULES reconciliation (GAT151RIVLED-019).
- Closeout / status flips (GAT151RIVLED-020).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p river_ledger` (bench-filtered smoke) — all new lanes run.
2. `cargo run -p simulate -- --game river_ledger --games 1000` — all seat counts/stack profiles simulate cleanly.
3. `cargo run -p fixture-check -- --game river_ledger` — the maximum-layer fixture validates.

### Invariants

1. Benchmark fixtures are deterministic; the largest exercises six distinct caps, folded money, ≥3 contestable pots, a returned top layer, and a split pot.
2. Thresholds follow the accepted CI process (ADRs 0002–0003); proposed ADR 0005 is not cited as binding.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/benches/river_ledger.rs` + `thresholds.json` — the §7.7 hot-path lanes and calibrated thresholds.

### Commands

1. `cargo bench -p river_ledger`
2. `cargo run -p simulate -- --game river_ledger --games 1000`
3. `cargo run -p fixture-check -- --game river_ledger` — bench + simulate + fixture are the correct performance/evidence boundary; thresholds are calibrated under the CI lanes, not in this ticket.
