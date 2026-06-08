# Token Bazaar Benchmark Notes

Game ID: `token_bazaar`

Rules version: `token-bazaar-rules-v1`

Last updated: 2026-06-08

## Benchmark Surfaces

The native harness lives at `games/token_bazaar/benches/token_bazaar.rs`;
thresholds live at `games/token_bazaar/benches/thresholds.json`.

| Operation | Evidence | Threshold posture |
|---|---|---|
| standard setup | `standard_setup` | smoke floor, baseline pending |
| initial legal action tree | `legal_actions_initial` | smoke floor, baseline pending |
| validate/apply collect | `validate_apply_collect` | smoke floor, baseline pending |
| validate/apply exchange | `validate_apply_exchange` | smoke floor, baseline pending |
| validate/apply fulfill with refill | `validate_apply_fulfill_refill` | smoke floor, baseline pending |
| public view projection | `public_view_generation` | smoke floor, baseline pending |
| effect serialization and public filtering | `effect_serialization_filtering` | smoke floor, baseline pending |
| replay command stream | `replay_command_stream` | smoke floor, baseline pending |
| random legal playout | `random_legal_playout` | smoke floor, baseline pending |
| Level 1 bot decision | `level1_bot_decision` | smoke floor, baseline pending |
| WASM operation smoke proxy | `wasm_operation_smoke` | smoke floor, baseline pending |

## Threshold Posture

`thresholds.json` intentionally uses a threshold of `1` for every operation with
`baseline_pending_non_blocking` rationale. These are smoke floors, not
performance claims. Token Bazaar does not yet have stable CI measurements,
CPU isolation, or a repeated-run baseline, so the gate lane compiles and runs
the native bench harness without asserting runner-specific throughput.

Follow-up calibration should collect repeated CI measurements, document the
environment, and then tighten thresholds under the existing ADR-0002 and
ADR-0003 benchmark-gating semantics.

## Verification Commands

- `cargo bench -p token_bazaar -- legal_actions`
- `cargo bench -p token_bazaar`
- `cargo run -p bench-report -- --input <token_bazaar_benchmark_report.txt> --thresholds games/token_bazaar/benches/thresholds.json`
