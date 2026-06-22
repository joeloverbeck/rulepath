# Crest Ledger Benchmark Notes

Game ID: `poker_lite`

Rules version: `poker-lite-rules-v1`

Last updated: 2026-06-09

## Benchmark Surfaces

The native harness lives at `games/poker_lite/benches/poker_lite.rs`;
thresholds live at `games/poker_lite/benches/thresholds.json`.

| Operation | Evidence | Threshold posture |
|---|---|---|
| setup, shuffle, and deal | `setup_shuffle_deal` | smoke floor, baseline pending |
| initial pledge legal action tree | `legal_actions_initial_pledge` | smoke floor, baseline pending |
| validate press command | `validate_press` | smoke floor, baseline pending |
| apply press command | `apply_press` | smoke floor, baseline pending |
| observer view projection | `project_observer_view` | smoke floor, baseline pending |
| viewer-scoped public export and import | `public_export_import` | smoke floor, baseline pending |
| terminal state hash | `state_hash_terminal` | smoke floor, baseline pending |
| Level 2 bot decision | `level2_bot_decision` | smoke floor, baseline pending |
| full Level 2 hand playout | `level2_full_playout` | provisional `>= 2,000` hands/sec floor |

## Threshold Posture

Most operations intentionally use a threshold of `1` with
`baseline_pending_non_blocking` rationale. These are smoke floors, not calibrated
throughput claims.

`level2_full_playout` uses a provisional `2,000` completed hands/sec floor so
Gate 2 can catch severe regressions in the native hand loop while Rulepath still
lacks stable repeated measurements for Crest Ledger on benchmark runners.

Follow-up calibration: collect repeated Gate 2 CI measurements after
`poker_lite` is registered in the tool lanes, document the runner environment,
and tighten thresholds under ADR-0002, ADR-0003, and accepted ADR-0005
benchmark-gating semantics. This document and the threshold manifest remain
provisional until those repeated measurements exist.

## Verification Commands

- `cargo bench -p poker_lite -- legal_actions`
- `cargo bench -p poker_lite`
- `cargo run -p bench-report -- --input <poker_lite_benchmark_report.txt> --thresholds games/poker_lite/benches/thresholds.json`
