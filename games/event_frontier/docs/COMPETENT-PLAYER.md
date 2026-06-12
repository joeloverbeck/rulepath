# Event Frontier Competent Player Analysis

Game ID: `event_frontier`

Implemented variant: `event_frontier_standard`

Rules version checked: `event-frontier-rules-v1`

Prepared by: Codex

Date: 2026-06-12

## Purpose and authority

This document records original strategy analysis for the implemented Event
Frontier standard variant. It is not rule authority; `RULES.md` wins if any
strategy statement conflicts with a stable rule ID.

The current public bots are Level 1 scripted policies. This analysis documents
what competent play means for those policies and records the balance retune used
to bring Level 1 versus Level 1 games inside the Gate 14 Assumption A5 band.

## Sources and cross-checks

| Source/reference | Path/reference | Date consulted | Used for | Notes |
|---|---|---|---|---|
| Rules authority | `games/event_frontier/docs/RULES.md` | 2026-06-12 | stable rule IDs and legal constraints | Strategy claims checked against rules. |
| Bot implementation | `games/event_frontier/src/bots.rs` | 2026-06-12 | Level 1 decision table | Documentation mirrors implemented priority order. |
| Bot tests | `games/event_frontier/tests/bots.rs` | 2026-06-12 | conformance and no-leak evidence | Verifies legality, determinism, table examples, and explanation redaction. |
| Balance probe | temporary 1,000-game Level 1 versus Level 1 harness, seeds `0..999` | 2026-06-12 | balance and victory-type mix | Same command loop as the golden trace bot-vs-bot harness: public view, legal action tree, validation, and `bot_seed(seed, action_index)`. |

## Competent-player summary

Competent Event Frontier play is about timing public card opportunities while
staying honest about the next Reckoning. The Charter wants broad site presence:
agents and depots count for site majority, and the Charter can end the game at a
Reckoning by holding the variant's site threshold. The Freeholders want enough
public caches for their instant condition, but settlers still matter because
caches do not score sites.

Both factions should preserve resources for meaningful operation values, pass
when low on resources, and prefer actions that remain legal through the Rust
action tree. No competent player or bot may use unrevealed deck order.

## Phases and situations

| Phase/situation | What competent players notice | Rule IDs |
|---|---|---|
| First choice on a favorable event | Taking the event uses the current card's public typed effect and makes the faction ineligible for the next card. | `EF-TURN-003`, `EF-CARD-001` through event-specific IDs |
| First or second operation choice | Operation value, resources, edicts, and site preconditions define the legal path set. | `EF-ACT-006`, `EF-OP-001` through `EF-OP-009` |
| Reckoning approaching | Instant victory is checked before scoring, income, and reset. | `EF-END-001`, `EF-END-002`, `EF-SCORE-004` |
| Low resources | Passing gains one resource and preserves eligibility. | `EF-SCORE-002` |
| Terminal result | No further actions are legal and hidden deck order remains hidden. | `EF-END-005` |

## Immediate tactics

| Tactic | Situation | Why it matters | Rule IDs | Bot feature candidate? |
|---|---|---|---|---:|
| Charter writes exposed caches | Public cache cluster sits at an agent-occupied site. | Removes Freeholder instant-victory progress and gains a fund. | `EF-OP-006`, `EF-END-002` | yes |
| Charter surveys contested sites | Charter can legally place an agent adjacent to presence. | Builds toward the site-majority instant condition and improves scoring. | `EF-OP-004`, `EF-END-001`, `EF-SCORE-004` | yes |
| Charter fortifies held contested sites | A site has enough agents and no depot. | Depot adds Charter presence and can matter at Reckoning. | `EF-OP-005`, `EF-SCORE-004` | yes |
| Freeholders cache at safe sites | A settler-occupied, depot-free site is below cache cap. | Builds toward Freeholder instant victory. | `EF-OP-008`, `EF-END-002` | yes |
| Freeholders trek toward exposed caches | Cache sites are threatened by Charter agents. | Settler escorts improve site presence and keep cache plans resilient. | `EF-OP-007`, `EF-SCORE-004` | yes |
| Freeholders rally or spread settlers | Charter majorities are visible. | Breaks site scoring and final fallback pressure. | `EF-OP-009`, `EF-SCORE-004` | yes |

## Threats to block

| Threat | Visible signal | Blocking response candidates | Rule IDs | Hidden-info risk |
|---|---|---|---|---|
| Charter instant win | Public victory distance says Charter is close to its site threshold. | Freeholder spread, trek, or rally to break majorities. | `EF-END-001`, `EF-SCORE-004` | none |
| Freeholder instant win | Public victory distance says Freeholders are close to the cache threshold. | Charter writ at the largest legal cache cluster. | `EF-END-002`, `EF-OP-006` | none |
| Bad resource tempo | Acting faction has one or fewer resources and low-impact legal operations. | Pass to gain one resource and preserve eligibility. | `EF-SCORE-002` | none |
| Edict-affected operation | Active edict changes legal costs or blocked sites. | Choose another legal operation or pass. | `EF-EDICT-*`, `EF-OP-*` | none |

## Principles

| Principle type | Principle | Visible evidence | Rule IDs |
|---|---|---|---|
| positional | Charter values breadth; Freeholders value settler presence near caches and contested sites. | Public site component counts and victory-distance view. | `EF-SCORE-004`, `EF-END-001`, `EF-END-002` |
| resource/accounting | Operation plans should account for current resource pools, pass income, and edict costs. | Public resource pools and action-tree metadata. | `EF-SCORE-001`, `EF-SCORE-002`, `EF-SCORE-003` |
| card/deck | Current and next public cards are usable; deeper deck order is forbidden. | Public current and next card only. | `EF-COMP-011`, `EF-VIS-001` |
| tempo/initiative | Taking event or operation gives power now but can forfeit next-card eligibility. | Public eligibility markers. | `EF-TURN-003`, `EF-TURN-004`, `EF-COMP-015` |
| risk/control | Do not overfocus on one terminal path: final fallback still scores settler and Charter presence. | Public scores and Reckoning count. | `EF-END-004`, `EF-SCORE-006` |

## Hidden-information boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Notes |
|---|---:|---:|---:|---|
| Current card | yes | yes | yes | Public. |
| Next public card | yes | yes | yes | Public by setup/projection. |
| Undrawn deck order beyond next public card | no | no | no | Hidden from all viewers and bots. |
| Public map components, resources, scores, edicts, eligibility | yes | yes | yes | One public projection is shared by observer and seats. |
| Future random outcomes | no | no | no | No hidden-state sampling, search, ML, RL, or Monte Carlo policy. |

## Balance retune

The initial standard constants missed Assumption A5 in a 1,000-game Level 1
versus Level 1 probe: Charter 865 wins, Freeholders 135 wins, Charter instant
212, Freeholder instant 0, final fallback 788.

The standard scenario was retuned with local scenario constants only:
`standard_starting_resources = "2,4"`,
`standard_freeholder_cache_threshold = 6`,
`standard_start_settlers = "site_landing:3,site_high_meadow:1"`, and
`standard_start_caches = "site_landing:1,site_high_meadow:1"`.

The post-retune 1,000-game probe used seeds `0..999`, Level 1 bots for both
factions, action cap 96, and deterministic bot seed
`seed * 0x9e3779b97f4a7c15 + action_index`. Result:

| Metric | Count | Percent |
|---|---:|---:|
| Charter wins | 602 | 60.2% |
| Freeholder wins | 398 | 39.8% |
| Charter instant victories | 110 | 11.0% of games |
| Freeholder instant victories | 22 | 2.2% of games |
| Final fallback victories | 868 | 86.8% of games |
| Capped nonterminal games | 0 | 0.0% |

This is inside the 35-65% per-faction band and includes all three victory
types.

## Known hard problems

| Problem | Why hard | Out of scope for current bot? | Notes |
|---|---|---:|---|
| Reckoning timing | Reckonings are seeded but hidden beyond current/next public cards. | yes | Bot uses only public current/next cards. |
| Multi-card tempo planning | Eligibility choices affect later cards. | yes | Level 1 uses immediate public-distance and resource rules. |
| Full strategic search | Would invite MCTS/Monte Carlo/ML/RL pressure. | yes | Public v1/v2 bots stay scripted and explainable. |

## Tests implied by strategy claims

| Strategy claim | Test type | Evidence |
|---|---|---|
| Bots choose legal actions over many seeds. | bot decision | `cargo test -p event_frontier --test bots` |
| Level 1 bots are deterministic for same public inputs. | bot decision | `level1_bots_are_deterministic_for_same_public_inputs` |
| Favorable event and operation-family decisions match the table. | bot decision | `favorable_public_events_match_the_decision_table`, `faction_level1_policies_rank_distinct_operation_families` |
| Bot inputs and explanations do not expose hidden deck order. | no-leak | `bot_inputs_and_explanations_do_not_expose_undrawn_deck_order` |
| Retuned balance fits Assumption A5 and all victory types occur. | simulation | 1,000-game Level 1 probe recorded above. |
