# Draughts Lite Benchmark Notes

Game ID: `draughts_lite`

Rules version: `draughts_lite-rules-v1`

Last updated: 2026-06-07

## Benchmark surfaces

The native harness lives at `games/draughts_lite/benches/draughts_lite.rs`; thresholds live at `games/draughts_lite/benches/thresholds.json`.

| Operation | Evidence | Threshold posture |
|---|---|---|
| standard setup | `standard_setup` | smoke floor, baseline pending |
| initial legal action tree | `legal_actions_initial` | smoke floor, baseline pending |
| midgame no-capture legal action tree | `legal_actions_midgame_no_capture` | smoke floor, baseline pending |
| mandatory-capture legal action tree | `legal_actions_mandatory_capture` | smoke floor, baseline pending |
| capture-rich multi-jump legal action tree | `legal_actions_multi_jump` | smoke floor, baseline pending |
| validate/apply quiet move | `validate_apply_quiet` | smoke floor, baseline pending |
| validate/apply single capture | `validate_apply_single_capture` | smoke floor, baseline pending |
| validate/apply multi-jump capture | `validate_apply_multi_jump` | smoke floor, baseline pending |
| public view generation | `public_view_generation` | smoke floor, baseline pending |
| replay throughput | `replay_throughput` | smoke floor, baseline pending |
| Level 0 bot decision | `level0_bot_decision` | smoke floor, baseline pending |
| Level 1 bot decision | `level1_bot_decision` | smoke floor, baseline pending |

## Threshold posture

`thresholds.json` intentionally uses a threshold of `1` for every operation with
`baseline_pending_non_blocking` rationale. This is a smoke floor, not a
performance claim. Draughts Lite does not yet have stable CI measurements, CPU
isolation, or a repeated-run baseline, so the PR lane compiles and runs the
native bench harness without making a runner-specific throughput assertion.

Future threshold tightening should use repeated CI evidence and the existing
ADR-0002 and ADR-0003 benchmark-gating semantics.

## First local full measurements

Command: `cargo bench -p draughts_lite -- --test`

Environment: linux x86_64, rustc 1.93.0 (254b59607 2026-01-19), local native benchmark run with no CPU pinning, thermal isolation, or hardware probe.

| Operation | Iterations | Measured current | Threshold |
|---|---:|---:|---:|
| `standard_setup` | 20,000 | 3,969,199.02 setups/sec | 1 |
| `legal_actions_initial` | 50,000 | 74,061.50 trees/sec | 1 |
| `legal_actions_midgame_no_capture` | 50,000 | 162,329.70 trees/sec | 1 |
| `legal_actions_mandatory_capture` | 50,000 | 500,547.60 trees/sec | 1 |
| `legal_actions_multi_jump` | 50,000 | 280,953.49 trees/sec | 1 |
| `validate_apply_quiet` | 50,000 | 529,454.62 actions/sec | 1 |
| `validate_apply_single_capture` | 50,000 | 840,362.97 actions/sec | 1 |
| `validate_apply_multi_jump` | 25,000 | 680,842.72 actions/sec | 1 |
| `public_view_generation` | 25,000 | 68,697.70 views/sec | 1 |
| `replay_throughput` | 5,000 | 3,038.21 replays/sec | 1 |
| `level0_bot_decision` | 25,000 | 69,232.48 decisions/sec | 1 |
| `level1_bot_decision` | 10,000 | 148,024.69 decisions/sec | 1 |

## Legal-actions smoke measurement

Command: `cargo bench -p draughts_lite legal_actions`

Environment: same local machine and toolchain as the full run.

| Operation | Iterations | Measured current | Threshold |
|---|---:|---:|---:|
| `legal_actions_initial` | 50,000 | 73,831.46 trees/sec | 1 |
| `legal_actions_midgame_no_capture` | 50,000 | 166,185.50 trees/sec | 1 |
| `legal_actions_mandatory_capture` | 50,000 | 508,139.90 trees/sec | 1 |
| `legal_actions_multi_jump` | 50,000 | 271,722.15 trees/sec | 1 |

## Verification commands

- `cargo bench -p draughts_lite -- --test`
- `cargo bench -p draughts_lite legal_actions`
- `cargo run -p bench-report -- --game draughts_lite --input <draughts_lite_bench.json>`
