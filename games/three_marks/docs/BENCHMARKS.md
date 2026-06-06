# Three Marks Benchmark Notes

Game ID: `three_marks`

Rules version: `three_marks-rules-v1`

Last updated: 2026-06-06

## Benchmark surfaces

The native harness lives at `games/three_marks/benches/three_marks.rs`; thresholds live at `games/three_marks/benches/thresholds.json`.

| Operation | Evidence | Threshold posture |
|---|---|---|
| legal actions | `legal_actions` | measured baseline floor |
| apply action | `apply_action` | measured baseline floor |
| public view generation | `public_view_generation` | measured baseline floor |
| replay step projection | `replay_step_projection` | conservative CI floor |
| serialization round trip | `serialization_roundtrip` | measured baseline floor |
| replay throughput | `replay_throughput` | measured baseline floor |
| random playout | `random_playout` | provisional measured baseline; ADR follow-up required |
| Level 0 bot decision | `level0_bot_decision` | measured baseline floor |
| Level 1 bot decision | `level1_bot_decision` | measured baseline floor |

## Random-playout target miss

Gate 4 carried a visible 300,000+ games/sec expectation for tiny native random playouts. The first full local Three Marks run measured below that target and `thresholds.json` records a provisional 50,000 games/sec floor with `measured_baseline_adr_followup_required`.

This is not a silent target change. Any permanent recalibration needs ADR discipline or a later accepted performance ticket. Until then, the threshold protects against regression from the measured baseline while keeping the target miss visible.

## Verification commands

- `cargo bench -p three_marks`
- Extract the marked JSON between `BEGIN_THREE_MARKS_BENCHMARK_JSON` and `END_THREE_MARKS_BENCHMARK_JSON`.
- `cargo run -p bench-report -- --input <three_marks_bench.json> --thresholds games/three_marks/benches/thresholds.json`
