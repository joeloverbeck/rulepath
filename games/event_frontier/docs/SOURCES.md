# Event Frontier Sources

Game ID: `event_frontier`

Public display name: `Event Frontier`

Implemented variant: `event_frontier_standard`, `event_frontier_hard_winter`, and `event_frontier_land_rush`

Prepared by: `Codex`

Created: 2026-06-12

Last updated: 2026-06-12

Rules version connected to this source note: `event-frontier-rules-v1`

## Source-use statement

Event Frontier is an original Rulepath event-frontier capstone game. The Gate
14 spec records an external mechanics research pass over card-driven initiative,
periodic scoring, scripted-bot patterns, and typed rule-exception engineering.
Those sources informed abstract design and engineering risks only.

No source rules prose, examples, card text, board text, faction names, card
names, icons, screenshots, scans, fonts, assets, art direction, UI layout,
product naming, or trade dress is copied. Rulepath rule prose, UI copy, visual
presentation, assets, icons, faction labels, site labels, operation names, event
labels, and component text for `event_frontier` are original project material.

Public presentation must use neutral Rulepath labels such as Event Frontier,
Charter, Freeholders, agents, depots, settlers, caches, funds, provisions,
edicts, Reckonings, Charterhouse, Landing, Crossing, Granite Pass, High Meadow,
and Old Mill. It must not claim to be, imitate, or borrow presentation from any
commercial game.

## Consulted sources

All sources in this table are project-authority, rationale, workflow,
mechanics-prior-art, or IP-boundary sources only. No source prose or assets are
copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 14 Event Frontier capstone spec | `../../../archive/specs/gate-14-event-frontier-event-complexity-capstone.md` | 2026-06-12 | project authority | product scope, original rules proposal, event/eligibility/Reckoning/victory shape, IP-risk register, and ticket sequencing | none | Governs this game's implementation tickets and acceptance evidence. |
| Rulepath Foundations | `../../../docs/FOUNDATIONS.md` | 2026-06-12 | project authority | behavior authority, static-data boundary, kernel vocabulary boundary, no-leak invariants, public-bot limits, and IP conservatism | none | Establishes the non-negotiable architecture and no-leak constraints. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-12 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, outcome explanation documentation, and official-game evidence. |
| Rulepath Engine/Game/Data Boundary | `../../../docs/ENGINE-GAME-DATA-BOUNDARY.md` | 2026-06-12 | project authority | typed static content, behavior-looking field rejection, and no behavior-in-data rule | none | Especially relevant because event-card data is the gate's highest-risk static-data surface. |
| Rulepath Mechanic Atlas | `../../../docs/MECHANIC-ATLAS.md` | 2026-06-12 | project authority | repeated-mechanic hard gates, local-only decisions, and promotion-debt checks | none | Ticket 002 owns the full Event Frontier ledger update. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-12 | project authority | naming, originality, public/private content boundary, asset/font policy, and human/legal review triggers | none | Requires original public prose/assets and avoids source-confusing presentation. |
| Gate 14 ticket `GAT14EVEFROEVE-001` | `../../../archive/tickets/GAT14EVEFROEVE-001.md` | 2026-06-12 | project authority | front-loaded rules/source-doc acceptance criteria and trade-dress register requirements | none | This ticket owns the initial `RULES.md` and `SOURCES.md` files. |
| COIN-family initiative and eligibility design notes | mechanics research summarized in the Gate 14 spec | 2026-06-12 | mechanics prior art | deterministic printed-side initiative, eligibility consequences, and pass/act tempo economy | none | Used only as abstract mechanics context; no product vocabulary, rules text, faction structure, card names, layouts, or presentation copied. |
| Card-driven event-vs-operation design notes | mechanics research summarized in the Gate 14 spec | 2026-06-12 | mechanics prior art | event-or-operation choice pressure and implementation risk around card interactions | none | Used to motivate per-card fixtures and typed Rust card behavior. |
| Periodic scoring design notes | mechanics research summarized in the Gate 14 spec | 2026-06-12 | mechanics prior art | ordered scoring/reset pipeline and edge cases around victory-before-reset | none | Used only as a caution for explicit ordering and rule IDs. |
| Layered modifier engineering notes | mechanics research summarized in the Gate 14 spec | 2026-06-12 | engineering prior art | deterministic ordering for temporary rule exceptions | none | Adapted as typed edicts in stable order; no external text or game-specific semantics copied. |
| Scripted solo-bot structure notes | mechanics research summarized in the Gate 14 spec | 2026-06-12 | bot prior art | deterministic decision tables with explicit tiebreaks and legality supervision | none | Used to avoid ambiguous bot flowcharts; Rulepath bots choose only from legal trees. |

## Adopted design facts

The implemented Event Frontier variants adopt these facts in original Rulepath
prose:

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Two seats and two factions | The Charter and Freeholders provide one institutional side and one independent side. | Two factions prove asymmetric victory and scripted policies without multiplying UI and balance work. |
| Six-site graph | Six public sites connected by eight public trails form the map. | Small graph keeps long-game action trees readable while proving graph/event interaction. |
| Public resources | Charter funds and Freeholder provisions pay for operations and are visible to all viewers. | Public accounting is part of the Gate 14 primitive-pressure review. |
| Event deck | A seeded, hidden-from-all 21-card deck drives initiative and event pressure. | Proves hidden order, deterministic shuffle, and long replay without private hands. |
| Current and next cards public | The current card and the next card are public planning information. | Keeps the game strategic while preserving deeper deck-order redaction. |
| Eligibility | Acting on event or operation costs next-card eligibility; passing preserves eligibility and pays. | Proves deterministic initiative without reaction windows. |
| Edicts | Four typed temporary modifiers expire at the next Reckoning. | Proves rule exceptions as Rust variants, not data scripts. |
| Reckonings | Three scoring/reset cards always resolve victory check, site scoring, income, and reset in order. | Proves periodic scoring and reset under replay. |
| Asymmetric victory | Charter wins by broad site majority; Freeholders win by cache threshold; final fallback compares cumulative score. | Proves different instant victory conditions through one terminal contract. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `event_frontier_standard` / Event Frontier | Gate 14 event-complexity capstone proof scope. | yes |
| alternate scenarios | `event_frontier_hard_winter` and `event_frontier_land_rush` | Gate 14 scenario-setup proof scope. | yes |
| player count | Exactly two seats. | Two seats are sufficient for initiative, asymmetric actions, and asymmetric victory. | yes |
| event deck | Deterministic shuffled epochs, hidden deeper order, public current and next cards. | Replay/no-leak requirement and event-frontier proof. | yes |
| terminal result | Charter instant victory, Freeholder instant victory, both-met Freeholder rule, or final fallback. | Gate 14 asymmetric-victory proof. | yes |
| optional rule included | Eligibility, event/op/pass menus, compound operations, edicts, Reckonings, Level 0 and Level 1 bot support. | Gate 14 acceptance requirements. | yes |
| optional rule excluded | More factions, private hands, hidden objectives, reaction windows, budgeted multi-command turns, combat, dice, mid-game shuffles, hosted multiplayer, public MCTS/ISMCTS/Monte Carlo/ML/RL bots. | Out of scope and forbidden by Gate 14. | yes |
| Rulepath deviation from adjacent games | Original frontier premise, original factions/sites/card labels, no copied product vocabulary, no source card text, no source map or component presentation. | IP conservatism and Gate 14 focus. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `EF-AMB-001` | Whether card data may encode event effects. | Gate 14 spec and Rulepath static-data boundary. | Card data identifies cards and typed parameters only; Rust owns every card behavior. | `EF-VAR-004`, `EF-EVENT-001`, `EF-AMB-001` | strict-parse tests, serialization tests, fixture-check | resolved |
| `EF-AMB-002` | Whether Reckonings appear as command-stream actors. | Gate 14 spec and Rulepath replay contract. | Reckonings resolve as Rust automation when the card becomes current. | `EF-TURN-008`, `EF-RNG-002`, `EF-AMB-002` | replay traces, command-log tests | resolved |
| `EF-AMB-003` | Whether the next card is public. | Gate 14 proposed rules and hidden-info posture. | Current and next cards are public; deeper order is hidden. | `EF-SETUP-005`, `EF-VIS-002`, `EF-VIS-003`, `EF-AMB-003` | visibility tests, export tests, browser no-leak smoke | resolved |
| `EF-AMB-004` | Whether eligibility is a response system. | Gate 14 scope and mechanic-atlas reaction-window row. | Eligibility is sequential initiative; it opens no interrupt or cancellation window. | `EF-TURN-001` through `EF-TURN-007`, `EF-AMB-004` | action-tree tests, no pending-response checks | resolved |
| `EF-AMB-005` | Whether operations repeat multi-action budgets. | Gate 14 scope and mechanic-atlas budget hard-gate candidate. | Each operation is one compound command; no regenerated budgeted command sequence exists. | `EF-ACT-005`, `EF-ACT-006`, `EF-OP-002`, `EF-AMB-005` | action-shape tests, property tests | resolved |
| `EF-AMB-006` | Whether terminal state reveals the rest of the deck. | Rulepath hidden-info invariant and Gate 14 no-leak posture. | Undrawn order remains hidden after terminal state. | `EF-END-005`, `EF-VIS-002`, `EF-RNG-003`, `EF-AMB-006` | terminal no-leak trace, public export tests, browser no-leak smoke | resolved |

## Public naming rationale

Public ID: `event_frontier`

Display name: `Event Frontier`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | unclear for the category; original name chosen | Event-card and area-control games carry commercial-presentation risk, so public naming avoids known product titles. |
| neutral name chosen? | yes | `Event Frontier` describes the original Rulepath premise without invoking a specific source game. |
| trademark risk considered? | yes | Public docs and UI avoid source product names, source faction names, source scoring-card labels, and source component terms. |
| trade-dress risk considered? | yes | Public presentation must avoid copied card layouts, player mats, map palettes, scoring tracks, counters, icons, board treatments, or source product layouts. |
| affiliation implication avoided? | yes | Sources are cited only as project authority, mechanics prior art, and IP-boundary context. Public UI must not imply compatibility, inspiration branding, or affiliation. |
| public help text needs disclaimer? | no | Neutral naming and source notes are sufficient unless later review requests additional text. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Card-driven initiative mechanic similarity | low for abstract mechanics, expression still reviewed | Use original faction names, event names, rules prose, and presentation. | no |
| Commercial counterinsurgency or historical-card-game vocabulary | medium if copied product terms, card text, or faction-board presentation appears | Use Charter/Freeholders, event/op/pass, edict/Reckoning, and original UI treatments; avoid source terms and layouts. | yes if copied expression appears |
| Periodic area-scoring presentation | medium if copied score-card names, area names, or board treatment appears | Use six original sites and Rust-projected Reckoning breakdowns; no source scoring-card labels or map art. | yes if source-confusing presentation appears |
| Woodland or faction-war trade dress | medium if copied faction themes, map areas, component silhouettes, or palette appears | Use neutral frontier-administration and settlement language; no source visuals or faction identities. | yes if source-confusing presentation appears |
| Rule-exception card text | high if copied from any commercial game | Edict names and behavior are original; card copy must be original and concise. | yes if copied text appears |
| Visual trade dress | medium if copied boards, cards, icons, screenshots, fonts, component layout, or marketing copy appear | Use original board treatment and no source screenshots/scans/icons/fonts. | yes if copied or source-confusing |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---:|---|---|---:|
| Game docs | `games/event_frontier/docs/RULES.md`, `games/event_frontier/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied source rules, examples, card text, or tables. | yes |
| Public game name | `Event Frontier` | original project name | Rulepath/Codex-authored public name | Public name avoids known product branding. | yes |
| Faction labels | Charter, Freeholders | original local labels | Rulepath/Codex-authored labels | Generic institutional/settlement language; no copied faction names. | yes |
| Site labels | Charterhouse, Landing, Crossing, Granite Pass, High Meadow, Old Mill | original local labels | Rulepath/Codex-authored labels | Generic place labels; no copied board location names. | yes |
| Component labels | agents, depots, settlers, caches, funds, provisions | original local labels | Rulepath/Codex-authored labels | Common descriptive terms arranged for original rules. | yes |
| Operation labels | survey, fortify, writ, trek, cache, rally | original local labels | Rulepath/Codex-authored labels | Labels are ordinary words and not copied from a product ruleset. | yes |
| Event and edict labels | Toll Roads, Survey Ban, Requisition, Long Season, Reckoning | original local labels | Rulepath/Codex-authored labels | Public wording remains subject to pre-polish naming review. | yes |
| Game UI/assets | none in this ticket | not applicable | none | UI/assets land in later tickets. | yes |

## Generated asset review notes

| Generated asset | Tool/model if known | Prompt/source inputs safe? | Similarity/trade-dress risk | Human review result | Public-safe? |
|---|---|---:|---|---|---:|
| none | not applicable | yes | none | No generated art/assets in this ticket. | yes |

## Font status

| Font | Source/license | Bundled in public artifact? | Review status | Notes |
|---|---|---:|---|---|
| system font stack | not bundled | no | safe by default | No font files are introduced by this ticket. |

## Public/private content boundary

| Content | Public allowed? | Location | Notes |
|---|---:|---|---|
| Rulepath original rules summary | yes | `games/event_frontier/docs/RULES.md` | Original Rulepath prose. |
| Project-authority Gate 14 facts | yes | `games/event_frontier/docs/SOURCES.md` | Summarized as rationale only. |
| Abstract event/initiative/scoring/modifier mechanics context | yes | `games/event_frontier/docs/SOURCES.md` | Discussed as design context, not copied expression. |
| Public source prose, examples, faction names, card text, or component tables | no | none | No external source prose is copied. |
| Commercial/licensed rules text | no | none | No source material is redistributed. |
| Commercial board art, card faces, faction boards, icons, screenshots, scans, fonts, or table presentation | no | none | No assets introduced in this ticket. |
| Private licensed stress-test content | no public shipment | none | No private licensed content is involved. |

Private licensed stress tests are late, isolated, optional, non-public, and
non-architectural. They must not contaminate `engine-core`, public assets,
public docs, or public web bundles.

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Public name is neutral and original. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | Local labels only; no source card text. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no | No art or UI asset in this ticket. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no | None. |
| uncertainty about public-domain status | no for abstract mechanics; commercial expression remains avoided | No source material is redistributed. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release blocking concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| Public visual design has not yet undergone asset/trade-dress review. | no for this ticket | Review generated or drawn assets when browser presentation lands. | Rulepath maintainers |
| Public card labels may need final naming review before release. | no for this ticket | Rename locally before implementation if maintainers prefer different labels. | Rulepath maintainers |

## Player-rules source notes

Confirm before merging player-facing rules:

- [x] Prose is original Rulepath wording.
- [x] No copied rulebook text, examples, diagrams, assets, names, fonts, or trade dress.
- [x] No strategy advice copied from `COMPETENT-PLAYER.md`.
- [x] No hidden match-state examples or seed-specific deck order.
- [x] No YAML front matter.
- [x] No selectors, conditions, triggers, or action schemas.
- [x] Formal rules version checked matches `RULES.md`.

## Rule-source-to-rule-ID cross-reference

Every release-relevant stable rule ID in `RULES.md` must have source or design
rationale support here. Event Frontier is original, so the primary support is
the Gate 14 spec plus the design rationale summarized below.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `EF-SCOPE-001` through `EF-SCOPE-003` | Original Gate 14 capstone scope and exclusions. | Gate 14 spec and Rulepath foundations. | yes | Prevents scope creep and architecture claims. |
| `EF-VAR-001` through `EF-VAR-004` | Standard, Hard Winter, Land Rush, and no behavior-in-data. | Gate 14 scenario-setup proof and Rulepath static-data boundary. | yes | Scenario data remains content only. |
| `EF-COMP-001` through `EF-COMP-016` | Seats, factions, sites, trails, components, resources, deck, cards, edicts, Reckonings, eligibility, and scores. | Gate 14 component and architecture proof. | no | Vocabulary remains game-local. |
| `EF-SETUP-001` through `EF-SETUP-005` | Two-seat setup, scenario constants, graph validation, epoch shuffle, public current/next cards, and initial state. | Gate 14 deterministic setup and replay-stability requirements. | yes | Hidden deck order remains internal. |
| `EF-TURN-001` through `EF-TURN-009` | First/second eligible flow, no-eligible discard, first/second choices, cleanup, Reckoning, and terminal state. | Gate 14 event/initiative proof. | yes | Reckoning is automation, not an actor. |
| `EF-ACT-001` through `EF-ACT-008` | Rust-owned action menus, constrained second choices, operation trees, waiting, and automated/terminal empty trees. | Gate 14 behavior-authority and legal-only UI requirements. | no | TypeScript computes no legality. |
| `EF-OP-001` through `EF-OP-009` | Operation costs, site bounds, limited operation, Charter operations, and Freeholder operations. | Gate 14 large-action-tree and resource-pressure proof. | no | Operations are one compound command each. |
| `EF-EVENT-001`, `EF-EVENT-002`, `EF-EDICT-001` through `EF-EDICT-007` | Event behavior, deterministic resolution, typed edicts, cost/block/extra-site modifiers, stable edict order, and expiry. | Gate 14 rule-exception proof and static-data boundary. | yes | Every card behavior is Rust-owned. |
| `EF-RESTRICT-001` through `EF-RESTRICT-008` | Wrong actor, wrong seat/ineligible, unavailable/stale, menu constraint, op validation, affordability, edict, automated-phase, and terminal restrictions. | Rulepath diagnostic/replay invariants plus Gate 14 spec. | no | Reject without mutation. |
| `EF-SCORE-001` through `EF-SCORE-006` | Resource pools, pass income, op costs, site scoring, Reckoning income, and final fallback score. | Gate 14 public accounting and periodic scoring proof. | yes | Public accounting is reviewed in ticket 002. |
| `EF-END-001` through `EF-END-005` | Charter instant, Freeholder instant, both-met Freeholder rule, final fallback, and terminal no-action/no-undrawn-reveal posture. | Gate 14 asymmetric-victory proof. | yes | Outcome explanation cites these IDs. |
| `EF-VIS-001` through `EF-VIS-005` | Public facts, hidden undrawn order, current/next reveal, discard history, and bot-rationale limits. | Gate 14 hidden-order no-leak exit criteria. | yes | Browser-facing surfaces are protected. |
| `EF-RNG-001` through `EF-RNG-004` | Seeded epoch shuffle, deterministic card/Reckoning consequences, redacted export, and stable serialization. | Gate 14 replay/export requirements. | no | Public export must not reconstruct undrawn order. |
| `EF-BOT-001` through `EF-BOT-003` | Random legal bot, Charter Level 1 policy boundary, and Freeholder Level 1 policy boundary. | Gate 14 scripted-bot baseline and Rulepath public-bot law. | no | Search, sampling, and learning bots are forbidden. |
| `EF-AMB-001` through `EF-AMB-006` | Chosen resolutions for behavior-in-data, Reckoning actor status, next-card visibility, eligibility, budget non-use, and terminal no-reveal. | Gate 14 scope, mechanic atlas, and no-leak posture. | yes | Tests/traces must preserve these decisions. |
| `EF-DEV-001` through `EF-DEV-004`, `EF-OOS-001` through `EF-OOS-008` | IP, scope, no-leak, and out-of-scope variants. | Gate 14 forbidden changes and IP policy. | yes | Prevents scope creep and source confusion. |

## Final source/IP checklist

- Sources are summarized in original language.
- Source quality and date consulted are recorded.
- Variant choices and deviations are explicit.
- Ambiguities connect to rule IDs and tests.
- Public naming avoids affiliation and trade-dress risk.
- Assets are original, verified public-domain, license-reviewed, or generated-reviewed.
- Fonts are system-only or license-reviewed.
- Public/private content boundary is explicit.
- Human/legal review triggers are not hidden.
- Release blockers are recorded.
