# Token Bazaar Sources

Game ID: `token_bazaar`

Public display name: `Token Bazaar`

Implemented variant: `token_bazaar_standard`

Prepared by: `Codex`

Created: 2026-06-08

Last updated: 2026-06-08

Rules version connected to this source note: `token-bazaar-rules-v1`

## Source-use statement

Token Bazaar is an original Rulepath ruleset. It uses generic public-economy
vocabulary such as market, contracts, supply, payment, and resources only as
descriptive mechanism vocabulary. It does not copy commercial board or card game
rules, names, prose, examples, component text, icons, screenshots, scans, fonts,
assets, art direction, or trade dress.

Sources do not authorize copied prose, token text, card text, icons,
screenshots, scans, fonts, assets, art direction, component text, or trade
dress. Rulepath rule prose, UI copy, visual presentation, assets, icons, and
component text for `token_bazaar` are original.

## Consulted sources

All sources in this table are rationale or project-authority sources only. No
source prose or assets are copied.

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Gate 9 Token Bazaar Browser Proof spec | `../../../specs/gate-9-token-bazaar-browser-proof.md` | 2026-06-08 | project authority | product scope, rule identity, contract queue, action families, browser proof requirements | none | Governs `token_bazaar` identity, rules, docs, tests, replay, bot, WASM, UI, benchmark, and capstone obligations. |
| Rulepath Official Game Contract | `../../../docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-08 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, and official-game evidence. |
| Rulepath IP Policy | `../../../docs/IP-POLICY.md` | 2026-06-08 | project authority | naming and IP safety | none | Supports neutral public naming and forbids copied prose, assets, and trade dress. |
| BoardGameGeek mechanics vocabulary | `https://boardgamegeek.com/browse/boardgamemechanic` | 2026-06-08 | community taxonomy/reference | generic vocabulary context for market/contracts as mechanism labels | none | Used only to confirm that broad mechanism words are generic vocabulary, not as a rules model. |
| WAI-ARIA Authoring Practices Guide | `https://www.w3.org/WAI/ARIA/apg/` | 2026-06-08 | standards guidance | UI keyboard and widget affordance reference | none | UI guidance only; not a rules source. |
| WCAG Understanding Success Criterion 1.4.3: Contrast (Minimum) | `https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html` | 2026-06-08 | standards guidance | resource chip and contract-card readability reference | none | UI accessibility guidance only; not a rules source. |
| WCAG Understanding Success Criterion 2.3.3: Animation from Interactions | `https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html` | 2026-06-08 | standards guidance | reduced-motion rationale for browser accounting feedback | none | UI accessibility guidance only; not a rules source. |

## Adopted design facts

The implemented `token_bazaar_standard` variant adopts these facts in original
Rulepath prose:

| Adopted item | Rulepath statement | Rationale |
|---|---|---|
| Two-seat public economy | Exactly two seats, `seat_0` and `seat_1`, play with all state public. | Gate 9 proves public accounting rather than hidden information. |
| Three local resources | The game-local resources are `amber`, `jade`, and `iron`. | Three resource types are enough to prove costs, payments, exchange, supply exhaustion, and readable UI chips. |
| Shared supply and public inventories | Supply starts at 14 of each resource; each seat starts with 1 of each. | Bounded counts keep simulations small while giving collect/exchange/fulfill actions room to matter. |
| Three-slot market | `slot_0`, `slot_1`, and `slot_2` show visible contracts. | Visible market state is the browser proof surface. |
| Deterministic ten-contract queue | Contract order is fixed and refills from the queue front. | Deterministic refill keeps replay and hashes stable without RNG. |
| Collect bundles | Six collect bundles allow either two matching resources or one of two adjacent mixed pairs. | Small action tree proves Rust-owned choices and supply gating. |
| Inefficient exchange | Exchange pays two matching resources for one different resource. | The action proves supply return and conversion legality without dominating fulfillment. |
| Exact-cost fulfillment | A visible contract can be fulfilled only by paying its exact cost. | Exact payments make accounting effects and replay checks clear. |
| Eight-turn cap | Each seat may take at most eight turns. | Finite matches keep simulation, e2e, and benchmark proof bounded. |
| Public deterministic tie-breaks | Score, fulfilled-contract count, total inventory, then draw. | Terminal outcomes require no hidden data or browser computation. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | `token_bazaar_standard` | Gate 9 public resource/economy proof scope. | yes |
| player count | Exactly two seats. | Small official-game proof with clear turn alternation and browser controls. | yes |
| first seat | `seat_0`. | Deterministic setup and replay simplicity. | yes |
| turn cap | 8 turns per seat. | Gate 9 finite-proof scope. | yes |
| optional rule included | Collect, exchange, fulfill, forced pass, deterministic refill, terminal tie-breaks. | Gate 9 acceptance requirements. | yes |
| optional rule excluded | Hidden simultaneous commitments, auctions, betting, negotiation, random setup/refill, alternate queues, more than two seats, generic economy primitives. | Out of scope for Gate 9 and deferred or forbidden by the spec. | yes |
| Rulepath deviation from common economy games | Fully public, deterministic, fixed queue, no trading, no auctions, no randomness. | Keeps the gate focused on resource accounting and browser proof. | yes |
| out-of-scope variant | `secret_draft` commitment/reveal gate. | Gate 9 spec explicitly defers simultaneous hidden choices to Gate 9.1. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `TB-AMB-001` | Whether upcoming queued contracts are hidden from players. | Gate 9 spec, browser proof goals. | The game has no hidden information; Rust owns any projection of queue count or upcoming public queue data. | `TB-SETUP-005`, `TB-SETUP-006`, `TB-VIS-001`, `TB-VIS-002` | public-view, serialization, replay, and browser smoke tests | resolved |
| `TB-AMB-002` | Whether pass may be voluntary. | Gate 9 forced-pass rule. | `pass` is legal only when no collect, exchange, or fulfill action is legal. | `TB-ACT-004`, `TB-RESTRICT-004` | forced-pass legality test if reachable | resolved |
| `TB-AMB-003` | Whether remaining inventory tie-break values resource types differently. | Gate 9 winner and tie-break rule. | Remaining inventory tie-break is the total count of resources, with no resource-type value ordering. | `TB-SCORE-005`, `TB-END-003` | terminal tie-break tests and golden traces | resolved |

## Public naming rationale

Public ID: `token_bazaar`

Display name: `Token Bazaar`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | yes | The name uses generic words for local tokens and a public exchange setting. |
| neutral name chosen? | yes | `Token Bazaar` is Rulepath naming for this original variant. |
| trademark risk considered? | yes | Public docs and UI avoid product branding, source names, logos, slogans, and affiliation wording. |
| trade-dress risk considered? | yes | Resource chips, market cards, colors, labels, icons, layout, animation, and help text must be original Rulepath presentation. |
| affiliation implication avoided? | yes | Sources are cited only as rationale, project authority, or accessibility references. |
| public help text needs disclaimer? | no | Neutral naming and source notes are sufficient unless later review requests additional text. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---:|---|---:|
| Generic market/contracts/resource vocabulary | low for vocabulary, expression still reviewed | Use original prose, original labels, and Rulepath presentation. | no |
| Commercial resource-economy game resemblance | medium if visual or rule presentation is imitated | Do not copy commercial rule structures, examples, component text, resource names, art direction, icon style, or layout. | yes if found |
| Contract labels | low | Use original placeholder labels listed in `RULES.md`; rename only for clarity and IP safety. | no |
| Browser resource chips or contract cards | medium if styled after a recognizable product | Use original Rulepath UI, system fonts, and accessible labels. | yes if copied |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| Game docs | `games/token_bazaar/docs/RULES.md`, `games/token_bazaar/docs/SOURCES.md` | original | Rulepath/Codex-authored prose | No copied rulebook text. | yes |
| Resource names | `amber`, `jade`, `iron` | original local identifiers using generic material words | Rulepath/Codex-authored local vocabulary | Names are game-local and not source-derived. | yes |
| Contract ids and labels | `balanced-wares`, `amber-guild`, `iron-guild`, `jade-guild`, `amber-focus`, `jade-focus`, `iron-focus`, `sun-route`, `stone-route`, `crown-route` | original placeholder labels | Rulepath/Codex-authored identifiers and labels | No commercial source is used as a label model. | yes |
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
| Rulepath original rules summary | yes | `games/token_bazaar/docs/RULES.md` | Original Rulepath prose. |
| Project-authority Gate 9 facts | yes | `games/token_bazaar/docs/SOURCES.md` | Summarized as rationale only. |
| Generic market/contracts vocabulary | yes | `games/token_bazaar/docs/RULES.md`, `games/token_bazaar/docs/SOURCES.md` | Used as generic mechanism vocabulary, not copied expression. |
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
| card/component text from a protected source | no | Local resource and contract labels only. |
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
rationale support here. Token Bazaar is original, so the primary support is the
Gate 9 spec plus the design rationale summarized below.

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `TB-COMP-001` through `TB-COMP-007` | Game-local seats, resources, supply, inventories, contracts, market slots, and fulfilled lists. | Gate 9 spec and original public-accounting proof design. | no | Vocabulary remains game-local. |
| `TB-SETUP-001` through `TB-SETUP-006` | Deterministic two-seat setup, supply, inventories, scores, queue, and initial market. | Gate 9 spec and replay-stability requirement. | no | No RNG. |
| `TB-TURN-001` through `TB-TURN-004` | Alternating active turns, eight-turn cap, and terminal gameplay stop. | Gate 9 spec and bounded simulation/e2e requirement. | no | `seat_0` starts. |
| `TB-ACT-001` through `TB-ACT-005` | Rust-owned collect, exchange, fulfill, forced pass, and terminal action legality. | Gate 9 spec and behavior-authority boundary. | no | TypeScript computes no legality. |
| `TB-COLLECT-001` through `TB-COLLECT-006` | Six collect bundles and supply gating. | Gate 9 spec and small action-tree proof design. | no | Bundle ids are stable. |
| `TB-EXCHANGE-001` through `TB-EXCHANGE-003` | Two-for-one exchange constraints and effects. | Gate 9 spec and conversion-legality proof design. | no | Intentionally inefficient. |
| `TB-FULFILL-001` through `TB-FULFILL-006` | Visible slot fulfillment, exact payment, scoring, fulfilled list, refill, terminal check. | Gate 9 spec and market-refill proof design. | no | Contract costs/points are fixed. |
| `TB-RESTRICT-001` through `TB-RESTRICT-004` | Wrong-seat, invalid, stale, and forced-pass restrictions. | Rulepath diagnostic/replay invariants plus Gate 9 spec. | no | Reject without mutation. |
| `TB-SCORE-001` through `TB-SCORE-005` | Score, accounting deltas, exchange deltas, fulfill deltas, and inventory tie-break. | Gate 9 spec and effect-visible accounting requirement. | no | All facts public. |
| `TB-END-001` through `TB-END-003` | Turn-cap terminal, market-exhaustion terminal, and tie-break order. | Gate 9 spec. | no | Draw only after all tie-breaks tie. |
| `TB-VIS-001` through `TB-VIS-003` | Fully public projection and no hidden-state surfaces. | Gate 9 spec and Rulepath no-leak invariant. | no | Gate 9.1 handles hidden commitments. |
| `TB-RNG-001` through `TB-RNG-003` | No randomness, deterministic queue/refill, stable serialization/replay. | Gate 9 spec and replay/hash invariants. | no | Do not touch `engine-core::DeterministicRng`. |
| `TB-AMB-001` through `TB-AMB-003` | Queue projection, forced pass, and inventory tie-break resolutions. | Gate 9 spec and original design rationale. | yes | Covered by future tests/traces. |
| `TB-VAR-001` through `TB-VAR-003` | Original variant boundaries and out-of-scope variants. | Gate 9 scope and successor Gate 9.1 sequencing. | yes | No generic primitive promotion here. |

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
