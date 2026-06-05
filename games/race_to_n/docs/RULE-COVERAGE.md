# race_to_n Rule Coverage Matrix

Game ID: `race_to_n`

Rules version: `race_to_n-rules-v1`

Data/manifest version: `1`

Engine version: `0.1.0`

Prepared by: `Codex`

Last updated: 2026-06-05

## Purpose

This matrix maps every stable rule ID in `RULES.md` to implementation and evidence. Silent gaps are not allowed. UI smoke and benchmarks are evidence for integration/performance only; Rust rule tests, properties, traces, replay, and serialization tests are the rule authority.

## Status labels

| Status | Meaning |
|---|---|
| covered | Implementation and required evidence exist. |
| covered-by-trace | Golden trace or replay evidence is the primary proof. |
| not-applicable | The rule or surface truly does not apply, with rationale. |
| intentionally-deferred | Deferred by a documented gate/stage decision. |
| unsupported | Explicitly not implemented for this variant. |

## Primary Evidence Files

| Evidence | Coverage role |
|---|---|
| `games/race_to_n/src/setup.rs` | deterministic setup, fixed seats, fixed variant |
| `games/race_to_n/src/actions.rs` | action tree and target-bounded additions |
| `games/race_to_n/src/rules.rs` | validation, freshness-token rejection, state transition, terminal outcome |
| `games/race_to_n/src/visibility.rs` | public view projection |
| `games/race_to_n/src/state.rs` | snapshot/replay JSON and stable serialization |
| `games/race_to_n/src/bots.rs` | Level 0 random legal bot wiring |
| `games/race_to_n/tests/rule_tests.rs` | named rule behavior tests |
| `games/race_to_n/tests/property_tests.rs` | invariants across reachable states |
| `games/race_to_n/tests/replay_tests.rs` | golden traces and replay/hash reproduction |
| `games/race_to_n/tests/serialization_tests.rs` | public view, snapshot, replay JSON round trips |
| `games/race_to_n/tests/bot_tests.rs` | bot legality, determinism, no direct mutation |
| `tools/simulate` | random legal playouts with per-action invariant checks |
| `tools/replay-check` | Trace Schema v1 replay drift gate over all golden traces |
| `tools/fixture-check` | Trace Schema v1 fixture/static-data validation and migration-note enforcement |
| `tools/trace-viewer` | human-readable Trace Schema v1 triage summaries with replay annotations |
| `tools/rule-coverage` | structural coverage drift checker for this matrix |
| `tools/seed-reducer` | deterministic failure-report normalizer and Trace Schema v1 reproducer emitter |
| `tools/bench-report` | hard-failing benchmark threshold checker for marked benchmark reports |
| `apps/web/scripts/smoke-ui.mjs` | browser boundary smoke over WASM API |
| `games/race_to_n/benches/race_to_n.rs` | native benchmark coverage |

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `R-SCOPE-001` | Deterministic two-seat numeric race. | `RaceState`, `setup_match`, `validate_command`, `apply_action`. | `cargo test -p race_to_n`; golden traces; quick/full simulation. | covered | Game rules use no wall-clock or OS entropy. |
| `R-SCOPE-002` | Foundation-smoke role with full evidence. | Whole Gate 1 evidence surface. | docs set, tests, traces, replay, simulation, benchmarks, UI smoke, CI wiring. | covered | Public polish remains intentionally modest. |
| `R-VAR-001` | Race to 21 selected. | `Variant::race_to_21`, `SetupOptions::default`. | setup tests; serialization tests; golden traces. | covered | No alternate variant is selected in Gate 1. |
| `R-VAR-002` | Fixed parameters: target 21, max add 3, two seats, seat 0 first. | `variants.rs`, `setup.rs`. | `static_data_parses_and_rejects_unknown_fields`; setup tests; public view tests. | covered | Manifest/data version is `1`. |
| `R-COMP-001` | Public counter. | `CounterValue`, `RaceState.counter`, `PublicView.counter`. | rule tests; public-view serialization tests; UI smoke. | covered | Counter is visible to all viewers. |
| `R-COMP-002` | Public seats. | `RaceSeat`, `SeatId`, `RaceState.seats`. | setup tests; rule tests; public view tests. | covered | Seat IDs are internal; public view uses public seat names. |
| `R-SETUP-001` | Initial total is 0. | `setup_match`. | `setup_is_deterministic_for_same_inputs`; UI smoke starts at `0 / 21`. | covered | Also covered by golden trace initial state. |
| `R-SETUP-002` | Seat 0 starts. | `setup_match` sets `RaceSeat::Seat0`. | setup tests; UI smoke active seat. | covered | No first-player randomness. |
| `R-SETUP-003` | Target and max addition are fixed. | `Variant::race_to_21`, static data parsers. | variant parser tests; public view test; serialization tests. | covered | Static data rejects behavior-looking keys. |
| `R-TURN-001` | Active seat submits one addition. | `legal_action_tree`, `validate_command`. | `r_turn_001_r_turn_002_valid_action_advances_turn_and_token`; UI smoke. | covered | Freshness token is required for submission. |
| `R-TURN-002` | Non-terminal action passes turn. | `apply_action` emits `TurnChanged` and flips active seat. | rule tests; property tests; golden normal trace; simulation. | covered | Terminal action does not pass turn. |
| `R-ACTION-001` | Additions are 1..min(3, remaining). | `legal_additions`, `legal_action_tree`, `validate_command`. | `r_action_001_legal_actions_are_flat_and_target_bounded`; property tests; bot tests. | covered | Tests include near-target caps. |
| `R-RESTRICT-001` | Invalid, stale, wrong-seat, malformed, terminal submissions are rejected. | `validate_command` diagnostics. | `r_restrict_001_validation_is_fail_closed_for_invalid_stale_and_wrong_actor`; invalid/stale golden trace; UI stale smoke. | covered | Diagnostics are viewer-safe and mutation-free. |
| `R-SCORE-001` | Winner-only outcome; no score. | `RaceState.winner`, terminal effect. | terminal rule tests; terminal golden trace; serialization tests. | covered | Draws are impossible. |
| `R-END-001` | Reaching 21 ends the game and mover wins. | `apply_action` sets winner and emits `GameEnded`. | terminal rule tests; property tests; terminal golden trace; simulation. | covered | Exact target only; overshoot is absent from action tree and rejected by validation. |
| `R-VIS-001` | All state and choices are public. | `project_view`, public effects, `EffectLog::since`. | public view tests; serialization tests; replay tests mark hidden state not applicable; UI smoke. | covered | No hidden state exists for this game. |
| `R-RNG-001` | Game rules use no randomness. | Setup/rules ignore seed for rule behavior; bot RNG is separate. | setup determinism tests; replay tests; bot tests isolate bot seed. | covered | Simulation seed drives bot policy, not game rules. |
| `R-RNG-002` | Replay reproduces state/effect/action-tree/view hashes. | `RaceSnapshot`, `RaceReplayJson`, stable hash surfaces. | `replay_reproduces_hashes_for_same_inputs`; `golden_traces_match_expected_hashes`; serialization tests. | covered-by-trace | Golden traces include normal, terminal, bot, and invalid/stale cases. |
| `R-AMB-001` | Race-to-N counting selected over take-away phrasing. | `Variant::race_to_21`, add-based actions/effects. | rule tests and traces use `add-*`; docs source ambiguity log. | covered | No take-away pile implementation exists. |
| `R-AMB-002` | No overshoot. | `legal_additions` caps remaining distance; validation rejects unavailable paths. | near-target rule tests; property tests; invalid trace. | covered | Totals 18, 19, and 20 are covered. |
| `R-VAR-003` | Addition race deviates from common removal phrasing. | Docs/static metadata and add-based UI copy. | `SOURCES.md`, `RULES.md`, UI smoke labels. | covered | This is an original Rulepath presentation. |
| `R-VAR-004` | Out-of-scope variants are unsupported. | Only default setup/variant is exposed; parsers reject unknown fields. | static data parser tests; docs admission; no UI picker. | unsupported | Multi-pile, misere, randomized starts, and generalized options remain out of scope. |

## Visibility Surface Matrix

| Surface | Rule IDs | Evidence | Must not reveal | Status |
|---|---|---|---|---|
| public view | `R-VIS-001` | `project_view_yields_public_view_type_and_expected_fields`; UI smoke | internal seat IDs or hidden state | covered |
| action tree | `R-ACTION-001`, `R-VIS-001` | action-tree rule/property tests; UI renders Rust choices | unavailable choices or hidden reasons | covered |
| preview | `R-ACTION-001`, `R-END-001`, `R-VIS-001` | not implemented in Gate 1 | not applicable | not-applicable |
| effect log | `R-TURN-002`, `R-END-001`, `R-VIS-001` | golden traces; UI smoke effect rows | private state | covered |
| diagnostics | `R-RESTRICT-001`, `R-VIS-001` | invalid/stale rule tests and trace; UI stale smoke | hidden/private data | covered |
| DOM attributes/test IDs | `R-VIS-001` | UI code review and smoke | hidden state | covered |
| local storage/replay export | `R-RNG-002`, `R-VIS-001` | not implemented in Gate 1 web harness | not applicable | not-applicable |
| bot explanations/candidate rankings | `R-ACTION-001`, `R-VIS-001` | not implemented beyond public effects | not applicable | not-applicable |
| dev inspector/public build boundary | `R-VIS-001` | no dev inspector; `render_game_to_text` is concise public smoke state | internal state | covered |

## Simulation And Benchmark Coverage

| Surface | Command/evidence | Rule IDs stressed | Status |
|---|---|---|---|
| quick simulation | `cargo run -p simulate -- --game race_to_n --games 1000` | `R-TURN-*`, `R-ACTION-001`, `R-END-001`, `R-RNG-001` | covered |
| exit simulation | `cargo run -p simulate -- --game race_to_n --games 100000` | `R-TURN-*`, `R-ACTION-001`, `R-END-001`, `R-RNG-001` | covered |
| replay drift gate | `cargo run -p replay-check -- --game race_to_n --all` | `R-RNG-002`, trace hash surfaces, terminal/outcome replay | covered |
| fixture/schema gate | `cargo run -p fixture-check -- --game race_to_n` | Trace Schema v1 fixture integrity, static data versioning, migration notes | covered |
| rule coverage gate | `cargo run -p rule-coverage -- --game race_to_n` | exactly one coverage row per stable rule ID; no silent unsupported gaps | covered |
| trace triage viewer | `cargo run -p trace-viewer -- --game race_to_n --trace games/race_to_n/tests/golden_traces/shortest-normal.trace.json` | viewer-safe trace metadata, commands, checkpoints, hashes, diagnostics, rationales | covered |
| seed reproducer normalizer | `cargo run -p seed-reducer -- --game race_to_n --failure-report <report>` | simulation failure report to deterministic replay/simulation command and trace reproducer | covered |
| native benchmarks | `cargo bench -p race_to_n` | legal actions, apply, view/effects, serialization/replay, playout, bot latency | covered |
| Stage-1 perf budget | `BENCHMARKS.md`; `docs/adr/0001-stage-1-random-playout-budget.md`; `cargo run -p bench-report -- --input <report> --thresholds games/race_to_n/benches/thresholds.json` | random playout throughput | covered | ADR 0001 accepts a 100,000 games/sec validated-playout floor and `bench-report` hard-fails below it. |

## Review Checklist

- Every rule ID in `RULES.md` has exactly one primary row here.
- Every unsupported or not-applicable item has an explicit rationale.
- Rust tests and traces are primary for rule correctness.
- UI smoke is marked as browser integration evidence only.
- Golden traces cover normal, terminal, invalid/stale, bot, hidden-state rationale, stochastic rationale, and serialization/replay surfaces where applicable.
- Replay/hash determinism is covered.
- Serialization compatibility is covered.
- Visibility/no-leak surfaces are covered or explicitly `not-applicable`.
- Bot coverage uses Rust action APIs and public views.
- Benchmark relevance is recorded for hot paths.
