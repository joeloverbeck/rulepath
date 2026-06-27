# Starbridge Crossing Sources

Game ID: `starbridge_crossing`

Public display name: `Starbridge Crossing`

Implemented variant: `starbridge_crossing_classic_star_v1`

Prepared by: `Codex`

Created: 2026-06-27

Last updated: 2026-06-27

Rules version connected to this source note: `starbridge-crossing-rules-v1`

## Source-Use Statement

Starbridge Crossing is an original Rulepath implementation in the Star Halma /
Chinese Checkers rules family. External references were consulted only to
verify public rules-family facts and implementation-relevant variant choices:
the 121-space star board, common supported seat counts, 10-piece homes,
opposite-home objective, single-step movement, jump-chain movement, no capture,
base-blocking ambiguity, coordinate/topology modeling, and accessible dense-board
presentation.

No source rules prose, examples, board diagrams, peg imagery, product naming,
component text, icons, screenshots, scans, fonts, assets, art direction, table
layout, or trade dress is copied. Rulepath rule prose, UI copy, visual
presentation, assets, icons, space ids, peg ids, and component text for
`starbridge_crossing` are original.

Public presentation must use **Starbridge Crossing**. "Chinese Checkers",
"Star Halma", "Stern-Halma", and "Halma" may appear only as rules-family or
source-history labels in source notes and explanatory maintenance context. They
must not be used as the public product identity, renderer identity, asset theme,
or trade-dress target.

Human IP/public-release review is pending. Any title screening, asset review,
or source-use review recorded before release is maintenance evidence only, not
legal clearance.

## Consulted Sources

All sources in this table are rationale, project-authority, rules-family,
accessibility, implementation-prior-art, or mechanic-context sources only. No
source prose, code, APIs, assets, or presentation are copied.

| Source ID | Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|---|
| `SC-SPEC` | Rulepath Gate 20 Starbridge Crossing spec | `../../../archive/specs/gate-20-starbridge-crossing-star-halma.md` | 2026-06-27 | project authority | product scope, locked variant, rule IDs, source tags, seat counts, docs, tests, replay, bot, WASM, UI, benchmark, and capstone obligations | none | Archived gate authority for this completed ticket series. |
| `SC-OGC` | Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-27 | project authority | requirements-first workflow and official-game evidence | none | Governs rules summary, source notes, player rules, rule coverage, outcome docs, no-leak proof, and web exposure. |
| `SC-IP` | Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-27 | project authority | naming, source-use limits, public asset caution, and release review posture | none | Supports the original public name and forbids copied prose, assets, and trade dress. |
| `SC-WIKI` | Wikipedia, "Chinese checkers" | `https://en.wikipedia.org/wiki/Chinese_checkers` | 2026-06-27 | public overview / family-history reference | Stern-Halma / Chinese Checkers history, 121-hole star board, 10 pieces, move/hop/no-capture facts, common 2/3/4/6 seats, and variant notes | none | Overview only; not project authority and not copied. |
| `SC-ACM` | ACM Hong Kong Chapter Computer Chinese Checkers Competition rules | `https://i.cs.hku.hk/~clyip/ACM/2005/CCC/ccc.html` | 2026-06-27 | computational rules reference | 121 positions, six-direction neighbor model, jump sequences, home/target areas, and numbered board representation | none | Used for implementation-shape comparison only. |
| `SC-BELL` | George I. Bell, "The Shortest Game of Chinese Checkers" | `https://arxiv.org/pdf/0803.1245` | 2026-06-27 | mathematical/computational analysis | 121-hole board, coordinate representation, 10 pieces, step/jump-chain rules, no capture, voluntary stop, and base-blocking discussion | none | Supports ambiguity notes and shortest-path context. |
| `SC-UCT` | Sturtevant et al., Chinese Checkers endgame / UCT paper | `https://www.cs.du.edu/~sturtevant/papers/UCT-endgame.pdf` | 2026-06-27 | computational strategy context | shortest-path, blocking, and endgame evaluation context | none | Negative boundary reminder: Rulepath public bots do not use MCTS/Monte Carlo. |
| `SC-ENV` | ArXiv 2405.18733 Chinese Checkers environment paper | `https://arxiv.org/pdf/2405.18733` | 2026-06-27 | computational environment reference | explicit turn limits, no-backtracking/cycle constraints, cube/axial coordinate masking | none | Informs Rulepath's finite action-tree guard and deterministic ply limit. |
| `SC-HEX` | Amit Patel, "Hexagonal Grids" | `https://www.redblobgames.com/grids/hexagons/` | 2026-06-27 | implementation reference | axial/cube coordinate modeling, neighbor/distance algorithms, and map-storage tradeoffs for hex-like boards | none | Coordinate technique reference only; no code imported. |
| `SC-A11Y-GRID` | WAI-ARIA Authoring Practices Grid Pattern | `https://www.w3.org/WAI/ARIA/apg/patterns/grid/` | 2026-06-27 | accessibility reference | keyboard/focus model for dense interactive surfaces | none | UI reference only; no component API imported. |
| `SC-WCAG` | WCAG 2.2 and Understanding SC 2.5.8 Target Size Minimum | `https://www.w3.org/TR/WCAG22/`, `https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html` | 2026-06-27 | accessibility reference | target-size and interaction baseline | none | UI acceptance reference. |
| `SC-SVG-A11Y` | Inclusive Learning Design Handbook, "SVG and Accessibility" | `https://handbook.floeproject.org/approaches/svg-and-accessibility/` | 2026-06-27 | accessibility reference | SVG titles/descriptions and keyboard-accessible alternatives | none | UI reference only. |

## Adopted Design Facts

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Public identity | The public game is Starbridge Crossing; family names are source labels only. | Original identity reduces source confusion and trade-dress risk. |
| Board | Exactly 121 stable spaces in a six-pointed star topology. | Family sources and Gate 20 scope align on the classic star board. |
| Seat counts | 2, 3, 4, and 6 seats are supported; default setup uses 2 seats. | Common family counts plus Gate 20's discontinuous variable-seat proof. |
| Seat labels | Six point labels in clockwise order: `north`, `north_east`, `south_east`, `south`, `south_west`, `north_west`. | Stable labels support setup, docs, renderer, and replay. |
| Pegs per seat | Each active seat starts with 10 pegs. | Classic family baseline selected by the spec. |
| Objective | Move all own pegs to the opposite home point. | Core family objective and Rulepath outcome model. |
| Turn order | One active seat at a time in deterministic clockwise order, skipping finished seats. | Rulepath active-seat model. |
| Step move | A peg may move to an adjacent empty space. | Core family movement. |
| Hop move | A peg may hop over one adjacent occupied space into the empty space beyond; no peg is captured. | Core family movement. |
| Hop chain | A hop chain may continue, change direction, and stop after any legal hop. | Chosen variant and action-tree contract. |
| Chain guard | A hop chain may not revisit a landing space during one turn. | Rulepath finite action-tree resolution. |
| Blocked pass | If no step or hop exists, Rust exposes forced `pass_blocked`. | Replay/simulation explicitness. |
| Finish order | Ranks are assigned as seats finish; the match ends when all but one seat has finished. | Multi-seat standings requirement. |
| Turn limit | Default deterministic `max_plies` is 2000 for public simulations and benchmarks. | Safeguard for simulation and benchmark evidence. |
| Visibility | Every board fact is public to every viewer. | Starbridge Crossing has no hidden-information class. |

## Variant Choice And Deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `starbridge_crossing_classic_star_v1` / Starbridge Crossing | Gate 20 scope. | yes |
| player count | Exactly 2, 3, 4, and 6 seats; default 2 | Family baseline plus Rulepath variable-seat proof. | yes |
| unsupported 5 seats | Rejected by setup diagnostics | Gate 20 pins the common discontinuous set and explicitly excludes 5. | yes |
| piece count | 10 pegs per active seat | Classic family baseline. | yes |
| two-player 15-piece variant | Excluded | Keeps one fixture shape across all supported seat counts. | yes |
| partnerships | Excluded | Gate 20 proves individual competitive race only. | yes |
| square Halma board | Excluded | Source history only; no rectangular Halma variant is implemented. | yes |
| long-distance hop variants | Excluded | Gate 20 pins one-adjacent-occupied-space hop rules. | yes |
| capture | Excluded | Hopped pegs remain on board. | yes |
| repeated hop landing | Rejected within one turn | Rulepath deterministic finite action-tree resolution. | yes |
| blocked no-move | Forced `pass_blocked` only when no move exists | Simulation/replay resolution; not an optional strategic pass. | yes |
| finish model | Continue for full finish-order standings | Rulepath multi-seat outcome contract. | yes |
| public name | Starbridge Crossing | Original neutral Rulepath identity. | yes |

## Ambiguity Log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `SC-AMB-001` | Whether the official fixture should include 5 seats or unusual two-player variants. | Gate 20 spec, `SC-WIKI`, `SC-BELL`. | Support exactly 2, 3, 4, and 6 seats with 10 pegs each. | `SC-SETUP-001`, `SC-SETUP-005` | setup 2/3/4/6 and invalid-5 diagnostics | resolved |
| `SC-AMB-002` | Whether hops can span empty gaps or only one adjacent occupied space. | Gate 20 spec, `SC-WIKI`, `SC-ACM`, `SC-BELL`. | Hop over exactly one adjacent occupied space into the empty beyond-space. | `SC-MOVE-003`, `SC-MOVE-004` | one-hop and invalid-hop traces | resolved |
| `SC-AMB-003` | Whether a jump chain must continue. | Gate 20 spec, `SC-WIKI`, `SC-BELL`, `SC-ACM`. | The player may stop after any legal hop landing. | `SC-MOVE-005`, `SC-MOVE-006` | stop-midway trace | resolved |
| `SC-AMB-004` | How to keep hop-chain action trees finite. | Gate 20 spec, `SC-ENV`. | A move cannot revisit a landing space already used in that hop chain. | `SC-MOVE-007` | repeat-landing rejected trace | resolved |
| `SC-AMB-005` | Whether blocked positions create stalemate, optional pass, or forced pass. | Gate 20 spec, `SC-BELL`. | Rust supplies forced `pass_blocked` only when no step or hop exists. | `SC-MOVE-009` | blocked-pass trace | resolved |
| `SC-AMB-006` | Whether the game ends at first finisher or full standings. | Gate 20 spec and Rulepath multi-seat outcome contract. | Continue until all but one active seat has a finish rank. | `SC-FINISH-001` through `SC-FINISH-004` | finish-order and terminal standings traces | resolved |
| `SC-AMB-007` | Whether a natural turn limit exists. | Gate 20 spec, `SC-ENV`. | Add deterministic `max_plies`, default 2000. | `SC-FINISH-005`, `SC-FINISH-006` | turn-limit trace | resolved |

## Public Naming Rationale

Public ID: `starbridge_crossing`

Display name: `Starbridge Crossing`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | constrained | Family names are common descriptors but are not used as the Rulepath product name. |
| neutral name chosen? | yes | "Starbridge" evokes the star board and bridge-like hop chains; "Crossing" evokes the race to the opposite point. |
| trademark risk considered? | yes | Neutral original title avoids product-source confusion. Human title review remains pending before release. |
| trade-dress risk considered? | yes | Renderer/icon work must avoid existing board, peg, app, and packaging presentation mimicry. |
| affiliation implication avoided? | yes | Docs and UI must not imply affiliation with source sites, publishers, competitions, or apps. |

## Trademark And Trade-Dress Concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Public use of Chinese Checkers / Star Halma as product identity | medium | Use Starbridge Crossing for public identity; keep family labels in source notes and maintenance context. | no |
| Existing star-board diagrams, peg colors, package visuals, or app layouts | human review needed | Build original Rulepath SVG board and pegs; do not copy screenshots, board diagrams, icons, animations, color treatments, or layout. | yes before public release |
| Source phrasing | medium if paraphrased too closely | Maintain consulted-not-copied notes and original Rulepath phrasing. | yes if copied prose appears |
| Generated or custom icon assets | human review needed | Record prompt/review or project-authored SVG notes when asset lands. | yes before public release |

## Asset Provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---:|---|---|---:|
| Game docs | `games/starbridge_crossing/docs/RULES.md`, `games/starbridge_crossing/docs/SOURCES.md` | original | Rulepath/Codex-authored prose from project spec and summarized source facts | No copied rules text, examples, diagrams, or tables. | yes |
| Public game name | `Starbridge Crossing` | original project name | Rulepath/Codex-authored public name | Pending human release review. | pending |
| Space ids and peg labels | future game-local Rust/static metadata | original implementation expression over common topology facts | Rulepath/Codex-authored labels and IDs | Topology facts are common; rendered board/peg assets must still be original or reviewed. | pending later asset review |
| Game UI/assets | none in this ticket | not applicable | none | UI/assets land in later tickets. | yes |

## Generated Asset Review Notes

| Generated asset | Tool/model if known | Prompt/source inputs safe? | Similarity/trade-dress risk | Human review result | Public-safe? |
|---|---|---:|---|---|---:|
| none | not applicable | yes | none | No generated art/assets in this ticket. | yes |

## Font Status

| Font | Source/license | Bundled in public artifact? | Review status | Notes |
|---|---|---:|---|---|
| system font stack | not bundled | no | safe by default | No font files are introduced by this ticket. |

## Public/Private Content Boundary

| Content | Public allowed? | Location | Notes |
|---|---:|---|---|
| Rulepath original rules summary | yes | `games/starbridge_crossing/docs/RULES.md` | Original Rulepath prose. |
| Gate 20 project-authority facts | yes | `games/starbridge_crossing/docs/SOURCES.md` | Summarized as rationale only. |
| Generic Star Halma / Chinese Checkers family rules facts | yes | this note and later player help | Summarized, not copied. |
| Public source prose, examples, tables, diagrams, screenshots, or board art | no | none | No source prose or visual source material is copied. |
| Commercial/licensed rules text | no | none | No source material is redistributed. |
| Commercial board/peg art, product names, icons, screenshots, fonts, app layouts, or table presentation | no | none | No assets introduced in this ticket. |
| Private licensed stress-test content | no public shipment | none | No private licensed content is involved. |

Private licensed stress tests are late, isolated, optional, non-public, and
non-architectural. They must not contaminate `engine-core`, public assets,
public docs, or public web bundles.

## Human/Legal Review Triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no for current name; title still pending release review | Public name is original. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| board/component text from a protected source | no | Space and peg labels are original implementation identifiers. |
| scanned/copied art, icon, screenshot, board, peg, or UI asset | no in this ticket | Later renderer/icon work must review assets. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no in this ticket | Later generated assets require review notes. |
| uncertainty about public-domain status | no for abstract rules-family facts | No source material is redistributed. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release Blocking Concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| human IP/public-release review pending | yes before public release | Maintainer/human review of name, presentation, source use, assets, and release checklist. | Rulepath maintainers |
| visual/board/peg assets not yet reviewed | later-ticket blocker if introduced | Renderer/icon tickets must record asset provenance and trade-dress review. | Rulepath maintainers |
| public icon/board asset review | yes before release | Record original/project-owned/generated-review provenance in release checklist. | Rulepath maintainers |

## Rule-Source-To-Rule-ID Cross-Reference

Every release-relevant stable rule ID in `RULES.md` must have source or design
rationale support here. Starbridge Crossing uses public rules-family facts
expressed in original Rulepath prose, with project authority from the Gate 20
spec.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `SC-ID-001` through `SC-ID-002` | Identity, variant, public name, rules/data versions. | `SC-SPEC`, Rulepath IP policy. | yes | Public release review remains pending. |
| `SC-SETUP-001` through `SC-SETUP-006` | Supported seats, seat labels, active home sets, 121 spaces, 10 pegs, deterministic setup. | `SC-SPEC`, `SC-WIKI`, `SC-ACM`, `SC-BELL`, `SC-ENV`. | yes | 5 seats and 15-piece two-player variants are excluded. |
| `SC-VIS-001` through `SC-VIS-004` | All-public views, seat-viewer parity, no hidden-information class, public replay export. | Rulepath foundations, Gate 20 spec. | no | ADR 0004 is not applicable with rationale. |
| `SC-TURN-001` through `SC-TURN-003` | Active seat, one move per turn, finished-seat skipping. | Gate 20 spec and Rulepath multi-seat contract. | yes | Finished pegs remain public occupancy. |
| `SC-MOVE-001` through `SC-MOVE-009` | Steps, hops, hop chains, stop-anywhere, cycle guard, no mixed moves, forced blocked pass. | `SC-SPEC`, `SC-WIKI`, `SC-ACM`, `SC-BELL`, `SC-ENV`. | yes | Cycle guard and blocked pass are Rulepath resolutions. |
| `SC-FINISH-001` through `SC-FINISH-006` | Finish rank, full standings, terminal all-but-one-finished, turn-limit fallback. | Gate 20 spec, Rulepath multi-seat/outcome law, `SC-ENV`. | yes | Turn limit is deterministic safeguard. |
| `SC-REPLAY-001` through `SC-REPLAY-002` | Deterministic replay and Trace Schema v1 coverage. | Rulepath replay/fixture/hash law and Gate 20 spec. | no | No schema migration authorized. |
| `SC-BOT-001` through `SC-BOT-003` | L0 random legal, higher-bot evidence gate, prohibited algorithms. | Rulepath AI law, Gate 20 spec, `SC-UCT` as negative context. | yes | MCTS/Monte Carlo remain forbidden. |
| `SC-UI-001` through `SC-UI-003` | Rust-owned legal controls, 121-space accessible renderer, no TypeScript legality. | Rulepath UI law, Gate 20 spec, `SC-A11Y-GRID`, `SC-WCAG`, `SC-SVG-A11Y`. | no | Browser renders Rust-authored facts only. |

## Final Source/IP Checklist

- Sources are summarized in original language.
- Source quality and date consulted are recorded.
- Variant choices and deviations are explicit.
- Public identity is original and neutral.
- Rules-family labels are not product names.
- No source prose, examples, diagrams, code, board art, peg art, screenshots,
  scans, icons, fonts, or trade dress are copied.
- No public asset has been introduced by this ticket.
- Human IP/public-release review remains pending before external release.
