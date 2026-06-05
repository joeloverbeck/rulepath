# race_to_n Sources

Game ID: `race_to_n`

Public display name: `Race to 21`

Implemented variant: `single-counter normal-play race to 21; add 1, 2, or 3`

Prepared by: `Codex`

Created: 2026-06-05

Last updated: 2026-06-05

Rules version connected to this source note: `race_to_n-rules-v1`

## Source-use statement

Rulepath uses consulted sources to verify rules, variants, terminology context,
public-history context, and ambiguity resolution.

Sources do not authorize copied prose, component text, icons, screenshots,
scans, fonts, assets, or trade dress. Rulepath rule prose, UI copy, visual
presentation, assets, icons, and component text for `race_to_n` are original.

No source prose, rules text, examples, assets, or presentation were copied into
this game. The implemented game is a neutral numeric abstraction in the Nim /
subtraction-game family.

## Consulted sources

| Source name | URL/reference | Date consulted | Source quality | Used for | Copied prose/assets status | Notes |
|---|---|---|---|---|---|---|
| Wikipedia: Nim | `https://en.wikipedia.org/wiki/Nim` | 2026-06-05 | reputable secondary | terminology and normal-play / misere comparison | none | Used only to confirm the public family context of two-player take-away games and normal-play vs misere outcome conventions. |
| Wikipedia: Subtraction game | `https://en.wikipedia.org/wiki/Subtraction_game` | 2026-06-05 | reputable secondary | rule-family context and subtraction-set comparison | none | Used only to confirm that single-number games with allowed subtractions are a recognized abstract family. |
| Cornell Math Explorers: How to play NIM | `https://pi.math.cornell.edu/~mec/2003-2004/graphtheory/nim/howtoplaynim.html` | 2026-06-05 | educational secondary | variant comparison | none | Used only as a plain-language cross-check for last-move-wins normal play in Nim-family teaching material. |
| Rulepath Gate 1 spec | `specs/gate-1-race-to-n.md` | 2026-06-05 | project authority | product scope and evidence requirements | none | Governs `foundation-smoke` role, Rust authority, docs, tests, replay, bot, WASM, and benchmark obligations. |
| Rulepath Official Game Contract | `docs/OFFICIAL-GAME-CONTRACT.md` | 2026-06-05 | project authority | documentation and evidence workflow | none | Governs source notes, original rules prose, coverage matrix, and official-game evidence. |
| Rulepath IP Policy | `docs/IP-POLICY.md` | 2026-06-05 | project authority | naming and IP safety | none | Supports a neutral public name and forbids copied prose, assets, and trade dress. |

## Variant choice and deviations

| Item | Decision | Source/rationale | Public-facing note needed? |
|---|---|---|---:|
| implemented variant | Single shared counter starts at 0. On each turn the active seat adds 1, 2, or 3 without passing 21. The mover who makes the counter exactly 21 wins. | Chosen as the smallest deterministic two-seat numeric game that proves setup, flat legal actions, validation, terminal detection, effects, replay, bot legality, and WASM plumbing without hidden information. | yes |
| player count | Exactly 2 seats. | Gate 1 requires a tiny two-seat game. | yes |
| optional rule included | Normal-play last move wins. | Simpler for first plumbing proof than misere; terminal outcome is tied directly to the action that reaches the target. | yes |
| optional rule excluded | Misere last move loses. | Adds an outcome inversion that is not needed for Gate 1 plumbing evidence. | yes |
| Rulepath deviation from common variant | Uses addition toward a target instead of removing objects from one or more piles. | Equivalent numeric pressure for this gate, but terminology stays neutral and UI-friendly for `race_to_n`. | yes |
| out-of-scope variant | Multi-pile Nim, arbitrary subtraction sets, misere play, target choices beyond the declared variant, randomized setup. | Gate 1 forbids generalized piles/resources and asks for a single declared variant. | yes |

## Ambiguity log

| Ambiguity ID | Ambiguity | Sources compared | Chosen resolution | Rule ID(s) affected | Tests/traces required | Status |
|---|---|---|---|---|---|---|
| `AMB-001` | Whether Gate 1 should use take-the-last subtraction or race-to-N counting. | Gate 1 spec, Nim-family sources, subtraction-game source. | Use race-to-N counting because the game ID and public role emphasize a target counter while preserving the same small legal-action shape. | `R-VAR-001`, `R-SETUP-003`, `R-ACTION-001`, `R-END-001` | Shortest normal trace, terminal trace, rule tests for target-bounded legal actions. | resolved |
| `AMB-002` | Whether a move may overshoot the target. | Design rationale. | No overshoot. Legal additions are capped at the remaining distance to 21, so terminal state is exact and easy to validate. | `R-ACTION-001`, `R-RESTRICT-001`, `R-END-001` | Rule tests for totals 18, 19, and 20; invalid diagnostic trace for overshoot. | resolved |
| `AMB-003` | Which seat acts first. | Design rationale and deterministic replay needs. | Seat 0 acts first. No random setup. | `R-SETUP-002`, `R-TURN-001`, `R-RNG-001` | Setup snapshot and replay/hash tests. | resolved |

## Public naming rationale

Public ID: `race_to_n`

Display name: `Race to 21`

| Concern | Decision | Notes |
|---|---|---|
| common descriptive name safe? | yes | `Race to 21` describes an abstract counting race, not a branded product. |
| neutral name chosen? | yes | Avoids public product names from commercial games. |
| trademark risk considered? | yes | No commercial title, logo, or affiliation is used. |
| trade-dress risk considered? | yes | The planned UI is a minimal Rulepath harness, not a copy of any source presentation. |
| affiliation implication avoided? | yes | Source notes and public copy do not imply affiliation with any rules source. |
| public help text needs disclaimer? | no | No trademark-forward or brand-adjacent public name is used. |

## Trademark and trade-dress concerns

| Concern | Risk level | Mitigation | Release blocker? |
|---|---|---|---:|
| Nim-family public terminology | low | Public game name is `Race to 21`; Nim appears only in source/context notes. | no |
| Visual resemblance to source examples | none | No source visuals are used; UI will be original and minimal. | no |

## Asset provenance

| Asset group | Files/IDs | Status | Author/license/source | Review notes | Public-safe? |
|---|---|---|---|---|---:|
| Game docs | `games/race_to_n/docs/*.md` | original | Rulepath/Codex-authored prose | No copied rulebook text. | yes |
| Game UI/assets | none in this ticket | not applicable | none | UI/assets are out of scope for GAT1RACTON-001. | yes |

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
| Rulepath original rules summary | yes | `games/race_to_n/docs/RULES.md` | Original Rulepath prose. |
| public-domain/classic rule facts | yes | `games/race_to_n/docs/SOURCES.md` | Summarized as family context only. |
| commercial/licensed rules text | no | none | No commercial rules text is used. |
| private licensed stress-test content | no public shipment | none | No private licensed content is involved. |
| source screenshots/scans | no | none | No screenshots/scans are used. |

## Human/legal review triggers

| Trigger | Applies? | Notes/blocker |
|---|---:|---|
| commercial game name or recognizable branding | no | Neutral name only. |
| copied or closely paraphrased rules prose | no | Rules are original. |
| card/component text from a protected source | no | No cards or protected component text. |
| scanned/copied art, icon, screenshot, board, card, or UI asset | no | No art or UI asset in this ticket. |
| bundled font file | no | None. |
| generated art with possible trade-dress similarity | no | None. |
| uncertainty about public-domain status | no | Sources are used only as context; the implemented rules are original. |
| source forbids redistribution or reuse | no | No source material is redistributed. |
| private licensed content touched public path | no | None. |

## Release blocking concerns

| Concern | Blocking? | Required resolution | Owner |
|---|---:|---|---|
| none identified for GAT1RACTON-001 | no | Continue evidence workflow in later tickets. | Rulepath |

## Rule-source-to-rule-ID cross-reference

| Rule ID | Rule summary | Source(s) or design rationale | Ambiguity/deviation? | Notes |
|---|---|---|---:|---|
| `R-SCOPE-001` | Tiny deterministic two-seat numeric race. | Gate 1 spec. | no | Plumbing proof role. |
| `R-SCOPE-002` | Foundation-smoke public role. | Gate 1 spec and OGC readiness labels. | no | Modest UI, full evidence. |
| `R-VAR-001` | Single-counter race to 21. | `AMB-001` design rationale. | yes | Selected variant. |
| `R-VAR-002` | Fixed parameters: target 21, max add 3, two seats, seat 0 first. | Gate 1 simplification and replay needs. | no | No setup options in Gate 1. |
| `R-COMP-001` | Counter. | Design rationale. | no | Public integer value. |
| `R-COMP-002` | Seat. | Gate 1 two-seat requirement. | no | `seat_0`, `seat_1`. |
| `R-SETUP-001` | Initial total is 0. | Design rationale. | no | Deterministic setup. |
| `R-SETUP-002` | Seat 0 starts. | `AMB-003`. | yes | Deterministic setup. |
| `R-SETUP-003` | Variant constants are fixed. | `AMB-001`, `AMB-002`. | yes | Target and max add are not user-selected. |
| `R-TURN-001` | Active seat chooses one legal addition. | Design rationale. | no | Flat action shape. |
| `R-TURN-002` | Non-terminal moves pass turn to the other seat. | Design rationale. | no | No cleanup phase. |
| `R-ACTION-001` | Legal additions are 1..3 capped by remaining distance. | `AMB-002`. | yes | Rust-generated. |
| `R-RESTRICT-001` | Overshoot, zero, negative, wrong-seat, and terminal submissions are illegal. | `AMB-002` and Rust authority requirement. | yes | Viewer-safe diagnostics required. |
| `R-SCORE-001` | No score beyond winner/outcome. | Gate 1 tiny game scope. | no | Terminal winner is the only result. |
| `R-END-001` | Reaching 21 ends the game and the mover wins. | Normal-play design choice. | yes | Exact target only. |
| `R-VIS-001` | All game state is public. | Perfect-information Gate 1 scope. | no | No hidden information. |
| `R-RNG-001` | Game setup and rules use no randomness. | Deterministic game design. | no | Bot RNG is external to rules. |
| `R-RNG-002` | Replay records commands and hashes deterministic surfaces. | Gate 1 spec. | no | Implemented in later tickets. |
| `R-AMB-001` | Race-to-N counting selected over take-away phrasing. | `AMB-001`. | yes | Pinning assumption 1. |
| `R-AMB-002` | No overshoot. | `AMB-002`. | yes | Exact terminal target. |

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
