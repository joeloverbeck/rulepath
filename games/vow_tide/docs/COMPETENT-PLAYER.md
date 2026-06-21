# Vow Tide Competent Player Analysis

Game ID: `vow_tide`

Implemented variant: `vow_tide_standard`

Rules version checked: `vow-tide-rules-v1`

Date: 2026-06-21

## Purpose And Authority

This document is strategy analysis for future bot work. It is not rule
authority. [RULES.md](RULES.md) wins over this document whenever they differ.

All prose is original Rulepath prose. Sources are recorded in [SOURCES.md](SOURCES.md).

## Sources And Observations

| Source/reference | Date consulted | Used for | Copied prose status | Notes |
|---|---:|---|---|---|
| [RULES.md](RULES.md) | 2026-06-21 | implemented rule IDs and legal boundaries | none | Rule authority. |
| [SOURCES.md](SOURCES.md) | 2026-06-21 | rules-family facts and variant choices | none | Consulted-not-copied source notes. |
| self-play/code review | 2026-06-21 | strategy implications from implemented bidding, play, scoring, and simulations | none | No external strategy prose copied. |

## Rules Cross-Check

| Strategy area | Rule IDs checked | Any uncertainty? | Notes |
|---|---|---:|---|
| bid calibration | `VT-BID-RANGE-001`, `VT-HOOK-001`, `VT-SCORE-001` | no | Exact bids are valuable only when hit. |
| contract-relative play | `VT-FOLLOW-001`, `VT-TRICK-WIN-001`, `VT-SCORE-001` | no | Extra tricks after meeting a bid can turn a success into a miss. |
| hidden information | `VT-VIEW-001`, `VT-BOT-001`, `VT-REPLAY-001` | no | Competent inference must use legal public and own-seat information only. |
| standings and co-wins | `VT-STANDINGS-001`, `VT-OUTCOME-001` | no | Public scores matter, but no L1 kingmaking policy is admitted. |

## Competent-Player Summary

A competent Vow Tide player estimates a realistic contract from their own hand,
the hand size, the trump suit, and the public bid context. During play, they
switch posture based on how many tricks they still need: secure affordable wins
when short of contract, shed or avoid wins once the contract is met, and obey
follow-suit constraints exactly. They remember public cards and public bids but
do not know opponent hands, hidden stock, future deals, or hidden intentions.

The shipped Level 1 bot is not a competent-player proxy. It is a safe,
deterministic baseline that implements only the most direct bid-and-contract
heuristics.

## Seat And Opponent Model

| Field | Analysis | Rule IDs | Notes |
|---|---|---|---|
| supported seat range | 3-7 seats | `VT-SEATS-001` | Everyone is an independent table competitor. |
| number of opponents | 2-6 opponents | `VT-SEATS-001` | No teams or partnerships. |
| partnership/team roles | none | `VT-SEATS-001` | No shared hidden information. |
| turn-order pressure | dealer bids last and is constrained by the hook; trick winner leads next | `VT-HOOK-001`, `VT-NEXT-LEAD-001` | Late bidding is informationally stronger but constrained. |

## Phases And Situations

| Phase/situation | What competent players notice | Rule IDs | Notes |
|---|---|---|---|
| early bidding | own high cards, trump controls, suit shape, hand size | `VT-BID-RANGE-001`, `VT-SCORE-001` | Overbidding and underbidding both score zero on a miss. |
| dealer bid | public total before dealer and the forbidden hook value | `VT-HOOK-001` | Dealer may be pushed away from the preferred bid. |
| leading a trick | whether winning a trick helps or hurts the current contract | `VT-FIRST-LEAD-001`, `VT-SCORE-001` | Any held card may lead, including trump. |
| following suit | forced follow-suit may remove the desired contract posture | `VT-FOLLOW-001` | Legal set, not strategy, decides available cards. |
| void in led suit | opportunity to trump for a win or shed a low-risk loser | `VT-FOLLOW-001`, `VT-TRICK-WIN-001` | Future hidden cards are still unknown. |
| late hand | exact current trick count relative to bid | `VT-SCORE-001` | One extra trick can ruin a made contract. |

## Immediate Tactics

| Tactic | Situation | Why it matters | Rule IDs | Bot feature candidate? |
|---|---|---|---|---:|
| avoid the dealer hook | dealer bidding after public prefix total | illegal hook value cannot be chosen | `VT-HOOK-001` | yes |
| secure a cheap current win | still needs tricks and a low card is currently winning | reaches contract without spending a higher card | `VT-TRICK-WIN-001`, `VT-SCORE-001` | yes |
| shed a losing card after contract met | no more tricks needed | avoids overtrick miss risk | `VT-SCORE-001` | yes |
| follow suit exactly | holding led suit | illegal off-suit shortcuts must not be considered | `VT-FOLLOW-001` | yes |
| avoid claiming hidden certainty | any hidden opponent hand/stock situation | actual unknown cards cannot drive policy | `VT-VIEW-001`, `VT-BOT-001` | yes |

## Positional, Resource, Card, And Tempo Principles

| Principle type | Principle | Visible evidence | Rule IDs | Notes |
|---|---|---|---|---|
| positional | dealer is last bidder but hook-constrained | public bid order and total | `VT-BID-ORDER-001`, `VT-HOOK-001` | Later information may still force a second-best legal bid. |
| resource/accounting | bid minus tricks taken is the central play posture | public bid and trick count | `VT-SCORE-001` | L1 uses this directly. |
| card/hand/deck | own trump controls and aces raise bid confidence | own hand and public trump | `VT-TRUMP-001`, `VT-BOT-001` | L1 uses aces plus trump jack-or-higher controls only. |
| tempo/initiative | winning a trick controls the next lead | public current trick | `VT-NEXT-LEAD-001` | Future Level 2 may value lead control. |
| risk/control | zero bids score well but require avoiding every trick | public bid and legal cards | `VT-SCORE-001` | L1 handles only contract-needed posture. |

## Common Beginner Mistakes

| Mistake | Why it is bad | How competent play avoids it | Rule IDs | Bot test implied? |
|---|---|---|---|---:|
| bidding from hand strength without considering exactness | taking too many tricks still scores zero | estimate likely tricks, not total card power | `VT-SCORE-001` | yes |
| forgetting the dealer hook | selected bid is illegal | remove the hook value from dealer choices | `VT-HOOK-001` | yes |
| winning after contract is already met | creates overtrick miss risk | prefer legal losing cards when possible | `VT-SCORE-001` | yes |
| assuming an opponent is void from hidden knowledge | unavailable to the seat | infer only from public plays already made | `VT-VIEW-001`, `VT-BOT-001` | yes |
| using stock identity or deck order | forbidden hidden future/random fact | never include hidden stock/deck in bot input | `VT-TRUMP-001`, `VT-BOT-001` | yes |

## Risk Posture

| Situation | Cautious posture | Aggressive posture | Recommended default | Notes |
|---|---|---|---|---|
| uncertain bid | choose nearer lower legal bid | choose nearer higher legal bid | cautious for L1 tie-break | L1 ties to lower numeric bid. |
| needing one trick | secure the lowest current win | spend high card to force a win | cautious | L1 uses lowest current winner when available. |
| contract already met | shed lowest losing card | try to control lead anyway | cautious | Future L2 may account for lead control. |
| zero bid | avoid every win | take control to shed later | cautious | Requires careful current-trick comparison. |

## Visible Signals

| Signal | Visible to whom | Strategic meaning | Bot feature candidate | Notes |
|---|---|---|---:|---|
| public bids | all viewers | contract targets and table pressure | yes | L1 reads own bid and public rows. |
| trick counts | all viewers | how many tricks each seat has taken | yes | L1 computes own needed count. |
| current trick cards | all viewers | current winning card and suit/trump context | yes | L1 uses the pure comparator. |
| trump indicator | all viewers | trump suit only | yes | Indicator card is public; hidden stock is not. |
| own hand | owning seat only | legal play/bid features | yes | L1 uses own controls. |
| hidden stock count | all viewers as count | no identity meaning | no for L1 | Count may display; identity/order forbidden. |

## Hidden/Private Information Boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| own private hand | yes | yes | yes | none | Authorized seat-private information. |
| public bids/tricks/scores | yes | yes | yes | none | Public table facts. |
| opponent private hands | no | no actual identities | no | high | May infer only from public play history. |
| hidden stock identity/order | no | no | no | high | Stock count is not card knowledge. |
| future deals/randomness | no | no | no | high | Seed reconstruction is forbidden. |
| bot candidate rankings | no public default | not strategy input | no public hidden facts | medium | Dev-only and viewer-safe if later added. |

## Private Inference Forbidden

| Tempting shortcut | Why forbidden | Required bot guard/test | Notes |
|---|---|---|---|
| reading opponent hands to decide whether a low winner is safe | unavailable to acting seat | opponent-hand no-leak and mutation tests for future L2 | L1 does not inspect opponent hands. |
| reading hidden stock to adjust bid | hidden setup fact | bot input no-leak canary | Covered by L1 input test. |
| simulating hidden worlds from actual state | hidden-state shortcut and Monte Carlo-like path | public v1/v2 exclusion | No L2/search admitted. |
| reconstructing deck from seed | future hidden setup fact | replay/export no-leak tests | Seed material must not be bot input. |

## Kingmaking And Coalition Risk

| Risk | Visible trigger | Competent response principle | Bot feature candidate? | Rule IDs | Notes |
|---|---|---|---:|---|---|
| table-leader help | public standings show one leader near final hands | use only public scores and legal cards; avoid gratuitously feeding an obvious leader when a safer legal play exists | future | `VT-STANDINGS-001` | Not in L1. |
| kingmaking by overtrick pressure | a non-winning seat can force another player's miss | evaluate only public bid/trick counts and current trick | future | `VT-SCORE-001` | Requires careful multi-opponent evidence. |

## Strategy Examples

| Example ID | Situation | Candidate choices | Competent choice | Explanation | Rule IDs |
|---|---|---|---|---|---|
| `VT-S-EX-001` | Dealer prefers bid 1, but public prefix makes 1 the hook value. | `bid/0`, `bid/2` | nearest legal alternative by policy | Hook value is illegal; choose from legal leaves only. | `VT-HOOK-001` |
| `VT-S-EX-002` | Needs one trick, follows a led suit with queen and ace; queen is currently winning. | queen / ace | queen | Secure current win with cheaper card. | `VT-TRICK-WIN-001`, `VT-SCORE-001` |
| `VT-S-EX-003` | Contract already met and can legally lose with a low off-suit card. | low losing card / trump | low losing card | Avoid overtrick miss risk. | `VT-SCORE-001` |

## Anti-Examples

| Anti-example ID | Bad choice | Why it is bad | Rule IDs | Bot guard/test implied |
|---|---|---|---|---|
| `VT-S-BAD-001` | Claiming an off-suit play is legal while holding led suit. | Illegal; Rust must reject. | `VT-FOLLOW-001` | legal action API only |
| `VT-S-BAD-002` | Bidding because the hidden stock contains many trumps. | Hidden stock identity is unavailable. | `VT-VIEW-001`, `VT-BOT-001` | no-leak input test |
| `VT-S-BAD-003` | Marketing L1 as a competent Oh Hell expert. | The policy is shallow and not evidence-backed as Level 2. | `VT-BOT-001` | evidence-pack gate |

## Known Hard Problems

| Problem | Why hard | Out-of-scope for current bot? | Notes |
|---|---|---:|---|
| bid calibration by seat count and table style | Needs source-safe strategy, simulations, and hidden-info-safe features. | yes | Future Level 2 only. |
| endgame leader targeting | Multi-opponent incentives can create kingmaking risk. | yes | Requires public-score-only tests. |
| public card-memory inference | Legal inference from played cards is useful but easy to confuse with hidden hand peeking. | yes | Future evidence pack item. |

## Candidate Level 2 Features

| Candidate feature | Derived from | Visible to bot? | Used for | Hidden-info risk | Test implied |
|---|---|---:|---|---|---|
| richer own-hand control estimate | own hand, trump, hand size | yes | bid candidate ranking | low | own-view tests |
| public trick win/loss annotation | current trick and legal card | yes | play priorities | low | comparator tests |
| public standing pressure | cumulative scores and hand index | yes | endgame priorities | medium | kingmaking examples |
| public card-memory summary | played cards only | yes | future trick-risk estimate | medium | no hidden-state tests |

## Review Checklist

- This document does not authorize Level 2.
- Strategy claims are checked against [RULES.md](RULES.md).
- Hidden-information boundaries are explicit.
- Level 1 is not represented as competent-human play.
