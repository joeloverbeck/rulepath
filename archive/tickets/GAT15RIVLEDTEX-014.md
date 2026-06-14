# GAT15RIVLEDTEX-014: Native simulation (`--seat-count`) and benchmarks by seat count

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tools/simulate` (`src/main.rs`, `Cargo.toml`), `games/river_ledger/benches/river_ledger.rs`, `benches/thresholds.json`, `docs/BENCHMARKS.md`
**Deps**: GAT15RIVLEDTEX-013

## Problem

River Ledger must register in native simulation and benchmarking across 3/4/5/6 seats, with native benchmarks the source of performance truth. The simulator currently has no seat-count input and is two-seat-oriented, so this gate adds a `--seat-count` flag and a seat-count-aware River Ledger runner, plus benches for the expensive paths that regress with seat count.

## Assumption Reassessment (2026-06-14)

1. `tools/simulate/src/main.rs` exposes `--games`, `--start-seed`, `--action-cap`, `--failure-report-out` (lines 156–158, 221) — **no `--seat-count`** — and every game runner hard-codes the two-seat `two_seat_counts` helper, while `Summary::new(seats)` / `increment_seat_count` / `wins_by_seat: BTreeMap<String,u64>` already support N seats (Infra B). Change rationale: River Ledger is the first >2-seat simulation consumer, so adding `--seat-count` + a seat-count-aware `run_river_ledger_simulation` is additive new work, not a plumbing rebuild.
2. `specs/...-base.md` §4.3 (Simulator row), §5 G15-RL-009, §7.1, and Assumption 2 fix the `--seat-count` flag, the seat range {3,4,5,6}, and the benchmark metric set; `games/poker_lite/benches/{poker_lite.rs,thresholds.json}` are the bench precedent.
3. Cross-artifact boundary under audit: `tools/simulate` gains a `river_ledger` dependency + dispatch building an N-seat summary via the generic helpers (not `two_seat_counts`); `BENCHMARKS.md` is authored here and is later read by `tools/rule-coverage` (GAT15RIVLEDTEX-015 `Deps` this ticket).
4. FOUNDATIONS §2 + §11 determinism motivate this ticket: simulation/benchmarks drive the Rust engine and decide no behavior; seeded playouts are deterministic; thresholds are realistic, variance-aware, and must not incentivize a lookup-table evaluator or weaken explanation fidelity.

## Architecture Check

1. Adding `--seat-count` and a seat-count-aware runner that uses the existing generic `Summary` helpers extends the simulator additively without touching other games' two-seat runners.
2. No backwards-compatibility aliasing/shims — additive flag + new runner; existing flags unchanged.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4); no lookup-table evaluator (spec out-of-scope).

## Verification Layers

1. `--seat-count {3,4,5,6}` drives seeded playouts with seat-keyed summaries -> `cargo run -p simulate -- --game river_ledger --seat-count N` for each N.
2. Benches cover setup/legal-action/apply/projection/evaluator/replay/full-playout -> `cargo bench -p river_ledger`.
3. Summary uses the seat-keyed `wins_by_seat` map, not two-seat assumptions -> simulate output inspection for 6 seats.
4. `BENCHMARKS.md` records native + WASM-smoke results -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `tools/simulate`

Add `river_ledger` dependency; add a `--seat-count` flag; add `run_river_ledger_simulation` building an N-seat `wins_by_seat` summary via `Summary::new(seats)` / `increment_seat_count`.

### 2. Benches + doc

Author `games/river_ledger/benches/river_ledger.rs` + `benches/thresholds.json` (setup/deal, legal-action generation, apply, projection per viewer, replay export/import, full playout, evaluator showdown batch incl. 6×21 candidates) and `docs/BENCHMARKS.md`.

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/simulate/Cargo.toml` (modify)
- `games/river_ledger/benches/river_ledger.rs` (new)
- `games/river_ledger/benches/thresholds.json` (new)
- `games/river_ledger/docs/BENCHMARKS.md` (new)

## Out of Scope

- `replay-check`/`fixture-check`/`rule-coverage` registration (GAT15RIVLEDTEX-015).
- WASM/web exposure (016/017); `ci/games.json` (018).
- CI floor tuning that hides regressions; lookup-table evaluator.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game river_ledger --seat-count 3 --games 1000 --start-seed 1503` (and N=4/5/6) — seeded seat-keyed summaries.
2. `cargo bench -p river_ledger` — all benchmark lanes run.
3. `cargo check --workspace` — simulator change compiles workspace-wide.

### Invariants

1. Simulation/benchmarks decide no behavior; seeded playouts are deterministic (§2/§11).
2. Summaries are seat-keyed across 3–6 seats; no two-seat assumption for River Ledger (§11).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/benches/river_ledger.rs` + `thresholds.json` (new) — native benchmark lanes.
2. `tools/simulate/src/main.rs` (modify) — seat-count-aware River Ledger runner.

### Commands

1. `for n in 3 4 5 6; do cargo run -p simulate -- --game river_ledger --seat-count $n --games 1000 --start-seed 150$n; done`
2. `cargo bench -p river_ledger && cargo check --workspace`
3. The simulate + bench commands are the correct boundary; rule-coverage consumption of `BENCHMARKS.md` is validated in GAT15RIVLEDTEX-015.

## Outcome

Completed: 2026-06-14

Summary:

- Added River Ledger to `tools/simulate` with a `--seat-count` flag, 3-6 seat validation, and a seat-keyed `wins_by_seat` summary built through the generic `Summary::new` / `increment_seat_count` helpers.
- Added seeded River Ledger simulator playouts using the Rust legal-action tree, Level 2 bot decisions, command validation, and Rust-owned action application.
- Added native River Ledger benchmark registration plus benchmark lanes for setup/deal, legal actions, apply, all-viewer projection, public replay export/import, evaluator batches, and Level 2 full playouts.
- Added provisional benchmark thresholds and `games/river_ledger/docs/BENCHMARKS.md`.

Deviations:

- Benchmark thresholds are smoke floors pending repeated CI hardware baselines, as recorded in `thresholds.json` and `BENCHMARKS.md`.
- Replay/fixture/rule-coverage registration remains out of scope for GAT15RIVLEDTEX-015.
- Pre-existing unrelated `.claude/skills/spec-to-tickets/*` worktree edits were left untouched and unstaged.

Verification:

- `cargo run -p simulate -- --game river_ledger --seat-count 3 --games 1 --start-seed 1503`
- `for n in 3 4 5 6; do cargo run -p simulate -- --game river_ledger --seat-count $n --games 1000 --start-seed 150$n; done`
- `cargo bench -p river_ledger`
- `cargo check --workspace`
- `cargo fmt --all --check`
- `node scripts/check-doc-links.mjs`
- `bash scripts/boundary-check.sh`
- `git diff --check`
