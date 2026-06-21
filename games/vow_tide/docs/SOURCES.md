# Vow Tide Sources

Game ID: `vow_tide`

Public display name: `Vow Tide`

Implemented variant: `vow_tide_standard`

Prepared by: Codex

Created: 2026-06-21

Last updated: 2026-06-21

Rules version connected to this source note: `vow-tide-rules-v1`

## Source-use statement

Rulepath uses consulted sources to verify rules-family facts, variant choices,
terminology context, ambiguity resolution, and strategy prior art. Sources do
not authorize copied prose, card text, examples, score sheets, icons, board art,
screenshots, scans, fonts, assets, UI copy, or trade dress. Public Rulepath
rules, help text, component labels, visuals, iconography, and presentation must
be original, project-owned, public-domain where verified, or separately
license-reviewed.

This note summarizes facts in original Rulepath language. It does not paste
external rules text.

## Consulted sources

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Pagat: Oh Hell | `https://www.pagat.com/exact/ohhell.html` | 2026-06-21 | reputable rules authority | player-count range, changing hand-size schedule, turn-up trump, bid order, dealer hook, follow-suit, trick winner, scoring variants, tie variants | none | Primary family reference for public-domain/common rule facts. |
| Trickster Cards: Oh Hell rules | `https://www.trickstercards.com/game/oh-hell/rules/` | 2026-06-21 | reputable secondary / online implementation rules | schedule comparison, bidding and hook comparison, play sequence, scoring-family context | none | Used only as a comparison source; no UI or copy is imported. |
| Haskell Oh Hell prior-art implementation | referenced by Gate 17 spec Appendix C | 2026-06-21 | prior-art implementation / strategy context | L1 non-search ideas: estimate bids from own cards and change play posture relative to contract | none | Strategy prior art only; no code, formulas, probability text, or architecture is imported. |
| Scala Oh Hell prior-art implementation | referenced by Gate 17 spec Appendix C | 2026-06-21 | prior-art implementation / variant context | notes alternate schedule/order, random trump, total-bid constraint, and contract scoring variants | none | Prior art only; networking, one-card visibility, negative scoring, and architecture are excluded. |
| Rulepath Gate 17 spec | `specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md` | 2026-06-21 | project authority | locked Vow Tide variant, neutral naming, exact deviations, proof obligations, no-leak matrix, bot limits | project-authored | Governing implementation source under foundation docs. |
| Rulepath IP policy | `docs/IP-POLICY.md` | 2026-06-21 | repository law | original prose, neutral naming, no-copy, asset/font review, release triggers | project-authored | Controls public artifact posture. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `vow_tide_standard` | Gate 17 spec locks one official exact-bid changing-hand-size variant. | yes |
| player count | 3-7 seats; default 4 | Pagat records 3-7 as the family range; roadmap/spec require the full range. | yes |
| deck | one 52-card deck, four suits, ranks 2 through ace, no jokers | Common source baseline. | no |
| maximum hand size | `K=min(10,floor(51/N))`: 10 for 3-5 seats, 8 for 6, 7 for 7 | Reconciles Pagat-style maxima with the reserved trump indicator. | yes |
| schedule | descend from `K` to 1, then ascend from 2 to `K`; one-card hand once | Pagat/Trickster comparison plus locked Rulepath schedule. | yes |
| trump | next undealt card is public trump indicator; remaining stock hidden | Common source shape; Rulepath makes stock redaction explicit. | yes |
| bid order and hook | left of dealer through dealer; dealer may not make total bids equal hand size | Common source shape; Gate 17 pins the exact `H-S` rule. | yes |
| bid mutability | accepted bids are immutable | Rulepath replay/command-log decision; some table variants are looser. | yes |
| scoring | exact bid scores `10+bid`; every miss scores zero | Pagat names this simple family form; Rulepath excludes consolation and penalty variants. | yes |
| terminal tie | fixed schedule ends; equal high scores co-win with competition ranking | Tie-break hands are optional family variants; Rulepath keeps deterministic fixed length. | yes |
| L1 strategy | own-hand/public-fact rule-informed baseline only | Prior-art strategy ideas are useful, but hidden-world sampling/search is forbidden. | no |
| out-of-scope variants | teams, partnerships, secret bids, one-card forehead rule, extra tie hands, alternate schedules, no-trump, target score, negative penalties | Gate 17 scope control. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `VT-AMB-001` | Whether bids may be edited before the next bidder. | Pagat-style table rule context; Rulepath replay policy. | No edit action; accepted bids are immutable. | `VT-BID-PUBLIC-001` | duplicate bid diagnostic and immutable bid trace. | resolved |
| `VT-AMB-002` | Which scoring formula represents the shipped variant. | Pagat scoring variants; Trickster comparison; Gate 17 product rationale. | Exact scores `10+bid`; misses score zero. | `VT-SCORE-001` | exact-zero, exact-positive, under, over traces. | resolved |
| `VT-AMB-003` | Whether equal top scores force extra hands. | Pagat optional tie handling; fixed Rulepath schedule. | Co-winners share rank 1; no extra hand. | `VT-STANDINGS-001`, `VT-TERMINAL-001` | terminal co-winner trace. | resolved |
| `VT-AMB-004` | Whether the one-card hand is duplicated on the way down/up. | Source schedule variants; Gate 17 locked sequence. | One-card hand occurs once; ascending resumes at 2. | `VT-SCHEDULE-001` | schedule property tests for N=3..7. | resolved |
| `VT-AMB-005` | Whether the one-card hand uses forehead/other-player visibility. | Known family variants; hidden-information policy. | Every seat sees only its own card. | `VT-VIEW-001` | one-card no-leak tests. | resolved |

## Public naming rationale

Public ID: `vow_tide`

Display name: `Vow Tide`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | constrained | "Oh Hell" is a common rules-family label but is not the selected product name. |
| neutral name chosen? | yes | "Vow" describes the public exact-trick commitment; "Tide" describes the hand-size flow. |
| trademark risk considered? | yes | Neutral original title avoids product-source confusion. |
| trade-dress risk considered? | yes | Renderer/icon work must avoid existing app/table/card presentation mimicry. |
| name/trade-dress risk mitigated? | constrained | Naming is mitigated; final public icon/table art needs human IP/public-release review. |
| casino/brand term avoided? | yes | Public copy avoids casino/stakes framing. |
| affiliation implication avoided? | yes | Docs and UI must not imply affiliation with source sites or commercial apps. |
| public help text needs disclaimer? | no | Source notes are enough unless later review requests a disclaimer. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---|---|---:|
| Public use of "Oh Hell" as product identity | medium | Use Vow Tide for public product identity; keep "Oh Hell" as rules-family/source label. | no |
| Existing online card-app layouts or visual table style | human review needed | Build original Rulepath SVG/card/table presentation; do not copy screenshots, layouts, icons, or color treatments. | yes before public release |
| Generated or custom icon assets | human review needed | Record prompt/review or project-authored SVG notes when asset lands. | yes before public release |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| Game docs | `games/vow_tide/docs/RULES.md`, `games/vow_tide/docs/SOURCES.md`, `games/vow_tide/docs/GAME-IMPLEMENTATION-ADMISSION.md` | original | Rulepath/Codex-authored prose from project spec and summarized source facts | No copied rules text or examples. | yes |
| Card/component visuals | future renderer assets | human review needed | not yet created | Must be original/project-owned or reviewed generated assets. | no |
| Public icon | future web/catalog asset | human review needed | not yet created | Ticket 018/019 must record provenance. | no |
| Fonts | system font stack only unless later reviewed | safe by default | no bundled font files in this ticket | Later UI work must preserve this unless reviewed. | yes |

## Original prose and asset plan

| Public artifact area | Original prose/asset plan | Source facts allowed | Copied material excluded | Review owner |
|---|---|---|---|---|
| rules prose | Use Rulepath-authored `RULES.md` for formal contract and a later original `HOW-TO-PLAY.md` for players. | public-domain/common rule facts, locked Gate 17 choices, source comparisons | copied rules prose, examples, score sheets, diagrams, help text | Rulepath maintainers |
| UI/help copy | Later public help explains Vow Tide in Rulepath wording and avoids source-app phrasing. | selected schedule, bidding, hook, play, scoring, visibility facts | copied UI copy, strategy copy, marketing copy | Rulepath maintainers |
| visual assets | Use original Rulepath card/table/icon presentation or reviewed generated/project-owned assets. | generic card suits/ranks as common components | copied art, scans, screenshots, proprietary icons, fonts, trade dress | Rulepath maintainers |

## Generated asset review notes

| Generated asset | Tool/model if known | Prompt/source inputs safe? | Similarity/trade-dress risk | Human review result | Public-safe? |
|---|---|---:|---|---|---:|
| _None in this ticket_ | not applicable | not applicable | not applicable | not applicable | not applicable |

## Font status

| Font | Source/license | Bundled in public artifact? | Review status | Notes |
|---|---|---:|---|---|
| system font stack | not bundled | no | safe by default | No font file lands in ticket 001. |

## Public/private content boundary

| Content | Public allowed? | Location | Notes |
|---|---:|---|---|
| Rulepath original rules summary | yes | [RULES.md](RULES.md) | Project-authored. |
| Public-domain/common rules-family facts | yes | this note and later player help | Summarized, not copied. |
| Commercial/licensed rules text | no | none | No copied external text. |
| Private licensed stress-test content | no public shipment | none | No private content is touched. |
| Source screenshots/scans | no by default | none | Not used. |

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | yes | "Oh Hell" remains a source label; Vow Tide mitigates product identity risk. Final review still required. |
| copied or closely paraphrased rules prose | no | This ticket uses original prose. |
| card/component text from a protected source | no | Only ordinary rank/suit labels are expected. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no | No assets land in this ticket. |
| bundled font file | no | System fonts only so far. |
| generated art with possible trade-dress similarity | no | No generated art lands in this ticket. |
| uncertainty about public-domain status | no | Mechanics/family facts are treated as public/common; expression is not copied. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release blocking concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| Final public icon/table/card asset review | yes before release | Record original/project-owned/generated-review provenance in release checklist. | Rulepath maintainers |
| Human IP/public-release review | yes before release | Close in ticket 022 after docs, assets, public rules, web surfaces, traces, and bundles exist. | Rulepath maintainers |

## Rule-source-to-rule-ID cross-reference

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `VT-IDENTITY-001` | One Vow Tide standard variant, neutral name, rules/data versions. | Gate 17 spec; IP policy. | yes | "Oh Hell" not used as product identity. |
| `VT-SEATS-001` | 3-7 seats, default 4, stable seat IDs/labels. | Pagat range; roadmap/spec. | no | Full range is official. |
| `VT-CARDS-001` | Standard 52-card deck, no jokers. | Common source baseline. | no | Original IDs/labels required. |
| `VT-SCHEDULE-001` | `K=min(10,floor(51/N))`, down to one then back up. | Pagat/Trickster comparison; Gate 17 derivation. | yes | One-card hand once. |
| `VT-DEALER-001` | `seat_0` starts; dealer rotates after each hand. | Common family rule; deterministic setup rationale. | yes | Fixed initial dealer for reproducibility. |
| `VT-DEAL-001` | Deterministic shuffle and clockwise deal. | Common deal shape; Rulepath deterministic replay law. | yes | Per-hand seed partition is Rulepath-specific. |
| `VT-TRUMP-001` | Turn-up trump indicator; hidden stock. | Common family rule; Rulepath no-leak law. | no | Indicator is not playable. |
| `VT-BID-ORDER-001` | Sequential public bids left of dealer through dealer. | Pagat/Trickster comparison. | no | Submitted bids are public. |
| `VT-BID-RANGE-001` | Bids are `0..=H`. | Pagat/Trickster comparison. | no | Validator and legal tree must agree. |
| `VT-HOOK-001` | Dealer excludes `H-S` when in range. | Pagat/Trickster comparison; Gate 17 exact pinning. | no | Handles out-of-range prefix sums. |
| `VT-BID-PUBLIC-001` | Accepted bids are immutable and public. | Rulepath replay/command rationale. | yes | Excludes bid-change option. |
| `VT-FIRST-LEAD-001` | Left of dealer leads first trick; any card may lead. | Common family rule; Gate 17 locked variant. | no | Trump can be led. |
| `VT-FOLLOW-001` | Must follow suit if able; void may play any. | Common family rule; trick-taking helper decision. | no | Pure subset uses promoted helper. |
| `VT-TRICK-WIN-001` | Highest trump else highest led suit wins. | Common family rule; helper decision. | no | Off-suit non-trumps cannot win. |
| `VT-NEXT-LEAD-001` | Trick winner leads next trick. | Common family rule. | no | Game-local phase policy. |
| `VT-HAND-END-001` | Hand ends after H tricks. | Schedule/hand-size rule. | no | Conservation required. |
| `VT-SCORE-001` | Exact `10+bid`, miss zero. | Pagat scoring variant; Gate 17 product decision. | yes | Excludes consolation/penalty variants. |
| `VT-HAND-ADVANCE-001` | Record result, then advance dealer/schedule or terminal. | Gate 17 deterministic transition contract. | no | Effects ordered. |
| `VT-TERMINAL-001` | Match ends after final scheduled hand. | Source schedule; Rulepath fixed replay choice. | yes | No target score. |
| `VT-STANDINGS-001` | Highest score wins; equal high scores co-win; competition ranking. | Gate 17 tie policy. | yes | No extra tie hand. |
| `VT-VIEW-001` | Observer/seat projections and hidden hands/stock redaction. | Foundation no-leak law; Gate 17 matrix. | no | Exhaustive N-seat proof required later. |
| `VT-EFFECT-001` | Viewer-filtered semantic effects. | Architecture/UI contracts. | no | Effects are not animation instructions. |
| `VT-REPLAY-001` | Deterministic internal replay and viewer-scoped exports. | Trace Schema v1; ADR 0004 posture in Gate 17 spec. | no | No schema migration. |
| `VT-BOT-001` | L0/L1 use authorized views/legal leaves; no forbidden search/sampling/ML. | AI-BOTS; Gate 17 bot contract. | no | L2 not admitted. |
| `VT-OUTCOME-001` | Rust-authored seat-keyed rankings and hand breakdown. | Official-game outcome contract. | no | UI renders, not computes. |
| `VT-BOUNDARY-001` | Rust owns behavior; data/TypeScript stay non-authoritative. | Foundations and data boundary. | no | No kernel noun growth. |

## Final source/IP checklist

- Sources are summarized in original language.
- Source quality and date consulted are recorded.
- Variant choices and deviations are explicit.
- Ambiguities connect to rule IDs and future tests/traces.
- Public naming avoids affiliation and trade-dress risk.
- Assets are not added in this ticket; future assets require provenance review.
- Fonts are system-only unless later reviewed.
- Public/private content boundary is explicit.
- Human/legal review triggers are recorded for final closeout.
- Release blockers are recorded.
