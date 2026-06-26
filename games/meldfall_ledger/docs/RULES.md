# Meldfall Ledger Rules

Game ID: `meldfall_ledger`

Public display name: `Meldfall Ledger`

Implemented variant: `classic_500_single_deck_v1`

Rules version: `meldfall-ledger-rules-v1`

Data version: `meldfall-ledger-data-v1`

Prepared by: `Codex`

Created: 2026-06-26

Last updated: 2026-06-26

## Rule Authority

This document is the original Rulepath rules summary for Meldfall Ledger, a
neutral Rulepath implementation in the Five Hundred Rummy / Rummy 500 rules
family. Sources and variant comparisons belong in `SOURCES.md`; this document
states the implementation contract that Rust code, traces, rule coverage, bots,
replay, WASM, and UI must satisfy.

Stable `ML-*` rule IDs are requirements. They must remain stable after
implementation unless intentionally migrated with a migration note and matching
updates in `RULE-COVERAGE.md`, traces, tests, player-facing docs, and any
consumer that names the affected rule.

Rust owns setup, legal actions, validation, hidden hands, draw and discard
rules, meld and lay-off legality, scoring, semantic effects, visibility
projection, replay behavior, terminal detection, and bot decisions. TypeScript
may present only Rust/WASM output.

Game nouns in this document belong to `games/meldfall_ledger` only. They do
not authorize `card`, `deck`, `hand`, `suit`, `rank`, `meld`, `set`, `run`,
`stock`, `discard`, `pile`, `tableau`, `lay-off`, or related vocabulary in
`engine-core`.

## Metadata

| Field | Value |
|---|---|
| game id | `meldfall_ledger` |
| public display name | `Meldfall Ledger` |
| variant | `classic_500_single_deck_v1` |
| rules version | `meldfall-ledger-rules-v1` |
| data version | `meldfall-ledger-data-v1` |
| source note | `games/meldfall_ledger/docs/SOURCES.md` |
| coverage matrix | `games/meldfall_ledger/docs/RULE-COVERAGE.md` |

## Purpose And Scope

Meldfall Ledger proves an official variable-seat hidden-information meld game:
private hands, a hidden stock, a public discard pile, public meld groups, laying
off onto any tabled meld, multi-card discard-pile pickup, round scoring with
hand penalties, cumulative match scoring to 500, viewer-scoped exports, and a
large public table plus private-hand browser surface.

Public presentation uses the original name Meldfall Ledger. "Five Hundred
Rummy", "Rummy 500", "500 Rum", and related names are rules-family/source
labels only, not the public product identity. The game uses original Rulepath
prose and presentation, with no copied rulebook text, card art, app layout, or
trade dress.

The only Gate 19 variant is `classic_500_single_deck_v1`.

## Identity, Seats, And Setup

| Rule ID | Rule | Visibility | Notes |
|---|---|---|---|
| `ML-ID-001` | The game id is `meldfall_ledger`; the variant is `classic_500_single_deck_v1`; the rules/data versions are `meldfall-ledger-rules-v1` and `meldfall-ledger-data-v1`. | public metadata | Manifest, catalog, WASM, tools, traces, and docs must agree. |
| `ML-ID-002` | Public copy uses **Meldfall Ledger** as the game name. Rules-family labels may appear in source notes and maintenance context only. | public copy | This avoids source confusion and copied trade dress. |
| `ML-SETUP-001` | Supported seat counts are exactly 2, 3, 4, 5, and 6. Default setup uses 4 seats. Every other seat count is rejected by Rust with a stable viewer-safe diagnostic. | public diagnostic | There are no teams, partnerships, roles, or elimination. |
| `ML-SETUP-002` | Stable seats serialize in `seat_0` through `seat_5` order, with fallback public labels `Seat 1` through `Seat 6`. | public | Seat keys are stable across views, traces, tools, and WASM. |
| `ML-SETUP-003` | The game uses one standard 52-card deck: four suits, ranks 2 through ace, no jokers, and no duplicate card identities. | internal until projected | Card nouns are game-local only. |
| `ML-SETUP-004` | For 2 seats, Rust deals 13 cards to each seat. For 3 through 6 seats, Rust deals 7 cards to each seat. One card becomes the initial face-up discard; all remaining cards form the hidden stock. | owner-private hands plus public counts | Deal counts are part of the pinned variant. |
| `ML-SETUP-005` | The initial dealer is `seat_0` unless setup explicitly supplies another legal dealer. The first active seat is the seat left of the dealer, and play proceeds clockwise. | public | Dealer rotation between rounds is Rust-owned. |
| `ML-SETUP-006` | Setup and deal ordering are deterministic from match seed, seat count, variant, rules version, and data version. | replay authority | Wall-clock time and browser randomness are not inputs. |

## Visibility And Zones

| Rule ID | Rule | Visible to whom | Notes |
|---|---|---|---|
| `ML-VIS-001` | A public observer sees public meld groups, public discard pile order, stock count, hand counts, scores, active seat, dealer, turn phase, diagnostics, public effects, and terminal standings. | public observer and all seat viewers | Public facts are still projected by Rust. |
| `ML-VIS-002` | A seat viewer sees that seat's own hand while it remains private. | owning seat only | Other seats receive counts only. |
| `ML-VIS-003` | Opponent hands, unseen stock order, next stock card, internal shuffle tail, private action labels, bot private rankings, and hidden diagnostics are never visible to unauthorized viewers. | hidden from public and other seats | This applies to view JSON, action trees, previews, effects, replay exports, DOM, a11y labels, logs, storage, tests, and simulator summaries. |
| `ML-VIS-004` | The discard pile is public and ordered oldest to newest, with the newest/top discard last. | public | Discard identities are not hidden once discarded. |
| `ML-VIS-005` | Melded and laid-off cards are public after they enter the tabled meld tableau. | public | Tabled cards remain public in every viewer projection. |
| `ML-VIS-006` | Public round settlement exposes tabled-card totals, in-hand penalty totals, remaining hand counts, round deltas, cumulative scores, ranks, and winner flags, but not opponents' exact unmelded cards. | public settlement | Seat-private settlement may include the viewer's own remaining cards. |

## Turn Flow

| Rule ID | Rule | Legal action or result | Notes |
|---|---|---|---|
| `ML-TURN-001` | A turn starts with the active seat choosing a draw source. | draw phase | Wrong-seat, stale, terminal, and wrong-phase commands are rejected. |
| `ML-TURN-002` | The active seat may draw one card from the hidden stock when the stock is non-empty. | `draw/stock` | The drawn card becomes visible only to the acting seat. Public effects state only that a stock draw occurred and the stock count changed. |
| `ML-TURN-003` | The active seat may draw from the public discard pile by selecting a visible discard card. Rust gives the seat that card plus every newer card above it. | `draw/discard/<index>` | The selected index is over the public oldest-to-newest discard order. |
| `ML-TURN-004` | The selected discard card from any discard-pile draw, including the top discard, must be used immediately in a new meld or legal lay-off during the same turn. | pending pickup commitment | The seat may not finish the turn or discard the committed card while the commitment remains unsatisfied. |
| `ML-TURN-005` | After drawing, the active seat may create any number of legal new melds and may lay off any number of legal cards onto existing public melds. | table-play phase | Melding and laying off are optional except when needed to satisfy a pickup commitment or empty the hand. |
| `ML-TURN-006` | If the active seat still has cards and no pickup commitment remains, the seat may discard exactly one owned card to end the turn. | `discard/<card>` | The discarded card becomes the newest public discard. |
| `ML-TURN-007` | If the active seat has no cards after legal melds and lay-offs, the round ends immediately; no final discard is required. | go out without discard | No floating rule exists. |
| `ML-TURN-008` | If the active seat discards its final card after legal table plays, the round ends after that discard. | go out by final discard | The final discard remains public. |
| `ML-TURN-009` | If the stock is empty and play cannot continue through a legal or accepted discard draw, Rust ends the round and settles scores. | stock exhaustion | The discard pile is not reshuffled into the stock in this variant. |

## Melds And Lay-Offs

| Rule ID | Rule | Legal action or result | Notes |
|---|---|---|---|
| `ML-MELD-001` | A set is a new meld of 3 or 4 cards with the same rank and distinct suits. | legal meld kind | Single-deck play prevents duplicate suit/rank cards. |
| `ML-MELD-002` | A run is a new meld of 3 or more consecutive cards in the same suit. | legal meld kind | Runs are evaluated by Rust from game-local rank/suit types. |
| `ML-MELD-003` | Aces may be low in `A-2-3` style runs or high in `Q-K-A` style runs, but runs may not wrap around the ace. | run legality | `K-A-2` and `Q-K-A-2` are illegal. |
| `ML-MELD-004` | A new meld may use only cards owned by the active seat and must remove those cards from that seat's private hand atomically when accepted. | ownership check | No card may exist in two zones at once. |
| `ML-MELD-005` | Accepted meld groups receive stable public meld IDs, an origin seat, ordered public cards, and per-card score-credit owner data. | public tableau | The origin seat does not override per-card score credit. |
| `ML-LAYOFF-001` | A seat may lay off an owned card onto any existing public meld, including another seat's meld, when the resulting meld remains legal. | legal lay-off | Rust validates set/run extension. |
| `ML-LAYOFF-002` | A laid-off card scores to the seat that played it, not necessarily to the origin seat of the meld group. | score-credit rule | Each tabled card records its score-credit owner. |
| `ML-LAYOFF-003` | Already tabled meld groups cannot be rearranged, split, merged, or remelded. | explicit exclusion | Gate 19 supports extension only. |

## Scoring and accounting

Card values are constant in every scoring context:

```text
ace = 15
king, queen, jack, ten = 10
2 through 9 = pip value
```

Stable scoring tokens for later coverage, outcome, and trace consumers:

- `score-card-value`
- `score-tabled`
- `score-inhand-penalty`
- `score-round-delta`
- `score-cumulative`
- `score-credit-owner`
- `score-settlement-visibility`

| Rule ID | Rule | Notes |
|---|---|---|
| `ML-SCORE-001` | Aces count as 15, face cards and tens count as 10, and ranks 2 through 9 count as their pip value. | A low ace in a run still scores 15. |
| `ML-SCORE-002` | Each seat receives positive round points equal to the values of public tabled cards whose score-credit owner is that seat. | Melded and laid-off cards use the same credit model. |
| `ML-SCORE-003` | Each seat subtracts the values of cards still in that seat's private hand at round settlement. | Round deltas may be negative. |
| `ML-SCORE-004` | Each seat's round delta is tabled positives minus in-hand penalties. | Rust emits the ordered score components. |
| `ML-SCORE-005` | Each round delta is added to the seat's cumulative match score. | Cumulative scores may be negative. |
| `ML-SCORE-006` | The table-card `played_by` seat is the score-credit owner. The origin seat of a meld group is not a scoring shortcut. | This protects lay-off credit. |
| `ML-SCORE-007` | Public settlement exposes public score totals/counts without exposing unauthorized unmelded card identities. | See `ML-VIS-006`. |

## Terminal conditions

Stable terminal tokens for later coverage, outcome, and trace consumers:

- `match-target-500`
- `match-tie-continue`

| Rule ID | Rule | Notes |
|---|---|---|
| `ML-MATCH-001` | Terminal eligibility is evaluated only after a round has settled. At least one seat must have cumulative score 500 or higher. | No mid-turn terminal shortcut exists. |
| `ML-MATCH-002` | If exactly one seat has the highest score among seats at or above 500, that seat wins the match. | The unique highest eligible seat is the winner. |
| `ML-MATCH-003` | When multiple seats tie for the highest score at or above 500, the match continues with another round. | Dealer and seat order are not tiebreakers. |
| `ML-MATCH-004` | If no seat is at or above 500 after settlement, the match continues with the next round. | Negative and below-target scores remain ordinary scores. |
| `ML-MATCH-005` | Terminal standings are seat-keyed, stable in seat order, and include cumulative score, latest round delta, rank, and winner flag. | TypeScript renders Rust-authored standings only. |
| `ML-MATCH-006` | A non-terminal settled round advances dealer clockwise, clears round-only table state, deals a fresh round, and starts with the seat left of the new dealer. | Transition is deterministic and Rust-owned. |

## Replay, Bots, And UI

| Rule ID | Rule | Notes |
|---|---|
| `ML-REPLAY-001` | The same accepted command stream reproduces state, effects, views, and hashes under fixed seed, seat count, variant, rules version, and data version. | Replay is deterministic and Rust-owned. |
| `ML-REPLAY-002` | Viewer-scoped public and seat-private exports never elevate privilege on import. | Public exports remain public; seat exports remain scoped to the authorized viewer. |
| `ML-REPLAY-003` | Trace Schema v1 records setup, draw source, discard-pile pickup, melds, lay-offs, scoring, terminal state, visibility notes, and migration notes as applicable. | No trace schema migration is authorized by this rules doc. |
| `ML-BOT-001` | L0 random-legal bots select deterministically from Rust legal actions and submit through normal validation. | L0 never constructs legality itself. |
| `ML-BOT-002` | Any L1 rule-informed bot may use only public facts, its own private hand, Rust legal actions, and deterministic authored preferences. | Opponent hands, stock order, hidden-world sampling, MCTS, ISMCTS, Monte Carlo rollouts, ML, and RL are forbidden. |
| `ML-BOT-003` | Bot explanations and candidate rankings must be viewer-safe and must not reveal unauthorized hidden cards or stock order. | Explanations are public/seat-scoped presentation of Rust-owned choices. |
| `ML-UI-001` | Browser controls present Rust legal actions and Rust-safe previews; TypeScript must not validate melds, lay-offs, pickup commitments, scoring, or terminal state. | Legal-only UI is required. |
| `ML-UI-002` | The public UI must support large private hands, public meld groups, public discard-tail choices, score ledger, replay/import/export controls, and a no-drag-required action path. | Keyboard and single-pointer alternatives are required. |
| `ML-UI-003` | DOM text, accessibility names, `data-testid` values, storage, console logs, and animation/effect surfaces must not contain unauthorized hidden card identities. | Browser no-leak proof is required before public exposure. |

## Diagnostics

The implementation must expose stable, viewer-safe diagnostic codes for at
least these cases: wrong rules version, stale command, invalid seat count,
unknown seat, wrong active seat, terminal state, wrong phase, malformed action,
empty stock, invalid discard index, unsatisfied pickup commitment, unknown card,
card not owned, invalid set, invalid run, invalid ace wrap, invalid lay-off,
duplicate zone ownership, illegal final discard, and unsupported variant.

Diagnostics may name public phase, public discard cards, public meld groups,
stock count, hand counts, and active seat where relevant. They must not list
unauthorized hidden hands, stock order, future randomness, or private bot
features.

## Known Ambiguities And Chosen Resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `ML-SETUP-004` | Some larger tables use more than one deck. | One 52-card deck for every supported seat count. | Gate 19 variant pinning. | setup 2/4/6 traces and conservation tests | Deliberate deviation. |
| `ML-TURN-004` | Some tables let the top discard be taken without immediate use. | Every discard-pile pickup, including top discard, creates an immediate-use commitment. | Gate 19 strict variant. | top-discard commitment trace | Prevents a special-case UI rule. |
| `ML-MELD-003` | Ace run handling varies. | Ace may be low or high, never around-the-corner. | Gate 19 variant pinning. | ace-low/high/no-wrap trace | Low ace still scores 15. |
| `ML-TURN-007` | Some tables require a final discard or allow floating. | A seat may go out by tabling every card; final discard is not required. | Gate 19 scope. | go-out without discard trace | Floating is excluded. |
| `ML-MATCH-003` | At/above-500 ties may use house tiebreakers. | Equal highest eligible scores continue to another round. | Rulepath avoids arbitrary tiebreaks. | tie-continuation trace | Dealer/seat order not used. |
| `ML-VIS-006` | Some physical tables reveal remaining cards at settlement. | Public settlement shows totals/counts only; exact unmelded card identities stay scoped. | Rulepath no-leak posture. | public/private settlement no-leak tests | Future changes require accepted source note and matrix update. |

## Explicit Out-Of-Scope Variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `ML-ID-001` | Jokers, wild cards, two-deck shoes, opening minimums, calling "Rummy", frozen piles, floating, discard-required going out, around-the-corner runs, partnerships, teams, and tabled-meld rearrangement. | Gate 19 proves one pinned classic single-deck variant. | Later accepted spec only. |
| `ML-BOT-002` | L2 authored policy, hidden-world search, MCTS, ISMCTS, Monte Carlo rollouts, ML, RL, runtime LLMs, or stock/opponent-hand sampling. | Public v1/v2 bot law forbids these or requires separate evidence/ADR. | Future strategy evidence pack or ADR as applicable. |
| `ML-UI-001` | Browser-side legality shortcuts for melds, lay-offs, pickup commitments, scoring, or terminal state. | TypeScript is presentation-only. | Not admissible without foundation change. |

## Rule Coverage Link

The implementation and evidence mapping will live in `RULE-COVERAGE.md`.
Every `ML-*` rule in this document must appear there. Silent gaps are not
allowed.

## Rule-ID Migration Notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| _None_ | _Not applicable_ | Initial Gate 19 rule set. | not applicable | 2026-06-26 |
