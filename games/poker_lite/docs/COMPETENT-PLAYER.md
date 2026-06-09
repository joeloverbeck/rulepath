# Crest Ledger Competent Player Analysis

Game ID: `poker_lite`

Implemented variant: `poker_lite_standard`

Rules version checked: `poker-lite-rules-v1`

Date: 2026-06-09

## Purpose and Authority

This is strategy analysis for the implemented Crest Ledger variant. It informs
the Level 2 bot but does not define rules. If this document conflicts with
[RULES.md](RULES.md), the rules win.

## Sources and References

| Source/reference | Date consulted | Source quality | Used for | Copied prose status | Notes |
|---|---|---|---|---|---|
| [RULES.md](RULES.md) | 2026-06-09 | rules authority | legal actions, reveal timing, terminal outcomes | none | Local Rulepath prose. |
| [bots.rs](../tests/bots.rs) | 2026-06-09 | executable evidence | policy examples and no-leak checks | none | Tests for the shipped Level 2 bot. |

## Competent-Player Summary

Competent Crest Ledger play is bounded public-price management under hidden
private crests:

- choose only legal pledge actions for the active seat;
- understand when two holds close a round and reveal or resolve;
- press with strong private potential before the center is public;
- protect a made pair after center reveal;
- match affordable public prices instead of yielding too cheaply;
- avoid spending the one lift in a round without a made public reason;
- yield only when the public price is bad enough that continuing is worse.

## Phases and Situations

| Phase/situation | What competent players notice | Important rules | Notes |
|---|---|---|---|
| Opening pledge round | Center is hidden; only own private rank and public price are known. | legal action tree, hidden center | High private rank supports pressure; low/middle ranks prefer cheaper closure. |
| Facing a press/lift | Public outstanding amount fixes the cost to continue. | match/yield/lift legality | Match when affordable; lift mainly when own public strength justifies it. |
| Center revealed | Own private rank can be compared to public center rank. | center reveal, showdown strength | A made pair is strong and worth protecting. |
| Final pledge round | Closing the round resolves showdown unless someone yields. | terminal rules | Avoid unnecessary risk if a legal close is adequate. |

## Immediate Tactics

| Tactic | Situation | Why it matters | Bot feature candidate? |
|---|---|---|---:|
| Press high private rank pre-reveal | Active with no outstanding amount before center reveal | It applies public pressure while the acting seat has strong private potential. | yes |
| Match affordable price | Facing an outstanding amount not above the round unit | Preserves showdown access without overusing lift. | yes |
| Protect made pair | Center is visible and own private rank matches it | A pair beats unpaired high-card strength. | yes |
| Yield to poor price | Facing public price without made pair and above default comfort | Avoids adding markers to a weak position. | yes |

## Hidden/Private Information Boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| own private crest rank | yes | yes | yes | low | Bot input stores rank/bucket, not full opponent state. |
| opponent private crest | no | no | no | high | Covered by `level2_input_whitelist_excludes_forbidden_hidden_material`. |
| hidden center before reveal | no | no | no | high | Bot input says `center_rank=hidden`. |
| deck tail/future order | no | no | no | high | Not present in bot input. |
| public pool/contributions/round price | yes | yes | yes | none | Public pledge state. |

## Strategy Examples

| Example ID | Situation | Candidate choices | Competent choice | Explanation |
|---|---|---|---|---|
| `S-EX-001` | Opening, high private rank, no outstanding amount | hold, press | press | Pressure is justified by strong private potential. |
| `S-EX-002` | Facing a public price with a made pair after reveal | lift, match, yield | match | Continue to showdown while avoiding unnecessary extra lift risk. |
| `S-EX-003` | Weak uncertain hand facing expensive public price | match, yield | yield | Stop paying into a low-confidence position. |

## Common Beginner Mistakes

| Mistake | Why it is bad | How competent play avoids it | Bot test implied? |
|---|---|---|---:|
| Lifting whenever available | The one-lift cap is valuable and extra cost may be unjustified. | Reserve lift for made-pair pressure or clearly favorable public price. | yes |
| Yielding to every press | Cheap matches preserve showdown equity. | Match affordable public prices. | yes |
| Acting as if the hidden center is known | Leaks or assumes unavailable information. | Treat hidden center as unknown until reveal. | yes |

## Translation to Level 2 Bot Features

| Candidate feature | Visible to bot? | Used for | Hidden-info risk | Test |
|---|---:|---|---|---|
| legal action tree | yes | candidate extraction | none | `random_and_level2_decisions_are_legal_and_do_not_mutate_state` |
| own rank/bucket | yes | pressure and pair protection | low | `level2_input_whitelist_excludes_forbidden_hidden_material` |
| public center rank after reveal | yes | made-pair detection | low | `level2_policy_uses_authored_priority_and_stable_tie_break` |
| public price | yes | match/yield/lift posture | none | `level2_bots_finish_many_games_with_legal_actions_under_cap` |

## Review Checklist

- Strategy prose is original.
- Rules authority is separate from strategy.
- Hidden-information boundaries are explicit.
- Examples map to bot tests in [bots.rs](../tests/bots.rs).
- No strategy claim requires MCTS, Monte Carlo, ML, RL, or hidden-state sampling.
