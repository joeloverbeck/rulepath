# Event Frontier AI

Game ID: `event_frontier`

Implemented variant: `event_frontier_standard`

Rules version: `event-frontier-rules-v1`

Last updated: 2026-06-12

Prepared by: `Codex`

## Purpose

This document is the Event Frontier bot registry and status page. It links to
the detailed competent-player and bot-evidence docs instead of duplicating the
full strategy pack.

Public v1/v2 bots must not use MCTS, ISMCTS, Monte Carlo, ML, RL, hidden-state
sampling, or unrevealed deck order.

## Bot summary

| Bot | Level | Policy/version | Public default? | Information access | Status | Evidence |
|---|---:|---|---:|---|---|---|
| random legal | 0 | `event_frontier_random_legal_v0` | no | legal action tree only | implemented and tested | `games/event_frontier/tests/bots.rs`, `simulate` |
| Charter baseline | 1 | `event_frontier_charter_level1_v1` | yes for Charter seat | public view plus legal action tree | implemented and tested | `BOT-STRATEGY-EVIDENCE-PACK.md` |
| Freeholder baseline | 1 | `event_frontier_freeholders_level1_v1` | yes for Freeholder seat | public view plus legal action tree | implemented and tested | `BOT-STRATEGY-EVIDENCE-PACK.md` |
| authored policy | 2 | not implemented | no | not applicable | deferred | Level 1 is the current public bot target. |
| shallow deterministic search | 3 | not allowed | no | not applicable | not allowed | Hidden-order game and public bot law exclude search. |

## Level 0: random legal bot

| Item | Decision/evidence |
|---|---|
| legal action API used | `legal_action_tree` leaves collected from Rust |
| deterministic seed behavior | `EventFrontierRandomBot::new(Seed(...))` selects deterministically from the legal tree |
| action selection method | random legal among legal action paths using deterministic seed |
| simulation tests | `cargo run -p simulate -- --game event_frontier --games 1000` |
| legality tests | `level0_and_level1_bots_choose_validating_actions_over_many_seeds` |
| replay/hash tests | `bot-vs-bot-full-game.trace.json` |
| known limitations | random; not competent |
| public explanation text | selected a seeded random legal Event Frontier action |
| benchmark evidence | `bot_l1_choice_*` and `full_random_playout` benchmark identities cover bot pressure paths |

## Level 1: rule-informed baseline bot

| Item | Decision/evidence |
|---|---|
| policy name/version | `event_frontier_charter_level1_v1`, `event_frontier_freeholders_level1_v1` |
| decision order summary | choose favorable public events, then public victory/block/resource operations, then pass/fallback |
| immediate tactics | Charter denies caches and builds majorities; Freeholders build caches and break majorities |
| mandatory rule handling | every chosen path validates through `validate_bot_decision` and `validate_command` |
| tie-break method | deterministic public tuple ordering, then raw action path |
| information access | public projection and legal tree only |
| explanation examples | see `BOT-STRATEGY-EVIDENCE-PACK.md#public-explanation-examples` |
| tests | `cargo test -p event_frontier --test bots` |
| benchmarks | `bot_l1_choice_charter`, `bot_l1_choice_freeholders`, `full_random_playout` |
| public suitability | suitable for Gate 14 demo; balance probe is inside the 35-65 percent band |

## Level 2: authored policy bot

Required evidence pack: `games/event_frontier/docs/BOT-STRATEGY-EVIDENCE-PACK.md`

Competent-player analysis:
`games/event_frontier/docs/COMPETENT-PLAYER.md`

| Item | Summary only |
|---|---|
| policy name/version | not implemented |
| evidence pack status | complete for Level 1, not a Level 2 authorization |
| phase model | current public event, first/second choice, Reckoning, terminal |
| candidate extraction | public legal leaves grouped by action family and site facts |
| lexicographic priority vector | Level 1 uses fixed public tuple ranking for operations |
| bounded scoring tie-breakers | deterministic path tie-break only |
| deterministic seeded tie-break | Level 1 currently does not use random tie-breaks |
| style profiles | none |
| explanation contract | public facts only; no undrawn deck order |
| known weaknesses | no multi-card hidden-order planning or search |
| public default suitability | Level 1 is suitable; Level 2 is deferred |

## Level 3: shallow deterministic search

| Requirement | Status | Evidence |
|---|---|---|
| perfect-information game | no | Undrawn deck order beyond next card is hidden. |
| small enough search space | not evaluated | Search is out of scope. |
| deterministic limits | not applicable | Search is out of scope. |
| documented evaluator | no | Not planned. |
| fallback policy | Level 1 | Existing scripted policy is the public default. |
| explanation says search was used | not applicable | Search is forbidden here. |
| no hidden-information search | covered by exclusion | `EF-BOT-*`, `EF-VIS-002` |
| ADR required? | yes for any future search | Public bot law forbids current use. |

## Exact information access table

| Information | Human acting seat sees? | Bot sees? | Public observer sees? | Tests/notes |
|---|---:|---:|---:|---|
| legal action tree | yes | yes | no gameplay authority | `tests/bots.rs` |
| public board/state | yes | yes | yes | one shared public projection |
| own private hand/role/zone | not applicable | not applicable | not applicable | no private hands or roles |
| opponent private hand/role/zone | not applicable | not applicable | not applicable | no private hands or roles |
| unrevealed deck/order beyond next card | no | no | no | `bot_inputs_and_explanations_do_not_expose_undrawn_deck_order` |
| current and next public card | yes | yes | yes | safe public facts |
| private logs | not applicable | not applicable | not applicable | public effects are viewer-filtered |
| dev/test full state | no for public bot | no for public bot | no | native tests only |

## Decision order summary

| Bot | Decision order |
|---|---|
| random legal | collect legal leaves, choose deterministically by seed |
| Charter Level 1 | favorable Charter event, then `writ`, `survey`, `fortify`, pass/fallback |
| Freeholder Level 1 | favorable Freeholder event, then `cache`, `trek` toward caches, `rally`/spread, pass/fallback |

## Explanation examples

| Bot | Situation | Example explanation | Hidden-info safe? | Test |
|---|---|---|---:|---|
| random legal | any legal action tree | Selected a seeded random legal Event Frontier action. | yes | bot legality tests |
| Charter Level 1 | public cache threat | Charter wrote down caches at a public site to deny a Freeholder cache threat. | yes | bot evidence examples |
| Freeholder Level 1 | legal cache opportunity | Freeholders laid a cache at a public site to move toward the cache threshold. | yes | bot evidence examples |

## Known weaknesses

| Bot | Weakness | Why acceptable | Mitigation/future trigger |
|---|---|---|---|
| random legal | poor strategy | Required baseline only | use Level 1 for public demo |
| Level 1 policies | no deep tempo/search planning | Public v1/v2 forbids hidden-state sampling and search pressure | future ADR if public bot law changes |
| Level 1 policies | may miss multi-card plans beyond public next card | Hidden deck tail cannot be used | keep explanations honest and public |

## Tests

| Test | Purpose | Bot(s) | Status | Notes |
|---|---|---|---|---|
| legality over seeds | bot chooses only legal action paths | all | covered | `level0_and_level1_bots_choose_validating_actions_over_many_seeds` |
| determinism | fixed public input produces fixed decision | Level 1 | covered | `level1_bots_are_deterministic_for_same_public_inputs` |
| no-leak input view | hidden-order game | all public bots | covered | bot inputs omit undrawn deck order |
| no-leak explanation/ranking | hidden-order game | Level 1 | covered | explanations use public facts only |
| replay/hash | bot decisions reproduce in replay | all | covered | `bot-vs-bot-full-game.trace.json` |
| explanation smoke | non-random bots explain decisions | Level 1 | covered | evidence pack examples |
| decision examples | policy examples choose expected actions | Level 1 | covered | favorable event and operation-family tests |

## Benchmarks

| Benchmark | Target | Current baseline | Bot(s) | Status | Notes |
|---|---:|---:|---|---|---|
| legal action generation | smoke floor | `benches/thresholds.json` | all | covered | action-tree benchmark identities exist |
| candidate extraction | smoke floor | `benches/thresholds.json` | Level 1 | covered | covered by bot decision benchmarks |
| bot decision latency | smoke floor | `benches/thresholds.json` | Level 1 | covered | `bot_l1_choice_charter`, `bot_l1_choice_freeholders` |
| playout throughput | 100+ turns/sec roadmap target; smoke threshold now | `benches/thresholds.json` | Level 1 | covered | `full_random_playout` |
| explanation generation | no separate threshold | not applicable | Level 1 | covered by tests | explanation strings are static public text |
| WASM/browser smoke | ticket 018 | pending browser board ticket | public bots | planned | code half lands in ticket 017/018 |

## Simulation metrics

| Run | Bots | Seeds/games | Metrics recorded | Failures | Notes |
|---|---|---:|---|---|---|
| `cargo run -p simulate -- --game event_frontier --games 1000` | Charter Level 1 vs Freeholder Level 1 | 1,000 | wins, victory types, average cards, average scores, pass rate, throughput | 0 action-cap failures | Charter 602, Freeholders 398 after retune |

## Public default suitability

| Check | Status | Notes |
|---|---|---|
| bot is legal and deterministic | yes | choices validate through Rust |
| bot does not look broken | yes | retuned Level 1 probe has both factions winning |
| bot is fair under information rules | yes | uses public projection only |
| explanations are safe and useful | yes | examples cite public facts only |
| latency fits public UX | yes | smoke benchmark identities exist |
| known weaknesses acceptable | yes | no search is expected for public v1/v2 |
| public default decision | yes | use faction Level 1 policies for public demo |
