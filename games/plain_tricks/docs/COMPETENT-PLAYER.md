# Plain Tricks Competent Player Analysis

Game ID: `plain_tricks`

Implemented variant: `plain_tricks_standard`

Rules version checked: `plain-tricks-rules-v1`

Date: 2026-06-09

## Purpose and Authority

This is strategy analysis for the implemented Plain Tricks variant. It informs
the Level 2 bot but does not define rules. If this document conflicts with
[RULES.md](RULES.md), the rules win.

## Sources and References

| Source/reference | Date consulted | Source quality | Used for | Copied prose status | Notes |
|---|---|---|---|---|---|
| [RULES.md](RULES.md) | 2026-06-09 | rules authority | legal plays, trick winner, round scoring, terminal outcome | none | Local Rulepath prose. |
| [replay.rs](../tests/replay.rs) | 2026-06-09 | executable evidence | trace categories, no-leak replay/export surfaces | none | Golden replay evidence for rules and visibility. |
| [visibility.rs](../tests/visibility.rs) | 2026-06-09 | executable evidence | observer/seat visibility boundaries | none | No-leak examples. |

## Competent-Player Summary

Competent Plain Tricks play is short-horizon trick management under hidden
hands:

- choose only legal play actions for the active seat;
- follow suit when possible and use the full hand only when leading or unable
  to follow;
- win a trick when taking the lead improves the round score or controls the
  next lead;
- duck a trick with a low legal card when spending a winner is unnecessary;
- lead established high cards when the public history shows they are likely to
  hold;
- lead low from a longer suit when no immediate winner is clear;
- use public off-suit plays as evidence that a seat lacked the led suit at that
  moment, without recording an explicit hidden void flag.

## Phases and Situations

| Phase/situation | What competent players notice | Important rules | Notes |
|---|---|---|---|
| Opening lead | No current led suit exists; the leader may play any card in hand. | legal action tree, lead suit | Prefer a high card for control or a low card from length when conserving strength. |
| Following suit | The led suit is public and constrains legal choices if held. | must-follow | If able to follow, compare legal same-suit ranks against the lead card. |
| Unable to follow | The full hand is legal because no led-suit card is held. | off-suit cannot win | Spend the least useful card unless shedding a dangerous high card is better. |
| After a trick resolves | Winner leads next; trick counts are public. | trick winner, leader rotation | Leading control can be worth more when the score is close. |
| Late round | Remaining hand size and public history narrow practical choices. | six tricks per round | Preserve known winners when already ahead; seek control when behind. |

## Immediate Tactics

| Tactic | Situation | Why it matters | Bot feature candidate? |
|---|---|---|---:|
| Beat the led card cheaply | Follower has one or more led-suit cards that can win | Wins the trick while preserving higher winners. | yes |
| Duck with the lowest legal card | Follower cannot win or does not need control | Conserves stronger cards and avoids waste. | yes |
| Lead a likely winner | Leader has a high card in a suit with public lower cards already spent | Converts strength into a trick and next lead. | yes |
| Lead low from length | No clear winner is available | Probes safely while preserving higher cards. | yes |
| Respect public void evidence | A prior off-suit follow happened in a suit | Avoid overvaluing that suit against that seat later. | yes |

## Hidden/Private Information Boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| own hand | yes | yes | yes | low | Seat-private view only. |
| legal action tree | yes | yes | yes | none | Rust supplies legal actions. |
| public current trick and trick history | yes | yes | yes | none | Played cards are public after play. |
| public trick counts/totals | yes | yes | yes | none | Scoring surface is public. |
| public off-suit evidence | yes | yes | yes | medium | Use only as public-history inference, not as an explicit hidden void table. |
| opponent hand | no | no | no | high | Must not appear in bot input or explanations. |
| tail cards | no | no | no | high | Internal only, including terminal exports. |
| seed/shuffle order | no | no | no | high | Not strategy input. |

## Strategy Examples

| Example ID | Situation | Candidate choices | Competent choice | Explanation |
|---|---|---|---|---|
| `PT-EX-001` | Following `Gale 3` with `Gale 4` and `Gale 6` legal | Gale 4, Gale 6 | Gale 4 | Win cheaply and preserve the higher Gale. |
| `PT-EX-002` | Following `River 5` with only lower River cards legal | River 1, River 3 | River 1 | The trick cannot be won; spend the lowest legal card. |
| `PT-EX-003` | Leading while behind late in the round with a high unplayed Ember | several hand cards | high Ember | Try to win and keep lead control. |
| `PT-EX-004` | Unable to follow the led suit | full hand | lowest low-value discard | Off-suit cannot win, so discard without exposing extra strategy. |

## Common Beginner Mistakes

| Mistake | Why it is bad | How competent play avoids it | Bot test implied? |
|---|---|---|---:|
| Spending the highest card whenever it can win | Wastes future control. | Win with the cheapest winning card. | yes |
| Treating off-suit cards as able to win | Contradicts trick resolution. | Off-suit follows are discard opportunities. | yes |
| Ignoring next lead | Misses the value of winning a trick. | Weigh trick count and leader control together. | yes |
| Acting as if opponent hand or tail is known | Hidden-info leak. | Use own hand and public history only. | yes |

## Translation to Level 2 Bot Features

| Candidate feature | Visible to bot? | Used for | Hidden-info risk | Test |
|---|---:|---|---|---|
| legal action tree | yes | candidate extraction | none | GAT101PLATRI-013 bot legality tests |
| own hand card ranks/suits | yes | cheap win, duck, lead choice | low | GAT101PLATRI-013 input whitelist |
| public current trick | yes | follow/beat/duck decision | none | GAT101PLATRI-013 priority tests |
| public history and scores | yes | likely winner and control posture | none | GAT101PLATRI-013 repeated playouts |
| explicit opponent void table | no | not used | high | GAT101PLATRI-013 no-leak tests |

## Review Checklist

- Strategy prose is original.
- Rules authority is separate from strategy.
- Hidden-information boundaries are explicit.
- No strategy claim requires MCTS, Monte Carlo, ML, RL, hidden-state sampling,
  opponent-hand access, tail access, or seed reconstruction.
