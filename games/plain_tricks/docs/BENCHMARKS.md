# Plain Tricks Benchmarks

Game ID: `plain_tricks`

Rules version: `plain-tricks-rules-v1`

Last updated: 2026-06-09

## Native Target

The provisional native floor is at least 2,000 completed random-legal matches
per second for `random_legal_full_playout` in the native benchmark lane.

This is a provisional Gate 10.1 floor under ADR 0002 and ADR 0003. ADR 0005 is
accepted variance-aware calibration doctrine, but this provisional floor is not
claimed as an ADR 0005-calibrated CI floor. A follow-up calibration ticket should
replace the smoke floors and confirm or adjust the random-legal floor using
repeated measurements from the selected benchmark runner.

## Operations

| Operation | Unit | Purpose | Threshold posture |
|---|---|---|---|
| `setup_shuffle_deal` | `setups_per_second` | Seeded deck construction, shuffle, deal, and initial state creation. | Smoke floor pending calibration. |
| `legal_actions_lead` | `trees_per_second` | Legal action tree generation for a leader. | Smoke floor pending calibration. |
| `legal_actions_follow` | `trees_per_second` | Legal action tree generation for a follower after a led card. | Smoke floor pending calibration. |
| `validate_play` | `validations_per_second` | Command-envelope validation for a legal play. | Smoke floor pending calibration. |
| `apply_play` | `actions_per_second` | Applying a single lead play. | Smoke floor pending calibration. |
| `trick_resolution` | `tricks_per_second` | Applying the follower play that resolves a trick. | Smoke floor pending calibration. |
| `project_observer_view` | `views_per_second` | Observer-safe public view projection. | Smoke floor pending calibration. |
| `project_seat_view` | `views_per_second` | Seat-scoped private view projection. | Smoke floor pending calibration. |
| `public_export_import` | `exports_per_second` | Viewer-scoped public replay export and import. | Smoke floor pending calibration. |
| `random_legal_full_playout` | `matches_per_second` | Complete random-legal matches from setup to terminal. | Provisional native floor: 2,000 matches/sec. |
| `level2_full_playout` | `matches_per_second` | Complete Level 2 bot matches from setup to terminal. | Smoke floor pending calibration. |

## CI Strategy

Pull requests run a non-gating bench smoke for compilation and basic execution.
The scheduled, workflow-dispatch, and push-to-main gate runs `cargo bench -p
plain_tricks`, captures the native report, and validates it with
`cargo run -p bench-report -- --input <report> --thresholds
games/plain_tricks/benches/thresholds.json`.

Thresholds live in `games/plain_tricks/benches/thresholds.json`; the benchmark
binary emits matching operation names and units in the standard Rulepath
benchmark-report JSON format.
