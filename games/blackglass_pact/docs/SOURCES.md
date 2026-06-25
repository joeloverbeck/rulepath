# Blackglass Pact Sources

Game ID: `blackglass_pact`

Public display name: `Blackglass Pact`

Implemented variant: `blackglass_pact_standard`

Prepared by: `Codex`

Created: 2026-06-25

Last updated: 2026-06-25

Rules version connected to this source note: `blackglass-pact-rules-v1`

## Source-Use Statement

Blackglass Pact is an original Rulepath implementation in the classic
four-player partnership Spades rules family. External references were consulted
only to verify public rules-family facts and common variant choices: fixed
opposite partnerships, a standard 52-card deck, spades as permanent trump,
round-table bidding, nil and blind-nil variants, follow-suit play,
broken-spades lead restrictions, partnership contracts, bags, 500-point target,
and common tie-continuation handling.

No source rules prose, examples, card imagery, product naming, component text,
icons, screenshots, scans, fonts, assets, art direction, table layout, or trade
dress is copied. Rulepath rule prose, UI copy, visual presentation, assets,
icons, card ids, and component text for `blackglass_pact` are original.

Public presentation must use **Blackglass Pact**. "Spades" may appear only as a
rules-family label in source notes and explanatory maintenance context. It must
not be used as the public product identity, renderer identity, asset theme, or
trade-dress target.

Human IP/public-release review is pending. The exact-title screening recorded
below is only a preliminary collision check and is not legal clearance.

## Consulted Sources

All sources in this table are rationale, project-authority, taxonomy,
accessibility, implementation-prior-art, or mechanic-context sources only. No
source prose, code, APIs, assets, or presentation are copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 18 Blackglass Pact spec | `../../../specs/gate-18-blackglass-pact-spades-partnership-trick-taking.md` | 2026-06-25 | project authority | product scope, locked variant, rule IDs, scoring model, visibility taxonomy, command suite, docs, tests, replay, bot, WASM, UI, benchmark, and capstone obligations | none | Governs this gate until archived. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-25 | project authority | requirements-first workflow and official-game evidence | none | Governs rules summary, source notes, player rules, rule coverage, outcome docs, no-leak proof, and web exposure. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-25 | project authority | naming, source-use limits, and public asset caution | none | Supports the original public name and forbids copied prose, assets, and trade dress. |
| Rulepath repository source bibliography | `../../../docs/SOURCES.md` | 2026-06-25 | project authority | central bibliography pattern and source-use rules | none | The capstone adds the repo-level Blackglass Pact entry. |
| Pagat Spades | `https://www.pagat.com/auctionwhist/spades.html` | 2026-06-25 | public rules-family reference | fixed partnerships, round-table bidding, plus/minus 10x contract, failed-nil attribution, nil values, blind nil values, bag rollover, 500 target, documented variants | none | Consulted for facts and variant comparison only. |
| Bicycle Cards Spades | `https://bicyclecards.com/how-to-play/spades/` | 2026-06-25 | public rules-family reference | standard deck/rank, one-at-a-time deal from dealer's left, bidding/lead order, follow suit, spades trump, broken-spades exception, 13 tricks, common 500 target, extra-hand tie handling | none | Bicycle's zero-on-set scoring is contrast evidence and is not adopted. |
| Trickster Cards Spades Basics | `https://www.trickstercards.com/help/spades/` | 2026-06-25 | public rules-family reference | team bids, nil values, play flow, public score concepts | none | Used as secondary common-rules confirmation. |
| Trickster Cards Spades Rules | `https://www.trickstercards.com/help/spades-rules/` | 2026-06-25 | public rules-family reference | blind nil before cards are revealed, 100-point trailing eligibility, no-pass choices, broken-spades options, 10-bag penalty, nil values, failed-nil alternatives | none | Helps document selected and rejected house-rule options. |
| CardGames.io Spades article | `https://cardgames.io/blog/how-to-play-spades/` | 2026-06-25 | public teaching reference | accessible secondary confirmation of partnership bidding, play, and scoring family | none | No prose or UI is copied. |
| University of Chicago IM Spades rules | `https://athletics.uchicago.edu/sports/2023/6/12/intramurals-im-rules-spades.aspx` | 2026-06-25 | institutional rules reference | broken-spades, plus/minus 10x contract, 10-bag rollover, 500 target, and contrast for failed-nil/no-bag rules | none | Blackglass Pact deliberately selects the Pagat/Trickster failed-nil-as-bags alternative. |
| `lukiffer/SpadesBot` | `https://github.com/lukiffer/SpadesBot` | 2026-06-25 | external implementation prior art | separation of blind decision from later hand-bearing deal/bid input | none | No code or API shape is imported. |
| Cohensius, Meir, Oved, Stern, "Bidding in Spades" | `https://arxiv.org/abs/1912.11323` | 2026-06-25 | research context | distinction between bidding and play plus domain-feature motivation | none | The machine-learning/expected-utility method is not adopted. |
| Pagat Spades Strategy and Tips | `https://www.pagat.com/auctionwhist/spadetip.html` | 2026-06-25 | strategy background | bidding controls, nil risk, partnership play, setting, and bag pressure | none | Later L1 policy prose remains original and rules-checked. |
| Bounded title screening for "Blackglass Pact" | search screening note | 2026-06-25 | preliminary project screening | no prominent exact game-title conflict found in screening results | none | Not legal clearance; human review remains mandatory. |
| WCAG 2.2 | `https://www.w3.org/TR/WCAG22/` | 2026-06-25 | accessibility reference | overall accessibility acceptance posture | none | Used for UI acceptance context, not rules behavior. |
| WAI Understanding SC 1.4.1 Use of Color | `https://www.w3.org/WAI/WCAG21/Understanding/use-of-color.html` | 2026-06-25 | accessibility reference | non-color-only partnership identity | none | Used for UI acceptance context. |
| WAI Understanding SC 2.4.3 Focus Order | `https://w3c.github.io/wcag/understanding/focus-order.html` | 2026-06-25 | accessibility reference | seat/team/control reading and focus order | none | Used for UI acceptance context. |
| WAI Understanding SC 4.1.3 Status Messages | `https://www.w3.org/WAI/WCAG22/Understanding/status-messages.html` | 2026-06-25 | accessibility reference | blind/bid/trick/score announcements without focus theft | none | Used for UI acceptance context. |
| WAI Understanding SC 4.1.2 Name, Role, Value | `https://www.w3.org/WAI/WCAG21/Understanding/name-role-value.html` | 2026-06-25 | accessibility reference | controls, groups, replay, and score semantics | none | Used for UI acceptance context. |
| `dozingcat/CardsWithCats` Spades implementation | `https://github.com/dozingcat/CardsWithCats/blob/master/lib/spades/spades.dart` | 2026-06-25 | external implementation prior art | explicit teams, bags, target score, and local game modeling | none | No code, architecture, or bot technique is imported. |

## Adopted Design Facts

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Public identity | The public game is Blackglass Pact; "Spades" is a rules-family/source label only. | Original identity reduces source confusion and trade-dress risk. |
| Four seats | Exactly four stable seats play each match. | Classic partnership Spades baseline and Gate 18 scope. |
| Fixed partnerships | `team_0` is North-South and `team_1` is East-West. | Opposite fixed partnerships are the selected family shape. |
| Standard deck | A local standard 52-card deck is shuffled and fully dealt. | Common family baseline and 13-card hand structure. |
| Dealer and first leader | Initial dealer is `seat_0`; dealer rotates after non-terminal hands; first leader is left of dealer. | Fixed start supports reproducible fixtures; rotation supports multi-hand proof. |
| Blind nil before deal | Blind commitments happen before shuffle/deal and before card identity exists in any projected surface. | Makes no-leak and RNG independence unambiguous. |
| Blind nil eligibility | A seat is eligible only when its team trails by at least 100 points. | Documented common option selected by the spec. |
| No nil passing | No card exchange or partner consultation follows nil or blind nil. | Passing is a variant; excluding it keeps the hidden-hand boundary narrow. |
| Bid vocabulary | Legal bids are nil and 1 through 13; no numeric zero or total-13 hook. | Nil replaces zero; Vow Tide's exact-bid hook is not part of this family lock. |
| Team ordinary contract | Positive numeric partner bids sum into the team contract. | Standard partnership scoring shape. |
| Individual nils | Nil and blind nil remain seat-keyed and contribute zero to the ordinary contract. | Preserves individual nil outcomes inside team scoring. |
| Spades trump | Spades always trump; broken-spades lead restriction applies. | Common family baseline. |
| Follow suit | A follower holding led suit must play it; void followers may play any owned card. | Core trick-taking rule and promoted helper fit. |
| Trick winner | Highest spade wins, otherwise highest led-suit rank wins. | Fits the promoted comparator with spades as caller-projected trump. |
| Ordinary scoring | Made/set ordinary contract scores plus/minus 10 times contract. | Selected partnership form. |
| Bags | Made overtricks and failed-nil tricks add bag points and persistent bags; every 10 bags applies -100 and rolls over. | Selected sandbagging form. |
| Failed nil attribution | Failed nil tricks do not help the ordinary contract and do add bags. | Selected Pagat/Trickster alternative over no-bag failed nil. |
| Target | A unique higher team at 500 or more wins after a completed hand; exact ties continue. | Avoids arbitrary bag, seat, or dealer tiebreaks. |

## Variant Choice And Deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `blackglass_pact_standard` / Blackglass Pact | Gate 18 partnership trick-taking proof scope. | yes |
| player count | Exactly four seats. | Roadmap and spec lock fixed-four partnership proof. | yes |
| partnerships | Fixed opposite teams, North-South and East-West. | Classic family baseline with stable Rulepath IDs. | yes |
| blind nil | Available only when trailing by at least 100, declared before deal. | Common option selected for a strong no-leak proof. | yes |
| nil passing | Excluded. | Variant option; out of scope for this gate. | yes |
| bidding | Nil and 1-13 only; no zero/pass/minimum/total-bid hook. | Spec selects a narrow family shape and keeps Vow Tide's hook out. | yes |
| failed nil tricks | Do not help ordinary contract; add bag points and bags. | Selected common variant; UChicago-style no-bag contrast is rejected. | yes |
| bag threshold | Every 10 bags is -100, repeated, with remainder. | Common sandbagging form. | yes |
| terminal tie | Exact tie at or above 500 continues. | Avoids arbitrary tiebreaks. | yes |
| public name | Blackglass Pact. | Original neutral Rulepath identity. | yes |
| optional rules excluded | Jokers, deuce-high, no-trump, pass cards, moon/Boston, mirrors, suicide, bags-off, variable targets, hosted play. | Out of Gate 18 scope. | yes |

## Ambiguity Log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `BP-AMB-001` | Whether blind nil happens after a hidden deal or before any card identity exists. | Pagat/Trickster family descriptions and Rulepath no-leak law. | Commit before shuffle/deal. | `BP-BLIND-002`, `BP-BLIND-006`, `BP-BLIND-007` | blind no-leak corpus, paired-seed deal test | resolved |
| `BP-AMB-002` | Whether a blind-nil declaration allows card passing. | Pagat and Trickster variant options. | No passing/exchange/consultation. | `BP-BLIND-009` | action-tree and no-leak tests | resolved |
| `BP-AMB-003` | Whether numeric zero is a separate bid. | Common nil vocabulary and selected variant. | Nil is the only zero-trick bid. | `BP-BID-003`, `BP-BID-004` | bid tree boundary tests | resolved |
| `BP-AMB-004` | Whether a dealer total-bid hook applies. | Vow Tide contrast and Spades source variants. | No total-13 hook. | `BP-BID-006` | regression test contrasting Vow Tide | resolved |
| `BP-AMB-005` | Whether failed nil tricks help the partner's ordinary contract. | Pagat, Trickster, UChicago contrast. | They do not help ordinary contract; they add bags. | `BP-SCORE-009`, `BP-SCORE-010` | scoring/unit traces | resolved |
| `BP-AMB-006` | Whether exact ties at or above 500 use bags, dealer, or seat order. | Family references and Rulepath no-arbitrary-tiebreak posture. | Continue complete hands. | `BP-END-004`, `BP-END-006` | terminal tie trace | resolved |
| `BP-AMB-007` | Whether partner cards are team-private. | Rulepath hidden-info law and fixed-team scope. | No team-private hand visibility exists. | `BP-SETUP-004`, `BP-VIS-003` | pairwise partner no-leak matrix | resolved |

## Public Naming Rationale

Public ID: `blackglass_pact`

Display name: `Blackglass Pact`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | avoid as public identity | "Spades" is a common rules-family term, but Rulepath uses an original catalog identity. |
| neutral name chosen? | yes | "Blackglass" evokes the permanent black trump suit without suit glyphs; "Pact" evokes fixed partnerships and public commitments. |
| trademark risk considered? | yes | A bounded exact-title screening found no prominent exact game-title conflict, but this is not legal clearance. |
| trade-dress risk considered? | yes | Public presentation must use original cards, table layout, iconography, and copy. |
| rules-family label retained? | yes, only in source notes | "Spades" may describe the researched family, not the product brand. |

## Trademark And Trade-Dress Concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Spades-family similarity | low for abstract public-domain mechanics, expression still reviewed | Use original prose, name, assets, icon, table layout, and UI copy. | no |
| conventional playing-card imagery | medium if copied suit art/card faces appear | Use original or reviewed assets; do not copy card faces, icons, scans, fonts, or product presentation. | yes if copied or trade-dress-like |
| casino/real-money association | medium through Spades/card-table presentation | Use board-game language; no wagering, payout, chip, casino, or affiliation framing. | yes if introduced |
| source phrasing | medium if paraphrased too closely | Maintain consulted-not-copied notes and original Rulepath phrasing. | yes if copied prose appears |
| public title | pending human review | Exact-title screening is preliminary only. | yes before public release |

## Asset Provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---:|---|---|---:|
| Game docs | `games/blackglass_pact/docs/RULES.md`, `games/blackglass_pact/docs/SOURCES.md`, `games/blackglass_pact/docs/RULE-COVERAGE.md` | original | Rulepath/Codex-authored prose | No copied source rules or tables. | yes |
| Public game name | `Blackglass Pact` | original project name | Rulepath/Codex-authored public name | Pending human release review. | pending |
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
| Rulepath original rules summary | yes | `games/blackglass_pact/docs/RULES.md` | Original Rulepath prose. |
| Gate 18 project-authority facts | yes | `games/blackglass_pact/docs/SOURCES.md` | Summarized as rationale only. |
| Generic Spades/trick-taking family facts | yes | `games/blackglass_pact/docs/SOURCES.md` | Used as context, not copied expression. |
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
| commercial game name or recognizable branding | no for current name; title still pending release review | Public name is original and screening found no prominent exact-title conflict. |
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
| human IP/public-release review pending | yes before public release | Maintainer/human review of name, presentation, source use, assets, and release checklist. | Rulepath maintainers |
| visual/card assets not yet reviewed | later-ticket blocker if introduced | Renderer/icon tickets must record asset provenance and trade-dress review. | Rulepath maintainers |

## Rule-Source-To-Rule-ID Cross-Reference

Every release-relevant stable rule ID in `RULES.md` must have source or design
rationale support here. Blackglass Pact uses public rules-family facts
expressed in original Rulepath prose, with project authority from the Gate 18
spec.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `BP-ID-001` through `BP-ID-002` | Identity, variant, public name, rules/data versions. | Gate 18 spec, Rulepath IP policy. | yes | Public release review remains pending. |
| `BP-SETUP-001` through `BP-SETUP-006` | Fixed four seats, stable seat/team IDs, initial dealer, replay inputs. | Gate 18 spec, Pagat, Bicycle, Trickster. | yes | Stable IDs are Rulepath formalization. |
| `BP-BLIND-001` through `BP-BLIND-010` | Blind-nil eligibility, timing, actions, no-leak, RNG independence, no passing, skipped bidding. | Gate 18 spec, Pagat, Trickster, external prior-art separation note. | yes | Pre-deal commitment is Rulepath's no-leak formalization. |
| `BP-DEAL-001` through `BP-DEAL-006` | Standard deck, clockwise full deal, 13 private cards, public counts, replay stability. | Gate 18 spec, Bicycle, Trickster, Pagat. | no | Deal bytes independent of blind choices. |
| `BP-BID-001` through `BP-BID-010` | Bidding order, nil/1-13 leaves, immutable public bids, team contract aggregation, no total hook. | Gate 18 spec, Pagat, Bicycle, Trickster, CardGames.io. | yes | Vow Tide hook deliberately excluded. |
| `BP-PLAY-001` through `BP-PLAY-012` | First leader, broken-spades lead restriction, follow suit, trump resolution, helper reuse. | Gate 18 spec, Bicycle, Trickster, CardGames.io, game-stdlib helper authority. | yes | Shared helper is reused unchanged. |
| `BP-SCORE-001` through `BP-SCORE-016` | Contract, nil, blind nil, failed nil, bags, rollover, ordered scoring breakdown. | Gate 18 spec, Pagat, Trickster, CardGames.io, UChicago contrast. | yes | Failed-nil-as-bags selected. |
| `BP-END-001` through `BP-END-010` | Completed-hand terminal evaluation, 500 target, exact tie continuation, stable standings. | Gate 18 spec, Pagat, Bicycle, Trickster. | yes | No arbitrary tiebreaker. |
| `BP-VIS-001` through `BP-VIS-008` | Public observer, seat-private hand, partner no-leak, blind no-leak, safe diagnostics, viewer-scoped export, browser no-leak. | Rulepath foundations, Gate 18 spec, multi-seat/no-leak contracts. | yes | Rulepath visibility law is authoritative. |
| `BP-REPLAY-001` through `BP-REPLAY-003` | Deterministic replay, Trace Schema v1, scoped golden changes. | Rulepath replay/fixture/hash law and Gate 18 spec. | no | No schema migration authorized. |
| `BP-BOT-001` through `BP-BOT-004` | L0 random legal, bounded L1 authorized inputs, safe explanations, prohibited algorithms absent. | Rulepath AI law, Gate 18 spec, Spades bidding/strategy sources. | yes | Research methods are not adopted. |
| `BP-UI-001` through `BP-UI-006` | Grouped partnership UI, Rust legal controls, Rust-derived score/outcome, hotseat erasure, accessible observer/replay/outcome, reduced motion. | Rulepath UI law, Gate 18 spec, WCAG/WAI references. | no | TypeScript remains presentation-only. |

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
- Human IP/public-release review remains pending.
