# Masked Claims Benchmarks

Game ID: `masked_claims`

Rules version: `masked-claims-rules-v1`

Last updated: 2026-06-11

## Native Target

Masked Claims starts with smoke floors for every benchmarked operation. The
floor is intentionally non-heroic: each operation must execute and exceed one
operation per second in the native benchmark lane.

This is a provisional Gate 11 posture under ADR 0002, ADR 0003, and ADR 0005.
A follow-up calibration ticket should replace the smoke floors with
variance-aware thresholds after repeated measurements from the selected
benchmark runner.

## Operations

| Operation | Unit | Purpose | Threshold posture |
|---|---|---|---|
| `legal_actions_claim_phase` | `trees_per_second` | Legal action tree generation for the active claimant. | Smoke floor pending calibration. |
| `legal_actions_reaction_window` | `trees_per_second` | Legal action tree generation for the responder window. | Smoke floor pending calibration. |
| `validate_claim` | `validations_per_second` | Command-envelope validation for a legal claim. | Smoke floor pending calibration. |
| `apply_claim_open_window` | `actions_per_second` | Applying a claim and opening the reaction window. | Smoke floor pending calibration. |
| `apply_accept_resolution` | `actions_per_second` | Applying an accept response and veiling the mask. | Smoke floor pending calibration. |
| `apply_challenge_resolve_reveal` | `actions_per_second` | Applying a challenge response, reveal, and scoring. | Smoke floor pending calibration. |
| `project_public_view_pending_reaction` | `views_per_second` | Observer-safe projection while a claim is pending. | Smoke floor pending calibration. |
| `project_public_view_after_reveal` | `views_per_second` | Observer-safe projection after a challenged reveal. | Smoke floor pending calibration. |
| `state_hash_terminal` | `hashes_per_second` | Stable terminal-state summary hashing. | Smoke floor pending calibration. |
| `public_export_timeline` | `exports_per_second` | Viewer-scoped public replay export/import. | Smoke floor pending calibration. |
| `level1_bot_claim_decision` | `decisions_per_second` | Level 1 claimant decision over the legal tree. | Smoke floor pending calibration. |
| `level1_bot_response_decision` | `decisions_per_second` | Level 1 responder decision over the reaction tree. | Smoke floor pending calibration. |

## CI Strategy

Pull requests should run a non-gating bench smoke for compilation and basic
execution. Scheduled or explicitly dispatched benchmark lanes can run
`cargo bench -p masked_claims`, capture the native report, and validate it with
`cargo run -p bench-report -- --input <report> --thresholds
games/masked_claims/benches/thresholds.json` once bench-report registration is
wired.

Thresholds live in `games/masked_claims/benches/thresholds.json`; the benchmark
binary emits matching operation names and units in the standard Rulepath
benchmark-report JSON format.
