# masked_claims Mechanics Inventory

Game ID: `masked_claims`

Roadmap stage/gate: Gate 11 - bluffing / reaction-window proof

Rules version: `masked-claims-rules-v1`

Prepared by: `Codex`

Last updated: 2026-06-11

## Purpose

This inventory applies `docs/MECHANIC-ATLAS.md` to Masked Claims. It records
the game-local mechanic shapes, repeated-shape pressure, and extraction
posture. It is evidence, not permission to generalize.

## Mechanic inventory categories

| Category | Game-local description | Evidence in rules | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | No board topology; the play surface is hands, pedestal, veiled galleries, exposed rows, and score/counter areas. | `MC-COMP-004` through `MC-COMP-010` | local-only | Zones are game-local view concepts. |
| component/zone model | Fifteen masks, two private hands, one internal reserve, one pedestal, two veiled galleries, two exposed rows. | `MC-COMP-*`, `MC-SETUP-*` | repeated-shape candidate | Private-hand pressure is recorded in `PRIMITIVE-PRESSURE-LEDGER.md`. |
| action shape | Claim actions are compound paths; response actions are flat accept/challenge leaves. | `MC-ACT-*` | local-only | No generic claim or response primitive is added. |
| turn/phase model | Alternating claim phase opens a single responder reaction window, then cleanup advances or ends. | `MC-TURN-*` | local-only | The reaction window is first official local use. |
| randomness/chance | Seeded setup shuffle only; no in-game stochastic event after setup. | `MC-RNG-001` | repeated-shape candidate | Fourth-use shuffle/private-hand review is complete in the ledger. |
| visibility/hidden information | Own hand is private, pedestal identity is hidden until challenge, accepted masks and reserve never reveal. | `MC-VIS-*` | repeated-shape candidate | Hidden-info contracts follow ADR 0004. |
| resource/accounting | Public scores plus exposed-lies, successful-challenges, and challenges-declared counters. | `MC-SCORE-*`, `MC-END-*` | local-only | Counters are terminal-tiebreak evidence only. |
| movement/capture/placement | Claimed masks move from hand to pedestal, then veiled gallery or exposed row. | `MC-TURN-003`, `MC-TURN-004` | local-only | Movement is zone transfer, not board movement. |
| pattern/line/directional scanning | Not applicable. | none | local-only | There is no spatial pattern detection. |
| commitment/reveal | A hidden mask is bound to a public declared grade; challenge may reveal, accept never reveals. | `MC-VIS-003` through `MC-VIS-005` | local-only | Similar to prior reveal pressure but not simultaneous commitment. |
| reaction/window/pending response | One claim opens one responder-only accept/challenge window. | `MC-COMP-007`, `MC-TURN-002`, `MC-ACT-003` | local-only | First official local use; ADR required before broad generalization. |
| scoring/outcome | Eight claim turns produce score win, public tiebreak win, or draw. | `MC-SCORE-*`, `MC-END-*` | local-only | Outcome rationale is public and deterministic. |
| semantic effect shape | Claim, window opened, accept, challenge, reveal, score change, turn advance, terminal. | `MC-TURN-*`, `MC-SCORE-*` | local-only | Effects drive logs, animation, replay, and explanations. |
| UI interaction pattern | Claimant selects mask then grade; responder sees accept/challenge; non-actors see waiting state. | `MC-ACT-*`, `MC-VIS-*` | local-only | TypeScript presents Rust action trees only. |
| bot policy pattern | Random legal and Level 1 claim/response policy using legal APIs and allowed views. | `MC-BOT-*` | local-only | No MCTS, sampling, ML, or hidden-state peeking. |
| benchmark/performance pressure | Legal action generation, claim/response apply, view projection, replay/export, and Level 1 decisions. | `BENCHMARKS.md` | local-only | Smoke floors pending calibration. |

## Repeated-shape comparison

| Mechanic shape | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---:|---|---|---|
| deterministic shuffle / private hand / staged reveal | `high_card_duel`, `secret_draft`, `poker_lite`, `plain_tricks` | yes | Seeded setup, private ownership, viewer-filtered projection. | Masks use graded claims and accepted masks never reveal. | Ledger records fourth-use defer/reject posture; no promotion. |
| reaction window / pending response | none as an official mechanic row | no | Secret Draft has waiting-state UI, but not a responder interrupt. | Masked Claims has a responder-only accept/challenge window after a claim. | Record first official local use; ADR required if generalized. |
| simultaneous commitment/reveal | `secret_draft` candidate row | no | Hidden placement can later reveal. | Claim is sequential, single-seat, and conditionally revealed; no simultaneous selection. | Keep the simultaneous row unchanged. |

## Primitives reused

| Primitive | Source | Why reused | Rule IDs covered | Tests proving compatibility | Notes |
|---|---|---|---|---|---|
| `Actor`, `Viewer`, `ActionTree`, `CommandEnvelope`, `EffectEnvelope`, `VisibilityScope` | `engine-core` | Generic contracts already express actor-specific action trees and viewer filtering. | `MC-ACT-*`, `MC-VIS-*` | `cargo test -p masked_claims`; wasm-api bridge test | No game nouns enter the kernel. |
| deterministic `SeededRng` discipline | `engine-core` | Setup shuffle must replay from seed. | `MC-RNG-001` | `tests/replay.rs`; `tests/property.rs` | No game-stdlib promotion. |

## Local mechanics

| Local mechanic | Why local | Extraction risk | Rule IDs | Tests/traces | Notes |
|---|---|---|---|---|---|
| claim pedestal | Specific to binding one mask to one declared grade. | medium | `MC-COMP-006`, `MC-VIS-003` | `claim-pending-window.trace.json`; `tests/visibility.rs` | Must not become a generic claim engine. |
| accept/challenge window | First official use; unresolved broad design. | high | `MC-COMP-007`, `MC-TURN-002` | `claim-pending-window.trace.json`; `tests/rules.rs` | ADR required before broad reaction abstraction. |
| veiled accepted gallery | This game proves accepted hidden facts never reveal. | medium | `MC-COMP-008`, `MC-VIS-005` | `accepted-mask-never-revealed.trace.json` | Not a generic hidden-zone helper. |
| public tiebreak counters | Specific scoring rationale for this variant. | low | `MC-SCORE-004`, `MC-SCORE-005`, `MC-END-*` | terminal traces; `tests/rules.rs` | Pure game-local accounting. |

## Primitive candidates

| Candidate | Status | Games exerting pressure | Required next step | Blocker? |
|---|---|---|---|---:|
| deterministic shuffle / private hand / staged reveal | rejected/deferred with rationale | `high_card_duel`, `secret_draft`, `poker_lite`, `plain_tricks`, `masked_claims` | Keep local patterns and revisit only with concrete duplication pain. | no |
| reaction window / pending response | ADR-required | `masked_claims` first use | Record next reaction-capable game as repeated-pressure trigger. | no |
| claim/challenge resolution | ADR-required | `masked_claims` only | Do not generalize from one game. | no |

## Extraction or defer rationale

| Shape | Decision | Rationale | Back-port needed? | Trace impact | Benchmark impact |
|---|---|---|---:|---|---|
| deterministic shuffle / private hand / staged reveal | defer | Current local implementations remain understandable; promotion would need a narrower proof than this gate supplies. | no | none | none |
| reaction window / pending response | local | First use is intentionally game-local and single-depth. | no | none | reaction-window benchmarks remain game-local |
| simultaneous commitment/reveal | reject as same-shape | Masked Claims is sequential and conditional, not simultaneous. | no | none | none |

## Effects, UI, and bot notes

| Area | Requirement | Rule IDs | Notes |
|---|---|---|---|
| semantic effects | claim placed, window opened, accept, challenge, reveal, score, turn, terminal | `MC-TURN-*`, `MC-SCORE-*` | Effects are the UI/replay animation authority. |
| UI interaction pattern | mask+grade selection, responder choices, waiting state | `MC-ACT-*` | Rust action tree controls all choices. |
| Rust-generated previews | action metadata gives declared grade and safe waiting context | `MC-ACT-006` | No hidden tile metadata in public surfaces. |
| bot policy pattern | Level 0 random legal; Level 1 claim/response baseline | `MC-BOT-*` | Uses own/public allowed view only. |
| visibility/no-leak | hidden pedestal, veiled masks, hands, reserve, bot rationales, exports | `MC-VIS-*`, `MC-RNG-002` | Native visibility tests and wasm bridge cover no-leak paths. |
| benchmark pressure | action trees, apply, projection, replay/export, bot decisions | `BENCHMARKS.md` | Smoke floors pending calibration. |

## Required repo atlas update

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | yes | Already updated for Gate 11 primitive/reaction rows. | maintainers |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes | Already records fourth-use shuffle decision and first-use reaction window. | maintainers |
| ADR | no | No broad primitive is promoted by this game. | maintainers |

## Anti-patterns

- Do not promote claim, challenge, mask, grade, pedestal, or reaction nouns into `engine-core`.
- Do not treat the reaction window as a reusable interrupt stack.
- Do not move scoring, legality, visibility, or bot policy into static data or TypeScript.
- Do not reveal veiled masks, unplayed hands, the reserve, or unchallenged pedestal identities.

## Review checklist

- All mechanic atlas categories are filled.
- Repeated shapes are compared against prior official games.
- Third-use/fourth-use pressure is documented.
- `engine-core` remains noun-free.
- Static data remains typed metadata and fixtures only.
- UI, bot, visibility, and benchmark impacts are recorded.
