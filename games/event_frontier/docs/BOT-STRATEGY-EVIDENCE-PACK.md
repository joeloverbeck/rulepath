# Event Frontier Bot Strategy Evidence Pack

Game ID: `event_frontier`

Implemented variant: `event_frontier_standard`

Rules version: `event-frontier-rules-v1`

Bot target: Level 1 authored scripted policies

Policy names/versions:
`event_frontier_charter_level1_v1`,
`event_frontier_freeholders_level1_v1`

Prepared by: Codex

Date: 2026-06-12

## Purpose and gate

This pack records the implemented Level 1 Event Frontier bot policies and the
balance evidence for Gate 14. It consumes `COMPETENT-PLAYER.md`; it does not
claim a Level 2 bot and does not authorize search, MCTS, ISMCTS, Monte Carlo,
ML, or RL.

Both Level 1 policies use only public projection data plus the legal action
tree. Chosen paths are submitted through normal validation.

## Source documents consumed

| Document/source | Path/reference | Status | Notes |
|---|---|---|---|
| Rules | `games/event_frontier/docs/RULES.md` | read | Stable rule IDs and public/hidden boundary. |
| Competent-player analysis | `games/event_frontier/docs/COMPETENT-PLAYER.md` | read | Strategy and balance retune record. |
| Bot implementation | `games/event_frontier/src/bots.rs` | read | Source of truth for priority tables below. |
| Bot tests | `games/event_frontier/tests/bots.rs` | read | Legality, determinism, table, and no-leak tests. |
| Golden bot trace | `games/event_frontier/tests/golden_traces/bot-vs-bot-full-game.trace.json` | read | Replay/hash evidence for bot playout fixture. |

## Explicit public v1/v2 exclusions

The policies do not use omniscient state, hidden-state shortcuts, future random
outcomes, unbounded weights, static-data tactical conditions, random blunders,
MCTS, ISMCTS, Monte Carlo, ML, or RL.

## Exact bot input view

| Input item | Included? | Source | Visible to acting seat? | No-leak evidence |
|---|---:|---|---:|---|
| Legal action paths | yes | `legal_action_tree` leaves | yes | Decisions validate through `validate_bot_decision`. |
| Public view | yes | `project_view` | yes | Seat and observer projections are output-equivalent. |
| Policy seed | yes | bot constructor | not game information | Used only for random-legal Level 0; Level 1 is deterministic for same public input. |
| Hidden undrawn deck order | no | forbidden | no | `bot_inputs_and_explanations_do_not_expose_undrawn_deck_order`. |
| Future random outcomes | no | forbidden | no | No sampled search or hidden-state model. |

## Legal action API used

| API/contract | Purpose | Determinism requirement | Tests |
|---|---|---|---|
| `legal_action_tree` | Enumerates legal action leaves from Rust state and actor. | Same state and actor produce same candidate order. | `level0_and_level1_bots_choose_validating_actions_over_many_seeds` |
| `validate_command` / `validate_bot_decision` | Confirms chosen action paths use normal command validation. | A bot cannot bypass legality. | `level0_and_level1_bots_choose_validating_actions_over_many_seeds` |
| `project_view` | Supplies public facts used by Level 1 policy. | Does not expose undrawn deck order. | `bot_inputs_and_explanations_do_not_expose_undrawn_deck_order` |

## Layer 1 family choice

Both factions first choose an action family:

| Priority | Condition from public input | Choice |
|---:|---|---|
| 1 | Event is legal and current public event favors the acting faction. | `event` |
| 2 | An operation is legal and own victory distance is at most 2, opponent victory distance is at most 2, or own resources exceed 1. | `operation` |
| 3 | Pass is legal and own resources are at most 1. | `pass` |
| 4 | Any operation remains legal. | `operation` |
| 5 | Event remains legal. | `event` |
| 6 | Otherwise. | `pass` |

Favorable events are closed Rust match arms in `event_favors`. Charter-favored
cards are `ef_border_survey`, `ef_depot_grants`, `ef_charter_audit`,
`ef_agents_recall`, and `ef_granite_pass_snows`. Freeholder-favored cards are
`ef_river_mists`, `ef_high_meadow_fair`, `ef_freeholder_moot`,
`ef_old_mill_strike`, and `ef_cache_boom`.

## Charter operation priorities

| Priority | Candidate group | Ranking tuple, highest wins | Explanation fragment |
|---:|---|---|---|
| 1 | `writ` | site cache count, total selected caches, reverse site index, raw path | Wrote down caches to deny a Freeholder cache threat. |
| 2 | `survey` | site where `agents + depot <= settlers`, site has settlers, reverse site index, raw path | Surveyed to extend public site majority pressure. |
| 3 | `fortify` | settlers at site, agents at site, reverse site index, raw path | Fortified a contested held site. |
| 4 | fallback | pass if legal | Passed to save funds. |

## Freeholder operation priorities

| Priority | Candidate group | Ranking tuple, highest wins | Explanation fragment |
|---:|---|---|---|
| 1 | `cache` | cache count plus one, settlers minus agents, reverse site index, raw path | Laid a cache toward the public cache threshold. |
| 2 | `trek` toward cache | target has cache and agents, target cache count, reverse site index, raw path | Trekked to escort a public exposed cache. |
| 3 | `rally` or `trek` spread | target has more agents than settlers, emptiest settler count, reverse site index, raw path | Spread presence to break Charter majorities. |
| 4 | fallback | pass if legal | Passed to save provisions. |

## Deterministic tie-break

| Item | Decision |
|---|---|
| Level 1 seed source | Constructor seed is accepted for interface consistency but no random Level 1 tie-break is currently used. |
| Tie-break input identity | Public candidate path and public site facts. |
| Stable ordering rule | Lexicographic tuple comparison, then raw action path string. |
| Replay/hash interaction | Golden traces include a bot-vs-bot full-game fixture. |

## Public explanation examples

| Situation | Chosen action | Public explanation | Hidden-info safe? | Rule IDs |
|---|---|---|---:|---|
| Charter sees a legal cache-removal operation. | `operation/writ/...` | Charter wrote down caches at a public site to deny a Freeholder cache threat. | yes | `EF-OP-006`, `EF-END-002` |
| Charter sees a favorable Charter event. | `event` | Charter resolved the public event because the current card favors Charter. | yes | `EF-TURN-003` |
| Freeholders see a legal cache action. | `operation/cache/...` | Freeholders laid a cache at a public site to move toward the cache threshold. | yes | `EF-OP-008`, `EF-END-002` |
| Freeholders are low on provisions and can pass. | `pass` | Freeholders passed to save provisions for later public operations. | yes | `EF-SCORE-002` |

## Decision examples and expected choices

| Example ID | Situation | Expected choice | Test name |
|---|---|---|---|
| `BOT-EX-001` | Charter current card is `BorderSurvey`. | `event` | `favorable_public_events_match_the_decision_table` |
| `BOT-EX-002` | Freeholders current card is `HighMeadowFair`. | `event` | `favorable_public_events_match_the_decision_table` |
| `BOT-EX-003` | Charter has operation resources on an unfavored card. | One of `survey`, `fortify`, or `writ`. | `faction_level1_policies_rank_distinct_operation_families` |
| `BOT-EX-004` | Freeholders have operation resources on an unfavored card. | One of `cache`, `trek`, or `rally`. | `faction_level1_policies_rank_distinct_operation_families` |

## Balance and retune evidence

The initial standard scenario missed Assumption A5 in a 1,000-game Level 1
versus Level 1 run: Charter 865 wins, Freeholders 135 wins, Charter instant 212,
Freeholder instant 0, final fallback 788.

The standard scenario constants were retuned:

| Constant | Retuned standard value |
|---|---|
| starting resources | Charter 2 funds, Freeholders 4 provisions |
| Charter instant threshold | 4 majority sites |
| Freeholder instant threshold | 6 public caches |
| starting settlers | Landing 3, High Meadow 1 |
| starting caches | Landing 1, High Meadow 1 |

The post-retune probe used seeds `0..999`, Level 1 bots for both factions,
action cap 96, and deterministic bot seed
`seed * 0x9e3779b97f4a7c15 + action_index`.

| Result | Count | Percent |
|---|---:|---:|
| Charter wins | 602 | 60.2% |
| Freeholder wins | 398 | 39.8% |
| Charter instant victories | 110 | 11.0% |
| Freeholder instant victories | 22 | 2.2% |
| Final fallback victories | 868 | 86.8% |
| Capped nonterminal games | 0 | 0.0% |

The retuned result is inside the required 35-65% per-faction band and includes
Charter instant, Freeholder instant, and final fallback victory types.

## Test plan

| Test area | Specific tests/evidence |
|---|---|
| legality | `cargo test -p event_frontier --test bots` |
| determinism | `level1_bots_are_deterministic_for_same_public_inputs` |
| no hidden-state access | `bot_inputs_and_explanations_do_not_expose_undrawn_deck_order` |
| priority examples | `favorable_public_events_match_the_decision_table`, `faction_level1_policies_rank_distinct_operation_families` |
| replay/hash | `cargo test -p event_frontier --test golden_traces` |
| balance | 1,000-game Level 1 probe recorded above |
