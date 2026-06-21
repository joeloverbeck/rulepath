# GAT17VOWTIDOHHEL-013: Simulator registration and seeded simulations by seat count

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — modifies `tools/simulate/src/main.rs`
**Deps**: 012

## Problem

Register `vow_tide` in the `simulate` tool with a `--seat-count 3..=7` (default 4) dispatch, and produce deterministic seeded L0/L1 playouts for every seat count with seat-keyed summaries (wins/co-wins, exact-bid rates, average actions/hands, hook exclusions, completion). The simulator arm is the consumer of the 012 bots, so it lands with them.

## Assumption Reassessment (2026-06-21)

1. `tools/simulate/src/main.rs` is a hard-coded per-game registry: `GAME_*` constants (~`:52`), a validation check (~`:222`), and a `run_simulation()` dispatch (~`:340`, e.g. `run_briar_circuit_simulation`). `--seat-count` already exists (`river_ledger` allows 3–6 at `:226`; `briar_circuit` pins 4 at `:223`) — vow_tide adds a 3–7 constraint.
2. Spec §4.5 (simulator row) + §7.1 fix the per-seat-count commands (`--seat-count {3..7} --games 1000 --start-seed 170N00 --action-cap 2048`) and require ≥1 all-L0 and ≥1 all-L1/mixed run per count; a cap hit is a reproducible failure, never a draw.
3. Cross-artifact boundary under audit: the simulator consumes the 012 bot policies + 008/009 legal/terminal behavior and emits the seat-keyed machine-readable summary; its determinism (seed→identical stream) is under audit.
4. FOUNDATIONS §2 (deterministic simulation) is the principle under audit; the reassessment-resolved finding that `simulate` is a hard-coded registry (not generic) is the change rationale.

## Architecture Check

1. Placing the `simulate` arm with the bots ticket's consumer scope (rather than a generic tool bundle) matches the official-game pattern and keeps the seeded-simulation smoke runnable as soon as bots land.
2. No shims; an additive dispatch arm mirroring `run_briar_circuit_simulation`.
3. `engine-core` untouched; no `game-stdlib` change.

## Verification Layers

1. 3–7 seeded L0/L1 playouts complete with no cap hits → `cargo run -p simulate -- --game vow_tide --seat-count {N} --games 1000 ...`.
2. Seat-keyed summary fields present (wins/co-wins/exact-rate/avg actions/hands/version/seed) → inspect simulator output.
3. Deterministic rerun → same seed → identical summary.
4. Bots use the normal legal path in simulation → bot-legality (covered in 012) + simulation completion.

## What to Change

### 1. Simulator registration + seat constraint

Add `const GAME_VOW_TIDE`, the validation entry, and a `run_vow_tide_simulation()` dispatch accepting `--seat-count 3..=7` (default 4); L0/L1 bot dispatch; seat-keyed wins/co-wins/exact-bid rates/avg actions+hands/hook exclusions/completion, with reproducible cap-failure output (seed, seat count, command stream, phase, hashes, reason).

## Files to Touch

- `tools/simulate/src/main.rs` (modify)

## Out of Scope

- `replay-check`/`fixture-check`/`rule-coverage` arms (015/016); bot docs (014).
- `ci/games.json` sim_flags row (added with CI wiring in 019).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game vow_tide --seat-count 3 --games 1000 --start-seed 170300 --action-cap 2048` — completes, seat-keyed summary, no cap hits.
2. The same for `--seat-count {4,5,6,7}` with the §7.1 seeds.
3. `cargo build -p simulate` — registry compiles.

### Invariants

1. Identical seed + seat count reproduces an identical simulation summary.
2. A cap hit is reported as a reproducible failure, never counted as a completion or draw.

## Test Plan

### New/Modified Tests

1. `tools/simulate/src/main.rs` — the new dispatch arm (exercised by the §7.1 command matrix; no separate unit test file).

### Commands

1. `cargo run -p simulate -- --game vow_tide --seat-count 7 --games 1000 --start-seed 170700 --action-cap 2048`
2. `for n in 3 4 5 6 7; do cargo run -p simulate -- --game vow_tide --seat-count $n --games 1000 --start-seed 170${n}00 --action-cap 2048; done`
3. Narrower command rationale: the seat-count matrix run is the determinism+completion boundary; bot legality is proven natively in 012.

## Outcome

Completed 2026-06-21. Registered `vow_tide` in `tools/simulate` with `--seat-count 3..=7` validation and default seat count 4. The simulator now runs deterministic mixed L1/L0 bot playouts through normal legal leaves and Rust validation/application, emits seat-keyed wins/co-wins/exact-bid rates plus average actions/hands, hook exclusions, completion rate, and reports action-cap failures as reproducible failures with seed, seat count, command stream, phase, and replay snapshot hashes. Vow Tide summary output intentionally omits wall-clock throughput so identical seed + seat count produces identical stdout.

Verification:

1. `cargo fmt --all --check` passed.
2. `cargo build -p simulate` passed.
3. `cargo run -p simulate -- --game vow_tide --seat-count 3 --games 1000 --start-seed 170300 --action-cap 2048` passed with `action_cap_failures=0`.
4. `cargo run -p simulate -- --game vow_tide --seat-count 4 --games 1000 --start-seed 170400 --action-cap 2048` passed with `action_cap_failures=0`.
5. `cargo run -p simulate -- --game vow_tide --seat-count 5 --games 1000 --start-seed 170500 --action-cap 2048` passed with `action_cap_failures=0`.
6. `cargo run -p simulate -- --game vow_tide --seat-count 6 --games 1000 --start-seed 170600 --action-cap 2048` passed with `action_cap_failures=0`.
7. `cargo run -p simulate -- --game vow_tide --seat-count 7 --games 1000 --start-seed 170700 --action-cap 2048` passed with `action_cap_failures=0`.
8. Direct same-seed stdout comparison for seat count 4, games 1000, start seed 170400 passed.
