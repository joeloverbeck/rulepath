# Plain Tricks Rules

Game ID: `plain_tricks`

Public display name: `Plain Tricks`

Implemented variant: `plain_tricks_standard`

Rules version: `plain-tricks-rules-v1`

Prepared by: `Codex`

Created: 2026-06-09

Last updated: 2026-06-09

## Rule authority

This document is the original Rulepath rules summary for the implemented
variant. Sources belong in `SOURCES.md`; this document states the Rulepath
implementation contract.

Stable rule IDs are requirements. They must remain stable after implementation
unless intentionally migrated with a migration note and corresponding updates in
`RULE-COVERAGE.md`, traces, tests, and docs.

## Metadata

| Field | Value |
|---|---|
| game id | `plain_tricks` |
| public display name | `Plain Tricks` |
| variant | `plain_tricks_standard` |
| rules version | `plain-tricks-rules-v1` |
| source note | `games/plain_tricks/docs/SOURCES.md` |
| coverage matrix | `games/plain_tricks/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/plain_tricks/docs/MECHANICS.md` |
| implementation admission | `games/plain_tricks/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

Plain Tricks is a two-seat, original Rulepath hidden-hand trick-taking
microgame. Each round deals private hands from a small deterministic deck, then
seats play six tricks under a must-follow-suit rule. The game proves
lead/follow legality, trick resolution, trick-winner-led turn order, round
scoring, deal rotation, viewer-safe hidden-hand behavior, deterministic replay,
and Rust-owned bot decisions.

Rust owns setup, legal actions, validation, private hand storage, trick
resolution, scoring, deal rotation, semantic effects, replay behavior,
visibility projection, and bot decisions. TypeScript may present only
Rust/WASM output.

The game does not implement a general card engine, trump suits, bidding,
partnerships, penalty-card scoring, configurable variants, more than two seats,
or shared engine vocabulary for this game's local nouns.

## Implemented variant

The only shipped variant is `plain_tricks_standard`.

| Field | Value |
|---|---|
| seats | `seat_0`, `seat_1` |
| deck | eighteen cards: three suits by six ranks, one copy each |
| suits | `gale`, `river`, `ember` |
| ranks | `1` through `6`; rank 6 is highest |
| private hands | six cards per seat per round |
| tail | six undealt cards per round; internal only and never revealed |
| tricks | six tricks per round |
| rounds | two rounds, freshly shuffled from one continuing seeded RNG stream |
| round-1 first leader | `seat_0` |
| round-2 first leader | `seat_1` |
| scoring | one point per trick won |
| terminal outcomes | most total points wins; 6-6 is `Split` |
| maximum gameplay actions | exactly 24 card plays |

## Components and game-local vocabulary

Game nouns in this section belong to `games/plain_tricks` only. They do not
authorize `card`, `deck`, `hand`, `suit`, `rank`, `trick`, `lead`, `follow`,
`void`, `deal`, or similar nouns in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `PT-COMP-001` | seat | One of the two players, `seat_0` or `seat_1`. | public | No other player counts ship in this variant. |
| `PT-COMP-002` | trick card | One game-local card with a stable id, suit, rank, and neutral label. | private, hidden, or public depending on play timing | Card identity becomes public only when played, except to the owner in a private hand view. |
| `PT-COMP-003` | suit | One of `gale`, `river`, or `ember`. | visible only with a visible card | The led suit controls follower legality and trick resolution. |
| `PT-COMP-004` | rank | One of `1`, `2`, `3`, `4`, `5`, or `6`; higher rank wins within the led suit. | visible only with a visible card | Single-copy cards prevent equal-rank same-suit ties in a trick. |
| `PT-COMP-005` | private hand | The six cards owned by a seat for the current round. | owner only until each card is played | Opponent and observer views expose counts only. |
| `PT-COMP-006` | tail | The six cards not dealt to either seat in a round. | internal only | Tail identities are never revealed, including at terminal. |
| `PT-COMP-007` | current trick | The one- or two-card play sequence being resolved. | public after cards are played | The first card establishes the led suit. |
| `PT-COMP-008` | trick history | Resolved tricks with played cards, winner, and running trick counts. | public | Contains only cards already played. |
| `PT-COMP-009` | round score | Tricks won by each seat in the current round. | public | Each won trick is one point. |
| `PT-COMP-010` | match total | Sum of both round scores for each seat. | public | Determines the terminal outcome. |

### Standard card list

Static data may carry card IDs, labels, suit/rank metadata, fixtures, and
version declarations. Static data must not carry legality, trick comparison,
scoring, deal routing, bot policy, selectors, triggers, or formula behavior.

| Stable order | Card ID | Suit | Rank | Label |
|---:|---|---|---:|---|
| 1 | `gale_1` | `gale` | 1 | Gale 1 |
| 2 | `gale_2` | `gale` | 2 | Gale 2 |
| 3 | `gale_3` | `gale` | 3 | Gale 3 |
| 4 | `gale_4` | `gale` | 4 | Gale 4 |
| 5 | `gale_5` | `gale` | 5 | Gale 5 |
| 6 | `gale_6` | `gale` | 6 | Gale 6 |
| 7 | `river_1` | `river` | 1 | River 1 |
| 8 | `river_2` | `river` | 2 | River 2 |
| 9 | `river_3` | `river` | 3 | River 3 |
| 10 | `river_4` | `river` | 4 | River 4 |
| 11 | `river_5` | `river` | 5 | River 5 |
| 12 | `river_6` | `river` | 6 | River 6 |
| 13 | `ember_1` | `ember` | 1 | Ember 1 |
| 14 | `ember_2` | `ember` | 2 | Ember 2 |
| 15 | `ember_3` | `ember` | 3 | Ember 3 |
| 16 | `ember_4` | `ember` | 4 | Ember 4 |
| 17 | `ember_5` | `ember` | 5 | Ember 5 |
| 18 | `ember_6` | `ember` | 6 | Ember 6 |

## Setup and deal rotation

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `PT-SETUP-001` | Create exactly two seats, `seat_0` and `seat_1`. | deterministic | public | No other seat counts ship. |
| `PT-SETUP-002` | Construct the eighteen-card deck in stable order, then shuffle it with Rulepath's deterministic seeded RNG discipline. | seeded deterministic | internal until projection | Same seed and rules version must reproduce the deal. |
| `PT-SETUP-003` | Deal six cards to `seat_0`, six cards to `seat_1`, and leave six cards as the internal tail. | seeded deterministic | mixed | Each owner sees only their own hand; the tail is never visible. |
| `PT-SETUP-004` | Round 1 starts with `seat_0` leading trick 1. | deterministic | public | The current trick starts empty. |
| `PT-SETUP-005` | Round 2 reshuffles the full deck from the continuing RNG stream, redeals, and starts with `seat_1` leading trick 1. | seeded deterministic | mixed | Round 1 hands and tail do not carry into round 2. |
| `PT-SETUP-006` | Emit private deal effects for each owner's hand and public deal effects containing only counts, round index, and leader. | deterministic | mixed | Public setup effects must not contain unplayed card ids, suits, ranks, or labels. |

## Turn, trick, and round sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `PT-TURN-001` | Start of a trick. | current leader | The leader may play any card from hand. | A valid lead card is applied. |
| `PT-TURN-002` | A trick has a led card and no follower card. | other seat | The follower must obey the follow-suit legality rules. | A valid follower card is applied. |
| `PT-TURN-003` | A trick has two cards. | none during resolution | Rust resolves the trick, updates round score, and records public history. | Resolution completes. |
| `PT-TURN-004` | A non-final trick resolves. | trick winner | The trick winner leads the next trick. | The next lead card is applied. |
| `PT-TURN-005` | Trick 6 of round 1 resolves. | none during round close | Rust scores the round, rotates the deal, and starts round 2 with `seat_1` leading. | Round-2 setup completes. |
| `PT-TURN-006` | Trick 6 of round 2 resolves. | none during terminal resolution | Rust scores the round and resolves the match. | Terminal outcome is recorded. |
| `PT-TURN-007` | Terminal state. | none | Expose outcome and no normal gameplay actions. | No further gameplay action advances the game. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality,
follow-suit availability, trick winner, scoring, rotation, terminal outcome,
tie handling, hidden-info filtering, or bot policy.

| Rule ID | Situation | Legal card(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `PT-ACT-001` | Actor leads the trick. | Every card in the actor's hand. | `play/<card-id>` leaf paths | Leader is unconstrained by suit because no suit is led yet. |
| `PT-ACT-002` | Actor follows and holds at least one card of the led suit. | Exactly the actor's held cards of the led suit. | `play/<card-id>` leaf paths | Off-suit cards are unavailable and rejected if submitted. |
| `PT-ACT-003` | Actor follows and holds no card of the led suit. | Every card in the actor's hand. | `play/<card-id>` leaf paths | This free discard publicly implies void in the led suit. |
| `PT-ACT-004` | Terminal state. | none | empty gameplay tree | Terminal states expose no normal gameplay actions. |
| `PT-ACT-005` | Any generated legal action. | safe metadata only | action metadata | Metadata may include action family, actor, round index, trick index, led suit/card after the lead, and the actor's own playable card in that actor's own tree. It must not include opponent hand, tail, unplayed non-actor cards, hidden inference, or bot ranking. |
| `PT-ACT-006` | A non-actor viewer requests actions. | none | empty tree | Non-actor viewers must not receive card leaves for another seat's private hand. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `PT-RESTRICT-001` | Unknown or non-seat actor submits a gameplay action. | Reject without mutation. | Viewer-safe wrong-actor diagnostic. | Only `seat_0` and `seat_1` can act. |
| `PT-RESTRICT-002` | The wrong seat submits while another seat is active. | Reject without mutation. | Viewer-safe wrong-seat diagnostic. | Diagnostic may name public active seat only. |
| `PT-RESTRICT-003` | A malformed or unavailable action path is submitted. | Reject without mutation. | Viewer-safe unavailable-action diagnostic. | Diagnostic must not include hidden card facts. |
| `PT-RESTRICT-004` | A stale command is submitted after state has advanced. | Reject without mutation. | Viewer-safe stale-command diagnostic. | Replay/hash state must not change. |
| `PT-RESTRICT-005` | A card not in the actor's hand is submitted. | Reject without mutation. | Viewer-safe not-in-hand diagnostic that echoes only the submitted card id when it came from the actor command. | The diagnostic must not reveal any held alternative. |
| `PT-RESTRICT-006` | A follower submits off-suit while holding at least one led-suit card. | Reject without mutation. | Viewer-safe must-follow diagnostic such as "a card of the led suit must be played." | The diagnostic must not name the held led-suit card(s). |
| `PT-RESTRICT-007` | A gameplay action is submitted in terminal state. | Reject without mutation. | Viewer-safe terminal-state diagnostic. | Outcome is final. |

## Trick resolution

| Rule ID | Resolution rule | Timing | Edge case | Notes |
|---|---|---|---|---|
| `PT-TRICK-001` | The leader's played card establishes the led suit for the trick. | lead play | none | The follower legality set is computed from the led suit and follower hand. |
| `PT-TRICK-002` | If both played cards share the led suit, the higher rank wins the trick. | after follower play | no equal rank within suit exists | Rank 6 is highest and rank 1 is lowest. |
| `PT-TRICK-003` | If the follower plays off-suit, the leader wins the trick. | after follower play | off-suit card rank is ignored | Off-suit cards never win. |
| `PT-TRICK-004` | The trick winner leads the next trick unless the round is complete. | trick resolution | final trick closes the round | Turn order is Rust-owned. |

## Scoring and accounting

| Rule ID | Scoring rule | Timing | Edge case | Notes |
|---|---|---|---|---|
| `PT-SCORE-001` | Each won trick adds one point to the winning seat's current-round score. | trick resolution | none | A round contains exactly six points. |
| `PT-SCORE-002` | After six tricks, record the round score and add it to match totals. | round close | none | Across two rounds, total match points always sum to 12. |
| `PT-SCORE-003` | The first round closes into a fresh round-2 deal and leader rotation. | round-1 close | none | The full deck is reshuffled from the continuing RNG stream. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `PT-END-001` | Round 2 closes and one seat has more total points. | `TrickWin` for the higher total. | `PT-END-002` if totals are equal | Terminal rationale may cite round scores and totals. |
| `PT-END-002` | Round 2 closes and totals are 6-6. | `Split` with `each = 6` | exact split | There is no priority-seat tiebreaker. |
| `PT-END-003` | Terminal state is reached. | no further gameplay actions | none | Terminal does not reveal the tail. |

## Visibility and private information

Public/browser payloads must not reveal hidden information through public views,
seat views, action trees, previews, diagnostics, effect logs, DOM attributes,
test IDs, logs, local storage, replay exports, bot explanations, candidate
rankings, or dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `PT-VIS-001` | Round index, trick index, leader, active seat, scores, totals, hand counts, current played cards, resolved trick history, and terminal status. | observer and both seat viewers | after setup and after each projection | public view, seat view, action tree metadata, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, dev inspector | These are safe public facts. |
| `PT-VIS-002` | A seat's unplayed hand cards. | owning seat only | while the cards remain in that seat's hand | public view, opponent view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | Opponent and observer payloads must not contain unplayed card ids, suits, ranks, or labels. |
| `PT-VIS-003` | A card played to the current trick. | all viewers | from the moment it is played | public view, effects, replay export, DOM | Played cards remain public in trick history. |
| `PT-VIS-004` | Tail cards. | no browser-facing viewer | never | public view, seat view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | Native internal tests may inspect the tail under test authority. |
| `PT-VIS-005` | Void information. | all viewers only by implication from an off-suit play | when the follower legally plays off-suit | view summaries and history | Views carry no explicit opponent-void flags. |
| `PT-VIS-006` | Legal card choices. | active actor through Rust-authorized action tree | nonterminal active turn | action tree and controls | The actor's tree may name only that actor's own playable cards. |
| `PT-VIS-007` | Bot rationale and candidate ranking. | public only if projected by Rust as viewer-safe text/data | after bot decision or diagnostics | bot explanation, dev inspector, logs, replay export | Public rationale may cite legal action family and public history; private actor rationale may cite own-hand reasoning only. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `PT-RNG-001` | Plain Tricks uses deterministic seeded shuffle for each round deal and no later random draw outside bot choice. | Same seed, rules version, variant, and command sequence must reproduce internal state and effects. | internal for hidden deal facts | Round 2 continues from the same RNG stream. |
| `PT-RNG-002` | Public replay export is viewer-scoped and redacted. | Public exports must not include seed material that reconstructs hands or tail. | public export is redacted | Seat-scoped export may include only that seat's own visible hand observations at the times they were visible. |
| `PT-RNG-003` | Serialization order must remain stable. | Golden traces and replay checks must fail on unintended ordering changes. | mixed | Stable order covers cards, actions, effects, views, and summaries. |

## Bot-relevant non-authoritative strategy notes

These notes describe intended product behavior, not extra legal authority.
Implemented bots must choose from the Rust legal tree and validate through the
normal action path.

| Rule ID | Strategy note | Allowed input | Forbidden input |
|---|---|---|---|
| `PT-BOT-001` | A random-legal bot may select any legal leaf from its current action tree with deterministic tie-breaking. | legal action tree and bot RNG stream | direct state mutation or illegal fallback |
| `PT-BOT-002` | An authored bot may prefer cheap winners when following, low discards when losing, and public-history-informed leads. | own hand, legal tree, current trick, public trick history, scores, round/trick index | opponent hand, tail, seed reconstruction, hidden-hand enumeration, sampled deals, MCTS, ISMCTS, Monte Carlo, ML, or RL |

## Variant posture and out-of-scope rules

| Rule ID | Boundary | Rulepath position | Notes |
|---|---|---|---|
| `PT-VAR-001` | Public variant | `plain_tricks_standard` is the only shipped variant. | Variant data may label the game but cannot define behavior. |
| `PT-VAR-002` | Public naming | Public copy uses **Plain Tricks** as an original neutral Rulepath name. | The game is not branded as Whist, Hearts, or any commercial card product. |
| `PT-OOS-001` | Trump, bidding, partnerships, penalties, passing, exposed-card dummy play, and 3+ seats. | out of scope | Adding any of these requires a later spec and evidence. |
| `PT-OOS-002` | General trick-taking or card-game framework. | out of scope | Local implementation does not authorize engine vocabulary or helpers. |
| `PT-OOS-003` | Static-data rule behavior. | forbidden | No formulas, selectors, triggers, scripts, loops, or tactical policy in data. |
| `PT-OOS-004` | Solver or learning bots. | forbidden for public v1/v2 | No MCTS, ISMCTS, Monte Carlo, ML, RL, or hidden-state sampling. |

## Rule-ID migration log

No rule IDs have been migrated.
