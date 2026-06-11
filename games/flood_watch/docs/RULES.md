# Flood Watch Rules

Game ID: `flood_watch`

Public display name: `Flood Watch`

Implemented variant: `flood_watch_standard` and `flood_watch_deluge`

Rules version: `flood-watch-rules-v1`

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

Rust owns setup, deterministic event-deck order, legal action generation,
validation, budget tracking, event resolution, shared terminal detection,
semantic effects, public projection, replay/export behavior, and bot decisions.
TypeScript may present only Rust/WASM output.

## Metadata

| Field | Value |
|---|---|
| game id | `flood_watch` |
| public display name | `Flood Watch` |
| variants | `flood_watch_standard`, `flood_watch_deluge` |
| rules version | `flood-watch-rules-v1` |
| source note | `games/flood_watch/docs/SOURCES.md` |
| coverage matrix | `games/flood_watch/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/flood_watch/docs/MECHANICS.md` |
| implementation admission | `games/flood_watch/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

Flood Watch is a two-seat original Rulepath cooperative event-pressure game.
The team protects five river-town districts through one storm. On each turn, one
seat spends a small action budget to bail flood water, raise levees, or forecast
the next event. After the acting seat ends the turn or spends the last action,
the environment resolves a deterministic batch of event cards from a shuffled
deck.

Both seats win together if the event deck is exhausted without an inundation.
Both seats lose together as soon as any district reaches the inundated level.
There are no per-seat scores, no solo winner, and no traitor objective.

The game proves shared win/loss, hidden-from-all event-deck order, deterministic
environment automation, asymmetric role powers, multi-action turn budgets,
typed scenario setup, viewer-safe replay/export, and cooperative bots. It does
not implement a generic cooperative engine, event-deck engine, role-power
system, action-budget system, scenario engine, graph map, flood-spread model,
hosted multiplayer, timers, or reaction windows.

## Implemented variants

| Rule ID | Variant rule | Source/rationale link |
|---|---|---|
| `FW-VAR-001` | `flood_watch_standard` is the default public variant. It uses five districts, two seats, a three-action turn budget, two event draws per environment phase, flood levels `0..=3`, and levee stacks capped at 2. | `SOURCES.md#variant-choice-and-deviations` |
| `FW-VAR-002` | `flood_watch_deluge` is the harder scenario. It keeps the same action rules and terminal rules while increasing starting pressure and event composition according to typed scenario constants. | `SOURCES.md#variant-choice-and-deviations` |
| `FW-VAR-003` | Scenario data may declare labels, starting flood levels, levee cap, action budget, draws per phase, and counts for closed Rust event kinds. Scenario data must not declare event effects, legal conditions, triggers, formulas, bot policy, or terminal logic. | Rulepath static-data boundary |

## Components and game-local vocabulary

Game nouns in this section belong to `games/flood_watch` only. They do not
authorize `event`, `deck`, `card`, `role`, `scenario`, `district`, `flood`,
`levee`, `budget`, `environment`, `cooperative`, or shared-outcome vocabulary
in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `FW-COMP-001` | seat | One of the two cooperative players, `seat_0` or `seat_1`. | public | Both seats share the same terminal result. |
| `FW-COMP-002` | role | A public deterministic action modifier assigned by scenario setup. | public | `seat_0` starts as Pumpwright; `seat_1` starts as Levee Warden unless a scenario later states otherwise. |
| `FW-COMP-003` | district | One of five public areas: Riverside, Old Docks, Market, Terraces, and Gardens. | public | Districts are flat named targets; they have no adjacency, graph, routes, or movement. |
| `FW-COMP-004` | flood level | A public counter from 0 to 3 on each district. | public | Level 3 is inundated and causes shared loss. |
| `FW-COMP-005` | levee stack | A public prevention counter on each district. | public | Levees absorb incoming rise before flood level increases. |
| `FW-COMP-006` | event deck | A deterministically shuffled stack of typed event cards. | hidden until reveal/draw | The undrawn order is hidden from every browser-facing viewer and bot. |
| `FW-COMP-007` | event card | A closed Rust event kind: Downpour for a district, Storm Surge for a district, or Reprieve. | visible only when forecast or drawn | What each kind does is Rust behavior, not static data behavior. |
| `FW-COMP-008` | forecast marker | A public reveal of the current top event card without drawing it. | public after forecast | Forecast reveals the top card to both seats and observers. |
| `FW-COMP-009` | action budget | The acting seat's remaining action points for the turn. | public | The legal tree regenerates after every action. |
| `FW-COMP-010` | environment phase | The deterministic event-resolution batch after a turn ends. | public through effects | It is automation inside Rust application, not a command-stream actor. |
| `FW-COMP-011` | shared outcome | The team's terminal result: won or lost. | public at terminal | There are no individual winners or tiebreaks. |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `FW-SETUP-001` | Create exactly two seats, `seat_0` and `seat_1`. | deterministic | public | Other seat counts are out of scope. |
| `FW-SETUP-002` | Load the selected typed scenario constants: district labels, starting flood levels, action budget, draws per environment phase, levee cap, and event-kind counts. | deterministic | public except deck order | Unknown fields and behavior-looking fields are rejected. |
| `FW-SETUP-003` | Assign public roles from scenario data: Pumpwright and Levee Warden in the standard setup. | deterministic | public | Role powers affect Rust action application only. |
| `FW-SETUP-004` | Build the event deck from closed Rust event kinds in stable scenario-declared counts, then shuffle it with Rulepath's deterministic seeded RNG discipline. | seeded deterministic | internal until forecast/draw | The authored scenario never stores shuffled order. |
| `FW-SETUP-005` | Initialize districts, empty forecast marker, turn 1, active seat `seat_0`, full action budget, no terminal outcome, and a fresh command token. | deterministic | public | Same seed, variant, rules version, and command stream reproduce the match. |

## Turn, round, and phase sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `FW-TURN-001` | Action phase. | active seat only | The active seat spends budget one valid action at a time: bail, reinforce, forecast, or end turn. | A valid action is applied. |
| `FW-TURN-002` | Teammate waiting state. | non-active teammate has no gameplay action | Rust supplies an empty tree with safe waiting metadata. | The active seat ends the turn or spends the final budget point. |
| `FW-TURN-003` | Budget exhaustion. | active seat | If the active seat spends the final budget point on a non-terminal action, Rust immediately resolves the environment phase. | The final budget point is applied. |
| `FW-TURN-004` | Explicit end turn. | active seat | The active seat may forfeit remaining budget and proceed to environment resolution. | `end_turn` validates. |
| `FW-TURN-005` | Environment phase. | none as command actor | Rust draws and resolves the scenario's event count in order, emitting public semantic effects for every step. | The batch completes, the deck exhausts, or a district is inundated. |
| `FW-TURN-006` | Non-terminal cleanup. | next active seat | Rust clears the forecast marker if its card was drawn, alternates the active seat, refills the budget, increments the turn, and projects the new legal tree. | The next action phase begins. |
| `FW-TURN-007` | Terminal state. | none | Rust exposes the shared outcome and no normal gameplay actions. | No further gameplay action advances the match. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality, budget
state, role magnitude, environment resolution, event ordering, terminal outcome,
hidden-info filtering, or bot policy.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `FW-ACT-001` | Active seat during action phase. | `bail/<district>`, `reinforce/<district>`, `forecast`, and `end_turn` as available. | flat leaves with budget metadata | The action tree contains only currently legal leaves. |
| `FW-ACT-002` | Bail choice. | One bail leaf for each district with flood level at least 1. | `bail/<district>` | Pumpwright removes up to 2 levels; other roles remove up to 1; never below 0. |
| `FW-ACT-003` | Reinforce choice. | One reinforce leaf for each district below the levee cap. | `reinforce/<district>` | Levee Warden places up to 2 levees; other roles place up to 1; never above cap. |
| `FW-ACT-004` | Forecast choice. | `forecast` only while the top event card has not already been forecast. | single leaf | Reveals the top event card publicly without drawing it. |
| `FW-ACT-005` | End-turn choice. | `end_turn` is always legal during action phase. | single leaf | Prevents action-tree stalls. |
| `FW-ACT-006` | Non-active teammate or observer. | none | empty gameplay tree | Waiting metadata may name the active seat and public phase only. |
| `FW-ACT-007` | Terminal state. | none | empty gameplay tree | Terminal states expose no normal gameplay actions. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `FW-RESTRICT-001` | Unknown or non-seat actor submits a gameplay action. | Reject without mutation. | Viewer-safe wrong-actor diagnostic. | Only the two seats can act. |
| `FW-RESTRICT-002` | The wrong seat submits while a teammate is active. | Reject without mutation. | Viewer-safe wrong-seat diagnostic. | Diagnostic may name only public active-seat facts. |
| `FW-RESTRICT-003` | A malformed or unavailable action path is submitted. | Reject without mutation. | Viewer-safe unavailable-action diagnostic. | No state mutation. |
| `FW-RESTRICT-004` | A stale command is submitted after state has advanced. | Reject without mutation. | Viewer-safe stale-command diagnostic. | Replay/hash state must not change. |
| `FW-RESTRICT-005` | A bail action targets a dry or unknown district. | Reject without mutation. | Viewer-safe district/bail diagnostic. | Does not mention event-deck order. |
| `FW-RESTRICT-006` | A reinforce action targets an unknown or full-levee district. | Reject without mutation. | Viewer-safe district/levee diagnostic. | Does not mention event-deck order. |
| `FW-RESTRICT-007` | Forecast is submitted while the top card is already forecast or the deck is empty. | Reject without mutation. | Viewer-safe forecast diagnostic. | Does not identify any unrevealed card. |
| `FW-RESTRICT-008` | A gameplay action is submitted in terminal state. | Reject without mutation. | Viewer-safe terminal-state diagnostic. | Outcome is final. |

## Environment resolution

| Rule ID | Environment rule | Timing | Effect order | Notes |
|---|---|---|---|---|
| `FW-ENV-001` | The environment phase resolves as part of applying the turn-ending command or the final budget-spending command. | after action phase | `TurnEnded` then `EnvironmentPhaseBegan` before draws | No synthetic environment actor appears in the command stream. |
| `FW-ENV-002` | Draw up to the scenario's event count from the top of the event deck, one at a time. | environment phase | `EventDrawn` before that card's consequences | A forecast card becomes public no later than its draw effect. |
| `FW-ENV-003` | A Downpour raises its district by 1 after levee absorption. | event resolution | `LeveeAbsorbed` before `FloodLevelRose` when both apply | District target is public once the card is drawn or forecast. |
| `FW-ENV-004` | A Storm Surge raises its district by 2 after levee absorption. | event resolution | absorption before rise | Multiple levees may absorb a multi-point rise. |
| `FW-ENV-005` | A Reprieve changes no district level or levee count. | event resolution | `EventDrawn` only, plus safe calm copy if emitted | It still consumes one event card. |
| `FW-ENV-006` | If a district reaches level 3, Rust emits the inundation and terminal effects immediately and stops the remaining environment draws. | mid-environment phase | draw, absorption/rise, inundation, terminal | Undrawn card identities remain hidden after loss. |
| `FW-ENV-007` | If the deck becomes empty after resolving a card and no loss has occurred, Rust emits deck-exhaustion and terminal-win effects. | environment phase | `DeckExhausted` before terminal win | The team wins together. |

## Scoring and accounting

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `FW-SCORE-001` | Flood levels are public district counters from 0 through 3. | setup, actions, event resolution | level 3 causes shared loss | There are no points. |
| `FW-SCORE-002` | Levee stacks are public prevention counters from 0 through the scenario cap. | setup, actions, event resolution | levees absorb before levels rise | Excess prevention above cap is ignored by validation. |
| `FW-SCORE-003` | The action budget is public and decreases by 1 for each bail, reinforce, or forecast action. | action phase | `end_turn` may forfeit remaining budget | The budget refills at non-terminal turn cleanup. |
| `FW-SCORE-004` | Event-deck remaining composition is public as counts derived from scenario composition minus drawn cards. | after setup and each draw | card order remains hidden | Counts may be shown to players and bots; order may not. |
| `FW-SCORE-005` | The match has no individual score, no seat ranking, and no tiebreaker. | terminal | not applicable | Terminal result is shared. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `FW-END-001` | Any district reaches flood level 3. | shared team loss | none | Terminal rationale may name the inundated district and public levels. |
| `FW-END-002` | The final event card resolves without any district reaching flood level 3. | shared team win | none | Terminal rationale may cite deck exhaustion and surviving district levels. |
| `FW-END-003` | Terminal state is reached. | no further gameplay actions | none | Terminal does not reveal undrawn cards or the original deck order. |

## Outcome explanation traceability

Every terminal rule that can decide a match has a stable rule ID and enough
detail for the web outcome surface to cite it.

| Outcome/explanation fact | Stable rule ID(s) | Decisive when | Notes for UI explanation |
|---|---|---|---|
| Shared loss by inundation | `FW-END-001`, `FW-SCORE-001`, `FW-ENV-006` | A district reaches level 3 during setup validation or event resolution. | Cite the district and its public level; do not reveal any undrawn event. |
| Shared win by deck exhaustion | `FW-END-002`, `FW-ENV-007` | The last card resolves without an inundation. | Cite deck exhaustion and public surviving district levels. |
| No per-seat winner | `FW-END-001`, `FW-END-002`, `FW-SCORE-005` | Any terminal result. | Copy must describe the team result, not a winner seat. |

This table is traceability only. It is not a behavior DSL, selector table, or
TypeScript decision source. Rust remains the source of terminal detection and
rationale projection.

## Visibility and private information

Public/browser payloads must not reveal hidden information through public views,
seat views, action trees, previews, diagnostics, effect logs, DOM attributes,
test IDs, logs, local storage, replay exports, bot explanations, candidate
rankings, or dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `FW-VIS-001` | Seats, roles, active seat, turn, action budget, district levels, levee stacks, drawn events, forecast card, public remaining-composition counts, and terminal result. | observer and both seat viewers | after setup and after each projection, subject to reveal timing | public view, seat view, action tree metadata, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, dev inspector | These are safe public facts. |
| `FW-VIS-002` | Undrawn event-deck order and identities below the forecasted top card. | no browser-facing viewer and no bot | never before draw/forecast | public view, seat view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | Native internal tests may inspect under test authority only. |
| `FW-VIS-003` | Forecast top card. | all viewers | after a valid forecast until drawn or terminal projection says it is no longer active | public view, effects, replay export, DOM | Forecast is a public reveal, not a private peek. |
| `FW-VIS-004` | Drawn event cards. | all viewers | from `EventDrawn` effect onward | public view, effects, replay export, DOM | Drawn cards remain public history. |
| `FW-VIS-005` | Bot rationale and candidate rankings. | only if projected by Rust as viewer-safe public data | after bot decision or diagnostics | bot explanation, dev inspector, logs, replay export | Rationale may cite public levels, levees, forecast, remaining-composition counts, and legal tree only. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `FW-RNG-001` | Flood Watch uses deterministic seeded shuffle for setup and no random event sampling after setup. | Same seed, rules version, variant, scenario data, and command stream must reproduce internal state and effects. | internal for hidden deck order | Shuffle implementation follows the primitive-pressure ledger decision. |
| `FW-RNG-002` | Event draws are deterministic consequences of command application. | Replay must not require a separate environment command actor or TypeScript timer. | public only as effects reveal cards | The environment phase is replayed from the same seat command stream. |
| `FW-RNG-003` | Public replay export is viewer-scoped and redacted. | Public exports must not include seed material or undrawn deck order that reconstructs future cards. | public export is redacted | Drawn and forecast cards may appear from their reveal point onward. |
| `FW-RNG-004` | Serialization order must remain stable. | Golden traces and replay checks must fail on unintended ordering changes. | mixed | Stable order covers districts, event history, actions, effects, views, counters, and terminal rationale. |

## Bot-relevant non-authoritative strategy notes

These notes describe intended product behavior, not extra legal authority.
Implemented bots must choose from the Rust legal tree and validate through the
normal action path.

| Rule ID | Strategy note | Allowed input | Forbidden input |
|---|---|---|---|
| `FW-BOT-001` | A random-legal bot may select any legal leaf from its current action tree with deterministic tie-breaking. | legal action tree and bot RNG stream | direct state mutation or illegal fallback |
| `FW-BOT-002` | A Level 1 bot should rescue imminent shared losses first, then reinforce forecast threats, then use public expected-threat counting, then forecast with spare budget. | public view, legal action tree, public forecast, remaining-composition counts, public district levels/levees, own public role | undrawn deck order, hidden-state sampling, MCTS, ISMCTS, Monte Carlo, ML, or RL |
| `FW-BOT-003` | A cooperative teammate bot may play either public role and either seat. | same viewer-safe public facts available to a human teammate | any hidden card order or unauthorized internal state |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `FW-AMB-001` | Whether the environment acts as a synthetic actor. | No. Environment resolution is an atomic Rust consequence of the turn-ending or final-budget command. | Gate 12 replay contract | replay trace, command-log test | Preserves existing command-stream semantics. |
| `FW-AMB-002` | Whether terminal loss reveals the rest of the deck. | No. Undrawn cards remain hidden after terminal loss or win. | hidden-info no-leak posture | terminal no-leak trace, replay export test, browser no-leak smoke | Drawn and forecast cards remain public history. |
| `FW-AMB-003` | Whether forecast is private to the acting seat. | No. Forecast reveals the top card publicly to both seats and observers. | cooperative public-state design | visibility test, effect-log test | There is no per-seat private information. |
| `FW-AMB-004` | Whether districts have adjacency or spreading floods. | No. Districts are flat targets. | Gate 13 owns graph pressure | rule coverage, setup validation | No movement, routes, graph, or spread. |
| `FW-AMB-005` | Whether scenario data can encode event behavior. | No. Data declares counts and constants only; Rust defines each closed event kind's behavior. | Rulepath static-data boundary | strict-parse tests, fixture-check | No selectors, triggers, formulas, or scripts. |

## Rulepath deviations from adjacent cooperative/event games

| Rule ID | Adjacent pattern | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `FW-DEV-001` | Named commercial cooperative games often use distinctive islands, disease maps, deserts, spirits, fires, role cards, or crisis decks. | Flood Watch uses original river-town labels, flat districts, and neutral event names. | Avoid source confusion and trade-dress imitation. | yes |
| `FW-DEV-002` | Some cooperative games use adjacency, movement, spreading, outbreaks, epidemics, reshuffles, or escalating decks. | Flood Watch excludes adjacency, movement, spread, outbreak/epidemic vocabulary, and re-shuffle escalation. | Keep Gate 12 focused and leave graph pressure for Gate 13. | yes |
| `FW-DEV-003` | Some games reveal all hidden state after the game ends. | Flood Watch never reveals undrawn event order in browser-facing surfaces. | Preserve no-leak and redacted export guarantees. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `FW-OOS-001` | Three-seat, four-seat, and solo variants. | Two seats prove shared outcome, role asymmetry, and teammate-bot mode. | Later product polish only. |
| `FW-OOS-002` | Graph topology, adjacency, flood spread, routes, regions, and player movement. | Gate 13 owns graph pressure. | `frontier_control` gate. |
| `FW-OOS-003` | Reaction windows, interrupts, or pending responses. | The environment is automation, not a seat response. | If a second reaction-capable game is proposed. |
| `FW-OOS-004` | Per-seat hidden information, traitors, secret roles, hidden objectives, or private hands. | Gate 12 hides only the event-deck order from everyone. | Later hidden-role/private-objective gate only with no-leak review. |
| `FW-OOS-005` | Hosted multiplayer, accounts, matchmaking, chat, ranked play, persistence, real-time timers, or undo. | V1/v2 are static/local-first. | Future ADR only. |
| `FW-OOS-006` | Public MCTS, ISMCTS, Monte Carlo, ML, RL, LLM, or hidden-state-sampling bots. | Public bot law forbids them. | Future ADR only. |

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every rule ID in this document must appear in `RULE-COVERAGE.md`. Silent gaps
are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| none | not applicable | Initial rule-ID set. | not applicable | 2026-06-11 |
