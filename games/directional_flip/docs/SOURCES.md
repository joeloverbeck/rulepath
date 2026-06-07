# Directional Flip Sources

Game ID: `directional_flip`

Public display name: `Directional Flip`

Implemented variant: `directional_flip_standard`

Prepared by: `Codex`

Created: 2026-06-07

Last updated: 2026-06-07

Rules version connected to this source note: `directional_flip-rules-v1`

## Source-use statement

Rulepath uses consulted sources to verify rules, variants, terminology context,
strategy context, accessibility expectations, public-history context, IP
caution, and ambiguity resolution.

Sources do not authorize copied prose, component text, icons, screenshots,
scans, fonts, assets, board art, token art, packaging style, marketing language,
or trade dress. Rulepath rule prose, UI copy, visual presentation, assets,
icons, and component text for `directional_flip` are original.

No Othello-branded prose, diagrams, palettes, icons, screenshots, scans, board
presentation, token presentation, rulebook examples, or trade dress are copied
into this game. The implemented game is a neutral abstract directional-flipping
game with original Rulepath naming and presentation.

## Consulted sources

All external sources in this table were consulted for the Gate 6 specification
and this source note.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| World Othello Federation official rules | `https://www.worldothello.org/about/about-othello/othello-rules/official-rules/english` | 2026-06-06 | official rules | rule-family verification and IP caution | none | Used only to verify the common 8 by 8 directional-bracketing family shape: four center discs, first-player move, legal bracketing, all qualifying lines flipping, no-move pass, terminal no-move state, and count winner. |
| World Othello Federation Othello information | `https://www.worldothello.org/about/wof-council-committees/partners-sponsors/othello-information` | 2026-06-06 | organization information | trademark/license caution | none | Used only to reinforce neutral naming and no-affiliation posture. |
| Justia OTHELLO trademark record | `https://trademarks.justia.com/730/61/othello-73061971.html` | 2026-06-06 | public trademark record | trademark caution | none | Used only as a caution that public naming and presentation must avoid Othello branding. |
| Othello Belgium strategy tips | `https://en.othellobelgium.be/leer-othello/tips-en-strategie` | 2026-06-06 | strategy reference | Level 2-lite strategy context | none | Used only for non-authoritative bot strategy ideas such as mobility, corners, stability, frontier exposure, and X/C-square caution. |
| Nederlandse Othello Vereniging strategy guide | `https://www.othello.nl/content/guides/comteguide/strategy.html` | 2026-06-06 | strategy reference | Level 2-lite strategy context | none | Used only for non-authoritative bot strategy ideas; not rule authority. |
| WAI-ARIA Authoring Practices: Grid Pattern | `https://www.w3.org/WAI/ARIA/apg/patterns/grid/` | 2026-06-06 | standards guidance | keyboard grid interaction guidance | none | Used only if the UI uses an ARIA grid or roving-focus pattern. |
| WCAG 2.2: Use of Color | `https://www.w3.org/TR/WCAG22/#use-of-color` | 2026-06-06 | standards guidance | non-color-only state encoding | none | Used as a UI acceptance reference, not as game-rule authority. |
| WCAG 2.2: Animation from Interactions | `https://www.w3.org/TR/WCAG22/#animation-from-interactions` | 2026-06-06 | standards guidance | reduced-motion rationale | none | Used as a UI acceptance reference for motion triggered by play and replay controls. |
| ReversiWorld bitboard move generation | `https://reversiworld.wordpress.com/2013/11/05/generating-moves-using-bitboard/` | 2026-06-06 | technical background | directional-scan pressure context | none | Used only to confirm directional scanning is real implementation pressure; Rulepath foundations override pressure toward opaque or generic engine abstractions. |
| Rulepath Gate 6 spec | `../../../../specs/gate-6-directional-flip.md` | 2026-06-07 | project authority | product scope and evidence requirements | none | Governs `directional_flip` identity, rule scope, docs, tests, replay, bot, WASM, UI, benchmark, primitive-pressure, and archive obligations. |
| Rulepath Official Game Contract | `../../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-07 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, and official-game evidence. |
| Rulepath IP Policy | `../../../../docs/IP-POLICY.md` | 2026-06-07 | project authority | naming and IP safety | none | Supports neutral public naming and forbids copied prose, assets, and trade dress. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `directional_flip_standard`: two seats play on an 8 by 8 board with four center discs; a legal placement brackets one or more contiguous opposing discs in a direct line; all bracketed discs in every qualifying direction flip; a no-move seat must take a forced pass; the game ends when neither seat can move or no continuation exists; higher final count wins and equal counts draw. | Gate 6 spec and directional-flipping rule-family research. | yes |
| player count | Exactly two seats, `seat_0` and `seat_1`. | Gate 6 scope and deterministic replay needs. | yes |
| first player | `seat_0` acts first. | Gate 6 setup requirement. | yes |
| coordinate identity | Stable cells `r1c1` through `r8c8`, with row 1 at the top and column 1 at the left in the default public view. | Gate 6 setup requirement and existing stable cell-id style. | yes |
| optional rule included | Explicit Rust-generated forced pass when the active seat has no legal placement. | Gate 6 replay/action-tree requirement. | yes |
| optional rule excluded | Alternate sizes, alternate openings, handicaps, clocks, scored-empty-square variants, custom setup variants, and tournament-specific procedures. | Gate 6 narrow official-game scope. | yes |
| Rulepath naming deviation | Public name is `Directional Flip`, not `Othello`, `Reversi`, or any source-affiliated name. | IP policy, trademark caution, and neutral naming requirement. | yes |
| out-of-scope variant | Advanced search/learning bots and generic tabletop abstractions. | Foundation docs and Gate 6 scope. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `DF-AMB-001` | Whether to use a common commercial or family name. | Gate 6 spec, IP policy, WOF trademark/license context, Justia trademark record. | Use `Directional Flip` and `directional_flip`. | `DF-IP-001` | Public docs and UI review. | resolved |
| `DF-AMB-002` | Which coordinate orientation to use. | Gate 6 spec, existing `three_marks` cell-id style. | Use `r1c1` through `r8c8`, row 1 top and column 1 left. | `DF-SETUP-001` | Setup, action-id, trace, and UI tests. | resolved |
| `DF-AMB-003` | Whether no-move turns are automatic or explicit. | Gate 6 spec and replay/action-tree needs. | Use an explicit Rust-generated forced-pass action. | `DF-ACTION-002`, `DF-ACTION-003`, `DF-PASS-001`, `DF-PASS-002` | Forced-pass and double-pass tests/traces. | resolved |
| `DF-AMB-004` | Which flips apply when a placement brackets in multiple directions. | Gate 6 spec and rule-family research. | Flip every bracketed disc in every qualifying direction. | `DF-FLIP-001`, `DF-FLIP-004`, `DF-PREVIEW-001`, `DF-EFFECT-002` | Rule tests, effect tests, preview/apply equality tests, golden trace. | resolved |
| `DF-AMB-005` | How to order grouped flip effects. | Gate 6 replay/hash and UI-effect requirements. | Use north, northeast, east, southeast, south, southwest, west, northwest; within each direction nearest to farthest. | `DF-FLIP-004`, `DF-EFFECT-002` | Effect order tests and replay/hash evidence. | resolved |

## Public naming rationale

Public ID: `directional_flip`

Display name: `Directional Flip`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | yes | The name describes the neutral action pattern without using a source-affiliated or trademark-forward title. |
| neutral name chosen? | yes | `Directional Flip` is original Rulepath naming for the implemented variant. |
| trademark risk considered? | yes | WOF and trademark-record sources motivate avoiding Othello-branded names, logos, slogans, and affiliation language. |
| trade-dress risk considered? | yes | Public board, discs, colors, labels, packaging-like copy, and layout must be original Rulepath presentation. |
| affiliation implication avoided? | yes | Source notes and public copy do not imply affiliation with any rules source, federation, publisher, or trademark owner. |
| public help text needs disclaimer? | no | No trademark-forward public name is used; a neutral source note is enough unless later review requests more. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Directional-flipping mechanics | low for mechanics, human review for expression | Implement neutral mechanics with original prose and visuals. | no |
| Othello name, marks, federation/publisher branding, logos, or slogans | high if copied | Use only `Directional Flip`, `directional_flip`, seat labels, disc labels, and stable coordinate labels in public product surfaces. | yes if found |
| Othello/Reversi-adjacent board/disc trade dress | medium to high if imitated | Use original Rulepath visual design, not copied boards, colors, proportions, token styling, packaging, screenshots, icons, fonts, or layouts. | yes if found |
| Common rule terminology | low | Use project-local wording and stable IDs; do not copy rulebook examples or marketing phrases. | no |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| Game docs | `games/directional_flip/docs/RULES.md`, `games/directional_flip/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied rulebook text. | yes |
| Game UI/assets | none in this ticket | not applicable | none | UI/assets are out of scope for GAT6DIRFLI-001. | yes |

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
| Rulepath original rules summary | yes | `games/directional_flip/docs/RULES.md` | Original Rulepath prose. |
| public-domain/classic rule facts | yes | `games/directional_flip/docs/SOURCES.md` | Summarized as family context only. |
| commercial/licensed rules text | no | none | No commercial rules text is used. |
| private licensed stress-test content | no public shipment | none | No private licensed content is involved. |
| source screenshots/scans | no | none | No screenshots/scans are used. |

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Neutral public name only; cited commercial/trademark context does not enter product framing. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | No cards or protected component text. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no | No art or UI asset in this ticket. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no | None. |
| uncertainty about public-domain status | no | Sources are used only as context; the implemented rules are original. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release blocking concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| none identified for GAT6DIRFLI-001 | no | Continue evidence workflow in later tickets. | Rulepath |

## Rule-source-to-rule-ID cross-reference

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `DF-SETUP-001` | Standard 8 by 8 setup has the correct four center discs and active first seat. | Gate 6 spec and directional-flipping rule-family research. | yes | Coordinate orientation resolved in `DF-AMB-002`. |
| `DF-ACTION-001` | Action tree exposes only legal placement choices when placements exist. | Gate 6 spec and Rust-ownership boundary. | no | TypeScript presents only. |
| `DF-ACTION-002` | Action tree exposes exactly one forced-pass choice when no placement exists and state is nonterminal. | Gate 6 spec and replay/action-tree needs. | yes | Explicit forced pass resolved in `DF-AMB-003`. |
| `DF-ACTION-003` | Pass is absent when any legal placement exists. | Gate 6 spec. | no | Rust decides pass availability. |
| `DF-LEGAL-001` | Placement requires at least one bracketed contiguous opposing line. | Directional-flipping rule-family research and Gate 6 spec. | no | Core legal placement rule. |
| `DF-LEGAL-002` | Occupied target is rejected. | Gate 6 validation requirements. | no | Viewer-safe diagnostic. |
| `DF-LEGAL-003` | Non-flipping target is rejected. | Gate 6 validation requirements. | no | Empty is insufficient. |
| `DF-LEGAL-004` | Out-of-bounds or malformed cell is rejected. | Gate 6 validation requirements and stable coordinate contract. | yes | Coordinate orientation resolved in `DF-AMB-002`. |
| `DF-LEGAL-005` | Stale command is rejected. | Foundation docs and engine freshness patterns. | no | State unchanged. |
| `DF-LEGAL-006` | Non-active actor is rejected. | Foundation docs and turn ownership. | no | State unchanged. |
| `DF-FLIP-001` | All bracketed discs in every qualifying direction flip. | Directional-flipping rule-family research and Gate 6 spec. | yes | Multi-direction handling resolved in `DF-AMB-004`. |
| `DF-FLIP-002` | No skipped own discs are used to create flips. | Gate 6 spec. | no | Contiguity rule. |
| `DF-FLIP-003` | No indirect or non-line discs flip. | Gate 6 spec. | no | No area-fill behavior. |
| `DF-FLIP-004` | Flip order is stable and documented. | Replay/hash determinism and UI effect needs. | yes | Order resolved in `DF-AMB-005`. |
| `DF-PREVIEW-001` | Preview flip set equals apply flip set. | Gate 6 preview/effect requirement. | no | Rust supplies previews. |
| `DF-PASS-001` | Forced pass advances turn through normal command path. | Gate 6 replay/action-tree requirement. | yes | Explicit forced pass resolved in `DF-AMB-003`. |
| `DF-PASS-002` | Double forced pass ends the game. | Gate 6 terminal requirement. | yes | Terminal no-move proof. |
| `DF-TERM-001` | Terminal action tree has no legal placement/pass choices. | Gate 6 terminal requirement. | no | Empty legal action surface. |
| `DF-SCORE-001` | Higher disc count wins. | Directional-flipping rule-family research and Gate 6 spec. | no | Final count outcome. |
| `DF-SCORE-002` | Equal disc count draws. | Directional-flipping rule-family research and Gate 6 spec. | no | Draw outcome. |
| `DF-EFFECT-001` | Placement emits accepted/place/grouped-flip/turn/terminal effects as applicable. | Gate 6 semantic-effect requirement. | no | Exact effect names may follow repo style. |
| `DF-EFFECT-002` | Grouped flip child entries match deterministic order. | Gate 6 semantic-effect and replay/hash requirements. | yes | Order resolved in `DF-AMB-005`. |
| `DF-VIEW-001` | Public view contains no hidden/internal state. | Foundation docs and perfect-information game scope. | no | No hidden state exists. |
| `DF-REPLAY-001` | Replay/hash deterministic across export/import/step/reset. | Foundation docs and Gate 6 replay requirement. | no | Deterministic setup and ordered effects. |
| `DF-SER-001` | Unknown fields and behavior-looking fields are rejected. | Foundation docs and trace schema policy. | no | Serialization must not own behavior. |
| `DF-BOT-001` | Level 0 random bot validates through command path. | AI policy and Gate 6 bot scope. | no | Legal API is authoritative. |
| `DF-BOT-002` | Level 2-lite validates through command path and remains deterministic. | AI policy, strategy sources, and Gate 6 bot scope. | no | No search/playout/learning/LLM move selection. |
| `DF-UI-001` | UI uses Rust legal choices/previews/effects and no TypeScript legality. | Foundation docs, UI boundary docs, and Gate 6 UI scope. | no | Browser presentation only. |
| `DF-UI-002` | Keyboard grid, forced pass control, reduced motion, and non-color-only state encoding pass smoke. | WAI-ARIA grid guidance and WCAG references. | no | UI evidence lands in later tickets. |
| `DF-IP-001` | Public presentation is neutral and original. | IP policy, WOF trademark/license context, Justia trademark record. | yes | Neutral naming resolved in `DF-AMB-001`. |
| `DF-PRIM-001` | Primitive-pressure ledger decision completed. | Foundation docs, mechanic atlas, and Gate 6 spec. | no | Governance evidence lands in GAT6DIRFLI-002. |

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
