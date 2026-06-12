# Event Frontier Mechanics Inventory

Game ID: `event_frontier`

Roadmap stage/gate: Gate 14

Rules version: `event-frontier-rules-v1`

Prepared by: `Codex`

Last updated: 2026-06-12

## Purpose

This inventory records the implemented Event Frontier mechanic shapes. It is
evidence for the mechanic atlas and primitive-pressure ledger; it is not
permission to generalize. All game nouns remain local to `games/event_frontier`.

## Mechanic inventory categories

| Category | Game-local description | Evidence in rules | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | Six named sites connected by public trails. | `EF-COMP-003`, `EF-COMP-004`, `EF-SETUP-003` | local-only | Graph-site legality stays local; no `board_space` reuse. |
| component/zone model | Agents, depots, settlers, caches, resources, scores, current/next/discarded cards, and hidden undrawn deck tail. | `EF-COMP-005` through `EF-COMP-016` | local-only | Components are public except deeper deck order. |
| action shape | Progressive Rust action tree for event, operation, limited operation, and pass choices. | `EF-ACT-001` through `EF-ACT-008` | local-only | Operations are compound commands, not budgets. |
| turn/phase model | First eligible choice, constrained second choice, cleanup, automated Reckoning, terminal. | `EF-TURN-001` through `EF-TURN-009` | local-only | Eligibility is initiative state, not a reaction window. |
| randomness/chance | Deterministic setup shuffle into three epochs; no random sampling after setup. | `EF-SETUP-004`, `EF-RNG-001`, `EF-RNG-002` | local-only | Current and next card are public; deeper order is hidden. |
| visibility/hidden information | Observer and seats share one public projection; undrawn deck order beyond next public card is redacted. | `EF-VIS-001` through `EF-VIS-005` | local-only | Hidden-information proof is narrower than private-hand games. |
| resource/accounting | Charter funds, Freeholder provisions, pass income, operation costs, Reckoning income, cumulative scores. | `EF-SCORE-001` through `EF-SCORE-006` | rejected/deferred with rationale | Third-use pressure was reviewed; helper extraction rejected for this gate. |
| movement/capture/placement | Charter placement/depot/cache removal and Freeholder settler movement/cache/rally placement. | `EF-OP-004` through `EF-OP-009` | local-only | No combat or capture resolution. |
| pattern/line/directional scanning | Not used. | `EF-DEV-003` | not applicable | Site majority is counted directly. |
| commitment/reveal | Staged public reveal of current and next event cards; no private commitments. | `EF-VIS-003`, `EF-RNG-003` | local-only | Replay exports preserve redaction. |
| reaction/window/pending response | Not used. | `EF-AMB-004`, `EF-OOS-003` | rejected/deferred with rationale | Eligibility is sequential, not interrupt timing. |
| scoring/outcome | Charter site-majority instant win, Freeholder cache instant win, both-met Freeholder rule, final fallback score/tiebreak. | `EF-END-001` through `EF-END-005` | local-only | Outcome rationale is public and Rust-owned. |
| semantic effect shape | Card reveal, choices, events, edicts, resources, operations, Reckoning, scoring, terminal effects. | `EF-EVENT-*`, `EF-EDICT-*`, `EF-SCORE-*`, `EF-END-*` | local-only | Effects drive UI and replay presentation. |
| UI interaction pattern | Public map plus event card rail, progressive operation construction, edict/Reckoning panels, victory-distance indicators. | `EF-ACT-*`, `EF-VIS-*` | local-only | TypeScript presents Rust/WASM output only. |
| bot policy pattern | Level 0 random legal and Level 1 faction-specific public-view policies. | `EF-BOT-001` through `EF-BOT-003` | local-only | No search, MCTS, ML, or hidden-state sampling. |
| benchmark/performance pressure | Setup shuffle, action-tree generation, peak operation branching, Reckoning, serialization, bot decisions, full playout. | `EF-RNG-*`, `EF-ACT-*`, `EF-BOT-*` | local-only | Smoke thresholds live in `benches/thresholds.json`. |

## Repeated-shape comparison

| Mechanic shape | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---:|---|---|---|
| public resource accounting | `token_bazaar`, `poker_lite` | no | Public counters and payments. | Event Frontier has faction-owned operation funding plus Reckoning income, not markets or pots. | defer/reject recorded in `PRIMITIVE-PRESSURE-LEDGER.md` |
| event pressure | `flood_watch` | no | Deterministic event cadence and semantic effects. | Flood Watch automates pressure; Event Frontier offers player choices over public cards. | keep local |
| graph/control asymmetry | `frontier_control` | partial | Named locations, asymmetric factions, public scoring pressure. | Different action cadence, victory rules, and no connectivity scoring. | keep local |
| hidden card order | `high_card_duel`, `poker_lite`, `plain_tricks` | partial | Redacted card order and no-leak proof. | No private hands; all seats share one projection. | keep local |

## Primitives reused

| Primitive | Source | Why reused | Rule IDs covered | Tests proving compatibility | Notes |
|---|---|---|---|---|---|
| none | not applicable | Event Frontier keeps its mechanics game-local. | not applicable | `boundary-check.sh`, `rule-coverage` | No new `game-stdlib` primitive is promoted. |

## Local mechanics

| Local mechanic | Why local | Extraction risk | Rule IDs | Tests/traces | Notes |
|---|---|---|---|---|---|
| event-card eligibility/initiative | First official use of this exact flow. | medium | `EF-TURN-*`, `EF-ACT-*` | rules tests, golden traces | Boundary check now guards `initiative` and `eligibility` in `engine-core`. |
| edict modifiers | Card-specific typed exceptions are game-specific. | high | `EF-EDICT-*` | edict traces, rules tests | No behavior in card data. |
| asymmetric victory | First official asymmetric instant/fallback combination. | medium | `EF-END-*` | terminal traces | Outcome section documents each cause. |
| public replay redaction for shared projection | Hidden tail only; no private seat views. | medium | `EF-VIS-*`, `EF-RNG-003` | visibility/replay tests | Terminal still does not reveal tail order. |

## Extraction or defer rationale

| Shape | Decision | Rationale | Back-port needed? | Trace impact | Benchmark impact |
|---|---|---|---:|---|---|
| resource accounting | defer/reject | Prior games use different ownership, timing, and settlement semantics. | no | none | local benchmarks only |
| multi-action budgets | reject as non-use | Operations are one compound command, not a regenerated action budget. | no | none | none |
| event-card behavior | local | A generic event engine would invite behavior in data. | no | preserve | local card/apply benchmarks |
| outcome rationale | local | Cause variants are game-specific and viewer-safe. | no | preserve | none |

## Effects, UI, and bot notes

| Area | Requirement | Rule IDs | Notes |
|---|---|---|---|
| semantic effects | Card, choice, operation, edict, resource, Reckoning, and terminal effects must be public-safe. | `EF-EVENT-*`, `EF-EDICT-*`, `EF-SCORE-*`, `EF-END-*` | UI animation and replay read effects, not inferred diffs. |
| UI interaction pattern | Progressive operation controls come from Rust legal trees. | `EF-ACT-*`, `EF-OP-*` | TypeScript never computes legality, cost, or scoring. |
| Rust-generated previews | No separate preview API exists for this gate. | `EF-ACT-*` | Legal tree metadata and public view are the safe presentation inputs. |
| bot policy pattern | Level 0 random legal and Level 1 faction policies use public projection only. | `EF-BOT-*`, `EF-VIS-*` | Candidate rankings stay viewer-safe if shown in dev tooling. |
| visibility/no-leak | Undrawn deck order beyond next card is forbidden in all browser-facing surfaces. | `EF-VIS-002`, `EF-RNG-003` | Includes outcome, replay export, logs, and bot explanations. |
| benchmark pressure | Peak operation branching, Reckoning, bot decisions, full playout. | `EF-ACT-*`, `EF-TURN-*`, `EF-BOT-*` | Thresholds are smoke floors pending calibration. |

## Required repo atlas update

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | already done | Gate 14 primitive-pressure decisions were recorded before implementation. | maintainer |
| `PRIMITIVE-PRESSURE-LEDGER.md` | already done | Resource-accounting third-use and budget non-use decisions are recorded. | maintainer |
| ADR | no | No promoted primitive or foundation migration is introduced. | not applicable |

## Review checklist

- All mechanic atlas categories are filled.
- Repeated shapes are compared against existing official games.
- Third-use public resource-accounting pressure is cleared by the ledger.
- `engine-core` remains noun-free.
- Static data remains typed identity, parameters, metadata, fixtures, traces, and reports only.
- Effects, UI, bot, visibility, and benchmark impacts are recorded.
