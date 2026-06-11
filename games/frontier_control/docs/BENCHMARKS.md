# Frontier Control Benchmarks

Game ID: `frontier_control`

Rules version: `frontier-control-rules-v1`

Last updated: 2026-06-11

## Native Target

Frontier Control starts with smoke floors for every benchmarked operation. The
threshold posture is deliberately non-blocking: each operation must compile,
run, and exceed one operation per second in the native benchmark lane.

This is a provisional Gate 13 posture under ADR 0002, ADR 0003, and ADR 0005.
A follow-up calibration ticket should replace the smoke floors with
variance-aware thresholds after repeated measurements from the selected
benchmark runner.

## Operations

| Operation | Unit | Purpose | Threshold posture |
|---|---|---|---|
| `legal_actions_garrison_midgame` | `trees_per_second` | Legal action tree generation for the Garrison's budgeted action phase. | Smoke floor pending calibration. |
| `legal_actions_prospectors_midgame` | `trees_per_second` | Legal action tree generation for the Prospectors' budgeted action phase. | Smoke floor pending calibration. |
| `validate_action` | `validations_per_second` | Command-envelope validation for a legal Frontier Control action. | Smoke floor pending calibration. |
| `apply_march_with_clash` | `actions_per_second` | Applying a march that resolves the asymmetric crew-into-guards clash. | Smoke floor pending calibration. |
| `apply_end_turn_round_scoring` | `actions_per_second` | Applying Garrison end-turn plus deterministic round scoring. | Smoke floor pending calibration. |
| `supply_connectivity_traversal` | `scores_per_second` | The scoring hot path, including guard-free supply connectivity traversal. | Smoke floor pending calibration. |
| `project_public_view_midgame` | `views_per_second` | Observer-safe public projection with Rust-computed supplied/cut flags. | Smoke floor pending calibration. |
| `state_hash_terminal` | `hashes_per_second` | Stable terminal-state summary hashing. | Smoke floor pending calibration. |
| `garrison_level1_bot_decision` | `decisions_per_second` | Garrison Level 1 bot decision over the public view and legal tree. | Smoke floor pending calibration. |
| `prospector_level1_bot_decision` | `decisions_per_second` | Prospector Level 1 bot decision over the public view and legal tree. | Smoke floor pending calibration. |
| `random_playout` | `playouts_per_second` | Full legal bot playout smoke through terminal. | Smoke floor pending calibration. |

## CI Strategy

Pull requests should run a non-gating bench smoke for compilation and basic
execution. Scheduled or explicitly dispatched benchmark lanes can run
`cargo bench -p frontier_control`, capture the native report, and validate it
with `cargo run -p bench-report -- --input <report> --thresholds
games/frontier_control/benches/thresholds.json`.

Thresholds live in `games/frontier_control/benches/thresholds.json`; the
benchmark binary emits matching operation names and units in the standard
Rulepath benchmark-report JSON format.
