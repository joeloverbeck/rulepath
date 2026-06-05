# race_to_n Rules

Game ID: `race_to_n`

Public display name: `Race to 21`

Implemented variant: `single-counter normal-play race to 21; add 1, 2, or 3`

Rules version: `race_to_n-rules-v1`

Prepared by: `Codex`

Created: 2026-06-05

Last updated: 2026-06-05

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
| game id | `race_to_n` |
| public display name | `Race to 21` |
| variant | `single-counter normal-play race to 21; add 1, 2, or 3` |
| rules version | `race_to_n-rules-v1` |
| source note | `games/race_to_n/docs/SOURCES.md` |
| coverage matrix | `games/race_to_n/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/race_to_n/docs/MECHANICS.md` |
| implementation admission | `games/race_to_n/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

| Rule ID | Rule statement | Notes |
|---|---|---|
| `R-SCOPE-001` | `race_to_n` is a deterministic two-seat numeric race played on one public counter. | This is the smallest official-game rules shape for Gate 1 plumbing. |
| `R-SCOPE-002` | The game is a `foundation-smoke` Rulepath game: modest presentation, full official evidence. | Later tickets must still provide tests, traces, replay, simulation, bot, benchmarks, WASM, and docs. |

## Implemented variant

| Rule ID | Rule statement | Source/rationale link |
|---|---|---|
| `R-VAR-001` | The implemented variant is `Race to 21`: start at 0, add 1 to 3 each turn, and win by making the counter exactly 21. | `SOURCES.md#variant-choice-and-deviations` |
| `R-VAR-002` | The variant has fixed parameters: target `21`, maximum addition `3`, exactly two seats, deterministic setup, and seat 0 first. | `SOURCES.md#variant-choice-and-deviations` |

## Components and game-local vocabulary

Define only terms needed for this game. Game nouns belong here or in
`games/race_to_n`, not in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `R-COMP-001` | Counter | The shared whole-number total that starts at 0 and rises during play. | public | The counter is the only game position. |
| `R-COMP-002` | Seat | One of the two player positions: `seat_0` or `seat_1`. | public | Seat identity controls turn ownership and terminal winner. |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `R-SETUP-001` | A new match starts with the counter at 0 and no winner. | deterministic | public | No setup choices. |
| `R-SETUP-002` | `seat_0` is the first active seat. | deterministic | public | No first-player randomization. |
| `R-SETUP-003` | The target is 21 and the per-turn maximum addition is 3. | deterministic | public | These are fixed constants for this variant. |

## Turn, round, and phase sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `R-TURN-001` | Normal turn while the counter is below 21. | Current active seat only. | The active seat selects one Rust-supplied legal addition and submits it with the current freshness token. | The submitted action validates and applies. |
| `R-TURN-002` | End of a non-terminal turn. | System transition. | If the counter remains below 21 after the action, active seat changes to the other seat. | Immediately after the action effect is recorded. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `R-ACTION-001` | Counter is below 21 and the acting seat owns the current turn. | Add any whole number from 1 through the smaller of 3 and the remaining distance to 21. | flat | At counter 18 legal additions are 1, 2, 3; at 19 they are 1, 2; at 20 only 1 is legal. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `R-RESTRICT-001` | Submitted action is not in the Rust-generated legal set, uses a stale freshness token, is submitted by the wrong seat, or is submitted after terminal state. | Reject the submission without changing state. | Diagnostic is viewer-safe and identifies the reason class without exposing anything private. | This includes zero, negative, overshoot, malformed, wrong-seat, stale, and terminal submissions. |

## Scoring and accounting

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `R-SCORE-001` | The game has no score total beyond terminal winner. | Evaluated after each valid action. | Draws are impossible in the declared variant. | Outcome is winner-only. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `R-END-001` | A valid action makes the counter exactly 21. | The acting seat wins immediately. | No tie. | Legal actions prevent overshooting 21. |

## Visibility and private information

Public/browser payloads must not reveal hidden information through public views,
action trees, previews, diagnostics, effect logs, DOM attributes, test IDs,
logs, local storage, replay exports, bot explanations, candidate rankings, or
dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `R-VIS-001` | Counter, active seat, legal additions for the active turn, effects, and terminal winner. | all viewers | Always once state exists. | public view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | The game is perfect-information; there is no hidden state to redact. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `R-RNG-001` | Setup and rules do not sample randomness. | Replaying the same commands from the same setup must reproduce the same state without RNG input. | public | Bot randomness is owned by the bot layer, not by game rules. |
| `R-RNG-002` | Replay evidence must bind setup, variant, command stream, effects, action tree, public view, and terminal outcome. | Later replay tests must reproduce state, effect, action-tree, and view hashes. | public | Required by Gate 1 exit criteria. |

## Bot-relevant non-authoritative strategy notes

These notes are not rule authority. Strategy claims must be checked against rule
IDs above.

| Situation | Practical note | Rule IDs checked | Hidden-info limit |
|---|---|---|---|
| Counter below 17 | A random legal bot may choose any legal addition uniformly; no strategic claim is required for Level 0. | `R-ACTION-001`, `R-RNG-001` | No hidden information exists. |
| Counter near 21 | Legal action generation must cap choices by remaining distance; bot choices still go through normal validation. | `R-ACTION-001`, `R-END-001` | No hidden information exists. |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `R-AMB-001` | Whether this game is phrased as removing from a pile or racing to a target. | Racing to a target. | `SOURCES.md#ambiguity-log` | Shortest normal trace, terminal trace, target-bounded legal-action rule tests. | Keeps public name and UI aligned with the game ID. |
| `R-AMB-002` | Whether additions may overshoot the target. | No overshoot. | `SOURCES.md#ambiguity-log` | Invalid diagnostic trace and rule tests at 18, 19, and 20. | Terminal target is exact. |

## Rulepath deviations from common variants

| Rule ID | Common variant behavior | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `R-VAR-003` | Many Nim-family explanations remove objects from one or more piles. | `race_to_n` adds to one counter until the target is reached. | A single public counter is simpler for Gate 1 UI, effects, and replay plumbing. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `R-VAR-004` | Multi-pile Nim, arbitrary subtraction sets, misere play, randomized starts, player-selected targets, generalized counters, or any pile/deck/board/resource framing. | Gate 1 needs one tiny declared variant and forbids generalized mechanic detours. | A later official game or roadmap gate explicitly requests the shape. |

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every rule ID in this document must appear in `RULE-COVERAGE.md`. Silent gaps
are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| not applicable | not applicable | Initial rule set. | not applicable | 2026-06-05 |
