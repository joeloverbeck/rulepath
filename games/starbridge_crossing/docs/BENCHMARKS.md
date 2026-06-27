# Starbridge Crossing Benchmarks

Game ID: `starbridge_crossing`

Variant: `starbridge_crossing_classic_star_v1`

Rules version: `starbridge-crossing-rules-v1`

Date: 2026-06-27

## Scope

Native Rust is the benchmark source of truth. The game-local crate cannot depend
back on `wasm-api` without creating a dependency cycle, so
`wasm_public_view_bridge_6p` measures the Rust public-view projection and stable
serialization that the WASM bridge consumes.

The benchmark harness covers the large-board pressure required by Gate 20:

- setup over the 121-space topology at 2, 3, 4, and 6 seats;
- legal action tree generation at opening and midgame max-seat states;
- jump-chain path enumeration from a prepared jump-bearing state;
- Rust-validated single-step, jump, and blocked-pass command application;
- bounded Level 0 max-seat playout throughput;
- all-public view serialization;
- deterministic replay of a max-seat trace;
- renderer-facing public-view projection and serialization.

Thresholds in [thresholds.json](../benches/thresholds.json) are provisional
native smoke floors under ADR 0003 and ADR 0005 benchmark policy until repeated
CI-runner baselines are available. Calibration may replace a provisional floor
only from measured evidence, and may not remove a workload, bypass Rust legal
validation, weaken replay determinism, or hide large-board pressure.

## Native Evidence

Run:

```bash
cargo bench -p starbridge_crossing
```

The benchmark executable prints a CSV-style summary plus a JSON block delimited
by `BEGIN_STARBRIDGE_CROSSING_BENCHMARK_JSON` and
`END_STARBRIDGE_CROSSING_BENCHMARK_JSON`.

Current native lanes:

- `setup_121_spaces_2p`
- `setup_121_spaces_3p`
- `setup_121_spaces_4p`
- `setup_121_spaces_6p`
- `legal_actions_start_6p`
- `legal_actions_midgame_6p`
- `jump_chain_enumeration_dense_6p`
- `apply_single_step_6p`
- `apply_multi_hop_6p`
- `apply_blocked_pass_6p`
- `simulate_l0_6p_64_actions`
- `serialize_public_view_6p`
- `replay_full_trace_6p`
- `wasm_public_view_bridge_6p`

## Profile Targets

| Profile | Operation | Provisional target | Current threshold file posture |
|---|---|---:|---|
| `setup_121_spaces_2p` | Construct 2-seat state over 121 spaces. | p95 under 2 ms after calibration. | Smoke floor: 1 setup/s. |
| `setup_121_spaces_3p` | Construct 3-seat state over 121 spaces. | p95 under 2 ms after calibration. | Smoke floor: 1 setup/s. |
| `setup_121_spaces_4p` | Construct 4-seat state over 121 spaces. | p95 under 3 ms after calibration. | Smoke floor: 1 setup/s. |
| `setup_121_spaces_6p` | Construct max-seat state over 121 spaces. | p95 under 4 ms after calibration. | Smoke floor: 1 setup/s. |
| `legal_actions_start_6p` | Generate the opening max-seat action tree. | p95 under 3 ms after calibration. | Smoke floor: 1 tree/s. |
| `legal_actions_midgame_6p` | Generate a midgame max-seat action tree. | p95 under 6 ms after calibration. | Smoke floor: 1 tree/s. |
| `jump_chain_enumeration_dense_6p` | Collect legal jump paths from a prepared jump-bearing state. | p95 under 8 ms after calibration. | Smoke floor: 1 tree/s. |
| `apply_single_step_6p` | Apply a Rust-validated single-step command. | p95 under 2 ms after calibration. | Smoke floor: 1 action/s. |
| `apply_multi_hop_6p` | Apply a Rust-validated jump command. | p95 under 4 ms after calibration. | Smoke floor: 1 action/s. |
| `apply_blocked_pass_6p` | Apply a blocked-pass command for an active seat with no legal move. | p95 under 2 ms after calibration. | Smoke floor: 1 action/s. |
| `simulate_l0_6p_64_actions` | Run a bounded 64-action max-seat Level 0 playout. | p95 under 250 ms after calibration. | Smoke floor: 1 playout/s. |
| `serialize_public_view_6p` | Stable-serialize the all-public max-seat view. | p95 under 4 ms after calibration. | Smoke floor: 1 view/s. |
| `replay_full_trace_6p` | Replay a deterministic 24-command max-seat trace. | p95 under 40 ms after calibration. | Smoke floor: 1 replay/s. |
| `wasm_public_view_bridge_6p` | Project and serialize the Rust public view consumed by WASM. | p95 under 5 ms after calibration. | Smoke floor: 1 view/s. |

## Seed And Fixture Manifest

| Workload family | Seed/fixture source | Notes |
|---|---|---|
| seat-count setup | seeds `20000+` | deterministic setup; Starbridge setup uses no stochastic game events. |
| opening action tree | seed `20100` | max-seat opening state. |
| midgame action tree | seed `20200`, 48 L0 actions | deterministic bounded playout, stops early if terminal. |
| jump-chain enumeration | seed `20300`, seek cap 256 actions | deterministic L0 playout until Rust exposes a jump path. |
| single-step apply | seeds `20400+` | first Rust-supplied `step` leaf from opening max-seat states. |
| jump apply | seed `20500`, prepared jump state | first Rust-supplied `jump` leaf from the prepared state. |
| blocked pass apply | seed `20600`, fixture-local no-peg active seat | synthetic blocked active-seat state for pass validation only. |
| Level 0 playout | seeds `20700+`, cap 64 | bot selects from Rust legal action trees; no MCTS, ML, RL, or hidden-state sampling. |
| public view serialization | seed `20800`, 32 L0 actions | all-public observer view. |
| replay trace | seed `20900`, 24 L0 commands | command list replayed through `replay_commands`. |
| WASM bridge surrogate | seed `21000`, 32 L0 actions | Rust projection/serialization consumed by the bridge; browser e2e lands separately. |

## Provisional Posture

This ticket records smoke thresholds rather than CI-calibrated floors. The
benchmarks keep large-board setup, move generation, jump enumeration, command
application, replay, serialization, and renderer-facing projection visible.
Future threshold calibration must cite repeated CI-runner evidence and keep the
same operation names unless a migration note updates both this document and
`thresholds.json`.

## CI Strategy

Pull requests may run a filtered smoke lane for compilation and basic execution:

```bash
cargo bench -p starbridge_crossing -- legal_actions_start_6p
```

Scheduled, workflow-dispatch, and main-branch performance gates can run the full
native lane:

```bash
cargo bench -p starbridge_crossing
cargo run -p bench-report -- --input <report> --thresholds games/starbridge_crossing/benches/thresholds.json
```

The threshold file records benchmark policy data consumed by bench tooling. It
is not rule behavior and does not affect deterministic game state.
