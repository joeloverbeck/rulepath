# Column Four Benchmark Notes

Game ID: `column_four`

Rules version: `column_four-rules-v1`

Last updated: 2026-06-06

## Benchmark surfaces

The native harness lives at `games/column_four/benches/column_four.rs`; thresholds live at `games/column_four/benches/thresholds.json`.

| Operation | Evidence | Threshold posture |
|---|---|---|
| legal actions | `legal_actions` | measured baseline floor |
| apply action | `apply_action` | measured baseline floor |
| public view generation | `public_view_generation` | measured baseline floor |
| replay step projection | `replay_step_projection` | measured baseline floor |
| serialization round trip | `serialization_roundtrip` | measured baseline floor |
| replay throughput | `replay_throughput` | measured baseline floor |
| random playout | `random_playout` | provisional target miss; measured baseline floor |
| Level 0 bot decision | `level0_bot_decision` | measured baseline floor |
| Level 2 bot decision | `level2_bot_decision` | measured baseline floor |

## Random-playout target status

`docs/TESTING-REPLAY-BENCHMARKING.md` records a provisional 100,000+ games/sec expectation for Column Four random playouts. The first local native smoke run on 2026-06-06 measured about 16,272 games/sec for `random_playout`, so `thresholds.json` records a 10,000 games/sec measured floor. That is a target miss, not a target change.

The threshold protects against regression from the measured local baseline while Gate 5 finishes CI wiring and broader benchmark reporting. Any permanent recalibration needs the normal ADR or accepted follow-up discipline.

## First local smoke measurements

Command: `cargo bench -p column_four -- --test`

Environment: linux x86_64, rustc 1.93.0 (254b59607 2026-01-19), local native benchmark run with no CPU pinning, thermal isolation, or hardware probe.

| Operation | Measured current | Threshold |
|---|---:|---:|
| `legal_actions` | 912,751.88 trees/sec | 200,000 |
| `apply_action` | 417,607.15 actions/sec | 200,000 |
| `public_view_generation` | 240,963.51 views/sec | 100,000 |
| `replay_step_projection` | 56,038.49 projections/sec | 45,000 |
| `serialization_roundtrip` | 550,056.22 roundtrips/sec | 50,000 |
| `replay_throughput` | 5,425.07 replays/sec | 4,000 |
| `random_playout` | 16,271.89 games/sec | 10,000 |
| `level0_bot_decision` | 721,311.29 decisions/sec | 100,000 |
| `level2_bot_decision` | 5,451.72 decisions/sec | 4,000 |

## Verification commands

- `cargo bench -p column_four`
- Extract the marked JSON between `BEGIN_COLUMN_FOUR_BENCHMARK_JSON` and `END_COLUMN_FOUR_BENCHMARK_JSON`.
- `cargo run -p bench-report -- --input <column_four_bench.json> --thresholds games/column_four/benches/thresholds.json`
