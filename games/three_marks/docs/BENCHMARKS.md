# Three Marks Benchmark Notes

Game ID: `three_marks`

Rules version: `three_marks-rules-v1`

Last updated: 2026-06-08

## Benchmark surfaces

The native harness lives at `games/three_marks/benches/three_marks.rs`;
thresholds live at `games/three_marks/benches/thresholds.json`.

Per [ADR 0003](../../../docs/adr/0003-ci-calibrated-benchmark-thresholds.md)
and [ADR 0005](../../../docs/adr/0005-variance-aware-ci-benchmark-floors.md),
the committed threshold is the enforced variance-aware `ubuntu-latest` CI floor:
at least 15% below the minimum observed across representative CI runs. Faster
native targets and known target misses remain documented here.

| Operation | Evidence | Threshold posture |
|---|---|---|
| legal actions | `legal_actions` | measured baseline floor |
| apply action | `apply_action` | measured baseline floor |
| public view generation | `public_view_generation` | variance-aware CI floor; native baseline preserved |
| replay step projection | `replay_step_projection` | conservative CI floor |
| serialization round trip | `serialization_roundtrip` | measured baseline floor |
| replay throughput | `replay_throughput` | measured baseline floor |
| random playout | `random_playout` | CI floor; native Stage 2 target miss remains visible |
| Level 0 bot decision | `level0_bot_decision` | measured baseline floor |
| Level 1 bot decision | `level1_bot_decision` | variance-aware CI floor; native baseline preserved |

## Random-playout target miss

Gate 4 carried a visible 300,000+ games/sec expectation for tiny native random
playouts. The first full local Three Marks run measured below that target, and
the representative CI runs `27101213584`/`27111487150`/`27114668009` measured
`random_playout` at a minimum of 34,133.55 games/sec on `ubuntu-latest`.

This is not a silent target change. ADR 0005 makes the committed 29,000 games/sec
threshold the variance-aware CI gate floor while this file preserves the 300,000+
games/sec native target as the aspirational benchmark and keeps the miss visible.
The earlier single-sample ADR 0003 floor from run `27087214359` is superseded for
this operation.

## CI-calibrated floors

| Operation | Native target/baseline | CI evidence | Enforced CI floor |
|---|---:|---:|---:|
| `public_view_generation` | 200,000 views/sec native baseline floor | minimum 196,177.62 views/sec across runs `27101213584`/`27111487150`/`27114668009` | 165,000 views/sec |
| `random_playout` | 300,000+ games/sec native target | minimum 34,133.55 games/sec across runs `27101213584`/`27111487150`/`27114668009` | 29,000 games/sec |
| `level1_bot_decision` | 35,000 decisions/sec native baseline floor | minimum 34,965.59 decisions/sec across runs `27101213584`/`27111487150`/`27114668009` | 29,000 decisions/sec |

## Verification commands

- `cargo bench -p three_marks`
- Extract the marked JSON between `BEGIN_THREE_MARKS_BENCHMARK_JSON` and `END_THREE_MARKS_BENCHMARK_JSON`.
- `cargo run -p bench-report -- --input <three_marks_bench.json> --thresholds games/three_marks/benches/thresholds.json`
