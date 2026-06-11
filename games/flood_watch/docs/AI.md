# flood_watch AI

Game ID: `flood_watch`

Implemented variant: `flood_watch_standard`

Rules version: `flood-watch-rules-v1`

Last updated: 2026-06-11

Prepared by: Codex

## Purpose

This document records the Flood Watch bot registry and public bot posture. Full strategy evidence lives in [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), with player-level context in [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md).

## Bot summary

| Bot | Level | Policy/version | Public default? | Information access | Status | Evidence |
|---|---:|---|---:|---|---|---|
| random legal | 0 | `flood_watch_random_legal_v0` / version 1 | no | legal action tree only | implemented and tested | `tests/bots.rs` |
| public priority baseline | 1 | `flood_watch_level1_public_priority_v1` / version 1 | yes | public projection and legal action tree | implemented and tested | `tests/bots.rs`; `BOT-STRATEGY-EVIDENCE-PACK.md` |
| authored policy | 2 | not planned | no | not applicable | deferred | Level 1 is the public default for Gate 12. |
| shallow deterministic search | 3 | not allowed | no | not applicable | not allowed | Hidden event deck forbids search over hidden order. |

Public v1/v2 bots MUST NOT use MCTS, ISMCTS, Monte Carlo-style bots, ML, or RL.

## Level 0: random legal bot

| Item | Decision/evidence |
|---|---|
| legal action API used | `legal_action_tree` |
| deterministic seed behavior | deterministic selection from the legal leaves with `Seed` |
| action selection method | random legal among legal action paths |
| simulation tests | `cargo run -p simulate -- --game flood_watch --games 1000` |
| legality tests | `cargo test -p flood_watch --test bots` |
| replay/hash tests | `cargo test -p flood_watch --test replay` |
| known limitations | random; not competent |
| public explanation text | selected a seeded random legal Flood Watch action |
| benchmark evidence | `cargo bench -p flood_watch` |

## Level 1: rule-informed baseline bot

| Item | Decision/evidence |
|---|---|
| policy name/version | `flood_watch_level1_public_priority_v1` / version 1 |
| decision order summary | rescue imminent loss, answer forecast threat, reinforce expected pressure, forecast, end turn |
| immediate tactics | avoid shared loss by bailing districts at level 2 or higher |
| mandatory rule handling | legal tree and validation remain Rust-owned |
| tie-break method | deterministic public ordering |
| information access | public projection, public forecast, remaining composition counts, legal action tree |
| explanation examples | "Bailed Riverside because it is one step from shared loss." |
| tests | `tests/bots.rs` |
| benchmarks | bot latency and random playout benchmark rows in `BENCHMARKS.md` |
| public suitability | suitable for Gate 12 cooperative demo |

## Level 2: authored policy bot

Required evidence pack: [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md)

Competent-player analysis: [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md)

| Item | Summary only |
|---|---|
| policy name/version | not implemented |
| evidence pack status | complete for Level 1 |
| phase model | action phase plus environment automation |
| candidate extraction | legal action tree leaves |
| lexicographic priority vector | documented for Level 1 |
| deterministic seeded tie-break | not needed beyond public ordering for Level 1 |
| explanation contract | public facts only |
| known weaknesses | no deep planning over future hidden order |
| public default suitability | Level 1 is suitable; Level 2 deferred |

## Level 3: shallow deterministic search

| Requirement | Status | Evidence |
|---|---|---|
| perfect-information game | no | event deck order is hidden |
| small enough search space | not applicable | search disallowed |
| deterministic limits | not applicable | search disallowed |
| documented evaluator | no | not planned |
| fallback policy | Level 1 | implemented |
| explanation says search was used | not applicable | no search |
| no hidden-information search | yes | no MCTS/hidden sampling |
| ADR required? | yes for future search | foundation bot law |

## Exact information access table

| Information | Human acting seat sees? | Bot sees? | Public observer sees? | Tests/notes |
|---|---:|---:|---:|---|
| legal action tree | yes | yes | no gameplay authority | action tree tests |
| public district state | yes | yes | yes | visibility tests |
| public roles | yes | yes | yes | setup tests |
| forecast card | yes if forecast | yes if forecast | yes if forecast | forecast trace |
| remaining composition counts | yes | yes | yes | visibility tests |
| undrawn deck order | no | no | no | no-leak tests |
| internal event deck | no | no | no | test-only authority |

## Decision order summary

| Bot | Decision order |
|---|---|
| random legal | seeded random legal leaf |
| public priority baseline | 1. bail imminent inundation 2. react to forecast 3. reinforce highest expected pressure 4. forecast 5. end turn |

## Explanation examples

| Bot | Situation | Example explanation | Hidden-info safe? | Test |
|---|---|---|---:|---|
| random legal | any legal tree | Selected a seeded random legal Flood Watch action. | yes | `tests/bots.rs` |
| public priority baseline | district at level 2 | Bailed Riverside because it is one step from shared loss. | yes | `tests/bots.rs` |
| public priority baseline | no immediate danger | Forecasted with spare budget because no public district action improved the position. | yes | `tests/bots.rs` |

## Known weaknesses

| Bot | Weakness | Why acceptable | Mitigation/future trigger |
|---|---|---|---|
| random legal | plays poorly | required baseline only | not public default |
| Level 1 | does not optimize across hidden future order | hidden-order sampling is forbidden | future ADR if a stronger public bot is needed |

## Tests

| Test | Purpose | Bot(s) | Status | Notes |
|---|---|---|---|---|
| legality over seeds | bot chooses only legal action paths | all | covered | `tests/bots.rs` |
| determinism | fixed seed/view/rules/policy produce fixed decision | Level 1 | covered | `tests/bots.rs` |
| no-leak input view | hidden deck unavailable | all public bots | covered | hidden-order invariance test |
| no-leak explanation/ranking | explanations use public facts | Level 1 | covered | evidence pack and tests |
| replay/hash | bot decisions reproduce | all | covered | replay tests |
| explanation smoke | non-random bot explains decisions | Level 1 | covered | `tests/bots.rs` |

## Benchmarks

| Benchmark | Target | Current baseline | Bot(s) | Status | Notes |
|---|---:|---:|---|---|---|
| legal action generation | native smoke | recorded in `BENCHMARKS.md` | all | covered | `cargo bench -p flood_watch` |
| bot decision latency | native smoke | recorded in `BENCHMARKS.md` | Level 1 | covered | no hidden search |
| playout throughput | native smoke | recorded in `BENCHMARKS.md` | Level 1 | covered | simulator registered in GAT12FLOWATCOO-015 |
| WASM/browser smoke | catalog/action/bot path | covered by GAT12FLOWATCOO-014 | Level 1 | covered | `smoke:wasm` |

## Public default suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes | validation tests pass |
| bot does not look broken | yes | prioritizes imminent loss and public pressure |
| bot is fair under information rules | yes | no hidden deck access |
| explanations are safe and useful | yes | public facts only |
| latency fits public UX | yes | benchmarked |
| known weaknesses acceptable | yes | no hidden sampling by design |
| public default decision | yes | Level 1 is the default cooperative teammate bot |
