# High Card Duel Benchmark Notes

Game ID: `high_card_duel`

Rules version: `high-card-duel-rules-v1`

Last updated: 2026-06-07

## Benchmark Surfaces

The native harness lives at `games/high_card_duel/benches/high_card_duel.rs`; thresholds live at `games/high_card_duel/benches/thresholds.json`.

| Operation | Evidence | Threshold posture |
|---|---|---|
| setup plus deterministic shuffle | `standard_setup_shuffle` | smoke floor, baseline pending |
| lead legal action tree | `legal_actions_lead` | smoke floor, baseline pending |
| reply legal action tree | `legal_actions_reply` | smoke floor, baseline pending |
| command validation | `validate_commit` | smoke floor, baseline pending |
| lead commit apply | `apply_commit` | smoke floor, baseline pending |
| reply commit, reveal, score, refill apply | `apply_reveal_refill` | smoke floor, baseline pending |
| observer public view generation | `public_view_generation` | smoke floor, baseline pending |
| seat-private view generation | `seat_private_view_generation` | smoke floor, baseline pending |
| effect log filtering | `effect_filtering` | smoke floor, baseline pending |
| public observer replay export | `public_replay_export` | smoke floor, baseline pending |
| internal replay reconstruction | `internal_replay_reconstruction` | smoke floor, baseline pending |
| stable serialization | `serialization` | smoke floor, baseline pending |
| random legal playout | `random_playout` | smoke floor, baseline pending |
| Level 0 bot decision | `level0_bot_decision` | smoke floor, baseline pending |

## Threshold Posture

`thresholds.json` intentionally uses a threshold of `1` for every operation with
`baseline_pending_non_blocking` rationale. This is a smoke floor, not a
performance claim. High Card Duel does not yet have stable CI measurements,
CPU isolation, or a repeated-run baseline, so the PR lane compiles and runs the
native bench harness without making a runner-specific throughput assertion.

Future threshold tightening should use repeated CI evidence and the existing
ADR-0002 and ADR-0003 benchmark-gating semantics.

## First Local Full Measurements

Command: `cargo bench -p high_card_duel`

Environment: linux x86_64, rustc 1.93.0 (254b59607 2026-01-19), local native benchmark run with no CPU pinning, thermal isolation, or hardware probe.

| Operation | Iterations | Measured current | Threshold |
|---|---:|---:|---:|
| `standard_setup_shuffle` | 20,000 | 3,122,463.00 setups/sec | 1 |
| `legal_actions_lead` | 50,000 | 981,575.78 trees/sec | 1 |
| `legal_actions_reply` | 50,000 | 985,530.40 trees/sec | 1 |
| `validate_commit` | 50,000 | 52,859,710.33 validations/sec | 1 |
| `apply_commit` | 50,000 | 6,468,388.98 actions/sec | 1 |
| `apply_reveal_refill` | 50,000 | 3,843,492.67 actions/sec | 1 |
| `public_view_generation` | 50,000 | 5,177,537.77 views/sec | 1 |
| `seat_private_view_generation` | 50,000 | 1,931,702.64 views/sec | 1 |
| `effect_filtering` | 50,000 | 69,280,864.63 filters/sec | 1 |
| `public_replay_export` | 5,000 | 24,570.69 exports/sec | 1 |
| `internal_replay_reconstruction` | 5,000 | 55,873.79 replays/sec | 1 |
| `serialization` | 25,000 | 33,965.26 serializations/sec | 1 |
| `random_playout` | 5,000 | 58,572.84 games/sec | 1 |
| `level0_bot_decision` | 50,000 | 854,261.28 decisions/sec | 1 |

## Legal-Actions Smoke Measurement

Command: `cargo bench -p high_card_duel -- legal_actions`

Environment: same local machine and toolchain as the full run.

| Operation | Iterations | Measured current | Threshold |
|---|---:|---:|---:|
| `legal_actions_lead` | 50,000 | 967,855.56 trees/sec | 1 |
| `legal_actions_reply` | 50,000 | 975,137.85 trees/sec | 1 |

## Verification Commands

- `cargo bench -p high_card_duel`
- `cargo bench -p high_card_duel -- legal_actions`
- `cargo run -p bench-report -- --game high_card_duel --input <high_card_duel_benchmark_report.txt>`
