# Column Four Benchmark Notes

Game ID: `column_four`

Rules version: `column_four-rules-v1`

Last updated: 2026-06-07

## Benchmark surfaces

The native harness lives at `games/column_four/benches/column_four.rs`;
thresholds live at `games/column_four/benches/thresholds.json`.

Per [ADR 0003](../../../docs/adr/0003-ci-calibrated-benchmark-thresholds.md),
the committed threshold is the enforced `ubuntu-latest` CI floor. Faster native
baselines and target misses remain documented here.

| Operation | Evidence | Threshold posture |
|---|---|---|
| legal actions | `legal_actions` | measured baseline floor |
| apply action | `apply_action` | measured baseline floor |
| public view generation | `public_view_generation` | measured baseline floor |
| replay step projection | `replay_step_projection` | CI floor; native baseline preserved |
| serialization round trip | `serialization_roundtrip` | measured baseline floor |
| replay throughput | `replay_throughput` | CI floor; native baseline preserved |
| random playout | `random_playout` | CI floor; native target miss remains visible |
| Level 0 bot decision | `level0_bot_decision` | measured baseline floor |
| Level 2 bot decision | `level2_bot_decision` | CI floor; native baseline preserved |

## Random-playout target status

`docs/TESTING-REPLAY-BENCHMARKING.md` records a provisional 100,000+ games/sec expectation for Column Four random playouts. The first local native smoke run on 2026-06-06 measured about 16,272 games/sec for `random_playout`, and the `BENCICAL-001` CI evidence run `27087214359` measured 9,700.09 games/sec on `ubuntu-latest`. That is a target miss, not a target change.

ADR 0003 makes the committed 9,000 games/sec threshold the CI gate floor while
this file preserves the 100,000+ games/sec native target as the aspirational
benchmark and keeps the miss visible.

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

## CI-calibrated floors

| Operation | Native target/baseline | CI evidence | Enforced CI floor |
|---|---:|---:|---:|
| `replay_step_projection` | 56,038.49 projections/sec native baseline | 34,175.00 projections/sec in run `27087214359` | 33,000 projections/sec |
| `replay_throughput` | 5,425.07 replays/sec native baseline | 3,334.35 replays/sec in run `27087214359` | 3,200 replays/sec |
| `random_playout` | 100,000+ games/sec native target; 16,271.89 games/sec native baseline | 9,700.09 games/sec in run `27087214359` | 9,000 games/sec |
| `level2_bot_decision` | 5,451.72 decisions/sec native baseline | 3,063.45 decisions/sec in run `27087214359` | 3,000 decisions/sec |

## Verification commands

- `cargo bench -p column_four`
- Extract the marked JSON between `BEGIN_COLUMN_FOUR_BENCHMARK_JSON` and `END_COLUMN_FOUR_BENCHMARK_JSON`.
- `cargo run -p bench-report -- --input <column_four_bench.json> --thresholds games/column_four/benches/thresholds.json`
