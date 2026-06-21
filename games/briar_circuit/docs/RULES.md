# Briar Circuit Rules

Game ID: `briar_circuit`

Public display name: `Briar Circuit`

Implemented variant: `briar_circuit_standard`

Rules version: `briar-circuit-rules-v1`

Prepared by: `Codex`

Created: 2026-06-21

Last updated: 2026-06-21

## Rule Authority

This document is the original Rulepath rules summary for the planned Briar
Circuit implementation. Sources belong in `SOURCES.md`; this document states
the Rulepath implementation contract that code, traces, coverage, bots, and UI
must satisfy.

Stable `BC-*` rule IDs are requirements. They must remain stable after
implementation unless intentionally migrated with a migration note and matching
updates in `RULE-COVERAGE.md`, traces, tests, and player-facing docs.

## Metadata

| Field | Value |
|---|---|
| game id | `briar_circuit` |
| public display name | `Briar Circuit` |
| variant | `briar_circuit_standard` |
| rules version | `briar-circuit-rules-v1` |
| source note | `games/briar_circuit/docs/SOURCES.md` |
| coverage matrix | `games/briar_circuit/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/briar_circuit/docs/MECHANICS.md` |
| implementation admission | `games/briar_circuit/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose And Scope

Briar Circuit is a fixed-four-seat, hidden-hand, trick-taking penalty game in
the classic Hearts rules family. It proves deterministic full-deck dealing,
private pass commitments, lead/follow obligations, point-card restrictions,
shoot-the-moon scoring, multi-hand threshold play, public observer projection,
seat-private views, viewer-scoped replay, and Rust-owned bot decisions.

Rust owns setup, legal actions, validation, private hand storage, pass
exchange, trick resolution, scoring, match termination, semantic effects,
visibility projection, replay behavior, and bot decisions. TypeScript may
present only Rust/WASM output.

Public presentation uses the original name Briar Circuit and neutral card-table
language. The implementation does not use copied rules prose, copied card art,
casino framing, hosted multiplayer, teams, variable seats, alternative Hearts
house rules, or a generic card/trick engine.

Game nouns in this document belong to `games/briar_circuit` only. They do not
authorize `card`, `deck`, `hand`, `suit`, `rank`, `pass`, `trick`, `heart`,
`queen`, `moon`, or related vocabulary in `engine-core`.

## Implemented Variant

The only planned Gate 16 variant is `briar_circuit_standard`.

| Field | Value |
|---|---|
| seats | exactly four independent seats, `seat_0` through `seat_3` |
| deck | standard 52-card deck, game-local representation |
| private hand size | 13 cards per seat after each deal and after each pass exchange |
| pass cycle | left, right, across, hold, repeating by hand index |
| first lead | holder of 2 clubs leads 2 clubs |
| trick size | four played cards |
| point cards | each heart is 1 point; queen of spades is 13 points |
| hand point total | 26 raw points before moon transformation |
| moon rule | capturing all 26 raw points gives the shooter 0 and each opponent 26 |
| match threshold | after a completed hand, any cumulative score at least 100 triggers evaluation |
| winner | unique lowest cumulative score |
| low-score tie | continue complete hands until the lowest score is unique |
| randomness | deterministic seeded shuffle only |
| excluded scope | variable seats, teams, bidding, trump, omnibus bonuses, moon choice, shoot the sun, hosted multiplayer, solver/search/ML bots |

## Components And Game-Local Vocabulary

Static data may carry IDs, display labels, version anchors, seat metadata,
fixtures, traces, and reports. Static data must not carry legality, pass
routing, trick comparison, point formulas, terminal logic, bot policy, visibility
selectors, triggers, conditions, or behavior scripts.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `BC-SETUP-001` | seat | One of exactly four stable participant positions, `seat_0` through `seat_3`. | public | Other seat counts are rejected by Rust with a stable diagnostic. |
| `BC-SETUP-002` | deck/card identity | A game-local 52-card ordered source with four suits and ranks 2 through ace. | internal until a card is owner-visible or publicly played | Canonical IDs and ordering support deterministic serialization. |
| `BC-DEAL-001` | private hand | The 13 cards dealt to one seat for the current hand. | owner only until each card is played | Other viewers receive counts or redacted placeholders only. |
| `BC-DEAL-002` | dealer | The seat marker that rotates clockwise after each hand; the next deal begins left of the dealer. | public | Initial dealer is `seat_0` for the standard deterministic setup. |
| `BC-PASS-001` | pass direction | The hand-index cycle: left, right, across, hold. | public direction only | Direction is public; selected card identities are not. |
| `BC-PASS-002` | pass selection | The exact three owned cards a seat commits on a pass hand. | owner only before exchange; provenance remains private | Submitted choices must be distinct and owned by the acting seat. |
| `BC-PASS-003` | atomic exchange | The point where all four committed selections are routed simultaneously. | mixed | No incoming identities are delivered before all seats confirm. |
| `BC-PASS-004` | hold hand | Every fourth hand skips passing and moves directly to play. | public | No selection or exchange exists on a hold hand. |
| `BC-PLAY-001` | opening lead | The required first play of a hand: 2 clubs by its holder. | public when played | No other card may open the hand. |
| `BC-TRICK-001` | current trick | The sequence of up to four public card plays being resolved. | public after each play | The first card establishes the led suit. |
| `BC-TRICK-002` | captured trick | The four cards awarded to the trick winner. | public card identities; point totals public | Captured records drive scoring. |

## Setup, Deal, And Pass Cycle

| Rule ID | Setup/deal rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `BC-SETUP-001` | Accept exactly four seats and reject every other seat count. | deterministic | public diagnostic | Wrong-seat-count diagnostics expose no hidden setup facts. |
| `BC-SETUP-002` | Construct the standard 52-card deck in canonical game-local order. | deterministic | internal before projection | The canonical card order is stable for IDs, action ordering, traces, and serialization. |
| `BC-DEAL-001` | Shuffle from the match seed and deal clockwise one card at a time beginning left of the dealer until each seat has 13 cards. | seeded deterministic | owner-private hands, public counts | There is no undealt remainder. |
| `BC-DEAL-002` | Rotate the dealer one seat clockwise after every completed hand. | deterministic | public | The holder of 2 clubs, not the dealer, opens play. |
| `BC-PASS-001` | Determine the pass direction from hand index modulo four: left, right, across, hold. | deterministic | public direction | The cycle repeats for the whole match. |
| `BC-PASS-004` | On a hold hand, skip pass selection and start play with the 2 clubs opening lead. | deterministic | public phase transition | No pass identities or provenance are created. |

## Pass Actions

Rust must generate and validate all pass actions. TypeScript must not decide
which cards may be selected, whether a pass is complete, or how cards are
routed.

| Rule ID | Situation | Legal action(s) | Rust-owned validation notes |
|---|---|---|---|
| `BC-PASS-002` | Acting seat is selecting cards on a pass hand. | select or unselect owned cards until exactly three distinct cards are staged; confirm only when exactly three are staged. | Reject duplicate, unowned, malformed, wrong-seat, stale, and wrong-phase submissions without mutation. |
| `BC-PASS-003` | Fewer than four seats have confirmed. | pending status only for public/other viewers. | Public status may show direction, pending/confirmed seats, and counts; it must not reveal card identities. |
| `BC-PASS-003` | All four seats have confirmed. | atomic exchange, then clear pass selections and provenance from unauthorized projections. | Incoming cards become visible to their new owner after exchange; pass origin remains private. |

## Play Legality

Rust must generate every legal card action and validate every submitted command.
TypeScript must not decide opening lead, follow-suit, first-trick restrictions,
hearts-broken state, trick winner, score, or terminal outcome.

| Rule ID | Situation | Legal card(s) | Rust-owned validation notes |
|---|---|---|---|
| `BC-PLAY-001` | First play of a hand. | Only 2 clubs, by its holder. | Reject any other opening lead. |
| `BC-PLAY-002` | A led suit exists and the actor holds at least one card of that suit. | Exactly the actor's held cards of the led suit. | Reject off-suit submissions without naming hidden alternatives. |
| `BC-PLAY-003` | A led suit exists and the actor is void in that suit. | Any card that survives the first-trick point restriction. | The off-suit play may imply void in public history, but no explicit opponent-void map is projected. |
| `BC-PLAY-004` | The first trick is in progress, the actor is void in clubs, and the actor has at least one non-point card. | Non-heart cards except queen of spades. | Hearts and queen of spades are unavailable and rejected. |
| `BC-PLAY-004` | The first trick is in progress, the actor is void in clubs, and every held card is a point card. | Every held card. | The no-alternative exception prevents an empty legal set. |
| `BC-PLAY-005` | Actor leads a later trick while hearts are unbroken and the actor holds a non-heart. | Non-heart cards only. | Reject a heart lead with a stable hearts-not-broken diagnostic. |
| `BC-PLAY-006` | Actor leads while hearts are unbroken and holds only hearts. | Every held heart. | A legal heart lead under this exception breaks hearts immediately. |
| `BC-PLAY-007` | Any heart is legally played. | not applicable | Hearts become broken from that play forward. |
| `BC-PLAY-007` | Queen of spades is legally played before any heart. | not applicable | Queen of spades alone does not break hearts. |

## Trick Resolution

| Rule ID | Resolution rule | Timing | Notes |
|---|---|---|---|
| `BC-TRICK-001` | The highest-ranked card of the led suit wins the trick; off-suit cards never win. | after the fourth card is played | Ace is high. Suits never break ties. |
| `BC-TRICK-002` | The trick winner captures all four played cards and leads the next trick unless the hand is complete. | trick resolution | Captured cards are the sole source of raw hand points. |

## Scoring and accounting

| Rule ID | Scoring/match rule | Timing | Notes |
|---|---|---|---|
| `BC-SCORE-001` | Each captured heart is 1 raw point; captured queen of spades is 13 raw points; every other card is 0. | hand scoring | Raw point computation is Rust-owned. |
| `BC-SCORE-002` | A complete hand contains 26 raw points total across all seats. | hand scoring | Conservation is a required property. |
| `BC-SCORE-003` | If one seat captures all 26 raw points, that seat adds 0 for the hand and each opponent adds 26. | hand scoring | Briar Circuit does not offer a subtract-26 choice. |
| `BC-MATCH-001` | Hand additions accumulate monotonically by seat. | after each scored hand | Lower cumulative score is better. |
| `BC-MATCH-002` | After a completed hand, if any cumulative score is at least 100, evaluate match end. | hand close | Threshold is never checked mid-hand. |
| `BC-MATCH-003` | If the lowest cumulative score is unique, that seat wins; if the low score is tied, continue complete hands with dealer and pass cycle rotation. | threshold evaluation | Seat order never breaks a low-score tie. |

## Terminal conditions

| Rule ID | Terminal rule | Rust source of truth | Notes |
|---|---|---|---|
| `BC-MATCH-002` | After a completed hand, if any cumulative score is at least 100, evaluate the match. | `score_completed_hand`, `OutcomeBreakdown.threshold_reached` | Threshold is never checked mid-hand. |
| `BC-MATCH-003` | A unique lowest cumulative score produces the terminal winner; a tied low score continues the match. | `OutcomeStatus::Terminal`, `OutcomeStatus::TiedLowContinuation`, `TerminalOutcome::UniqueLowScoreWin` | Seat order never breaks a low-score tie. |
| `BC-OUTCOME-001` | Terminal presentation uses Rust-authored per-seat scoring and outcome breakdowns. | `OutcomeBreakdown`, `SeatOutcomeBreakdown`, WASM `BriarCircuitPublicView` mirror | TypeScript renders projected outcome facts; it must not decide the winner. |

## Visibility And Private Information

Public/browser payloads must not reveal hidden information through public views,
seat views, action trees, previews, diagnostics, effect logs, DOM attributes,
test IDs, logs, local storage, replay exports, bot explanations, candidate
rankings, or dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection |
|---|---|---|---|---|
| `BC-VIS-001` | Unplayed private hand cards. | owning seat only | while unplayed | public view, other seat views, action trees, previews, diagnostics, effects, exports, DOM, logs, storage, bot surfaces |
| `BC-VIS-002` | Pass selection identities, incoming cards before exchange, and pass provenance. | selecting/receiving seat only as authorized; provenance remains private | selection/exchange lifecycle | public status carries direction and completion counts only |
| `BC-VIS-003` | Deck order, seed-reconstructable material, and future deal facts. | no browser-facing viewer | never | viewer-scoped export must not reconstruct hidden cards |
| `BC-VIS-004` | Private action trees, previews, diagnostics, effects, bot candidates, and bot explanation facts. | authorized seat only | when authorized by Rust projection | unauthorized viewers receive no hidden alternatives or private rationale |

## Replay And Randomness

| Rule ID | Replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `BC-REPLAY-001` | Internal replay reproduces state, effect, action-tree, and view hashes from seed, variant, seat order, and command stream. | deterministic | internal for hidden facts | No wall-clock or browser randomness. |
| `BC-REPLAY-002` | Public and seat-private exports reproduce only authorized observation timelines. | deterministic and viewer-scoped | public or owner-scoped | Exports must not include hidden deck material, opponent hands, pass provenance, or unauthorized bot data. |

## Bot Boundaries

Bot notes are not extra rule authority. Bots must choose from Rust-generated
legal actions and submit normal action paths through validation.

| Rule ID | Bot rule | Allowed inputs | Forbidden inputs |
|---|---|---|---|
| `BC-BOT-001` | Level 0 samples uniformly from the Rust legal leaf set using declared bot RNG. | legal action tree and bot RNG stream | direct state mutation, illegal fallback, opponent hands, deck order |
| `BC-BOT-002` | Level 1 uses only own projected hand, public state/history, legal actions, and deterministic tie-breaks. | own hand, public trick/pass/score history, legal actions | hidden opponent cards, pass provenance, deck tail, rollout sampling, MCTS, ISMCTS, Monte Carlo, ML, RL |

## UI And Outcome Presentation

| Rule ID | UI/outcome rule | Requirement |
|---|---|---|
| `BC-UI-001` | Browser controls expose Rust legal actions only and do not derive legality. |
| `BC-OUTCOME-001` | Rust supplies per-seat raw hand points, moon adjustment, hand addition, cumulative before/after, threshold/tie reason, terminal winner, and final standings for the outcome surface. |

## Variant Posture And Out-Of-Scope Rules

| Rule ID | Boundary | Rulepath position | Notes |
|---|---|---|---|
| `BC-VAR-001` | Public variant | `briar_circuit_standard` is the only shipped variant. | Variant data may label the game but cannot define behavior. |
| `BC-VAR-002` | Public naming | Public copy uses **Briar Circuit** as an original neutral Rulepath name. | "Hearts" remains a rules-family/source-note label only. |
| `BC-OOS-001` | Variable seats, partnerships, teams, bidding, trump, bonus variants, moon choice, shoot the sun, hosted multiplayer, and takebacks. | out of scope | Adding any requires a later spec and evidence. |
| `BC-OOS-002` | Generic card, suit, hand, pass, or trick helper. | out of scope for Gate 16 | Gate 16 records second-use pressure and keeps implementation local. |
| `BC-OOS-003` | Static-data rule behavior. | forbidden | No formulas, selectors, triggers, scripts, loops, or tactical policy in data. |
| `BC-OOS-004` | Solver or learning bots. | forbidden for public v1/v2 | No MCTS, ISMCTS, Monte Carlo, ML, RL, or hidden-state sampling. |

## Rule-ID Migration Log

No rule IDs have been migrated.
