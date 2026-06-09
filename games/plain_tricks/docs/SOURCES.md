# Plain Tricks Sources

Game ID: `plain_tricks`

Public display name: `Plain Tricks`

Implemented variant: `plain_tricks_standard`

Prepared by: `Codex`

Created: 2026-06-09

Last updated: 2026-06-09

Rules version connected to this source note: `plain-tricks-rules-v1`

## Source-use statement

Plain Tricks is an original Rulepath microgame in the public-domain
trick-taking family. Classic trick-game references were consulted only for the
longstanding mechanic family: leading, following suit, trick capture, trick
winner leading next, and scoring by tricks won.

No source rules prose, examples, card imagery, product naming, component text,
icons, screenshots, scans, fonts, assets, art direction, or trade dress is
copied. Rulepath rule prose, UI copy, visual presentation, assets, icons, card
ids, suit labels, and component text for `plain_tricks` are original.

Public presentation must use **Plain Tricks** and neutral component labels such
as Gale, River, Ember, rank numerals, lead, follow, trick, round, and score. It
must not claim to be Whist, Hearts, or any commercial card product.

## Consulted sources

All sources in this table are rationale, project-authority, taxonomy, or
mechanic-context sources only. No source prose or assets are copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 10.1 Plain Tricks trick-taking spec | `../../../specs/gate-10-1-plain-tricks-trick-taking-proof.md` | 2026-06-09 | project authority | product scope, original variant, lead/follow legality, deal rotation, no-leak, docs, tests, replay, bot, WASM, UI, benchmark, and capstone obligations | none | Governs this game's implementation tickets and acceptance evidence. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-09 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, and official-game evidence. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-09 | project authority | naming and IP safety | none | Supports neutral public naming and forbids copied prose, assets, and trade dress. |
| Rulepath repository source bibliography | `../../../docs/SOURCES.md` | 2026-06-09 | project authority | confirms the project already tracks the Pagat trick-taking overview as research context | none | The central bibliography says not to copy text from the trick-taking overview. |
| Pagat trick-taking overview | `https://www.pagat.com/class/trick.html` | 2026-06-09 | public game-family reference | mechanic-family context for lead, follow suit, trick capture, and trick scoring | none | Consulted for design altitude only; Plain Tricks prose, deck, labels, examples, and presentation are original. |

## Adopted design facts

The implemented `plain_tricks_standard` variant adopts these facts in original
Rulepath prose:

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Two seats | Exactly two seats play the match. | Two seats are enough to prove follow-suit legality, private hand projection, and trick-winner-led turn order. |
| Eighteen-card deck | Three suits by six ranks, one copy each, shuffled deterministically per round. | The deck is small enough for traces and no-leak tests while still supporting meaningful suit following. |
| Six-card hands | Each seat receives six private cards per round. | Six tricks per round prove repeated hidden-hand legality checks. |
| Six-card tail | Six cards remain undealt each round and are never revealed. | The tail proves persistent hidden residue and redacted replay/export behavior. |
| Must-follow-suit legality | A follower who holds the led suit must play it; a void follower may play any card. | This is the gate's core state-dependent legality proof. |
| Trick resolution | Highest rank in the led suit wins; off-suit cards never win. | This keeps resolution deterministic and easy to cover. |
| Deal rotation | `seat_0` leads round 1; `seat_1` leads round 2 after a fresh shuffle from the continuing RNG stream. | Rotation proves multi-round setup without adding variants. |
| One point per trick | Each won trick is one point; most total points over two rounds wins. | Trick-count scoring is clear and bounded. |
| Split tie | A 6-6 final total is a `Split`. | No priority-seat tiebreaker is hidden in the implementation. |
| Neutral public presentation | The public game name is Plain Tricks and public terms use original neutral suit labels. | The game avoids commercial card-game branding and copied trade dress. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `plain_tricks_standard` / Plain Tricks | Gate 10.1 trick/follow-suit proof scope. | yes |
| player count | Exactly two seats. | Small official-game proof with clear private-view boundaries. | yes |
| deck | Eighteen cards: `gale`, `river`, `ember` by ranks 1-6. | Compact proof shape from the Gate 10.1 spec. | yes |
| tail | Six undealt cards, never revealed. | Hidden-residue no-leak requirement. | yes |
| trick rule | Follower must follow suit when able; highest led-suit rank wins. | Classic public-domain trick-taking shape expressed in original Rulepath prose. | yes |
| optional rule included | Deal rotation, two-round match, split on 6-6, Level 0 and Level 2 bot support. | Gate 10.1 acceptance requirements. | yes |
| optional rule excluded | Trump, bidding, partnerships, point-card bonuses, penalties, passing, exposed-card dummy play, 3+ seats, configurable card families, public MCTS/ISMCTS/Monte Carlo/ML/RL bots. | Out of scope and forbidden by Gate 10.1. | yes |
| Rulepath deviation from common variants | Tiny original deck, no trump or bidding, no named conventional game identity, no copied card faces or suit symbols. | IP conservatism and public product posture. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `PT-AMB-001` | Whether owner seat can see its hand before play. | Gate 10.1 spec and hidden-info no-leak posture. | The owning seat may see its own hand; observer and opponent cannot. | `PT-VIS-002` | visibility/no-leak tests, seat-private-view trace, browser smoke | resolved |
| `PT-AMB-002` | Whether tail cards reveal after terminal. | Gate 10.1 no-leak exit criteria. | Tail cards are never revealed, including at terminal and in exports. | `PT-COMP-006`, `PT-VIS-004`, `PT-END-003` | terminal no-leak trace, public export tests | resolved |
| `PT-AMB-003` | Whether an off-suit play explicitly records opponent void state. | Gate 10.1 visibility model. | Void is public only by implication from the off-suit play; no explicit opponent-void flags are added. | `PT-ACT-003`, `PT-VIS-005` | void-free-discard trace, no-leak view tests | resolved |
| `PT-AMB-004` | Whether final 6-6 total uses a priority tiebreaker. | Gate 10.1 terminal requirement. | Equal final totals produce `Split`. | `PT-END-002` | tie-split trace | resolved |
| `PT-AMB-005` | Whether data files can encode follow-suit, trick winner, or scoring formulas. | Rulepath static-data boundary and Gate 10.1 forbidden changes. | Static data carries only typed content, labels, metadata, fixtures, traces, and reports; behavior lives in Rust. | `PT-ACT-001` through `PT-ACT-006`, `PT-TRICK-001` through `PT-TRICK-004`, `PT-SCORE-001` through `PT-SCORE-003` | strict-parse tests and boundary review | resolved |

## Public naming rationale

Public ID: `plain_tricks`

Display name: `Plain Tricks`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | yes for this project | The name is a neutral Rulepath description of a tiny original trick-taking proof, not a conventional product name. |
| neutral name chosen? | yes | `Plain Tricks` avoids Whist, Hearts, Spades, Bridge, and commercial names. |
| trademark risk considered? | yes | Public docs and UI avoid product branding, source names, logos, slogans, and affiliation wording. |
| trade-dress risk considered? | yes | Public presentation must avoid copied card faces, conventional suit icon dependency, product layouts, and commercial table presentation. |
| affiliation implication avoided? | yes | Sources are cited only as project authority or mechanic-family context. |
| public help text needs disclaimer? | no | Neutral naming and source notes are sufficient unless later review requests additional text. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Trick-taking family similarity | low for abstract public-domain mechanics, expression still reviewed | Use original prose, original tiny deck, neutral suits, and no conventional game branding. | no |
| Conventional card imagery | medium if copied suit symbols/card faces appear | Use original visual treatment and neutral labels; do not copy public or commercial card art. | yes if copied or trade-dress-like |
| Browser action labels | low | Use generic lead/follow/play language supplied by Rust-owned legal tree. | no |
| Source phrasing | medium if paraphrased too closely | Use consulted-not-copied notes and original Rulepath phrasing. | yes if copied prose appears |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---:|---|---|---:|
| Game docs | `games/plain_tricks/docs/RULES.md`, `games/plain_tricks/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied source rules or tables. | yes |
| Card ids and labels | `gale_1` through `ember_6`; Gale, River, Ember with rank numerals | original local identifiers using generic neutral words | Rulepath/Codex-authored identifiers and labels | No commercial source is used as a label model. | yes |
| Public game name | `Plain Tricks` | original project name | Rulepath/Codex-authored public name | Public name avoids conventional specific-game branding. | yes |
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
| Rulepath original rules summary | yes | `games/plain_tricks/docs/RULES.md` | Original Rulepath prose. |
| Project-authority Gate 10.1 facts | yes | `games/plain_tricks/docs/SOURCES.md` | Summarized as rationale only. |
| Generic trick-taking family vocabulary | yes | `games/plain_tricks/docs/SOURCES.md` | Used as context, not copied expression. |
| Public source prose, examples, or tables | no | none | No source prose is copied. |
| Commercial/licensed rules text | no | none | No source material is redistributed. |
| Commercial card faces, suit art, product names, icons, screenshots, fonts, or table presentation | no | none | No assets introduced in this ticket. |
| Private licensed stress-test content | no public shipment | none | No private licensed content is involved. |

Private licensed stress tests are late, isolated, optional, non-public, and
non-architectural. They must not contaminate `engine-core`, public assets,
public docs, or public web bundles.

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Public name is neutral and original. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | Local card labels only. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no | No art or UI asset in this ticket. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no | None. |
| uncertainty about public-domain status | no | Abstract trick-taking mechanics are public-domain family context; no source material is redistributed. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release blocking concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| none identified for this ticket | no | Continue evidence workflow in later tickets. | Rulepath |

## Rule-source-to-rule-ID cross-reference

Every release-relevant stable rule ID in `RULES.md` must have source or design
rationale support here. Plain Tricks is original, so the primary support is the
Gate 10.1 spec plus the design rationale summarized below.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `PT-COMP-001` through `PT-COMP-010` | Game-local seats, cards, suits, ranks, private hands, tail, current trick, trick history, round score, and match total. | Gate 10.1 spec and original trick-taking proof design. | no | Vocabulary remains game-local. |
| `PT-SETUP-001` through `PT-SETUP-006` | Deterministic two-seat setup, stable deck construction, seeded shuffle, private hands, tail, round-1 lead, round-2 deal rotation, and setup effects. | Gate 10.1 spec and replay-stability requirement. | no | Hidden setup facts remain internal or owner-private. |
| `PT-TURN-001` through `PT-TURN-007` | Lead/follow sequence, trick resolution, winner-led next trick, round close, deal rotation, and terminal action absence. | Gate 10.1 sequence requirements. | no | Round 2 starts with `seat_1`. |
| `PT-ACT-001` through `PT-ACT-006` | Rust-owned lead, follow-suit, void free-discard, terminal empty tree, safe action metadata, and non-actor empty tree. | Gate 10.1 behavior-authority and no-leak requirements. | no | TypeScript computes no legality. |
| `PT-RESTRICT-001` through `PT-RESTRICT-007` | Wrong actor, wrong seat, malformed/unavailable path, stale command, not-in-hand, must-follow, and terminal restrictions. | Rulepath diagnostic/replay invariants plus Gate 10.1 spec. | no | Reject without mutation. |
| `PT-TRICK-001` through `PT-TRICK-004` | Led suit, same-suit higher-rank winner, off-suit-never-wins, and trick-winner-leads. | Gate 10.1 trick-resolution design. | no | Single-copy deck avoids same-suit ties. |
| `PT-SCORE-001` through `PT-SCORE-003` | One point per trick, six-point round close, and round-2 fresh deal rotation. | Gate 10.1 scoring design. | no | Total match points always sum to 12. |
| `PT-END-001` through `PT-END-003` | Total-points win, 6-6 split, and terminal tail hiding. | Gate 10.1 terminal requirements. | yes | No priority tiebreaker. |
| `PT-VIS-001` through `PT-VIS-007` | Public facts, owner-only hands, played cards public, tail hiding, implicit voids, actor-only legal choices, and bot rationale limits. | Gate 10.1 hidden-info no-leak exit criteria. | yes | Browser-facing surfaces are protected. |
| `PT-RNG-001` through `PT-RNG-003` | Seeded setup, viewer-scoped replay export, and stable serialization. | Gate 10.1 deterministic replay/export requirements. | no | Public export must not reconstruct hidden cards. |
| `PT-BOT-001` through `PT-BOT-002` | Random-legal and authored-policy bot boundaries. | Gate 10.1 bot requirements and Rulepath public-bot law. | no | Solver and learning bots are forbidden. |
| `PT-VAR-001` through `PT-VAR-002`, `PT-OOS-001` through `PT-OOS-004` | Public posture deviations and out-of-scope variants. | Gate 10.1 scope, IP policy, and foundations. | yes | Prevents scope creep into a general card/trick framework. |
| `PT-AMB-001` through `PT-AMB-005` | Chosen resolutions for owner private view, tail reveal, void flags, split handling, and data behavior. | Gate 10.1 spec and Rulepath foundation constraints. | yes | Tests/traces must preserve these decisions. |

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
