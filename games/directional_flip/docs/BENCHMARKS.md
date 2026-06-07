# Directional Flip Benchmark Notes

Game ID: `directional_flip`

Rules version: `directional_flip-rules-v1`

Last updated: 2026-06-07

## Benchmark surfaces

The native harness lives at `games/directional_flip/benches/directional_flip.rs`; thresholds live at `games/directional_flip/benches/thresholds.json`.

| Operation | Evidence | Threshold posture |
|---|---|---|
| legal actions | `legal_actions` | smoke floor, baseline pending |
| flip scanning | `flip_scanning` | smoke floor, baseline pending |
| apply action | `apply_action` | smoke floor, baseline pending |
| public view generation | `public_view_generation` | smoke floor, baseline pending |
| replay step projection | `replay_step_projection` | smoke floor, baseline pending |
| serialization round trip | `serialization_roundtrip` | smoke floor, baseline pending |
| replay throughput | `replay_throughput` | smoke floor, baseline pending |
| random playout | `random_playout` | smoke floor, baseline pending |
| Level 0 bot decision | `level0_bot_decision` | smoke floor, baseline pending |
| Level 2-lite bot decision | `level2_bot_decision` | smoke floor, baseline pending |

## Threshold posture

`thresholds.json` intentionally uses a threshold of `1` for every operation with
`baseline_pending_non_blocking` rationale. This is a smoke floor, not a
performance claim. Directional Flip does not yet have stable CI measurements,
CPU isolation, or a repeated-run baseline, so this ticket records measured local
values without inventing a public throughput target.

Future threshold tightening should use repeated CI evidence and the existing
ADR-0002 benchmark-gating semantics.

## First local full measurements

Command: `cargo bench -p directional_flip`

Environment: linux x86_64, rustc 1.93.0 (254b59607 2026-01-19), local native benchmark run with no CPU pinning, thermal isolation, or hardware probe.

| Operation | Iterations | Measured current | Threshold |
|---|---:|---:|---:|
| `legal_actions` | 50,000 | 167,410.46 trees/sec | 1 |
| `flip_scanning` | 50,000 | 409,410.53 scans/sec | 1 |
| `apply_action` | 50,000 | 98,150.45 actions/sec | 1 |
| `public_view_generation` | 25,000 | 81,485.80 views/sec | 1 |
| `replay_step_projection` | 25,000 | 29,951.02 projections/sec | 1 |
| `serialization_roundtrip` | 25,000 | 527,944.05 roundtrips/sec | 1 |
| `replay_throughput` | 5,000 | 5,561.45 replays/sec | 1 |
| `random_playout` | 1,000 | 901.35 games/sec | 1 |
| `level0_bot_decision` | 25,000 | 165,265.10 decisions/sec | 1 |
| `level2_bot_decision` | 5,000 | 18,871.59 decisions/sec | 1 |

## Legal-actions smoke measurement

Command: `cargo bench -p directional_flip -- legal_actions`

Environment: same local machine and toolchain as the full run.

| Operation | Iterations | Measured current | Threshold |
|---|---:|---:|---:|
| `legal_actions` | 50,000 | 121,506.77 trees/sec | 1 |

## Verification commands

- `cargo bench -p directional_flip -- legal_actions`
- `cargo bench -p directional_flip`
- Extract the marked JSON between `BEGIN_DIRECTIONAL_FLIP_BENCHMARK_JSON` and `END_DIRECTIONAL_FLIP_BENCHMARK_JSON`.
- `cargo run -p bench-report -- --input <directional_flip_bench.json> --thresholds games/directional_flip/benches/thresholds.json` after bench-report registration lands.
