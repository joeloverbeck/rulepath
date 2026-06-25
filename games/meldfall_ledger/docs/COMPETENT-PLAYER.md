# Meldfall Ledger Competent Player Analysis

Game ID: `meldfall_ledger`

Implemented variant: `classic_500_single_deck_v1`

Rules version checked: `meldfall-ledger-rules-v1`

Date: 2026-06-26

## Purpose And Authority

This document is strategy analysis for future bot work. It is not rule
authority. [RULES.md](RULES.md) wins over this document whenever they differ.

All prose is original Rulepath prose. Sources are recorded in
[SOURCES.md](SOURCES.md).

## Sources And Observations

| Source/reference | Date consulted | Used for | Copied prose status | Notes |
|---|---:|---|---|---|
| [RULES.md](RULES.md) | 2026-06-26 | implemented rule IDs and legal boundaries | none | Rule authority. |
| [SOURCES.md](SOURCES.md) | 2026-06-26 | rules-family facts and strategy background | none | Consulted-not-copied source notes. |
| `games/meldfall_ledger/src/bots.rs` | 2026-06-26 | shipped L0 limits and L1-not-admitted status | none | L0 is a legality baseline, not competent play. |
| self-play/code review | 2026-06-26 | strategy implications from implemented draw, meld, lay-off, scoring, visibility, and simulation behavior | none | No external strategy prose copied. |

## Rules Cross-Check

| Strategy area | Rule IDs checked | Any uncertainty? | Notes |
|---|---|---:|---|
| draw source and pickup commitment | `ML-TURN-001` through `ML-TURN-004` | no | A discard pickup creates an immediate-use obligation for the selected card. |
| meld construction | `ML-MELD-001` through `ML-MELD-005` | no | Sets/runs are public after tabled; no rearrangement is allowed. |
| lay-off timing and credit | `ML-LAYOFF-001` through `ML-LAYOFF-003`, `ML-SCORE-006` | no | A player may extend any public meld and keeps score credit for the laid-off card. |
| discard and going out | `ML-TURN-005` through `ML-TURN-009` | no | A player may go out by tabling every card or by final discard. |
| scoring and match pressure | `ML-SCORE-*`, `ML-MATCH-*` | no | In-hand penalties can erase tabled gains; 500 ties continue. |
| hidden information | `ML-VIS-*`, `ML-BOT-*` | no | Opponent hands and stock order are not available to bots or public viewers. |

## Competent-Player Summary

A competent Meldfall Ledger player shapes the private hand toward likely sets
and runs, times melds so tabled points do not give away too much tempo, weighs
discard-pile pickups against the immediate-use burden, manages high-card
penalties, and watches public melds for safe lay-off chances. They infer
opponent proximity from public facts such as hand counts, tabled cards, discard
choices, and pickup behavior, but they never know opponent hands or hidden stock
order.

The shipped Level 0 bot is not a competent-player proxy. It is a safe,
deterministic random-legal baseline.

## Seat And Opponent Model

| Field | Analysis | Rule IDs | Notes |
|---|---|---|---|
| supported seats | 2, 3, 4, 5, or 6 | `ML-SETUP-001` | Policy evidence must cover variable table size. |
| partnership | none | `ML-SETUP-001` | Every seat scores independently. |
| opponent set | every other active seat | `ML-SETUP-002` | Multi-opponent risk increases with seat count. |
| order pressure | first active seat is left of dealer; play proceeds clockwise | `ML-SETUP-005` | Position affects what public facts are available before a choice. |

## Phases And Situations

| Phase/situation | What competent players notice | Rule IDs | Notes |
|---|---|---|---|
| draw choice | own meld candidates, public discard pile, stock count, hand size, and whether a pickup can be used immediately | `ML-TURN-001` through `ML-TURN-004` | The top discard is not free; it still creates the same use requirement. |
| table play | complete melds, safe lay-offs, pickup commitment satisfaction, go-out opportunity | `ML-TURN-005`, `ML-MELD-*`, `ML-LAYOFF-*` | Optional table play can become mandatory when a pickup commitment exists. |
| discard | remaining hand shape, high-card penalty exposure, public discard risk | `ML-TURN-006`, `ML-SCORE-003` | A discard may help the next seat if it completes a visible or likely meld. |
| going out | whether emptying the hand now beats holding for a larger score | `ML-TURN-007`, `ML-TURN-008`, `ML-SCORE-*` | Going out immediately settles opponent in-hand penalties. |
| stock exhaustion | stock count and discard-pile usability | `ML-TURN-009` | The discard pile is not reshuffled in this variant. |
| late match | cumulative scores, possible 500 reach, and tie-continuation risk | `ML-MATCH-*` | A unique highest score at or above 500 wins only after settlement. |

## Hand-Shaping Principles

| Principle | Visible facts used | Rule IDs | Notes |
|---|---|---|---|
| preserve near melds | own hand, public discard options | `ML-MELD-001` through `ML-MELD-003` | Two-card set/run fragments can become tabled points later. |
| prefer flexible cards | own hand and public table | `ML-LAYOFF-001` | Cards that extend existing public melds have immediate outlet value. |
| reduce isolated high cards | own hand and score posture | `ML-SCORE-001`, `ML-SCORE-003` | Aces and face cards are painful if still in hand at settlement. |
| account for seat count | public hand counts, stock count, opponent count | `ML-SETUP-001`, `ML-VIS-001` | More opponents means a risky discard reaches more potential beneficiaries. |
| avoid impossible plans | legal meld definitions and no-rearrangement rule | `ML-MELD-*`, `ML-LAYOFF-003` | A tabled meld cannot be split or rearranged later. |

## Meld Timing

| Situation | Competent principle | Rule IDs | Bot feature candidate? |
|---|---|---|---:|
| immediate pickup commitment | table the selected discard in a new meld or lay-off before ending the turn | `ML-TURN-004` | yes |
| large tabled score now | prefer tabled points when it reduces high-card exposure or approaches go-out | `ML-SCORE-002` through `ML-SCORE-004` | future |
| weak partial hand | sometimes delay marginal melds if tabling them exposes lay-off targets without improving go-out odds | `ML-LAYOFF-001`, `ML-SCORE-006` | future |
| go-out opportunity | empty the hand when the settlement swing is favorable | `ML-TURN-007`, `ML-TURN-008` | future |
| opponent near empty | table points and shed penalties before the round can end | `ML-VIS-001`, `ML-SCORE-003` | future |

## Discard Risk

| Discard consideration | Lawful evidence | Risk | Forbidden shortcut |
|---|---|---|---|
| completes an existing public meld by lay-off | public meld tableau | next seat or later seat may score it immediately | inspecting opponent hands to see whether they can use it |
| extends obvious public run/set pressure | public discards and tabled cards | may help a visible plan | reading hidden candidate rankings |
| sheds high card | own hand and score values | reduces own penalty but may feed another seat | checking hidden stock/order to see what happens next |
| breaks own future meld | own hand | can reduce future tabled points | none |
| final discard to go out | own hand after table plays | ends round and locks all penalties | revealing opponent unmelded identities to evaluate exact swing |

## Discard-Pile Pickup Risk And Reward

| Pickup choice | Reward | Risk/cost | Rule IDs |
|---|---|---|---|
| draw stock | adds one hidden card without public table obligation | no known card identity before draw; may not improve hand | `ML-TURN-002` |
| pick top discard | takes a known public card | still must use that card immediately in this variant | `ML-TURN-003`, `ML-TURN-004` |
| pick deeper discard | takes selected card and every newer discard above it | larger hand may add penalty burden and more forced cleanup | `ML-TURN-003`, `ML-TURN-004` |
| pick to lay off | converts selected card into immediate public score | may leave unwanted newer pickup cards in hand | `ML-LAYOFF-001`, `ML-SCORE-006` |
| pick to go out | can end the round with tabled points and no in-hand penalty | must satisfy the selected-card commitment before settlement | `ML-TURN-007`, `ML-TURN-008` |

## High-Card Penalty Management

| Situation | Competent response | Rule IDs | Notes |
|---|---|---|---|
| isolated ace or face card | meld, lay off, or discard before an opponent can go out | `ML-SCORE-001`, `ML-SCORE-003` | Aces are 15 even in low runs. |
| high card inside near run/set | keep only when the completion path is plausible from own hand and public facts | `ML-MELD-*` | Do not infer hidden stock contents. |
| opponent hand count drops | reduce exposed penalty load | `ML-VIS-001`, `ML-SCORE-003` | Public hand counts are legal proximity signals. |
| near 500 | prefer positive settlement that creates unique highest score | `ML-MATCH-001`, `ML-MATCH-002` | Ties at or above 500 continue. |
| tied high score pressure | avoid a weak settlement that only continues the match | `ML-MATCH-003` | Seat order does not break the tie. |

## Public Proximity Inference

| Signal | Visible to whom | Strategic meaning | Bot feature candidate | Notes |
|---|---:|---|---:|---|
| opponent hand count | all viewers | lower count means higher go-out threat | future | Exact cards remain hidden. |
| opponent pickup depth | all viewers from public discard action | selected public card likely had immediate use | future | Inference is legal because the selected card and public action are visible. |
| new public meld or lay-off | all viewers | confirms a scoring outlet and may expose new lay-off targets | yes | Tabled cards stay public. |
| skipped discard pickup | all viewers by action history | may imply no immediate use for visible discard, but not certainty | future | Do not convert into hidden hand knowledge. |
| stock count | all viewers | stock exhaustion pressure | future | Stock order remains hidden. |

## Common Beginner Mistakes

| Mistake | Why it is bad | How competent play avoids it | Rule IDs | Bot test implied? |
|---|---|---|---|---:|
| taking a discard without a plan for the selected card | pickup commitment blocks ending the turn | pick only when the selected card can be melded or laid off immediately | `ML-TURN-004` | yes |
| treating tabled meld origin as score credit | laid-off cards score to the player who played them | track per-card score credit | `ML-SCORE-006` | yes |
| holding isolated high cards too long | settlement subtracts in-hand values | shed, meld, or lay off high cards before go-out threat rises | `ML-SCORE-001`, `ML-SCORE-003` | yes |
| discarding into an obvious public lay-off | the next player may score the card | compare discard candidates against public melds | `ML-LAYOFF-001` | yes |
| assuming stock order | stock identities are hidden | reason from public counts and visible actions only | `ML-VIS-003`, `ML-BOT-002` | yes |
| claiming random legal play is competent | L0 ignores strategy | keep L0 as simulation/legality baseline only | `ML-BOT-001` | yes |

## Hidden/Private Information Boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| own private hand | yes | yes | yes | none | Authorized seat-private information. |
| public discard pile, melds, scores, hand counts, stock count | yes | yes | yes | none | Public table facts. |
| opponent private hands | no | no actual identities | no | high | May infer only from public behavior. |
| hidden stock order and next stock card | no | no | no | high | Stock draw identity is private to the drawing seat after draw. |
| private settlement cards for other seats | no | no actual identities | no | high | Public settlement exposes totals/counts only. |
| bot candidate rankings | no public default | not strategy input | no hidden facts | medium | Future rankings must be viewer-safe. |

## Strategy Examples

| Example ID | Situation | Candidate choices | Competent choice principle | Explanation | Rule IDs |
|---|---|---|---|---|---|
| `ML-S-EX-001` | The top discard completes a legal run with two own cards. | stock / top discard | prefer pickup if the selected discard can be tabled immediately | Pickup reward is high because the commitment has an immediate outlet. | `ML-TURN-003`, `ML-TURN-004`, `ML-MELD-002` |
| `ML-S-EX-002` | A deeper discard completes a set but two newer high cards would also be taken. | stock / deeper discard | compare immediate tabled score against added penalty burden | Deeper pickup can be correct, but it is not free. | `ML-TURN-003`, `ML-SCORE-*` |
| `ML-S-EX-003` | Opponent has one card and own hand contains an isolated ace. | discard ace / preserve ace | reduce high-card penalty if the discard does not create obvious public lay-off value | Public hand count creates legal go-out pressure. | `ML-VIS-001`, `ML-SCORE-001`, `ML-SCORE-003` |

## Anti-Examples

| Anti-example ID | Bad choice | Why it is bad | Rule IDs | Bot guard/test implied |
|---|---|---|---|---|
| `ML-S-BAD-001` | Ending the turn after taking a discard without using the selected card. | Illegal; pickup commitment remains unsatisfied. | `ML-TURN-004`, `ML-TURN-006` | legal action API only |
| `ML-S-BAD-002` | Discarding safely because the next stock card is known to be useless. | Stock order is hidden. | `ML-VIS-003`, `ML-BOT-002` | stock-order no-leak test |
| `ML-S-BAD-003` | Avoiding a discard because an opponent secretly holds matching cards. | Opponent hands are unavailable. | `ML-VIS-003`, `ML-BOT-002` | opponent-hand mutation/no-leak tests |
| `ML-S-BAD-004` | Marketing L0 as a competent rummy player. | The policy is random legal and not strategy-backed. | `ML-BOT-001` | evidence-pack gate |

## Candidate Level 2 Features

| Candidate feature | Derived from | Visible to bot? | Used for | Hidden-info risk | Test implied |
|---|---|---:|---|---|---|
| meld candidate extraction | own hand plus public meld tableau | yes | draw/table-play ranking | low | own-view tests |
| discard risk screen | own discard candidates plus public melds/discards | yes | discard ranking | medium | opponent-hand mutation invariance |
| pickup value model | public discard pile, own hand, immediate-use paths | yes | draw source ranking | medium | pickup commitment scenarios |
| high-card penalty posture | own hand values, public hand counts, score totals | yes | discard and table timing | low | hand-count pressure scenarios |
| public proximity model | hand counts, public actions, tabled cards, stock count | yes | risk posture | medium | full-trace exclusion tests |
| match target posture | public cumulative scores and tie state | yes | go-out and scoring priorities | low | target/tie scenarios |

## Review Checklist

- This document does not authorize Level 1 or Level 2.
- Strategy claims are checked against [RULES.md](RULES.md).
- Hidden-information boundaries are explicit.
- Level 0 is not represented as competent-human play.
