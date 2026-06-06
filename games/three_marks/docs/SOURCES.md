# Three Marks Sources

Game ID: `three_marks`

Public display name: `Three Marks`

Implemented variant: `three_marks_standard`

Prepared by: `Codex`

Created: 2026-06-06

Last updated: 2026-06-06

Rules version connected to this source note: `three_marks-rules-v1`

## Source-use statement

Rulepath uses consulted sources to verify rules, variants, terminology context,
public-history context, accessibility expectations, and ambiguity resolution.

Sources do not authorize copied prose, component text, icons, screenshots,
scans, fonts, assets, or trade dress. Rulepath rule prose, UI copy, visual
presentation, assets, icons, and component text for `three_marks` are original.

No source prose, rules text, examples, assets, board images, or presentation
were copied into this game. The implemented game is a neutral abstract
three-in-a-row placement game with original Rulepath naming and presentation.

## Consulted sources

All external sources in this table were consulted on 2026-06-06 for the Gate 4
specification and this source note.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Exploratorium: Tic-Tac-Toe puzzle page | `https://www.exploratorium.edu/explore/puzzles/tictactoe` | 2026-06-06 | educational secondary | classic rule confirmation | none | Used only to confirm the simple alternating 3 by 3 placement shape, three-in-a-row win, and filled-board draw concept. |
| Wikipedia: Tic-tac-toe | `https://en.wikipedia.org/wiki/Tic-tac-toe` | 2026-06-06 | broad secondary | classic rule scope and simple strategy background | none | Used only as background for common rule scope and non-authoritative priority ideas for a deterministic baseline bot. |
| U.S. Copyright Office: Games | `https://www.copyright.gov/register/tx-games.html` | 2026-06-06 | public agency guidance | IP posture | none | Used for the distinction between protectable expression and unprotected game mechanics or systems; this is not legal advice. |
| WAI-ARIA Authoring Practices: Grid Pattern | `https://www.w3.org/WAI/ARIA/apg/patterns/grid/` | 2026-06-06 | standards guidance | keyboard interaction guidance | none | Used only if the UI uses an ARIA grid or roving-focus pattern. |
| WAI-ARIA Authoring Practices: Layout Grid Examples | `https://www.w3.org/WAI/ARIA/apg/patterns/grid/examples/layout-grids/` | 2026-06-06 | standards guidance | arrow-key focus guidance | none | Used only for practical focus behavior guidance. |
| WCAG 2.2: Target Size Minimum | `https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html` | 2026-06-06 | accessibility guidance | touch target baseline | none | Used as a UI acceptance reference, not as game-rule authority. |
| WCAG 2.1: Animation from Interactions | `https://www.w3.org/WAI/WCAG21/Understanding/animation-from-interactions` | 2026-06-06 | accessibility guidance | reduced-motion rationale | none | Used as a UI acceptance reference for motion triggered by play and replay controls. |
| MDN: `prefers-reduced-motion` | `https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/prefers-reduced-motion` | 2026-06-06 | technical reference | reduced-motion implementation guidance | none | Used for CSS implementation guidance only. |
| MDN: SVG `title` element | `https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/title` | 2026-06-06 | technical reference | SVG accessibility guidance | none | Used for accessible SVG short-description guidance. |
| MDN: SVG `desc` element | `https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/desc` | 2026-06-06 | technical reference | SVG accessibility guidance | none | Used for accessible SVG long-description guidance. |
| MDN: ARIA `img` role | `https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/img_role` | 2026-06-06 | technical reference | grouped image semantics guidance | none | Used only when grouped SVG/image semantics are appropriate. |
| Rulepath Gate 4 spec | `specs/gate-4-three-marks-board-smoke.md` | 2026-06-06 | project authority | product scope and evidence requirements | none | Governs `three_marks` identity, rule scope, docs, tests, replay, bot, WASM, UI, benchmark, and archive obligations. |
| Rulepath Official Game Contract | `docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-06 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, and official-game evidence. |
| Rulepath IP Policy | `docs/IP-POLICY.md` | 2026-06-06 | project authority | naming and IP safety | none | Supports neutral public naming and forbids copied prose, assets, and trade dress. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | Two seats alternate placing their own marks into empty cells on a fixed 3 by 3 board. The first seat to complete a row, column, or diagonal wins. A full board with no line is a draw. | Chosen as the smallest board placement game that proves board legality, terminal line detection, draw detection, board-aware replay, direct board UI, and deterministic bots without hidden information. | yes |
| player count | Exactly 2 seats. | Gate 4 scope and classic three-in-a-row placement shape. | yes |
| first player | `seat_0` acts first. | Deterministic replay and Gate 4 assumptions. | yes |
| cell identity | Stable IDs `r1c1` through `r3c3`. | Gate 4 assumes stable row/column cell IDs for rules, actions, replay, and UI. | yes |
| optional rule included | Full-board draw when no line exists. | Required by Gate 4 exit criteria. | yes |
| optional rule excluded | Movement/sliding phase. | Gate 4 is not a Morris/Achi-style movement game. | yes |
| optional rule excluded | Misere, wild-mark, larger-board, configurable-board, generalized m,n,k, randomized setup, or multi-player variants. | These variants would obscure the narrow board-smoke goal and create premature abstraction pressure. | yes |
| Rulepath naming deviation | Public name is `Three Marks`, not a trademark-forward or source-affiliated name. | Neutral project-owned naming avoids source confusion and keeps presentation original. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `TM-AMB-001` | Whether to use the common public name or a neutral Rulepath public name. | Gate 4 spec, IP policy, source notes. | Use `Three Marks`. | `TM-SCOPE-002`, `TM-AMB-001` | UI smoke and docs review. | resolved |
| `TM-AMB-002` | Which seat places the first mark. | Gate 4 assumptions and deterministic replay needs. | `seat_0` places first. | `TM-SETUP-002`, `TM-AMB-002` | Setup test and replay/hash tests. | resolved |
| `TM-AMB-003` | Which cell ID convention to use. | Gate 4 assumptions and UI/replay needs. | Use `r1c1` through `r3c3`, counted top-to-bottom and left-to-right in the default public view. | `TM-COMP-002`, `TM-AMB-003` | Action-id stability tests and UI smoke. | resolved |
| `TM-AMB-004` | Whether a filled board with a line should be a draw. | Gate 4 rule scope and terminal clarity. | Line win takes precedence; draw requires a full board with no line. | `TM-SCORE-001`, `TM-END-001`, `TM-END-002`, `TM-AMB-004` | Terminal rule tests and traces. | resolved |

## Public naming rationale

Public ID: `three_marks`

Display name: `Three Marks`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | yes | The name describes a neutral abstract placement goal and is not source-affiliated. |
| neutral name chosen? | yes | Avoids commercial or trademark-forward framing. |
| trademark risk considered? | yes | No commercial title, logo, or affiliation is used. |
| trade-dress risk considered? | yes | The public board and marks must be original Rulepath presentation, not copied from sources or apps. |
| affiliation implication avoided? | yes | Source notes and public copy do not imply affiliation with any rules source. |
| public help text needs disclaimer? | no | No trademark-forward or brand-adjacent public name is used. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Classic three-in-a-row mechanics | low | Implement only neutral mechanics with original prose and visuals. | no |
| Commercial app or board presentation resemblance | medium if copied | Use original Rulepath SVG board and marks; do not copy source screenshots, boards, colors, icons, fonts, or layouts. | yes if found |
| Common rule terminology | low | Use project-local `Three Marks`, `seat`, `mark`, `cell`, and stable IDs. | no |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| Game docs | `games/three_marks/docs/RULES.md`, `games/three_marks/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied rulebook text. | yes |
| Game UI/assets | none in this ticket | not applicable | none | UI/assets are out of scope for GAT4THRMARBOA-001. | yes |

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
| Rulepath original rules summary | yes | `games/three_marks/docs/RULES.md` | Original Rulepath prose. |
| public-domain/classic rule facts | yes | `games/three_marks/docs/SOURCES.md` | Summarized as family context only. |
| commercial/licensed rules text | no | none | No commercial rules text is used. |
| private licensed stress-test content | no public shipment | none | No private licensed content is involved. |
| source screenshots/scans | no | none | No screenshots/scans are used. |

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Neutral name only. |
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
| none identified for GAT4THRMARBOA-001 | no | Continue evidence workflow in later tickets. | Rulepath |

## Rule-source-to-rule-ID cross-reference

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `TM-SCOPE-001` | Deterministic two-seat fixed-board placement game. | Gate 4 spec and classic rule confirmation. | no | Board-smoke role. |
| `TM-SCOPE-002` | Public name is `Three Marks`. | `TM-AMB-001` and IP policy. | yes | Neutral naming. |
| `TM-SCOPE-003` | Rust owns behavior and TypeScript presents. | Foundation docs. | no | Architecture invariant. |
| `TM-VAR-001` | Single shipped variant is `three_marks_standard`. | Gate 4 spec. | no | No variant picker in Gate 4. |
| `TM-VAR-002` | Fixed 3 by 3 board, two seats, alternating turns, line wins, full-board draws. | Gate 4 spec and classic rule confirmation. | no | Selected variant. |
| `TM-COMP-001` | Board. | Gate 4 board-smoke scope. | no | Game-local noun only. |
| `TM-COMP-002` | Stable cells `r1c1` through `r3c3`. | `TM-AMB-003`. | yes | Stable action/replay/UI target IDs. |
| `TM-COMP-003` | Seat-owned marks. | Gate 4 spec. | no | Original visual tokens required later. |
| `TM-COMP-004` | Two seats. | Gate 4 spec. | no | `seat_0`, `seat_1`. |
| `TM-COMP-005` | Winning lines. | Classic rule confirmation. | no | Game-local line detection only. |
| `TM-SETUP-001` | Board starts empty. | Gate 4 spec and classic rule confirmation. | no | Deterministic setup. |
| `TM-SETUP-002` | `seat_0` starts. | `TM-AMB-002`. | yes | Deterministic replay. |
| `TM-SETUP-003` | Perfect-information public setup. | Gate 4 assumptions. | no | No hidden state. |
| `TM-TURN-001` | Active seat places one legal mark. | Gate 4 spec and classic rule confirmation. | no | Rust legal action tree. |
| `TM-TURN-002` | Non-terminal placement passes turn. | Gate 4 spec and classic rule confirmation. | no | Alternation. |
| `TM-TURN-003` | Terminal placement ends normal turn sequence. | Gate 4 spec. | no | No post-terminal normal action. |
| `TM-ACTION-001` | Legal placements target empty cells. | Gate 4 spec and classic rule confirmation. | no | Rust-generated. |
| `TM-ACTION-002` | Occupied cells are not legal targets. | Gate 4 spec. | no | UI must consume Rust legality. |
| `TM-ACTION-003` | Terminal states expose no normal placements. | Gate 4 spec. | no | Replay/view preserves outcome. |
| `TM-RESTRICT-001` | Unknown cell rejected. | Gate 4 action model. | no | Viewer-safe diagnostic. |
| `TM-RESTRICT-002` | Occupied cell rejected. | Gate 4 spec. | no | Dev/API fail-closed path. |
| `TM-RESTRICT-003` | Stale, malformed, wrong-seat, terminal submissions rejected. | Foundation docs and Gate 4 spec. | no | State unchanged. |
| `TM-SCORE-001` | No score beyond winner or draw. | Gate 4 scope. | no | Line win before draw. |
| `TM-SCORE-002` | Ply count advances only on valid placement. | Replay and rule-test needs. | no | Max nine plies. |
| `TM-END-001` | Completing row, column, or diagonal wins immediately. | Classic rule confirmation. | no | Exact winning line required. |
| `TM-END-002` | Full board with no line is a draw. | Classic rule confirmation. | no | Draw is canonical wording. |
| `TM-END-003` | Terminal outcome remains unchanged after terminal. | Gate 4 spec. | no | Rejection path. |
| `TM-VIS-001` | All state is public. | Gate 4 assumptions. | no | Perfect information. |
| `TM-RNG-001` | Setup and rules use no randomness. | Gate 4 scope. | no | Bot randomness is external. |
| `TM-RNG-002` | Replay binds setup, commands, effects, action trees, views, and board projections. | Gate 4 spec. | no | Later replay evidence. |
| `TM-AMB-001` | Neutral name selected. | IP policy. | yes | `Three Marks`. |
| `TM-AMB-002` | First seat selected. | Deterministic replay needs. | yes | `seat_0`. |
| `TM-AMB-003` | Cell ID convention selected. | Gate 4 spec. | yes | `r1c1` through `r3c3`. |
| `TM-AMB-004` | Win/draw precedence selected. | Terminal clarity. | yes | Line win before draw. |
| `TM-VAR-003` | Related variants are not in scope. | Gate 4 spec. | yes | Narrow board smoke. |
| `TM-VAR-004` | Movement/sliding variants excluded. | Gate 4 spec. | yes | Not Morris/Achi. |
| `TM-VAR-005` | Misere/wild/larger/generalized variants excluded. | Gate 4 spec. | yes | No premature abstraction. |

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
