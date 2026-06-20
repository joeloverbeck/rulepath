# River Ledger Sources

Game ID: `river_ledger`

Public display name: `River Ledger`

Implemented variant: `river_ledger_standard`

Prepared by: `Codex`

Created: 2026-06-14

Last updated: 2026-06-20

Rules version connected to this source note: `river-ledger-rules-v2`

## Source-use statement

River Ledger is an original Rulepath implementation in the Texas Hold'Em rules
family. Consulted sources verify public rules-family facts: private hole cards,
shared community cards, street sequence, betting rounds, showdown comparison,
and standard poker hand ranking. They do not authorize copied prose, examples,
tables, card art, product names, screenshots, scans, fonts, icons, table
layouts, casino presentation, tournament branding, or trade dress.

Rulepath rule prose, UI copy, visual presentation, game name, component labels,
assets, tests, traces, bot explanations, and player-facing help for
`river_ledger` are original. Public presentation must use **River Ledger** and
neutral abstract contribution language.

## Consulted sources

All sources in this table are project-authority, rules-fact, policy, or
architecture-comparison sources only. No source prose or assets are copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 15 River Ledger spec | `../../../archive/specs/gate-15-river-ledger-texas-holdem-base.md` | 2026-06-14 | project authority | product scope, seat range, fixed-limit cap, no-leak matrix, docs, tests, replay, bots, tools, WASM, web, and benchmark obligations | none | Governs the Gate 15 tickets and acceptance evidence. |
| Rulepath Gate 15.1 River Ledger all-in / side pots spec | `../../../specs/gate-15-1-river-ledger-all-in-side-pots.md` | 2026-06-20 | project authority | finite stacks, all-in actions, full-unit reopening, side-pot construction/allocation, returns, no-leak, replay, bots, WASM/web, and benchmark obligations | none | Governs the v2 all-in/side-pot cutover. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-14 | project authority | official-game documentation and evidence workflow | none | Requires original rules prose, source notes, coverage, mechanics, no-leak tests, bot evidence, benchmarks, and web proof. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-14 | project authority | public naming, original prose, and trade-dress safety | none | Requires neutral/original presentation and forbids copied protected expression. |
| Rulepath Mechanic Atlas | `../../../docs/MECHANIC-ATLAS.md` | 2026-06-14 | project authority | card/deck/private-hand/evaluator/accounting primitive-pressure posture | none | River Ledger records pressure but no `game-stdlib` promotion is authorized by Gate 15. |
| Pagat, Texas Hold'em | `https://www.pagat.com/poker/variants/texasholdem.html` | 2026-06-14 | reputable community rules reference | public rules-family facts: hole cards, community board, betting streets, showdown, and player-count context | none | Used only for rules-family verification; River Ledger prose and variant decisions are original. |
| Pagat, poker hand ranking | `https://www.pagat.com/poker/rules/ranking.html` | 2026-06-14 | reputable community rules reference | hand category order, rank-vector comparison concepts, ace-low straight context | none | Used to verify evaluator facts; no ranking table or examples are copied. |
| Fournier, How to play Texas Hold'em Poker | `https://www.nhfournier.es/en/como-jugar/texas-holdem-poker/` | 2026-06-14 | card-maker rules explainer | secondary check on shared board/street/showdown family shape | none | Used only as comparison context. |
| OpenSpiel concepts | `https://openspiel.readthedocs.io/en/latest/concepts.html` | 2026-06-14 | research project documentation | imperfect-information observation/player-view vocabulary and prior-art comparison | none | Rulepath does not adopt OpenSpiel architecture or public search/RL bots. |
| OpenSpiel paper | `https://arxiv.org/abs/1908.09453` | 2026-06-14 | research paper | N-player imperfect-information research context | none | Context only; no algorithms, code, or public bot methods are copied. |
| boardgame.io | `https://boardgame.io/` | 2026-06-14 | open-source framework documentation | turn/phase/log/player-view comparison context | none | Context only; Rulepath keeps Rust as behavior authority. |

## Adopted design facts

The planned `river_ledger_standard` variant adopts these facts in original
Rulepath prose:

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| 3-6 seats | Official River Ledger accepts 3, 4, 5, and 6 seats only. | Gate 15 proves public scaling beyond two seats while avoiding heads-up special cases. |
| Standard 52-card deck | The game-local deck contains the normal rank/suit combinations used for hand evaluation. | The Texas Hold'Em family requires a familiar card universe; representation remains game-local. |
| Two private hole cards | Each seat receives two private hole cards at setup. | Private-hand no-leak proof is the primary hidden-information surface. |
| Five community cards | The board reveals three flop cards, then one turn card, then one river card. | Matches the chosen rules-family shape. |
| Fixed-limit contribution rounds | Preflop/flop use a small unit; turn/river use a big unit. | Keeps action trees bounded and avoids no-limit scope. |
| Raise cap | Each street permits one opening bet plus three raises. | Explicit cap supports diagnostics, bots, replay, and benchmarks. |
| Finite stacks and all-in | Gate 15.1 adds bounded public stacks and stack-capped `Call`, `Bet`, and `Raise` actions. | Keeps fixed-limit action families while allowing all-in outcomes. |
| Ordered side pots | Gate 15.1 builds ordered contribution layers, eligibility, returns, and per-pot allocation. | Makes allocation public and deterministic without TypeScript computation. |
| Showdown evaluator | Best five of seven cards are chosen by enumerating all 21 five-card subsets. | Correctness and auditability beat optimized lookup tables in Gate 15. |
| Split remainder | Tied winners split equal integer shares first; remainders follow stable button-order among tied winners. | Deterministic replay and Rust-authored outcome rationale. |
| Neutral public presentation | Public UI/docs use River Ledger and abstract contribution language. | Avoids casino product framing and source confusion. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `river_ledger_standard` / River Ledger | Gate 15 scope and public-scaling objective. | yes |
| player count | Exactly 3-6 seats. | N-seat proof surface; heads-up is out of scope. | yes |
| contribution structure | Fixed-limit only, with an explicit street raise cap. | Bounded action trees and clearer tests. | yes |
| all-in/side pots | Implemented in v2 as fixed-limit all-in and ordered side-pot accounting. | Gate 15.1 owns and closes this pressure. | yes |
| hand ranking | Standard category order with deterministic rank-vector tie breaks and no suit tie-breaks. | Rules-family fact verified by sources; exact implementation is Rust-owned. | yes |
| evaluator implementation | Exhaustive 21-subset search. | Auditability, explanation, and replay confidence. | no |
| optional rule excluded | Tournament structure, real-money features, rake, payouts, no-limit/pot-limit play, hosted multiplayer, copied casino presentation. | Gate 15 out-of-scope and foundation law. | yes |
| Rulepath deviation from commercial presentation | Abstract contribution units, original name, original prose/assets, and no copied product framing. | IP policy and public product posture. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `RL-AMB-001` | Whether heads-up is supported. | Gate 15 spec and Hold'Em rules-family references. | Official Gate 15 supports 3-6 seats only. | `RL-SETUP-SEATS-*` | setup validation tests and traces | resolved |
| `RL-AMB-002` | Whether burn cards are modeled publicly. | Gate 15 spec and source family context. | Burn advancement may be internal only; no viewer receives burn identities. | `RL-DEAL-BOARD-*`, `RL-VIS-DECKTAIL-*` | no-leak tests and replay export checks | resolved |
| `RL-AMB-003` | Whether all-in/side pots are part of standard River Ledger. | Gate 15 vs Gate 15.1 split. | v2 includes fixed-limit all-in and side pots inside `river_ledger_standard`; no no-limit/pot-limit variant is added. | `RL-ALLIN-*`, `RL-POT-*`, `RL-VIS-POT-*` | contribution/property tests, traces, web no-leak | resolved |
| `RL-AMB-007` | What reopens raising after incomplete all-in pressure. | Gate 15.1 research reconciliation; public poker rules vary by venue/rule set. | River Ledger uses a deliberate full-unit reopening rule for this fixed-limit game. | `RL-ALLIN-REOPEN-001` | reopen tests and traces | resolved |
| `RL-AMB-004` | Whether suits break equal evaluated hands. | Poker ranking references and Gate 15 spec. | Suits never break ties. | `RL-EVAL-TIEBREAK-*` | evaluator tests | resolved |
| `RL-AMB-005` | Whether folded hands reveal when everyone else folds. | Gate 15 no-leak requirement. | Foldout terminal reveals no folded seats' private hole cards. | `RL-SHOW-FOLDOUT-*`, `RL-VIS-FOLDOUT-*` | foldout no-leak trace and tests | resolved |
| `RL-AMB-006` | Whether static data can encode betting/evaluator formulas. | Rulepath static-data boundary. | No; static data is metadata/content only. | `RL-SETUP-VARIANT-*` | strict static-data tests | resolved |

## Public naming rationale

Public ID: `river_ledger`

Display name: `River Ledger`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | use only as rules-family descriptor | Texas Hold'Em describes the family; public product identity is River Ledger. |
| neutral name chosen? | yes | `River Ledger` is original Rulepath naming. |
| trademark risk considered? | yes | Public docs and UI avoid branding, affiliation, product slogans, and copied presentation. |
| trade-dress risk considered? | yes | Visuals must avoid copied table presentation, source branding, tournament framing, copied card art, and recognizable source layout. |
| affiliation implication avoided? | yes | Sources are cited only as rules-family/context references. |
| public help text needs disclaimer? | no current blocker | Neutral naming and source notes are sufficient unless later human/legal review requests more. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Hold'Em-family terminology | medium if product-forward | Use River Ledger as product name and keep source-family labels in docs/source notes. | no if presentation stays neutral |
| Casino-adjacent mechanics | medium | Use abstract contribution units and board-game presentation; avoid product visual cues and source presentation. | yes if found in public UI/assets |
| Standard card imagery | medium if copied | Use original/project-owned or compatible card visuals; no scans, proprietary faces, or copied art. | yes if copied |
| Browser action labels | medium | Labels may use rules-family action names when needed, but copy and visuals must remain original and neutral. | yes if copied or casino-framed |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| Game docs | `games/river_ledger/docs/RULES.md`, `games/river_ledger/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied source rules, examples, or tables. | yes |
| Public game name | `River Ledger` | original | Rulepath/Codex-authored public name | Avoids source/product naming. | yes |
| Card visuals/assets | none in this ticket | not applicable | none | Later UI/assets must document provenance. | yes |
| Fonts | none in this ticket | not applicable | none | Later UI must use system or reviewed fonts. | yes |

## Generated asset review notes

| Generated asset | Tool/model if known | Prompt/source inputs safe? | Similarity/trade-dress risk | Human review result | Public-safe? |
|---|---|---:|---|---|---:|
| none | not applicable | yes | none | No generated art/assets in this ticket. | yes |

## Public/private content boundary

| Content | Public allowed? | Location | Notes |
|---|---:|---|---|
| Rulepath original rules summary | yes | `games/river_ledger/docs/RULES.md` | Original Rulepath prose. |
| Rules-family facts | yes | `games/river_ledger/docs/SOURCES.md` | Summarized as facts and variant rationale. |
| Source prose, examples, or ranking tables | no | none | No source expression is copied. |
| Casino/product presentation | no | none | No assets or UI introduced in this ticket. |
| Private licensed stress-test content | no public shipment | none | No private licensed content is involved. |

Private licensed stress tests are late, isolated, optional, non-public, and
non-architectural. They must not contaminate `engine-core`, public assets,
public docs, or public web bundles.

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Public name is original. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| copied ranking table or examples | no | None copied. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no | No art or UI asset in this ticket. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no | None. |
| casino/product trade dress in public UI | not in this ticket | Later UI tickets must review. |
| private licensed content touched public path | no | None. |

## Release blocking concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| UI/assets not yet reviewed | no for this ticket | Later UI/public-release tickets must record asset and trade-dress review. | Rulepath |

## Rule-source-to-rule-ID cross-reference

Every release-relevant stable rule ID in `RULES.md` must have source or design
rationale support here. River Ledger uses external sources for rules-family
facts and Rulepath documents for scoped variant decisions.

| Rule family | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `RL-SETUP-*` | Seat range, variant, button/blind setup, initial street. | Gate 15 spec; Hold'Em family references for table/role context. | yes | 3-6 official seats and no heads-up path are Rulepath decisions. |
| `RL-DEAL-*` | 52-card deck, deterministic shuffle, two hole cards, five community cards, internal burn/deck tail. | Pagat/Fournier rules-family facts; Rulepath deterministic replay law. | yes | Burn identities never project. |
| `RL-BET-*` | Fixed-limit actions, blinds, call/check/bet/raise/fold, raise cap, contribution ledger. | Gate 15 spec; Hold'Em family references for betting-round shape. | yes | Fixed limit and cap are Rulepath-scoped. |
| `RL-STREET-*` | Preflop, flop, turn, river, showdown, foldout advancement. | Rules-family references plus Gate 15 no-leak terminal choice. | yes | Foldout keeps folded hands redacted. |
| `RL-EVAL-*` | Five-card category ranking and seven-card best-hand selection. | Pagat ranking reference and Gate 15 evaluator design. | no for category order; yes for implementation method | Exhaustive 21-subset search is Rulepath's chosen implementation. |
| `RL-SHOW-*` | Showdown eligibility, winner comparison, split, foldout explanation. | Gate 15 spec and outcome-explanation contract. | yes | Rust authors decisive rationale. |
| `RL-STACK-*` | Ordered stack setup and capped blind posts. | Gate 15.1 spec and Rulepath deterministic accounting law. | yes | Public stacks are abstract units only. |
| `RL-ALLIN-*` | Stack-capped call/bet/raise, actor exclusion, and full-unit reopening. | Gate 15.1 spec. | yes | The reopening rule is a River Ledger rule, not a universal poker authority claim. |
| `RL-POT-*` | Ordered side-pot layers, folded contribution retention, returns, per-pot winners, split/remainder allocation. | Gate 15.1 spec and prior Gate 15 split/remainder design. | yes | Implemented game-locally. |
| `RL-VIS-*` | Public/private projections, diagnostics, effect/replay/browser no-leak, view hashes. | Rulepath hidden-information law and Gate 15 pairwise no-leak matrix. | yes | Viewer-safe proof is first-class. |
| `RL-REPLAY-*` | Deterministic replay, hashes, viewer-scoped export/import, stable serialization. | Rulepath replay law and Gate 15 command suite. | no | Trace schema v1 is reused. |
| `RL-BOT-*` | Legal-action-only L0/L1/L2 bots and safe explanations. | Rulepath bot law and Gate 15 bot scope. | yes | No search/RL/hidden sampling. |
| `RL-UI-*` | Presentation-only web surface, safe controls, safe ledger, safe explanation, no casino/trade-dress posture. | Rulepath UI/IP law and Gate 15 web requirements. | yes | TypeScript computes no game behavior. |

## Final source/IP checklist

- Sources are summarized in original language.
- Source quality and date consulted are recorded.
- Variant choices and deviations are explicit.
- Ambiguities connect to rule IDs and tests.
- Public naming avoids affiliation and trade-dress risk.
- Rules prose and source notes are original.
- Assets are absent in this ticket; later assets require provenance review.
- Fonts are absent in this ticket.
- Public/private content boundary is explicit.
- Human/legal review triggers are not hidden.
- Release blockers are recorded.
