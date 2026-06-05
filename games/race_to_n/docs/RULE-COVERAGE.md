# race_to_n Rule Coverage Matrix

Game ID: `race_to_n`

Rules version: `race_to_n-rules-v1`

Data/manifest version: `not started`

Engine version: `Gate 1 pre-implementation`

Prepared by: `Codex`

Last updated: 2026-06-05

## Purpose

This is the requirements traceability matrix for `race_to_n`. It maps every
stable rule ID in `RULES.md` to implementation areas and verification evidence.

Silent gaps are not allowed. This ticket is requirements-first, so the rows are
intentionally `not started` until later tickets implement and verify them.

## Status labels

| Status | Meaning |
|---|---|
| covered | Implementation and required evidence exist. |
| partial | Some evidence exists, but required coverage is incomplete. |
| not started | Known requirement; no implementation/evidence yet. |
| intentionally deferred | Deferred by documented stage/gate decision. |
| unsupported | Explicitly not implemented for this variant. |
| not applicable | Truly not applicable, with rationale. |
| blocked | Cannot proceed until stated blocker is resolved. |

## Rule coverage matrix

| Rule ID | Rule summary | Source/rationale | Rust implementation module/function/area | Unit tests | Named rule tests | Golden traces | Property/invariant coverage | Simulation/fuzz coverage | Replay/hash coverage | Serialization coverage | Visibility/no-leak coverage | Bot coverage | UI smoke coverage if web-exposed | Benchmark relevance | Status | Notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `R-SCOPE-001` | Deterministic two-seat numeric race. | Gate 1 spec. | `games/race_to_n` crate, later. | not started | not started | shortest normal, later | not started | not started | not started | not started | not applicable: no hidden state | not started | not started | random-playout throughput | not started | Requirements pinned by GAT1RACTON-001. |
| `R-SCOPE-002` | Foundation-smoke role with full evidence. | OGC readiness labels and Gate 1 spec. | Whole evidence surface, later. | not started | not started | not started | not started | not started | not started | not started | not applicable: process role | not started | not started | all Gate 1 benchmarks | not started | Closed by final evidence tickets. |
| `R-VAR-001` | Race to 21 selected. | `SOURCES.md` variant choice. | setup/variant module, later. | not started | not started | shortest normal and terminal, later | not started | not started | not started | not started | not applicable: public constants | not started | not started | setup/playout | not started | Pins spec assumption 1. |
| `R-VAR-002` | Fixed parameters: target 21, max add 3, two seats, seat 0 first. | `SOURCES.md` variant choice. | setup/variant module, later. | not started | not started | setup trace, later | not started | not started | not started | not started | not applicable: public constants | not started | not started | setup/playout | not started | No user-selected variant parameters. |
| `R-COMP-001` | Public counter. | Design rationale. | state module, later. | not started | not started | shortest normal, later | not started | not started | not started | not started | not applicable: public | not started | not started | view/serialization | not started | Game-local vocabulary only. |
| `R-COMP-002` | Public seats. | Gate 1 two-seat scope. | ids/state modules, later. | not started | not started | setup trace, later | not started | not started | not started | not started | not applicable: public | not started | not started | view/serialization | not started | Game-local vocabulary only. |
| `R-SETUP-001` | Initial total is 0. | Design rationale. | setup module, later. | not started | not started | setup/shortest normal, later | not started | not started | not started | not started | not applicable: public | not started | UI start smoke, later | setup | not started | Deterministic setup. |
| `R-SETUP-002` | Seat 0 starts. | Deterministic replay rationale. | setup/state module, later. | not started | not started | setup trace, later | not started | not started | not started | not started | not applicable: public | not started | UI start smoke, later | setup | not started | No first-player RNG. |
| `R-SETUP-003` | Target and max addition are fixed. | `SOURCES.md` variant choice. | variant module, later. | not started | not started | setup trace, later | not started | not started | not started | not started | not applicable: public | not started | UI start smoke, later | setup | not started | Static data must not carry behavior. |
| `R-TURN-001` | Active seat submits one legal addition. | Design rationale. | rules/actions module, later. | not started | not started | shortest normal, later | not started | not started | not started | not started | not applicable: public | bot legality, later | one human action, later | apply latency | not started | Must validate freshness token later. |
| `R-TURN-002` | Non-terminal action passes turn. | Design rationale. | rules/effects module, later. | not started | not started | shortest normal, later | turn-order invariant, later | random simulation, later | not started | not started | not applicable: public | bot playout, later | bot turn smoke, later | apply/playout | not started | Terminal action does not pass turn. |
| `R-ACTION-001` | Legal additions are 1..min(3, remaining). | `SOURCES.md` ambiguity resolution. | actions/rules module, later. | not started | not started | terminal trace, later | legal-action invariant, later | random simulation, later | action-tree hash, later | not started | not applicable: public | random legal bot, later | legal action display, later | legal-action generation | not started | Rule tests must cover totals 18, 19, and 20. |
| `R-RESTRICT-001` | Invalid, stale, wrong-seat, malformed, terminal submissions are rejected. | Rust authority and diagnostics requirement. | validation/diagnostics module, later. | not started | not started | invalid/stale diagnostic trace, later | invalid-state invariant, later | random simulation negative checks, later | not started | not started | not applicable: public game, viewer-safe diagnostic still required | bot must not trigger invalid actions, later | stale diagnostic smoke, later | validation/apply | not started | Diagnostic wording must stay viewer-safe. |
| `R-SCORE-001` | Winner-only outcome; no score. | Tiny game scope. | outcome module, later. | not started | not started | terminal trace, later | terminal invariant, later | random simulation, later | not started | serialization, later | not applicable: public | bot playout, later | terminal display, later | apply/playout | not started | Draws impossible. |
| `R-END-001` | Reaching 21 ends the game and mover wins. | Normal-play choice. | rules/outcome/effects module, later. | not started | not started | terminal trace, later | terminal invariant, later | random simulation, later | state/effect hash, later | serialization, later | not applicable: public | bot playout, later | terminal display, later | apply/playout | not started | Exact target only. |
| `R-VIS-001` | All state and legal choices are public. | Perfect-information scope. | visibility/public-view module, later. | not started | not started | public-view trace, later | not applicable: no hidden state | not applicable: no hidden state | view hash, later | public-view serialization, later | not applicable: no hidden state exists | bot view, later | public view display, later | view/effect filtering | not started | Later no-leak evidence records not applicable with rationale. |
| `R-RNG-001` | Game rules use no randomness. | Deterministic setup/rules. | setup/rules module, later. | not started | not started | deterministic setup trace, later | deterministic invariant, later | simulation seeds drive bot only, later | replay hash, later | serialization, later | not applicable: no hidden RNG | bot RNG separate, later | not applicable: UI smoke does not prove RNG | replay/playout | not started | Bot RNG is not game-rule RNG. |
| `R-RNG-002` | Replay reproduces state/effect/action-tree/view hashes. | Gate 1 exit criteria. | replay/hash contracts, later. | not started | not started | golden traces, later | replay invariant, later | simulation failure replay, later | replay/hash tests, later | replay JSON round-trip, later | not applicable: no hidden state | bot-action trace, later | not applicable: UI smoke is not replay proof | replay throughput | not started | Implemented after engine contracts and game rules. |
| `R-AMB-001` | Race-to-N counting selected over take-away phrasing. | `SOURCES.md` ambiguity log. | setup/rules/docs, later. | not started | not started | shortest normal and terminal, later | not started | not started | not started | not started | not applicable: public | not started | UI copy, later | none | not started | No code may implement a take-away pile variant. |
| `R-AMB-002` | No overshoot. | `SOURCES.md` ambiguity log. | validation/actions module, later. | not started | totals 18/19/20 and overshoot rejection, later | invalid diagnostic trace, later | legal-action invariant, later | simulation, later | action-tree hash, later | not started | not applicable: public | random legal bot, later | stale/invalid smoke, later | legal-action generation and validation | not started | Legal set is capped by remaining distance. |
| `R-VAR-003` | Addition race deviates from common removal phrasing. | `SOURCES.md` variant choice. | docs/static metadata, later. | not started | not started | not started | not started | not started | not started | not started | not applicable: public | not started | UI copy, later | none | not started | Public note may explain Race to 21 as an abstract counting race. |
| `R-VAR-004` | Out-of-scope variants are unsupported. | Gate 1 forbidden changes. | variant validation, later. | not started | unsupported variant rejection, later | not started | not started | not started | not started | not started | not applicable: public | not started | not started | none | not started | Multi-pile, misere, randomized starts, and generalized options are not implemented. |

## Visibility/no-leak surface matrix

| Surface | Rule IDs | Evidence | Must not reveal | Status |
|---|---|---|---|---|
| public view | `R-VIS-001` | Future public-view tests. | not applicable: no hidden state exists | not started |
| action tree | `R-ACTION-001`, `R-VIS-001` | Future action-tree tests. | not applicable: legal actions are public | not started |
| preview | `R-ACTION-001`, `R-END-001`, `R-VIS-001` | Future preview or explicit non-use rationale. | not applicable: resulting counter is public | not started |
| effect log | `R-TURN-002`, `R-END-001`, `R-VIS-001` | Future golden traces. | not applicable: effects are public | not started |
| diagnostics | `R-RESTRICT-001`, `R-VIS-001` | Future invalid/stale diagnostic trace. | no private data; no hidden state exists | not started |
| DOM attributes/test IDs | `R-VIS-001` | Future UI smoke. | not applicable: no hidden state exists | not started |
| local storage/replay export | `R-RNG-002`, `R-VIS-001` | Future replay/serialization tests. | not applicable: replay data is public | not started |
| bot explanations/candidate rankings | `R-ACTION-001`, `R-VIS-001` | Future bot tests if explanations exist. | not applicable: bot sees public state only | not started |
| dev inspector/public build boundary | `R-VIS-001` | Future web boundary review. | not applicable: no hidden state exists | not started |

## Test mapping summary

| Test suite/file | Type | Rule IDs covered | What it proves | What it does not prove |
|---|---|---|---|---|
| `games/race_to_n/tests/rule_tests.rs` | named rule | `R-SETUP-*`, `R-TURN-*`, `R-ACTION-001`, `R-RESTRICT-001`, `R-END-001`, later | Rule-level behavior. | Replay, serialization, simulation, UI. |
| `games/race_to_n/tests/property_tests.rs` | property/invariant | `R-TURN-002`, `R-ACTION-001`, `R-END-001`, later | Legal-action and terminal invariants. | UI or benchmark performance. |
| `games/race_to_n/tests/replay_tests.rs` | replay/hash | `R-RNG-002`, later | Hash reproduction. | Rule prose completeness by itself. |
| `games/race_to_n/tests/serialization_tests.rs` | serialization | `R-COMP-*`, `R-SETUP-*`, `R-VIS-001`, `R-RNG-002`, later | JSON round trips and unknown-field rejection. | Rule correctness outside serialized surfaces. |
| `games/race_to_n/tests/bot_tests.rs` | bot | `R-ACTION-001`, `R-RNG-001`, later | Random legal bot legality and determinism. | Bot strength. |
| `apps/web` UI smoke | UI smoke | `R-ACTION-001`, `R-RESTRICT-001`, `R-VIS-001`, later | Browser path displays Rust-supplied state/actions/effects. | Rust rule correctness. |

## Simulation/fuzz coverage summary

| Simulation/fuzz run | Seeds/count | Bots/policies | Rule IDs stressed | Metrics recorded | Status/notes |
|---|---:|---|---|---|---|
| `tools/simulate race_to_n --games 100000` | 100,000+ | Level 0 random legal bot | `R-TURN-*`, `R-ACTION-001`, `R-END-001`, `R-RNG-001` | completed games, terminal outcomes, illegal action attempts, invariant failures, average length, playout throughput, failing seed command stream | not started; owned by GAT1RACTON-009 and GAT1RACTON-015. |

## Benchmark relevance map

| Benchmark | Rule IDs/mechanics relevant | Why relevant | Current threshold/status |
|---|---|---|---|
| legal-action generation | `R-ACTION-001`, `R-AMB-002` | Legal list is tiny; overhead must stay visible. | not started |
| apply action | `R-TURN-*`, `R-END-001` | Core state transition path. | not started |
| view/effect filtering | `R-VIS-001`, `R-TURN-002`, `R-END-001` | Public view and semantic effects feed WASM/UI. | not started |
| serialization + replay throughput | `R-RNG-002` | Gate 1 requires hash reproduction and replay evidence. | not started |
| random playout throughput | `R-ACTION-001`, `R-END-001` | Stage 1 simulation target stresses complete games. | not started |
| random-bot decision latency | `R-ACTION-001`, `R-RNG-001` | Level 0 bot must be cheap and deterministic. | not started |

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage rows updated? | Traces/tests updated? | Date |
|---|---|---|---:|---:|---|
| not applicable | not applicable | Initial rule set. | not applicable | not applicable | 2026-06-05 |

## Coverage review checklist

- Every rule ID in `RULES.md` has exactly one primary row here.
- Section numbers alone are not used as requirements keys.
- Every deferred, unsupported, or not-applicable item has an explicit rationale.
- Rust tests and traces are primary for rule correctness.
- UI smoke tests are marked as smoke only.
- Golden traces cover normal, terminal, invalid/stale, bot, hidden, stochastic, and serialization cases where applicable.
- Replay/hash determinism is covered.
- Serialization compatibility is covered or explicitly deferred.
- Visibility/no-leak surfaces are covered or explicitly `not applicable`.
- Bot coverage uses legal action APIs and allowed views.
- Benchmark relevance is recorded for hot rules/mechanics.
- Rule-ID migrations update rules, coverage, tests, traces, docs, and release notes.
