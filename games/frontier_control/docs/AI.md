# frontier_control AI

Game ID: `frontier_control`

Implemented variant: `frontier_control_standard`, `frontier_control_highlands`

Rules version: `frontier-control-rules-v1`

Last updated: 2026-06-11

Prepared by: Codex

## Purpose

This document records the Frontier Control bot registry and public bot posture.
Full strategy evidence lives in [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md),
with player-level context in [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md).

## Bot summary

| Bot | Level | Policy/version | Public default? | Information access | Status | Evidence |
|---|---:|---|---:|---|---|---|
| random legal | 0 | `frontier_control_random_legal_v0` / version 1 | no | legal action tree only | implemented and tested | `tests/bots.rs`; `simulate --game frontier_control` |
| Garrison baseline | 1 | `frontier_control_garrison_level1_v1` / version 1 | constrained | public projection and legal action tree | implemented and tested | `tests/bots.rs`; [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| Prospector baseline | 1 | `frontier_control_prospector_level1_v1` / version 1 | constrained | public projection and legal action tree | implemented and tested | `tests/bots.rs`; [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| authored policy | 2 | not implemented | no | not applicable | deferred | evidence pack is Level 1 only; retune required before polished public claim |
| shallow deterministic search | 3 | not implemented | no | not applicable | not allowed for Gate 13 | No search, MCTS, ISMCTS, Monte Carlo, ML, RL, or LLM move selection. |

Public v1/v2 bots MUST NOT use MCTS, ISMCTS, Monte Carlo-style bots, ML, or RL.

## Level 0: random legal bot

| Item | Decision/evidence |
|---|---|
| legal action API used | `legal_action_tree` and normal validation |
| deterministic seed behavior | deterministic selection from legal leaves with `Seed` |
| action selection method | random legal among legal action paths |
| simulation tests | `cargo run -p simulate -- --game frontier_control --games 1000` |
| legality tests | `cargo test -p frontier_control bots` |
| replay/hash tests | `cargo test -p frontier_control replay` and golden trace registration |
| known limitations | random; not competent |
| public explanation text | selected a seeded random legal Frontier Control action |
| benchmark evidence | `cargo bench -p frontier_control`; [BENCHMARKS.md](BENCHMARKS.md) |

## Level 1: rule-informed baseline bots

| Item | Decision/evidence |
|---|---|
| policy name/version | `frontier_control_garrison_level1_v1` and `frontier_control_prospector_level1_v1` / version 1 |
| decision order summary | Garrison: dismantle high-value stakes, patrol to contest crews/supply, reinforce forts, end turn. Prospectors: stake high-value legal sites, march toward value/openings, muster when useful, end turn. |
| immediate tactics | visible stake denial, visible supply/value pursuit, fort holding, legal no-stall turns |
| mandatory rule handling | legal tree and validation remain Rust-owned |
| tie-break method | deterministic public ordering with bounded public keys |
| information access | public projection and legal action tree only |
| explanation examples | "Garrison dismantled the public stake at Quarry to deny Prospector scoring."; "Prospectors staked Goldfield because it is the highest-value public legal site." |
| tests | `tests/bots.rs`, `tests/property.rs`, `level1_bot_sequence_reaches_terminal_without_illegal_actions` |
| benchmarks | bot decision benchmark rows in [BENCHMARKS.md](BENCHMARKS.md) |
| public suitability | constrained; legal and explainable, but current standard-map Level-1-vs-Level-1 simulation records a balance retune note |

## Level 2: authored policy bot

Required evidence pack: [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md)

Competent-player analysis: [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md)

| Item | Summary only |
|---|---|
| policy name/version | not implemented |
| evidence pack status | complete for Level 1 strategy evidence, not a Level 2 implementation authorization |
| phase model | early/mid/late scoring pressure is documented as future work |
| candidate extraction | legal action tree leaves plus public view facts |
| lexicographic priority vector | Level 1 has simple faction-specific priority order |
| bounded scoring tie-breakers | public stake value, crew/guard counts, supplied stake value, site order |
| deterministic seeded tie-break | current Level 1 branches are deterministic; random legal baseline uses seed |
| explanation contract | public facts only |
| known weaknesses | score-race urgency and balance retune remain open |
| public default suitability | constrained until retuned or replaced |

Do not code Level 2 before the evidence pack is upgraded for Level 2 and the
standard-map balance retune is resolved.

## Level 3: shallow deterministic search

| Requirement | Status | Evidence |
|---|---|---|
| perfect-information game | yes | `FC-VIS-004`; visibility tests |
| small enough search space | not established | benchmarks are smoke only |
| deterministic limits | not documented | no search is implemented |
| documented evaluator | no | out of scope |
| fallback policy | Level 1 | implemented |
| explanation says search was used | not applicable | no search |
| no hidden-information search | yes | no hidden information exists |
| ADR required? | yes for broad search work | foundation bot law and Gate 13 scope exclude search |

## Exact information access table

| Information | Human acting seat sees? | Bot sees? | Public observer sees? | Tests/notes |
|---|---:|---:|---:|---|
| legal action tree | yes | yes | no gameplay authority | `tests/bots.rs` |
| public sites/trails/labels | yes | yes | yes | visibility tests |
| guards, crews, forts, stakes | yes | yes | yes | visibility tests |
| supplied/cut stake status | yes | yes | yes | Rust-projected public view |
| scores, round, active faction, budget | yes | yes | yes | visibility tests |
| own private hand/role/zone | not applicable | not applicable | not applicable | perfect information |
| opponent private hand/role/zone | not applicable | not applicable | not applicable | perfect information |
| unrevealed deck/order/future random outcome | not applicable | no | not applicable | no game-rule randomness |
| private logs | not applicable | not applicable | not applicable | public effects only |
| dev/test full state | no for public bot | no for public bot | no | test-only authority |

Bots MUST NOT receive internal state shortcuts in public paths even though the
game is perfect-information.

## Decision order summary

| Bot | Decision order |
|---|---|
| random legal | seeded random legal leaf |
| Garrison Level 1 | 1. dismantle highest-value public legal stake 2. patrol to contest crews or supplied stakes 3. reinforce held fort 4. end turn |
| Prospector Level 1 | 1. stake highest-value public legal site 2. march toward public stake value or opening 3. muster when legal and useful 4. end turn |

## Explanation examples

| Bot | Situation | Example explanation | Hidden-info safe? | Test |
|---|---|---|---:|---|
| random legal | any legal tree | Selected a seeded random legal Frontier Control action. | yes | `tests/bots.rs` |
| Garrison Level 1 | staked site with guard | Garrison dismantled the public stake at Quarry to deny Prospector scoring. | yes | `tests/bots.rs` |
| Garrison Level 1 | patrol target with crews | Garrison patrolled from Gatehouse to Ford to contest public crew or supply pressure. | yes | `tests/bots.rs` |
| Prospector Level 1 | legal high-value stake | Prospectors staked Goldfield because it is the highest-value public legal site. | yes | `tests/bots.rs` |
| Prospector Level 1 | crew movement | Prospectors marched from Base Camp to Ford toward public stake value or an opening. | yes | `tests/bots.rs` |

## Known weaknesses

| Bot | Weakness | Why acceptable | Mitigation/future trigger |
|---|---|---|---|
| random legal | plays poorly | required baseline only | not public default |
| Garrison Level 1 | simple stake-denial priority can dominate current standard map | Level 1 proves legal explainable baseline, not polished balance | retune constants or policy before public polish |
| Prospector Level 1 | does not preserve supply paths or reason about score race deeply | Level 1 avoids search and weight soup | future Level 2 or Level 1 retune |
| both Level 1 bots | current registered 1000-game standard-map simulation is 1000-0 for Garrison | honest measured evidence is recorded | public default remains constrained |

## Tests

| Test | Purpose | Bot(s) | Status | Notes |
|---|---|---|---|---|
| legality over seeds | bot chooses only legal action paths | all | covered | `tests/bots.rs`, `tests/property.rs` |
| determinism | fixed seed/view/rules/policy produce fixed decision | Level 1 | covered | `tests/bots.rs` |
| no-leak input view | hidden-info games only | all public bots | not applicable | perfect-information game; visibility tests still prove equivalent views |
| no-leak explanation/ranking | hidden-info games only | Level 1 | not applicable | explanations use public facts |
| replay/hash | bot decisions reproduce in replay | all | covered | replay tests and bot golden trace |
| explanation smoke | non-random bots explain decisions | Level 1 | covered | `tests/bots.rs` |
| decision examples | policy examples choose expected actions | Level 2 | not applicable | no Level 2 bot |

## Benchmarks

| Benchmark | Target | Current baseline | Bot(s) | Status | Notes |
|---|---:|---:|---|---|---|
| legal action generation | native smoke | recorded in [BENCHMARKS.md](BENCHMARKS.md) | all | covered | `cargo bench -p frontier_control` |
| bot decision latency | native smoke | recorded in [BENCHMARKS.md](BENCHMARKS.md) | Level 1 | covered | no search |
| playout throughput | native smoke | simulator reports throughput | Level 1 | covered | `simulate --game frontier_control` |
| WASM/browser smoke | catalog/action/bot path | covered by GAT13FROCONASY-012 and later browser tickets | Level 1 | partial | full E2E in GAT13FROCONASY-016 |

## Simulation metrics

| Run | Bots | Seeds/games | Metrics recorded | Failures | Notes |
|---|---|---:|---|---|---|
| 2026-06-11 standard map | Garrison Level 1 vs Prospector Level 1 | 1000 | Garrison wins 1000, Prospectors wins 0, average score 16-0, average rounds 8, average length 32 | none | Outside A5 balance band; retune note recorded. |

## Public default suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes | validation tests pass |
| bot does not look broken | constrained | bots complete games legally but current balance is poor |
| bot is fair under information rules | yes | public projection only |
| explanations are safe and useful | yes | faction-specific public rationales |
| latency fits public UX | yes | benchmark smoke exists |
| known weaknesses acceptable | constrained | acceptable for Gate 13 proof, not final public balance |
| public default decision | constrained | usable for smoke/demo; retune before polished public claim |
