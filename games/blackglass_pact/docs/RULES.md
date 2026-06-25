# Blackglass Pact Rules

Game ID: `blackglass_pact`

Public display name: `Blackglass Pact`

Implemented variant: `blackglass_pact_standard`

Rules version: `blackglass-pact-rules-v1`

Data version: `blackglass-pact-data-v1`

Prepared by: `Codex`

Created: 2026-06-25

Last updated: 2026-06-25

## Rule Authority

This document is the original Rulepath rules summary for Blackglass Pact, a
fixed-four partnership trick-taking game in the classic Spades rules family.
Sources and variant comparisons belong in `SOURCES.md`; this document states
the implementation contract that code, traces, coverage, bots, replay, WASM,
and UI must satisfy.

Stable `BP-*` rule IDs are requirements. They must remain stable after
implementation unless intentionally migrated with a migration note and matching
updates in `RULE-COVERAGE.md`, traces, tests, player-facing docs, and any
consumer that names the affected rule.

Rust owns setup, legal actions, validation, hidden hands, blind-nil
commitments, bidding, trick resolution, scoring, semantic effects, visibility
projection, replay behavior, and bot decisions. TypeScript may present only
Rust/WASM output.

Game nouns in this document belong to `games/blackglass_pact` only. They do
not authorize `card`, `deck`, `hand`, `suit`, `rank`, `trick`, `trump`, `bid`,
`contract`, `nil`, `blind nil`, `bag`, `team`, `partnership`, or related
vocabulary in `engine-core`.

## Metadata

| Field | Value |
|---|---|
| game id | `blackglass_pact` |
| public display name | `Blackglass Pact` |
| variant | `blackglass_pact_standard` |
| rules version | `blackglass-pact-rules-v1` |
| data version | `blackglass-pact-data-v1` |
| source note | `games/blackglass_pact/docs/SOURCES.md` |
| coverage matrix | `games/blackglass_pact/docs/RULE-COVERAGE.md` |

## Purpose And Scope

Blackglass Pact proves an official fixed-four partnership trick-taking game:
public fixed teams, pre-deal blind-nil commitments, sequential public bidding,
individual nil contracts, team ordinary contracts, spades-trump trick play,
cumulative bags, team outcomes, public observer projection, four seat-private
views, viewer-scoped replay exports, and Rust-authored grouped UI data.

Public presentation uses the original name Blackglass Pact. "Spades" is a
rules-family/source label only, not the public product identity. The game uses
original Rulepath prose and presentation, with no copied rulebook text, card
art, casino framing, hosted multiplayer, wagering, or generic team/card engine.

The only Gate 18 variant is `blackglass_pact_standard`.

## Identity, Seats, Teams, And Setup

| Rule ID | Rule | Visibility | Notes |
|---|---|---|---|
| `BP-ID-001` | The game id is `blackglass_pact`; the standard variant is `blackglass_pact_standard`; the rules/data versions are `blackglass-pact-rules-v1` and `blackglass-pact-data-v1`. | public metadata | Manifest, catalog, WASM, tools, traces, and docs must agree. |
| `BP-ID-002` | Public copy uses **Blackglass Pact** as the game name. "Spades" may appear only as a rules-family/source label. | public copy | This avoids source confusion and copied trade dress. |
| `BP-SETUP-001` | Exactly four seats are supported. Every other seat count is rejected by Rust with a stable diagnostic. | public diagnostic | No 2-, 3-, 5-, 6-, 7-, or 8-seat mode exists in this variant. |
| `BP-SETUP-002` | Stable seats serialize in `seat_0`, `seat_1`, `seat_2`, `seat_3` order, with fallback public labels North, East, South, West. | public | Clockwise order is North, East, South, West. |
| `BP-SETUP-003` | `team_0` is North-South (`seat_0` and `seat_2`); `team_1` is East-West (`seat_1` and `seat_3`). | public | Partnerships are fixed, opposite, and known from setup. |
| `BP-SETUP-004` | Team IDs are stable and public, but they do not replace seat IDs and do not authorize partner-hand visibility. | public/team grouping | Every action, bid, trick, and private hand remains seat-keyed. |
| `BP-SETUP-005` | The initial dealer is `seat_0`. After each completed non-terminal hand, the dealer rotates one seat clockwise. | public | A terminal hand retains its completed-hand dealer in the terminal record. |
| `BP-SETUP-006` | Match seed and rules/data version inputs are recorded under existing replay law. | internal/public as authorized | No browser-facing export may reconstruct unauthorized hidden deal material. |

## Blind-Nil Commitment

Blind nil is resolved before any card identity exists in any hand, viewer,
action tree, preview, bot input, effect, browser state, or replay export.

| Rule ID | Rule | Legal action or result | Notes |
|---|---|---|---|
| `BP-BLIND-001` | At hand start, a seat is blind-nil eligible only when that seat's team trails the opposing team by at least 100 points. | eligibility gate | Tied scores and deficits below 100 are ineligible. Negative scores use the same point-difference rule. |
| `BP-BLIND-002` | Eligible decisions run left of dealer clockwise before shuffle and deal. | phase order | Ineligible seats are skipped without receiving controls. |
| `BP-BLIND-003` | The active eligible seat receives exactly `blind_nil/declare` and `blind_nil/decline`. | Rust legal leaves | There is no blind numeric bid and no browser-only alias. |
| `BP-BLIND-004` | Ineligible seats are deterministically skipped and receive no blind-nil control. | no action | Public progress may show who is pending or skipped when that is a public fact. |
| `BP-BLIND-005` | A declaration or decline is accepted once, becomes public immediately, and is immutable. | public commitment | A later command cannot edit or erase the decision. |
| `BP-BLIND-006` | No hand, deck order, future card, card-derived preview, or card-derived bot input exists before the blind decision. | no-leak rule | Blind policy may use only public score/order context. |
| `BP-BLIND-007` | Blind decisions do not change shuffle or deal bytes. | deterministic replay | For a fixed seed, hand index, and version set, declare and decline branches use the same deal order. |
| `BP-BLIND-008` | Both partners may independently declare blind nil when eligible; no special combined bonus, automatic win, or double-nil rule exists. | independent contracts | Each blind nil is scored for its own seat. |
| `BP-BLIND-009` | No card pass, exchange, partner consultation, or shared private surface follows a blind-nil declaration. | out of scope | The commitment does not create a second private protocol. |
| `BP-BLIND-010` | A declaring seat is skipped during ordinary bidding and has a fixed zero blind-nil contract. | bidding transition | The bid is `BlindNil`, not an ordinary numeric zero. |

## Deal And Hidden Hands

| Rule ID | Rule | Visibility | Notes |
|---|---|---|---|
| `BP-DEAL-001` | The deck contains exactly one of each standard 52-card rank/suit combination, with ranks 2 through ace and ace high. | internal until projected | Suits and ranks are game-local types. |
| `BP-DEAL-002` | After blind decisions complete, the deal begins left of dealer and proceeds one card at a time clockwise. | deterministic | Deal order is derived from match seed, hand index, rules version, and data version only. |
| `BP-DEAL-003` | Each seat receives 13 unique cards and no undealt tail remains. | owner-private hands plus public counts | Card conservation is required. |
| `BP-DEAL-004` | An unplayed card is visible only to its owning seat. | owner-private | Partner relationship grants no extra card access. |
| `BP-DEAL-005` | Public deal evidence exposes hand counts, dealer, hand index, phase, and next actor, but not card identities. | public observer/effects | Public deal effects must not carry hidden cards. |
| `BP-DEAL-006` | Deal and redeal ordering is stable across replay and serialization. | replay authority | No wall-clock, browser, or bot randomness affects deal order. |

## Bidding And Contracts

| Rule ID | Rule | Legal action or result | Notes |
|---|---|---|---|
| `BP-BID-001` | Ordinary bidding starts left of dealer and proceeds clockwise through dealer. | phase order | Blind-nil declarers are skipped. |
| `BP-BID-002` | Each non-blind seat bids exactly once. | immutable bid | A seat cannot pass, rebid, or edit after acceptance. |
| `BP-BID-003` | Legal bid leaves are `bid/nil` and `bid/1` through `bid/13`. | Rust legal leaves | Nil is the only zero-trick bid vocabulary. |
| `BP-BID-004` | Numeric zero, pass, rebid, negative bids, bids above 13, secret bids, simultaneous bids, and out-of-phase bids are illegal. | stable diagnostics | Vow Tide's total-bid hook is not imported. |
| `BP-BID-005` | Accepted bids become public immediately and are immutable. | public bids | Public bid history is replayed deterministically. |
| `BP-BID-006` | No total-13 or dealer last-bidder hook is applied. | explicit exclusion | The dealer may bid any legal nil or 1-13 value. |
| `BP-BID-007` | A team's ordinary contract is the sum of positive numeric bids made by that team's seats. | Rust-derived team field | The browser does not sum bids. |
| `BP-BID-008` | Nil and blind-nil bids contribute zero to the ordinary team contract. | contract calculation | They are evaluated separately for their bidding seat. |
| `BP-BID-009` | Each nil or blind-nil contract remains attached to its bidding seat. | seat-keyed contract | Team grouping does not merge individual nil obligations. |
| `BP-BID-010` | Team and seat bid projections use stable IDs and stable ordering. | public/seat views | Outcome, WASM, and UI rows must preserve canonical order. |

## Trick Play

Spades are always trump. Before spades are broken, a leader holding a non-spade
cannot lead a spade. A legal off-suit spade or only-spades lead breaks spades.

| Rule ID | Rule | Legal action or result | Notes |
|---|---|---|---|
| `BP-PLAY-001` | The seat left of dealer leads the first trick of every hand. | first leader | Later tricks are led by the prior trick winner. |
| `BP-PLAY-002` | Before spades are broken, a leader holding at least one non-spade cannot lead a spade. | lead restriction | A safe diagnostic must not enumerate hidden alternatives to unauthorized viewers. |
| `BP-PLAY-003` | A leader holding only spades may lead one, and that legal lead breaks spades. | no-empty-legal-set exception | The play is public. |
| `BP-PLAY-004` | A void follower may legally play a spade off suit; the first legal off-suit spade breaks spades. | public play | Public play may imply void in the led suit. |
| `BP-PLAY-005` | A follower holding at least one card of the led suit must play a card of that suit. | follow-suit obligation | The legal leaf set is Rust-authored. |
| `BP-PLAY-006` | A void follower may play any owned card. | void exception | Ownership is still required. |
| `BP-PLAY-007` | If any spade is played to a trick, the highest spade wins. | trick comparison | This uses the promoted comparator with caller-projected spades trump. |
| `BP-PLAY-008` | If no spade is played, the highest card of the led suit wins; off-suit non-spades cannot win. | trick comparison | Ace is high. |
| `BP-PLAY-009` | The trick winner leads the next trick. | transition | Winner and next leader are public. |
| `BP-PLAY-010` | Exactly four cards complete a trick, and exactly 13 completed tricks complete a hand. | hand boundary | Hand scoring occurs only after the thirteenth trick. |
| `BP-PLAY-011` | `game-stdlib::trick_taking::follow_suit_indices` is reused unchanged after game-local phase, actor, ownership, and lead checks. | helper conformance | No partnership or scoring behavior is added to the helper. |
| `BP-PLAY-012` | `game-stdlib::trick_taking::winning_play_index` is reused unchanged with `Some(Spades)` after four game-local plays exist. | helper conformance | Broken-spades, bidding, scoring, and visibility stay game-local. |

## Scoring and accounting

For each team in a completed hand:

```text
C = sum(positive numeric bids by the team's ordinary bidders)
O = sum(tricks won by those ordinary bidders)
ordinary_made = O >= C
ordinary_base = if ordinary_made { 10 * C } else { -10 * C }
ordinary_overtricks = if ordinary_made { O - C } else { 0 }
failed_nil_bags = tricks won by failed nil and failed blind-nil bidders
new_bags = ordinary_overtricks + failed_nil_bags
raw_bags = prior_bags + new_bags
bag_penalty_count = raw_bags / 10
next_bags = raw_bags % 10
hand_delta = ordinary_base + new_bags + nil_delta - 100 * bag_penalty_count
next_score = prior_score + hand_delta
```

| Rule ID | Rule | Notes |
|---|---|---|
| `BP-SCORE-001` | `C` sums positive numeric bids; `O` sums tricks won only by those ordinary bidders. | Nil and blind-nil trick wins are excluded from `O`. |
| `BP-SCORE-002` | The ordinary contract is made iff `O >= C`. | A contract with `C = 0` is made with zero ordinary base and no synthetic bags. |
| `BP-SCORE-003` | A made ordinary contract scores `+10 x C` base points. | Overtricks are separate. |
| `BP-SCORE-004` | A set ordinary contract scores `-10 x C` base points. | A set contract earns no ordinary overtrick points or ordinary bags. |
| `BP-SCORE-005` | Made ordinary overtricks add +1 point and one bag each. | Overtricks are `O - C` only when ordinary made. |
| `BP-SCORE-006` | A set ordinary contract produces no ordinary overtrick points or ordinary bags. | Failed nil bags may still apply separately. |
| `BP-SCORE-007` | A made ordinary nil scores +100; a failed ordinary nil scores -100. | Each nil is seat-keyed. |
| `BP-SCORE-008` | A made blind nil scores +200; a failed blind nil scores -200. | Blind nil is independent of the partner's result. |
| `BP-SCORE-009` | Failed nil and failed blind-nil tricks never help the ordinary contract. | They remain outside `O`. |
| `BP-SCORE-010` | Every failed nil or failed blind-nil trick adds +1 point and one bag, even when the ordinary contract is set. | This is the selected failed-nil attribution rule. |
| `BP-SCORE-011` | Bags persist across hands as a separate integer field. | Score digits are not authoritative bag storage. |
| `BP-SCORE-012` | Every 10 raw bags subtracts 100 points and removes 10 bags; multiple thresholds apply repeatedly. | Remainder bags carry forward. |
| `BP-SCORE-013` | Bag remainder survives sets, nil outcomes, and target crossing. | Bags do not reset because a team reaches the target. |
| `BP-SCORE-014` | The hand delta applies the component order in this section. | Rust emits the ordered breakdown. |
| `BP-SCORE-015` | Every completed hand exposes Rust-authored per-seat and per-team score components. | TypeScript renders supplied facts only. |
| `BP-SCORE-016` | Integer arithmetic must remain within supported evidence budgets. | Boundary tests protect supported simulations and fixtures. |

## Terminal conditions

| Rule ID | Rule | Notes |
|---|---|---|
| `BP-END-001` | Terminal evaluation occurs only after all 13 tricks and hand scoring. | No mid-hand terminal shortcut exists. |
| `BP-END-002` | At least one team must be at 500 or higher and team scores must differ. | Exact ties continue. |
| `BP-END-003` | When the terminal predicate is met, the unique higher-scoring team wins. | Both teams may be above 500. |
| `BP-END-004` | An exact tie at or above 500 continues to another complete hand. | No seat-order or bag tiebreaker. |
| `BP-END-005` | After tie continuation, falling below 500 does not create a terminal state. | The predicate is checked from the current completed hand. |
| `BP-END-006` | Bags, seat order, dealer, and team ID are not tiebreakers. | Only the terminal predicate decides. |
| `BP-END-007` | A non-terminal hand advances the dealer and starts a fresh blind-eligibility phase. | Eligibility is recomputed from current public scores. |
| `BP-END-008` | Terminal state retains the completed-hand dealer/context and does not start a phantom hand. | Replay and outcome keep the final hand context. |
| `BP-END-009` | `standings_by_team` is stable team-ID order with scores, bags, ranks, and winner flags. | Rust supplies competition ranks. |
| `BP-END-010` | `standings_by_seat` is stable seat-ID order with team, bid, tricks, nil result, and rank linkage. | TypeScript does not infer ranks. |

## Visibility, Replay, Bots, And UI

Public/browser payloads must not reveal hidden information through public views,
seat views, action trees, previews, diagnostics, effect logs, DOM attributes,
test IDs, logs, local storage, replay exports, bot explanations, candidate
rankings, or dev inspectors.

| Rule ID | Rule | Notes |
|---|---|---|
| `BP-VIS-001` | Public observer receives no unplayed card and no private control or candidate data. | Public facts include teams, bids, played cards, trick winners, scores, bags, and outcomes. |
| `BP-VIS-002` | A seat viewer receives that seat's own hand only. | Other hands appear only as counts or backs. |
| `BP-VIS-003` | Partner relationship grants no private visibility. | North never sees South's unplayed cards merely because they are partners. |
| `BP-VIS-004` | Blind phase exposes no future card-derived datum. | No card identity exists yet on browser-facing surfaces. |
| `BP-VIS-005` | Action trees and previews are actor- and viewer-scoped. | Unauthorized viewers receive no private alternatives. |
| `BP-VIS-006` | Diagnostics and effects reveal no unauthorized hand fact. | Safe diagnostics may state public phase/rule context only. |
| `BP-VIS-007` | Public export and all four seat exports round-trip without privilege elevation. | Viewer-scoped exports remain viewer-scoped on import. |
| `BP-VIS-008` | DOM, storage, logs, test IDs, accessibility tree, and animations contain no unauthorized datum. | Browser smoke must scan these surfaces. |
| `BP-REPLAY-001` | The same accepted command stream reproduces state, effects, and hashes under fixed versions. | Replay is deterministic and Rust-owned. |
| `BP-REPLAY-002` | Trace Schema v1 fields include phase, actor, team context, bids, score components, and migration notes as applicable. | No schema migration is authorized. |
| `BP-REPLAY-003` | No unrelated golden trace is regenerated for Gate 18. | Golden changes must be scoped and justified. |
| `BP-BOT-001` | L0 selects uniformly and deterministically from Rust legal leaves using isolated bot RNG. | L0 never constructs legality itself. |
| `BP-BOT-002` | L1 uses public facts, own hand, and lawful public-play deductions only. | No partner/opponent hand, future deck, sampled world, or hidden-state shortcut. |
| `BP-BOT-003` | L1 explanations and candidates are viewer-safe and deterministic. | Explanations cannot leak hidden cards or candidate rankings to unauthorized viewers. |
| `BP-BOT-004` | L2 is unadmitted; L3 and prohibited algorithms are absent. | No MCTS, ISMCTS, Monte Carlo, determinization, ML, RL, runtime LLM, or hidden-state sampling. |
| `BP-UI-001` | The grouped table renders fixed partners and stable team IDs without color-only meaning. | Structure/text/pattern must carry partnership identity. |
| `BP-UI-002` | Blind, bid, and card controls come only from Rust legal leaves. | TypeScript does not decide legality. |
| `BP-UI-003` | Team scores, bags, contracts, nil state, ranks, and explanations come from Rust. | Client arithmetic is presentation-only formatting. |
| `BP-UI-004` | Hotseat handoff removes the prior private subtree before the next seat render. | Partner/opponent private data must not linger in DOM/storage/logs. |
| `BP-UI-005` | Observer, replay, rules, and outcome surfaces are complete and accessible. | Shared surfaces render Rust-authored facts. |
| `BP-UI-006` | Reduced motion and logical focus/status behavior preserve semantic results. | Accessibility changes do not change game outcome. |

## Out-Of-Scope And Forbidden Rules

| Boundary | Rulepath position |
|---|---|
| variable seats | Out of scope; exactly four seats only. |
| alternate partnerships | Out of scope; no chosen, rotating, inferred, or three-team layout. |
| deck variants | Out of scope; no jokers, deuce-high, wild cards, second deck, or stripped deck. |
| bidding variants | Out of scope; no board minimum, total-bids-not-13 hook, second round, pass, zero numeric bid, blind numeric bid, moon/Boston, mirrors, suicide, or secret bid. |
| nil variants | Out of scope; no passing, partner consultation, double-nil special score, automatic win, or failed-nil-helping-team rule. |
| play variants | Out of scope; no renege/call-renege, claim, takeback, lowest-club first lead, or spades-lead-anytime mode. |
| scoring variants | Out of scope; no 200/300/1000 target, fixed hand count, mercy/backdoor loss, no-bag mode, 5-bag threshold, or arbitrary tiebreaker. |
| networking | Forbidden for v1/v2; no accounts, chat, matchmaking, tournaments, hosted multiplayer, stakes, or casino framing. |
| generic framework | Forbidden in this gate; no generic partnership/team/bid/contract/nil/bag framework and no `engine-core` noun growth. |
| behavior in data | Forbidden; no formulas, selectors, triggers, conditions, scripts, loops, tactical policy, or DSL/YAML behavior. |
