# Veiled Draft Sources

Game ID: `secret_draft`

Public display name: `Veiled Draft`

Implemented variant: `secret_draft_standard`

Prepared by: `Codex`

Created: 2026-06-08

Last updated: 2026-06-08

Rules version connected to this source note: `secret-draft-rules-v1`

## Source-use statement

Veiled Draft is an original Rulepath ruleset. It uses generic mechanism
vocabulary for simultaneous selection, open drafting, hidden information, and
imperfect-information modeling only as context. It does not copy commercial
board or card game rules, names, prose, examples, component text, icons,
screenshots, scans, fonts, assets, art direction, or trade dress.

Sources do not authorize copied prose, tile text, card text, icons,
screenshots, scans, fonts, assets, art direction, component text, or trade
dress. Rulepath rule prose, UI copy, visual presentation, assets, icons, and
component text for `secret_draft` are original.

## Consulted sources

All sources in this table are rationale, project-authority, taxonomy, or
modeling-context sources only. No source prose or assets are copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 9.1 Secret Draft Commitment / Reveal spec | `../../../specs/gate-9-1-secret-draft-commitment-reveal.md` | 2026-06-08 | project authority | product scope, rule identity, hidden commitment, reveal, drafting, no-leak, docs, tests, replay, bot, WASM, UI, benchmark, and capstone obligations | none | Governs this game's implementation tickets and acceptance evidence. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-08 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, and official-game evidence. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-08 | project authority | naming and IP safety | none | Supports neutral public naming and forbids copied prose, assets, and trade dress. |
| BoardGameGeek simultaneous action selection mechanic | `https://boardgamegeek.com/boardgamemechanic/2020/simultaneous-action-selection` | 2026-06-08 | community taxonomy/reference | generic mechanism vocabulary for simultaneous choices | none | Used only to confirm broad terminology context, not as a rules model. |
| BoardGameGeek open drafting mechanic | `https://boardgamegeek.com/boardgamemechanic/2041/open-drafting` | 2026-06-08 | community taxonomy/reference | generic mechanism vocabulary for choosing from a visible shared set | none | Used only to confirm broad terminology context, not as a rules model. |
| Lanctot et al., OpenSpiel: A Framework for Reinforcement Learning in Games | `https://arxiv.org/abs/1908.09453` | 2026-06-08 | research paper | imperfect-information and game-modeling context | none | Context only; Rulepath does not use ML/RL bots for this game. |
| OpenSpiel documentation and repository | `https://github.com/google-deepmind/open_spiel` | 2026-06-08 | open-source project documentation | imperfect-information terminology and modeling context | none | Context only; no code, game definitions, or algorithms are copied. |
| GamesRadar board-game types overview | `https://www.gamesradar.com/board-game-types/` | 2026-06-08 | reputable secondary overview | broad public vocabulary context for game-type descriptions | none | Context only; not a rule authority. |

## Adopted design facts

The implemented `secret_draft_standard` variant adopts these facts in original
Rulepath prose:

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Two-seat hidden commitment | Exactly two seats commit secretly each round. | Two seats are enough to prove hidden pending state, reveal synchronization, and browser no-leak behavior. |
| Fixed visible pool | Twelve draft items start public in stable order. | A fixed pool keeps replay/hash behavior deterministic and makes pool removal readable. |
| Three threads and four values | Items are divided into `ember`, `tide`, and `grove`, each with values 1 through 4. | Small scoring texture without procedural static data. |
| Six rounds | Two items are removed per reveal, so six rounds exhaust the pool. | Bounded matches support simulation, traces, browser smoke, and benchmarks. |
| Hidden per-seat commitment slots | A committed item stays inside Rust internal state until both seats have committed. | This is the Gate 9.1 no-leak proof surface. |
| Pending booleans | Public views may show whether each seat has committed. | Pending status is required for browser waiting state and does not reveal the chosen item. |
| Synchronized reveal batch | Both hidden choices become public in one reveal event after both seats commit. | Later betting, trick, and bluffing games need this pattern proven before they start. |
| Deterministic conflict fallback | If both seats choose the same item, priority wins it and the other seat receives the lowest stable-order remaining item. | No RNG or browser decision is needed to resolve contested choices. |
| Public scoring | Score uses base values, complete sets, high-thread bonuses, and a terminal conflict-discipline bonus. | Scoring is visible, deterministic, and small enough for rule coverage. |
| Public tie-break ladder | Score, set count, highest item, distinct threads, fewer priority conflict wins, then draw. | Terminal outcomes require no hidden data or browser computation. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `secret_draft_standard` | Gate 9.1 focused commitment/reveal proof scope. | yes |
| player count | Exactly two seats. | Small official-game proof with clear pending-seat UI. | yes |
| first priority seat | `seat_0`. | Deterministic setup and replay simplicity. | yes |
| round cap | 6 reveal batches. | Twelve items leave the pool two at a time. | yes |
| optional rule included | Simultaneous hidden commitments, pending booleans, synchronized reveal, deterministic conflict fallback, public scoring, terminal tie-breaks. | Gate 9.1 acceptance requirements. | yes |
| optional rule excluded | More seats, random setup, private hands, retained hidden choices after reveal, cryptographic commitments, hosted multiplayer, generic drafting primitives. | Out of scope for Gate 9.1. | yes |
| Rulepath deviation from common drafting games | Fixed visible pool, two seats, no random deal, no hidden hand, no source-derived card text, deterministic fallback. | Keeps the gate focused on no-leak pending/reveal behavior. | yes |
| out-of-scope variant | Poker, trick-taking, betting, bluffing, reaction-window, or hosted simultaneous-choice variants. | Gate 10 and later gates own those shapes. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `SD-AMB-001` | Whether the committing seat may see its own committed item before reveal. | Gate 9.1 no-leak scope and ADR 0004 replay/export posture. | No browser-facing or seat-view payload contains the item id before reveal, even for the committing seat. | `SD-VIS-002`, `SD-REVEAL-001` | visibility/no-leak tests, public export tests, browser smoke | resolved |
| `SD-AMB-002` | Whether committed items leave the visible pool immediately. | Simultaneous-choice proof requirements. | Items remain visibly available until reveal, so the second seat cannot infer the first commitment. | `SD-ACT-003`, `SD-REVEAL-003`, `SD-REVEAL-004` | legal-action tests, contested-pick trace | resolved |
| `SD-AMB-003` | What fallback item a non-priority seat receives after a contested reveal. | Gate 9.1 deterministic conflict fallback requirement. | Award the lowest stable-order remaining item after removing the contested item. | `SD-REVEAL-005`, `SD-SCORE-005` | contested-pick fallback tests and traces | resolved |
| `SD-AMB-004` | Whether static data may encode scoring or tie-break formulas. | Rulepath static-data boundary and Gate 9.1 forbidden changes. | Static data carries only IDs, labels, values, constants, and fixtures; all formulas live in typed Rust. | `SD-SCORE-001` through `SD-SCORE-005`, `SD-END-002` | serialization unknown-field tests and boundary review | resolved |

## Public naming rationale

Public ID: `secret_draft`

Display name: `Veiled Draft`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | yes | The public name uses neutral words for hidden selection and visible drafting. |
| neutral name chosen? | yes | `Veiled Draft` is Rulepath naming for this original variant. |
| trademark risk considered? | yes | Public docs and UI avoid product branding, source names, logos, slogans, and affiliation wording. |
| trade-dress risk considered? | yes | Tile labels, board layout, colors, reveal effects, and help text must be original Rulepath presentation. |
| affiliation implication avoided? | yes | Sources are cited only as project authority, taxonomy, or modeling context. |
| public help text needs disclaimer? | no | Neutral naming and source notes are sufficient unless later review requests additional text. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Generic simultaneous-selection and drafting vocabulary | low for vocabulary, expression still reviewed | Use original prose, original labels, and Rulepath presentation. | no |
| Commercial drafting-game resemblance | medium if visual or rule presentation is imitated | Do not copy commercial rule structures, examples, component text, art direction, icon style, board layout, or animations. | yes if found |
| Tile labels | low | Use the original neutral labels `Ember One` through `Grove Four`; rename if any later review finds source confusion. | no |
| Browser reveal and pending UI | medium if styled after a recognizable product | Use original Rulepath UI, system fonts, accessible labels, and no source-derived imagery. | yes if copied |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| Game docs | `games/secret_draft/docs/RULES.md`, `games/secret_draft/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied rulebook text. | yes |
| Thread names | `ember`, `tide`, `grove` | original local identifiers using generic natural words | Rulepath/Codex-authored local vocabulary | Names are game-local and not source-derived. | yes |
| Draft item ids and labels | `ember_1` through `grove_4`; `Ember One` through `Grove Four` | original placeholder labels | Rulepath/Codex-authored identifiers and labels | No commercial source is used as a label model. | yes |
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
| Rulepath original rules summary | yes | `games/secret_draft/docs/RULES.md` | Original Rulepath prose. |
| Project-authority Gate 9.1 facts | yes | `games/secret_draft/docs/SOURCES.md` | Summarized as rationale only. |
| Generic simultaneous-selection and open-drafting vocabulary | yes | `games/secret_draft/docs/RULES.md`, `games/secret_draft/docs/SOURCES.md` | Used as generic mechanism vocabulary, not copied expression. |
| Commercial/licensed rules text | no | none | No source prose is copied. |
| Commercial game names, component names, art, icons, screenshots, fonts, or trade dress | no | none | No assets introduced in this ticket. |
| Private licensed stress-test content | no public shipment | none | No private licensed content is involved. |

Private licensed stress tests are late, isolated, optional, non-public, and
non-architectural. They must not contaminate `engine-core`, public assets,
public docs, or public web bundles.

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Neutral public name only. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | Local draft item labels only. |
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
rationale support here. Veiled Draft is original, so the primary support is the
Gate 9.1 spec plus the design rationale summarized below.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `SD-COMP-001` through `SD-COMP-011` | Game-local seats, draft items, threads, values, visible pool, commitments, drafted collections, priority, fallback, reveal batch, and score summary. | Gate 9.1 spec and original no-leak drafting proof design. | no | Vocabulary remains game-local. |
| `SD-SETUP-001` through `SD-SETUP-005` | Deterministic two-seat setup, round, priority, visible pool, empty commitments, empty drafted collections, scores, terminal, and freshness. | Gate 9.1 spec and replay-stability requirement. | no | No RNG. |
| `SD-TURN-001` through `SD-TURN-006` | Six-round hidden commitment, pending state, synchronized reveal, round advance, terminal, and gameplay stop. | Gate 9.1 simultaneous-choice and pending-seat proof. | no | Reveal is grouped after both commitments. |
| `SD-ACT-001` through `SD-ACT-004` | Rust-owned commitment action legality and terminal action absence. | Gate 9.1 behavior-authority boundary. | no | TypeScript computes no legality. |
| `SD-RESTRICT-001` through `SD-RESTRICT-005` | Wrong actor, already committed, unavailable item, stale command, and terminal restrictions. | Rulepath diagnostic/replay invariants plus Gate 9.1 spec. | no | Reject without mutation. |
| `SD-REVEAL-001` through `SD-REVEAL-006` | Pending-only first commit, synchronized reveal, distinct awards, contested award, fallback award, and commitment cleanup. | Gate 9.1 reveal and no-leak proof design. | yes | Contest fallback is a chosen original resolution. |
| `SD-SCORE-001` through `SD-SCORE-005` | Base value, complete sets, high-thread bonuses, conflict-discipline bonus, and exact pool removal. | Gate 9.1 spec and original scoring design. | no | Static data carries no formulas. |
| `SD-END-001` through `SD-END-002` | Six-round terminal and public tie-break ladder. | Gate 9.1 spec and bounded terminal proof. | yes | Draw only after all comparisons tie. |
| `SD-VIS-001` through `SD-VIS-005` | Public facts, hidden committed item, pending booleans, legal choices, and bot-rationale visibility. | Gate 9.1 no-leak requirement and ADR 0004 export posture. | yes | Committing seat receives no pre-reveal item id. |
| `SD-RNG-001` through `SD-RNG-003` | No randomness, viewer-scoped public replay export, and stable serialization/replay. | Gate 9.1 spec and replay/hash invariants. | no | Internal traces may retain hidden choices for native proof. |
| `SD-AMB-001` through `SD-AMB-004` | Committing-seat visibility, committed item pool status, fallback item, and static-data formula ban. | Gate 9.1 reassessment and Rulepath boundaries. | yes | Covered by future tests/traces. |
| `SD-VAR-001` through `SD-VAR-005` | Original variant boundaries and out-of-scope variants. | Gate 9.1 scope and successor sequencing. | yes | No generic primitive promotion here. |

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
