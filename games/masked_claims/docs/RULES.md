# Masked Claims Rules

Game ID: `masked_claims`

Public display name: `Masked Claims`

Implemented variant: `masked_claims_standard`

Rules version: `masked-claims-rules-v1`

Prepared by: `Codex`

Created: 2026-06-11

Last updated: 2026-06-11

## Rule authority

This document is the original Rulepath rules summary for the implemented
variant. Sources belong in `SOURCES.md`; this document states the Rulepath
implementation contract.

Stable rule IDs are requirements. They must remain stable after implementation
unless intentionally migrated with a migration note and corresponding updates in
`RULE-COVERAGE.md`, traces, tests, and docs.

## Metadata

| Field | Value |
|---|---|
| game id | `masked_claims` |
| public display name | `Masked Claims` |
| variant | `masked_claims_standard` |
| rules version | `masked-claims-rules-v1` |
| source note | `games/masked_claims/docs/SOURCES.md` |
| coverage matrix | `games/masked_claims/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/masked_claims/docs/MECHANICS.md` |
| implementation admission | `games/masked_claims/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

Masked Claims is a two-seat original Rulepath hidden-information bluffing
microgame. Each turn, one seat commits one hidden mask to a public grade claim.
The other seat then holds a constrained reaction window and may accept the claim
or challenge it. Accepting leaves the mask hidden forever; challenging reveals
that one mask and resolves the claim from its actual grade.

The game proves claim actions, pending response windows, responder-only legal
actions, conditional reveal and scoring, deterministic terminal tiebreaks,
viewer-safe logs, hidden information that remains redacted for the match
lifetime, replay/export redaction, and Rust-owned bot decisions in both the
claim and response roles.

Rust owns setup, shuffle/deal, legal action generation, validation, reaction
window membership, resolution, reveal timing, scoring, terminal detection,
semantic effects, replay behavior, visibility projection, and bot decisions.
TypeScript may present only Rust/WASM output.

The game does not implement a general claim engine, reaction-window engine,
bluffing framework, role game, interrupt stack, hosted multiplayer, reaction
timeouts, or shared engine vocabulary for this game's local nouns.

## Implemented variant

The only shipped variant is `masked_claims_standard`.

| Field | Value |
|---|---|
| seats | `seat_0`, `seat_1` |
| masks | fifteen tiles: three copies of each grade 1 through 5 |
| grade labels | `Plain`, `Trimmed`, `Gilded`, `Jeweled`, `Master` |
| private hands | five masks per seat at setup |
| reserve | five undealt masks; internal only and never revealed |
| claim turns | exactly eight, alternating claimant each turn |
| reaction choices | accept or challenge |
| accepted masks | placed in a veiled gallery and never revealed |
| challenged masks | revealed and placed in an exposed row |
| terminal outcomes | score winner, tiebreak winner, or draw |
| maximum gameplay actions | exactly sixteen: eight claims plus eight responses |

## Components and game-local vocabulary

Game nouns in this section belong to `games/masked_claims` only. They do not
authorize `mask`, `grade`, `claim`, `challenge`, `reaction`, `pedestal`,
`gallery`, or similar nouns in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `MC-COMP-001` | seat | One of the two players, `seat_0` or `seat_1`. | public | No other player counts ship in this variant. |
| `MC-COMP-002` | mask tile | One game-local hidden component with a stable id and grade. | private, hidden, or public depending on reveal timing | A tile id becomes public only if that tile is challenged and revealed. |
| `MC-COMP-003` | grade | One of five ordered mask values, 1 through 5. | visible when safely projected | Grade labels are original Rulepath words. |
| `MC-COMP-004` | private hand | The five masks owned by a seat at setup. | owner only until each mask is claimed or the match ends | Unplayed hand masks remain hidden forever to non-owners. |
| `MC-COMP-005` | reserve | The five masks not dealt to either seat. | internal only | Reserve identities are never revealed, including at terminal. |
| `MC-COMP-006` | claim pedestal | The temporary face-down area holding the claimed mask and public declared grade while a response is pending. | mixed | The declared grade is public; the mask identity is hidden until a challenge reveals it. |
| `MC-COMP-007` | reaction window | The response phase opened by a claim. | public as phase metadata | Only the responder has accept/challenge actions. |
| `MC-COMP-008` | veiled gallery | A claimant's accepted masks. | public count and declared grades only | Tile identities in this gallery never reveal. |
| `MC-COMP-009` | exposed row | Revealed masks from challenged claims. | public | Contains only masks revealed by challenge. |
| `MC-COMP-010` | score and tiebreak counters | Public totals and public challenge-discipline counters. | public | Used for terminal outcome and rationale. |

### Standard mask list

Static data may carry mask IDs, grade labels, setup constants, fixture metadata,
and version declarations. Static data must not carry legality, resolution,
scoring, tiebreaks, bot policy, selectors, triggers, formulas, or scripts.

| Stable order | Mask ID | Grade | Label |
|---:|---|---:|---|
| 1 | `mask_g1_a` | 1 | Plain A |
| 2 | `mask_g1_b` | 1 | Plain B |
| 3 | `mask_g1_c` | 1 | Plain C |
| 4 | `mask_g2_a` | 2 | Trimmed A |
| 5 | `mask_g2_b` | 2 | Trimmed B |
| 6 | `mask_g2_c` | 2 | Trimmed C |
| 7 | `mask_g3_a` | 3 | Gilded A |
| 8 | `mask_g3_b` | 3 | Gilded B |
| 9 | `mask_g3_c` | 3 | Gilded C |
| 10 | `mask_g4_a` | 4 | Jeweled A |
| 11 | `mask_g4_b` | 4 | Jeweled B |
| 12 | `mask_g4_c` | 4 | Jeweled C |
| 13 | `mask_g5_a` | 5 | Master A |
| 14 | `mask_g5_b` | 5 | Master B |
| 15 | `mask_g5_c` | 5 | Master C |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `MC-SETUP-001` | Create exactly two seats, `seat_0` and `seat_1`. | deterministic | public | No other seat counts ship. |
| `MC-SETUP-002` | Construct the fifteen-mask set in stable order, then shuffle it with Rulepath's deterministic seeded RNG discipline. | seeded deterministic | internal until projection | Same seed and rules version must reproduce the deal. |
| `MC-SETUP-003` | Deal five masks to `seat_0`, five masks to `seat_1`, and leave five masks as the internal reserve. | seeded deterministic | mixed | Each owner sees only their own hand; the reserve is never visible. |
| `MC-SETUP-004` | Turn 1 starts with `seat_0` as claimant; claimants alternate every turn. | deterministic | public | Each seat makes exactly four claims. |
| `MC-SETUP-005` | Initialize empty pedestal, veiled galleries, exposed rows, scores, tiebreak counters, and terminal outcome. | deterministic | public or internal as appropriate | Hidden containers project only safe counts or visible contents. |

## Turn and phase sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `MC-TURN-001` | Claim phase. | current claimant | The claimant chooses one held mask and one declared grade. | A valid claim is applied and the reaction window opens. |
| `MC-TURN-002` | Reaction window. | responder only | The responder chooses accept or challenge. The claimant has no gameplay action and receives safe waiting metadata. | A valid response is applied. |
| `MC-TURN-003` | Accepted claim resolution. | none during resolution | Rust scores the declared grade for the claimant and moves the mask face-down to the claimant's veiled gallery. | Resolution completes. |
| `MC-TURN-004` | Challenged claim resolution. | none during resolution | Rust reveals the pedestal mask, compares actual grade with declared grade, scores the honest or exposed result, and moves the revealed mask to an exposed row. | Resolution completes. |
| `MC-TURN-005` | Non-final cleanup. | next claimant | Rust clears the pedestal, advances the turn, and alternates claimant. | The next claim is applied. |
| `MC-TURN-006` | Final cleanup after turn 8. | none | Rust resolves the terminal outcome and emits the public terminal rationale. | Terminal outcome is recorded. |
| `MC-TURN-007` | Terminal state. | none | Expose outcome and no normal gameplay actions. | No further gameplay action advances the game. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality, response
window membership, reveal timing, scoring, terminal outcome, tie handling,
hidden-info filtering, or bot policy.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `MC-ACT-001` | Claimant in claim phase. | One claim for each held mask and each declared grade 1 through 5. | flat leaves such as internal `claim/<mask-id>/<grade>` | Public summaries and exports redact the mask id to the declared grade. |
| `MC-ACT-002` | Responder in claim phase. | none | empty gameplay tree | Waiting metadata may name the claimant and public phase. |
| `MC-ACT-003` | Responder in reaction window. | `respond/accept` and `respond/challenge`. | reaction choices | The tree contains exactly the two response leaves. |
| `MC-ACT-004` | Claimant in reaction window. | none | empty gameplay tree | Waiting metadata may name the responder and explain that a response is pending. |
| `MC-ACT-005` | Terminal state. | none | empty gameplay tree | Terminal states expose no normal gameplay actions. |
| `MC-ACT-006` | Any generated legal action. | safe metadata only | action metadata | Metadata may include action family, actor, turn, declared grade, and safe waiting reasons; it must not include opponent hand, reserve, veiled tile ids, or hidden pedestal identity. |
| `MC-ACT-007` | A non-actor viewer requests actions. | none | empty tree | Non-actor viewers must not receive another seat's private claim leaves. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `MC-RESTRICT-001` | Unknown or non-seat actor submits a gameplay action. | Reject without mutation. | Viewer-safe wrong-actor diagnostic. | Only `seat_0` and `seat_1` can act. |
| `MC-RESTRICT-002` | The wrong seat submits while another seat is active. | Reject without mutation. | Viewer-safe wrong-seat diagnostic. | Diagnostic may name public active seat only. |
| `MC-RESTRICT-003` | A malformed or unavailable action path is submitted. | Reject without mutation. | Viewer-safe unavailable-action diagnostic. | Diagnostic must not include hidden mask facts. |
| `MC-RESTRICT-004` | A stale command is submitted after state has advanced. | Reject without mutation. | Viewer-safe stale-command diagnostic. | Replay/hash state must not change. |
| `MC-RESTRICT-005` | A claim names a mask not in the actor's hand. | Reject without mutation. | Viewer-safe not-in-hand diagnostic that echoes only the submitted id from the actor command. | Diagnostic must not reveal any held alternative. |
| `MC-RESTRICT-006` | A claim names a grade outside 1 through 5. | Reject without mutation. | Viewer-safe invalid-grade diagnostic. | No state mutation. |
| `MC-RESTRICT-007` | A response is submitted outside a reaction window. | Reject without mutation. | Viewer-safe wrong-phase diagnostic. | No state mutation. |
| `MC-RESTRICT-008` | A gameplay action is submitted in terminal state. | Reject without mutation. | Viewer-safe terminal-state diagnostic. | Outcome is final. |

## Scoring and accounting

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `MC-SCORE-001` | Accepting a claim gives the claimant points equal to the declared grade. | accepted claim resolution | none | The accepted mask moves to the claimant's veiled gallery and never reveals. |
| `MC-SCORE-002` | Challenging an honest claim, where actual grade is greater than or equal to declared grade, gives the claimant actual grade plus a 2-point truth bonus. | challenged claim resolution | underclaims are honest | The revealed mask moves to the claimant's exposed row. |
| `MC-SCORE-003` | Challenging an exposed lie, where actual grade is lower than declared grade, gives the claimant 0 for that claim and gives the responder points equal to declared minus actual grade. | challenged claim resolution | gap may be 1 through 4 | The revealed mask moves to the responder's exposed row as a trophy. |
| `MC-SCORE-004` | Each challenge increments the responder's challenges-declared counter. | challenged claim resolution | none | Used only for public tiebreak and rationale. |
| `MC-SCORE-005` | Each exposed lie increments the claimant's exposed-lies counter and the responder's successful-challenges counter. | exposed-lie resolution | none | Used only for public tiebreak and rationale. |
| `MC-SCORE-006` | Public score totals are cumulative across all eight turns. | every resolution | none | Scores and counters are public. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `MC-END-001` | Turn 8 resolves and one seat has more total points. | `ScoreWin` for the higher score. | `MC-END-002` if scores are equal | Terminal rationale may cite final scores. |
| `MC-END-002` | Turn 8 resolves with tied scores and one seat has fewer exposed lies. | `TiebreakWin` for fewer exposed lies. | `MC-END-003` if still tied | Lower exposed-lie count wins the first tiebreaker. |
| `MC-END-003` | Scores and exposed lies are tied, and one seat has more successful challenges. | `TiebreakWin` for more successful challenges. | `MC-END-004` if still tied | Higher successful-challenge count wins the second tiebreaker. |
| `MC-END-004` | Scores, exposed lies, and successful challenges are tied, and one seat declared fewer total challenges. | `TiebreakWin` for fewer challenges declared. | `MC-END-005` if still tied | Lower challenge count wins the third tiebreaker. |
| `MC-END-005` | Turn 8 resolves with all terminal tiebreakers tied. | `Draw` | exact draw | No priority-seat tiebreaker. |
| `MC-END-006` | Terminal state is reached. | no further gameplay actions | none | Terminal does not reveal veiled masks, unplayed hands, or reserve. |

## Outcome explanation traceability

Every scoring and terminal rule that can decide a match has a stable rule ID and
enough detail for the web outcome surface to cite it.

| Outcome/explanation fact | Stable rule ID(s) | Decisive when | Notes for UI explanation |
|---|---|---|---|
| Accepted claim points | `MC-SCORE-001` | A response accepts a claim. | Do not reveal the accepted mask identity. |
| Honest challenge points | `MC-SCORE-002` | A challenge reveals actual grade greater than or equal to declared grade. | The revealed mask may be shown because the challenge made it public. |
| Exposed lie points | `MC-SCORE-003` | A challenge reveals actual grade lower than declared grade. | The revealed mask may be shown because the challenge made it public. |
| Final score win | `MC-END-001` | One final score is higher. | Cite scores only. |
| Exposed-lie tiebreak | `MC-END-002` | Scores tie and exposed-lie counts differ. | Counts are public. |
| Successful-challenge tiebreak | `MC-END-003` | Prior factors tie and successful challenges differ. | Counts are public. |
| Challenge-discipline tiebreak | `MC-END-004` | Prior factors tie and total challenge counts differ. | Counts are public. |
| Draw | `MC-END-005` | Every terminal factor ties. | Do not imply any hidden reserve or veiled tile fact. |

This table is traceability only. It is not a behavior DSL, selector table, or
TypeScript decision source. Rust remains the source of scoring, terminal
detection, and rationale projection.

## Visibility and private information

Public/browser payloads must not reveal hidden information through public views,
seat views, action trees, previews, diagnostics, effect logs, DOM attributes,
test IDs, logs, local storage, replay exports, bot explanations, candidate
rankings, or dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `MC-VIS-001` | Turn number, claimant, responder, declared grade on the pedestal, scores, public counters, exposed rows, veiled-gallery counts and declared grades, and terminal status. | observer and both seat viewers | after setup and after each projection | public view, seat view, action tree metadata, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, dev inspector | These are safe public facts. |
| `MC-VIS-002` | A seat's unplayed hand masks. | owning seat only | while the masks remain in that seat's hand | public view, opponent view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | Opponent and observer payloads must not contain unplayed mask ids, grades, or labels. |
| `MC-VIS-003` | Pedestal mask identity before challenge resolution. | owner only while held; no browser-facing public viewer while on pedestal | hidden until challenged; never visible if accepted | public view, opponent view, action tree metadata, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | Public summaries show only declared grade. |
| `MC-VIS-004` | Challenged mask identity and actual grade. | all viewers | from the reveal effect onward | public view, effects, replay export, DOM | Revealed masks remain public in exposed rows. |
| `MC-VIS-005` | Accepted veiled-gallery mask identities. | no browser-facing viewer | never | public view, seat view after acceptance, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | The claimant no longer receives the accepted tile id after it enters the veiled gallery. |
| `MC-VIS-006` | Reserve mask identities. | no browser-facing viewer | never | public view, seat view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | Native internal tests may inspect the reserve under test authority. |
| `MC-VIS-007` | Legal claim choices. | active claimant through Rust-authorized action tree | claim phase | action tree and controls | The claimant's tree may name only that claimant's own held masks. Public action summaries redact tile ids. |
| `MC-VIS-008` | Legal response choices. | active responder through Rust-authorized action tree | reaction window | action tree and controls | The response tree names only accept/challenge and safe public context. |
| `MC-VIS-009` | Bot rationale and candidate ranking. | public only if projected by Rust as viewer-safe text/data | after bot decision or diagnostics | bot explanation, dev inspector, logs, replay export | Public rationale may cite own view, public counts, and safe declared grades only. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `MC-RNG-001` | Masked Claims uses deterministic seeded shuffle for setup and no later random draw outside bot choice. | Same seed, rules version, variant, and command sequence must reproduce internal state and effects. | internal for hidden deal facts | Shuffle/private-hand implementation follows the primitive-pressure ledger decision. |
| `MC-RNG-002` | Public replay export is viewer-scoped and redacted. | Public exports must not include seed material or command paths that reconstruct hands, accepted masks, or reserve. | public export is redacted | Claim commands are summarized by declared grade, not mask id. |
| `MC-RNG-003` | Serialization order must remain stable. | Golden traces and replay checks must fail on unintended ordering changes. | mixed | Stable order covers masks, actions, effects, views, counters, and summaries. |

## Bot-relevant non-authoritative strategy notes

These notes describe intended product behavior, not extra legal authority.
Implemented bots must choose from the Rust legal tree and validate through the
normal action path.

| Rule ID | Strategy note | Allowed input | Forbidden input |
|---|---|---|---|
| `MC-BOT-001` | A random-legal bot may select any legal leaf from its current action tree with deterministic tie-breaking. | legal action tree and bot RNG stream | direct state mutation or illegal fallback |
| `MC-BOT-002` | A Level 1 claim bot may mix honest claims, underclaims, and bounded bluffs using deterministic parameters. | own hand, own legal tree, public scores, public turn, public revealed masks | opponent hand, reserve, accepted mask identities, hidden pedestal peeking, sampled deals |
| `MC-BOT-003` | A Level 1 response bot may challenge certain lies from public counting and otherwise use a deterministic challenge threshold. | own hand, public declared grade, exposed rows, veiled counts/declared grades, legal response tree | opponent hand, reserve, accepted mask identities, hidden pedestal identity, MCTS, ISMCTS, Monte Carlo, ML, or RL |

## Variant posture and out-of-scope rules

| Rule ID | Boundary | Rulepath position | Notes |
|---|---|---|---|
| `MC-VAR-001` | Public variant | `masked_claims_standard` is the only shipped variant. | Variant data may label the game but cannot define behavior. |
| `MC-VAR-002` | Public naming | Public copy uses **Masked Claims** as an original neutral Rulepath name. | The game is not branded as Coup, Mascarade, Skull, Sheriff of Nottingham, Cockroach Poker, Perudo, Dudo, Liar's Dice, or poker. |
| `MC-OOS-001` | Three- or four-seat play, roles, powers, counter-claims, blocks, nested windows, cancellation, and replacement reactions. | out of scope | Adding any of these requires a later spec and evidence. |
| `MC-OOS-002` | General claim, bluffing, or reaction-window framework. | out of scope | Local implementation does not authorize engine vocabulary or helpers. |
| `MC-OOS-003` | Static-data rule behavior. | forbidden | No formulas, selectors, triggers, scripts, loops, or tactical policy in data. |
| `MC-OOS-004` | Solver or learning bots. | forbidden for public v1/v2 | No MCTS, ISMCTS, Monte Carlo, ML, RL, or hidden-state sampling. |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `MC-AMB-001` | Whether accepted masks reveal at terminal. | Accepted masks never reveal, including at terminal and in every export. | Gate 11 no-leak proof scope. | terminal no-leak trace, public export tests, browser no-leak smoke | resolved |
| `MC-AMB-002` | Whether unplayed hand masks and reserve reveal after the final turn. | Unplayed hand masks and reserve never reveal. | Gate 11 hidden-residue proof scope. | terminal no-leak trace, public export tests | resolved |
| `MC-AMB-003` | Whether an underclaim is treated as honest. | Actual grade greater than or equal to declared grade is honest and scores actual grade plus truth bonus. | Gate 11 scoring design. | honest-underclaim rule test and trace | resolved |
| `MC-AMB-004` | Whether the claimant can act during the response window. | The claimant has an empty gameplay tree and safe waiting metadata. | Gate 11 reaction-window proof scope. | reaction-window legality tests and browser smoke | resolved |
| `MC-AMB-005` | Whether data files can encode claim legality, resolution, scoring, or bot thresholds. | Static data carries only typed content, labels, metadata, fixtures, traces, and reports; behavior lives in Rust. | Rulepath static-data boundary and Gate 11 forbidden changes. | strict-parse tests and boundary review | resolved |

## Rulepath deviations from common variants

| Rule ID | Common variant behavior | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `MC-DEV-001` | Bluffing games often use role names, player powers, elimination, or multi-seat table talk. | Masked Claims uses only numeric mask grades and two seats. | Proves the reaction-window contract without role-roster expression or multiplayer design questions. | yes |
| `MC-DEV-002` | Some liar-style ladders use increasing claims across a round. | Each claim is independent: one hidden mask plus one declared grade. | Keeps the action tree flat and the proof bounded. | yes |
| `MC-DEV-003` | Some games reveal all hidden residue at game end. | Accepted masks, unplayed hands, and reserve never reveal. | Strengthens the no-leak and replay-export proof. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `MC-OOS-005` | Multi-seat bluffing. | Adds kingmaking and elimination questions without strengthening Gate 11. | Later spec explicitly scopes 3+ seats. |
| `MC-OOS-006` | Nested or chained reaction windows. | Gate 11 proves one clean single-depth window. | A future reaction-capable game needs repeated-pressure review. |
| `MC-OOS-007` | Hosted or timed reactions. | V1/v2 are local-first and timeout-free. | Accepted ADR for hosted multiplayer. |

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every rule ID in this document must appear in `RULE-COVERAGE.md`. Silent gaps
are not allowed.

## Rule-ID migration notes

No rule IDs have been migrated.
