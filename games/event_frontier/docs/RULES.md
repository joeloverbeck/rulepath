# Event Frontier Rules

Game ID: `event_frontier`

Public display name: `Event Frontier`

Implemented variant: `event_frontier_standard`, `event_frontier_hard_winter`, and `event_frontier_land_rush`

Rules version: `event-frontier-rules-v1`

Prepared by: `Codex`

Created: 2026-06-12

Last updated: 2026-06-12

## Rule authority

This document is the original Rulepath rules summary for the implemented
variants. Sources and IP notes belong in `SOURCES.md`; this document states the
Rulepath implementation contract.

Stable rule IDs are requirements. They must remain stable after implementation
unless intentionally migrated with a migration note and corresponding updates in
`RULE-COVERAGE.md`, traces, tests, and docs.

Rust owns setup, epoch shuffle, card sequencing, eligibility, legal action
generation, operation validation, event and edict behavior, Reckoning resolution,
asymmetric victory, semantic effects, hidden-order-safe projection,
replay/export behavior, and bot decisions. TypeScript may present only
Rust/WASM output.

## Metadata

| Field | Value |
|---|---|
| game id | `event_frontier` |
| public display name | `Event Frontier` |
| variants | `event_frontier_standard`, `event_frontier_hard_winter`, `event_frontier_land_rush` |
| rules version | `event-frontier-rules-v1` |
| source note | `games/event_frontier/docs/SOURCES.md` |
| coverage matrix | `games/event_frontier/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/event_frontier/docs/MECHANICS.md` |
| implementation admission | `games/event_frontier/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

Event Frontier is a two-seat original Rulepath competitive game about a season
of public events across a small frontier graph. The Charter tries to establish
order with agents, depots, and funds. The Freeholders try to preserve independent
settlement with settlers, caches, and provisions.

The game proves event-driven initiative, typed rule exceptions, compound
operations, periodic scoring and reset, asymmetric instant victory, hidden
deck-order redaction, and per-faction scripted bots. It does not implement a
generic event engine, card engine, faction engine, graph helper, modifier stack,
scoring helper, hosted multiplayer, reaction windows, private hands, or
budgeted multi-command turns.

| Rule ID | Rule statement | Notes |
|---|---|---|
| `EF-SCOPE-001` | Event Frontier is an original two-faction public capstone game driven by a seeded event deck and public map state. | Public product game; no private licensed content. |
| `EF-SCOPE-002` | The game proves Gate 14 complexity through event cards, eligibility, edicts, Reckonings, asymmetric victory, long replay, and scripted policy bots. | All behavior remains game-local typed Rust. |
| `EF-SCOPE-003` | The game excludes reaction windows, private hands, hidden objectives, mid-game randomness, hosted services, and generic helpers. | These exclusions are rule boundaries, not missing features. |

## Implemented variants

| Rule ID | Variant rule | Source/rationale link |
|---|---|---|
| `EF-VAR-001` | `event_frontier_standard` is the default public variant. It uses six sites, eight trails, three epochs, 21 cards, resource cap 9, and the standard start state. | `SOURCES.md#variant-choice-and-deviations` |
| `EF-VAR-002` | `event_frontier_hard_winter` uses the same Rust rules with tighter starting resources and pressure-oriented epoch composition. | `SOURCES.md#variant-choice-and-deviations` |
| `EF-VAR-003` | `event_frontier_land_rush` uses the same Rust rules with more open starts and a faster cache race. | `SOURCES.md#variant-choice-and-deviations` |
| `EF-VAR-004` | Scenario data may declare labels, starting components, resource starts, thresholds, epoch composition, and public presentation metadata. Scenario data must not declare legal conditions, event behavior, edict behavior, scoring formulas, victory logic, selectors, triggers, scripts, or bot policy. | Rulepath static-data boundary. |

## Components and game-local vocabulary

Game nouns in this section belong to `games/event_frontier` only. They do not
authorize event, card, deck, epoch, initiative, eligibility, edict, Reckoning,
faction, site, trail, depot, cache, settler, agent, fund, or provision
vocabulary in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `EF-COMP-001` | seat | One of the two players, `seat_0` or `seat_1`. | public | `seat_0` controls the Charter in the standard setup; `seat_1` controls the Freeholders. |
| `EF-COMP-002` | faction | The public side controlled by a seat: Charter or Freeholders. | public | Factions have different operation sets and different instant victory conditions. |
| `EF-COMP-003` | site | One of six public places: Charterhouse, Landing, Crossing, Granite Pass, High Meadow, and Old Mill. | public | Sites form a graph, not a rectangular board. |
| `EF-COMP-004` | trail | An undirected public edge between two sites. | public | Trails constrain Freeholder trek operations and Charter adjacency for survey. |
| `EF-COMP-005` | agent | A Charter presence marker at a site, capped at three per site. | public | Agents contribute to Charter presence and site majority. |
| `EF-COMP-006` | depot | A Charter structure at a site, capped at one per site. | public | A depot contributes to Charter presence and changes some operation costs under edicts. |
| `EF-COMP-007` | settler | A Freeholder presence marker at a site, capped at three per site. | public | Settlers move, place caches, and contribute to Freeholder presence. |
| `EF-COMP-008` | cache | A Freeholder marker at a site, capped at two per site. | public | Caches count toward Freeholder instant victory but do not count as site presence. |
| `EF-COMP-009` | funds | The Charter resource pool, from 0 through 9. | public | Funds pay operation costs. |
| `EF-COMP-010` | provisions | The Freeholder resource pool, from 0 through 9. | public | Provisions pay operation costs. |
| `EF-COMP-011` | event deck | A seeded 21-card deck in three epochs, with one Reckoning per epoch. | hidden order | Current and next cards are public; deeper undrawn order is hidden from every browser-facing viewer and bot. |
| `EF-COMP-012` | event card | A closed Rust card ID with typed content fields and Rust-defined behavior. | public when current, next, or discarded | Card data is identity and parameters only. |
| `EF-COMP-013` | edict | A public card-imposed typed modifier active until the next Reckoning. | public while active | Edicts are typed Rust variants, not data scripts. |
| `EF-COMP-014` | Reckoning | A scoring/reset card that always resolves through the ordered pipeline. | public when current or resolved | Reckonings are seeded into epochs and never placed first in an epoch. |
| `EF-COMP-015` | eligibility | Each faction's public ability to act on the current card. | public | Acting by event or operation makes that faction ineligible for the next card; passing preserves eligibility. |
| `EF-COMP-016` | score track | Public cumulative site-scoring totals. | public | Used only for the final fallback if no instant victory fires. |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `EF-SETUP-001` | Create exactly two seats. `seat_0` controls the Charter and `seat_1` controls the Freeholders unless a future typed variant says otherwise. | deterministic | public | More seats and teams are out of scope. |
| `EF-SETUP-002` | Load the selected typed scenario constants: sites, trails, starting components, resource starts, caps, thresholds, epoch composition, and public labels. | deterministic | public except deck order | Unknown fields and behavior-looking fields are rejected. |
| `EF-SETUP-003` | Validate site and trail content before play: every trail endpoint exists, duplicate sites and duplicate trails are rejected, the graph is connected, and starts respect caps. | deterministic | public diagnostics | Invalid content fails closed at setup. |
| `EF-SETUP-004` | Build three seven-card epochs. Each epoch contains exactly one Reckoning and six event cards, and the seeded shuffle must not place that epoch's Reckoning first. | seeded deterministic | internal until card reveal | The authored data never stores shuffled order. |
| `EF-SETUP-005` | Reveal the current card and the next public card when available, initialize both factions eligible, set scores to zero, set resources from scenario constants, and create a fresh command token. | seeded deterministic | public current/next only | Same seed, variant, rules version, data version, and command stream reproduce the match. |

## Turn, round, and phase sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `EF-TURN-001` | Start of a non-Reckoning card. | first eligible faction | Rust chooses the first eligible faction: the card's printed faction if eligible, otherwise the other eligible faction. | The first faction chooses event, operation, or pass. |
| `EF-TURN-002` | No eligible faction on a non-Reckoning card. | none | Rust discards the card unresolved and advances to the next card. | The discard effect and next reveal complete. |
| `EF-TURN-003` | First faction takes the event. | first eligible faction | Rust resolves the card's typed event behavior, then offers the second faction operation or pass. | The event effects complete. |
| `EF-TURN-004` | First faction takes an operation. | first eligible faction | Rust applies one compound operation, then offers the second faction event, limited operation, or pass. | The operation effects complete. |
| `EF-TURN-005` | First faction passes. | first eligible faction | Rust gives that faction +1 resource, preserves eligibility, and offers the second faction the full menu. | The pass effects complete. |
| `EF-TURN-006` | Second faction choice. | second eligible faction | Rust applies the constrained event, operation, limited operation, or pass menu determined by the first choice. | The second choice resolves or the card is double-passed. |
| `EF-TURN-007` | Card cleanup. | none | Factions that took an event or operation become ineligible for the next card; factions that passed stay eligible. The used card goes to discard. | The next card is revealed or terminal state exists. |
| `EF-TURN-008` | Reckoning card. | none as command actor | Rust always resolves the Reckoning pipeline: victory check, site scoring, income, edict expiry, and eligibility reset. | The pipeline completes or terminal state fires. |
| `EF-TURN-009` | Terminal state. | none | Rust exposes the terminal result and no normal gameplay actions. | No further gameplay action advances the match. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide eligibility, menu
constraints, operation costs, site bounds, edict applicability, scoring, victory,
or bot policy.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `EF-ACT-001` | First eligible faction before a non-Reckoning event. | `event`, `operation/<op>/...`, or `pass` when legal. | constrained choice plus progressive operation tree | The card's printed first faction and current eligibility determine the actor. |
| `EF-ACT-002` | Second faction after first event. | `operation/<op>/...` or `pass`. | constrained choice plus progressive operation tree | The second faction may not take the same card's event after the first faction took it. |
| `EF-ACT-003` | Second faction after first operation. | `event`, `limited_operation/<op>/...`, or `pass`. | constrained choice plus progressive operation tree | Limited operation permits exactly one selected site. |
| `EF-ACT-004` | Second faction after first pass. | `event`, `operation/<op>/...`, or `pass` when legal. | constrained choice plus progressive operation tree | A double pass discards the card without event or operation resolution. |
| `EF-ACT-005` | Charter operation. | `survey`, `fortify`, or `writ` with Rust-generated site choices. | progressive compound action | Select an operation type, then one through ops-value sites, then any required per-site sub-choice. |
| `EF-ACT-006` | Freeholder operation. | `trek`, `cache`, or `rally` with Rust-generated site choices. | progressive compound action | Select an operation type, then one through ops-value sites, then any required per-site sub-choice. |
| `EF-ACT-007` | Non-active seat or observer. | none | empty gameplay tree | Waiting metadata may name only public phase and active faction facts. |
| `EF-ACT-008` | Reckoning, auto-discard, or terminal state. | none | empty gameplay tree | Automated phases and terminal states expose no normal gameplay choices. |

## Operations

| Rule ID | Operation rule | Timing | Validation notes | Effects |
|---|---|---|---|---|
| `EF-OP-001` | An operation costs 1 resource per selected site unless an active edict modifies cost. | before operation application | The acting faction must afford the full Rust-computed cost. | Resource cost effects precede site consequences. |
| `EF-OP-002` | A full operation may select from 1 through the current card's ops value in sites. | action-tree construction and validation | Ops values are typed card parameters from 1 through 3. | Site effects resolve in stable selected-site order. |
| `EF-OP-003` | A limited operation may select exactly one site. | second-choice menu after first operation | The same per-op legality rules apply. | Site effects resolve normally. |
| `EF-OP-004` | `survey` places one Charter agent at the Charterhouse or at a site adjacent to existing Charter presence, up to the agent cap. | Charter operation | Rust checks adjacency and caps. | Emits an agent-placement effect. |
| `EF-OP-005` | `fortify` builds one depot at a site with at least two Charter agents and no depot. | Charter operation | Rust checks agent count and depot cap. | Emits a depot-built effect. |
| `EF-OP-006` | `writ` removes one cache from a site with Charter agent presence and at least one cache, then gains 1 fund. | Charter operation | Rust checks public cache and agent state. | Emits cache-removed and resource-gained effects. |
| `EF-OP-007` | `trek` moves one Freeholder settler from a selected site along one trail to an adjacent site with capacity. | Freeholder operation | Rust checks settler presence, trail adjacency, and destination cap. | Emits a settler-moved effect. |
| `EF-OP-008` | `cache` places one cache at a settler-occupied site with no depot and below the cache cap. | Freeholder operation | Rust checks settler presence, depot absence, and cache cap. | Emits a cache-laid effect. |
| `EF-OP-009` | `rally` places one settler at the Landing or at a site with at least one cache, up to the settler cap. | Freeholder operation | Rust checks site eligibility and cap. | Emits a settler-rallied effect. |

## Events and edicts

Card data names card identity, label, epoch pool, printed first faction, ops
value, edict flag, and UI metadata only. The behavior of every event and edict
is a closed Rust match on card ID.

| Rule ID | Event/edict rule | Timing | Effect order | Notes |
|---|---|---|---|---|
| `EF-EVENT-001` | Fourteen ordinary event cards resolve one-shot typed Rust effects such as public component placement, removal, relocation, or resource changes. | event choice | Card-resolved effect before component/resource effects. | Later card data must not encode these effects. |
| `EF-EVENT-002` | Event effects must be deterministic consequences of the current state, card ID, and command stream. | event resolution | Stable card-specific order. | No random sampling after setup. |
| `EF-EDICT-001` | Four edict cards activate typed public modifiers that remain active until the next Reckoning. | event choice on an edict card | `EdictActivated` before affected later actions. | Edicts are not scripts or data-defined rules. |
| `EF-EDICT-002` | Toll Roads increases selected-site operation cost by 1 resource per selected site until the next Reckoning. | validation and application while active | Cost annotation before resource-spend effect. | Rust computes affordability. |
| `EF-EDICT-003` | Survey Ban blocks `survey` and `rally` at contested sites until the next Reckoning. | action-tree construction and validation while active | Diagnostic before rejection if submitted stale or malformed. | A contested site has both agent and settler presence. |
| `EF-EDICT-004` | Requisition makes Charter operations at depot sites cost 0 for those selected sites until the next Reckoning. | validation and application while active | Cost annotation before resource-spend effect. | Applies only to Charter operations and depot sites. |
| `EF-EDICT-005` | Long Season lets the first eligible faction select one site beyond the card's ops value for a full operation until the next Reckoning. | action-tree construction while active | Ops-bound metadata before operation choices. | Does not affect limited operations. |
| `EF-EDICT-006` | When several edicts are active, Rust applies them in stable `(kind, activation_index)` order. | validation and application while active | Stable modifier order. | Prevents nondeterministic override behavior. |
| `EF-EDICT-007` | Every active edict expires during the next Reckoning reset. | Reckoning reset | `EdictExpired` effects before next-card reveal. | No edict persists across a Reckoning. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `EF-RESTRICT-001` | Unknown or non-seat actor submits gameplay action. | Reject without mutation. | Viewer-safe wrong-actor diagnostic. | Only the two seats can act. |
| `EF-RESTRICT-002` | The wrong seat or an ineligible faction submits. | Reject without mutation. | Viewer-safe wrong-seat or ineligible-faction diagnostic. | Diagnostic may name public eligibility only. |
| `EF-RESTRICT-003` | A malformed, stale, or unavailable action path is submitted. | Reject without mutation. | Viewer-safe unavailable-action or stale-token diagnostic. | Replay/hash state must not change. |
| `EF-RESTRICT-004` | A faction attempts a menu choice forbidden by the first choice. | Reject without mutation. | Viewer-safe menu-constraint diagnostic. | For example, second event after first event is illegal. |
| `EF-RESTRICT-005` | An operation selects too many sites, no sites, an unknown site, or a site that fails operation preconditions. | Reject without mutation. | Viewer-safe operation diagnostic. | Includes full vs limited operation bounds. |
| `EF-RESTRICT-006` | A faction cannot afford the Rust-computed operation cost. | Reject without mutation. | Viewer-safe resource diagnostic. | Does not reveal deck order. |
| `EF-RESTRICT-007` | An active edict blocks or changes a submitted action. | Reject or adjust only according to Rust edict rules. | Viewer-safe edict diagnostic or cost metadata. | TypeScript cannot compute edict consequences. |
| `EF-RESTRICT-008` | A gameplay action is submitted during Reckoning, auto-discard, or terminal state. | Reject without mutation. | Viewer-safe phase or terminal diagnostic. | Automated phases have no actor. |

## Scoring and accounting

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `EF-SCORE-001` | Funds and provisions are public resource pools from 0 through 9. | setup, pass, operation, event, Reckoning | Resource gains are capped at 9. | Resource state is public. |
| `EF-SCORE-002` | Passing gives the passing faction +1 resource and preserves that faction's eligibility for the next card. | pass choice | Resource cap still applies. | Double pass discards the card. |
| `EF-SCORE-003` | Operation costs are paid before operation consequences resolve. | operation application | Unaffordable operation is illegal. | Edicts may alter the final cost. |
| `EF-SCORE-004` | During a Reckoning, each site awards 1 cumulative score point to the faction with strictly greater presence. | after instant-victory check | Tied presence awards no point. | Charter presence is agents plus depot; Freeholder presence is settlers only. |
| `EF-SCORE-005` | Reckoning income gives each faction +2 resources after site scoring, capped at 9. | Reckoning income step | Terminal instant victory stops before income. | Income happens before eligibility reset if no terminal result fired. |
| `EF-SCORE-006` | Cumulative scores are public and matter only for final fallback after the third Reckoning if no instant victory has fired. | after each Reckoning | Freeholders win tied final fallback totals. | Outcome explanation cites the fallback rule. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `EF-END-001` | At a Reckoning victory check, the Charter has majority presence at at least four of six sites. | Charter instant victory unless Freeholders also meet their instant condition. | See `EF-END-003`. | Checked before site scoring, income, edict expiry, and eligibility reset. |
| `EF-END-002` | At a Reckoning victory check, the Freeholders have at least eight total caches. | Freeholder instant victory. | See `EF-END-003`. | Caches are public. |
| `EF-END-003` | Both instant victory conditions are true during the same Reckoning victory check. | Freeholders win. | Freeholders win the both-met rule. | The frontier outlasts the Charter's claim. |
| `EF-END-004` | The third Reckoning pipeline completes without an instant victory. | Higher cumulative score wins. | Freeholders win tied cumulative scores. | Final fallback uses public scores. |
| `EF-END-005` | Terminal state is reached. | No further gameplay actions are legal. | not applicable | Terminal state does not reveal undrawn deck order. |

## Outcome explanation traceability

Every scoring and terminal rule that can decide a match has a stable rule ID and
enough detail for the web outcome surface to cite it.

| Outcome/explanation fact | Stable rule ID(s) | Decisive when | Notes for UI explanation |
|---|---|---|---|
| Charter site-majority instant victory | `EF-END-001`, `EF-SCORE-004` | Charter controls at least four sites at a Reckoning victory check and Freeholders do not also meet cache victory. | Cite the public controlled-site count and Reckoning number. |
| Freeholder cache instant victory | `EF-END-002` | Freeholders have at least eight public caches at a Reckoning victory check. | Cite the public cache count. |
| Both-met instant victory rule | `EF-END-003` | Both instant conditions are true at the same Reckoning victory check. | State that Freeholders win the both-met rule; do not call it a draw. |
| Final fallback score comparison | `EF-END-004`, `EF-SCORE-006` | The third Reckoning completes without instant victory and scores differ. | Cite both final public scores. |
| Final fallback tiebreak | `EF-END-004`, `EF-SCORE-006` | The third Reckoning completes without instant victory and scores are tied. | State that Freeholders win tied fallback totals. |
| Terminal no-action state | `EF-END-005` | Any terminal result. | No normal action controls remain. |

This table is traceability only. It is not a behavior DSL, selector table, or
TypeScript decision source. Rust remains the source of scoring, terminal
detection, and rationale projection.

## Visibility and private information

Public/browser payloads must not reveal hidden information through public views,
seat views, action trees, previews, diagnostics, effect logs, DOM attributes,
test IDs, logs, local storage, replay exports, bot explanations, candidate
rankings, or dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `EF-VIS-001` | Seats, factions, sites, trails, public components, resources, scores, eligibility, active edicts, current card, next public card, discards, Reckoning count, and terminal result. | observer and both seat viewers | after setup and after each projection, subject to reveal timing | public view, seat view, action tree metadata, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, dev inspector | These are safe public facts. |
| `EF-VIS-002` | Undrawn event-deck order beyond the public current and next cards. | no browser-facing viewer and no bot | never before reveal | public view, seat view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | Native internal tests may inspect under test authority only. |
| `EF-VIS-003` | Current card and next public card. | all viewers | current card at card start; next card when one exists | public view, effects, replay export, DOM | The next-card reveal is public but does not expose deeper order. |
| `EF-VIS-004` | Discarded and resolved cards. | all viewers | after discard or resolution effect | public view, effects, replay export, DOM | Public history remains visible. |
| `EF-VIS-005` | Bot rationale and candidate rankings. | only if projected by Rust as viewer-safe public data | after bot decision or diagnostics | bot explanation, dev inspector, logs, replay export | Rationale may cite public state, public cards, legal actions, and public victory distances only. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `EF-RNG-001` | Event Frontier uses deterministic seeded setup shuffle and no random sampling after setup. | Same seed, rules version, variant, data version, and command stream must reproduce internal state and effects. | internal for undrawn order | Epoch shuffling must preserve the Reckoning-never-first constraint. |
| `EF-RNG-002` | Card reveal, discard, event resolution, operation resolution, and Reckoning resolution are deterministic consequences of command application and current card state. | Replay must not require a separate card or scoring actor. | public only as effects reveal facts | Automated Reckonings are replayed from the same command stream. |
| `EF-RNG-003` | Public replay export is viewer-scoped and redacted. | Public exports must not include seed material or undrawn deck order that reconstructs future cards. | public export is redacted | Current, next, and discarded cards may appear only from their reveal points onward. |
| `EF-RNG-004` | Serialization order must remain stable. | Golden traces and replay checks must fail on unintended ordering changes. | mixed | Stable order covers sites, cards, effects, actions, views, resources, scores, edicts, and terminal rationale. |

## Bot-relevant non-authoritative strategy notes

These notes describe intended product behavior, not extra legal authority.
Implemented bots must choose from the Rust legal tree and validate through the
normal action path.

| Rule ID | Strategy note | Allowed input | Forbidden input |
|---|---|---|---|
| `EF-BOT-001` | A random-legal bot may select any legal leaf from its current action tree with deterministic tie-breaking. | legal action tree and bot RNG stream | direct state mutation, hidden deck order, or illegal fallback |
| `EF-BOT-002` | A Charter Level 1 bot should deny imminent cache victory, extend majority toward four sites, fortify contested held sites, and save funds when action value is low. | public view, legal action tree, current/next public card, public resources, public sites, public victory distances | undrawn deck order, hidden-state sampling, MCTS, ISMCTS, Monte Carlo, ML, or RL |
| `EF-BOT-003` | A Freeholder Level 1 bot should complete the cache threshold, escort exposed caches, break Charter majorities, spread settlers, and save provisions when action value is low. | public view, legal action tree, current/next public card, public resources, public sites, public victory distances | undrawn deck order, direct state mutation, or bypassing validation |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `EF-AMB-001` | Whether event behavior can live in card data. | No. Data identifies cards and typed parameters only; Rust defines each card's behavior. | Rulepath static-data boundary | strict-parse tests, fixture-check, serialization tests | No selectors, triggers, formulas, or scripts. |
| `EF-AMB-002` | Whether a Reckoning is a command actor. | No. Reckoning resolves as Rust automation when its card becomes current. | Gate 14 replay contract | replay traces, command-log tests | No synthetic actor appears in the command stream. |
| `EF-AMB-003` | Whether the next card is hidden. | No. The current and next cards are public; deeper undrawn order is hidden. | Gate 14 visibility scope | visibility tests, public export tests, browser no-leak smoke | The public next card supports initiative planning. |
| `EF-AMB-004` | Whether eligibility is a reaction window. | No. Eligibility is normal sequential turn state, not an interrupt, response, or cancellation window. | Gate 14 scope and atlas review | action-tree tests, no pending-response tests | The reaction-window row remains a non-use. |
| `EF-AMB-005` | Whether operations are multi-action turn budgets. | No. An operation is one compound command, not a sequence of separately validated budgeted commands. | Gate 14 atlas hard-gate review | action-shape tests, property tests | The budget hard-gate candidate resolves as non-use. |
| `EF-AMB-006` | Whether terminal reveal exposes the rest of the deck. | No. Undrawn order remains hidden after terminal state. | hidden-info no-leak posture | terminal no-leak trace, replay export test, browser no-leak smoke | Public history remains visible. |

## Rulepath deviations from adjacent event, card, and area-control games

| Rule ID | Adjacent pattern | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `EF-DEV-001` | Some event-card games use product-specific faction systems, card names, map presentation, or source vocabulary. | Event Frontier uses original factions, sites, cards, terms, and UI presentation. | Avoid source confusion and trade-dress imitation. | yes |
| `EF-DEV-002` | Some card-driven games let card text itself define exceptions. | Event Frontier stores no rule text or rule behavior in card data; Rust owns every effect. | Preserve Rulepath's static-data boundary. | yes |
| `EF-DEV-003` | Some area-control games include combat, hidden objectives, or more than two factions. | Event Frontier has no combat resolution, no hidden objectives, and exactly two factions. | Keep Gate 14 focused on event/initiative/scoring/victory pressure. | yes |
| `EF-DEV-004` | Some games reveal all remaining deck contents after the match. | Event Frontier never reveals deeper undrawn order in browser-facing surfaces. | Preserve no-leak and redacted export guarantees. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `EF-OOS-001` | Three-seat, four-seat, solo, team, or more-than-two-faction variants. | Two factions prove the gate's asymmetric public proof shape. | Later product polish only. |
| `EF-OOS-002` | Per-seat private hands, secret objectives, hidden roles, hidden victory conditions, or private component ownership. | Gate 14 hides only undrawn deck order from everyone. | Later hidden-role/objective gate only with no-leak review. |
| `EF-OOS-003` | Reaction windows, interrupts, pending responses, cancellations, or replacement effects. | Eligibility is sequential initiative, not response timing. | If a second reaction-capable game is proposed. |
| `EF-OOS-004` | Multi-action turn budgets. | Gate 14 uses one compound command per operation. | If design drifts into regenerated budgeted commands, the atlas hard gate fires before code. |
| `EF-OOS-005` | Dice, mid-game shuffles, random event sampling after setup, or random tie resolution. | Replay proof requires setup-only randomness. | Future chance gate only. |
| `EF-OOS-006` | Generic event, card, deck, initiative, eligibility, edict, scoring, victory, graph, or faction helpers in `engine-core` or `game-stdlib`. | First/second uses and hard-gate decisions keep these shapes local. | Revisit only through the mechanic atlas. |
| `EF-OOS-007` | Hosted multiplayer, accounts, matchmaking, chat, ranked play, persistence, real-time timers, or undo. | V1/v2 are static/local-first. | Future ADR only. |
| `EF-OOS-008` | Public MCTS, ISMCTS, Monte Carlo, ML, RL, LLM, or hidden-state-sampling bots. | Public bot law forbids them. | Future ADR only. |

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every rule ID in this document must appear in `RULE-COVERAGE.md`. Silent gaps
are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| none | not applicable | Initial rule-ID set. | not applicable | 2026-06-12 |
