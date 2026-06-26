# Meldfall Ledger Sources

Game ID: `meldfall_ledger`

Public display name: `Meldfall Ledger`

Implemented variant: `classic_500_single_deck_v1`

Prepared by: `Codex`

Created: 2026-06-26

Last updated: 2026-06-26

Rules version connected to this source note: `meldfall-ledger-rules-v1`

## Source-Use Statement

Meldfall Ledger is an original Rulepath implementation in the Five Hundred
Rummy / Rummy 500 / 500 Rum rules family. External references were consulted
only to verify public rules-family facts and common variant choices: deck,
seat counts, deal sizes, draw/discard flow, meld types, laying off, discard
pickup, card values, stock exhaustion, target score, and tie continuation.

No source rules prose, examples, card imagery, product naming, component text,
icons, screenshots, scans, fonts, assets, art direction, table layout, or trade
dress is copied. Rulepath rule prose, UI copy, visual presentation, assets,
icons, card ids, and component text for `meldfall_ledger` are original.

Public presentation must use **Meldfall Ledger**. "Five Hundred Rummy",
"Rummy 500", "500 Rum", and similar names may appear only as rules-family
labels in source notes and explanatory maintenance context. They must not be
used as the public product identity, renderer identity, asset theme, or
trade-dress target.

Human IP/public-release review is pending. Any title screening, asset review,
or source-use review recorded before release is maintenance evidence only, not
legal clearance.

## Consulted Sources

All sources in this table are rationale, project-authority, rules-family,
accessibility, implementation-prior-art, or mechanic-context sources only. No
source prose, code, APIs, assets, or presentation are copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 19 Meldfall Ledger spec | `../../../archive/specs/gate-19-meldfall-ledger-five-hundred-rummy.md` | 2026-06-26 | project authority | product scope, locked variant, rule IDs, scoring model, visibility taxonomy, command suite, docs, tests, replay, bot, WASM, UI, benchmark, and capstone obligations | none | Archived gate authority. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-26 | project authority | requirements-first workflow and official-game evidence | none | Governs rules summary, source notes, player rules, rule coverage, outcome docs, no-leak proof, and web exposure. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-26 | project authority | naming, source-use limits, and public asset caution | none | Supports the original public name and forbids copied prose, assets, and trade dress. |
| Rulepath repository source bibliography | `../../../docs/SOURCES.md` | 2026-06-26 | project authority | central bibliography pattern and source-use rules | none | The capstone adds the repo-level Meldfall Ledger entry. |
| Pagat 500 Rum | `https://www.pagat.com/rummy/500rum.html` | 2026-06-25 | public rules-family reference | players/cards, deal, melds, lay-off, discard-pile pickup, scoring, stock exhaustion, and match target | none | Primary family reference named by the Gate 19 spec. |
| Bicycle Cards 500 Rum | `https://bicyclecards.com/how-to-play/500-rum` | 2026-06-25 | public rules-family reference | standard deck, target score, deal counts, meld/run definitions, highest-score-at-500 wording | none | Secondary common-rules confirmation. |
| Pagat Scoring Rummies | `https://www.pagat.com/rummy/Scoring_Rummies.html` | 2026-06-25 | public rules-family context | scoring-rummy family placement, 500 Rum deal summary, larger-table deck conventions, strategy context | none | Used to document the one-deck 5/6-seat deviation. |
| Rummy Rulebook Rummy 500 | `https://www.rummyrulebook.com/pages/rummy-500/` | 2026-06-25 | supplemental rules-family reference | draw/discard/meld/layoff/go-out/scoring comparison and variants | none | Used only as variant comparison. |
| `timpalpant/rummy` | `https://github.com/timpalpant/rummy` | 2026-06-25 | external implementation prior art | state-machine context for stock/discard actions and must-play-picked-discard invariant | none | No code, API, architecture, or text is imported. |
| RLCard Gin Rummy docs | `https://rlcard.org/rlcard.games.gin_rummy.html` | 2026-06-25 | external implementation taxonomy/prior art | comparison for meld-game modules and negative boundary reminder about RL/ML | none | Rulepath does not adopt RLCard architecture or public bot approach. |
| WAI-ARIA Authoring Practices Grid Pattern | `https://www.w3.org/WAI/ARIA/apg/patterns/grid/` | 2026-06-25 | accessibility reference | keyboard-operable dense card/tableau grouping | none | UI reference only; no copy or component API is imported. |
| WCAG 2.2 Understanding Dragging Movements | `https://www.w3.org/WAI/WCAG22/Understanding/dragging-movements.html` | 2026-06-25 | accessibility reference | no-drag-required card movement alternative | none | Supports click/select and keyboard action builder requirements. |

## Adopted Design Facts

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Public identity | The public game is Meldfall Ledger; Rummy-family names are source labels only. | Original identity reduces source confusion and trade-dress risk. |
| Seat range | 2 through 6 seats are supported; default setup uses 4 seats. | Gate 19 requires variable-N proof across a larger card-zone surface. |
| Deck | One standard 52-card deck, no jokers, all supported seat counts. | The spec deliberately chooses one deck even for 5/6 seats. |
| Deal | 2 seats receive 13 cards each; 3-6 seats receive 7 cards each; one initial face-up discard starts the discard pile. | Common source baseline plus Gate 19 single-deck pinning. |
| Turn order | Left of dealer starts and play proceeds clockwise. | Common family shape and deterministic setup contract. |
| Draw options | Stock draw or public discard-pile draw. | Core family shape. |
| Discard-pile pickup | Selecting a discard takes that card plus all newer cards above it; the selected card must be used immediately. | Signature Rummy 500-family behavior and Gate 19 strict variant. |
| Top discard commitment | The top discard also must be used immediately when picked. | Gate 19 chooses the stricter variant to keep legality unambiguous. |
| Melds | Sets are 3-4 same-rank cards; runs are 3+ consecutive same-suit cards. | Common family baseline. |
| Ace runs | Ace can be low or high but cannot wrap. | Variant ambiguity is pinned for tests and player explanation. |
| Card values | Ace 15, K/Q/J/10 10, ranks 2-9 pip value. | Common scoring-rummy value table selected by the spec. |
| Lay-off | Any seat may extend any public meld, including an opponent's meld, when the result remains legal. | Family behavior and larger tableau proof. |
| Lay-off score credit | The seat that plays a tabled card receives its score credit. | Separates group origin from per-card scoring. |
| Going out | A seat may go out by tabling every card or by discarding the last card after table plays. | Gate 19 excludes floating and discard-required variants. |
| Stock exhaustion | Empty stock with no legal/accepted discard draw settles the round. | Gate 19 excludes discard reshuffle. |
| Scoring | Tabled cards score positive to their played-by seats; in-hand cards score negative to their holders. | Core scoring-rummy contract. |
| Match target | After settlement, a unique highest seat at or above 500 wins; equal highest scores continue. | Avoids arbitrary tiebreakers. |

## Variant Choice And Deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `classic_500_single_deck_v1` / Meldfall Ledger | Gate 19 scope. | yes |
| player count | 2-6 seats; default 4 | Roadmap and spec require variable-N proof. | yes |
| deck count | One standard 52-card deck for 2-6 seats | Gate 19 deliberately excludes two-deck larger-table conventions. | yes |
| jokers/wilds | Excluded | Variant control and single-deck clarity. | yes |
| opening minimum | Excluded | Out of Gate 19 scope. | yes |
| top discard immediate-use | Required | Spec selects strict discard-pickup handling. | yes |
| discard pile reshuffle | Excluded | Stock exhaustion settles the round. | yes |
| final discard | Not required to go out | Spec pins no-floating/no-discard-required behavior. | yes |
| ace scoring | Ace always scores 15 | Avoids low-ace score ambiguity. | yes |
| settlement visibility | Public totals/counts only for unmelded cards | Rulepath hidden-information posture. | yes |
| public name | Meldfall Ledger | Original neutral Rulepath identity. | yes |
| optional rules excluded | Jokers, wilds, two decks, opening minimums, Call Rummy, frozen piles, around-the-corner runs, floating, mandatory final discard, partnerships, teams, rearranging tabled melds. | Out of Gate 19 scope. | yes |

## Ambiguity Log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `ML-AMB-001` | Whether 5/6 seats should use two decks. | Pagat/Bicycle/Scoring Rummies context and Gate 19 single-deck requirement. | One 52-card deck for every supported seat count. | `ML-SETUP-003`, `ML-SETUP-004` | setup 2/4/6 and conservation tests | resolved |
| `ML-AMB-002` | Whether the top discard must be used immediately. | Pagat strict-rule note and implementation prior art context. | Top discard pickup creates the same immediate-use commitment as deeper pickup. | `ML-TURN-004` | top-discard pickup trace | resolved |
| `ML-AMB-003` | How ace works in runs and scoring. | Bicycle/Pagat/Rummy Rulebook comparison and spec variant pin. | Ace can be low or high, cannot wrap, and always scores 15. | `ML-MELD-003`, `ML-SCORE-001` | ace run and scoring traces | resolved |
| `ML-AMB-004` | Whether a final discard is mandatory to go out. | Pagat/Rummy Rulebook comparison and Gate 19 scope. | Final discard is allowed but not required. | `ML-TURN-007`, `ML-TURN-008` | both go-out traces | resolved |
| `ML-AMB-005` | Whether exact remaining opponent cards are public at settlement. | Source-table behavior versus Rulepath no-leak law. | Public settlement uses totals/counts only; own remaining cards may appear in own seat-private export. | `ML-VIS-006`, `ML-SCORE-007` | settlement no-leak matrix | resolved |
| `ML-AMB-006` | Whether at/above-500 ties end by seat order or other tiebreaker. | Family source comparison and Rulepath deterministic no-arbitrary-tiebreak posture. | Equal highest eligible scores continue another round. | `ML-MATCH-003` | target-tie-continues trace | resolved |

## Public Naming Rationale

Public ID: `meldfall_ledger`

Display name: `Meldfall Ledger`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | constrained | Rummy-family names are common descriptors, but Rulepath uses an original catalog identity. |
| neutral name chosen? | yes | "Meldfall" evokes tabled melds growing over the round; "Ledger" evokes cumulative scoring and public accounting. |
| trademark risk considered? | yes | Neutral original title avoids product-source confusion. Human title review remains pending before release. |
| trade-dress risk considered? | yes | Renderer/icon work must avoid existing app/table/card presentation mimicry. |
| casino/brand term avoided? | yes | Public copy avoids wagering, casino, and affiliation framing. |
| affiliation implication avoided? | yes | Docs and UI must not imply affiliation with source sites, card companies, apps, or rulebook publishers. |

## Trademark And Trade-Dress Concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Public use of Rummy-family common names as product identity | medium | Use Meldfall Ledger for public identity; keep family labels in source notes and maintenance context. | no |
| Existing online rummy app layouts or table style | human review needed | Build original Rulepath SVG/card/table presentation; do not copy screenshots, layouts, icons, animations, or color treatments. | yes before public release |
| Conventional playing-card imagery | medium if copied card faces or suit art appear | Use original or reviewed assets; do not copy card faces, icons, scans, fonts, or product presentation. | yes if copied or trade-dress-like |
| Source phrasing | medium if paraphrased too closely | Maintain consulted-not-copied notes and original Rulepath phrasing. | yes if copied prose appears |
| Generated or custom icon assets | human review needed | Record prompt/review or project-authored SVG notes when asset lands. | yes before public release |

## Asset Provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---:|---|---|---:|
| Game docs | `games/meldfall_ledger/docs/RULES.md`, `games/meldfall_ledger/docs/SOURCES.md` | original | Rulepath/Codex-authored prose from project spec and summarized source facts | No copied rules text, examples, or tables. | yes |
| Public game name | `Meldfall Ledger` | original project name | Rulepath/Codex-authored public name | Pending human release review. | pending |
| Card ids and labels | future game-local Rust/static metadata | original implementation expression over common card facts | Rulepath/Codex-authored labels and IDs | Standard card identities are common facts; rendered card faces/assets must still be original or reviewed. | pending later asset review |
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
| Rulepath original rules summary | yes | `games/meldfall_ledger/docs/RULES.md` | Original Rulepath prose. |
| Gate 19 project-authority facts | yes | `games/meldfall_ledger/docs/SOURCES.md` | Summarized as rationale only. |
| Generic Rummy-family rules facts | yes | this note and later player help | Summarized, not copied. |
| Public source prose, examples, tables, diagrams, or screenshots | no | none | No source prose or visual source material is copied. |
| Commercial/licensed rules text | no | none | No source material is redistributed. |
| Commercial card faces, suit art, product names, icons, screenshots, fonts, app layouts, or table presentation | no | none | No assets introduced in this ticket. |
| Private licensed stress-test content | no public shipment | none | No private licensed content is involved. |

Private licensed stress tests are late, isolated, optional, non-public, and
non-architectural. They must not contaminate `engine-core`, public assets,
public docs, or public web bundles.

## Human/Legal Review Triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no for current name; title still pending release review | Public name is original. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | Only ordinary rank/suit labels are expected. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no in this ticket | Later renderer/icon work must review assets. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no in this ticket | Later generated assets require review notes. |
| uncertainty about public-domain status | no for abstract rules-family facts | No source material is redistributed. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release Blocking Concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| human IP/public-release review pending | yes before public release | Maintainer/human review of name, presentation, source use, assets, and release checklist. | Rulepath maintainers |
| visual/card assets not yet reviewed | later-ticket blocker if introduced | Renderer/icon tickets must record asset provenance and trade-dress review. | Rulepath maintainers |
| public icon/table/card asset review | yes before release | Record original/project-owned/generated-review provenance in release checklist. | Rulepath maintainers |

## Rule-Source-To-Rule-ID Cross-Reference

Every release-relevant stable rule ID in `RULES.md` must have source or design
rationale support here. Meldfall Ledger uses public rules-family facts
expressed in original Rulepath prose, with project authority from the Gate 19
spec.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `ML-ID-001` through `ML-ID-002` | Identity, variant, public name, rules/data versions. | Gate 19 spec, Rulepath IP policy. | yes | Public release review remains pending. |
| `ML-SETUP-001` through `ML-SETUP-006` | Seat range, stable seat IDs, single-deck deal, initial discard, dealer/start seat, replay inputs. | Gate 19 spec, Pagat, Bicycle, Pagat Scoring Rummies. | yes | One deck for 5/6 seats is deliberate. |
| `ML-VIS-001` through `ML-VIS-006` | Public observer, seat-private hands, hidden stock order, discard/tableau publicness, settlement redaction. | Rulepath foundations, Gate 19 spec, no-leak matrix. | yes | Public settlement keeps unmelded identities scoped. |
| `ML-TURN-001` through `ML-TURN-009` | Draw phase, stock draw, discard-pile pickup, immediate-use commitment, table plays, discard, go-out, stock exhaustion. | Gate 19 spec, Pagat, Bicycle, Rummy Rulebook, implementation prior-art note. | yes | Top discard also requires immediate use. |
| `ML-MELD-001` through `ML-MELD-005` | Set and run legality, ace low/high/no-wrap, ownership, public meld groups. | Gate 19 spec, Pagat, Bicycle, Rummy Rulebook. | yes | No remelding/rearranging tabled groups. |
| `ML-LAYOFF-001` through `ML-LAYOFF-003` | Lay-off onto any public meld, per-card score credit, no table rearrangement. | Gate 19 spec, Pagat, Rummy Rulebook. | yes | Score credit separates origin and played-by seats. |
| `ML-SCORE-001` through `ML-SCORE-007` | Card values, tabled positives, in-hand penalties, round delta, cumulative scores, score-credit owner, settlement visibility. | Gate 19 spec, Pagat, Bicycle, Rummy Rulebook, no-leak law. | yes | Scores may be negative. |
| `ML-MATCH-001` through `ML-MATCH-006` | 500 target, unique highest winner, tie continuation, next-round transition, standings. | Gate 19 spec, Pagat, Bicycle. | yes | Seat order is not a tiebreaker. |
| `ML-REPLAY-001` through `ML-REPLAY-003` | Deterministic replay, viewer-scoped export/import, Trace Schema v1 coverage. | Rulepath replay/fixture/hash law and Gate 19 spec. | no | No schema migration authorized. |
| `ML-BOT-001` through `ML-BOT-003` | L0 random legal, bounded L1 authorized inputs, safe explanations, prohibited algorithms absent. | Rulepath AI law, Gate 19 spec, rummy strategy notes. | yes | RLCard is negative prior art only. |
| `ML-UI-001` through `ML-UI-003` | Rust-owned legal controls, large hand/tableau affordances, browser no-leak. | Rulepath UI law, Gate 19 spec, WAI/WCAG references. | no | TypeScript remains presentation-only. |

## Final Source/IP Checklist

- Sources are summarized in original language.
- Source quality and date consulted are recorded.
- Variant choices and deviations are explicit.
- Ambiguities connect to rule IDs and future tests/traces.
- Public naming avoids affiliation and trade-dress risk.
- Assets are not added in this ticket; future assets require provenance review.
- Fonts are system-only unless later reviewed.
- Public/private content boundary is explicit.
- Human/legal review triggers are recorded for final closeout.
- Human IP/public-release review remains pending.
