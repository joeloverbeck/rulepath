# Frontier Control Rules

Game ID: `frontier_control`

Public display name: `Frontier Control`

Implemented variant: `frontier_control_standard` and `frontier_control_highlands`

Rules version: `frontier-control-rules-v1`

Prepared by: `Codex`

Created: 2026-06-11

Last updated: 2026-06-11

## Rule authority

This document is the original Rulepath rules summary for the implemented
variants. Sources and IP notes belong in `SOURCES.md`; this document states the
Rulepath implementation contract.

Stable rule IDs are requirements. They must remain stable after implementation
unless intentionally migrated with a migration note and corresponding updates in
`RULE-COVERAGE.md`, traces, tests, and docs.

Rust owns setup, graph validation, legal action generation, adjacency checks,
validation, budget tracking, movement, clash resolution, stake/control state,
supply connectivity, scoring, terminal detection, semantic effects, public
projection, replay/export behavior, and bot decisions. TypeScript may present
only Rust/WASM output.

## Metadata

| Field | Value |
|---|---|
| game id | `frontier_control` |
| public display name | `Frontier Control` |
| variants | `frontier_control_standard`, `frontier_control_highlands` |
| rules version | `frontier-control-rules-v1` |
| source note | `games/frontier_control/docs/SOURCES.md` |
| coverage matrix | `games/frontier_control/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/frontier_control/docs/MECHANICS.md` |
| implementation admission | `games/frontier_control/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

Frontier Control is a two-seat original Rulepath competitive area-control game.
One seat commands the Garrison, which tries to hold frontier forts. The other
seat commands the Prospectors, which try to establish supplied stakes across a
small trail graph. The map is a graph of named sites and edges, not a
rectangular board.

The game proves graph topology, site control, deterministic clashes, asymmetric
faction action sets, faction-specific scoring formulas, budgeted turns, public
semantic effects, per-faction bots, and a browser presentation driven by Rust
views and effects. It does not implement hidden information, game-rule
randomness, event decks, reaction windows, generic graph helpers, generic
faction helpers, hosted multiplayer, timers, or asymmetric victory conditions.

## Implemented variants

| Rule ID | Variant rule | Source/rationale link |
|---|---|---|
| `FC-VAR-001` | `frontier_control_standard` is the default public variant. It uses a seven-site frontier map, ten trails, two action points per turn, and eight scoring rounds. | `SOURCES.md#variant-choice-and-deviations` |
| `FC-VAR-002` | `frontier_control_highlands` uses the same Rust rules with different typed map constants, starts, site values, and round count. | `SOURCES.md#variant-choice-and-deviations` |
| `FC-VAR-003` | Variant data may declare site IDs, labels, edge pairs, fort flags, stake values, start units, caps, budgets, round counts, and faction labels. Variant data must not declare movement rules, clash outcomes, scoring formulas, legal conditions, triggers, selectors, bot policy, or terminal logic. | Rulepath static-data boundary |

## Components and game-local vocabulary

Game nouns in this section belong to `games/frontier_control` only. They do not
authorize graph, map, site, edge, adjacency, faction, unit, guard, crew, stake,
fort, clash, supply, control, or movement vocabulary in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `FC-COMP-001` | seat | One of the two players, `seat_0` or `seat_1`. | public | The standard faction assignment is deterministic. |
| `FC-COMP-002` | faction | The public side controlled by a seat: Garrison or Prospectors. | public | Factions have disjoint action vocabularies. |
| `FC-COMP-003` | site | A named public place on the frontier graph. | public | Standard sites are Gatehouse, Signal Hill, Base Camp, Ford, Quarry, Timberline, and Goldfield. |
| `FC-COMP-004` | trail | An undirected edge connecting two sites. | public | Adjacency constrains movement and supply paths. |
| `FC-COMP-005` | guard | A Garrison unit occupying a site. | public | Guards patrol, reinforce forts, and can dismantle stakes. |
| `FC-COMP-006` | crew | a Prospector unit occupying a site. | public | Crews march, stake sites, and can be mustered at camp. |
| `FC-COMP-007` | fort | A site marked as a Garrison scoring site. | public | A fort scores for the Garrison when held. |
| `FC-COMP-008` | stake | A Prospector marker on a stakeable site. | public | A supplied stake scores for the Prospectors. |
| `FC-COMP-009` | clash | A deterministic result when a unit enters a site containing opposing units. | public | Guard-entry and crew-entry clashes resolve differently. |
| `FC-COMP-010` | supply | A Rust-computed path from a staked site to Base Camp through sites with no guards. | public after projection | The browser displays supplied/cut status but never computes it. |
| `FC-COMP-011` | action budget | The active faction's remaining action points for the turn. | public | The legal tree regenerates after every action. |
| `FC-COMP-012` | score track | Public cumulative scores for the two factions. | public | Both factions score on one comparable numeric track. |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `FC-SETUP-001` | Create exactly two seats. `seat_0` controls the Garrison and `seat_1` controls the Prospectors unless a future typed variant says otherwise. | deterministic | public | Other seat counts are out of scope. |
| `FC-SETUP-002` | Load the selected typed map constants: sites, labels, edge pairs, fort flags, stake values, start units, unit caps, action budget, and round count. | deterministic | public | Unknown fields and behavior-looking fields are rejected. |
| `FC-SETUP-003` | Validate the map before play: every edge endpoint exists, duplicate sites and duplicate edges are rejected, start units refer to known sites, and the graph is connected. | deterministic | public diagnostics | Invalid content fails closed at setup. |
| `FC-SETUP-004` | Place starting guards and crews from the variant constants. In the standard variant, two guards start at each Garrison fort and three crews start at Base Camp. | deterministic | public | No game-rule randomness is used. |
| `FC-SETUP-005` | Initialize round 1, Prospectors active, full action budget, zero scores, no terminal outcome, and a fresh command token. | deterministic | public | Same variant and command stream reproduce the match. |

## Turn, round, and phase sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `FC-TURN-001` | Prospector action phase. | Prospector seat only | The Prospectors spend budget one valid action at a time: march, stake, muster, or end turn. | A valid action is applied. |
| `FC-TURN-002` | Garrison action phase. | Garrison seat only | The Garrison spends budget one valid action at a time: patrol, reinforce, dismantle, or end turn. | A valid action is applied. |
| `FC-TURN-003` | Waiting state. | non-active seat has no gameplay action | Rust supplies an empty tree with safe waiting metadata. | The active faction ends the turn or spends the final budget point. |
| `FC-TURN-004` | Budget exhaustion. | active seat | If the active faction spends the final budget point on a non-terminal action, Rust ends the turn immediately. | The final budget point is applied. |
| `FC-TURN-005` | Explicit end turn. | active seat | The active faction may forfeit remaining budget. | `end_turn` validates. |
| `FC-TURN-006` | Round scoring. | none as command actor | After the Garrison turn ends, Rust scores the round and emits a public scoring breakdown. | The scoring batch completes. |
| `FC-TURN-007` | Non-terminal cleanup. | next active seat | Rust advances to the next faction or next round, refills the budget, and projects the new legal tree. | The next action phase begins. |
| `FC-TURN-008` | Terminal state. | none | Rust exposes the final result and no normal gameplay actions. | No further gameplay action advances the match. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality,
adjacency, connectivity, clash results, scores, terminal outcome, or bot policy.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `FC-ACT-001` | Active Prospectors during action phase. | `march/<from>/<to>`, `stake/<site>`, `muster`, and `end_turn` as available. | flat leaves with budget metadata | The action tree contains only currently legal leaves. |
| `FC-ACT-002` | Prospector march. | One march leaf for each crew-occupied source and adjacent destination that can receive the movement or resolve a clash. | `march/<from>/<to>` | Rust checks source crews, trail adjacency, unit caps, and clash consequences. |
| `FC-ACT-003` | Prospector stake. | One stake leaf for each crew-occupied, guard-free, stakeable site with no existing stake. | `stake/<site>` | Forts, camp, zero-value sites, guarded sites, and already staked sites are illegal. |
| `FC-ACT-004` | Prospector muster. | `muster` while Base Camp has room for another crew and no guard occupies it. | single leaf | Adds one crew at Base Camp up to the unit cap. |
| `FC-ACT-005` | Active Garrison during action phase. | `patrol/<from>/<to>`, `reinforce/<fort>`, `dismantle/<site>`, and `end_turn` as available. | flat leaves with budget metadata | The action tree contains only currently legal leaves. |
| `FC-ACT-006` | Garrison patrol. | One patrol leaf for each guard-occupied source and adjacent destination that can receive the movement or resolve a clash. | `patrol/<from>/<to>` | Rust checks source guards, trail adjacency, unit caps, and clash consequences. |
| `FC-ACT-007` | Garrison reinforce. | One reinforce leaf for each held fort below the guard cap. | `reinforce/<fort>` | A fort is held when it has at least one guard and no crews. |
| `FC-ACT-008` | Garrison dismantle. | One dismantle leaf for each staked site with at least one guard. | `dismantle/<site>` | Removes the stake only; units remain. |
| `FC-ACT-009` | End-turn choice. | `end_turn` is always legal during an action phase. | single leaf | Prevents action-tree stalls. |
| `FC-ACT-010` | Non-active seat or observer. | none | empty gameplay tree | Waiting metadata may name the active faction and public phase only. |
| `FC-ACT-011` | Terminal state. | none | empty gameplay tree | Terminal states expose no normal gameplay actions. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `FC-RESTRICT-001` | Unknown or non-seat actor submits a gameplay action. | Reject without mutation. | Viewer-safe wrong-actor diagnostic. | Only the two seats can act. |
| `FC-RESTRICT-002` | The wrong seat submits while another faction is active. | Reject without mutation. | Viewer-safe wrong-seat diagnostic. | Diagnostic may name only public active-faction facts. |
| `FC-RESTRICT-003` | A malformed or unavailable action path is submitted. | Reject without mutation. | Viewer-safe unavailable-action diagnostic. | No state mutation. |
| `FC-RESTRICT-004` | A stale command is submitted after state has advanced. | Reject without mutation. | Viewer-safe stale-command diagnostic. | Replay/hash state must not change. |
| `FC-RESTRICT-005` | A move action uses non-adjacent sites or an unknown site. | Reject without mutation. | Viewer-safe trail/adjacency diagnostic. | Does not compute in TypeScript. |
| `FC-RESTRICT-006` | A move action tries to move a missing unit or exceed a site cap after legal clash resolution. | Reject without mutation. | Viewer-safe occupancy diagnostic. | The diagnostic contains only public occupancy facts. |
| `FC-RESTRICT-007` | A stake, muster, reinforce, or dismantle action targets a state where its Rust-owned preconditions do not hold. | Reject without mutation. | Viewer-safe action-specific diagnostic. | No hidden state exists. |
| `FC-RESTRICT-008` | A gameplay action is submitted in terminal state. | Reject without mutation. | Viewer-safe terminal-state diagnostic. | Outcome is final. |

## Movement, clashes, and control

| Rule ID | Rule | Timing | Effect order | Notes |
|---|---|---|---|---|
| `FC-CTRL-001` | A unit may move only along a trail from its current site to an adjacent site. | action application | movement effect before any clash effect | Trail adjacency is Rust behavior over typed map content. |
| `FC-CTRL-002` | When a crew enters a guarded site, one guard is removed and the entering crew is also removed. | Prospector march application | movement/clash effects before budget effect | Crews trade themselves for guards. |
| `FC-CTRL-003` | When a guard enters a crewed site, one crew is removed and the entering guard survives. | Garrison patrol application | movement/clash effects before budget effect | Guards are stronger when entering a clash. |
| `FC-CTRL-004` | A site may contain both factions only transiently during clash resolution. The projected settled state is public and deterministic. | after move application | clash resolution before view projection | Later implementation may model this as atomic mutation. |
| `FC-CTRL-005` | A stake remains until a Garrison dismantle action removes it. A cut stake scores zero for that round but is not automatically removed. | action and scoring | stake/dismantle effects before scoring effects | Stakes are public markers. |

## Scoring and accounting

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `FC-SCORE-GARRISON-FORT` | The Garrison gains 1 point for each fort that has at least one guard and zero crews. | after each Garrison turn | A fort with any crew scores zero for the Garrison that round. | Round-scoring effects carry the held/lost fort breakdown. |
| `FC-SCORE-PROSPECTOR-SUPPLY` | The Prospectors gain a staked site's stake value when Rust can trace a path from that site to Base Camp through sites with no guards. | after each Garrison turn | A staked site with no guard-free path to Base Camp scores zero that round and keeps its stake. | Round-scoring effects carry supplied/cut status. |
| `FC-SCORE-ACTION-BUDGET` | The action budget is public and decreases by 1 for each march, stake, muster, patrol, reinforce, or dismantle action. | action phase | `end_turn` may forfeit remaining budget. | The budget refills at non-terminal turn cleanup. |
| `FC-SCORE-STAKE-VALUE` | Stake values are public typed map constants. Forts, Base Camp, and zero-value sites are not stakeable. | setup and scoring | Invalid stake-value content fails setup validation. | Values are content; scoring formula is Rust behavior. |
| `FC-SCORE-COMPARABLE-TRACK` | Both factions score onto one comparable numeric track. | after every scoring round and terminal | Higher total wins after the final round. | Differing victory conditions are out of scope. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `FC-TERM-SCORE-COMPARE` | The final scheduled scoring round completes. | The faction with the higher total score wins. | If scores differ, no tiebreaker is used. | Terminal rationale names the final scores and decisive scoring source. |
| `FC-TERM-GARRISON-TIEBREAK` | The final scheduled scoring round completes with equal scores. | The Garrison wins. | Garrison is the incumbent tiebreak winner. | Terminal rationale must cite this rule ID. |
| `FC-TERM-NO-ACTIONS` | Terminal state is reached. | No further gameplay actions are legal. | not applicable | Terminal does not change map state after the terminal effect. |

## Outcome explanation traceability

Every scoring and terminal rule that can decide a match has a stable rule ID and
enough detail for the web outcome surface to cite it.

| Outcome/explanation fact | Stable rule ID(s) | Decisive when | Notes for UI explanation |
|---|---|---|---|
| Garrison fort score | `FC-SCORE-GARRISON-FORT` | Fort-holding points provide or contribute to the winning total. | Cite held/lost forts from Rust terminal data. |
| Prospector supplied-stake score | `FC-SCORE-PROSPECTOR-SUPPLY`, `FC-SCORE-STAKE-VALUE` | Supplied stakes provide or contribute to the winning total. | Cite supplied/cut stakes from Rust terminal data; do not recompute paths in TypeScript. |
| Higher final score | `FC-TERM-SCORE-COMPARE`, `FC-SCORE-COMPARABLE-TRACK` | Final totals differ after the final scoring round. | Cite both totals and the Rust-projected winner. |
| Garrison tiebreak | `FC-TERM-GARRISON-TIEBREAK` | Final totals are equal after the final scoring round. | State that the Garrison wins the incumbent tiebreak. |
| Terminal no-action state | `FC-TERM-NO-ACTIONS` | Any terminal result. | No normal action controls remain. |

This table is traceability only. It is not a behavior DSL, selector table, or
TypeScript decision source. Rust remains the source of scoring, terminal
detection, and rationale projection.

## Visibility and private information

Frontier Control is perfect-information. Public/browser payloads still must not
invent hidden state or carry full internal debug state through public surfaces.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `FC-VIS-001` | Seats, factions, active faction, round, budget, sites, trails, units, forts, stakes, scores, supplied/cut status, and terminal result. | observer and both seat viewers | after setup and after each projection | public view, seat view, action tree metadata, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, dev inspector | These are safe public facts. |
| `FC-VIS-002` | Variant map constants and site labels. | all viewers | after setup | public docs, view metadata, UI labels, replay export | Content is public and behavior-free. |
| `FC-VIS-003` | Bot rationale and candidate rankings. | only if projected by Rust as viewer-safe public data | after bot decision or diagnostics | bot explanation, dev inspector, logs, replay export | Rationale may cite public graph, units, stakes, scores, and legal tree only. |
| `FC-VIS-004` | Hidden information. | not applicable | never | all browser-facing surfaces | The game has no hidden units, secret objectives, hidden random order, or private state. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `FC-RNG-001` | Frontier Control uses no game-rule randomness. | Same rules version, variant, map data, and command stream must reproduce internal state and effects. | public | Bot tie-break RNG is bot infrastructure, not game-rule randomness. |
| `FC-RNG-002` | Map setup is deterministic typed content. | Replay and fixture checks must fail on invalid or changed map content without migration notes. | public | No setup shuffle or random sample exists. |
| `FC-RNG-003` | Round scoring is a deterministic consequence of the Garrison turn-ending or final-budget command. | Replay must not require a separate scoring actor or TypeScript timer. | public | The command stream remains seat-action only. |
| `FC-RNG-004` | Serialization order must remain stable. | Golden traces and replay checks must fail on unintended ordering changes. | public | Stable order covers sites, edges, units, actions, effects, views, scores, and terminal rationale. |

## Bot-relevant non-authoritative strategy notes

These notes describe intended product behavior, not extra legal authority.
Implemented bots must choose from the Rust legal tree and validate through the
normal action path.

| Rule ID | Strategy note | Allowed input | Forbidden input |
|---|---|---|---|
| `FC-BOT-001` | A random-legal bot may select any legal leaf from its current action tree with deterministic tie-breaking. | legal action tree and bot RNG stream | direct state mutation or illegal fallback |
| `FC-BOT-002` | A Garrison Level 1 bot should protect held forts, dismantle supplied stakes, and patrol to cut high-value supply paths using only public view facts. | public view, legal action tree, public graph projection, public units/stakes/scores | TS-computed connectivity, search, MCTS, ISMCTS, Monte Carlo, ML, or RL |
| `FC-BOT-003` | A Prospector Level 1 bot should create and preserve supplied stakes, muster when crews are scarce, and trade crews into guard-heavy sites when it advances scoring. | public view, legal action tree, public graph projection, public units/stakes/scores | hidden information, direct state mutation, or bypassing validation |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `FC-AMB-001` | Whether graph connectivity should be a shared helper. | No. Graph topology and connectivity scoring stay local to `games/frontier_control`. | Gate 13 primitive-pressure ledger | setup validation tests, scoring tests, atlas rows | This is the first official graph-map use. |
| `FC-AMB-002` | Whether TypeScript may compute supplied/cut highlighting from edges. | No. Rust projects supplied/cut status; TypeScript only renders it. | behavior-authority rule | web smoke, manual review | Connectivity affects scoring. |
| `FC-AMB-003` | Whether clash resolution opens a response window. | No. Clashes resolve immediately inside the mover's command. | Gate 13 reaction-window review | action/effect tests, trace | No pending response exists. |
| `FC-AMB-004` | Whether tied final scores are a draw. | No. The Garrison wins tied final scores. | original incumbent-tiebreak design | terminal tests, outcome explanation smoke | Cite `FC-TERM-GARRISON-TIEBREAK`. |
| `FC-AMB-005` | Whether map data can encode action or scoring rules. | No. Map data declares content only; Rust defines movement, clash, and scoring behavior. | Rulepath static-data boundary | strict-parse tests, fixture-check | No selectors, triggers, formulas, or scripts. |

## Rulepath deviations from adjacent area-control games

| Rule ID | Adjacent pattern | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `FC-DEV-001` | Published area-control games often use distinctive faction names, map regions, influence cubes, conquest tracks, or woodland/empire vocabulary. | Frontier Control uses original frontier-site labels, guards, crews, stakes, forts, and neutral graph presentation. | Avoid source confusion and trade-dress imitation. | yes |
| `FC-DEV-002` | Some asymmetric games use different victory conditions per faction. | Frontier Control uses different scoring formulas but one comparable score track and one final score comparison. | Gate 14 owns asymmetric victory conditions. | yes |
| `FC-DEV-003` | Some games use cards, dice, event decks, or hidden objectives. | Frontier Control uses no hidden information and no game-rule randomness. | Keep Gate 13 focused on graph topology and action/scoring asymmetry. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `FC-OOS-001` | Three-seat, four-seat, team, solo, or more-than-two-faction variants. | Two factions prove the gate's asymmetry surface. | Later product polish only. |
| `FC-OOS-002` | Hidden units, fog of war, secret objectives, secret faction powers, or private hands. | Perfect information is a deliberate Gate 13 scope choice. | Future hidden-role/objective gate only with no-leak review. |
| `FC-OOS-003` | Game-rule randomness, dice, shuffled decks, or random events. | Gate 13 isolates graph/asymmetry proof. | Gate 14 event work or later chance gate. |
| `FC-OOS-004` | Reaction windows, interrupts, or pending responses. | Clashes are immediate Rust consequences. | If a second reaction-capable game is proposed. |
| `FC-OOS-005` | Generic graph, control, faction, or pathfinding helpers in `engine-core` or `game-stdlib`. | First uses stay local and second-use rows are keep-local comparisons. | Revisit per mechanic atlas triggers. |
| `FC-OOS-006` | Hosted multiplayer, accounts, matchmaking, chat, ranked play, persistence, real-time timers, or undo. | V1/v2 are static/local-first. | Future ADR only. |
| `FC-OOS-007` | Public MCTS, ISMCTS, Monte Carlo, ML, RL, LLM, or hidden-state-sampling bots. | Public bot law forbids them. | Future ADR only. |

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every rule ID in this document must appear in `RULE-COVERAGE.md`. Silent gaps
are not allowed.
