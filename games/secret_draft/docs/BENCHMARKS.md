# Veiled Draft Benchmark Notes

Game ID: `secret_draft`

Rules version: `secret-draft-rules-v1`

Last updated: 2026-06-08

## Benchmark Surfaces

The native harness lives at `games/secret_draft/benches/secret_draft.rs`;
thresholds live at `games/secret_draft/benches/thresholds.json`.

| Operation | Evidence | Threshold posture |
|---|---|---|
| initial pool legal action tree | `legal_actions_initial_pool` | smoke floor, baseline pending |
| legal action tree after one hidden commit | `legal_actions_after_one_commit` | smoke floor, baseline pending |
| validate commit command | `validate_commit` | smoke floor, baseline pending |
| apply first hidden commit | `apply_first_commit` | smoke floor, baseline pending |
| apply second commit and resolve reveal | `apply_second_commit_resolve_reveal` | smoke floor, baseline pending |
| pending public view projection | `project_public_view_pending` | smoke floor, baseline pending |
| post-reveal public view projection | `project_public_view_after_reveal` | smoke floor, baseline pending |
| terminal state hash | `state_hash_terminal` | smoke floor, baseline pending |
| viewer-scoped public export timeline | `public_export_timeline` | smoke floor, baseline pending |
| Level 1 bot decision | `level1_bot_decision` | smoke floor, baseline pending |

## Threshold Posture

`thresholds.json` intentionally uses a threshold of `1` for every operation with
`baseline_pending_non_blocking` rationale. These are smoke floors, not
performance claims. Veiled Draft does not yet have stable CI measurements, CPU
isolation, or a repeated-run baseline, so the gate lane compiles and runs the
native bench harness without asserting runner-specific throughput.

Follow-up calibration: collect repeated Gate 2 CI measurements after
`secret_draft` is registered in the tool lanes, document the runner environment,
and tighten thresholds under ADR-0002, ADR-0003, and ADR-0005 benchmark-gating
semantics.

## Verification Commands

- `cargo bench -p secret_draft -- legal_actions`
- `cargo bench -p secret_draft`
- `cargo run -p bench-report -- --input <secret_draft_benchmark_report.txt> --thresholds games/secret_draft/benches/thresholds.json`
