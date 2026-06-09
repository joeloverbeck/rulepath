# Crest Ledger Sources

Game ID: `poker_lite`

Public display name: `Crest Ledger`

Implemented variant: `poker_lite_standard`

Prepared by: `Codex`

Created: 2026-06-09

Last updated: 2026-06-09

Rules version connected to this source note: `poker-lite-rules-v1`

## Source-use statement

Crest Ledger is an original Rulepath microgame. It uses public
research-minimal imperfect-information game structures only as context for the
small proof shape: two seats, private information, bounded public choices,
public accounting, and deterministic resolution.

Kuhn poker and Leduc-style benchmark games were consulted only as public
research-minimal structures for small imperfect-information betting games.
OpenSpiel informed vocabulary around information states, observations, and
imperfect-information modeling. No public rules prose, hand-ranking table,
casino imagery, product naming, component text, examples, icons, screenshots,
scans, fonts, assets, art direction, or trade dress is copied.

Rulepath rule prose, UI copy, visual presentation, assets, icons, and component
text for `poker_lite` are original. Public presentation must use **Crest
Ledger** and neutral terms such as crest, marker, pledge, shared pool, hold,
press, lift, match, and yield.

## Consulted sources

All sources in this table are rationale, project-authority, taxonomy, or
modeling-context sources only. No source prose or assets are copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 10 poker_lite betting / showdown spec | `../../../archive/specs/gate-10-poker-lite-betting-showdown.md` | 2026-06-09 | project authority | product scope, rule identity, neutral naming, hidden-card setup, pledge rounds, shared-pool accounting, no-leak, docs, tests, replay, bot, WASM, UI, benchmark, and capstone obligations | none | Governs this game's implementation tickets and acceptance evidence. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-09 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, and official-game evidence. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-09 | project authority | naming and IP safety | none | Supports neutral public naming and forbids copied prose, assets, and trade dress. |
| H. W. Kuhn, "A Simplified Two-Person Poker" | `https://sites.math.rutgers.edu/~zeilberg/akherim/PokerPapers/Kuhn1951.pdf` | 2026-06-09 | research paper | minimal two-player/private-card/opening-contribution/bounded-choice context | none | Consulted for design altitude only; no rule prose, examples, terminology, or presentation is copied. |
| Kroer and Sandholm, "An Algorithm for Constructing and Solving Imperfect Recall Abstractions of Large Extensive-Form Games," IJCAI-17, section 5 | `https://www.ijcai.org/proceedings/2017/0130.pdf` | 2026-06-09 | research paper | compact Leduc-style pattern: three ranks, two copies, one private card, one public card, two bounded rounds, limited raises, pair-before-high-card comparison | none | Consulted for proof shape only; Crest Ledger's prose, naming, action vocabulary, and presentation are original. |
| OpenSpiel, "Available games" | `https://openspiel.readthedocs.io/en/latest/games.html` | 2026-06-09 | open-source project documentation | classification context for simplified imperfect-information benchmark games | none | Context only; no game definitions or code are copied. |
| Lanctot et al., "OpenSpiel: A Framework for Reinforcement Learning in Games" | `https://arxiv.org/pdf/1908.09453` | 2026-06-09 | research paper | information-state / observation framing and imperfect-information vocabulary | none | Context only; Rulepath does not use ML/RL bots for this game. |
| OpenSpiel documentation PDF | `https://openspiel.readthedocs.io/_/downloads/en/latest/pdf/` | 2026-06-09 | open-source project documentation | observation/information-state model and game-modeling vocabulary | none | Context only; no code, game definitions, algorithms, or public presentation are copied. |

## Adopted design facts

The implemented `poker_lite_standard` variant adopts these facts in original
Rulepath prose:

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Two seats | Exactly two seats play the match. | Two seats are enough to prove hidden private information, public pledge pressure, and no-leak viewer filtering. |
| Six-crest deck | Three ranks, two copies per rank, shuffled deterministically at setup. | The deck is small enough for traces and no-leak tests while still supporting pair and high-rank comparisons. |
| One private crest per seat | Each seat receives one private crest at setup. | This is the private-view proof surface. |
| One hidden center crest | The center crest starts hidden and becomes public only after round 1 closes without yield. | This proves staged public reveal timing. |
| Opening contribution | Each seat contributes one marker before actions begin. | The shared pool starts from public, equal accounting. |
| Two pledge rounds | Round units are 1 then 2 markers, with at most one lift per round. | Bounded action pressure avoids unbounded betting scope and keeps replay small. |
| Yield terminal | A seat facing an outstanding pledge may yield; the other seat wins the current shared pool. | Terminal allocation can occur without revealing private crests. |
| Showdown comparator | If round 2 closes without yield, a pair with the center beats no pair, then higher private rank decides, with equal strength splitting. | This is deterministic, compact, and Rust-owned. |
| Neutral public presentation | The public game name is Crest Ledger and public terms use crest/marker/pledge/shared-pool language. | The game avoids casino trade dress and public poker-product framing. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `poker_lite_standard` / Crest Ledger | Gate 10 hidden-info accounting and showdown proof scope. | yes |
| player count | Exactly two seats. | Small official-game proof with clear private-view boundaries. | yes |
| deck | Six crests: `low`, `middle`, `high`, two copies each. | Compact proof shape from the Gate 10 spec. | yes |
| center reveal | Center crest reveals only after round 1 closes without yield. | Staged reveal and no-leak requirement. | yes |
| pledge structure | Two rounds, units 1 and 2, one lift per round. | Bounded public accounting proof. | yes |
| optional rule included | Yield terminal, grouped center reveal, grouped showdown reveal, exact split on equal strength, Level 0 and Level 2 bot support. | Gate 10 acceptance requirements. | yes |
| optional rule excluded | More seats, blinds, stacks, side pools, real-money framing, copied poker terminology in UI, configurable card families, public MCTS/ISMCTS/Monte Carlo/ML/RL bots. | Out of scope and forbidden by Gate 10. | yes |
| Rulepath deviation from common variants | Abstract markers, neutral vocabulary, tiny original component set, no casino table/felt/chip/payout presentation, no public poker-engine claim. | IP conservatism and public product posture. | yes |
| out-of-scope variant | Commercial poker variants, general poker engine, trick-taking half of Gate 10, Blackjack comparison case. | Gate 10 `poker_lite` closes only the betting/showdown proof; trick-taking follows separately. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `CL-AMB-001` | Whether owner seat can see its private crest before showdown. | Gate 10 spec and hidden-info no-leak posture. | The owning seat may see its own private crest; observer and opponent cannot. | `CL-VIS-002` | visibility/no-leak tests, seat-private-view trace, browser smoke | resolved |
| `CL-AMB-002` | Whether the center crest reveals on round-1 yield. | Gate 10 yield terminal and reveal timing requirements. | Yield terminal reveals no new hidden center or private crests. | `CL-REVEAL-001`, `CL-END-001`, `CL-VIS-003`, `CL-VIS-007` | yield-terminal-no-showdown trace and public export tests | resolved |
| `CL-AMB-003` | Whether yielded private crests become public after terminal. | Gate 10 no-leak exit criteria. | No private crests become public because of yield. | `CL-END-001`, `CL-VIS-002`, `CL-VIS-007` | folded-hand/yield no-leak tests and e2e smoke | resolved |
| `CL-AMB-004` | Whether same-strength showdown uses priority, copy identity, or split. | Gate 10 split determinism requirement. | Equal showdown strength splits the shared pool exactly. | `CL-SCORE-005`, `CL-END-003` | tie-split trace | resolved |
| `CL-AMB-005` | Whether data files can encode pledge or showdown formulas. | Rulepath static-data boundary and Gate 10 forbidden changes. | Static data carries only typed content, labels, metadata, fixtures, traces, and reports; formulas live in Rust. | `CL-COMP-013`, `CL-SCORE-002`, `CL-SCORE-004`, `CL-SCORE-005` | strict-parse tests and boundary review | resolved |

## Public naming rationale

Public ID: `poker_lite`

Display name: `Crest Ledger`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | no for public display | The internal id remains `poker_lite` for roadmap continuity, but public UI/docs use `Crest Ledger`. |
| neutral name chosen? | yes | `Crest Ledger` is original Rulepath naming for this microgame. |
| trademark risk considered? | yes | Public docs and UI avoid product branding, source names, logos, slogans, and affiliation wording. |
| trade-dress risk considered? | yes | Public presentation must avoid casino felt/table/chip/payout art direction and copied rule presentation. |
| affiliation implication avoided? | yes | Sources are cited only as project authority, research context, or modeling context. |
| public help text needs disclaimer? | no | Neutral naming and source notes are sufficient unless later review requests additional text. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Internal `poker_lite` id | low for internal id, medium if surfaced as product name | Use **Crest Ledger** for public display, docs prose, effect copy, and UI. | no if public copy stays neutral |
| Casino-adjacent mechanics | medium | Use abstract markers, shared-pool accounting language, neutral board-game presentation, and no real-money or casino imagery. | yes if public presentation mimics casino trade dress |
| Research benchmark similarity | low for abstract structure, expression still reviewed | Consult only for compact proof shape; use original prose, labels, action names, and UI. | no |
| Browser action labels | medium if casino terms appear | Use hold, press, lift, match, and yield; avoid casino/product vocabulary in public UI. | yes if copied or casino-framed |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| Game docs | `games/poker_lite/docs/RULES.md`, `games/poker_lite/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied source rules or tables. | yes |
| Crest ids and labels | `low_dawn` through `high_dusk`; Sprout/Current/Crown with Dawn/Dusk copies | original local identifiers using generic neutral words | Rulepath/Codex-authored identifiers and labels | No commercial source is used as a label model. | yes |
| Public game name | `Crest Ledger` | original | Rulepath/Codex-authored public name | Public name avoids casino/product framing. | yes |
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
| Rulepath original rules summary | yes | `games/poker_lite/docs/RULES.md` | Original Rulepath prose. |
| Project-authority Gate 10 facts | yes | `games/poker_lite/docs/SOURCES.md` | Summarized as rationale only. |
| Generic imperfect-information modeling vocabulary | yes | `games/poker_lite/docs/SOURCES.md` | Used as context, not copied expression. |
| Public research paper prose, examples, or tables | no | none | No source prose is copied. |
| Commercial/licensed rules text | no | none | No source material is redistributed. |
| Casino trade dress, product names, component names, art, icons, screenshots, fonts, or table presentation | no | none | No assets introduced in this ticket. |
| Private licensed stress-test content | no public shipment | none | No private licensed content is involved. |

Private licensed stress tests are late, isolated, optional, non-public, and
non-architectural. They must not contaminate `engine-core`, public assets,
public docs, or public web bundles.

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Public name is neutral and original. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | Local crest labels only. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no | No art or UI asset in this ticket. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no | None. |
| uncertainty about public-domain status | no | Sources are research/context references; no source material is redistributed. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release blocking concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| none identified for this ticket | no | Continue evidence workflow in later tickets. | Rulepath |

## Rule-source-to-rule-ID cross-reference

Every release-relevant stable rule ID in `RULES.md` must have source or design
rationale support here. Crest Ledger is original, so the primary support is the
Gate 10 spec plus the design rationale summarized below.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `CL-COMP-001` through `CL-COMP-013` | Game-local seats, crests, ranks, copies, private crests, center crest, deck tail, markers, shared pool, pledge rounds, outstanding pledge, lift cap, and terminal allocation. | Gate 10 spec and original hidden-info accounting proof design. | no | Vocabulary remains game-local. |
| `CL-SETUP-001` through `CL-SETUP-007` | Deterministic two-seat setup, stable deck construction, seeded shuffle, private/center/tail deal, opening contributions, round state, and setup effects. | Gate 10 spec and replay-stability requirement. | no | Hidden setup facts remain internal or owner-private. |
| `CL-TURN-001` through `CL-TURN-007` | Two pledge rounds, outstanding pledge handling, center reveal after round 1, showdown after round 2, yield terminal, and terminal action absence. | Gate 10 pledge and reveal sequence. | no | Round 2 lead alternates to `seat_1`. |
| `CL-ACT-001` through `CL-ACT-005` | Rust-owned hold, press, lift, match, yield legality and safe action metadata. | Gate 10 behavior-authority and no-leak requirements. | no | TypeScript computes no legality. |
| `CL-RESTRICT-001` through `CL-RESTRICT-006` | Wrong actor, wrong seat, malformed/unavailable action, stale command, lift cap, and terminal restrictions. | Rulepath diagnostic/replay invariants plus Gate 10 spec. | no | Reject without mutation. |
| `CL-PLEDGE-001` through `CL-PLEDGE-005` | Hold, press, lift, match, and yield accounting/resolution. | Gate 10 bounded pledge design. | no | Shared-pool accounting is exact. |
| `CL-REVEAL-001` through `CL-REVEAL-002` | Center grouped reveal and showdown grouped reveal. | Gate 10 staged reveal and no-leak proof. | yes | Yield suppresses new reveals. |
| `CL-SCORE-001` through `CL-SCORE-006` | Opening pool, action accounting, contribution bound, showdown comparator, exact split, and yield allocation. | Gate 10 accounting and showdown design. | yes | Equal strength splits. |
| `CL-END-001` through `CL-END-003` | Yield win, showdown win, and split terminal outcomes. | Gate 10 terminal requirements. | yes | Yield reveal remains redacted. |
| `CL-VIS-001` through `CL-VIS-008` | Public facts, private crest visibility, center visibility, deck-tail hiding, legal choices, grouped showdown, yield terminal, and bot rationale limits. | Gate 10 hidden-info no-leak exit criteria. | yes | Browser-facing surfaces are protected. |
| `CL-RNG-001` through `CL-RNG-003` | Seeded setup, viewer-scoped replay export, and stable serialization. | Gate 10 deterministic replay/export requirements. | no | Public export must not reconstruct hidden cards. |
| `CL-AMB-001` through `CL-AMB-005` | Chosen resolutions for owner private view, yield reveal, yielded private crests, split handling, and data behavior. | Gate 10 spec and Rulepath foundation constraints. | yes | Tests/traces must preserve these decisions. |
| `CL-VAR-001` through `CL-VAR-003`, `CL-OOS-001` through `CL-OOS-004` | Public posture deviations and out-of-scope variants. | Gate 10 scope, IP policy, and foundations. | yes | Prevents scope creep into casino/general-engine territory. |

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
