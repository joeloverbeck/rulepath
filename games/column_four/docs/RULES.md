# Column Four Rules

Game ID: `column_four`

Public display name: `Column Four`

Implemented variant: `column_four_standard`

Rules version: `column_four-rules-v1`

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
| game id | `column_four` |
| public display name | `Column Four` |
| variant | `column_four_standard` |
| rules version | `column_four-rules-v1` |
| source note | `games/column_four/docs/SOURCES.md` |
| coverage matrix | `games/column_four/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/column_four/docs/MECHANICS.md` |
| implementation admission | `games/column_four/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

| Rule ID | Rule statement | Notes |
|---|---|---|
| `CF-SCOPE-001` | `column_four` is a deterministic two-seat vertical four-in-a-row game with perfect information. | It is the Gate 5 public-polish official game. |
| `CF-SCOPE-002` | The public game name is `Column Four`, a neutral Rulepath name for the selected column-drop connection shape. | Public UI and docs must not imply affiliation with any external source or commercial product. |
| `CF-SCOPE-003` | Rust owns setup, legal actions, validation, gravity, placement, terminal detection, semantic effects, bot choices, replay projections, and viewer-safe views. | TypeScript presents Rust output only. |

## Implemented variant

| Rule ID | Rule statement | Source/rationale link |
|---|---|---|
| `CF-VAR-001` | The only shipped variant is `column_four_standard`. | `SOURCES.md#variant-choice-and-deviations` |
| `CF-VAR-002` | The variant uses seven columns, six rows, exactly two seats, deterministic setup, alternating turns, column-based placement, four-in-a-row wins, and full-board draws only when no line exists. | `SOURCES.md#variant-choice-and-deviations` |

## Components and game-local vocabulary

Define only terms needed for this game. Game nouns belong here or in
`games/column_four`, not in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `CF-COMP-001` | Board | The public seven-column by six-row play area containing forty-two stable cells. | public | Board vocabulary is game-local. |
| `CF-COMP-002` | Column | One vertical stack identified as `c1`, `c2`, `c3`, `c4`, `c5`, `c6`, or `c7`, counted left to right in the default public view. | public | The player chooses a column, not a cell. |
| `CF-COMP-003` | Row | One horizontal level identified as `r1`, `r2`, `r3`, `r4`, `r5`, or `r6`, counted bottom to top. | public | `r1` is the lowest landing row. |
| `CF-COMP-004` | Cell | One board position identified by row and column, from `r1c1` through `r6c7`. | public | Stable IDs are used by rules, traces, effects, replay, and UI accessibility labels. |
| `CF-COMP-005` | Piece | The original Rulepath token owned by one seat and placed into the landing cell of a selected column. | public | The piece must be visually distinct by more than color alone. |
| `CF-COMP-006` | Seat | One of the two player positions: `seat_0` or `seat_1`. | public | Seat identity controls piece ownership, turn ownership, terminal winner, and bot viewpoint. |
| `CF-COMP-007` | Winning line | Four contiguous same-seat cells in a horizontal, vertical, rising-diagonal, or falling-diagonal direction. | public | Line detection is Rust-owned and game-local. |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `CF-SETUP-001` | A new match starts with all forty-two cells empty. | deterministic | public | No setup choices. |
| `CF-SETUP-002` | `seat_0` is the first active seat and places the first piece. | deterministic | public | No first-player randomization. |
| `CF-SETUP-003` | Both seats, every cell, the active seat, and all legal columns are visible to all viewers from setup onward. | deterministic | public | Column Four has no hidden state. |

## Turn, round, and phase sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `CF-TURN-001` | Normal turn while the game is non-terminal. | Current active seat only. | The active seat selects one Rust-supplied legal column action and submits it with the current freshness token. | The submitted action validates and applies. |
| `CF-TURN-002` | Applying a valid non-terminal placement. | System transition. | Rust places the acting seat's piece in the lowest empty row of the selected column, records the semantic effects, and changes the active seat to the other seat. | Immediately after the placement is found not to win or draw. |
| `CF-TURN-003` | Applying a valid terminal placement. | System transition. | Rust places the piece, records the terminal outcome, and exposes no further normal placement turn. | Immediately after a win or draw is recorded. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `CF-ACTION-001` | The game is non-terminal and the acting seat owns the current turn. | Choose any non-full column. | flat column selection, one action per legal column | Each legal action targets exactly one column id. |
| `CF-ACTION-002` | A column contains six pieces. | No normal legal action targets that column. | not applicable | Full columns may be displayed with Rust-provided metadata, but they are not legal targets. |
| `CF-ACTION-003` | The game is terminal. | No normal placement actions are available. | not applicable | Replay and view surfaces must preserve the terminal outcome. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `CF-RESTRICT-001` | Submitted action targets an unknown column. | Reject the submission without changing state. | Diagnostic is viewer-safe and identifies the reason class. | Stable columns are exactly `c1` through `c7`. |
| `CF-RESTRICT-002` | Submitted action targets a full column. | Reject the submission without changing state. | Diagnostic is viewer-safe and explains that the column is full. | This protects dev/API paths even though normal UI must not offer the action. |
| `CF-RESTRICT-003` | Submitted action is stale, malformed, submitted by the wrong seat, or submitted after terminal state. | Reject the submission without changing state. | Diagnostic is viewer-safe and identifies the reason class. | Diagnostics must not include hidden or private data. |
| `CF-RESTRICT-004` | TypeScript could infer a landing row, full column, line, or draw from public data. | It still must not become the behavior authority. | not applicable | The browser may render Rust-provided data only. |

## Scoring and accounting

Placement and gravity rules are included here because they determine which public cell a scoring check evaluates after each legal drop.

| Rule ID | Rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `CF-PLACE-001` | A valid column action creates exactly one new piece for the acting seat. | During action application. | Invalid submissions create no piece. | Occupancy mutation is Rust-owned. |
| `CF-GRAVITY-001` | The new piece lands in the lowest empty row of the selected column. | During action application. | If no row is empty, the action is rejected as a full-column action. | TypeScript must not calculate the landing row as behavior. |
| `CF-SCORE-001` | The game has no score total beyond terminal winner or draw. | Evaluated after each valid placement. | A winning line takes precedence over a full-board draw on the same placement. | The terminal outcome is the only result. |
| `CF-SCORE-002` | The ply count increases by one for each valid placement. | Immediately after placement applies. | Maximum ply count is forty-two. | Invalid submissions do not advance ply count. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `CF-END-001` | A valid placement gives the acting seat four contiguous cells in a horizontal direction. | The acting seat wins immediately. | No draw is recorded for that placement. | Rust reports the winning seat and exact ordered four-cell line. |
| `CF-END-002` | A valid placement gives the acting seat four contiguous cells in a vertical direction. | The acting seat wins immediately. | No draw is recorded for that placement. | Rust reports the winning seat and exact ordered four-cell line. |
| `CF-END-003` | A valid placement gives the acting seat four contiguous cells in a rising diagonal direction. | The acting seat wins immediately. | No draw is recorded for that placement. | Rising diagonals increase by row and column in the coordinate system. |
| `CF-END-004` | A valid placement gives the acting seat four contiguous cells in a falling diagonal direction. | The acting seat wins immediately. | No draw is recorded for that placement. | Falling diagonals decrease by row while increasing by column in the coordinate system. |
| `CF-END-005` | All forty-two cells are occupied and no winning line exists after the final placement. | The game ends in a draw. | Draw is the canonical wording. | Tie may appear only as secondary explanatory wording in docs. |
| `CF-END-006` | More than one winning line is completed by the same placement. | The acting seat wins and Rust exposes one deterministic primary line. | Primary-line order is horizontal, vertical, rising diagonal, falling diagonal; within a direction choose the line whose ordered cell-id list sorts first lexicographically. | Additional lines may be exposed later only if documented and covered. |
| `CF-END-007` | A terminal state has already been reached. | The recorded winner or draw remains unchanged. | not applicable | Later normal apply attempts are rejected without changing state. |

Terminal public views also expose a Rust-owned outcome rationale. Wins use template key `column_four.line_completed`, decisive cause `line_completed`, the ordered decisive line cells, the line orientation, and rule IDs `CF-SCORE-001` plus the matching directional terminal rule `CF-END-001` through `CF-END-004`. Draws use template key `column_four.full_board_draw`, decisive cause `full_board_no_line`, `board_full=true`, and rule IDs `CF-SCORE-001` and `CF-END-005`.

## Visibility and private information

Public/browser payloads must not reveal hidden information through public views,
action trees, previews, diagnostics, effect logs, DOM attributes, test IDs,
logs, local storage, replay exports, bot explanations, candidate rankings, or
dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `CF-VIS-001` | Board cells, pieces, active seat, legal column targets for the active turn, Rust-provided previews, effects, terminal winner, winning line, and draw outcome. | all viewers | Always once state exists. | public view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | The game is perfect-information; there is no hidden state to redact. |
| `CF-VIS-002` | Private-view status. | all viewers | Always. | public/private view contract and replay export | The canonical status is `not_applicable_perfect_information`. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `CF-RNG-001` | Setup and rules do not sample randomness. | Replaying the same commands from the same setup must reproduce the same state without RNG input. | public | Bot randomness is owned by the bot layer, not by game rules. |
| `CF-RNG-002` | Replay evidence must bind setup, variant, command stream, effects, action tree, public view, board projection, terminal outcome, and deterministic hashes. | Replay checks must reproduce state, effects, action tree, view, replay projection, and hashes. | public | Required by Gate 5 exit criteria. |

## Bot-relevant non-authoritative strategy notes

These notes are not rule authority. Strategy claims must be checked against rule
IDs above.

| Situation | Practical note | Rule IDs checked | Hidden-info limit |
|---|---|---|---|
| A bot has an immediate winning column. | A Level 2 bot may prefer that legal column before other tactical priorities. | `CF-ACTION-001`, `CF-GRAVITY-001`, `CF-END-001` through `CF-END-004` | No hidden information exists. |
| The opponent has an immediate winning column. | A Level 2 bot may prefer a legal block before non-urgent placements. | `CF-ACTION-001`, `CF-GRAVITY-001`, `CF-END-001` through `CF-END-004` | No hidden information exists. |
| No immediate win or block exists. | A Level 2 bot may use deterministic, documented tactical preferences such as center pressure, safe threats, and avoiding immediate replies. | `CF-ACTION-001`, `CF-RNG-001` | No hidden information exists. |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `CF-AMB-001` | Which public name to use for a commercial-adjacent vertical four-in-a-row game. | Use `Column Four`. | `SOURCES.md#public-naming-rationale` | UI smoke and docs review. | Neutral project-owned naming avoids source confusion. |
| `CF-AMB-002` | Which seat acts first. | `seat_0` acts first. | Deterministic replay needs and Gate 5 spec. | Setup snapshot, rule tests, replay/hash tests. | No random setup. |
| `CF-AMB-003` | How to identify cells. | Use `r1c1` through `r6c7`, with rows counted bottom to top and columns left to right. | Gate 5 spec. | Action-id stability tests, rule tests, trace tests, UI smoke. | Bottom-origin rows make gravity and landing rows readable. |
| `CF-AMB-004` | Whether a final placement can both fill the board and complete a line. | The line win is recorded; draw applies only when the full board has no line. | Rule clarity and terminal precedence. | Terminal rule tests and traces. | This keeps draw semantics unambiguous. |
| `CF-AMB-005` | Which line to expose when one placement completes multiple lines. | Use the deterministic primary-line tie-break in `CF-END-006`. | Replay/hash determinism and UI highlight stability. | Multi-line terminal rule test and golden trace. | The outcome is the same winner regardless of which primary line is highlighted. |

## Rulepath deviations from common variants

| Rule ID | Common variant behavior | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `CF-VAR-003` | Some related games use proprietary names, commercial presentations, branded colors, PopOut, larger connection lengths, custom sizes, or other variant modes. | Gate 5 implements only a neutral 7 by 6 vertical column-drop game with four-in-a-row wins and full-board draws. | Keeps the official game narrow, replayable, IP-conservative, and public-polished without premature abstraction. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|
| `CF-VAR-004` | PopOut or piece-removal variants. | Gate 5 proves column placement and gravity only. | A later accepted spec requests removal mechanics. |
| `CF-VAR-005` | Misere, Five-in-a-Row, arbitrary m/n/k, custom board sizes, alternate gravity, simultaneous turns, more than two seats, randomized setup, or hidden-information variants. | These variants would obscure the narrow public-polish goal and create premature abstraction pressure. | A later accepted spec and architecture review request the shape. |

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every rule ID in this document must appear in `RULE-COVERAGE.md`. Silent gaps
are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| not applicable | not applicable | Initial rule set. | not applicable | 2026-06-06 |
