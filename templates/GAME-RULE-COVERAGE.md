# <game_id> Rule Coverage Matrix

Game ID: `<game_id>`

Rules version: `<rules_version>`

Data/manifest version: `<data_or_manifest_version>`

Engine version: `<engine_version>`

Prepared by: `<name/agent>`

Last updated: YYYY-MM-DD

Evidence receipt: [`GAME-EVIDENCE.md`](GAME-EVIDENCE.md)

Template realignment mapping: report `B-05 -> GAME-RULE-COVERAGE.md`. This
template owns the rule-ID-to-proof matrix. `GAME-EVIDENCE.md` owns cross-template
viewer matrices, trace profile status, benchmark workload IDs, release status,
and artifact-link rollups.

## Purpose

This is the requirements traceability matrix for the game. It maps every stable rule ID in `GAME-RULES.md` to implementation areas and verification evidence.

Silent gaps are not allowed. Use explicit `not applicable`, `intentionally deferred`, or `unsupported` rows with rationale.

UI smoke tests do not prove rule correctness. Rust tests, named rule tests, golden traces, replay/hash checks, serialization checks, visibility/no-leak checks, and simulations are primary.

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

Use stable evidence IDs so `GAME-EVIDENCE.md`, release checklists, and future
tools can link to proof without copying this matrix.

| Rule ID | Rule summary | Source/rationale | Rust implementation module/function/area | Evidence IDs | Fixture profile | Primary tests/traces | Visibility/no-leak evidence ID | Replay/hash evidence ID | Bot/UI evidence IDs | Status | Notes |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `R-SCOPE-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-SCOPE-001` | not applicable / `replay-command-v1` / `domain-evidence-v1` | `<tests/traces>` | not applicable / `LEAK-*` | not applicable / `TRACE-*` | not applicable / `UI-*` / `BOT-*` | not started | `<notes>` |
| `R-COMP-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-COMP-001` | `<profile>` | `<tests/traces>` | `<LEAK-* or not applicable>` | `<TRACE-* or not applicable>` | `<UI-*/BOT-* or not applicable>` | not started | `<notes>` |
| `R-SETUP-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-SETUP-001` | `setup-evidence-v1` | `<tests/traces>` | `<LEAK-* or not applicable>` | `<TRACE-* or not applicable>` | `<UI-*/BOT-* or not applicable>` | not started | `<notes>` |
| `R-TURN-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-TURN-001` | `<profile>` | `<tests/traces>` | `<LEAK-* or not applicable>` | `<TRACE-* or not applicable>` | `<UI-*/BOT-* or not applicable>` | not started | `<notes>` |
| `R-ACTION-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-ACTION-001` | `<profile>` | `<tests/traces>` | `<LEAK-* or not applicable>` | `<TRACE-* or not applicable>` | `<UI-*/BOT-* or not applicable>` | not started | `<notes>` |
| `R-RESTRICT-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-RESTRICT-001` | `<profile>` | `<tests/traces>` | `<LEAK-* or not applicable>` | `<TRACE-* or not applicable>` | `<UI-*/BOT-* or not applicable>` | not started | `<notes>` |
| `R-SCORE-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-SCORE-001` | `<profile>` | `<tests/traces>` | `<LEAK-* or not applicable>` | `<TRACE-* or not applicable>` | `<UI-*/BOT-* or not applicable>` | not started | `<notes>` |
| `R-END-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-END-001` | `<profile>` | `<tests/traces>` | `<LEAK-* or not applicable>` | `<TRACE-* or not applicable>` | `<UI-*/BOT-* or not applicable>` | not started | `<notes>` |
| `R-VIS-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-VIS-001` | `domain-evidence-v1` | `<tests/traces>` | `LEAK-001` | `<TRACE-* or not applicable>` | `<UI-*/BOT-* or not applicable>` | not started | `<notes>` |
| `R-RNG-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-RNG-001` | `replay-command-v1` / not applicable | `<tests/traces>` | `<LEAK-* or not applicable>` | `TRACE-001` | not applicable / `<BOT-*>` | not applicable | `<notes>` |
| `R-VAR-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-VAR-001` | `setup-evidence-v1` | `<tests/traces>` | `<LEAK-* or not applicable>` | `<TRACE-* or not applicable>` | `<UI-*/BOT-* or not applicable>` | not started | `<notes>` |
| `R-AMB-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `RULE-AMB-001` | `<profile>` | `<tests/traces>` | `<LEAK-* or not applicable>` | `<TRACE-* or not applicable>` | `<UI-*/BOT-* or not applicable>` | not started | `<notes>` |

## Intentionally deferred, unsupported, or not applicable rules

| Rule ID or gap | Status | Rationale | Evidence required later | Required before public release? | Owner |
|---|---|---|---|---:|---|
| `<rule_id_or_gap>` | intentionally deferred / unsupported / not applicable | `<rationale>` | `<future evidence>` | yes/no | `<owner>` |

## Golden trace catalog

| Trace file | Purpose | Command stream source | Seed | Rules/data/engine versions | Rule IDs covered | Expected result/hash evidence | Invalid/stale diagnostic coverage | Hidden-info coverage | Update policy |
|---|---|---|---|---|---|---|---|---|---|
| `<trace_file>` | normal play / terminal / draw / invalid diagnostic / stale diagnostic / bot action / hidden-info / stochastic / serialization | `<source>` | `<seed>` | `<versions>` | `<rule_ids>` | `<hashes/outcome>` | none / `<diagnostic>` | not applicable / `<coverage>` | preserve / update only with behavior note / update only with format migration note |

Golden traces SHOULD include, where applicable:

- normal legal play;
- terminal win/loss/draw/shared outcome;
- invalid action diagnostic;
- stale action diagnostic;
- random setup or draw determinism;
- hidden-information redaction;
- bot-selected action;
- replay import/export compatibility;
- serialization round trip.

## Invalid/stale diagnostic trace requirements

| Diagnostic scenario | Rule IDs | Required trace/test | Viewer-safe? | Status |
|---|---|---|---:|---|
| illegal action path | `<rule_ids>` | `<trace/test>` | yes/no | not started / partial / covered |
| stale freshness token | `<rule_ids>` | `<trace/test>` | yes/no | not started / partial / covered |
| unavailable action due to phase/seat | `<rule_ids>` | `<trace/test>` | yes/no | not started / partial / covered |
| hidden reason redaction | `<rule_ids>` | `<trace/test>` | yes/no/not applicable | not started / partial / covered / not applicable |

Diagnostics MUST explain safely. They MUST NOT reveal hidden state through error strings, disabled reasons, logs, action tree metadata, or replay exports.

## Hidden-information trace requirements

Use this section for hidden-information games. For perfect-information games, fill one explicit `not applicable` row.

| Surface | Rule IDs | Trace/test | Must not reveal | Status |
|---|---|---|---|---|
| public view | `<rule_ids>` | `<trace/test>` | `<hidden_info>` | not started / covered / not applicable |
| action tree | `<rule_ids>` | `<trace/test>` | `<hidden_info>` | not started / covered / not applicable |
| preview | `<rule_ids>` | `<trace/test>` | `<hidden_info>` | not started / covered / not applicable |
| effect log | `<rule_ids>` | `<trace/test>` | `<hidden_info>` | not started / covered / not applicable |
| diagnostics | `<rule_ids>` | `<trace/test>` | `<hidden_info>` | not started / covered / not applicable |
| DOM attributes/test IDs | `<rule_ids>` | `<trace/test>` | `<hidden_info>` | not started / covered / not applicable |
| local storage/replay export | `<rule_ids>` | `<trace/test>` | `<hidden_info>` | not started / covered / not applicable |
| bot explanations/candidate rankings | `<rule_ids>` | `<trace/test>` | `<hidden_info>` | not started / covered / not applicable |
| dev inspector/public build boundary | `<rule_ids>` | `<trace/test>` | `<hidden_info>` | not started / covered / not applicable |

## Terminal result and viewer-class coverage

Every terminal result and every viewer class in `GAME-RULES.md#seat-model` MUST
have explicit coverage. `GAME-EVIDENCE.md#viewer-matrix` owns the public and
per-seat viewer matrix rollup; this coverage file links the rule IDs and
evidence IDs that prove it.

| Terminal result | Rule IDs | Viewer class | Expected projection | Evidence ID/link | Status |
|---|---|---|---|---|---|
| `<win/loss/draw/split/shared outcome/elimination/no-reveal terminal>` | `<rule_ids>` | `<viewer class>` | `<per-seat or per-team facts visible to this viewer>` | `LEAK-*` / `TRACE-*` / `<trace/test>` | not started / partial / covered |

## Pairwise N-seat hidden-information coverage

Required for hidden-information N-seat games. `GAME-EVIDENCE.md#viewer-matrix`
owns the pairwise no-leak matrix rollup. Keep rule-specific evidence IDs here
and link the detailed matrix from the receipt.

| Pairwise no-leak evidence ID | Rule IDs | Source private datum | Viewer class | Receipt row/link | Status |
|---|---|---|---|---|---|
| `LEAK-001` | `<rule_ids>` | `<seat/private datum or not applicable>` | `<owning seat / teammate / opponent / public observer / replay viewer / dev-only harness>` | `GAME-EVIDENCE.md#viewer-matrix` | not started / partial / covered / not applicable |

## Test mapping summary

| Test suite/file | Type | Rule IDs covered | What it proves | What it does not prove |
|---|---|---|---|---|
| `<test_name>` | unit / named rule / property / replay / serialization / no-leak / bot / UI smoke / benchmark | `<rule_ids>` | `<proof>` | `<limits>` |

## Simulation/fuzz coverage summary

| Simulation/fuzz run | Seeds/count | Bots/policies | Rule IDs stressed | Metrics recorded | Status/notes |
|---|---:|---|---|---|---|
| `<run>` | `<count>` | `<bots>` | `<rule_ids>` | completed games, terminal outcomes, illegal action attempts, invariant failures, average length, playout throughput, failing seed command stream | `<notes>` |

## Benchmark relevance links

`GAME-EVIDENCE.md` owns benchmark workload IDs and current status. This section
links rules to benchmark evidence without duplicating benchmark receipts.

| Benchmark evidence ID | Rule IDs/mechanics relevant | Why relevant | Receipt row/link |
|---|---|---|---|
| `BENCH-001` | `<rule_ids>` | `<reason>` | `GAME-EVIDENCE.md#benchmarks-and-bot-policy` |

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage rows updated? | Traces/tests updated? | Date |
|---|---|---|---:|---:|---|
| `<old_id>` | `<new_id>` | `<reason>` | yes/no | yes/no | YYYY-MM-DD |

## Coverage review checklist

- Every rule ID in `GAME-RULES.md` has exactly one primary row here.
- Section numbers alone are not used as requirements keys.
- Evidence IDs are stable and linked from `GAME-EVIDENCE.md` where cross-template status is needed.
- Every deferred, unsupported, or not-applicable item has an explicit rationale.
- Rust tests and traces are primary for rule correctness.
- UI smoke tests are marked as smoke only.
- Golden traces cover normal, terminal, invalid/stale, bot, hidden, stochastic, and serialization cases where applicable.
- Replay/hash determinism is covered.
- Serialization compatibility is covered or explicitly deferred.
- Visibility/no-leak surfaces are covered or explicitly `not applicable`.
- Bot coverage uses legal action APIs and allowed views.
- Benchmark relevance links are recorded for hot rules/mechanics; current workload status lives in `GAME-EVIDENCE.md`.
- Rule-ID migrations update rules, coverage, tests, traces, docs, and release notes.
