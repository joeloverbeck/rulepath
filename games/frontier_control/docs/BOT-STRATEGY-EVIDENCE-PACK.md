# Frontier Control Bot Strategy Evidence Pack

Game ID: `frontier_control`

Implemented variant: `frontier_control_standard` and `frontier_control_highlands`

Rules version: `frontier-control-rules-v1`

Bot target: Level 1 rule-informed baseline policies

Policy name/version: `frontier_control_garrison_level1_v1`,
`frontier_control_prospector_level1_v1`

Prepared by: `Codex`

Date: 2026-06-11

## Purpose and gate

This evidence pack documents the implemented Frontier Control public bots. It
is intentionally scoped to Level 0 random legal and Level 1 rule-informed
baseline play. It does not authorize or claim a Level 2 authored-policy bot.

The Level 1 policies must be deterministic under rules version, variant,
public view, legal action tree, policy version, and declared seed. They must use
the Rust legal action API and validate through the normal command path. Their
explanations must be viewer-safe public prose.

## Explicit public v1/v2 exclusions

Frontier Control public bots do not use:

- omniscient state;
- hidden-state shortcuts;
- future random outcomes;
- unbounded weight soup;
- static data tactical conditions;
- random blunder injection by default;
- public v1/v2 MCTS;
- public v1/v2 ISMCTS;
- public v1/v2 Monte Carlo-style bots;
- public v1/v2 ML;
- public v1/v2 RL;
- runtime LLM move selection.

Future search, ML, RL, or runtime LLM work requires foundation-doc review and
an ADR where required.

## Source documents consumed

| Document/source | Path/reference | Required? | Status | Notes |
|---|---|---:|---|---|
| `RULES.md` | `games/frontier_control/docs/RULES.md` | yes | read | Stable rule IDs and bot boundary. |
| `COMPETENT-PLAYER.md` | `games/frontier_control/docs/COMPETENT-PLAYER.md` | yes | read | Strategy and balance posture for Level 1 docs. |
| Bot implementation | `games/frontier_control/src/bots.rs` | yes | read | Authoritative policy order for current Level 1 bots. |
| Bot tests | `games/frontier_control/tests/bots.rs` | yes | read | Legality, validation-path, and explanation smoke evidence. |
| Property tests | `games/frontier_control/tests/property.rs` | yes | read | Full Level-1-vs-Level-1 terminal smoke evidence. |
| Golden bot trace | `games/frontier_control/tests/golden_traces/bot-vs-bot-full-game.trace.json` | yes | read | Records bot-vs-bot trace intent. |
| Benchmarks | `games/frontier_control/docs/BENCHMARKS.md`, `games/frontier_control/benches/frontier_control.rs` | yes | read | Level 1 decision latency operations exist. |

## Exact bot input view

| Input item | Included? | Source | Visible to acting seat? | Notes/no-leak test |
|---|---:|---|---:|---|
| legal action tree/action paths | yes | Rust `legal_action_tree` | yes | `validate_bot_decision` submits the selected path through normal validation. |
| public view | yes | Rust `project_view` | yes | Frontier Control is perfect-information; every viewer gets the same public projection. |
| acting seat private view | no | not applicable | not applicable | No private game state exists. |
| command/effect history visible to seat | no direct history model | public effects/replay | yes | Level 1 policies are memoryless over current public view and legal tree. |
| policy seed/tie-break state | declared but not materially used by Level 1 policies | bot constructor | not game info | Current priority order is deterministic without stochastic tie-breaks. |
| hidden opponent/private state | no | forbidden | no | Not present in this game. |
| unrevealed deck/order/future random outcomes | no | forbidden | no | Not present in this game. |
| dev/test full state | no for public policy input | testing may inspect after action | no | Tests may set up states but public policy uses view plus legal tree. |

## Legal action API used

| API/contract | Purpose | Determinism requirement | Tests |
|---|---|---|---|
| `legal_action_tree(state, actor)` | Enumerates candidates for the active faction seat. | Same state, actor, rules, and variant produce the same legal leaves. | `bots_select_legal_paths_for_both_factions`, `random_and_level1_bots_select_legal_public_actions` |
| `validate_command(state, command)` via `validate_bot_decision` | Proves the selected path is legal through the same validation path as human input. | Validation rejects stale, wrong-seat, malformed, and unavailable paths without mutation. | `level1_garrison_policy_is_legal_and_faction_named` |
| `command_for_decision` | Converts a bot decision into a normal command envelope with current freshness token. | The command path preserves actor, action path, rules version, and freshness token. | `level1_garrison_policy_is_legal_and_faction_named` |

## Candidate extraction plan

The implemented Level 1 bots do not build a separate opaque candidate model.
They read single-segment legal leaves from the Rust action tree, parse only
game-local action families, and rank legal leaves with public view facts.

| Candidate group | Extraction rule | Rule IDs | Visible facts used | Hidden-info risk | Tests |
|---|---|---|---|---|---|
| Garrison dismantle | legal `dismantle/<site>` leaves | `FC-ACT-008` | public stakes, public guard location, public stake value | none | bot unit tests |
| Garrison patrol | legal `patrol/<from>/<to>` leaves | `FC-ACT-006`, `FC-CTRL-003` | public crews, supplied stakes, site order | none | bot unit/property tests |
| Garrison reinforce | legal `reinforce/<fort>` leaves | `FC-ACT-007`, `FC-SCORE-GARRISON-FORT` | public fort state and cap legality from tree | none | bot unit tests |
| Prospector stake | legal `stake/<site>` leaves | `FC-ACT-003`, `FC-SCORE-STAKE-VALUE` | public stake values and legal stake leaves | none | bot unit/property tests |
| Prospector march | legal `march/<from>/<to>` leaves | `FC-ACT-002`, `FC-CTRL-002` | public stake values, guard presence, site order | none | bot unit/property tests |
| Prospector muster | legal `muster` leaf | `FC-ACT-004` | action-tree legality | none | bot unit/property tests |
| End turn fallback | legal `end_turn` leaf | `FC-ACT-009` | action-tree legality | none | bot unit/property tests |

## Phase model

| Phase/situation | Detection from allowed input | Policy node(s) active | Rule IDs | Notes |
|---|---|---|---|---|
| Prospector action phase | public active faction and Prospector legal leaves | stake, march, muster, end turn | `FC-TURN-001`, `FC-ACT-001` | Prospectors act before the Garrison scoring response. |
| Garrison action phase | public active faction and Garrison legal leaves | dismantle, patrol, reinforce, end turn | `FC-TURN-002`, `FC-ACT-005` | Garrison gets the final action phase before round scoring. |
| Waiting or terminal | empty legal tree | no bot gameplay action | `FC-TURN-003`, `FC-ACT-011` | Public bot returns no legal action if called outside active seat context. |

## Implemented lexicographic priority vectors

The current Level 1 bots use hard priority order plus stable site-order
tie-breaks. Earlier rows dominate later rows.

### Garrison Level 1

| Slot | Priority | Higher/better value | Source evidence | Rule IDs | Tests | Explanation fragment |
|---:|---|---|---|---|---|---|
| 1 | dismantle public stake | highest public stake value among legal dismantles | `src/bots.rs::best_dismantle_target` | `FC-ACT-008`, `FC-CTRL-005` | bot legality tests | "dismantled the public stake" |
| 2 | patrol toward public pressure | destination with more crews, then supplied stake value, then stable site order | `src/bots.rs::best_patrol_target` | `FC-ACT-006`, `FC-CTRL-003` | bot/property tests | "contest public crew or supply pressure" |
| 3 | reinforce first legal fort | first legal reinforce target in stable site order | `src/bots.rs::first_site_action` | `FC-ACT-007`, `FC-SCORE-GARRISON-FORT` | bot legality tests | "hold a public fort" |
| 4 | end turn fallback | legal `end_turn` exists | `src/bots.rs::fallback_end_turn` | `FC-ACT-009` | terminal smoke | "no public legal action improved the position" |

### Prospector Level 1

| Slot | Priority | Higher/better value | Source evidence | Rule IDs | Tests | Explanation fragment |
|---:|---|---|---|---|---|---|
| 1 | stake public value | highest public stake value among legal stakes | `src/bots.rs::best_stake_target` | `FC-ACT-003`, `FC-SCORE-STAKE-VALUE` | bot legality tests | "highest-value public legal site" |
| 2 | march toward value/opening | destination with higher unstaked value, then guard presence, then stable site order | `src/bots.rs::best_march_target` | `FC-ACT-002`, `FC-CTRL-002` | bot/property tests | "toward public stake value or an opening" |
| 3 | muster | legal `muster` exists | `src/bots.rs::select_decision` | `FC-ACT-004` | bot/property tests | "crews are below the useful expansion threshold" |
| 4 | end turn fallback | legal `end_turn` exists | `src/bots.rs::fallback_end_turn` | `FC-ACT-009` | terminal smoke | "no public legal action improved the position" |

## Bounded scoring tie-breakers

The Level 1 policies use small tuple tie-breaks, not a weight soup.

| Score term | Range | Meaning | Used after slots | Visible inputs | Tests | Explanation text |
|---|---:|---|---|---|---|---|
| public stake value | `0..3` in current variants | Prefer richer legal stake/dismantle targets. | stake and dismantle priorities | public variant value projected in view | bot tests, doc review | "highest-value public legal site" |
| crew count at patrol destination | `0..3` by unit cap | Prefer patrols that contest visible crew pressure. | Garrison patrol priority | public crew count | bot tests | "contest public crew" |
| supplied stake value at patrol destination | `0..3` in current variants | Prefer patrols that can interfere with current public supplied stakes. | Garrison patrol priority | Rust-projected supplied flag and public value | visibility/bot tests | "supply pressure" |
| unstaked destination value | `0..3` in current variants | Prefer marches toward unclaimed scoring value. | Prospector march priority | public stake value and stake marker | bot tests | "public stake value" |
| guard presence at march destination | `0..1` | Prefer a legal opening or useful guard trade after stake value. | Prospector march priority | public guard count | bot tests | "opening" |
| stable site order | `0..6` | Deterministic tie-break across otherwise equal legal targets. | all site target priorities | fixed `SiteId::ALL` order | determinism tests | not surfaced unless needed |

## Deterministic seeded tie-break

| Item | Decision |
|---|---|
| seed source | `Seed` in bot constructor; random legal bot uses it directly. Current Level 1 policies are deterministic under stable legal tree and public view. |
| tie-break input identity | action family, source site, destination site, target site, and fixed `SiteId::ALL` order. |
| stable ordering rule | tuple comparison with reverse site index for max-by stable target selection, or first legal site in stable order for reinforce. |
| reproducibility tests | `bots_are_deterministic_under_declared_inputs`; Level-1 terminal smoke. |
| replay/hash interaction | Bot choices become normal command envelopes; replay determinism is over the resulting command stream. |

Random blunder injection is not used.

## Style profile hooks

| Profile | Variation | Must not affect | Hidden-info safe? | Tests |
|---|---|---|---:|---|
| default | The only implemented Level 1 profile for each faction. | legality, public-view input boundary, deterministic action selection, validation path | yes | bot and property tests |

No alternate difficulty/personality profile is implemented for this gate.

## Forbidden hidden information

| Information | Why forbidden | Potential leak surface | Required no-leak test |
|---|---|---|---|
| hidden/private state | Public bots must use player-available information only. | input view, explanations, candidate ranking, replay export | visibility and serialization tests |
| future opponent choices | Not known to the acting bot. | explanation, ranking, simulation shortcuts | explanation review |
| future random outcomes or deck order | Not present in Frontier Control. | not applicable | not applicable markers in traces |
| full-state shortcuts that bypass public view/legal tree | Would violate bot law even in a perfect-information game. | bot input and dev diagnostics | bot validation-path tests |

## Memory and belief model

| Item | Decision | Allowed inputs | Forbidden inputs | Tests |
|---|---|---|---|---|
| memory model | none | current public view and legal tree | hidden actual state, private logs | bot tests |
| belief model | none | not applicable for perfect information | sampled hidden state, future outcomes | visibility tests |
| redaction model | one public projection | public facts only | hidden facts or dev-only state | visibility and replay tests |

## Explanation contract

Every non-random Level 1 decision returns:

| Field | Required? | Notes |
|---|---:|---|
| policy name/version | yes | `BotDecision.policy_id` and `policy_version`. |
| chosen priority reason | yes | Rationale maps to the selected priority branch. |
| relevant visible fact | yes | Site/faction/action names are public. |
| tie-break note | optional | Current explanations do not expose tuple internals. |
| hidden-info disclaimer | no | Frontier Control has no hidden information; docs still forbid hidden shortcuts. |
| fallback/search note | if fallback | End-turn fallback says no public legal action improved the position. |

## Public explanation examples

| Situation | Chosen action | Public explanation | Hidden-info safe? | Rule IDs |
|---|---|---|---:|---|
| Guard occupies a staked Goldfield and dismantle is legal. | `dismantle/site_goldfield` | "Garrison dismantled the public stake at Goldfield to deny Prospector scoring." | yes | `FC-ACT-008` |
| Garrison patrols from Gatehouse to Ford. | `patrol/site_gatehouse/site_ford` | "Garrison patrolled from Gatehouse to Ford to contest public crew or supply pressure." | yes | `FC-ACT-006` |
| Prospectors can stake Goldfield. | `stake/site_goldfield` | "Prospectors staked Goldfield because it is the highest-value public legal site." | yes | `FC-ACT-003` |
| Prospectors can muster and no higher priority applies. | `muster` | "Prospectors mustered because public crews are below the useful expansion threshold." | yes | `FC-ACT-004` |

## Dev-mode ranking examples

Dev mode may show candidate rankings only when viewer-safe. Frontier Control has
no hidden information, but ranking still must avoid full-state debug shortcuts
that bypass the public view.

| Situation | Candidate ranking excerpt | Redactions needed? | Hidden-info safe? | Notes |
|---|---|---:|---:|---|
| Garrison has two legal dismantles. | `dismantle` candidates sorted by public stake value, then stable site order. | no | yes | Candidate list is derived from legal leaves. |
| Prospectors have several legal stakes. | `stake` candidates sorted by public stake value, then stable site order. | no | yes | Do not include future Garrison replies. |

## Decision examples and expected choices

| Example ID | Situation | Candidate choices | Expected choice | Priority vector reason | Rule IDs | Test name |
|---|---|---|---|---|---|---|
| `FC-BOT-EX-001` | Garrison has legal dismantle and patrol choices. | `dismantle/<site>`, `patrol/<from>/<to>`, `reinforce/<fort>`, `end_turn` | highest-value legal `dismantle/<site>` | Garrison slot 1 dominates patrol/reinforce. | `FC-ACT-008` | existing bot tests plus future targeted fixture |
| `FC-BOT-EX-002` | Prospectors have legal stake choices. | multiple `stake/<site>` leaves | highest-value legal `stake/<site>` | Prospector slot 1. | `FC-ACT-003`, `FC-SCORE-STAKE-VALUE` | existing bot tests plus future targeted fixture |
| `FC-BOT-EX-003` | No non-fallback priority is legal. | `end_turn` | `end_turn` | fallback keeps the bot legal and avoids action-tree stalls. | `FC-ACT-009` | terminal smoke |

## Known weaknesses

| Weakness | Why acceptable for Level 1 | Mitigation | Future trigger |
|---|---|---|---|
| Current policies do not use score-race urgency. | Level 1 only needs obvious public tactics and legal baseline play. | Documented as a known weakness and balance-retune surface. | Public polish or Level 2 work. |
| Garrison currently prioritizes dismantle before patrol. | It is simple, explainable, and legal, but may overfit immediate stake removal. | Simulation exposed a standard-map balance miss. | Public polish or Level 2 retune work. |
| Prospectors currently stake before path preservation. | It creates visible scoring pressure but can allow easy Garrison denial. | Tune map constants or priority order before claiming balanced public play. | Public polish or Level 2 retune work. |
| Level 1 seed is not materially used by deterministic priority branches. | Stable deterministic behavior is acceptable for Level 1. | Random legal bot covers seeded random baseline; Level 1 tie-breaks remain stable. | Need for varied public difficulty profiles. |

## Balance evidence and retune posture

Assumption A5 targets a 35-65% per-faction Level-1-vs-Level-1 win band on the
standard map. The registered CLI measurement command is:

```sh
cargo run -p simulate -- --game frontier_control --games 1000
```

The current evidence is:

| Evidence | Command/source | Result | A5 interpretation |
|---|---|---|---|
| Level 1 legality and explanation smoke | `cargo test -p frontier_control bots` | both factions select legal public actions and produce faction-named rationales | required policy evidence, not a win-rate measure |
| Level-1-vs-Level-1 terminal smoke | `cargo test -p frontier_control level1_bot_sequence_reaches_terminal_without_illegal_actions` | standard-map bot sequence reaches terminal without illegal actions | proves completion, not balance |
| Registered simulation on 2026-06-11 | `cargo run -p simulate -- --game frontier_control --games 1000` | Garrison wins 1000-0; average score 16-0; average rounds 8; average length 32 | outside the A5 band; retune required before claiming balanced public play |
| Temporary local probe on 2026-06-11 | current Rust Level 1 policies over one deterministic highlands playout | Prospectors win 15-3 | cross-map signal only; A5 names standard map |

This pack therefore records a retune note instead of asserting balance. Before
public polish, adjust typed constants or Level 1 priority order and rerun the
measurement until the standard-map Level-1-vs-Level-1 result has an acceptable
retune decision.

## Test plan

| Test area | Required? | Specific tests | Evidence |
|---|---:|---|---|
| legality | yes | `bots_select_legal_paths_for_both_factions`, `random_and_level1_bots_select_legal_public_actions`, `level1_garrison_policy_is_legal_and_faction_named` | existing bot tests |
| determinism | yes | `bots_are_deterministic_under_declared_inputs` | existing bot unit test |
| no hidden-state access | yes, as boundary smoke | visibility/replay tests and one-public-view trace markers | existing visibility and golden trace coverage |
| candidate extraction | yes | current bot unit tests plus manual review of `src/bots.rs` | this evidence pack maps candidate groups to code |
| priority vector | yes | manual review and existing bot tests | future targeted fixtures can make each priority branch explicit |
| bounded scoring | yes | tuple tie-break doc review, no large weights | current code uses small tuple keys only |
| seeded tie-break | yes | deterministic bot test and random legal bot smoke | seed is declared; Level 1 branches are stable |
| explanations | yes | bot tests assert faction-named rationale; examples documented here | existing bot tests |
| simulation/fuzz | yes | `cargo run -p simulate -- --game frontier_control --games 1000` | registered; current 1000-game standard-map run records a retune note |
| replay/hash | yes | bot golden trace and replay support tests | existing trace/replay coverage |
| benchmark | yes | `cargo bench -p frontier_control` | benchmark operations include both Level 1 decision functions |

## Latency and benchmark budget

| Operation | Target/budget | Measurement command | Baseline | Notes |
|---|---:|---|---:|---|
| Garrison Level 1 decision | smoke floor: above 1 decision/second | `cargo bench -p frontier_control -- garrison_level1_bot_decision` | pending calibration | Native benchmark operation exists. |
| Prospector Level 1 decision | smoke floor: above 1 decision/second | `cargo bench -p frontier_control -- prospector_level1_bot_decision` | pending calibration | Native benchmark operation exists. |
| Random playout | smoke floor: above 1 playout/second | `cargo bench -p frontier_control -- random_playout` | pending calibration | Useful for broad terminal smoke, not A5 balance. |

## Public UX note

Public UI may show the latest Level 1 bot rationale as short viewer-safe text
beside the effect log. It should name the public site/action reason, not expose
debug tuple scores or future-looking claims.

## Review checklist

- `COMPETENT-PLAYER.md` was consumed.
- Legal action API and validation path are exact.
- Bot input view is explicit.
- No omniscient state, hidden-state shortcuts, or future random outcomes are used.
- Candidate extraction uses legal action paths and allowed views.
- Priority vectors are lexicographic and faction-specific.
- Bounded scores are small, named, documented, and public.
- Tie-breaks are deterministic under stable candidate identity.
- Hidden-information boundaries are stated even though the game is perfect-information.
- Public v1/v2 MCTS, ISMCTS, Monte Carlo bots, ML, RL, and runtime LLM move selection are absent.
- Simulation balance is recorded as pending CLI refresh with an explicit retune note.
- Public UX note is concise and product-facing.
