# Blackglass Pact Competent Player Analysis

Game ID: `blackglass_pact`

Implemented variant: `blackglass_pact_standard`

Rules version checked: `blackglass-pact-rules-v1`

Date: 2026-06-25

## Purpose And Authority

This document is strategy analysis for future bot work. It is not rule
authority. [RULES.md](RULES.md) wins over this document whenever they differ.

All prose is original Rulepath prose. Sources are recorded in [SOURCES.md](SOURCES.md).

## Sources And Observations

| Source/reference | Date consulted | Used for | Copied prose status | Notes |
|---|---:|---|---|---|
| [RULES.md](RULES.md) | 2026-06-25 | implemented rule IDs and legal boundaries | none | Rule authority. |
| [SOURCES.md](SOURCES.md) | 2026-06-25 | rules-family facts and strategy background | none | Consulted-not-copied source notes. |
| `games/blackglass_pact/src/bots.rs` | 2026-06-25 | shipped L0/L1 limits | none | L1 is a bounded baseline, not competent play. |
| self-play/code review | 2026-06-25 | strategy implications from implemented bidding, play, scoring, visibility, and simulations | none | No external strategy prose copied. |

## Rules Cross-Check

| Strategy area | Rule IDs checked | Any uncertainty? | Notes |
|---|---|---:|---|
| blind nil posture | `BP-BLIND-001` through `BP-BLIND-010` | no | Blind decisions happen before any hand exists. |
| bidding and nil risk | `BP-BID-001` through `BP-BID-010`, `BP-SCORE-007` through `BP-SCORE-010` | no | Nil is individual and does not help ordinary contract when failed. |
| trick play and spades | `BP-PLAY-001` through `BP-PLAY-012` | no | Follow-suit legality and spades trump dominate candidate availability. |
| scoring, bags, target | `BP-SCORE-*`, `BP-END-*` | no | Bag pressure and exact ties matter after hand scoring only. |
| hidden information | `BP-VIS-*`, `BP-BOT-*` | no | Partner relationship grants no private hand access. |

## Competent-Player Summary

A competent Blackglass Pact player estimates likely team tricks without
treating every face card as certain, uses spade length/control and short suits
to reason about ruff potential, and separates individual nil goals from the
ordinary team contract. They count public cards, public void signals, public
bids, current trick winners, bags, and score posture, but they never know
partner or opponent unplayed cards.

The shipped Level 1 bot is not a competent-player proxy. It is a safe,
deterministic baseline that implements only direct public-score, own-hand, and
legal-leaf heuristics.

## Seat, Team, And Opponent Model

| Field | Analysis | Rule IDs | Notes |
|---|---|---|---|
| supported seats | exactly four | `BP-SETUP-001` | No variable table size. |
| partnership | `team_0` is North-South; `team_1` is East-West | `BP-SETUP-003` | Partner score is shared; hand information is not. |
| opponent set | two opposing seats | `BP-SETUP-003` | Competent play tracks both opponents through public actions. |
| order pressure | blind and bidding proceed left of dealer; trick winner leads next | `BP-BLIND-002`, `BP-BID-001`, `BP-PLAY-009` | Position changes what is already public. |

## Phases And Situations

| Phase/situation | What competent players notice | Rule IDs | Notes |
|---|---|---|---|
| blind commitment | public score deficit, partner declaration state, target posture | `BP-BLIND-*`, `BP-END-*` | No card identity exists yet. |
| ordinary bidding | own high cards, spade controls, voids/short suits, partner public bid, table score | `BP-BID-*`, `BP-SCORE-*` | Bid likely team tricks, not total card strength. |
| nil decision | own spade height, unsupported high cards, long suits likely to force a winner, position | `BP-BID-003`, `BP-SCORE-007` | A partner may help through public play but cannot promise hidden cover. |
| leading | spades-broken state, safe suits, contract need, nil protection, bag pressure | `BP-PLAY-001`, `BP-PLAY-002`, `BP-PLAY-003` | Breaking spades carelessly can give opponents control. |
| following | forced suit, current winning card, whether winning helps contract or hurts nil/bags | `BP-PLAY-005`, `BP-PLAY-007`, `BP-PLAY-008` | Legal set comes first. |
| late hand | remaining public cards, public voids, team contract progress, nil status, bags | `BP-SCORE-*` | Card memory is public-history memory only. |

## Bidding Principles

| Principle | Visible facts used | Rule IDs | Notes |
|---|---|---|---|
| count likely tricks, not card points | own hand, public position, selected variant | `BP-BID-*` | Aces can lose to spades or be trapped by suit play; kings/queens depend on suit length and exposure. |
| value spade control | own spades, spades as permanent trump | `BP-PLAY-007` | Long spades can create control after suits drain. |
| account for ruff potential | own voids/short suits and enough spades | `BP-PLAY-006`, `BP-PLAY-007` | Ruff potential is not certainty about hidden leads. |
| respect partner public bid | partner bid, team score, bags | `BP-BID-007`, `BP-SCORE-001` | Partner bid is public intent/risk, not hand knowledge. |
| keep nil separate | own hand risk and nil scoring | `BP-BID-008`, `BP-SCORE-007`, `BP-SCORE-009` | Failed nil tricks do not rescue ordinary contract. |

## Nil And Partner Coverage

| Situation | Competent principle | Lawful evidence | Forbidden shortcut |
|---|---|---|---|
| own nil | Prefer nil only with low spade danger, few unsupported high cards, and escape cards. | own hand, own position, public bids | assuming opponents hold covering cards. |
| partner nil | Cover through public trick play when legal and affordable. | partner public bid, current trick, public voids | using partner unplayed hand. |
| opponent nil | Set when it does not sacrifice a higher team priority. | opponent public nil, current trick, public score | assuming hidden opponent suit length. |
| failed nil | Stop treating failed-nil tricks as ordinary help. | public trick ownership and scoring rule | adding failed nil tricks to team ordinary contract. |

## Play Principles

| Principle | Situation | Why it matters | Rule IDs | Bot feature candidate? |
|---|---|---|---|---:|
| legality first | every play | Illegal off-suit or spade-lead shortcuts are unavailable. | `BP-PLAY-002` through `BP-PLAY-006` | yes |
| protect own nil | nil bidder is at risk of taking a trick | Nil value is individual and high. | `BP-SCORE-007` | yes |
| cover partner nil | partner nil is still alive and currently exposed | Team score benefits from partner nil. | `BP-SCORE-007`, `BP-SCORE-009` | yes |
| make ordinary contract | ordinary team still needs tricks | Set contracts are costly. | `BP-SCORE-001` through `BP-SCORE-004` | yes |
| avoid bags after contract | ordinary contract is safe and extra tricks only add bags | Every ten bags costs 100. | `BP-SCORE-005`, `BP-SCORE-012` | yes |
| set opponents | opponents have nil or ordinary contract pressure | Denial can swing score. | `BP-SCORE-*` | future |

## Score And Target Posture

| Situation | Competent response | Rule IDs | Notes |
|---|---|---|---|
| behind by 100+ before blind phase | consider blind-nil risk from public score only | `BP-BLIND-001` | Card-free commitment makes blind nil a score play. |
| near target with many bags | reduce overtrick appetite after making contract | `BP-SCORE-012`, `BP-END-002` | Bags can erase a winning hand. |
| exact tie at or above 500 | keep playing full hands until a unique higher team exists | `BP-END-004` | No arbitrary seat/bag tiebreak. |
| negative scores | evaluate deficit and risk normally | `BP-BLIND-001`, `BP-SCORE-*` | Negative totals do not change rule math. |

## Public Void Inference And Card Memory

| Signal | Visible to whom | Strategic meaning | Bot feature candidate | Notes |
|---|---:|---|---:|---|
| a seat fails to follow suit | all viewers after public play | that seat was void in the led suit at that moment | future | Inference is legal because the play is public. |
| completed tricks | all viewers | played cards cannot be in unplayed hands | future | Memory must be built from public history, not full trace. |
| partner lead or discard | all viewers | may reveal public posture | future | Does not identify partner's remaining cards. |
| current winning card | all viewers | determines whether a legal play currently wins or loses | yes | Must use public trick only. |

## Common Beginner Mistakes

| Mistake | Why it is bad | How competent play avoids it | Rule IDs | Bot test implied? |
|---|---|---|---|---:|
| overbidding face cards | high cards are not guaranteed tricks in a trump game | estimate likely tricks with suit length, spades, and position | `BP-BID-*`, `BP-PLAY-*` | yes |
| bidding nil with high spades | spades are hard to shed safely | screen nil risk from own spade height and support | `BP-BID-003`, `BP-SCORE-007` | yes |
| breaking spades carelessly | gives trump control and can expose nils/contracts | lead spades only under a legal, documented reason | `BP-PLAY-002`, `BP-PLAY-003` | yes |
| helping a failed nil into ordinary contract | failed nil tricks do not count for ordinary team `O` | keep nil and ordinary accounting separate | `BP-SCORE-009` | yes |
| forgetting bag rollover | overtricks can create a -100 penalty | track bags as a separate resource | `BP-SCORE-011`, `BP-SCORE-012` | yes |
| assuming partner unseen cards | partner hand is private | react only to public partner bid/play/void facts | `BP-VIS-003`, `BP-BOT-002` | yes |

## Hidden/Private Information Boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| own private hand | yes | yes | yes | none | Authorized seat-private information. |
| public bids/tricks/scores/bags | yes | yes | yes | none | Public table facts. |
| partner private hand | no | no actual identities | no | high | Partnership does not grant hand sharing. |
| opponent private hands | no | no actual identities | no | high | May infer only from public play history. |
| future deal/deck order | no | no | no | high | Blind phase has no hand at all. |
| bot candidate rankings | no public default | not strategy input | no hidden facts | medium | Dev-only and viewer-safe if later added. |

## Strategy Examples

| Example ID | Situation | Candidate choices | Competent choice principle | Explanation | Rule IDs |
|---|---|---|---|---|---|
| `BP-S-EX-001` | Team trails by 220 before blind phase and partner has not declared. | declare / decline | consider declare only from public score risk | Blind nil is a score posture decision before cards exist. | `BP-BLIND-001`, `BP-BLIND-006` |
| `BP-S-EX-002` | Own hand has low cards and no spades or high controls. | nil / numeric bid | consider nil if legal and team posture allows | Nil risk comes from own authorized hand and public context. | `BP-BID-003`, `BP-SCORE-007` |
| `BP-S-EX-003` | Partner nil is alive and currently losing unless the bot overplays cheaply. | cover / shed | cover if it does not endanger a higher team priority | Partner nil can be protected through public trick play only. | `BP-SCORE-007`, `BP-VIS-003` |

## Anti-Examples

| Anti-example ID | Bad choice | Why it is bad | Rule IDs | Bot guard/test implied |
|---|---|---|---|---|
| `BP-S-BAD-001` | Leading a spade before broken while holding a club. | Illegal; Rust must reject. | `BP-PLAY-002` | legal action API only |
| `BP-S-BAD-002` | Declaring blind nil because the hidden deal is weak. | No card identity exists before blind decision. | `BP-BLIND-006` | blind no-card no-leak test |
| `BP-S-BAD-003` | Covering partner nil because partner secretly holds only low cards. | Partner hand is unavailable. | `BP-VIS-003`, `BP-BOT-002` | partner-hand mutation/no-leak tests |
| `BP-S-BAD-004` | Marketing L1 as a strong Spades player. | The policy is shallow and not Level 2 evidence-backed. | `BP-BOT-004` | evidence-pack gate |

## Candidate Level 2 Features

| Candidate feature | Derived from | Visible to bot? | Used for | Hidden-info risk | Test implied |
|---|---|---:|---|---|---|
| richer own-hand trick estimate | own ranks/suits, public bid order | yes | bid candidate ranking | low | own-view tests |
| nil danger score | own spades/high cards/suit lengths and public position | yes | nil candidate filtering | low | nil scenario corpus |
| partner nil cover priority | public partner nil, current trick, public voids | yes | play candidate ranking | medium | partner-hand mutation invariance |
| opponent nil setting | public opponent nil and current trick | yes | denial priorities | medium | hidden-opponent no-leak tests |
| bag/target posture | public scores and bags | yes | risk posture | low | target/bag scenarios |
| public card-memory summary | played cards only | yes | legal inference | medium | full-trace exclusion tests |

## Review Checklist

- This document does not authorize Level 2.
- Strategy claims are checked against [RULES.md](RULES.md).
- Hidden-information boundaries are explicit.
- Level 1 is not represented as competent-human play.
