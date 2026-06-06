# Column Four Sources

Game ID: `column_four`

Public display name: `Column Four`

Implemented variant: `column_four_standard`

Prepared by: `Codex`

Created: 2026-06-06

Last updated: 2026-06-06

Rules version connected to this source note: `column_four-rules-v1`

## Source-use statement

Rulepath uses consulted sources to verify rules, variants, terminology context,
public-history context, accessibility expectations, reduced-motion expectations,
and ambiguity resolution.

Sources do not authorize copied prose, component text, icons, screenshots,
scans, fonts, assets, board art, token art, packaging style, marketing language,
or trade dress. Rulepath rule prose, UI copy, visual presentation, assets,
icons, and component text for `column_four` are original.

No source prose, rules text, examples, assets, board images, token images,
screenshots, or presentation were copied into this game. The implemented game
is a neutral abstract vertical four-in-a-row column game with original Rulepath
naming and presentation.

## Consulted sources

All external sources in this table were consulted on 2026-06-06 for the Gate 5
specification and this source note.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Wikipedia: Connect Four | `https://en.wikipedia.org/wiki/Connect_Four` | 2026-06-06 | broad secondary | classic rule-family context | none | Used only as background for the common seven-column, six-row, vertical drop, four-in-a-row family shape; not used for public naming or prose. |
| Wikipedia: m,n,k-game | `https://en.wikipedia.org/wiki/M,n,k-game` | 2026-06-06 | broad secondary | variant comparison | none | Used only to mark generalized m/n/k configurability as out of scope. |
| U.S. Copyright Office: Games | `https://www.copyright.gov/register/tx-games.html` | 2026-06-06 | public agency guidance | IP posture | none | Used for the distinction between a game idea or method of play and protectable expressive text, art, or board/container presentation; this is not legal advice. |
| U.S. Copyright Office: What Does Copyright Protect? | `https://www.copyright.gov/help/faq/faq-protect.html` | 2026-06-06 | public agency guidance | IP posture | none | Used as general copyright background; this is not legal advice. |
| USPTO: Trademark basics | `https://www.uspto.gov/trademarks/basics` | 2026-06-06 | public agency guidance | trademark posture | none | Used only to reinforce neutral naming and no-affiliation posture; this is not legal advice. |
| WAI-ARIA Authoring Practices: Grid Pattern | `https://www.w3.org/WAI/ARIA/apg/patterns/grid/` | 2026-06-06 | standards guidance | keyboard interaction guidance | none | Used only if the UI uses an ARIA grid or roving-focus pattern. |
| WAI-ARIA Authoring Practices: Layout Grid Examples | `https://www.w3.org/WAI/ARIA/apg/patterns/grid/examples/layout-grids/` | 2026-06-06 | standards guidance | arrow-key focus guidance | none | Used only for practical focus behavior guidance. |
| WCAG 2.2: Target Size Minimum | `https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum` | 2026-06-06 | accessibility guidance | touch target baseline | none | Used as a UI acceptance reference, not as game-rule authority. |
| WCAG 2.1: Animation from Interactions | `https://www.w3.org/WAI/WCAG21/Understanding/animation-from-interactions` | 2026-06-06 | accessibility guidance | reduced-motion rationale | none | Used as a UI acceptance reference for motion triggered by play and replay controls. |
| MDN: `prefers-reduced-motion` | `https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion` | 2026-06-06 | technical reference | reduced-motion implementation guidance | none | Used for CSS implementation guidance only. |
| MDN: SVG `title` element | `https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/title` | 2026-06-06 | technical reference | SVG accessibility guidance | none | Used for accessible SVG short-description guidance. |
| MDN: SVG `desc` element | `https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/desc` | 2026-06-06 | technical reference | SVG accessibility guidance | none | Used for accessible SVG long-description guidance. |
| MDN: ARIA `img` role | `https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/img_role` | 2026-06-06 | technical reference | grouped image semantics guidance | none | Used only when grouped SVG/image semantics are appropriate. |
| Rulepath Gate 5 spec | `specs/gate-5-column-four-public-polish.md` | 2026-06-06 | project authority | product scope and evidence requirements | none | Governs `column_four` identity, rule scope, docs, tests, replay, bot, WASM, UI, benchmark, and archive obligations. |
| Rulepath Official Game Contract | `docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-06 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, and official-game evidence. |
| Rulepath IP Policy | `docs/IP-POLICY.md` | 2026-06-06 | project authority | naming and IP safety | none | Supports neutral public naming and forbids copied prose, assets, and trade dress. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | Two seats alternate placing their own pieces into non-full columns on a seven-column by six-row board. A piece lands in the lowest empty row of the selected column. The first seat to complete four contiguous cells horizontally, vertically, or diagonally wins. A full board with no line is a draw. | Chosen as the Gate 5 game because it proves gravity, column-first legality, line detection under gravity, public animation, replay projection, and deterministic bots without hidden information. | yes |
| player count | Exactly two seats. | Gate 5 scope and common vertical four-in-a-row family shape. | yes |
| first player | `seat_0` acts first. | Deterministic replay and Gate 5 assumptions. | yes |
| coordinate identity | Stable columns `c1` through `c7`, rows `r1` through `r6`, and cells `r1c1` through `r6c7`; rows count bottom to top. | Gate 5 assumes stable row/column cell IDs for rules, actions, effects, replay, and UI. | yes |
| optional rule included | Full-board draw when no line exists. | Required by Gate 5 exit criteria. | yes |
| optional rule excluded | PopOut or removal variant. | Gate 5 is placement-only after gravity. | yes |
| optional rule excluded | Misere, Five-in-a-Row, arbitrary m/n/k, custom sizes, alternate gravity, simultaneous turns, more than two seats, randomized setup, or hidden-information variants. | These variants would obscure the narrow public-polish goal and create premature abstraction pressure. | yes |
| Rulepath naming deviation | Public name is `Column Four`, not a trademark-forward or source-affiliated name. | Neutral project-owned naming avoids source confusion and keeps presentation original. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `CF-AMB-001` | Whether to use a common commercial name or a neutral Rulepath public name. | Gate 5 spec, IP policy, source notes. | Use `Column Four`. | `CF-SCOPE-002`, `CF-AMB-001` | UI smoke and docs review. | resolved |
| `CF-AMB-002` | Which seat places the first piece. | Gate 5 assumptions and deterministic replay needs. | `seat_0` places first. | `CF-SETUP-002`, `CF-AMB-002` | Setup test and replay/hash tests. | resolved |
| `CF-AMB-003` | Which cell ID convention to use. | Gate 5 assumptions and UI/replay needs. | Use `r1c1` through `r6c7`, with rows bottom-to-top and columns left-to-right. | `CF-COMP-002`, `CF-COMP-003`, `CF-COMP-004`, `CF-AMB-003` | Action-id stability tests, rule tests, trace tests, UI smoke. | resolved |
| `CF-AMB-004` | Whether a filled board with a line should be a draw. | Gate 5 rule scope and terminal clarity. | Line win takes precedence; draw requires a full board with no line. | `CF-SCORE-001`, `CF-END-001` through `CF-END-005`, `CF-AMB-004` | Terminal rule tests and traces. | resolved |
| `CF-AMB-005` | Which line to expose when one placement creates multiple lines. | Gate 5 replay/UI requirements. | Expose a deterministic primary line: horizontal, vertical, rising diagonal, falling diagonal; within a direction choose the lexicographically first ordered cell-id list. | `CF-END-006`, `CF-AMB-005` | Multi-line terminal test and golden trace. | resolved |

## Public naming rationale

Public ID: `column_four`

Display name: `Column Four`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | yes | The name describes a neutral abstract column connection goal and is not source-affiliated. |
| neutral name chosen? | yes | Avoids commercial or trademark-forward framing. |
| trademark risk considered? | yes | No commercial title, logo, slogan, or affiliation is used in product surfaces. |
| trade-dress risk considered? | yes | The public board, pieces, colors, labels, packaging-like copy, and layout must be original Rulepath presentation. |
| affiliation implication avoided? | yes | Source notes and public copy do not imply affiliation with any rules source or commercial publisher. |
| public help text needs disclaimer? | no | No trademark-forward or brand-adjacent public name is used. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Vertical four-in-a-row mechanics | low for mechanics, human review for expression | Implement neutral mechanics with original prose and visuals. | no |
| Commercial name, logo, or branding | high if copied | Use only `Column Four`, `column_four`, `seat`, `piece`, `column`, and stable coordinate labels in public product surfaces. | yes if found |
| Blue rack / red-yellow disc commercial presentation resemblance | medium to high if imitated | Use original Rulepath visual design, not copied boards, colors, proportions, token styling, packaging, screenshots, icons, fonts, or layouts. | yes if found |
| Common rule terminology | low | Use project-local wording and stable IDs; do not copy rulebook examples or marketing phrases. | no |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| Game docs | `games/column_four/docs/RULES.md`, `games/column_four/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied rulebook text. | yes |
| Game UI/assets | none in this ticket | not applicable | none | UI/assets are out of scope for GAT5COLFOUPUB-001. | yes |

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
| Rulepath original rules summary | yes | `games/column_four/docs/RULES.md` | Original Rulepath prose. |
| public-domain/classic rule facts | yes | `games/column_four/docs/SOURCES.md` | Summarized as family context only. |
| commercial/licensed rules text | no | none | No commercial rules text is used. |
| private licensed stress-test content | no public shipment | none | No private licensed content is involved. |
| source screenshots/scans | no | none | No screenshots/scans are used. |

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Neutral public name only; cited commercial context does not enter product framing. |
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
| none identified for GAT5COLFOUPUB-001 | no | Continue evidence workflow in later tickets. | Rulepath |

## Rule-source-to-rule-ID cross-reference

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `CF-SCOPE-001` | Deterministic two-seat vertical four-in-a-row game. | Gate 5 spec and classic rule-family context. | no | Public-polish official game role. |
| `CF-SCOPE-002` | Public name is `Column Four`. | `CF-AMB-001` and IP policy. | yes | Neutral naming. |
| `CF-SCOPE-003` | Rust owns behavior and TypeScript presents. | Foundation docs. | no | Architecture invariant. |
| `CF-VAR-001` | Single shipped variant is `column_four_standard`. | Gate 5 spec. | no | No variant picker in Gate 5. |
| `CF-VAR-002` | Seven columns, six rows, two seats, alternating turns, column placement, four-line wins, full-board draws. | Gate 5 spec and classic rule-family context. | no | Selected variant. |
| `CF-COMP-001` | Board. | Gate 5 scope. | no | Game-local noun only. |
| `CF-COMP-002` | Stable columns `c1` through `c7`. | `CF-AMB-003`. | yes | Stable action/replay/UI target IDs. |
| `CF-COMP-003` | Stable rows `r1` through `r6`, bottom to top. | `CF-AMB-003`. | yes | Gravity-readable coordinate convention. |
| `CF-COMP-004` | Stable cells `r1c1` through `r6c7`. | `CF-AMB-003`. | yes | Stable effect/replay/UI target IDs. |
| `CF-COMP-005` | Seat-owned pieces. | Gate 5 spec. | no | Original visual tokens required later. |
| `CF-COMP-006` | Two seats. | Gate 5 spec. | no | `seat_0`, `seat_1`. |
| `CF-COMP-007` | Winning lines. | Gate 5 spec and classic rule-family context. | no | Game-local line detection only. |
| `CF-SETUP-001` | Board starts empty. | Gate 5 spec and classic rule-family context. | no | Deterministic setup. |
| `CF-SETUP-002` | `seat_0` starts. | `CF-AMB-002`. | yes | Deterministic replay. |
| `CF-SETUP-003` | Perfect-information public setup. | Gate 5 assumptions. | no | No hidden state. |
| `CF-TURN-001` | Active seat chooses one legal column. | Gate 5 spec and classic rule-family context. | no | Rust legal action tree. |
| `CF-TURN-002` | Non-terminal placement passes turn. | Gate 5 spec and classic rule-family context. | no | Alternation. |
| `CF-TURN-003` | Terminal placement ends normal turn sequence. | Gate 5 spec. | no | No post-terminal normal action. |
| `CF-ACTION-001` | Legal placements target non-full columns. | Gate 5 spec and classic rule-family context. | no | Rust-generated. |
| `CF-ACTION-002` | Full columns are not legal targets. | Gate 5 spec. | no | UI must consume Rust legality. |
| `CF-ACTION-003` | Terminal states expose no normal placements. | Gate 5 spec. | no | Replay/view preserves outcome. |
| `CF-RESTRICT-001` | Unknown column rejected. | Gate 5 action model. | no | Viewer-safe diagnostic. |
| `CF-RESTRICT-002` | Full column rejected. | Gate 5 spec. | no | Dev/API fail-closed path. |
| `CF-RESTRICT-003` | Stale, malformed, wrong-seat, terminal submissions rejected. | Foundation docs and Gate 5 spec. | no | State unchanged. |
| `CF-RESTRICT-004` | Browser does not own legality or derived rules. | Foundation docs and Gate 5 spec. | no | Boundary guard. |
| `CF-PLACE-001` | Valid placement creates one piece. | Gate 5 spec. | no | Occupancy mutation. |
| `CF-GRAVITY-001` | Piece lands in the lowest empty row. | Gate 5 spec and classic rule-family context. | no | Rust-owned gravity. |
| `CF-SCORE-001` | No score beyond winner or draw. | Gate 5 scope. | no | Line win before draw. |
| `CF-SCORE-002` | Ply count advances only on valid placement. | Replay and rule-test needs. | no | Max forty-two plies. |
| `CF-END-001` | Horizontal four-line wins. | Gate 5 spec and classic rule-family context. | no | Terminal rule. |
| `CF-END-002` | Vertical four-line wins. | Gate 5 spec and classic rule-family context. | no | Terminal rule. |
| `CF-END-003` | Rising diagonal four-line wins. | Gate 5 spec and classic rule-family context. | no | Terminal rule. |
| `CF-END-004` | Falling diagonal four-line wins. | Gate 5 spec and classic rule-family context. | no | Terminal rule. |
| `CF-END-005` | Full board with no line is a draw. | Gate 5 spec and classic rule-family context. | no | Draw condition. |
| `CF-END-006` | Multiple completed lines use deterministic primary-line tie-break. | `CF-AMB-005`. | yes | Stable replay and UI highlight. |
| `CF-END-007` | Terminal outcome remains unchanged after terminal. | Gate 5 spec. | no | Rejection path. |
| `CF-VIS-001` | All game state is public. | Gate 5 assumptions. | no | Perfect information. |
| `CF-VIS-002` | Private-view status is not applicable. | Gate 5 assumptions. | no | No hidden state. |
| `CF-RNG-001` | Setup and rules use no randomness. | Gate 5 scope. | no | Bot randomness is external. |
| `CF-RNG-002` | Replay binds setup, commands, effects, action trees, views, and projections. | Gate 5 spec. | no | Later replay evidence. |
| `CF-AMB-001` | Neutral name selected. | IP policy. | yes | `Column Four`. |
| `CF-AMB-002` | First seat selected. | Deterministic replay needs. | yes | `seat_0`. |
| `CF-AMB-003` | Cell ID convention selected. | Gate 5 spec. | yes | `r1c1` through `r6c7`. |
| `CF-AMB-004` | Win/draw precedence selected. | Terminal clarity. | yes | Line win before draw. |
| `CF-AMB-005` | Primary-line tie-break selected. | Replay and UI stability. | yes | Deterministic highlight. |
| `CF-VAR-003` | Related variants and commercial presentation are not in scope. | Gate 5 spec and IP policy. | yes | Narrow public-polish scope. |
| `CF-VAR-004` | PopOut/removal excluded. | Gate 5 spec. | yes | Placement-only gate. |
| `CF-VAR-005` | Misere/larger/generalized/custom/alternate variants excluded. | Gate 5 spec. | yes | No premature abstraction. |

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
