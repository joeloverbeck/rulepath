# High Card Duel Sources

Game ID: `high_card_duel`

Public display name: `High Card Duel`

Implemented variant: `high_card_duel_standard`

Prepared by: `Codex`

Created: 2026-06-07

Last updated: 2026-06-07

Rules version connected to this source note: `high-card-duel-rules-v1`

## Source-use statement

Rulepath uses consulted sources to understand high-card comparison, hidden
information, chance, deterministic shuffle rationale, UI accessibility
expectations, and IP caution.

Sources do not authorize copied prose, card text, icons, screenshots, scans,
fonts, assets, art direction, component text, or trade dress. Rulepath rule
prose, UI copy, visual presentation, assets, icons, and component text for
`high_card_duel` are original.

High Card Duel is an original Rulepath ruleset. It uses the broad, uncopyrighted
idea that a higher numeric card can beat a lower numeric card, but it does not
copy War's full rules, text, examples, escalation procedures, card identities,
visual presentation, or commercial deck trade dress. It also does not implement
blackjack, poker, betting, chips, payouts, casino tables, or branded card-game
presentation.

## Consulted sources

All external sources in this table were consulted for the Gate 8 specification
and this source note. They are rationale sources only; no source prose or assets
are copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Bicycle Cards, War rules | `https://bicyclecards.com/how-to-play/war` | 2026-06-07 | commercial rules summary | high-card comparison context and War-not-copied review | none | Used only to identify what not to copy: War's public presentation and escalation shape are not implemented. |
| Pagat, War rules and notes | `https://www.pagat.com/war/war.html` | 2026-06-07 | reputable community reference | variant and ambiguity context for high-card comparison games | none | Used only as historical/contextual reference; no variant text, examples, or procedures are copied. |
| boardgame.io Game API, playerView | `https://github.com/boardgameio/boardgame.io/blob/main/docs/documentation/api/Game.md` | 2026-06-07 | project documentation | hidden-information projection precedent | none | Used as rationale that player-specific views are a known design concern, not as implementation authority. |
| OpenSpiel introduction | `https://openspiel.readthedocs.io/en/latest/intro.html` | 2026-06-07 | research project documentation | imperfect-information and chance-game context | none | Used for conceptual context only. |
| OpenSpiel paper | `https://arxiv.org/abs/1908.09453` | 2026-06-07 | research paper | imperfect-information and reproducible-game context | none | Used for rationale only; no algorithms or text are copied into game rules. |
| Ludii imperfect-information / nondeterminism universality paper | `https://arxiv.org/abs/2205.00451` | 2026-06-07 | research paper | hidden-information and nondeterminism context | none | Used for rationale only. |
| Lemire, Fast Random Integer Generation in an Interval | `https://arxiv.org/abs/1805.10941` | 2026-06-07 | research paper | bounded random integer caution for shuffle proof | none | Used to motivate unbiased bounded index generation rather than modulo reduction. |
| Fisher-Yates shuffle background | `https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle` | 2026-06-07 | secondary reference | shuffle algorithm naming and background | none | Used only for algorithm identity; implementation evidence must live in code/tests. |
| WAI-ARIA Authoring Practices: Grid Pattern | `https://www.w3.org/WAI/ARIA/apg/patterns/grid/` | 2026-06-07 | standards guidance | possible card/table keyboard interaction guidance | none | Used only if the UI adopts a grid-like keyboard pattern. |
| WAI Understanding WCAG SC 2.3.3: Animation from Interactions | `https://www.w3.org/WAI/WCAG21/Understanding/animation-from-interactions` | 2026-06-07 | standards guidance | reduced-motion rationale | none | Used as a UI acceptance reference for reveal and replay motion. |
| Rulepath Gate 7.2 + Gate 8 spec | `../../../archive/specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md` | 2026-06-07 | project authority | product scope, rule identity, source list, hidden-info proof requirements | none | Governs `high_card_duel` identity, rules, docs, tests, replay, bot, WASM, UI, benchmark, primitive-pressure, and checkpoint obligations. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-07 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, and official-game evidence. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-07 | project authority | naming and IP safety | none | Supports neutral public naming and forbids copied prose, assets, and trade dress. |

## Adopted design facts

The implemented `high_card_duel_standard` variant adopts these facts in original
Rulepath prose:

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Two-seat duel | Exactly two seats, `seat_0` and `seat_1`, play a finite six-round match. | Gate 8 needs a small hidden-information proof with clear viewer ownership. |
| Local duel deck | The game-local deck has 24 identities: two neutral sigils for each numeric rank `1` through `12`. | Distinct identities support private hands, reveal history, and replay while keeping comparison simple. |
| Deterministic shuffle | Setup shuffles with Rust-owned deterministic randomness and an unbiased bounded index. | Gate 8 proves chance without relying on browser state or modulo-biased hidden setup. |
| Private hands | Each seat sees only its own hand before reveal. | Hidden-information proof and no-leak tests require seat-private projections. |
| Face-down commitment | The lead commits first; the reply chooses without seeing the lead card identity. | This creates a compact hidden commitment surface. |
| Simultaneous reveal | Both committed cards become public together after the reply commits. | Prevents partial reveal leakage and creates a single public scoring point. |
| High rank scores | Higher numeric rank earns one point; equal ranks tie the round. | Simple comparison keeps the gate focused on hidden-info surfaces. |
| Six-round terminal | Round six ends the game; higher score wins, equal score draws. | Finite length keeps simulations and no-leak proof bounded. |

## War-not-copied rationale

| Concern | High Card Duel decision | Notes |
|---|---|---|
| Full War rule structure | Not copied. | High Card Duel has private hands, face-down lead/reply commitments, six fixed rounds, scoring by round, refill order, and no war escalation stack. |
| War prose or examples | Not copied. | Public rules are original Rulepath prose. |
| Standard playing-card identities | Not used. | The game uses local numeric ranks and neutral sigil identities, not suits/faces or a 52-card deck. |
| Commercial card presentation | Not used. | UI direction is a neutral wayfarer duel-table theme with no casino, poker, blackjack, betting, chips, or branded deck trade dress. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `high_card_duel_standard` | Gate 8 spec and Rulepath hidden-info proof scope. | yes |
| player count | Exactly two seats. | Gate 8 scope and viewer authorization model. | yes |
| first lead | `seat_0` leads round 1; lead alternates by round. | Deterministic replay and balanced turn order. | yes |
| round limit | Six rounds. | Gate 8 finite-proof scope. | yes |
| optional rule included | Private lead/reply commitment and simultaneous reveal. | Hidden-information proof requirement. | yes |
| optional rule excluded | War escalation, betting, blackjack decisions, poker hands, chips, payouts, suit hierarchy, postgame reveal-all. | Gate 8 scope and IP/product boundaries. | yes |
| Rulepath deviation from common high-card games | Private hands plus face-down lead/reply commitment replace public top-card flipping. | Original Rulepath design to test hidden-info surfaces. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `HCD-AMB-001` | Whether to use a standard 52-card deck. | Gate 8 spec, War sources, IP policy. | Use a local 24-card duel deck with numeric ranks and neutral sigils. | setup and round IDs in `RULES.md` | setup tests, serialization tests, no-leak tests | resolved |
| `HCD-AMB-002` | Whether equal ranks have a tie-breaker. | Gate 8 spec and high-card comparison context. | Equal ranks tie the round and award no point. | round IDs in `RULES.md` | tie-round golden trace and scoring tests | resolved |
| `HCD-AMB-003` | Whether terminal state reveals unused deck or hands. | Gate 8 spec and hidden-info no-leak law. | Terminal public export keeps unused deck tail and unplayed private cards hidden by default. | setup, round, action, and diagnostic IDs in `RULES.md` | terminal no-leak tests and public export/import trace | resolved |
| `HCD-AMB-004` | Whether the shuffle may use modulo reduction. | Gate 8 spec and bounded-random literature. | Use unbiased bounded index generation for shuffle proof. | setup IDs in `RULES.md` | RNG/shuffle tests and deterministic replay tests | resolved |

## Public naming rationale

Public ID: `high_card_duel`

Display name: `High Card Duel`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | yes | The name describes a generic high-card comparison and duel structure. |
| neutral name chosen? | yes | `High Card Duel` is Rulepath naming for this original variant. |
| trademark risk considered? | yes | Public docs and UI avoid product branding, source names, logos, slogans, and affiliation wording. |
| trade-dress risk considered? | yes | Cards, table, colors, labels, icons, layout, animation, and help text must be original Rulepath presentation. |
| affiliation implication avoided? | yes | Sources are cited only as rationale/context. |
| public help text needs disclaimer? | no | Neutral naming and source notes are sufficient unless later review requests additional text. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| High-card comparison as a common mechanic | low for mechanics, expression still reviewed | Use original prose and presentation. | no |
| War-specific presentation or escalation structure | medium if imitated | Do not implement War escalation, examples, source text, or visual style. | yes if found |
| Casino, blackjack, poker, betting, or chip presentation | medium to high if imitated | Keep theme neutral and avoid casino vocabulary and visuals. | yes if found |
| Standard playing-card faces, suits, or commercial deck art | medium if copied | Use local numeric ranks and neutral sigil identities with original artwork. | yes if copied |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| Game docs | `games/high_card_duel/docs/RULES.md`, `games/high_card_duel/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied rulebook text. | yes |
| Card IDs | `hcd:r01:a` through `hcd:r12:b` | original local identifiers | Rulepath/Codex-authored identifiers | Numeric ranks and neutral sigils only. | yes |
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
| Rulepath original rules summary | yes | `games/high_card_duel/docs/RULES.md` | Original Rulepath prose. |
| High-card comparison context | yes | `games/high_card_duel/docs/SOURCES.md` | Summarized as rationale only. |
| War source prose or examples | no | none | No source prose is copied. |
| Blackjack/poker/casino content | no | none | Explicitly out of scope. |
| Standard card art, scans, screenshots, icons, fonts, or trade dress | no | none | No assets introduced in this ticket. |
| Private licensed stress-test content | no public shipment | none | No private licensed content is involved. |

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Neutral public name only. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | Local numeric ranks and neutral identifiers only. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no | No art or UI asset in this ticket. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no | None. |
| uncertainty about public-domain status | no | Sources are rationale references; no source material is redistributed. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release blocking concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| none identified for this ticket | no | Continue evidence workflow in later tickets. | Rulepath |

## Rule-source-to-rule-ID cross-reference

Every release-relevant stable rule ID in `RULES.md` must have source or design
rationale support here. High Card Duel is original, so most entries cite
Rulepath design rationale rather than external rule authority.

| Rule family | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| setup | Local deck construction, deterministic shuffle, alternating private deal, initial score/phase/lead, internal deck storage. | Gate 8 spec, deterministic replay law, bounded-random rationale, original Rulepath design. | yes | Local 24-card deck replaces standard deck assumptions. |
| round | Lead/reply commitment, simultaneous reveal, rank comparison, tie handling, discard history, refill, lead alternation, six-round terminal scoring. | Gate 8 spec, original Rulepath design, high-card comparison context. | yes | Commitment/refill structure is original and not War. |
| action | Viewer-scoped private commit choices and empty terminal/observer trees. | Gate 8 hidden-info proof and Rulepath Rust-legality boundary. | no | Action labels are private data. |
| diagnostics | Public-safe wrong-seat, wrong-phase, invalid-card, stale, commitment-conflict, and browser-visible diagnostic redaction. | Gate 8 no-leak proof and Rulepath hidden-information firewall. | no | Diagnostics must not echo unauthorized hidden IDs. |

## Final source/IP checklist

- Sources are summarized in original language.
- Source quality and date consulted are recorded.
- Variant choices and deviations are explicit.
- Ambiguities connect to rule IDs and tests.
- Public naming avoids affiliation and trade-dress risk.
- Assets are original or absent in this ticket.
- Fonts are system-only or absent in this ticket.
- Public/private content boundary is explicit.
- Human/legal review triggers are not hidden.
- Release blockers are recorded.
