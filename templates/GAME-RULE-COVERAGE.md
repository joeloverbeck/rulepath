# <game_id> Rule Coverage Matrix

Game ID: `<game_id>`

Rules version: `<rules_version>`

Data/manifest version: `<data_or_manifest_version>`

Engine version: `<engine_version>`

Prepared by: `<name/agent>`

Last updated: YYYY-MM-DD

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

| Rule ID | Rule summary | Source/rationale | Rust implementation module/function/area | Unit tests | Named rule tests | Golden traces | Property/invariant coverage | Simulation/fuzz coverage | Replay/hash coverage | Serialization coverage | Visibility/no-leak coverage | Bot coverage | UI smoke coverage if web-exposed | Benchmark relevance | Status | Notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `R-SCOPE-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | not applicable / `<coverage>` | `<coverage>` | `<smoke>` | none / `<benchmark>` | not started | `<notes>` |
| `R-COMP-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<smoke>` | `<benchmark>` | not started | `<notes>` |
| `R-SETUP-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<smoke>` | `<benchmark>` | not started | `<notes>` |
| `R-TURN-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<smoke>` | `<benchmark>` | not started | `<notes>` |
| `R-ACTION-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<smoke>` | `<benchmark>` | not started | `<notes>` |
| `R-RESTRICT-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<smoke>` | `<benchmark>` | not started | `<notes>` |
| `R-SCORE-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<smoke>` | `<benchmark>` | not started | `<notes>` |
| `R-END-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<smoke>` | `<benchmark>` | not started | `<notes>` |
| `R-VIS-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<smoke>` | `<benchmark>` | not started | `<notes>` |
| `R-RNG-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<smoke>` | `<benchmark>` | not applicable | `<notes>` |
| `R-VAR-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<smoke>` | `<benchmark>` | not started | `<notes>` |
| `R-AMB-001` | `<summary>` | `<source/rationale>` | `<module/area>` | `<tests>` | `<tests>` | `<traces>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<coverage>` | `<smoke>` | `<benchmark>` | not started | `<notes>` |

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

## Test mapping summary

| Test suite/file | Type | Rule IDs covered | What it proves | What it does not prove |
|---|---|---|---|---|
| `<test_name>` | unit / named rule / property / replay / serialization / no-leak / bot / UI smoke / benchmark | `<rule_ids>` | `<proof>` | `<limits>` |

## Simulation/fuzz coverage summary

| Simulation/fuzz run | Seeds/count | Bots/policies | Rule IDs stressed | Metrics recorded | Status/notes |
|---|---:|---|---|---|---|
| `<run>` | `<count>` | `<bots>` | `<rule_ids>` | completed games, terminal outcomes, illegal action attempts, invariant failures, average length, playout throughput, failing seed command stream | `<notes>` |

## Benchmark relevance map

| Benchmark | Rule IDs/mechanics relevant | Why relevant | Current threshold/status |
|---|---|---|---|
| `<benchmark>` | `<rule_ids>` | `<reason>` | `<threshold/status>` |

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage rows updated? | Traces/tests updated? | Date |
|---|---|---|---:|---:|---|
| `<old_id>` | `<new_id>` | `<reason>` | yes/no | yes/no | YYYY-MM-DD |

## Coverage review checklist

- Every rule ID in `GAME-RULES.md` has exactly one primary row here.
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
