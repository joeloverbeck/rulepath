# Briar Circuit Sources

Game ID: `briar_circuit`

Public display name: `Briar Circuit`

Implemented variant: `briar_circuit_standard`

Prepared by: `Codex`

Created: 2026-06-21

Last updated: 2026-06-21

Rules version connected to this source note: `briar-circuit-rules-v1`

## Source-Use Statement

Briar Circuit is an original Rulepath implementation in the classic
four-player Hearts rules family. External Hearts references were consulted only
to verify public-domain rules-family facts and common variant choices: four
players, a 52-card deck, passing direction cycle, 2 clubs opening lead,
follow-suit obligation, point cards, shoot-the-moon conventions, 100-point
threshold, and low-score winning.

No source rules prose, examples, card imagery, product naming, component text,
icons, screenshots, scans, fonts, assets, art direction, table layout, or trade
dress is copied. Rulepath rule prose, UI copy, visual presentation, assets,
icons, card ids, and component text for `briar_circuit` are original.

Public presentation must use **Briar Circuit**. "Hearts" may appear only as a
rules-family label in source notes and explanatory maintenance context. It must
not be used as the public product identity, renderer identity, asset theme, or
trade-dress target.

## Consulted Sources

All sources in this table are rationale, project-authority, taxonomy,
accessibility, or mechanic-context sources only. No source prose or assets are
copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 16 Briar Circuit spec | `../../../archive/specs/gate-16-briar-circuit-trick-taking.md` | 2026-06-21 | project authority | product scope, variant lock, rule IDs, naming rationale, no-leak taxonomy, command suite, docs, tests, replay, bot, WASM, UI, benchmark, and capstone obligations | none | Governs this game's implementation tickets and acceptance evidence. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-21 | project authority | requirements-first workflow and official-game evidence | none | Governs rules summary, source notes, player rules, rule coverage, outcome docs, no-leak proof, and web exposure. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-21 | project authority | naming, source-use limits, and public asset caution | none | Supports the original neutral public name and forbids copied prose, assets, and trade dress. |
| Rulepath repository source bibliography | `../../../docs/SOURCES.md` | 2026-06-21 | project authority | central bibliography pattern and source-use rules | none | This per-game note adds the Briar Circuit-specific rules-family references. |
| Pagat Hearts | `https://www.pagat.com/reverse/hearts.html` | 2026-06-21 | public rules-family reference | four-player Hearts baseline, pass cycle, 2 clubs opening, follow-suit, hearts-broken choice, point values, moon conventions, threshold, low-score tie continuation | none | Consulted for rule facts and variant comparison only; Rulepath prose and presentation are original. |
| Bicycle Hearts | `https://bicyclecards.com/how-to-play/hearts` | 2026-06-21 | public rules-family reference | common Hearts teaching shape, 52-card/four-player baseline, passing cycle, point values, moon convention comparison | none | Used as a secondary comparison where sources differ; no text, diagrams, examples, or presentation copied. |
| OpenSpiel | `https://openspiel.readthedocs.io/en/latest/intro.html` and `https://github.com/google-deepmind/open_spiel` | 2026-06-21 | research/prior-art context | imperfect-information game framework context and warning against importing research AI/search assumptions into public bots | none | Prior-art only. Briar Circuit does not adopt OpenSpiel APIs, game descriptions, bots, or examples. |
| WAI-ARIA Authoring Practices keyboard interface | `https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/` | 2026-06-21 | accessibility reference | later keyboard/focus obligations for card/pass controls | none | Used for UI acceptance context, not rules behavior. |
| WCAG reduced motion CSS technique C39 | `https://www.w3.org/WAI/WCAG22/Techniques/css/C39` | 2026-06-21 | accessibility reference | later reduced-motion evidence for effect presentation | none | Used for UI acceptance context, not rules behavior. |

## Adopted Design Facts

The implemented `briar_circuit_standard` variant adopts these facts in original
Rulepath prose:

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Four seats | Exactly four independent seats play each match. | Gate 16 fixes a four-seat public-scaling proof. |
| Standard deck | A game-local standard 52-card deck is shuffled and fully dealt. | This matches the selected rules family and gives every seat 13 private cards. |
| Dealer rotation | Initial dealer is `seat_0`; deal rotates clockwise after every hand. | Fixed initial dealer supports reproducible fixtures; rotation supports multi-hand proof. |
| Opening lead | The holder of 2 clubs must lead it to the first trick. | This is the selected standard opening rule. |
| Pass cycle | The pass cycle is left, right, across, hold. | This is the selected common four-player cycle and proves private simultaneous commitment. |
| Follow-suit obligation | A seat holding the led suit must play it; a void seat may discard under the remaining restrictions. | This is the core repeated trick-taking mechanic under Gate 16. |
| First-trick point restriction | A void-in-clubs seat may not discard a heart or queen of spades while any non-point card is available. | The no-alternative exception prevents empty legal sets. |
| Hearts broken | Hearts are broken only by a played heart, including a legal all-hearts lead; queen of spades alone does not break hearts. | The spec selects the hearts-only definition after comparing source variants. |
| Trick winner | Highest card of the led suit wins; off-suit cards never win. | This is deterministic and matches the selected family rule. |
| Point values | Hearts are 1 point each; queen of spades is 13; all other cards are 0. | This produces 26 raw hand points. |
| Moon rule | Capturing all 26 raw points gives the shooter 0 and every opponent 26. | Fixed add-26-to-opponents avoids a choice branch and keeps scoring deterministic. |
| Match threshold | After a complete hand, scores at or above 100 trigger terminal evaluation. | The selected common threshold gives a clear multi-hand end condition. |
| Low-score tie | If the lowest cumulative score is tied, play more complete hands until the low score is unique. | This avoids an arbitrary seat-order tiebreaker. |
| Neutral public presentation | The public game name is Briar Circuit and public visuals/copy are original. | The game avoids source confusion and copied expression. |

## Variant Choice And Deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `briar_circuit_standard` / Briar Circuit | Gate 16 trick-taking proof scope. | yes |
| player count | Exactly four seats. | Roadmap and spec lock fixed-four-seat Hearts-family proof. | yes |
| deck | Standard 52-card deck, fully dealt. | Selected family baseline. | yes |
| pass directions | Left, right, across, hold; repeat. | Common four-player cycle selected by spec. | yes |
| first lead | Holder of 2 clubs leads 2 clubs. | Selected standard opening rule. | yes |
| first trick restriction | No hearts or queen of spades on trick one while a non-point discard exists. | Common no-points-on-first-trick variant with explicit no-alternative exception. | yes |
| hearts broken | Hearts break by hearts only; queen of spades does not break hearts. | Pagat-style choice selected by spec over broader house-rule phrasing. | yes |
| moon resolution | Shooter adds 0; each opponent adds 26. | Deterministic common convention. | yes |
| low-score tie | Continue complete hands. | Avoids arbitrary seat-order tiebreak. | yes |
| optional rules excluded | Omnibus/jack bonus, spot hearts, cancellation hearts, partnership Hearts, shoot the sun, moon subtraction choice, Q-spades-breaks-hearts variant, variable seats. | Out of Gate 16 scope. | yes |
| public name | Briar Circuit. | Original neutral Rulepath identity; "Hearts" is source/research label only. | yes |

## Ambiguity Log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `BC-AMB-001` | Whether queen of spades breaks hearts. | Pagat and Bicycle-style descriptions. | Queen of spades does not break hearts; only a heart does. | `BC-PLAY-005`, `BC-PLAY-006`, `BC-PLAY-007` | legality tests, broken-heart traces | resolved |
| `BC-AMB-002` | Whether point cards are ever allowed on trick one. | Common no-points rule and legal-set completeness requirement. | Forbid hearts and queen of spades only while a non-point discard exists; allow all if every held card is a point card. | `BC-PLAY-004` | positive/negative rule tests, exception trace | resolved |
| `BC-AMB-003` | Whether moon shooter may subtract 26 instead of adding 26 to opponents. | Common moon variants. | No choice; shooter adds 0 and each opponent adds 26. | `BC-SCORE-003`, `BC-OUTCOME-001` | moon unit/rule trace and outcome check | resolved |
| `BC-AMB-004` | Whether low-score ties at threshold use seat order. | Source tie-continuation notes and Rulepath no-arbitrary-tiebreak posture. | Continue complete hands until the lowest score is unique. | `BC-MATCH-003` | threshold-tie fixture/trace and simulation | resolved |
| `BC-AMB-005` | Whether pass provenance becomes public after a passed card is later played. | Rulepath hidden-info no-leak law and spec visibility taxonomy. | The card identity becomes public when played; who passed it remains private. | `BC-VIS-002`, `BC-REPLAY-002` | pairwise visibility/export/e2e tests | resolved |
| `BC-AMB-006` | Whether data files can encode pass routing, legality, scoring, or bot priorities. | Rulepath static-data boundary. | Data may carry typed metadata and fixtures only; behavior lives in Rust. | `BC-OOS-003` | strict parse, boundary, and code review | resolved |

## Public Naming Rationale

Public ID: `briar_circuit`

Display name: `Briar Circuit`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | avoid for public identity | "Hearts" is common, but Rulepath favors original catalog identity where presentation risk is avoidable. |
| neutral name chosen? | yes | "Briar" evokes penalty pressure; "Circuit" evokes passing/turn cycles. |
| trademark risk considered? | yes | Public copy avoids affiliation, product branding, slogans, logos, and conventional package identity. |
| trade-dress risk considered? | yes | Public presentation must use original cards/table/iconography and no copied layouts or art. |
| rules-family label retained? | yes, only in source notes | "Classic Hearts" may be used to describe the researched family, not the public product brand. |

## Trademark And Trade-Dress Concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Hearts-family similarity | low for abstract public-domain mechanics, expression still reviewed | Use original prose, name, assets, icon, table layout, and UI copy. | no |
| Conventional card imagery | medium if copied suit art/card faces appear | Use original or reviewed assets; do not copy card faces, icons, scans, fonts, or product presentation. | yes if copied or trade-dress-like |
| Casino/real-money association | low to medium through playing-card presentation | Use board-game language; no wagering, payout, chip, casino, or affiliation framing. | yes if introduced |
| Source phrasing | medium if paraphrased too closely | Maintain consulted-not-copied notes and original Rulepath phrasing. | yes if copied prose appears |

## Asset Provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---:|---|---|---:|
| Game docs | `games/briar_circuit/docs/RULES.md`, `games/briar_circuit/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied source rules or tables. | yes |
| Public game name | `Briar Circuit` | original project name | Rulepath/Codex-authored public name | Avoids using Hearts as product identity. | yes |
| Card ids and labels | standard rank/suit labels planned in game-local Rust/static metadata | original implementation expression over common card facts | Rulepath/Codex-authored labels and IDs | Standard card identities are common facts; rendered card faces/assets must still be original or reviewed. | pending later asset review |
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
| Rulepath original rules summary | yes | `games/briar_circuit/docs/RULES.md` | Original Rulepath prose. |
| Gate 16 project-authority facts | yes | `games/briar_circuit/docs/SOURCES.md` | Summarized as rationale only. |
| Generic Hearts/trick-taking family facts | yes | `games/briar_circuit/docs/SOURCES.md` | Used as context, not copied expression. |
| Public source prose, examples, tables, diagrams, or screenshots | no | none | No source prose or visual source material is copied. |
| Commercial/licensed rules text | no | none | No source material is redistributed. |
| Commercial card faces, suit art, product names, icons, screenshots, fonts, or table presentation | no | none | No assets introduced in this ticket. |
| Private licensed stress-test content | no public shipment | none | No private licensed content is involved. |

Private licensed stress tests are late, isolated, optional, non-public, and
non-architectural. They must not contaminate `engine-core`, public assets,
public docs, or public web bundles.

## Human/Legal Review Triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no for current name | Public name is neutral and original. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | Standard card facts only; labels/IDs are authored locally. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no in this ticket | Later renderer/icon work must review assets. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no in this ticket | Later generated assets require review notes. |
| uncertainty about public-domain status | no for abstract rules-family facts | No source material is redistributed. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release Blocking Concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| visual/card assets not yet reviewed | later-ticket blocker if introduced | Renderer/icon tickets must record asset provenance and trade-dress review. | Rulepath |

## Rule-Source-To-Rule-ID Cross-Reference

Every release-relevant stable rule ID in `RULES.md` must have source or design
rationale support here. Briar Circuit uses public rules-family facts expressed
in original Rulepath prose, with project authority from the Gate 16 spec.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `BC-SETUP-001` through `BC-SETUP-002` | Fixed four seats and canonical 52-card deck. | Gate 16 spec, Pagat, Bicycle. | no | Vocabulary remains game-local. |
| `BC-DEAL-001` through `BC-DEAL-002` | Seeded full deal, 13 cards per seat, dealer rotation. | Gate 16 spec and rules-family references. | no | Hidden deal facts remain private. |
| `BC-PASS-001` through `BC-PASS-004` | Left/right/across/hold pass cycle, three-card commitments, atomic exchange, hold hand. | Gate 16 spec, Pagat, Bicycle. | yes | Pass privacy/provenance is Rulepath-specific no-leak policy. |
| `BC-PLAY-001` through `BC-PLAY-007` | 2 clubs opening, follow-suit, void discard, first-trick point restriction, hearts-broken restrictions, queen of spades non-breaking choice. | Gate 16 spec, Pagat, Bicycle. | yes | Source divergences resolved by the spec. |
| `BC-TRICK-001` through `BC-TRICK-002` | Highest led suit wins; winner captures and leads next. | Gate 16 spec and trick-taking family facts. | no | Off-suit never wins. |
| `BC-SCORE-001` through `BC-SCORE-003` | Heart/queen point values, 26-point conservation, fixed moon transform. | Gate 16 spec, Pagat, Bicycle. | yes | No moon choice. |
| `BC-MATCH-001` through `BC-MATCH-003` | Cumulative scores, 100 threshold, unique low score wins, low ties continue. | Gate 16 spec and rules-family references. | yes | No seat-order tiebreaker. |
| `BC-VIS-001` through `BC-VIS-004` | Private hands, pass privacy/provenance, deck material redaction, private action/effect/bot surface protection. | Gate 16 spec and Rulepath no-leak law. | yes | Pairwise no-leak evidence required. |
| `BC-REPLAY-001` through `BC-REPLAY-002` | Internal deterministic replay and viewer-scoped exports. | Gate 16 spec and Rulepath replay law. | no | Public export must not reconstruct hidden cards. |
| `BC-BOT-001` through `BC-BOT-002` | L0 random legal and bounded L1 inputs. | Gate 16 spec and Rulepath public-bot law. | no | Search/ML bots are forbidden. |
| `BC-UI-001` | Browser controls expose legal Rust actions only. | Gate 16 spec and Rulepath UI law. | no | TypeScript computes no legality. |
| `BC-OUTCOME-001` | Rust-authored per-seat scoring and terminal breakdown. | Gate 16 spec and official-game outcome law. | no | TypeScript renders supplied facts only. |

## Final Source/IP Checklist

- Sources are summarized in original language.
- Source quality and date consulted are recorded.
- Variant choices and deviations are explicit.
- Ambiguities connect to rule IDs and tests.
- Public naming avoids affiliation and trade-dress risk.
- Assets are original, verified public-domain, license-reviewed, or generated-reviewed before public use.
- Fonts are system-only or license-reviewed.
- Public/private content boundary is explicit.
- Human/legal review triggers are not hidden.
- Release blockers are recorded.
