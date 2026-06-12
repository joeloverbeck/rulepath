# Frontier Control Sources

Game ID: `frontier_control`

Public display name: `Frontier Control`

Implemented variant: `frontier_control_standard` and `frontier_control_highlands`

Prepared by: `Codex`

Created: 2026-06-11

Last updated: 2026-06-11

Rules version connected to this source note: `frontier-control-rules-v1`

## Source-use statement

Frontier Control is an original Rulepath asymmetric graph area-control game.
The Gate 13 spec states that no external research pass backed the spec; this
initial source note therefore records project-authority sources, implementation
rationale, and an IP/trade-dress avoidance register rather than claiming
external rule authority.

No source rules prose, examples, faction names, component text, card text,
icons, screenshots, scans, fonts, assets, art direction, UI layout, product
naming, or trade dress is copied. Rulepath rule prose, UI copy, visual
presentation, assets, icons, site labels, faction labels, action labels, and
component text for `frontier_control` are original project material.

Public presentation must use neutral Rulepath labels such as Frontier Control,
Garrison, Prospectors, Gatehouse, Signal Hill, Base Camp, Ford, Quarry,
Timberline, Goldfield, guards, crews, stakes, forts, trails, march, patrol,
reinforce, muster, dismantle, clash, and supply. It must not claim to be,
imitate, or borrow presentation from any published area-control or asymmetric
faction game.

## Consulted sources

All sources in this table are project-authority, rationale, workflow, or
IP-boundary sources only. No source prose or assets are copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 13 Frontier Control asymmetric area-control spec | `../../../specs/gate-13-frontier-control-asymmetric-area-control-proof.md` | 2026-06-11 | project authority | product scope, original variants, graph topology proof, faction asymmetry, scoring formulas, docs, tests, replay, bot, WASM, UI, benchmark, and capstone obligations | none | Governs this game's implementation tickets and acceptance evidence. |
| Rulepath Foundations | `../../../docs/FOUNDATIONS.md` | 2026-06-11 | project authority | behavior authority, static-data boundary, engine/game vocabulary boundary, no-leak invariants, public-bot limits, and IP conservatism | none | Establishes the non-negotiable architecture and public-product constraints. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-11 | project authority | documentation and evidence workflow | none | Governs original rules prose, source notes, coverage matrix, outcome explanation documentation, and official-game evidence. |
| Rulepath Mechanic Atlas | `../../../docs/MECHANIC-ATLAS.md` | 2026-06-11 | project authority | primitive-pressure process, board-space applicability audit, second-use comparisons, and first-use graph/control/asymmetry rows | none | Prevents speculative graph, control, budget, or faction helper promotion. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-11 | project authority | naming, originality, public/private content boundary, asset/font policy, and human/legal review triggers | none | Requires original public prose/assets and avoids source-confusing presentation. |
| Rulepath Gate 13 ticket `GAT13FROCONASY-001` | `../../../tickets/GAT13FROCONASY-001.md` | 2026-06-11 | project authority | front-loaded rules/source-doc acceptance criteria and trade-dress register requirements | none | This ticket owns the initial `RULES.md` and `SOURCES.md` files. |
| U.S. Copyright Office, Games circular | `https://www.copyright.gov/help/faq/faq-protect.html` | 2026-06-11 | legal/policy reference | games rules-vs-expression boundary | none | Used only as a conservative reminder that expression and assets require care. |
| *DaVinci Editrice S.R.L. v. ZiKo Games, LLC* | public case reference | 2026-06-11 | legal/policy reference | games rules-vs-expression boundary | none | Used only as a high-level reminder to avoid copied expression and trade dress. |

## Adopted design facts

The implemented Frontier Control variants adopt these facts in original
Rulepath prose:

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Two seats | Exactly two competitive seats play the match. | Two seats are enough to prove asymmetric faction action sets and per-faction bots. |
| Public factions | The Garrison and the Prospectors are public original faction labels. | They provide asymmetric rules without hidden roles or source-confusing branding. |
| Graph map | Sites are joined by trails. Movement and supply use adjacency, not coordinates. | Gate 13 proves graph topology while keeping graph nouns game-local. |
| Perfect information | All units, sites, trails, stakes, scores, and outcomes are public. | The gate's proof budget stays on graph/asymmetry rather than no-leak hidden state. |
| No game-rule randomness | Setup and scoring are deterministic typed content and Rust behavior. | Replay/hash proof is clear and no shuffle/deck row is engaged. |
| Budgeted turns | Each active faction spends a small action budget before the turn passes. | Records a second-use comparison with Flood Watch while staying local. |
| Asymmetric action sets | Prospectors march, stake, and muster; the Garrison patrols, reinforces, and dismantles. | Proves disjoint action vocabularies through generic action-tree contracts. |
| Asymmetric clashes | Crews trade themselves for guards when entering; guards survive when entering crewed sites. | Proves faction-specific application rules without a generic contest helper. |
| Faction-specific scoring | The Garrison scores held forts; the Prospectors score supplied stakes. | Proves asymmetric scoring formulas feeding one comparable track. |
| Garrison tiebreak | Equal final scores are won by the Garrison. | Gives the terminal outcome a deterministic incumbent tiebreak. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `frontier_control_standard` / Frontier Control | Gate 13 asymmetric graph/control proof scope. | yes |
| second map | `frontier_control_highlands` | Gate 13 typed-content proof that maps are content, not behavior. | yes |
| player count | Exactly two seats. | Small official-game proof with clear faction asymmetry. | yes |
| hidden information | None. | Deliberate Gate 13 scope choice. | yes |
| game-rule randomness | None. | Deliberate Gate 13 replay/scope choice. | yes |
| optional rule included | Graph adjacency, two-action turns, asymmetric clash rules, stake supply scoring, Garrison tiebreak, Level 0 and Level 1 bot support. | Gate 13 acceptance requirements. | yes |
| optional rule excluded | Three or more factions, team play, solo play, hidden objectives, event decks, reaction windows, asymmetric victory conditions, hosted multiplayer, public MCTS/ISMCTS/Monte Carlo/ML/RL bots. | Out of scope and forbidden by Gate 13. | yes |
| Rulepath deviation from adjacent area-control games | Original frontier labels, small graph, no named commercial faction vocabulary, no region-conquest trade dress, no woodland/clearing vocabulary, no influence-cube imitation, no source board layout. | IP conservatism and Gate 13 focus. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `FC-AMB-001` | Whether graph topology should use the promoted rectangular board-space helper. | Gate 13 spec, Mechanic Atlas, board-space primitive scope. | No. Frontier Control is a site/edge graph with no rectangular coordinates; `board_space` is not applicable. | `FC-COMP-003`, `FC-COMP-004`, `FC-CTRL-001` | primitive-pressure ledger, setup validation tests | resolved |
| `FC-AMB-002` | Whether browser code may compute supply connectivity for highlighting. | Gate 13 spec and Rulepath behavior-authority law. | No. Rust computes and projects supplied/cut status. | `FC-SCORE-PROSPECTOR-SUPPLY`, `FC-VIS-001` | web smoke, manual review | resolved |
| `FC-AMB-003` | Whether clashes create reaction windows. | Gate 13 spec and reaction-window atlas row. | No. Clashes resolve immediately in the mover's command. | `FC-CTRL-002`, `FC-CTRL-003` | action/effect tests, trace | resolved |
| `FC-AMB-004` | Whether tied final scores are draws. | Gate 13 original rules sketch. | No. The Garrison wins tied final scores. | `FC-TERM-GARRISON-TIEBREAK` | terminal tests, outcome explanation smoke | resolved |
| `FC-AMB-005` | Whether map data may encode action, clash, or scoring formulas. | Rulepath static-data boundary and Gate 13 forbidden changes. | No. Data declares typed content only; Rust owns all behavior. | `FC-VAR-003`, `FC-SETUP-002`, `FC-SETUP-003` | strict-parse tests, fixture-check | resolved |

## Public naming rationale

Public ID: `frontier_control`

Display name: `Frontier Control`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | unclear for the category; original name chosen | Area-control and asymmetric-faction games carry commercial-presentation risk, so public naming avoids known product titles. |
| neutral name chosen? | yes | `Frontier Control` describes an original Rulepath premise without invoking a specific source game. |
| trademark risk considered? | yes | Public docs and UI avoid names from Root, Risk, El Grande, Small World, Kemet, Blood Rage, A Game of Thrones: The Board Game, Twilight Struggle, and similar commercial games. |
| trade-dress risk considered? | yes | Public presentation must avoid copied map layouts, faction boards, influence cubes, woodland clearings, conquest-track treatments, component shapes, product palettes, and source vocabulary. |
| affiliation implication avoided? | yes | Sources are cited only as project authority and IP-boundary context. Public UI must not imply compatibility, inspiration branding, or affiliation. |
| public help text needs disclaimer? | no | Neutral naming and source notes are sufficient unless later review requests additional text. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Area-control mechanic similarity | low for abstract mechanics, expression still reviewed | Use original rules prose, original names, a compact original graph, and no source visuals. | no |
| Root-like woodland faction presentation | medium if woodland creatures, clearings, suit symbols, faction-board layout, or source vocabulary appear | Avoid woodland-creature theming, the word `clearing`, source faction names, and any source-like board/faction presentation. | yes if source-confusing presentation appears |
| Risk-style conquest-map trade dress | medium if world-map/territory conquest presentation, army markers, or source map language appears | Use an abstract frontier graph with original site labels and no world-conquest framing. | yes if source-confusing presentation appears |
| El Grande / Small World / Kemet / Blood Rage / AGoT-style influence or faction presentation | medium if copied region layouts, faction powers, iconography, or distinctive names appear | Keep components abstract and original: guards, crews, stakes, forts, trails. | yes if source-confusing presentation appears |
| Twilight Struggle-like conflict-track/political presentation | low unless copied geopolitical framing appears | Avoid real-world political conflict framing and source-specific terminology. | yes if source-confusing presentation appears |
| Role/faction/component text | high if copied from any commercial game | Garrison, Prospectors, site labels, and action labels are original local labels. | yes if copied text appears |
| Visual trade dress | medium if copied boards, cards, icons, screenshots, fonts, component layout, or marketing copy appear | Use original SVG treatment and no source screenshots/scans/icons/fonts. | yes if copied or source-confusing |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---:|---|---|---:|
| Game docs | `games/frontier_control/docs/RULES.md`, `games/frontier_control/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied source rules, examples, roles, or tables. | yes |
| Public game name | `Frontier Control` | original project name | Rulepath/Codex-authored public name | Public name avoids known product branding. | yes |
| Faction labels | Garrison, Prospectors | original local labels | Rulepath/Codex-authored labels | Avoids named commercial factions and woodland/political/conquest source vocabulary. | yes |
| Site labels | Gatehouse, Signal Hill, Base Camp, Ford, Quarry, Timberline, Goldfield | original local labels | Rulepath/Codex-authored labels | Generic frontier-place language; no copied board location names. | yes |
| Action/component labels | guards, crews, stakes, forts, trails, march, patrol, reinforce, muster, dismantle, clash, supply | original local labels | Rulepath/Codex-authored labels | Functional, neutral, and game-local. | yes |
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
| Rulepath original rules summary | yes | `games/frontier_control/docs/RULES.md` | Original Rulepath prose. |
| Project-authority Gate 13 facts | yes | `games/frontier_control/docs/SOURCES.md` | Summarized as rationale only. |
| Abstract graph/control/asymmetry mechanic context | yes | `games/frontier_control/docs/SOURCES.md` | Discussed as project design context, not copied expression. |
| Public source prose, examples, faction names, component tables, or board layouts | no | none | No external source prose or layout is copied. |
| Commercial/licensed rules text | no | none | No source material is redistributed. |
| Commercial board art, faction boards, cards, icons, screenshots, scans, fonts, or table presentation | no | none | No assets introduced in this ticket. |
| Private licensed stress-test content | no public shipment | none | No private licensed content is involved. |

Private licensed stress tests are late, isolated, optional, non-public, and
non-architectural. They must not contaminate `engine-core`, public assets,
public docs, or public web bundles.

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Public name is neutral and original. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | Local labels only; no source card or component text. |
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
rationale support here. Frontier Control is original, so the primary support is
the Gate 13 spec plus the design rationale summarized below.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `FC-VAR-001` through `FC-VAR-003` | Standard and Highlands variants, typed constants, and no behavior-in-data. | Gate 13 graph-map and static-data proof plus Rulepath static-data boundary. | yes | Variant data remains content only. |
| `FC-COMP-001` through `FC-COMP-012` | Seats, factions, sites, trails, guards, crews, forts, stakes, clash, supply, budget, and score track. | Gate 13 component and architecture proof. | no | Vocabulary remains game-local. |
| `FC-SETUP-001` through `FC-SETUP-005` | Two-seat setup, map constants, graph validation, deterministic start units, and initial public state. | Gate 13 deterministic setup and replay-stability requirements. | no | No game-rule randomness exists. |
| `FC-TURN-001` through `FC-TURN-008` | Faction action phases, waiting state, budget exhaustion, end turn, round scoring, cleanup, and terminal state. | Gate 13 multi-action budget and asymmetric-turn proof. | yes | No synthetic scoring actor. |
| `FC-ACT-001` through `FC-ACT-011` | Rust-owned march, stake, muster, patrol, reinforce, dismantle, end-turn, waiting, and terminal action-tree rules. | Gate 13 behavior-authority and legal-only UI requirements. | no | TypeScript computes no legality. |
| `FC-RESTRICT-001` through `FC-RESTRICT-008` | Wrong actor, wrong seat, unavailable path, stale command, invalid movement, invalid occupancy, action precondition, and terminal restrictions. | Rulepath diagnostic/replay invariants plus Gate 13 spec. | no | Reject without mutation. |
| `FC-CTRL-001` through `FC-CTRL-005` | Movement adjacency, asymmetric clashes, settled control, and stake persistence. | Gate 13 graph/control/asymmetry proof and original rules design. | yes | Effects and traces must preserve order. |
| `FC-SCORE-GARRISON-FORT`, `FC-SCORE-PROSPECTOR-SUPPLY`, `FC-SCORE-ACTION-BUDGET`, `FC-SCORE-STAKE-VALUE`, `FC-SCORE-COMPARABLE-TRACK` | Fort scoring, supplied-stake scoring, budget accounting, map values, and comparable final track. | Gate 13 scoring proof and outcome-explanation requirement. | yes | Outcome explanation cites these IDs. |
| `FC-TERM-SCORE-COMPARE`, `FC-TERM-GARRISON-TIEBREAK`, `FC-TERM-NO-ACTIONS` | Higher final score wins, Garrison tied-score tiebreak, and terminal no-action posture. | Gate 13 terminal requirements and original tiebreak design. | yes | Outcome explanation cites these IDs. |
| `FC-VIS-001` through `FC-VIS-004` | Public facts, map constants, bot-rationale limits, and hidden-info not-applicable posture. | Gate 13 perfect-information scope and Rulepath no-leak invariant. | yes | Browser-facing surfaces remain public-safe. |
| `FC-RNG-001` through `FC-RNG-004` | No game-rule randomness, deterministic map setup, scoring as command consequence, and stable serialization. | Gate 13 replay/export requirements. | no | Bot RNG is separate infrastructure. |
| `FC-BOT-001` through `FC-BOT-003` | Random legal bot, Garrison Level 1 public-information policy, and Prospector Level 1 public-information policy. | Gate 13 per-faction bot baseline and Rulepath public-bot law. | no | Solver, sampling, and learning bots are forbidden. |
| `FC-AMB-001` through `FC-AMB-005` | Chosen resolutions for graph helpers, TS connectivity, reaction windows, tiebreak, and no behavior-in-data. | Gate 13 scope and Rulepath foundation constraints. | yes | Tests/traces must preserve these decisions. |
| `FC-DEV-001` through `FC-DEV-003`, `FC-OOS-001` through `FC-OOS-007` | IP, scope, no-leak, no-randomness, and out-of-scope variants. | Gate 13 forbidden changes and IP policy. | yes | Prevents scope creep and source confusion. |

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
