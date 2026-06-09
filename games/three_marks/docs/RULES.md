# Three Marks Rules

Game ID: `three_marks`

Public display name: `Three Marks`

Implemented variant: `three_marks_standard`

Rules version: `three_marks-rules-v1`

Prepared by: `Codex`

Created: 2026-06-06

Last updated: 2026-06-06

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
| game id | `three_marks` |
| public display name | `Three Marks` |
| variant | `three_marks_standard` |
| rules version | `three_marks-rules-v1` |
| source note | `games/three_marks/docs/SOURCES.md` |
| coverage matrix | `games/three_marks/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/three_marks/docs/MECHANICS.md` |
| implementation admission | `games/three_marks/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

| Rule ID | Rule statement | Notes |
|---|---|---|
| `TM-SCOPE-001` | `three_marks` is a deterministic two-seat fixed-board placement game with perfect information. | It is the first official Rulepath board smoke game. |
| `TM-SCOPE-002` | The public game name is `Three Marks`, a neutral Rulepath name for the selected classic three-in-a-row placement shape. | Public UI and docs must not imply affiliation with any external source or commercial product. |
| `TM-SCOPE-003` | Rust owns setup, legal actions, validation, placement, terminal detection, semantic effects, bot choices, replay projections, and viewer-safe views. | TypeScript presents Rust output only. |

## Implemented variant

| Rule ID | Rule statement | Source/rationale link |
|---|---|---|
| `TM-VAR-001` | The only shipped variant is `three_marks_standard`. | `SOURCES.md#variant-choice-and-deviations` |
| `TM-VAR-002` | The variant uses a fixed 3 by 3 board, exactly two seats, deterministic setup, alternating turns, line wins, and full-board draws. | `SOURCES.md#variant-choice-and-deviations` |

## Components and game-local vocabulary

Define only terms needed for this game. Game nouns belong here or in
`games/three_marks`, not in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `TM-COMP-001` | Board | The public 3 by 3 play area containing nine stable cells. | public | Board vocabulary is game-local. |
| `TM-COMP-002` | Cell | One fixed board position identified as `r1c1`, `r1c2`, `r1c3`, `r2c1`, `r2c2`, `r2c3`, `r3c1`, `r3c2`, or `r3c3`. | public | Row numbers count from top to bottom; column numbers count from left to right in the default public view. |
| `TM-COMP-003` | Mark | The original Rulepath token placed by a seat into an empty cell. | public | The mark must be visually distinct by more than color alone. |
| `TM-COMP-004` | Seat | One of the two player positions: `seat_0` or `seat_1`. | public | Seat identity controls mark ownership, turn ownership, and terminal winner. |
| `TM-COMP-005` | Line | One of the eight possible three-cell groups: three rows, three columns, and two diagonals. | public | Line detection is Rust-owned and game-local. |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `TM-SETUP-001` | A new match starts with all nine cells empty. | deterministic | public | No setup choices. |
| `TM-SETUP-002` | `seat_0` is the first active seat and places the first mark. | deterministic | public | No first-player randomization. |
| `TM-SETUP-003` | Both seats and all cells are visible to all viewers from setup onward. | deterministic | public | Three Marks has no hidden state. |

## Turn, round, and phase sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `TM-TURN-001` | Normal turn while the game is non-terminal. | Current active seat only. | The active seat selects one Rust-supplied legal placement action and submits it with the current freshness token. | The submitted action validates and applies. |
| `TM-TURN-002` | End of a non-terminal placement. | System transition. | If the placement does not complete a line and does not fill the board, the active seat changes to the other seat. | Immediately after the placement effect is recorded. |
| `TM-TURN-003` | End of a terminal placement. | System transition. | If the placement wins or draws the game, the active seat does not receive another normal placement turn. | Immediately after terminal outcome is recorded. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `TM-ACTION-001` | The game is non-terminal and the acting seat owns the current turn. | Place the acting seat's mark in any empty cell. | flat targeted placement, one action per legal cell | Each legal action targets exactly one empty cell. |
| `TM-ACTION-002` | A cell is occupied. | No normal legal action targets that cell. | not applicable | Occupied cells may be displayed, but they are not legal targets. |
| `TM-ACTION-003` | The game is terminal. | No normal placement actions are available. | not applicable | Replay and view surfaces must preserve the terminal outcome. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `TM-RESTRICT-001` | Submitted action targets an unknown cell. | Reject the submission without changing state. | Diagnostic is viewer-safe and identifies the reason class. | Stable cells are exactly the nine IDs listed in `TM-COMP-002`. |
| `TM-RESTRICT-002` | Submitted action targets an occupied cell. | Reject the submission without changing state. | Diagnostic is viewer-safe and identifies the reason class. | This protects dev/API paths even though normal UI must not offer the action. |
| `TM-RESTRICT-003` | Submitted action is stale, malformed, submitted by the wrong seat, or submitted after terminal state. | Reject the submission without changing state. | Diagnostic is viewer-safe and identifies the reason class. | Diagnostics must not include hidden or private data. |

## Scoring and accounting

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `TM-SCORE-001` | The game has no score total beyond terminal winner or draw. | Evaluated after each valid placement. | A winning line takes precedence over a full-board draw on the same placement. | The terminal outcome is the only result. |
| `TM-SCORE-002` | The ply count increases by one for each valid placement. | Immediately after placement applies. | Maximum ply count is nine. | Invalid submissions do not advance ply count. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `TM-END-001` | A valid placement gives the acting seat all three cells in any row, column, or diagonal. | The acting seat wins immediately. | No draw is recorded for that placement. | Rust reports the winning seat and the exact ordered three-cell line. |
| `TM-END-002` | All nine cells are occupied and no winning line exists. | The game ends in a draw. | Draw is the canonical wording. | Tie may appear only as secondary explanatory wording in docs. |
| `TM-END-003` | A terminal state has already been reached. | The recorded winner or draw remains unchanged. | not applicable | Later normal apply attempts are rejected without changing state. |

Terminal public views also expose a Rust-owned outcome rationale. Wins use template key `three_marks.line_completed`, decisive cause `line_completed`, the ordered decisive line cells, and rule IDs `TM-SCORE-001` and `TM-END-001`. Draws use template key `three_marks.full_board_draw`, decisive cause `full_board_no_line`, `board_full=true`, and rule IDs `TM-SCORE-001` and `TM-END-002`.

## Visibility and private information

Public/browser payloads must not reveal hidden information through public views,
action trees, previews, diagnostics, effect logs, DOM attributes, test IDs,
logs, local storage, replay exports, bot explanations, candidate rankings, or
dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `TM-VIS-001` | Board cells, marks, active seat, legal placement targets for the active turn, effects, terminal winner, winning line, and draw outcome. | all viewers | Always once state exists. | public view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | The game is perfect-information; there is no hidden state to redact. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `TM-RNG-001` | Setup and rules do not sample randomness. | Replaying the same commands from the same setup must reproduce the same state without RNG input. | public | Bot randomness is owned by the bot layer, not by game rules. |
| `TM-RNG-002` | Replay evidence must bind setup, variant, command stream, effects, action tree, public view, board projection, and terminal outcome. | Replay checks must reproduce state, effect, action-tree, view, and replay hashes. | public | Required by Gate 4 exit criteria. |

## Bot-relevant non-authoritative strategy notes

These notes are not rule authority. Strategy claims must be checked against rule
IDs above.

| Situation | Practical note | Rule IDs checked | Hidden-info limit |
|---|---|---|---|
| A bot has an immediate winning placement. | A Level 1 bot may prefer that legal placement before other tactical priorities. | `TM-ACTION-001`, `TM-END-001` | No hidden information exists. |
| The opponent has an immediate winning placement. | A Level 1 bot may prefer a legal block before non-urgent placements. | `TM-ACTION-001`, `TM-END-001` | No hidden information exists. |
| No immediate win or block exists. | A Level 1 bot may use deterministic, documented position preferences such as center before corners before edges. | `TM-ACTION-001`, `TM-RNG-001` | No hidden information exists. |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `TM-AMB-001` | Which public name to use for a Tic-Tac-Toe-like placement game. | Use `Three Marks`. | `SOURCES.md#public-naming-rationale` | UI smoke and docs review. | Neutral project-owned naming avoids source confusion. |
| `TM-AMB-002` | Which seat acts first. | `seat_0` acts first. | Deterministic replay needs and Gate 4 spec. | Setup snapshot, rule tests, replay/hash tests. | No random setup. |
| `TM-AMB-003` | How to identify cells. | Use `r1c1` through `r3c3`. | Gate 4 spec. | Action-id stability tests, rule tests, UI smoke. | Different public layouts must still preserve stable IDs. |
| `TM-AMB-004` | Whether a final placement can both fill the board and complete a line. | The line win is recorded; draw applies only when the full board has no line. | Rule clarity and terminal precedence. | Terminal rule tests and traces. | This keeps draw semantics unambiguous. |

## Rulepath deviations from common variants

| Rule ID | Common variant behavior | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `TM-VAR-003` | Some related games add movement, sliding, larger boards, or alternate winning conditions. | Gate 4 implements only fixed 3 by 3 placement until a row, column, or diagonal is completed, or the board fills without a line. | Keeps the first board game narrow, replayable, and public-domain-safe. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|
| `TM-VAR-004` | Movement or sliding phases, including Morris-like or Achi-like behavior. | Gate 4 is a placement-only board smoke. | A later roadmap gate explicitly requests a movement game. |
| `TM-VAR-005` | Misere, wild-mark, larger-board, configurable-board, generalized m,n,k, randomized setup, or multi-player variants. | Gate 4 proves one small declared variant and forbids generalized mechanic detours. | A later accepted spec and architecture review request the shape. |

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every rule ID in this document must appear in `RULE-COVERAGE.md`. Silent gaps
are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| not applicable | not applicable | Initial rule set. | not applicable | 2026-06-06 |
