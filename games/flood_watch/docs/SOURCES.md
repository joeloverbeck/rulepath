# Flood Watch Sources

Game ID: `flood_watch`

Public display name: `Flood Watch`

Implemented variant: `flood_watch_standard` and `flood_watch_deluge`

Prepared by: `Codex`

Created: 2026-06-11

Last updated: 2026-06-11

Rules version connected to this source note: `flood-watch-rules-v1`

## Source-use statement

Flood Watch is an original Rulepath cooperative event-pressure game. The Gate
12 spec states that no external research pass backed the spec; this initial
source note therefore records project-authority sources, implementation
rationale, and an IP/trade-dress avoidance register rather than claiming
external rule authority.

No source rules prose, examples, role names, component text, card text, icons,
screenshots, scans, fonts, assets, art direction, UI layout, product naming, or
trade dress is copied. Rulepath rule prose, UI copy, visual presentation,
assets, icons, district labels, role labels, event labels, and component text
for `flood_watch` are original project material.

Public presentation must use neutral Rulepath labels such as Flood Watch,
Riverside, Old Docks, Market, Terraces, Gardens, Pumpwright, Levee Warden,
Downpour, Storm Surge, Reprieve, bail, reinforce, forecast, levee, and
inundated. It must not claim to be, imitate, or borrow presentation from any
commercial cooperative game.

## Consulted sources

All sources in this table are project-authority, rationale, workflow, or
IP-boundary sources only. No source prose or assets are copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 12 Flood Watch cooperative event-pressure spec | `../../../specs/gate-12-flood-watch-cooperative-event-pressure-proof.md` | 2026-06-11 | project authority | product scope, original variant, cooperative terminal rule, event-deck pressure, role powers, action budget, deterministic automation, docs, tests, replay, bot, WASM, UI, benchmark, and capstone obligations | none | Governs this game's implementation tickets and acceptance evidence. |
| Rulepath Foundations | `../../../docs/FOUNDATIONS.md` | 2026-06-11 | project authority | behavior authority, static-data boundary, engine/game vocabulary boundary, no-leak invariants, public-bot limits, and IP conservatism | none | Establishes the non-negotiable architecture and no-leak constraints. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-11 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, outcome explanation documentation, and official-game evidence. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-11 | project authority | naming, originality, public/private content boundary, asset/font policy, and human/legal review triggers | none | Requires original public prose/assets and avoids source-confusing presentation. |
| Rulepath Gate 12 ticket `GAT12FLOWATCOO-001` | `../../../tickets/GAT12FLOWATCOO-001.md` | 2026-06-11 | project authority | front-loaded rules/source-doc acceptance criteria and trade-dress register requirements | none | This ticket owns the initial `RULES.md` and `SOURCES.md` files. |

## Adopted design facts

The implemented Flood Watch variants adopt these facts in original Rulepath
prose:

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Two seats | Exactly two cooperative seats play the match. | Two seats are enough to prove shared outcome, asymmetric roles, and teammate-bot mode. |
| Shared terminal outcome | The team wins together or loses together. | Gate 12's new terminal proof is cooperative rather than per-seat. |
| Five flat districts | Riverside, Old Docks, Market, Terraces, and Gardens are public target areas with no adjacency. | Flat districts prove event pressure without pre-empting Gate 13 graph pressure. |
| Flood levels and levees | Public flood levels rise toward inundation and public levees absorb incoming rise. | Simple counters make the environment pressure readable and testable. |
| Event deck | A seeded, hidden-from-all deck drives environment pressure. | Proves deterministic automation and redacted replay/export without per-seat private holdings. |
| Forecast | A seat may reveal the top event card publicly before it is drawn. | Gives players a planning action while keeping the deck order hidden. |
| Roles | Pumpwright improves bail; Levee Warden improves reinforce. | Public deterministic role powers prove game-local asymmetry. |
| Multi-action budget | The active seat submits several validated actions before the environment responds. | Proves budgeted turns and legal-tree regeneration. |
| Typed scenarios | Standard and Deluge share rules but vary constants. | Proves scenario setup as typed content, not behavior data. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `flood_watch_standard` / Flood Watch | Gate 12 cooperative event-pressure proof scope. | yes |
| harder scenario | `flood_watch_deluge` | Gate 12 scenario-setup proof scope. | yes |
| player count | Exactly two seats. | Small official-game proof with clear teammate roles. | yes |
| event deck | Deterministic shuffled deck, hidden until forecast or draw. | Replay/no-leak requirement and event-pressure proof. | yes |
| terminal result | Shared win on deck exhaustion; shared loss on inundation. | Cooperative outcome proof. | yes |
| optional rule included | Role powers, public forecast, levee absorption, multi-action budget, Level 0 and Level 1 bot support. | Gate 12 acceptance requirements. | yes |
| optional rule excluded | Three- or four-seat play, solo play, movement, adjacency, spreading floods, reaction windows, traitor mechanics, per-seat hidden hands, re-shuffle escalation, hosted multiplayer, public MCTS/ISMCTS/Monte Carlo/ML/RL bots. | Out of scope and forbidden by Gate 12. | yes |
| Rulepath deviation from adjacent cooperative games | Original river-town theme, flat districts, no named commercial component vocabulary, no outbreak/epidemic/shore-up/sandbag language, no island/desert/spirit/firefighter presentation, no role roster copied from any product. | IP conservatism and Gate 12 focus. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `FW-AMB-001` | Whether environment resolution should be a command actor. | Gate 12 spec and Rulepath replay contract. | Environment resolution is an atomic Rust consequence of the turn-ending or final-budget command. | `FW-TURN-003`, `FW-TURN-004`, `FW-ENV-001`, `FW-RNG-002` | replay traces, command-log tests | resolved |
| `FW-AMB-002` | Whether undrawn event cards reveal at terminal. | Gate 12 no-leak posture and Foundations hidden-information invariant. | Undrawn cards never reveal in browser-facing surfaces, including post-terminal. | `FW-VIS-002`, `FW-RNG-003`, `FW-END-003` | terminal no-leak trace, public export tests, browser no-leak smoke | resolved |
| `FW-AMB-003` | Whether forecast is private. | Gate 12 cooperative public-state design. | Forecast is public to both seats and observers. | `FW-ACT-004`, `FW-VIS-003` | visibility tests, effect-log tests | resolved |
| `FW-AMB-004` | Whether flood should spread between districts. | Gate 12 out-of-scope graph rule and Gate 13 successor scope. | Districts are flat targets with no adjacency or spread. | `FW-COMP-003`, `FW-OOS-002` | rule coverage, setup validation | resolved |
| `FW-AMB-005` | Whether scenario files may encode event scripts. | Foundations static-data boundary and Gate 12 forbidden changes. | Scenario files declare constants and event-kind counts only; Rust owns event behavior. | `FW-VAR-003`, `FW-AMB-005` | strict-parse tests, fixture-check | resolved |

## Public naming rationale

Public ID: `flood_watch`

Display name: `Flood Watch`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | unclear for the category; original name chosen | Cooperative disaster/event-pressure games carry commercial-presentation risk, so public naming avoids known product titles. |
| neutral name chosen? | yes | `Flood Watch` describes an original Rulepath premise without invoking a specific source game. |
| trademark risk considered? | yes | Public docs and UI avoid names from Forbidden Island, Forbidden Desert, Pandemic, Spirit Island, Flash Point, and similar commercial games. |
| trade-dress risk considered? | yes | Public presentation must avoid copied island tiles, desert maps, disease/outbreak tracks, spirit boards, firefighter/rescue layouts, role-card treatments, component shapes, and product palettes. |
| affiliation implication avoided? | yes | Sources are cited only as project authority and IP-boundary context. Public UI must not imply compatibility, inspiration branding, or affiliation. |
| public help text needs disclaimer? | no | Neutral naming and source notes are sufficient unless later review requests additional text. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Cooperative event-pressure mechanic similarity | low for abstract mechanics, expression still reviewed | Use original rules prose, original names, flat districts, original event labels, and no source visuals. | no |
| Forbidden Island / Forbidden Desert similarity | medium if copied island/desert vocabulary, tile treatments, sand/water component language, or role names appear | Avoid island/tile presentation, "shore up", "sandbag", named commercial roles, treasure/desert visual language, and source-like component layouts. | yes if copied expression appears |
| Pandemic-style disease/outbreak presentation | medium if outbreak/epidemic/infection vocabulary or map-track trade dress appears | Use flood-specific original labels; no outbreak, epidemic, infection, disease cubes, city network, or escalation re-shuffle. | yes if source-confusing presentation appears |
| Spirit Island-style spirit/power presentation | medium if spirit boards, power cards, invader tracks, or source vocabulary appear | Use no spirits, settlers/invaders, power-card layout, growth phases, or source visual language. | yes if source-confusing presentation appears |
| Flash Point-style fire/rescue presentation | medium if firefighter roles, building-fire visuals, rescue iconography, or source layouts appear | Flood Watch uses town-district pressure without firefighter/rescue vocabulary or copied board shapes. | yes if source-confusing presentation appears |
| Role/card/component text | high if copied from any commercial game | Pumpwright and Levee Warden are original labels; event text is original and minimal. | yes if copied text appears |
| Visual trade dress | medium if copied boards, cards, icons, screenshots, fonts, component layout, or marketing copy appear | Use original board treatment and no source screenshots/scans/icons/fonts. | yes if copied or source-confusing |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---:|---|---|---:|
| Game docs | `games/flood_watch/docs/RULES.md`, `games/flood_watch/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied source rules, examples, roles, or tables. | yes |
| Public game name | `Flood Watch` | original project name | Rulepath/Codex-authored public name | Public name avoids known product branding. | yes |
| District labels | Riverside, Old Docks, Market, Terraces, Gardens | original local labels | Rulepath/Codex-authored labels | Generic town-place language; no copied board location names. | yes |
| Role labels | Pumpwright, Levee Warden | original local labels | Rulepath/Codex-authored labels | Avoids named commercial roles, "Engineer", "Pilot", "Messenger", and source-specific rosters. | yes |
| Event labels | Downpour, Storm Surge, Reprieve | original local labels | Rulepath/Codex-authored labels | Avoids outbreak/epidemic/infection, flood-water-level source phrasing, sandstorm/escalation vocabulary, and copied card text. | yes |
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
| Rulepath original rules summary | yes | `games/flood_watch/docs/RULES.md` | Original Rulepath prose. |
| Project-authority Gate 12 facts | yes | `games/flood_watch/docs/SOURCES.md` | Summarized as rationale only. |
| Abstract cooperative/event-pressure mechanic context | yes | `games/flood_watch/docs/SOURCES.md` | Discussed as project design context, not copied expression. |
| Public source prose, examples, role names, card text, or component tables | no | none | No external source prose is copied. |
| Commercial/licensed rules text | no | none | No source material is redistributed. |
| Commercial board art, card faces, role rosters, event cards, icons, screenshots, scans, fonts, or table presentation | no | none | No assets introduced in this ticket. |
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
| Deeper external prior-art review was not part of the spec. | no for this ticket | If maintainers want additional research before public release, run it before visual/assets polish and update this file. | Rulepath maintainers |

## Rule-source-to-rule-ID cross-reference

Every release-relevant stable rule ID in `RULES.md` must have source or design
rationale support here. Flood Watch is original, so the primary support is the
Gate 12 spec plus the design rationale summarized below.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `FW-VAR-001` through `FW-VAR-003` | Standard and Deluge scenarios, typed constants, and no behavior-in-data. | Gate 12 scenario-setup proof and Rulepath static-data boundary. | yes | Scenario data remains content only. |
| `FW-COMP-001` through `FW-COMP-011` | Seats, roles, districts, flood levels, levees, event deck, cards, forecast, budget, environment phase, and shared outcome. | Gate 12 component and architecture proof. | no | Vocabulary remains game-local. |
| `FW-SETUP-001` through `FW-SETUP-005` | Two-seat setup, scenario constants, role assignment, seeded deck construction, and initial public state. | Gate 12 deterministic setup and replay-stability requirements. | no | Hidden deck order remains internal. |
| `FW-TURN-001` through `FW-TURN-007` | Budgeted action phase, teammate waiting state, final-budget trigger, explicit end turn, environment phase, cleanup, and terminal state. | Gate 12 multi-action budget and environment automation proof. | yes | No synthetic environment actor. |
| `FW-ACT-001` through `FW-ACT-007` | Rust-owned bail, reinforce, forecast, end-turn, waiting, and terminal action-tree rules. | Gate 12 behavior-authority and legal-only UI requirements. | no | TypeScript computes no legality. |
| `FW-RESTRICT-001` through `FW-RESTRICT-008` | Wrong actor, wrong seat, unavailable path, stale command, invalid bail/reinforce/forecast, and terminal restrictions. | Rulepath diagnostic/replay invariants plus Gate 12 spec. | no | Reject without mutation. |
| `FW-ENV-001` through `FW-ENV-007` | Environment batch trigger, event draw, Downpour, Storm Surge, Reprieve, inundation early stop, and deck-exhaustion win. | Gate 12 event-pressure proof and original rules design. | yes | Effect order must be tested. |
| `FW-SCORE-001` through `FW-SCORE-005` | Flood levels, levees, action budget, remaining composition, and no individual scores. | Gate 12 cooperative shared-outcome proof. | no | Counters are public except deck order. |
| `FW-END-001` through `FW-END-003` | Shared loss, shared win, and terminal no-action/no-undrawn-reveal posture. | Gate 12 terminal and no-leak requirements. | yes | Outcome explanation cites these IDs. |
| `FW-VIS-001` through `FW-VIS-005` | Public facts, hidden undrawn deck, forecast reveal, drawn events, and bot-rationale limits. | Gate 12 hidden-info no-leak exit criteria. | yes | Browser-facing surfaces are protected. |
| `FW-RNG-001` through `FW-RNG-004` | Seeded setup, deterministic event draws, redacted export, and stable serialization. | Gate 12 replay/export requirements. | no | Public export must not reconstruct undrawn events. |
| `FW-BOT-001` through `FW-BOT-003` | Random legal bot, Level 1 public-information policy, and teammate-bot boundary. | Gate 12 bot baseline and Rulepath public-bot law. | no | Solver, sampling, and learning bots are forbidden. |
| `FW-AMB-001` through `FW-AMB-005` | Chosen resolutions for automation actor, terminal deck reveal, public forecast, flat districts, and no behavior-in-data. | Gate 12 scope and Rulepath foundation constraints. | yes | Tests/traces must preserve these decisions. |
| `FW-DEV-001` through `FW-DEV-003`, `FW-OOS-001` through `FW-OOS-006` | IP, scope, no-leak, and out-of-scope variants. | Gate 12 forbidden changes and IP policy. | yes | Prevents scope creep and source confusion. |

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
