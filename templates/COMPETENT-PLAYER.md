# <game_id> Competent Player Analysis

Game ID: `<game_id>`

Implemented variant: `<variant>`

Rules version checked: `<rules_version>`

Prepared by: `<name/agent>`

Date: YYYY-MM-DD

Evidence receipt: [`GAME-EVIDENCE.md`](GAME-EVIDENCE.md)

Template realignment mapping: report `B-10 -> COMPETENT-PLAYER.md`. This
template owns human-readable strategy claims, examples, anti-examples, visible
inference, and rule-ID links when a game needs public strategy guidance or a
Level 2 strategy base. `GAME-EVIDENCE.md` owns completion profile, bot level,
policy ID, benchmark workload, and release-status links.

## Purpose and authority

This document is human/LLM-authored strategy analysis for the implemented Rulepath variant. It feeds Level 2 bot design.

This document is not rule authority. Strategy claims MUST be checked against `GAME-RULES.md`. If a strategy claim conflicts with rules, the rules win and this document must be corrected.

Do not copy external strategy prose. Summarize in original language and record sources.

## Sources and consulted strategy references

| Source/reference | URL/reference | Date consulted | Source quality | Used for | Copied prose status | Notes |
|---|---|---|---|---|---|---|
| `<source>` | `<url_or_reference>` | YYYY-MM-DD | expert guide / rules authority / community discussion / self-play observation / human analysis / unverified | strategy / tactics / terminology / common mistakes / examples | none | `<notes>` |

## Rules cross-check

| Strategy area | Rule IDs checked | Any uncertainty? | Notes |
|---|---|---:|---|
| `<area>` | `<rule_ids>` | yes/no | `<notes>` |

## Competent-player summary

In original Rulepath prose, summarize what competent play means for this game and variant.

- `<summary_point>`

## Seat and opponent model

| Field | Analysis | Rule IDs | Notes |
|---|---|---|---|
| supported seat range | `<min..max seats>` | `<rule_ids>` | `<notes>` |
| number of opponents | `<one opponent / multiple opponents / opposing teams / table competitors>` | `<rule_ids>` | `<notes>` |
| partnership/team roles | none / `<team, partnership, coalition, asymmetric role>` | `<rule_ids>` | `<notes>` |
| seat/turn-order pressure | `<first/last/leader/dealer/reaction/simultaneous implications>` | `<rule_ids>` | `<notes>` |

## Phases and situations

| Phase/situation | What competent players notice | Important rule IDs | Notes |
|---|---|---|---|
| `<phase_or_situation>` | `<observations>` | `<rule_ids>` | `<notes>` |

## Immediate tactics

| Tactic | Situation | Why it matters | Rule IDs | Bot feature candidate? |
|---|---|---|---|---:|
| `<tactic>` | `<situation>` | `<reason>` | `<rule_ids>` | yes/no |

Examples: immediate win, immediate block, forced rule compliance, obvious material/point gain, avoiding immediate terminal loss.

## Threats to block

| Threat | How a player detects it from visible information | Blocking response candidates | Rule IDs | Hidden-info risk |
|---|---|---|---|---|
| `<threat>` | `<visible_signal>` | `<responses>` | `<rule_ids>` | none / low / medium / high |

## Positional, resource, card, and tempo principles

Fill only relevant rows. Use explicit `not applicable` where needed.

| Principle type | Principle | Visible evidence | Rule IDs | Notes |
|---|---|---|---|---|
| positional | `<principle>` / not applicable | `<evidence>` | `<rule_ids>` | `<notes>` |
| resource/accounting | `<principle>` / not applicable | `<evidence>` | `<rule_ids>` | `<notes>` |
| card/hand/deck | `<principle>` / not applicable | `<evidence>` | `<rule_ids>` | `<notes>` |
| tempo/initiative | `<principle>` / not applicable | `<evidence>` | `<rule_ids>` | `<notes>` |
| risk/control | `<principle>` / not applicable | `<evidence>` | `<rule_ids>` | `<notes>` |

## Common beginner mistakes

| Mistake | Why it is bad | How competent play avoids it | Rule IDs | Bot test implied? |
|---|---|---|---|---:|
| `<mistake>` | `<reason>` | `<avoidance>` | `<rule_ids>` | yes/no |

## Risk posture

| Situation | Cautious posture | Aggressive posture | Recommended default | Notes |
|---|---|---|---|---|
| `<situation>` | `<behavior>` | `<behavior>` | cautious / balanced / aggressive | `<notes>` |

## Visible signals

| Signal | Visible to whom | Strategic meaning | Bot feature candidate | Notes |
|---|---|---|---:|---|
| `<signal>` | all / acting seat / owning seat / public observer | `<meaning>` | yes/no | `<notes>` |

## Public table inference allowed

| Public signal | Inference a competent player may make | Opponent set affected | Bot feature candidate? | Rule IDs |
|---|---|---|---:|---|
| `<signal>` | `<inference from public information only>` | `<one opponent / all opponents / team / table>` | yes/no | `<rule_ids>` |

## Hidden/private information boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| `<information>` | yes/no | yes/no | yes/no | none / low / medium / high | `<notes>` |

Competent inference is allowed only from legal information: public information, the player's own private information, remembered observations from legal views, and rule/variant knowledge. Actual hidden state, future random outcomes, unrevealed deck order, secret roles, opponent private hands, hidden commitments, and private logs are forbidden.

## Private inference forbidden

| Tempting shortcut | Why forbidden | Required bot guard/test | Notes |
|---|---|---|---|
| `<shortcut such as reading opponent hand, hidden role, deck order, private log, hidden commitment>` | unavailable to the acting seat | `<test>` | `<notes>` |

## Inference allowed vs forbidden peeking

| Scenario | Allowed inference | Forbidden shortcut | Test implied |
|---|---|---|---|
| `<scenario>` | `<allowed>` | `<forbidden>` | `<test>` |

## Kingmaking and coalition risk

Required for 3+ seat, team, partnership, or coalition-sensitive games. For games where this cannot occur, add one explicit `not applicable` row.

| Risk | Visible trigger | Competent response principle | Bot feature candidate? | Rule IDs | Notes |
|---|---|---|---:|---|---|
| `<kingmaking/coalition/table-leader/rival-selection risk or not applicable>` | `<public signal>` | `<principle based only on legal information>` | yes/no | `<rule_ids>` | `<notes>` |

## Strategy examples

| Example ID | Situation | Candidate choices | Competent choice | Explanation | Rule IDs |
|---|---|---|---|---|---|
| `S-EX-001` | `<situation>` | `<choices>` | `<choice>` | `<explanation>` | `<rule_ids>` |

## Anti-examples

| Anti-example ID | Bad choice | Why it is bad | Rule IDs | Bot guard/test implied |
|---|---|---|---|---|
| `S-BAD-001` | `<choice>` | `<reason>` | `<rule_ids>` | `<test>` |

## Known hard problems

| Problem | Why hard | Out-of-scope for current bot? | Notes |
|---|---|---:|---|
| `<problem>` | `<reason>` | yes/no | `<notes>` |

## Out-of-scope advanced strategy

| Strategy idea | Why out of scope | Future trigger |
|---|---|---|
| `<advanced_strategy>` | `<reason>` | `<trigger>` |

## Translation to candidate Level 2 bot features

These are candidates for `BOT-STRATEGY-EVIDENCE-PACK.md`; they are not yet a
policy. Exact bot input view, deterministic ranking, tie-breaks, benchmark IDs,
and implementation tests belong in the bot evidence pack and `GAME-EVIDENCE.md`.

| Candidate feature | Derived from | Visible to bot? | Used for | Hidden-info risk | Test implied |
|---|---|---:|---|---|---|
| `<feature>` | `<section/example>` | yes/no | candidate extraction / priority / tie-break / explanation / style profile | none / low / medium / high | `<test>` |

## Tests implied by strategy claims

Use this section for strategy-claim test implications only. Bot implementation
test status and benchmark workload IDs are recorded in `GAME-EVIDENCE.md` and
the bot evidence pack.

| Strategy claim | Rule IDs | Test type | Test name placeholder | Notes |
|---|---|---|---|---|
| `<claim>` | `<rule_ids>` | unit / named rule / bot decision / simulation / no-leak / explanation / benchmark | `<test_name>` | `<notes>` |

## Review checklist

- All strategy prose is original.
- Sources are recorded and not copied.
- Strategy claims are checked against `GAME-RULES.md`.
- Hidden-information boundaries are explicit.
- Allowed inference and forbidden peeking are separated.
- Examples and anti-examples are concrete enough to test.
- Candidate bot features are evidence, not a hidden implementation plan.
- This document is linked from the Level 2 bot evidence pack if Level 2 is pursued.
