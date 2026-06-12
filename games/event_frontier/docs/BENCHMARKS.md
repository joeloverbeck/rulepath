# Event Frontier Benchmarks

Game ID: `event_frontier`

Rules version: `event-frontier-rules-v1`

Last updated: 2026-06-12

## Native Target

Event Frontier starts with smoke floors for every benchmarked operation. The
threshold posture is deliberately non-blocking for pull requests: each
operation must compile, run, and exceed one operation per second in the native
benchmark lane.

The ROADMAP stage budget is `100+ turns/sec` for the full playout path. The
initial threshold remains a smoke floor until repeated CI measurements establish
variance-aware calibrated floors under ADR 0002, ADR 0003, and ADR 0005.

## Operations

| Operation | Unit | Purpose | Threshold posture |
|---|---|---|---|
| `setup_standard` | `setups_per_second` | Standard scenario setup, including deterministic epoch shuffle. | Smoke floor pending calibration. |
| `shuffle_and_deal_epochs` | `traces_per_second` | Setup shuffle plus internal trace marker construction for hidden/stochastic surfaces. | Smoke floor pending calibration. |
| `legal_tree_first_choice` | `trees_per_second` | First-choice action-tree generation for the current event card. | Smoke floor pending calibration. |
| `legal_tree_peak_op_branching` | `trees_per_second` | Peak Charter operation branching with multi-site choices. | Smoke floor pending calibration. |
| `apply_event` | `actions_per_second` | Applying an event choice and typed event effects. | Smoke floor pending calibration. |
| `apply_op_multi_site` | `actions_per_second` | Applying a full multi-site operation command. | Smoke floor pending calibration. |
| `edict_modifier_projection` | `views_per_second` | Public projection with active edict modifier state. | Smoke floor pending calibration. |
| `reckoning_pipeline` | `reckonings_per_second` | Reckoning victory check, scoring, income, reset, and advance pipeline. | Smoke floor pending calibration. |
| `serialize_view` | `serializations_per_second` | Stable public-view serialization and hashing. | Smoke floor pending calibration. |
| `bot_l1_choice_charter` | `decisions_per_second` | Charter Level 1 bot decision over public view and legal tree. | Smoke floor pending calibration. |
| `bot_l1_choice_freeholders` | `decisions_per_second` | Freeholders Level 1 bot decision over public view and legal tree. | Smoke floor pending calibration. |
| `full_random_playout` | `turns_per_second` | Full deterministic Level 1 playout throughput, reported as turns per second. | Smoke floor pending calibration; target budget is 100+ turns/sec. |

## CI Strategy

Pull requests should run a non-gating bench smoke for compilation and basic
execution. Scheduled, push-to-main, and explicitly dispatched benchmark lanes
can run `cargo bench -p event_frontier`, capture the native report, and validate
it with `cargo run -p bench-report -- --input <report> --thresholds
games/event_frontier/benches/thresholds.json` once ticket 015 registers the game
with `bench-report`.

Thresholds live in `games/event_frontier/benches/thresholds.json`; the benchmark
binary emits matching operation names and units in the standard Rulepath
benchmark-report JSON format.
