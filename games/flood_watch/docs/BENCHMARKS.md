# Flood Watch Benchmarks

Game ID: `flood_watch`

Rules version: `flood-watch-rules-v1`

Last updated: 2026-06-11

## Native Target

Flood Watch starts with smoke floors for every benchmarked operation. The
threshold posture is deliberately non-blocking: each operation must compile,
run, and exceed one operation per second in the native benchmark lane.

This is a provisional Gate 12 posture under ADR 0002, ADR 0003, and ADR 0005.
A follow-up calibration ticket should replace the smoke floors with
variance-aware thresholds after repeated measurements from the selected
benchmark runner.

## Operations

| Operation | Unit | Purpose | Threshold posture |
|---|---|---|---|
| `legal_actions_action_phase` | `trees_per_second` | Legal action tree generation for the active budgeted action phase. | Smoke floor pending calibration. |
| `validate_action` | `validations_per_second` | Command-envelope validation for a legal Flood Watch action. | Smoke floor pending calibration. |
| `apply_bail` | `actions_per_second` | Applying a role-modified bail action. | Smoke floor pending calibration. |
| `apply_reinforce` | `actions_per_second` | Applying a role-modified reinforce action. | Smoke floor pending calibration. |
| `apply_end_turn_environment_phase` | `actions_per_second` | Applying end-turn plus deterministic environment automation. | Smoke floor pending calibration. |
| `project_public_view_midgame` | `views_per_second` | Observer-safe public projection in a midgame action phase. | Smoke floor pending calibration. |
| `state_hash_terminal` | `hashes_per_second` | Stable terminal-state summary hashing. | Smoke floor pending calibration. |
| `public_export_timeline` | `exports_per_second` | Viewer-scoped public replay export construction. | Smoke floor pending calibration. |
| `level1_bot_decision` | `decisions_per_second` | Level 1 cooperative bot decision over the public view and legal tree. | Smoke floor pending calibration. |
| `random_playout` | `playouts_per_second` | Full cooperative playout smoke using legal bot actions. | Smoke floor pending calibration. |

## CI Strategy

Pull requests should run a non-gating bench smoke for compilation and basic
execution. Scheduled or explicitly dispatched benchmark lanes can run
`cargo bench -p flood_watch`, capture the native report, and validate it with
`cargo run -p bench-report -- --input <report> --thresholds
games/flood_watch/benches/thresholds.json`.

Thresholds live in `games/flood_watch/benches/thresholds.json`; the benchmark
binary emits matching operation names and units in the standard Rulepath
benchmark-report JSON format.
