# Three Marks Benchmark Notes

Game ID: `three_marks`

Rules version: `three_marks-rules-v1`

Last updated: 2026-06-07

## Benchmark surfaces

The native harness lives at `games/three_marks/benches/three_marks.rs`;
thresholds live at `games/three_marks/benches/thresholds.json`.

Per [ADR 0003](../../../docs/adr/0003-ci-calibrated-benchmark-thresholds.md),
the committed threshold is the enforced `ubuntu-latest` CI floor. Faster native
targets and known target misses remain documented here.

| Operation | Evidence | Threshold posture |
|---|---|---|
| legal actions | `legal_actions` | measured baseline floor |
| apply action | `apply_action` | measured baseline floor |
| public view generation | `public_view_generation` | measured baseline floor |
| replay step projection | `replay_step_projection` | conservative CI floor |
| serialization round trip | `serialization_roundtrip` | measured baseline floor |
| replay throughput | `replay_throughput` | measured baseline floor |
| random playout | `random_playout` | CI floor; native Stage 2 target miss remains visible |
| Level 0 bot decision | `level0_bot_decision` | measured baseline floor |
| Level 1 bot decision | `level1_bot_decision` | measured baseline floor |

## Random-playout target miss

Gate 4 carried a visible 300,000+ games/sec expectation for tiny native random
playouts. The first full local Three Marks run measured below that target, and
the `BENCICAL-001` CI evidence run `27087214359` measured `random_playout` at
36,369.11 games/sec on `ubuntu-latest`.

This is not a silent target change. ADR 0003 makes the committed 35,000
games/sec threshold the CI gate floor while this file preserves the 300,000+
games/sec native target as the aspirational benchmark and keeps the miss
visible.

## CI-calibrated floors

| Operation | Native target/baseline | CI evidence | Enforced CI floor |
|---|---:|---:|---:|
| `random_playout` | 300,000+ games/sec target | 36,369.11 games/sec in run `27087214359` | 35,000 games/sec |

## Verification commands

- `cargo bench -p three_marks`
- Extract the marked JSON between `BEGIN_THREE_MARKS_BENCHMARK_JSON` and `END_THREE_MARKS_BENCHMARK_JSON`.
- `cargo run -p bench-report -- --input <three_marks_bench.json> --thresholds games/three_marks/benches/thresholds.json`
