# Vow Tide Rules

Game ID: `vow_tide`

Public display name: `Vow Tide`

Implemented variant: `vow_tide_standard`

Rules version: `vow-tide-rules-v1`

Prepared by: Codex

Created: 2026-06-21

Last updated: 2026-06-21

## Rule authority

This is the original Rulepath rules summary for `vow_tide_standard`. It states
the implementation contract that later Rust, trace, coverage, WASM, bot, and UI
work must satisfy. Sources and variant comparisons live in [SOURCES.md](SOURCES.md);
this document does not copy external rules prose, score sheets, examples, art,
or presentation.

Stable `VT-*` rule IDs are requirements. After traces exist, renaming or
splitting an ID is a migration that must update coverage, traces, tests, and
docs together.

## Metadata

| Field | Value |
|---|---|
| game id | `vow_tide` |
| public display name | `Vow Tide` |
| variant | `vow_tide_standard` |
| rules version | `vow-tide-rules-v1` |
| data version | `vow-tide-data-v1` |
| source note | [SOURCES.md](SOURCES.md) |
| coverage matrix | `RULE-COVERAGE.md` |
| mechanic inventory | `MECHANICS.md` |
| implementation admission | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) |

## Purpose and scope

| Rule ID | Rule statement | Notes |
|---|---|---|
| `VT-IDENTITY-001` | Vow Tide implements one official variant, `vow_tide_standard`, under rules version `vow-tide-rules-v1` and data version `vow-tide-data-v1`. | The public name is Vow Tide; "Oh Hell" is a rules-family source label only. |
| `VT-BOUNDARY-001` | All setup, legal actions, validation, deal, bidding, trick resolution, scoring, terminal detection, visibility, effects, replay, and bot decisions are Rust-owned. Static data is metadata/content only, and TypeScript presents Rust/WASM output without deciding legality. | No `engine-core` card, suit, rank, hand, trick, trump, bid, dealer, schedule, or score noun is admitted. |

## Seat model

| Rule ID | Seat-model field | Rule statement | Notes |
|---|---|---|---|
| `VT-SEATS-001` | supported seats | Supported seat counts are exactly 3, 4, 5, 6, and 7. Default public setup is 4 seats. Supplied seat order is the clockwise order. Stable trace IDs are `seat_0` through `seat_6`; fallback public labels are `Tide 1` through `Tide 7`. Every other count is rejected with a stable viewer-safe diagnostic. | No partnerships, teams, elimination, or roles exist in this variant. |
| `VT-DEALER-001` | dealer rotation | `seat_0` is the initial dealer. After each resolved hand, the dealer advances one seat clockwise. Dealer never changes during a hand. | Bid order and first leader derive from dealer in Rust. |

## Components and vocabulary

| Rule ID | Term/component | Original Rulepath definition | Visibility |
|---|---|---|---|
| `VT-CARDS-001` | deck and cards | The game uses one 52-card deck: four suits, ranks 2 through ace, ace high, no jokers, no duplicate identities. | Card identities are public only when in the trump indicator or a played trick; unplayed hands and stock remain private. |
| `VT-TRUMP-001` | trump indicator | After each deal, the next undealt card is turned face up. Its suit is trump for that hand. The indicator is public and cannot be played. The remaining undealt stock stays hidden. | The stock identity/order is never browser-facing. |

## Setup, schedule, and deal

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `VT-SCHEDULE-001` | For seat count `N`, maximum hand size is `K = min(10, floor(51/N))`. The hand schedule descends from `K` to `1`, then ascends from `2` to `K`; the one-card hand occurs once. Total hands are `2K-1`. | deterministic | public | `K=10` for 3-5 seats, `K=8` for 6, and `K=7` for 7. |
| `VT-DEAL-001` | Each hand uses deterministic Rust RNG derived from the match seed and hand index. Cards are shuffled and dealt one at a time clockwise, beginning left of the dealer, until every seat has the scheduled hand size. | random but replay-deterministic | mixed | Dealt cards are owner-private until played; deal conservation is mandatory. |
| `VT-TRUMP-001` | The first undealt card after the deal becomes the public trump indicator; every later undealt card remains hidden stock. | random but replay-deterministic | mixed | The hand-size formula guarantees an indicator exists for every supported count. |

## Bidding

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `VT-BID-ORDER-001` | Bidding phase | Bidding starts with the seat left of dealer, proceeds clockwise, and ends with the dealer. Each seat submits exactly one bid. Accepted bids are public immediately. | flat `bid/<n>` leaves | Wrong-seat, duplicate, wrong-phase, and stale commands are rejected. |
| `VT-BID-RANGE-001` | Any non-dealer bidder, and dealer before hook filtering | A normal bid is any integer from `0` through the current hand size `H`. | flat leaves in ascending numeric order | Out-of-range or malformed paths receive stable diagnostics. |
| `VT-HOOK-001` | Dealer's bid | Let `S` be the sum of earlier bids. If `H-S` is in `0..=H`, that one value is illegal for the dealer. If `H-S` is outside the range, no otherwise-valid dealer bid is removed. | flat leaves with one omitted value when applicable | Legal tree and validator must agree; at least one dealer bid remains legal. |
| `VT-BID-PUBLIC-001` | After bid acceptance | A submitted bid is immutable and public. An unsubmitted focused bid is not authoritative game state and cannot leak through previews, storage, logs, or replay. | not applicable after acceptance | The variant has no edit-bid action. |

## Trick play

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `VT-FIRST-LEAD-001` | First trick of a hand | The seat left of dealer leads and may play any held card, including trump. | flat `play/<card_id>` leaves | There is no trump-broken or first-trick restriction. |
| `VT-FOLLOW-001` | Following a led suit | A follower holding at least one card of the led suit must play that suit. A void follower may play any held card, including trump. | flat card leaves in canonical hand order | The promoted `game-stdlib::trick_taking` helper supplies the pure led-suit subset; ownership, phase, diagnostics, and effects remain game-local. |
| `VT-TRICK-WIN-001` | Resolving a complete trick | Highest trump wins if any trump was played. Otherwise, highest rank in led suit wins. Off-suit non-trumps cannot win. Stable first occurrence wins equal projected values. | Rust transition | The promoted helper supplies the pure comparator; local Rust maps the winning index to a seat and state transition. |
| `VT-NEXT-LEAD-001` | After a non-final trick | The trick winner leads the next trick. | Rust transition | Winner-leads sequencing remains game-local, not a stdlib policy. |
| `VT-HAND-END-001` | After `H` tricks in a hand | The hand ends after exactly `H` tricks. Every seat has zero cards, and all dealt cards are in completed tricks. | Rust transition | Scoring and next-hand setup follow atomically. |

## Scoring and accounting

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `VT-SCORE-001` | A seat whose tricks taken exactly equal its bid scores `10 + bid`; every miss, under or over, scores zero. Cumulative scores never decrease. | End of each hand | A successful zero bid scores 10. | No consolation trick points and no negative penalties. |
| `VT-HAND-ADVANCE-001` | Hand result/history is recorded atomically before dealer and schedule advance. If more hands remain, Rust rotates dealer, deals the next scheduled hand, reveals trump, clears bids/tricks, and enters bidding. | End of each hand | The one-card hand occurs once before the schedule ascends. | Effects must preserve deterministic order. |

## Terminal conditions

| Rule ID | Terminal rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `VT-TERMINAL-001` | The match ends only after the final scheduled hand. | After final hand scoring | No point target and no extra tie-break hand. | Simulation action caps are test guards, not game rules. |
| `VT-STANDINGS-001` | Highest cumulative score wins. Equal top scores are co-winners. Standings use competition ranking with stable seat order only for serialization/display. | Terminal | Tied leaders share rank 1. | Rust supplies the ranked outcome; TypeScript must not re-rank. |
| `VT-OUTCOME-001` | Rust supplies seat-keyed hand history, exact/miss totals, successful zero count, cumulative score, rank, co-winner flag, and concise public decisive facts for the shared outcome surface. | Terminal and current standings | Hidden hand/stock facts are never part of public rationale. | UI renders the Rust-authored fields. |

## Visibility and private information

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `VT-VIEW-001` | Public table facts: seats, labels, dealer, schedule, hand size, trump indicator, submitted bids, current/completed trick plays, trick counts, scores, hand results, terminal standings. | Observer and every seat viewer | As Rust makes the fact public | view, action tree, preview, diagnostics, effects, replay export, WASM, DOM, storage, logs, bot explanation | Public facts are still projected by Rust. |
| `VT-VIEW-001` | Own unplayed hand | Owning seat only | While cards remain in that seat's hand | view, action tree, preview, effects, replay export, DOM, storage, logs, bot input | Hotseat handoff must remove the prior private subtree before showing another hand. |
| `VT-VIEW-001` | Other seats' unplayed hands and hidden stock identity/order | No browser viewer | Never in normal browser/export surfaces | all browser, WASM, replay-export, bot, dev-panel, test-ID, a11y, and log surfaces | Internal native traces may be omniscient test artifacts only. |
| `VT-EFFECT-001` | Semantic effects | Authorized viewer only after filtering | On transitions | effect log, animation scheduler, replay UI, bot explanations, diagnostics | Effects are cause facts, not animation instructions, and must not leak hidden identities. |
| `VT-BOT-001` | Bot inputs and explanations | Acting bot and viewer-safe surfaces | During bot decision/explanation | bot view, candidates, rankings, logs, explanation text | Bots may use own hand plus public facts and legal leaves only. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `VT-REPLAY-001` | Internal replay reproduces setup, deals, commands, effects, action trees, views, and hashes from seed, versions, seats, variant, and command stream. | Trace Schema v1; no schema migration authorized. | Internal/test full traces may include authority facts; browser exports are viewer-scoped observation histories. | Public and seat-private exports never retroactively reveal hidden stock or other hands. |
| `VT-DEAL-001` | Per-hand randomness is partitioned by match seed and hand index under documented versioned Rust rules. | Same inputs reproduce same hand; different seeds may produce different hands. | Stock and unplayed hands are redacted from unauthorized viewers. | Wall-clock time and browser randomness are not inputs. |

## Diagnostics

The implementation must expose stable, viewer-safe diagnostic codes for at least
these cases: wrong rules version, stale command, invalid seat count, unknown
seat, wrong active seat, terminal state, wrong phase, malformed action,
out-of-range bid, dealer hook bid, duplicate bid, unknown card, card not owned,
must-follow-suit, and action unavailable. Diagnostics may name public phase,
seat labels, hand size, public bids, and public led suit where relevant; they
must not list hidden hands, stock contents, future randomness, or private bot
features.

## Bot-relevant non-authoritative strategy notes

| Situation | Practical note | Rule IDs checked | Hidden-info limit |
|---|---|---|---|
| Bidding | A bounded L1 bot may estimate a contract from its own high cards, trump controls, hand size, and public bids, then choose the nearest legal value. | `VT-BID-RANGE-001`, `VT-HOOK-001`, `VT-BOT-001` | No hidden-world sampling, actual stock peeking, or opponent-hand inference from unauthorized data. |
| Playing while needing tricks | A bounded L1 bot may prefer a cheap current-winning legal card, using public trick facts and its own hand. | `VT-FOLLOW-001`, `VT-TRICK-WIN-001`, `VT-BOT-001` | "Currently winning" is public-trick comparison, not prediction from hidden cards. |
| Playing after contract is met | A bounded L1 bot may prefer losing or low-risk legal cards to avoid extra tricks. | `VT-SCORE-001`, `VT-BOT-001` | No kingmaking or hidden score inference is admitted for L1. |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `VT-BID-PUBLIC-001` | Some physical tables allow a bid to change before the next bidder. | Accepted bids are immutable commands. | Replay/determinism clarity. | bid immutability and duplicate diagnostics. | No edit-bid action exists. |
| `VT-SCORE-001` | Exact-bid games commonly vary between exact-only, plus-trick, and penalty scoring. | Vow Tide uses exact `10+bid`, miss zero. | Product focus on exact vows and simpler explanation. | exact zero, exact positive, under, over traces. | No negative scores. |
| `VT-STANDINGS-001` | Some variants add tie-break hands. | Fixed schedule ends the match and allows co-winners. | Deterministic fixed-length replay and co-winner-safe outcomes. | terminal unique and co-winner traces. | No sudden-death. |
| `VT-FIRST-LEAD-001` | Some trick-taking games restrict early trump leads. | Any held card may be led, including trump. | Locked variant simplification and rules-family baseline. | trump-may-lead trace. | No trump-broken state. |

## Rulepath deviations from common variants

| Rule ID | Common variant behavior | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `VT-BID-PUBLIC-001` | Informal bid changes may be allowed before the next bid in some tables. | Bids are immutable once accepted. | Stable command log, replay, UI, and diagnostics. | yes |
| `VT-SCORE-001` | Some tables award trick points even after missed contracts or use penalties. | Exact scores `10+bid`; misses score zero. | Keeps scoring legible and exact-contract centered. | yes |
| `VT-STANDINGS-001` | Some tables play extra tie-break hands. | Equal high scores are co-winners. | Fixed schedule and deterministic closeout. | yes |
| `VT-SCHEDULE-001` | Some variants use alternate hand-size sequences. | Vow Tide descends from `K` to 1, then ascends to `K`, with the one-card hand once. | Locked official schedule for all supported seat counts. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `VT-IDENTITY-001` | Alternate schedules, no-trump hands, fixed trump, rotating suits, jokers, duplicate decks, secret/simultaneous bids, bid changes, one-card forehead rule, teams, partnerships, target score, negative scoring, or tie-break hands. | Gate 17 proves one official variable-N exact-bid variant. | Later accepted spec only. |
| `VT-BOT-001` | L2 authored policy, search, MCTS, ISMCTS, Monte Carlo/rollout, ML, RL, runtime LLM, or hidden-world sampling. | Public v1/v2 bot law forbids these or requires separate evidence/ADR. | Future L2 evidence pack or ADR as applicable. |

## Rule coverage link

The implementation and evidence mapping will live in `RULE-COVERAGE.md`.
Every `VT-*` rule in this document must appear there. Silent gaps are not
allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| _None_ | _Not applicable_ | Initial Gate 17 rule set. | not applicable | 2026-06-21 |
