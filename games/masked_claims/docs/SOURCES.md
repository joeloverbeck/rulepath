# Masked Claims Sources

Game ID: `masked_claims`

Public display name: `Masked Claims`

Implemented variant: `masked_claims_standard`

Prepared by: `Codex`

Created: 2026-06-11

Last updated: 2026-06-11

Rules version connected to this source note: `masked-claims-rules-v1`

## Source-use statement

Masked Claims is an original Rulepath microgame. External sources were
consulted only for proof vocabulary, mechanic-family context, state-machine
patterns, anti-degeneracy analysis, leak anti-patterns, and the
rules-versus-expression boundary.

No source rules prose, examples, role names, component text, card text, icons,
screenshots, scans, fonts, assets, art direction, UI layout, product naming, or
trade dress is copied. Rulepath rule prose, UI copy, visual presentation,
assets, icons, mask ids, grade labels, and component text for `masked_claims`
are original.

Public presentation must use **Masked Claims** and neutral original component
labels such as Plain, Trimmed, Gilded, Jeweled, Master, claim, accept,
challenge, veiled gallery, and exposed row. It must not claim to be, imitate, or
borrow presentation from any commercial bluffing, hidden-role, or dice game.

## Consulted sources

All sources in this table are rationale, project-authority, taxonomy,
mechanic-context, modeling, or IP-boundary sources only. No source prose or
assets are copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 11 Masked Claims bluff/reaction-window spec | `../../../specs/gate-11-masked-claims-bluff-reaction-proof.md` | 2026-06-11 | project authority | product scope, original variant, claim/reaction/resolution rules, hidden-information posture, docs, tests, replay, bot, WASM, UI, benchmark, and capstone obligations | none | Governs this game's implementation tickets and acceptance evidence. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-11 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, and official-game evidence. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-11 | project authority | naming and IP safety | none | Requires original bluffing-game name and forbids copied prose, assets, roles, trade dress, and source-confusing presentation. |
| Shafi, Truong, and Lee-Heidenreich, "Learning to Play Coup" | `https://web.stanford.edu/class/aa228/reports/2018/final81.pdf` | 2026-06-11 | non-peer-reviewed academic course report | deterministic claim/challenge phase-machine modeling and never-bluff degeneration as a design warning | none | Consulted for modeling pattern only. Masked Claims copies no role names, abilities, card text, theme, examples, or prose. |
| boardgame.io documentation, "Stages" | `https://github.com/boardgameio/boardgame.io/blob/main/docs/documentation/stages.md` | 2026-06-11 | engine documentation | within-turn response-window pattern where a stage move set replaces the global action space | none | Consulted as architecture precedent, not copied as API or implementation. |
| boardgame.io documentation, "Secret State" | `https://github.com/boardgameio/boardgame.io/blob/main/docs/documentation/secret-state.md` | 2026-06-11 | engine documentation | pure per-viewer filtering pattern for secret state | none | Consulted as visibility precedent. Rulepath uses its own Rust/WASM contracts. |
| boardgame.io issue #399 / PR #400 | `https://github.com/boardgameio/boardgame.io/issues/399` | 2026-06-11 | public issue/PR postmortem | auxiliary-surface leak warning: undo or initial-state history can leak secrets even when the primary view is filtered | none | Used as a no-leak anti-pattern for browser/dev/replay surfaces. |
| Neller and Hnath, "Approximating Optimal Dudo Play with Fixed-Strategy Iteration Counterfactual Regret Minimization" | `http://cs.gettysburg.edu/~tneller/papers/acg2011.pdf` | 2026-06-11 | academic paper | ordered enumerable claim ladders, graded challenge penalties, and bounded-memory observations | none | Consulted for abstract claim-ladder design context only. No dice game names, tables, examples, or prose are copied. |
| Reiley, Urbancic, and Walker, "Stripped-Down Poker: A Classroom Game with Signaling and Bluffing" | `http://www.davidreiley.com/papers/Poker.pdf` | 2026-06-11 | academic/economics teaching paper | no-pure-equilibrium requirement and simple bluff/call equilibrium intuition for bot-parameter starting points | none | Consulted for anti-degeneracy rationale only. Masked Claims is not poker and uses no poker presentation. |
| Ahle, "snyd" | `https://github.com/thomasahle/snyd` | 2026-06-11 | open-source solver repository | first-mover-imbalance evidence in strictly increasing claim ladders and balance sensitivity | none | Consulted for design caution only. No solver code or data is used. |
| U.S. Copyright Office, "Games" registration circular | `https://www.copyright.gov/register/tx-games.html` | 2026-06-11 | government guidance | rules-versus-expression boundary | none | Supports the project distinction between abstract mechanics and protected expression. |
| DaVinci Editrice S.r.l. v. ZiKo Games, S.D. Tex. 2016 | case reference cited by the Gate 11 spec | 2026-06-11 | legal/IP context | rules-versus-expression boundary for games | none | Consulted only as IP context. This note is not legal advice. |

## Adopted design facts

The implemented `masked_claims_standard` variant adopts these facts in original
Rulepath prose:

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Two seats | Exactly two seats play the match. | Two seats are enough to prove claimant/responder reaction windows and private-view boundaries. |
| Fifteen masks | Three copies of each grade 1 through 5 are shuffled deterministically. | Small enough for traces while supporting public counting and hidden residue. |
| Five-mask hands | Each seat receives five private masks at setup. | Each seat makes four claims and retains one hidden unplayed mask. |
| Five-mask reserve | Five masks remain undealt and never revealed. | Proves persistent hidden residue and redacted replay/export behavior. |
| Eight turns | Claimants alternate for exactly eight total claims. | Each seat has equal claim opportunities and the terminal condition is fixed. |
| Reaction window | A claim opens a responder-only accept/challenge window. | Proves pending response legality without a generic engine abstraction. |
| Accept resolution | Accepted claims score the declared grade and keep the mask hidden forever. | Creates a lifetime no-reveal surface for no-leak proof. |
| Challenge resolution | Challenged claims reveal exactly one mask and score honest or exposed outcomes. | Proves conditional reveal and conditional scoring. |
| Public tiebreak ladder | Score, exposed lies, successful challenges, challenge discipline, then draw. | Keeps terminal rationale public and deterministic. |
| Neutral public presentation | The public game name is Masked Claims and grade labels are original neutral terms. | Avoids commercial hidden-role, bluffing, dice, and poker branding. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `masked_claims_standard` / Masked Claims | Gate 11 bluff/reaction-window proof scope. | yes |
| player count | Exactly two seats. | Small official-game proof with clear claimant/responder roles. | yes |
| mask set | Fifteen masks: three copies of grades 1 through 5. | Compact proof shape from the Gate 11 spec. | yes |
| hidden reserve | Five undealt masks, never revealed. | Hidden-residue no-leak requirement. | yes |
| reaction rule | The responder may accept or challenge after every claim. | Gate 11 reaction-window exit criteria. | yes |
| optional rule included | Conditional reveal, graded scoring, terminal tiebreaks, Level 0 and Level 1 bot support. | Gate 11 acceptance requirements. | yes |
| optional rule excluded | Three- or four-seat play, roles, powers, nested windows, blocks, counter-claims, reaction timeouts, hosted multiplayer, public MCTS/ISMCTS/Monte Carlo/ML/RL bots. | Out of scope and forbidden by Gate 11. | yes |
| Rulepath deviation from common bluffing games | Tiny original component set, no role roster, no player elimination, no dice, no casino/poker presentation, no copied component names. | IP conservatism and public product posture. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `MC-AMB-001` | Whether accepted masks reveal at terminal. | Gate 11 spec and hidden-info no-leak posture. | Accepted masks never reveal, including at terminal and in exports. | `MC-SCORE-001`, `MC-VIS-005`, `MC-END-006` | terminal no-leak trace, public export tests, browser no-leak smoke | resolved |
| `MC-AMB-002` | Whether unplayed hand masks and reserve reveal after the final turn. | Gate 11 no-leak posture. | Unplayed hand masks and reserve never reveal. | `MC-COMP-004`, `MC-COMP-005`, `MC-VIS-002`, `MC-VIS-006`, `MC-END-006` | terminal no-leak trace, public export tests | resolved |
| `MC-AMB-003` | Whether underclaims are honest. | Gate 11 scoring constants and anti-degeneracy rationale. | Actual grade greater than or equal to declared grade is honest. | `MC-SCORE-002` | honest-underclaim rule test and trace | resolved |
| `MC-AMB-004` | Whether the claimant may act during a reaction window. | Gate 11 reaction-window proof scope and boardgame.io stage pattern context. | Claimant receives no gameplay actions and safe waiting metadata while responder acts. | `MC-ACT-004`, `MC-TURN-002` | reaction-window legality tests and browser smoke | resolved |
| `MC-AMB-005` | Whether data files can encode claim legality, scoring formulas, or bot policy. | Rulepath static-data boundary and Gate 11 forbidden changes. | Static data carries only typed content, labels, metadata, fixtures, traces, and reports; behavior lives in Rust. | `MC-OOS-003`, `MC-RNG-003` | strict-parse tests and boundary review | resolved |

## Public naming rationale

Public ID: `masked_claims`

Display name: `Masked Claims`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | unclear for the category; original name chosen | Bluffing and hidden-role games have commercial presentation risk, so public naming avoids known product names. |
| neutral name chosen? | yes | `Masked Claims` describes the original Rulepath component and action pattern without invoking a specific source game. |
| trademark risk considered? | yes | Public docs and UI avoid Coup, Mascarade, Skull, Sheriff of Nottingham, Cockroach Poker, Perudo, Dudo, Liar's Dice, poker branding, logos, slogans, and affiliation wording. |
| trade-dress risk considered? | yes | Public presentation must avoid copied role cards, dice cups, skull tokens, market stalls, poker tables, component layouts, and commercial visual language. |
| affiliation implication avoided? | yes | Sources are cited only as project authority, mechanic-family context, modeling, or IP-boundary context. |
| public help text needs disclaimer? | no | Neutral naming and source notes are sufficient unless later review requests additional text. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Bluffing-game mechanic similarity | low for abstract mechanics, expression still reviewed | Use original prose, original mask-grade component set, no roles, no dice, no casino/poker presentation, and neutral visuals. | no |
| Hidden-role or bluffing product names | medium if copied or referenced in public UI | Public game name and UI labels are original and do not name source products except in this source note. | yes if product names enter player-facing UI as branding |
| Role/card/component text | high if copied from a commercial game | Masked Claims has no roles or ability text; all component labels are original. | yes if copied text appears |
| Visual trade dress | medium if copied component layouts, icons, card faces, dice cups, table themes, or screenshots appear | Use original board treatment and no source screenshots/scans/icons. | yes if copied or source-confusing |
| Source phrasing | medium if paraphrased too closely | Use consulted-not-copied notes and original Rulepath phrasing. | yes if copied prose appears |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---:|---|---|---:|
| Game docs | `games/masked_claims/docs/RULES.md`, `games/masked_claims/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied source rules, examples, roles, or tables. | yes |
| Mask ids and labels | `mask_g1_a` through `mask_g5_c`; Plain, Trimmed, Gilded, Jeweled, Master | original local identifiers using neutral descriptive words | Rulepath/Codex-authored identifiers and labels | No commercial source is used as a label model. | yes |
| Public game name | `Masked Claims` | original project name | Rulepath/Codex-authored public name | Public name avoids known product branding. | yes |
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
| Rulepath original rules summary | yes | `games/masked_claims/docs/RULES.md` | Original Rulepath prose. |
| Project-authority Gate 11 facts | yes | `games/masked_claims/docs/SOURCES.md` | Summarized as rationale only. |
| Generic bluffing/reaction-window mechanic context | yes | `games/masked_claims/docs/SOURCES.md` | Used as context, not copied expression. |
| Public source prose, examples, role names, or tables | no | none | No source prose is copied. |
| Commercial/licensed rules text | no | none | No source material is redistributed. |
| Commercial card faces, role rosters, dice art, icons, screenshots, fonts, or table presentation | no | none | No assets introduced in this ticket. |
| Private licensed stress-test content | no public shipment | none | No private licensed content is involved. |

Private licensed stress tests are late, isolated, optional, non-public, and
non-architectural. They must not contaminate `engine-core`, public assets,
public docs, or public web bundles.

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Public name is neutral and original. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | Local mask labels only; no role text. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no | No art or UI asset in this ticket. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no | None. |
| uncertainty about public-domain status | no for abstract mechanics; commercial expression remains avoided | No source material is redistributed. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release blocking concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| none identified for this ticket | no | Continue evidence workflow in later tickets. | Rulepath |

## Rule-source-to-rule-ID cross-reference

Every release-relevant stable rule ID in `RULES.md` must have source or design
rationale support here. Masked Claims is original, so the primary support is the
Gate 11 spec plus the design rationale summarized below.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `MC-COMP-001` through `MC-COMP-010` | Game-local seats, masks, grades, private hands, reserve, pedestal, reaction window, galleries, exposed row, score, and tiebreak counters. | Gate 11 spec and original reaction-window proof design. | no | Vocabulary remains game-local. |
| `MC-SETUP-001` through `MC-SETUP-005` | Deterministic two-seat setup, stable mask construction, seeded shuffle, private hands, reserve, first claimant, and empty public counters. | Gate 11 spec and replay-stability requirement. | no | Hidden setup facts remain internal or owner-private. |
| `MC-TURN-001` through `MC-TURN-007` | Claim phase, reaction window, accept resolution, challenge resolution, cleanup, terminal resolution, and terminal action absence. | Gate 11 sequence requirements. | no | Single-depth reaction window only. |
| `MC-ACT-001` through `MC-ACT-007` | Rust-owned claim choices, responder-only response choices, claimant waiting tree, terminal empty tree, safe action metadata, and non-actor empty tree. | Gate 11 behavior-authority and no-leak requirements. | no | TypeScript computes no legality. |
| `MC-RESTRICT-001` through `MC-RESTRICT-008` | Wrong actor, wrong seat, malformed/unavailable path, stale command, not-in-hand, invalid grade, wrong-phase response, and terminal restrictions. | Rulepath diagnostic/replay invariants plus Gate 11 spec. | no | Reject without mutation. |
| `MC-SCORE-001` through `MC-SCORE-006` | Accept scoring, honest challenge scoring, exposed lie scoring, challenge counters, exposed-lie counters, and cumulative totals. | Gate 11 scoring design and anti-degeneracy rationale. | yes | Underclaims are honest. |
| `MC-END-001` through `MC-END-006` | Score win, exposed-lie tiebreak, successful-challenge tiebreak, challenge-discipline tiebreak, draw, and terminal hidden-residue posture. | Gate 11 terminal requirements. | yes | No priority-seat tiebreaker. |
| `MC-VIS-001` through `MC-VIS-009` | Public facts, owner-only hands, hidden pedestal identity, challenged reveal, accepted masks hidden forever, reserve hiding, actor-only legal choices, responder legal choices, and bot rationale limits. | Gate 11 hidden-info no-leak exit criteria. | yes | Browser-facing surfaces are protected. |
| `MC-RNG-001` through `MC-RNG-003` | Seeded setup, viewer-scoped replay export, and stable serialization. | Gate 11 deterministic replay/export requirements. | no | Public export must not reconstruct hidden masks. |
| `MC-BOT-001` through `MC-BOT-003` | Random-legal and Level 1 claim/response bot boundaries. | Gate 11 bot requirements and Rulepath public-bot law. | no | Solver, sampling, and learning bots are forbidden. |
| `MC-VAR-001` through `MC-VAR-002`, `MC-OOS-001` through `MC-OOS-007`, `MC-DEV-001` through `MC-DEV-003` | Public posture, deviations, and out-of-scope variants. | Gate 11 scope, IP policy, and foundations. | yes | Prevents scope creep into a general bluff/reaction framework. |
| `MC-AMB-001` through `MC-AMB-005` | Chosen resolutions for accepted mask reveal, hidden residue, underclaims, claimant waiting state, and data behavior. | Gate 11 spec and Rulepath foundation constraints. | yes | Tests/traces must preserve these decisions. |

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
