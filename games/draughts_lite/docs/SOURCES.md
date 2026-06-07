# Draughts Lite Sources

Game ID: `draughts_lite`

Public display name: `Draughts Lite`

Implemented variant: `draughts_lite_standard`

Prepared by: `Codex`

Created: 2026-06-07

Last updated: 2026-06-07

Rules version connected to this source note: `draughts_lite-rules-v1`

## Source-use statement

Rulepath uses consulted sources to verify rules, variants, terminology context,
public-history context, accessibility expectations, IP caution, and ambiguity
resolution.

Sources do not authorize copied prose, component text, icons, screenshots,
scans, fonts, assets, board art, token art, packaging style, marketing language,
or trade dress. Rulepath rule prose, UI copy, visual presentation, assets,
icons, and component text for `draughts_lite` are original.

No federation rulebook prose, diagrams, board presentation, screenshots, scans,
opening books, engine data, or source-specific trade dress are copied into this
game. The implemented game is a neutral, original Rulepath presentation of a
classic draughts/checkers-family rules subset.

## Consulted sources

All external sources in this table were consulted for the Gate 7 specification
and this source note.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| World Checkers/Draughts Federation, Rules of Draughts | `https://wcdf.net/rules/rules_of_checkers_english.pdf` | 2026-06-07 | official rules | rule verification and variant scope | none | Used to verify the English draughts/checkers-family shape adopted here: 8 by 8 board, ordinary men, kings, compulsory capture, same-piece continuation, promotion stopping a capture turn, and win-by-no-piece/no-move outcomes. |
| Schaeffer et al., Checkers Is Solved, Science 2007 | `https://www.cs.cornell.edu/courses/cs6700/2013sp/readings/06-b-Checkers-Solved-Science-2007.pdf` | 2026-06-07 | reputable secondary / research paper | strong-engine exclusion rationale | none | Used only as context for avoiding public strong-engine claims, opening books, endgame databases, and search-heavy bot scope in this gate. |
| University of Alberta Chinook project | `https://webdocs.cs.ualberta.ca/~chinook/games/` | 2026-06-07 | reputable secondary / research project | solved-game context and strong-engine exclusion rationale | none | Used only as context; no engine data, move tables, examples, assets, or text are copied. |
| WAI-ARIA Authoring Practices: Grid Pattern | `https://www.w3.org/WAI/ARIA/apg/patterns/grid/` | 2026-06-07 | standards guidance | keyboard grid interaction guidance | none | Used only if the UI uses an ARIA grid or roving-focus pattern. |
| WAI Understanding WCAG SC 2.3.3: Animation from Interactions | `https://www.w3.org/WAI/WCAG21/Understanding/animation-from-interactions` | 2026-06-07 | standards guidance | reduced-motion rationale | none | Used as a UI acceptance reference for motion triggered by play and replay controls. |
| Rulepath Gate 7 spec | `archive/specs/gate-7-draughts-lite-compound-action-tree.md` | 2026-06-07 | project authority | product scope and evidence requirements | none | Governs `draughts_lite` identity, rule scope, docs, tests, replay, bot, WASM, UI, benchmark, primitive-pressure, and archive obligations. |
| Rulepath Official Game Contract | `../../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-07 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, and official-game evidence. |
| Rulepath IP Policy | `../../../../docs/IP-POLICY.md` | 2026-06-07 | project authority | naming and IP safety | none | Supports neutral public naming and forbids copied prose, assets, and trade dress. |

## Adopted rules

The implemented `draughts_lite_standard` variant adopts these rule facts in
original Rulepath prose:

| Adopted item | Rulepath statement | Rule IDs |
|---|---|---|
| Board and setup | The game uses an 8 by 8 board, playable dark squares, and 12 men per seat on the first three home rows. | `DL-VAR-001`, `DL-VAR-002`, `DL-SETUP-001`, `DL-SETUP-002` |
| Men | Men move and jump diagonally forward only. | `DL-VAR-003`, `DL-ACTION-001`, `DL-ACTION-002` |
| Kings | Kings move and jump one diagonal square forward or backward. | `DL-COMP-005`, `DL-ACTION-004` |
| Capture obligation | If any capture is available to the active seat, a capture must be chosen. | `DL-ACTION-002`, `DL-RESTRICT-001` |
| Same-piece continuation | A capturing piece must keep jumping while the same piece has a legal continuation. | `DL-TURN-004`, `DL-RESTRICT-002` |
| No maximum-capture mandate | A player may choose among available complete capture paths; the longest path is not mandatory. | `DL-RESTRICT-004`, `DL-AMB-004` |
| Promotion | A man that reaches the opponent's king row becomes a king. | `DL-MOVE-003` |
| Promotion during capture | If a man reaches the king row during a capture sequence, that move ends immediately. | `DL-ACTION-003`, `DL-AMB-003` |
| Terminal wins | A player wins when the opponent has no pieces or no legal move after the completed action. | `DL-END-001`, `DL-END-002` |

## Omitted rules and adjudication

| Omitted item | Rulepath decision | Public-facing note needed? |
|---|---|---:|
| International or alternate draughts variants | Not included in Gate 7. | yes |
| Flying kings and long-range king capture | Not included; kings move and capture one diagonal square. | yes |
| Backward movement or backward capture by men | Not included; men are forward-only. | yes |
| Maximum-capture rules | Not included; compulsory capture does not imply longest capture sequence. | yes |
| Huffing or penalty removal for missed capture | Not included; illegal quiet moves are rejected by Rust validation. | yes |
| Clocks, ratings, resignation, agreement draws, repetition claims, referee timing, and long no-progress adjudication | Not included; Gate 7 has only no-piece and no-legal-move terminal wins. | yes |
| Opening books, endgame databases, solved-game tables, and strong-engine claims | Not included; bot scope is modest and non-search. | yes |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `draughts_lite_standard`: two seats play on an 8 by 8 board with 12 men each, forward-only men, one-step kings, mandatory capture, mandatory same-piece continuation, promotion-stop on capture, and wins by no opponent pieces or no opponent legal move. | Gate 7 spec and WCDF English draughts/checkers rule verification. | yes |
| player count | Exactly two seats, `seat_0` and `seat_1`. | Gate 7 scope and deterministic replay needs. | yes |
| first player | `seat_0` acts first. | Gate 7 setup requirement and existing Rulepath fixed-first-player precedent. | yes |
| coordinate identity | Stable cells `r1c1` through `r8c8`, with row 1 at the top and column 1 at the left in the default public view. | Gate 7 spec and existing stable cell-id style. | yes |
| playable-square parity | Playable dark squares are cells where `row + column` is odd. | Gate 7 spec. | yes |
| optional rule included | Rust-generated multi-segment compound action paths for origin, landing, and forced continuation. | Gate 7 compound-action proof. | yes |
| optional rule excluded | Maximum-capture mandate, huffing, flying kings, alternate board sizes, tournament draw procedures, and search/learning bots. | Gate 7 narrow official-game scope and public bot policy. | yes |
| Rulepath deviation from common variants | Public name is `Draughts Lite`, not an affiliation-forward federation or commercial name. | IP policy and neutral naming requirement. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `DL-AMB-001` | Which public name to use for a checkers/draughts-family game. | Gate 7 spec, IP policy. | Use `Draughts Lite` and `draughts_lite`. | `DL-AMB-001`, `DL-SCOPE-001` | Public docs and UI review. | resolved |
| `DL-AMB-002` | Which coordinate orientation and playable-square parity to use. | Gate 7 spec, existing stable cell-id style. | Use `r1c1` through `r8c8`, row 1 top, column 1 left, dark playable cells where `row + column` is odd. | `DL-VAR-002`, `DL-AMB-002` | Setup tests, action-tree tests, traces, and UI smoke. | resolved |
| `DL-AMB-003` | Whether a man promoted during a capture continues as a king in the same turn. | WCDF English draughts/checkers rules, Gate 7 spec. | Promotion during capture ends the move immediately. | `DL-ACTION-003`, `DL-MOVE-003`, `DL-AMB-003` | Promotion-stop rule test and golden trace. | resolved |
| `DL-AMB-004` | Whether available captures must maximize the number of pieces captured. | WCDF English draughts/checkers rules, Gate 7 scope. | No maximum-capture rule; any complete legal capture path may be selected. | `DL-RESTRICT-004`, `DL-AMB-004` | Alternative-capture test and trace. | resolved |
| `DL-AMB-005` | Whether tournament draw and referee procedures are part of terminal logic. | WCDF rules context, Gate 7 scope. | Omit tournament adjudication; terminal wins are no opponent pieces or no opponent legal move. | `DL-END-001`, `DL-END-002`, `DL-END-003` | Terminal rule tests and coverage. | resolved |

## Public naming rationale

Public ID: `draughts_lite`

Display name: `Draughts Lite`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | yes | `Draughts` is used descriptively for a classic game family; `Lite` signals the scoped Rulepath variant. |
| neutral name chosen? | yes | `Draughts Lite` is Rulepath naming for this implementation and does not claim federation, publisher, or tournament affiliation. |
| trademark risk considered? | yes | Public docs and UI avoid product branding, logos, slogans, source-specific names, and affiliation wording. |
| trade-dress risk considered? | yes | Board, pieces, colors, labels, icons, layout, animation, and help text must be original Rulepath presentation. |
| affiliation implication avoided? | yes | Sources are cited as rule references only; public surfaces do not imply endorsement. |
| public help text needs disclaimer? | no | Neutral naming and source notes are sufficient unless later human/legal review requests additional text. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Classic draughts/checkers mechanics | low for mechanics, human review for expression | Implement neutral mechanics with original prose and visuals. | no |
| Federation branding, logos, source diagrams, or source presentation | high if copied | Cite sources only; do not copy source artwork, diagrams, examples, board styling, or text. | yes if found |
| Commercial checkers/draughts product presentation | medium to high if imitated | Use original Rulepath visual design and labels. | yes if found |
| Common rule terminology | low | Use stable Rulepath IDs and original prose; avoid rulebook sentence structure and examples. | no |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| Game docs | `games/draughts_lite/docs/RULES.md`, `games/draughts_lite/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied rulebook text. | yes |
| Game UI/assets | none in this ticket | not applicable | none | UI/assets are out of scope for GAT7DRALITCOM-001. | yes |
| Opening books / engine data / solved-game tables | none | not applicable | none | Explicitly excluded from public game scope. | yes |

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
| Rulepath original rules summary | yes | `games/draughts_lite/docs/RULES.md` | Original Rulepath prose. |
| classic rule facts | yes | `games/draughts_lite/docs/SOURCES.md` | Summarized as rule-family context only. |
| federation rulebook prose | no by default | none | No source prose is copied. |
| opening books, endgame data, solved-game tables | no public shipment | none | Explicitly out of scope. |
| private licensed stress-test content | no public shipment | none | No private licensed content is involved. |
| source screenshots/scans/diagrams | no | none | No screenshots, scans, or diagrams are used. |

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Neutral public name only. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | No cards or protected component text. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no | No art or UI asset in this ticket. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no | None. |
| uncertainty about public-domain status | no | Sources are used only as references; public prose is original. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release blocking concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| none identified for GAT7DRALITCOM-001 | no | Continue evidence workflow in later tickets. | Rulepath |

## Rule-source-to-rule-ID cross-reference

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `DL-SCOPE-001` | Two-seat deterministic Draughts Lite scope. | Gate 7 spec and WCDF English draughts/checkers rule verification. | no | Official game scope. |
| `DL-SCOPE-002` | Complete move is one multi-segment replay command. | Gate 7 compound-action proof and Rulepath replay contract. | no | Core gate purpose. |
| `DL-SCOPE-003` | Tournament adjudication and strong-engine scope are excluded. | Gate 7 spec and solved-game context sources. | yes | Omission resolved in `DL-AMB-005`. |
| `DL-VAR-001` | Standard variant uses 8 by 8 playable dark squares. | Gate 7 spec and WCDF English draughts/checkers rule verification. | no | Board geometry. |
| `DL-VAR-002` | Stable coordinates and playable parity. | Gate 7 spec and existing Rulepath cell-id style. | yes | Coordinate ambiguity resolved in `DL-AMB-002`. |
| `DL-VAR-003` | Men are forward-only; kings are any-diagonal one-step pieces. | WCDF English draughts/checkers rule verification and Gate 7 scope. | no | No flying kings. |
| `DL-SETUP-001` | Two seats. | Gate 7 scope. | no | `seat_0` and `seat_1`. |
| `DL-SETUP-002` | Each seat starts with 12 men on first three home rows. | WCDF English draughts/checkers rule verification and Gate 7 spec. | no | Deterministic setup. |
| `DL-SETUP-003` | `seat_0` acts first. | Gate 7 setup requirement. | no | Fixed first actor. |
| `DL-ACTION-001` | Quiet moves exist only with no capture available. | WCDF English draughts/checkers rule verification and Gate 7 mandatory-capture scope. | no | Rust-generated legal tree. |
| `DL-ACTION-002` | Captures are mandatory and represented as complete capture paths. | WCDF English draughts/checkers rule verification and Gate 7 compound-action scope. | no | Rust validation authority. |
| `DL-ACTION-003` | Promotion during capture ends the move. | WCDF English draughts/checkers rule verification and Gate 7 spec. | yes | Resolved in `DL-AMB-003`. |
| `DL-ACTION-004` | Kings jump one adjacent opposing piece in any diagonal direction. | WCDF English draughts/checkers rule verification and Gate 7 scope. | no | No flying kings. |
| `DL-RESTRICT-001` | Capturing is mandatory. | WCDF English draughts/checkers rule verification. | no | Quiet move diagnostic expected. |
| `DL-RESTRICT-002` | Same-piece continuation is mandatory. | WCDF English draughts/checkers rule verification and Gate 7 spec. | no | Partial capture diagnostic expected. |
| `DL-RESTRICT-003` | Invalid, stale, malformed, wrong-actor, or illegal paths are rejected without mutation. | Rulepath foundations and replay/action validation contract. | no | Viewer-safe diagnostics. |
| `DL-RESTRICT-004` | No maximum-capture mandate. | WCDF English draughts/checkers rule verification and Gate 7 scope. | yes | Resolved in `DL-AMB-004`. |
| `DL-MOVE-001` | Quiet move mutation. | Gate 7 spec and adopted rules. | no | No capture removal. |
| `DL-MOVE-002` | Jump mutation removes one opposing piece. | Gate 7 spec and adopted rules. | no | Path order matters. |
| `DL-MOVE-003` | Promotion on king row. | WCDF English draughts/checkers rule verification and Gate 7 spec. | yes | Promotion-stop edge case. |
| `DL-END-001` | Opponent no pieces means acting seat wins. | WCDF English draughts/checkers rule verification and Gate 7 spec. | no | Terminal win. |
| `DL-END-002` | Opponent no legal move means acting seat wins. | WCDF English draughts/checkers rule verification and Gate 7 spec. | yes | No stalemate draw. |
| `DL-END-003` | Draw/tournament adjudication omitted. | Gate 7 scope. | yes | Omission resolved in `DL-AMB-005`. |
| `DL-VIS-001` | Public perfect-information view and no hidden leakage. | Rulepath foundations and Gate 7 perfect-information scope. | no | No hidden information exists. |
| `DL-RNG-001` | No game-rule randomness. | Gate 7 deterministic setup scope. | no | Bot randomness remains bot-layer concern. |
| `DL-REPLAY-001` | Replay records one complete multi-segment action path per move. | Gate 7 spec and Trace Schema v1 list-shaped action paths. | no | No schema bump. |
| `DL-BOT-001` | Level 0 bot uses complete Rust legal paths. | Gate 7 bot scope and Rulepath AI policy. | no | Normal command path. |
| `DL-BOT-002` | Level 1 bot remains modest and non-search. | Gate 7 bot scope and solved-game context sources. | no | No strong-engine claim. |
| `DL-UI-001` | UI presents Rust legality and does not calculate legal moves. | Rulepath boundary docs and Gate 7 UI scope. | no | TypeScript presentation only. |
| `DL-UI-002` | UI supports accessible, reduced-motion, non-color-only interaction. | WAI-ARIA grid guidance, WCAG animation guidance, and Gate 7 public-polish scope. | no | UI evidence lands later. |
| `DL-AMB-001` | Neutral public naming. | IP policy and Gate 7 spec. | yes | Resolved. |
| `DL-AMB-002` | Coordinate orientation and parity. | Gate 7 spec and existing Rulepath style. | yes | Resolved. |
| `DL-AMB-003` | Promotion during capture stop. | WCDF English draughts/checkers rule verification. | yes | Resolved. |
| `DL-AMB-004` | Maximum-capture omission. | WCDF English draughts/checkers rule verification and Gate 7 scope. | yes | Resolved. |

## Final source/IP checklist

- Sources are summarized in original language.
- Source quality and date consulted are recorded.
- Variant choices and deviations are explicit.
- Adopted and omitted rules are separated.
- Ambiguities connect to rule IDs and tests.
- Public naming avoids affiliation and trade-dress risk.
- Assets are original, verified public-domain, license-reviewed, or generated-reviewed.
- Fonts are system-only or license-reviewed.
- Public/private content boundary is explicit.
- Human/legal review triggers are not hidden.
- Release blockers are recorded.
